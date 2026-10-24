//! Everything Paperback asks of a PDF library, answered by hayro instead of pdfium when the `hayro` feature is on.
//!
//! The surface is the same as [`crate::pdfium`]'s, type for type and method for method, so the parser compiles against either without knowing which it has. What differs is underneath. pdfium is asked one question at a time through handles; hayro interprets a page's content stream once into a device. So a page here is read in full when it is loaded, and every question after that is answered from what was collected.
//!
//! The text layer is a port of pdfium's own (`core/fpdftext`), not a fresh design: the parser's heuristics were measured against the characters, inferred spaces and line breaks pdfium produces, and they only hold if those come out the same.

use std::{
	collections::{HashMap, HashSet},
	sync::OnceLock,
};

use hayro_syntax::{
	DecryptionError, LoadPdfError, Pdf,
	object::{Array, Dict, MaybeRef, Name, Object, ObjectIdentifier},
};

mod bidi;
mod links;
mod page;
mod tags;
mod text;

#[allow(unused_imports)]
pub use page::{
	AnnotationLink, Bounds, LinkTarget, MAX_FORM_DEPTH, ObjectKind, PAGE_IMAGE_COVERAGE, PageBitmap, PageImage,
	PageObject, PdfPage,
};
#[allow(unused_imports)]
pub use tags::{Tag, TagTree};
#[allow(unused_imports)]
pub use text::{CharBox, FontInfo, PdfTextPage, TextMatrix};

/// There is no library to find: hayro is compiled in. Kept so the callers that point pdfium at its binary need no feature checks of their own.
pub const fn set_library_path(_path: &str) {}

/// Why a PDF could not be read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PdfError {
	/// The file is encrypted and the password was missing or wrong. Kept separate from the rest because it is the one failure the reader can do something about.
	PasswordRequired,
	/// Never produced by this backend, since there is no library to fail to load. Kept so callers match the same set of errors either way.
	#[allow(dead_code)]
	LibraryUnavailable,
	/// The file could not be opened or parsed.
	CouldNotOpen,
	/// A request on a document that did open failed, such as a page that does not exist.
	Failed,
}

impl std::fmt::Display for PdfError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::PasswordRequired => write!(f, "the document is password protected"),
			Self::LibraryUnavailable => write!(f, "the pdf library could not be loaded"),
			Self::CouldNotOpen => write!(f, "the document could not be opened"),
			Self::Failed => write!(f, "the pdf library rejected the request"),
		}
	}
}

impl std::error::Error for PdfError {}

/// An open PDF.
pub struct PdfDocument {
	pdf: Pdf,
	/// Each page's object, by its position in the document, so a destination naming a page object can be turned into a page number.
	page_numbers: HashMap<ObjectIdentifier, i32>,
	/// The structure tree, read once the first time a page asks for its tags rather than once per page.
	structure: OnceLock<Option<tags::Structure>>,
}

impl PdfDocument {
	/// Opens `path`, decrypting with `password` when the file is encrypted.
	///
	/// # Errors
	///
	/// [`PdfError::PasswordRequired`] when the file is encrypted and the password does not fit, and [`PdfError::CouldNotOpen`] for a file that cannot be read or is not readable as a PDF.
	pub fn open(path: &str, password: Option<&str>) -> Result<Self, PdfError> {
		let bytes = std::fs::read(path).map_err(|_| PdfError::CouldNotOpen)?;
		Self::from_bytes(bytes, password)
	}

	/// Opens a PDF already in memory, which is how the repaired copy of a broken file is read.
	///
	/// # Errors
	///
	/// The same failures as [`PdfDocument::open`].
	pub fn from_bytes(bytes: Vec<u8>, password: Option<&str>) -> Result<Self, PdfError> {
		let pdf = Pdf::new_with_password(bytes, password.unwrap_or("")).map_err(|error| match error {
			LoadPdfError::Decryption(DecryptionError::PasswordProtected) => PdfError::PasswordRequired,
			_ => PdfError::CouldNotOpen,
		})?;
		let page_numbers = pdf
			.pages()
			.iter()
			.enumerate()
			.filter_map(|(index, page)| Some((page.raw().obj_id()?, i32::try_from(index).ok()?)))
			.collect();
		Ok(Self { pdf, page_numbers, structure: OnceLock::new() })
	}

	pub fn page_count(&self) -> i32 {
		i32::try_from(self.pdf.pages().len()).unwrap_or(i32::MAX)
	}

