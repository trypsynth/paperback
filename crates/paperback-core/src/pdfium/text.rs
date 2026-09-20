//! A page's text layer: the glyphs, where each one sits, and what font it is set in.
//!
//! Pdfium exposes this a character at a time, and almost every getter answers through an
//! out-param rather than a return value. The wrappers here return what was asked for instead,
//! so the parser reads as arithmetic over positions rather than as a sequence of buffer
//! preparations.

use std::ffi::{c_int, c_ulong, c_void};

use pdfium_render::prelude::{FPDF_PAGELINK, FPDF_TEXTPAGE, FS_MATRIX};

use super::{PageObject, PdfPage, bindings};

/// The box one glyph occupies on the page, in PDF points.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CharBox {
	pub left: f64,
	pub right: f64,
	pub bottom: f64,
	pub top: f64,
}

/// The text matrix in force for one glyph, which is what turns a nominal font size into the
/// size the glyph is actually drawn at.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TextMatrix {
	pub a: f32,
	pub b: f32,
	pub c: f32,
	pub d: f32,
	pub e: f32,
	pub f: f32,
}

/// A glyph's font, as far as deciding whether it is monospaced needs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FontInfo {
	pub name: String,
	/// The PDF font descriptor flags, whose fixed-pitch bit is more reliable than the name.
	pub flags: i32,
}

/// A page's text, borrowed from the page it belongs to.
pub struct PdfTextPage<'page> {
	handle: FPDF_TEXTPAGE,
	_page: std::marker::PhantomData<&'page PdfPage<'page>>,
}

impl<'page> PdfTextPage<'page> {
	pub(super) fn of(page: &'page PdfPage<'page>) -> Option<Self> {
		let handle = bindings().FPDFText_LoadPage(page.handle());
		if handle.is_null() {
			return None;
		}
		Some(Self { handle, _page: std::marker::PhantomData })
	}

	/// How many characters the page holds, or `None` when pdfium cannot say.
	///
	/// Zero and "cannot say" are different answers and the caller acts differently on each: a
	/// page of pure picture really has no characters, while a count pdfium refuses means the
	/// per-character reading is not available and the whole page has to be taken as one blob.
	pub fn char_count(&self) -> Option<i32> {
		let count = bindings().FPDFText_CountChars(self.handle);
		(count >= 0).then_some(count)
	}

	/// The whole page as one string.
	pub fn text(&self) -> String {
		self.extract(0, self.char_count().unwrap_or(0))
	}

	/// `count` characters starting at `start`.
	pub fn extract(&self, start: i32, count: i32) -> String {
		if count <= 0 {
			return String::new();
		}
		// One extra for the terminator pdfium always writes.
		let mut buffer = vec![0u16; count as usize + 1];
		let written = bindings().FPDFText_GetText(self.handle, start, count, buffer.as_mut_ptr());
		if written <= 0 {
			return String::new();
		}
		let end = (written as usize).saturating_sub(1).min(buffer.len());
		String::from_utf16_lossy(&buffer[..end])
	}

	pub fn unicode_at(&self, index: i32) -> u32 {
		bindings().FPDFText_GetUnicode(self.handle, index)
	}

	/// Where the glyph at `index` sits, as the point its baseline starts from.
	pub fn char_origin(&self, index: i32) -> Option<(f64, f64)> {
		let mut x = 0.0;
		let mut y = 0.0;
		let ok = bindings().FPDFText_GetCharOrigin(self.handle, index, &mut x, &mut y);
		(ok != 0).then_some((x, y))
	}

	pub fn char_box(&self, index: i32) -> Option<CharBox> {
		let (mut left, mut right, mut bottom, mut top) = (0.0, 0.0, 0.0, 0.0);
		let ok = bindings().FPDFText_GetCharBox(self.handle, index, &mut left, &mut right, &mut bottom, &mut top);
		(ok != 0).then_some(CharBox { left, right, bottom, top })
	}

	pub fn font_size(&self, index: i32) -> f64 {
		bindings().FPDFText_GetFontSize(self.handle, index)
	}

