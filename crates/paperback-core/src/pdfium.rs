//! Everything Paperback asks of Pdfium: opening a document, walking its tag tree, reading text
//! with glyph positions, and rendering a page for OCR.
//!
//! Pdfium is a C API, and it shows. Strings come back through a call-twice dance: once with no
//! buffer to be told the length, once more to have it filled, always UTF-16LE with a trailing
//! NUL counted in the length. Absent values are `-1` or a null handle. Every handle has to be
//! closed by hand, in the right order, or the process leaks or crashes. None of that belongs in
//! a parser, so all of it stops here, and what leaves this module is `Option`, `String`,
//! iterators, and handles that close themselves when dropped.
//!
//! The raw FFI declarations and the library loading come from `pdfium-render`. This module is
//! only the layer above them, and it is deliberately narrow: it covers what the PDF parser and
//! the OCR renderer ask for and nothing else. Anything Pdfium can do that Paperback does not
//! need is absent on purpose, because an unused wrapper is an unused wrapper that still has to
//! be read and maintained.
//!
//! Borrowing is what keeps the close order right. A [`PdfPage`] borrows the document it came
//! from, a [`PdfTextPage`] and a [`TagTree`] borrow their page, and a [`Tag`] borrows its tree,
//! so the compiler refuses the orderings that would hand Pdfium a freed handle. That is why
//! these types carry lifetimes rather than reference counts.

use std::{
	ffi::{c_ulong, c_void},
	path::Path,
	sync::{Mutex, OnceLock},
};

mod page;
mod tags;
mod text;

// The boundary's whole surface, gathered here so one import names everything a caller can
// touch. Some of these are only ever reached as the return type of a method rather than named
// outright, which is what the allow is for.
#[allow(unused_imports)]
pub use page::{AnnotationLink, Bounds, LinkTarget, ObjectKind, PageBitmap, PageObject, PdfPage};
use pdfium_render::prelude::{FPDF_BOOKMARK, FPDF_DOCUMENT, Pdfium, PdfiumLibraryBindings};
#[allow(unused_imports)]
pub use tags::{Tag, TagTree};
#[allow(unused_imports)]
pub use text::{CharBox, FontInfo, PdfTextPage, TextMatrix, WebLinks};

/// Pdfium's own code for "this file is encrypted and the password was wrong or missing".
const FPDF_ERR_PASSWORD: c_ulong = 4;

/// Where [`set_library_path`] was told to find the Pdfium binary, recorded before anything tries
/// to use it. The desktop app ships the library beside its executable and the mobile builds wrap
/// it in a framework, so the path is never known at compile time.
static LIBRARY_PATH: Mutex<Option<String>> = Mutex::new(None);
/// The loaded library. Binding runs once, on whichever call needs Pdfium first.
static PDFIUM: OnceLock<Option<Pdfium>> = OnceLock::new();

/// Points Pdfium at the library file to load, which must be called before anything opens a PDF.
///
/// Recording the path rather than loading immediately keeps startup cheap for the many readers
/// who never open a PDF at all: the library is only pulled in once a document actually needs it.
pub fn set_library_path(path: &str) {
	if let Ok(mut guard) = LIBRARY_PATH.lock() {
		*guard = Some(path.to_string());
	}
}

/// The loaded Pdfium bindings, or `None` when the library could not be loaded.
///
/// Callers treat a failure here the same way they treat a corrupt file, so a reader without the
/// library installed gets "this document could not be opened" rather than a crash.
fn library() -> Option<&'static dyn PdfiumLibraryBindings> {
	PDFIUM.get_or_init(bind_library).as_ref().map(Pdfium::bindings)
}

fn bind_library() -> Option<Pdfium> {
	let configured = LIBRARY_PATH.lock().ok().and_then(|guard| guard.clone());
	let bindings = match configured {
		Some(path) => {
			let file = Pdfium::pdfium_platform_library_name_at_path(Path::new(&path));
			Pdfium::bind_to_library(&file).or_else(|error| {
				tracing::warn!(path = %file.display(), %error, "could not load pdfium from the configured path");
				Pdfium::bind_to_system_library()
			})
		}
		None => Pdfium::bind_to_system_library(),
	};
	match bindings {
		Ok(bindings) => Some(Pdfium::new(bindings)),
		Err(error) => {
			tracing::error!(%error, "pdfium could not be loaded; pdf support is unavailable");
			None
		}
	}
}

/// The bindings, for the handful of places that still speak raw FFI.
///
/// Panicking is not an option here and neither is returning `None` from every leaf call, so the
/// callers that reach for this have already established the library is loaded by holding an open
/// [`PdfDocument`], which cannot exist without it.
pub(crate) fn bindings() -> &'static dyn PdfiumLibraryBindings {
	library().expect("a document cannot be open unless pdfium loaded")
}