	/// Loads one page, which stays borrowed from this document so it cannot outlive it.
	///
	/// # Errors
	///
	/// [`PdfError::Failed`] when there is no such page.
	pub fn page(&self, index: i32) -> Result<PdfPage<'_>, PdfError> {
		PdfPage::load(self, index)
	}

	/// One of the document's metadata fields, such as `Title` or `Author`.
	pub fn metadata(&self, key: &str) -> Option<String> {
		let metadata = self.pdf.metadata();
		let value = match key {
			"Title" => metadata.title.as_deref(),
			"Author" => metadata.author.as_deref(),
			"Subject" => metadata.subject.as_deref(),
			"Keywords" => metadata.keywords.as_deref(),
			"Creator" => metadata.creator.as_deref(),
			"Producer" => metadata.producer.as_deref(),
			_ => None,
		}?;
		Some(decode_text_string(value)).filter(|text| !text.is_empty())
	}

	/// The bookmark outline, flattened depth-first with the nesting recorded as a level.
	///
	/// A damaged outline can point back at itself. The walk stops descending the branch that loops and keeps everything it read on the way, because a partial contents list is far more use to a reader than none.
	pub fn outline(&self, max_depth: u32) -> Vec<OutlineEntry> {
		let mut entries = Vec::new();
		let Some(outlines) = self.catalog().and_then(|catalog| catalog.get::<Dict<'_>>(b"Outlines")) else {
			return entries;
		};
		let mut seen = HashSet::new();
		self.walk_outline(&outlines, 0, max_depth, &mut seen, &mut entries);
		entries
	}

	fn walk_outline(
		&self,
		parent: &Dict<'_>,
		level: u32,
		max_depth: u32,
		seen: &mut HashSet<ObjectIdentifier>,
		entries: &mut Vec<OutlineEntry>,
	) {
		let mut next = parent.get::<Dict<'_>>(b"First");
		while let Some(item) = next {
			if let Some(id) = item.obj_id()
				&& !seen.insert(id)
			{
				return;
			}
			let title = item.get::<hayro_syntax::object::String<'_>>(b"Title").map(|title| decode_text_string(&title));
			let page = self.destination_page(&item);
			if let Some(title) = title.filter(|title| !title.is_empty())
				&& let Some(page) = page
			{
				entries.push(OutlineEntry { level, title, page });
			}
			if level + 1 < max_depth {
				self.walk_outline(&item, level + 1, max_depth, seen, entries);
			}
			next = item.get::<Dict<'_>>(b"Next");
		}
	}

	/// Where an outline item or link annotation points: its `/Dest`, or failing that the destination of a `GoTo` action in its `/A`, which is the order pdfium tries them in.
	fn destination_page(&self, item: &Dict<'_>) -> Option<i32> {
		if let Some(destination) = item.get::<Object<'_>>(b"Dest")
			&& let Some(page) = self.resolve_destination(destination, 0)
		{
			return Some(page);
		}
		let action = item.get::<Dict<'_>>(b"A")?;
		if action.get::<Name<'_>>(b"S").as_deref() != Some(b"GoTo".as_slice()) {
			return None;
		}
		self.resolve_destination(action.get::<Object<'_>>(b"D")?, 0)
	}

	/// The page a destination names, following named destinations through the catalog's `/Dests` dictionary and its `/Names` tree.
	fn resolve_destination(&self, destination: Object<'_>, depth: u32) -> Option<i32> {
		if depth > 8 {
			return None;
		}
		match destination {
			Object::Array(array) => self.page_of_explicit_destination(&array),
			Object::Dict(dict) => self.resolve_destination(dict.get::<Object<'_>>(b"D")?, depth + 1),
			Object::Name(name) => {
				let target = self.catalog()?.get::<Dict<'_>>(b"Dests")?.get::<Object<'_>>(name.as_ref())?;
				self.resolve_destination(target, depth + 1)
			}
			Object::String(name) => {
				let tree = self.catalog()?.get::<Dict<'_>>(b"Names")?.get::<Dict<'_>>(b"Dests")?;
				let target = lookup_name_tree(&tree, name.as_bytes(), 0)?;
				self.resolve_destination(target, depth + 1)
			}
			_ => None,
		}
	}

	fn page_of_explicit_destination(&self, array: &Array<'_>) -> Option<i32> {
		match array.raw_iter().next()? {
			MaybeRef::Ref(reference) => self.page_numbers.get(&reference.into()).copied(),
			// A destination into another document names its page by number; this one has no such page to give.
			MaybeRef::NotRef(_) => None,
		}
	}

	fn catalog(&self) -> Option<Dict<'_>> {
		let xref = self.pdf.xref();
		xref.get::<Dict<'_>>(xref.root_id())
	}

	const fn pdf(&self) -> &Pdf {
		&self.pdf
	}

	fn page_number(&self, id: ObjectIdentifier) -> Option<i32> {
		self.page_numbers.get(&id).copied()
	}

	fn structure(&self) -> Option<&tags::Structure> {
		self.structure.get_or_init(|| tags::Structure::read(self, &self.catalog()?)).as_ref()
	}
}