	pub fn text_matrix(&self, index: i32) -> Option<TextMatrix> {
		let mut matrix = FS_MATRIX { a: 0.0, b: 0.0, c: 0.0, d: 0.0, e: 0.0, f: 0.0 };
		let ok = bindings().FPDFText_GetMatrix(self.handle, index, &mut matrix);
		(ok != 0).then_some(TextMatrix { a: matrix.a, b: matrix.b, c: matrix.c, d: matrix.d, e: matrix.e, f: matrix.f })
	}

	/// The font the glyph at `index` is set in.
	///
	/// The length pdfium reports counts the trailing NUL and describes what the name would need
	/// rather than what fitted, so a name longer than the buffer is reported as absent rather
	/// than as a truncation that would then be matched against.
	pub fn font(&self, index: i32) -> Option<FontInfo> {
		const NAME_LIMIT: usize = 128;
		let mut buffer = [0u8; NAME_LIMIT];
		let mut flags: c_int = 0;
		let capacity = c_ulong::try_from(buffer.len()).unwrap_or(c_ulong::MAX);
		let written = bindings().FPDFText_GetFontInfo(
			self.handle,
			index,
			buffer.as_mut_ptr().cast::<c_void>(),
			capacity,
			&mut flags,
		) as usize;
		let end = written.saturating_sub(1);
		if end == 0 || end > buffer.len() {
			// No name, or one too long to have been written; the flags are still worth having.
			return Some(FontInfo { name: String::new(), flags });
		}
		Some(FontInfo { name: String::from_utf8_lossy(&buffer[..end]).into_owned(), flags })
	}

	/// Whether pdfium invented this character rather than finding it in the content stream, as
	/// it does for the spaces it infers between words that are merely far apart.
	pub fn is_generated(&self, index: i32) -> bool {
		bindings().FPDFText_IsGenerated(self.handle, index) == 1
	}

	/// The content-stream object that drew the glyph at `index`, which is what carries the
	/// marked-content id linking it back to the tag tree.
	pub fn text_object(&self, index: i32) -> Option<PageObject<'_>> {
		PageObject::of(bindings().FPDFText_GetTextObject(self.handle, index))
	}

	/// The text inside a rectangle on the page, which is how a link annotation's words are read.
	pub fn bounded_text(&self, left: f64, top: f64, right: f64, bottom: f64) -> String {
		let needed = bindings().FPDFText_GetBoundedText(self.handle, left, top, right, bottom, std::ptr::null_mut(), 0);
		if needed <= 0 {
			return String::new();
		}
		let mut buffer = vec![0u16; needed as usize];
		let written =
			bindings().FPDFText_GetBoundedText(self.handle, left, top, right, bottom, buffer.as_mut_ptr(), needed);
		if written <= 0 {
			return String::new();
		}
		let end = (written as usize).min(buffer.len());
		let text: Vec<u16> = buffer[..end].iter().copied().take_while(|&unit| unit != 0).collect();
		String::from_utf16_lossy(&text)
	}

	/// The URLs pdfium can find in the page's text, with the character range each covers.
	pub fn web_links(&self) -> Vec<(i32, i32, String)> {
		let handle = bindings().FPDFLink_LoadWebLinks(self.handle);
		if handle.is_null() {
			return Vec::new();
		}
		let links = WebLinks { handle };
		let count = bindings().FPDFLink_CountWebLinks(links.handle);
		let mut found = Vec::new();
		for index in 0..count {
			let mut start = 0;
			let mut char_count = 0;
			if bindings().FPDFLink_GetTextRange(links.handle, index, &mut start, &mut char_count) == 0 {
				continue;
			}
			let mut url = vec![0u16; 2048];
			let len = bindings().FPDFLink_GetURL(links.handle, index, url.as_mut_ptr(), 2048);
			if len <= 0 {
				continue;
			}
			let end = (len as usize).saturating_sub(1).min(url.len());
			found.push((start, char_count, String::from_utf16_lossy(&url[..end])));
		}
		found
	}
}

impl Drop for PdfTextPage<'_> {
	fn drop(&mut self) {
		bindings().FPDFText_ClosePage(self.handle);
	}
}

/// Holds the web-link handle only long enough to read it, so an early return still closes it.
pub struct WebLinks {
	handle: FPDF_PAGELINK,
}

impl Drop for WebLinks {
	fn drop(&mut self) {
		bindings().FPDFLink_CloseWebLinks(self.handle);
	}
}
