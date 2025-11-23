use std::{
	ffi::{CStr, CString, c_void},
	ptr,
};

use anyhow::{Result, anyhow, bail};

use crate::document::TocItem;

pub struct PdfiumLibrary;

impl PdfiumLibrary {
	pub fn new() -> Self {
		unsafe {
			ffi::FPDF_InitLibrary();
		}
		Self
	}
}

impl Drop for PdfiumLibrary {
	fn drop(&mut self) {
		unsafe {
			ffi::FPDF_DestroyLibrary();
		}
	}
}

pub struct PdfDocument {
	handle: ffi::FPDF_DOCUMENT,
}

impl PdfDocument {
	pub fn load(path: &str, password: Option<&str>) -> Result<Self> {
		let path_cstr = CString::new(path).map_err(|_| anyhow!("PDF path contains embedded NUL bytes"))?;
		let password_cstr = match password {
			Some(pwd) if !pwd.is_empty() => {
				Some(CString::new(pwd).map_err(|_| anyhow!("PDF password contains embedded NUL bytes"))?)
			}
			_ => None,
		};
		let handle = unsafe {
			ffi::FPDF_LoadDocument(path_cstr.as_ptr(), password_cstr.as_ref().map_or(ptr::null(), |pwd| pwd.as_ptr()))
		};
		if handle.is_null() {
			return Err(map_pdfium_error("Failed to open PDF document"));
		}
		Ok(Self { handle })
	}

	pub fn page_count(&self) -> Result<i32> {
		let count = unsafe { ffi::FPDF_GetPageCount(self.handle) };
		if count < 0 {
			bail!("Failed to read page count");
		}
		Ok(count)
	}

	pub fn load_page(&self, index: i32) -> Option<PdfPage> {
		let handle = unsafe { ffi::FPDF_LoadPage(self.handle, index) };
		if handle.is_null() { None } else { Some(PdfPage { handle }) }
	}

	pub fn extract_metadata(&self, tag: &[u8]) -> Option<String> {
		let tag_cstr = CStr::from_bytes_with_nul(tag).ok()?;
		let length = unsafe { ffi::FPDF_GetMetaText(self.handle, tag_cstr.as_ptr(), ptr::null_mut(), 0) };
		if length <= 2 {
			return None;
		}
		let mut buffer = vec![0u16; length as usize / 2];
		let written = unsafe {
			ffi::FPDF_GetMetaText(self.handle, tag_cstr.as_ptr(), buffer.as_mut_ptr().cast::<c_void>(), length)
		};
		if written <= 2 {
			return None;
		}
		sanitize_utf16_buffer(&buffer, written)
	}

	pub fn extract_toc(&self, page_offsets: &[usize]) -> Vec<TocItem> {
		let first = unsafe { ffi::FPDFBookmark_GetFirstChild(self.handle, ptr::null_mut()) };
		if first.is_null() {
			return Vec::new();
		}
		extract_outline_items(self.handle, first, page_offsets)
	}
}

impl Drop for PdfDocument {
	fn drop(&mut self) {
		if !self.handle.is_null() {
			unsafe {
				ffi::FPDF_CloseDocument(self.handle);
			}
		}
	}
}

pub struct PdfPage {
	handle: ffi::FPDF_PAGE,
}

impl PdfPage {
	pub fn load_text_page(&self) -> Option<PdfTextPage> {
		let handle = unsafe { ffi::FPDFText_LoadPage(self.handle) };
		if handle.is_null() { None } else { Some(PdfTextPage { handle }) }
	}
}

impl Drop for PdfPage {
	fn drop(&mut self) {
		if !self.handle.is_null() {
			unsafe {
				ffi::FPDF_ClosePage(self.handle);
			}
		}
	}
}

pub struct PdfTextPage {
	handle: ffi::FPDF_TEXTPAGE,
}

impl PdfTextPage {
	pub fn extract_text(&self) -> String {
		let char_count = unsafe { ffi::FPDFText_CountChars(self.handle) };
		if char_count <= 0 {
			return String::new();
		}
		let buffer_size = usize::try_from(char_count + 1).unwrap_or(0);
		if buffer_size == 0 {
			return String::new();
		}
		let mut buffer = vec![0u16; buffer_size];
		let written = unsafe { ffi::FPDFText_GetText(self.handle, 0, char_count, buffer.as_mut_ptr()) };
		if written <= 1 {
			return String::new();
		}
		let actual_len = usize::try_from(written).unwrap_or(0).saturating_sub(1);
		buffer.truncate(actual_len);
		String::from_utf16_lossy(&buffer)
	}
}

impl Drop for PdfTextPage {
	fn drop(&mut self) {
		if !self.handle.is_null() {
			unsafe {
				ffi::FPDFText_ClosePage(self.handle);
			}
		}
	}
}