/// One entry in a PDF's bookmark outline.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutlineEntry {
	/// How deep this entry sits, counting from zero at the top.
	pub level: u32,
	pub title: String,
	/// The page it points at, zero-based. Entries pointing nowhere are left out entirely, so this is always a real page.
	pub page: i32,
}

/// Finds `key` in a name tree, descending its `/Kids` and searching each leaf's `/Names` pairs.
fn lookup_name_tree<'a>(node: &Dict<'a>, key: &[u8], depth: u32) -> Option<Object<'a>> {
	if depth > 32 {
		return None;
	}
	if let Some(names) = node.get::<Array<'a>>(b"Names") {
		let mut items = names.iter::<Object<'a>>();
		while let (Some(name), Some(value)) = (items.next(), items.next()) {
			if let Object::String(name) = name
				&& name.as_bytes() == key
			{
				return Some(value);
			}
		}
	}
	let kids = node.get::<Array<'a>>(b"Kids")?;
	kids.iter::<Dict<'a>>().find_map(|kid| lookup_name_tree(&kid, key, depth + 1))
}

/// Decodes a PDF text string: UTF-16 or UTF-8 when it opens with that encoding's byte order mark, `PDFDocEncoding` otherwise.
fn decode_text_string(bytes: &[u8]) -> String {
	if let Some(rest) = bytes.strip_prefix(&[0xFE, 0xFF]) {
		let units: Vec<u16> = rest.as_chunks::<2>().0.iter().map(|&pair| u16::from_be_bytes(pair)).collect();
		return String::from_utf16_lossy(&units);
	}
	if let Some(rest) = bytes.strip_prefix(&[0xFF, 0xFE]) {
		let units: Vec<u16> = rest.as_chunks::<2>().0.iter().map(|&pair| u16::from_le_bytes(pair)).collect();
		return String::from_utf16_lossy(&units);
	}
	if let Some(rest) = bytes.strip_prefix(&[0xEF, 0xBB, 0xBF]) {
		return String::from_utf8_lossy(rest).into_owned();
	}
	bytes.iter().map(|&byte| pdf_doc_char(byte)).collect()
}

/// `PDFDocEncoding` agrees with Latin-1 except in 0x18-0x1F and 0x80-0x9F, where it puts the typographic marks Latin-1 leaves out.
fn pdf_doc_char(byte: u8) -> char {
	const LOW: [char; 8] =
		['\u{02D8}', '\u{02C7}', '\u{02C6}', '\u{02D9}', '\u{02DD}', '\u{02DB}', '\u{02DA}', '\u{02DC}'];
	const HIGH: [char; 32] = [
		'\u{2022}', '\u{2020}', '\u{2021}', '\u{2026}', '\u{2014}', '\u{2013}', '\u{0192}', '\u{2044}', '\u{2039}',
		'\u{203A}', '\u{2212}', '\u{2030}', '\u{201E}', '\u{201C}', '\u{201D}', '\u{2018}', '\u{2019}', '\u{201A}',
		'\u{2122}', '\u{FB01}', '\u{FB02}', '\u{0141}', '\u{0152}', '\u{0160}', '\u{0178}', '\u{017D}', '\u{0131}',
		'\u{0142}', '\u{0153}', '\u{0161}', '\u{017E}', '\u{FFFD}',
	];
	match byte {
		0x18..=0x1F => LOW[usize::from(byte - 0x18)],
		0x80..=0x9F => HIGH[usize::from(byte - 0x80)],
		0xA0 => '\u{20AC}',
		_ => char::from(byte),
	}
}

#[cfg(test)]
mod tests {
	use super::decode_text_string;

	#[test]
	fn a_text_string_with_a_byte_order_mark_is_utf16() {
		assert_eq!(decode_text_string(&[0xFE, 0xFF, 0x00, 0x48, 0x00, 0x69]), "Hi");
	}

	/// The bytes Latin-1 would read as control characters are quotation marks in `PDFDocEncoding`.
	#[test]
	fn pdf_doc_encoding_has_its_own_quotation_marks() {
		assert_eq!(decode_text_string(&[0x8D, b'Q', 0x8E]), "\u{201C}Q\u{201D}");
	}
}
