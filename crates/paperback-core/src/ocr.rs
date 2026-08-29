//! Helpers for OCR of image-only PDF pages.
//!
//! The PDF parser emits an [`IMAGE_ONLY_PLACEHOLDER`] line for every page that contains an
//! image but no extractable text. At runtime the desktop app renders that page to an RGBA
//! bitmap with [`render_pdf_page`] and hands the pixels to the Windows OCR engine (the `WinRT`
//! `Windows.Media.Ocr` API, reached through the `windows` crate in the desktop crate — this
//! crate stays platform-neutral). The recognized text then replaces the placeholder in the
//! document buffer.

use anyhow::Result;
use pdfium::{PdfiumDocument, PdfiumRenderConfig};

/// The placeholder line the PDF parser inserts for a page that has an image but no extractable
/// text. The UI matches the caret's line against `t(IMAGE_ONLY_PLACEHOLDER)` to decide whether
/// Enter should run OCR, so both the parser and the desktop crate must use this same message id.
pub const IMAGE_ONLY_PLACEHOLDER: &str = "[Image only. Press enter to OCR.]";

/// An RGBA8 page render ready to hand to the Windows OCR engine.
#[derive(Debug)]
pub struct RenderedPage {
	pub width: u32,
	pub height: u32,
	pub rgba: Vec<u8>,
}

/// Renders PDF page `page_index` (0-based) to an RGBA8 bitmap at a resolution good enough for
/// OCR (about 1800 px across; the Windows OCR engine downscales to its own maximum dimension
/// when a page exceeds it).
///
/// Runs on the caller's thread — the desktop app calls it on the UI thread before handing the
/// pixels to the OCR worker thread, because pdfium's binding is a process-global and the UI
/// thread is the only place pdfium is otherwise used.
pub fn render_pdf_page(file_path: &str, password: Option<&str>, page_index: i32) -> Result<RenderedPage> {
	let document = PdfiumDocument::new_from_path(file_path, password)?;
	let page = document.page(page_index)?;
	let bitmap = page.render(&PdfiumRenderConfig::new().with_width(1800))?;
	let (width, height) = (bitmap.width(), bitmap.height());
	let rgba = bitmap.as_rgba_bytes()?;
	Ok(RenderedPage { width: u32::try_from(width).unwrap_or(0), height: u32::try_from(height).unwrap_or(0), rgba })
}