/// Why a PDF could not be read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PdfError {
	/// The file is encrypted and the password was missing or wrong. Kept separate from the rest
	/// because it is the one failure the reader can do something about, so the app prompts on it
	/// instead of reporting the file as broken.
	PasswordRequired,
	/// The library itself is missing, so no PDF can be read at all.
	LibraryUnavailable,
	/// The file could not be opened or parsed.
	CouldNotOpen,
	/// Pdfium refused a request on a document that did open, such as a page that will not load.
	Failed,
}

impl std::fmt::Display for PdfError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::PasswordRequired => write!(f, "the document is password protected"),
			Self::LibraryUnavailable => write!(f, "the pdfium library could not be loaded"),
			Self::CouldNotOpen => write!(f, "the document could not be opened"),
			Self::Failed => write!(f, "pdfium rejected the request"),
		}
	}
}

impl std::error::Error for PdfError {}

/// An open PDF.
///
/// Owns its Pdfium handle and closes it on drop. Unlike the page and tag types it carries no
/// lifetime, because the OCR renderer holds one open across many separate page renders.
pub struct PdfDocument {
	handle: FPDF_DOCUMENT,
	/// The bytes a from-memory document was read out of, held because pdfium keeps reading them
	/// for as long as the document is open. `None` for one opened from a path.
	_bytes: Option<Vec<u8>>,
}

// Pdfium's handles are not tied to the thread that made them as long as the bindings serialize
// access, which `pdfium-render`'s thread_safe wrapper does. Batch OCR needs this: it opens the
// document on the UI thread and renders pages on a worker.
unsafe impl Send for PdfDocument {}

impl PdfDocument {
	/// Opens `path`, decrypting with `password` when the file is encrypted.
	///
	/// # Errors
	///
	/// [`PdfError::PasswordRequired`] when the file is encrypted and the password does not fit,
	/// [`PdfError::LibraryUnavailable`] when Pdfium is missing, and [`PdfError::CouldNotOpen`]
	/// for a file that is not readable as a PDF.
	pub fn open(path: &str, password: Option<&str>) -> Result<Self, PdfError> {
		let Some(bindings) = library() else { return Err(PdfError::LibraryUnavailable) };
		let handle = bindings.FPDF_LoadDocument(path, password);
		if !handle.is_null() {
			return Ok(Self { handle, _bytes: None });
		}
		if bindings.FPDF_GetLastError() == FPDF_ERR_PASSWORD {
			Err(PdfError::PasswordRequired)
		} else {
			Err(PdfError::CouldNotOpen)
		}
	}

	/// Opens a PDF already in memory, which is how the repaired copy of a broken file is read.
	///
	/// Pdfium does not copy the bytes, it reads them where they lie for as long as the document
	/// is open, so the buffer is carried along in the returned value rather than borrowed. That
	/// is the whole reason this is a separate constructor and not a thin wrapper.
	///
	/// # Errors
	///
	/// The same failures as [`PdfDocument::open`].
	pub fn from_bytes(bytes: Vec<u8>, password: Option<&str>) -> Result<Self, PdfError> {
		let Some(bindings) = library() else { return Err(PdfError::LibraryUnavailable) };
		let handle = bindings.FPDF_LoadMemDocument64(&bytes, password);
		if handle.is_null() {
			return if bindings.FPDF_GetLastError() == FPDF_ERR_PASSWORD {
				Err(PdfError::PasswordRequired)
			} else {
				Err(PdfError::CouldNotOpen)
			};
		}
		Ok(Self { handle, _bytes: Some(bytes) })
	}

	pub fn page_count(&self) -> i32 {
		bindings().FPDF_GetPageCount(self.handle)
	}

	/// Loads one page, which stays borrowed from this document so it cannot outlive it.
	///
	/// # Errors
	///
	/// [`PdfError::Failed`] when the page will not load, which a damaged file can do for some of
	/// its pages while the rest read normally.
	pub fn page(&self, index: i32) -> Result<PdfPage<'_>, PdfError> {
		PdfPage::load(self, index)
	}

	/// One of the document's metadata fields, such as `Title` or `Author`.
	pub fn metadata(&self, key: &str) -> Option<String> {
		utf16_out_param(|buffer, len| bindings().FPDF_GetMetaText(self.handle, key, buffer, len))
	}

	/// The bookmark outline, flattened depth-first with the nesting recorded as a level.
	///
	/// A PDF's outline is a linked tree, and a damaged one can point back at itself. Rather than
	/// abandoning the whole table of contents when that happens, the walk stops descending the
	/// branch that loops and keeps everything it read on the way, because a partial contents list
	/// is far more use to a reader than none.
	pub fn outline(&self, max_depth: u32) -> Vec<OutlineEntry> {
		let mut entries = Vec::new();
		let mut seen = std::collections::HashSet::new();
		self.walk_outline(std::ptr::null_mut(), 0, max_depth, &mut seen, &mut entries);
		entries
	}

	fn walk_outline(
		&self,
		parent: FPDF_BOOKMARK,
		level: u32,
		max_depth: u32,
		seen: &mut std::collections::HashSet<usize>,
		entries: &mut Vec<OutlineEntry>,
	) {
		let bindings = bindings();
		let mut bookmark = bindings.FPDFBookmark_GetFirstChild(self.handle, parent);
		while !bookmark.is_null() {
			if !seen.insert(bookmark as usize) {
				return;
			}
			let title = utf16_out_param(|buffer, len| bindings.FPDFBookmark_GetTitle(bookmark, buffer, len));
			let destination = bindings.FPDFBookmark_GetDest(self.handle, bookmark);
			let page =
				if destination.is_null() { -1 } else { bindings.FPDFDest_GetDestPageIndex(self.handle, destination) };
			if let Some(title) = title
				&& page >= 0
			{
				entries.push(OutlineEntry { level, title, page });
			}
			if level + 1 < max_depth {
				self.walk_outline(bookmark, level + 1, max_depth, seen, entries);
			}
			bookmark = bindings.FPDFBookmark_GetNextSibling(self.handle, bookmark);
		}
	}

	pub(crate) fn handle(&self) -> FPDF_DOCUMENT {
		self.handle
	}
}