fn extract_outline_items(
	document: ffi::FPDF_DOCUMENT,
	mut bookmark: ffi::FPDF_BOOKMARK,
	page_offsets: &[usize],
) -> Vec<TocItem> {
	let mut items = Vec::new();
	while !bookmark.is_null() {
		let name = read_bookmark_title(bookmark).unwrap_or_default();
		let offset = unsafe {
			let dest = ffi::FPDFBookmark_GetDest(document, bookmark);
			if dest.is_null() {
				usize::MAX
			} else {
				let page_index = ffi::FPDFDest_GetDestPageIndex(document, dest);
				if page_index < 0 {
					usize::MAX
				} else {
					usize::try_from(page_index)
						.ok()
						.and_then(|idx| page_offsets.get(idx).copied())
						.unwrap_or(usize::MAX)
				}
			}
		};
		let mut toc_item = TocItem::new(name, String::new(), offset);
		let child = unsafe { ffi::FPDFBookmark_GetFirstChild(document, bookmark) };
		if !child.is_null() {
			toc_item.children = extract_outline_items(document, child, page_offsets);
		}
		items.push(toc_item);
		bookmark = unsafe { ffi::FPDFBookmark_GetNextSibling(document, bookmark) };
	}
	items
}

fn read_bookmark_title(bookmark: ffi::FPDF_BOOKMARK) -> Option<String> {
	let length = unsafe { ffi::FPDFBookmark_GetTitle(bookmark, ptr::null_mut(), 0) };
	if length <= 2 {
		return None;
	}
	let mut buffer = vec![0u16; length as usize / 2];
	let written = unsafe { ffi::FPDFBookmark_GetTitle(bookmark, buffer.as_mut_ptr().cast::<c_void>(), length) };
	if written <= 2 {
		return None;
	}
	sanitize_utf16_buffer(&buffer, written)
}

fn sanitize_utf16_buffer(buffer: &[u16], written_bytes: u32) -> Option<String> {
	let total_units = (written_bytes as usize / 2).saturating_sub(1);
	if total_units == 0 {
		return None;
	}
	buffer.get(..total_units).map(String::from_utf16_lossy)
}

fn map_pdfium_error(default_message: &str) -> anyhow::Error {
	use crate::parser::PASSWORD_REQUIRED_ERROR_PREFIX;
	let last_error = unsafe { ffi::FPDF_GetLastError() };
	match last_error {
		ffi::FPDF_ERR_PASSWORD => anyhow!("{PASSWORD_REQUIRED_ERROR_PREFIX}Password required or incorrect"),
		code if code != 0 => anyhow!("{default_message} (PDFium error code {code})"),
		_ => anyhow!("{default_message}"),
	}
}

mod ffi {
	#![allow(non_camel_case_types)]

	use std::ffi::c_void;

	pub type FPDF_DOCUMENT = *mut c_void;
	pub type FPDF_PAGE = *mut c_void;
	pub type FPDF_TEXTPAGE = *mut c_void;
	pub type FPDF_BOOKMARK = *mut c_void;
	pub type FPDF_DEST = *mut c_void;

	pub const FPDF_ERR_PASSWORD: u32 = 4;

	#[link(name = "pdfium")]
	unsafe extern "C" {
		pub fn FPDF_InitLibrary();
		pub fn FPDF_DestroyLibrary();
		pub fn FPDF_LoadDocument(file_path: *const i8, password: *const i8) -> FPDF_DOCUMENT;
		pub fn FPDF_CloseDocument(document: FPDF_DOCUMENT);
		pub fn FPDF_GetLastError() -> u32;
		pub fn FPDF_GetPageCount(document: FPDF_DOCUMENT) -> i32;
		pub fn FPDF_LoadPage(document: FPDF_DOCUMENT, page_index: i32) -> FPDF_PAGE;
		pub fn FPDF_ClosePage(page: FPDF_PAGE);
		pub fn FPDFText_LoadPage(page: FPDF_PAGE) -> FPDF_TEXTPAGE;
		pub fn FPDFText_ClosePage(text_page: FPDF_TEXTPAGE);
		pub fn FPDFText_CountChars(text_page: FPDF_TEXTPAGE) -> i32;
		pub fn FPDFText_GetText(text_page: FPDF_TEXTPAGE, start_index: i32, count: i32, result: *mut u16) -> i32;
		pub fn FPDF_GetMetaText(document: FPDF_DOCUMENT, tag: *const i8, buffer: *mut c_void, buflen: u32) -> u32;
		pub fn FPDFBookmark_GetFirstChild(document: FPDF_DOCUMENT, bookmark: FPDF_BOOKMARK) -> FPDF_BOOKMARK;
		pub fn FPDFBookmark_GetNextSibling(document: FPDF_DOCUMENT, bookmark: FPDF_BOOKMARK) -> FPDF_BOOKMARK;
		pub fn FPDFBookmark_GetTitle(bookmark: FPDF_BOOKMARK, buffer: *mut c_void, buflen: u32) -> u32;
		pub fn FPDFBookmark_GetDest(document: FPDF_DOCUMENT, bookmark: FPDF_BOOKMARK) -> FPDF_DEST;
		pub fn FPDFDest_GetDestPageIndex(document: FPDF_DOCUMENT, dest: FPDF_DEST) -> i32;
	}
}