/// One entry in a PDF's bookmark outline.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutlineEntry {
	/// How deep this entry sits, counting from zero at the top.
	pub level: u32,
	pub title: String,
	/// The page it points at, zero-based. Entries pointing nowhere are left out entirely, so
	/// this is always a real page.
	pub page: i32,
}

impl Drop for PdfDocument {
	fn drop(&mut self) {
		bindings().FPDF_CloseDocument(self.handle);
	}
}

/// Reads one of Pdfium's UTF-16LE string out-params.
///
/// Every string getter in the C API works the same way: call it with no buffer to be told how
/// many bytes the text needs, allocate that, then call it again to have it filled. The trailing
/// NUL is counted in that length but is not part of the text, so it is dropped here rather than
/// in every caller.
pub(crate) fn utf16_out_param(mut fill: impl FnMut(*mut c_void, c_ulong) -> c_ulong) -> Option<String> {
	let len = fill(std::ptr::null_mut(), 0);
	if len == 0 {
		return None;
	}
	let mut buffer = vec![0u8; len as usize];
	fill(buffer.as_mut_ptr().cast(), len);
	let utf16: Vec<u16> =
		buffer.as_chunks::<2>().0.iter().map(|&pair| u16::from_le_bytes(pair)).take_while(|&unit| unit != 0).collect();
	if utf16.is_empty() {
		return None;
	}
	Some(String::from_utf16_lossy(&utf16))
}

/// Reads one of Pdfium's UTF-8 string out-params, which the URI getters use in place of the
/// UTF-16 the rest of the API returns.
pub(crate) fn utf8_out_param(mut fill: impl FnMut(*mut c_void, c_ulong) -> c_ulong) -> Option<String> {
	let len = fill(std::ptr::null_mut(), 0);
	if len == 0 {
		return None;
	}
	let mut buffer = vec![0u8; len as usize];
	fill(buffer.as_mut_ptr().cast(), len);
	let end = buffer.iter().position(|&byte| byte == 0).unwrap_or(buffer.len());
	if end == 0 {
		return None;
	}
	Some(String::from_utf8_lossy(&buffer[..end]).into_owned())
}

#[cfg(test)]
mod tests {
	use super::*;

	/// The dance this helper exists to hide: the first call reports the byte length, the second
	/// fills the buffer, and the NUL that the length counts is not part of the string.
	#[test]
	fn a_utf16_out_param_is_read_in_two_calls_without_its_terminator() {
		let text: Vec<u8> = "Heading\0".encode_utf16().flat_map(u16::to_le_bytes).collect();
		// `c_ulong`, not a fixed width: pdfium's lengths are C unsigned longs, which are 32 bits
		// on Windows and 64 everywhere else.
		let len = c_ulong::try_from(text.len()).unwrap();
		let read = utf16_out_param(|buffer, size| {
			if buffer.is_null() {
				return len;
			}
			unsafe { std::ptr::copy_nonoverlapping(text.as_ptr(), buffer.cast::<u8>(), size as usize) };
			len
		});
		assert_eq!(Some("Heading".to_string()), read);
	}

	/// A zero length means the field is absent, and must not allocate or return an empty string
	/// that callers would then have to check for themselves.
	#[test]
	fn a_field_that_is_not_there_reads_as_none() {
		assert_eq!(None, utf16_out_param(|_, _| 0));
	}

	/// Some fields come back as nothing but a terminator, which is also absence.
	#[test]
	fn a_string_of_only_a_terminator_reads_as_none() {
		let read = utf16_out_param(|buffer, _| {
			if !buffer.is_null() {
				unsafe { std::ptr::write_bytes(buffer.cast::<u8>(), 0, 2) };
			}
			2
		});
		assert_eq!(None, read);
	}
}
