//! Page rendering for OCR of image-only PDF pages.
//!
//! The PDF parser tags every page that has an image but no extractable text with a
//! [`MarkerType::ImageOnlyPage`](crate::document::MarkerType::ImageOnlyPage) marker and an
//! [`image_only_placeholder`] line. At runtime the desktop app renders those pages to RGBA
//! bitmaps through [`PageRenderer`] and hands the pixels to the platform OCR engine
//! (`Windows.Media.Ocr`, or Vision on macOS), which lives in the desktop crate so this one
//! stays platform-neutral. The recognized text then replaces the placeholder in the buffer.

use anyhow::Result;
use pdfium::{PdfiumDocument, PdfiumRenderConfig};

use crate::t;

/// The placeholder line the PDF parser inserts for a page that has an image but no extractable
/// text. Purely what the reader sees: the OCR flow locates these pages by their
/// `ImageOnlyPage` marker, so this string is free to be translated and to change.
///
/// Written as literal [`t`] calls rather than a translated constant because the pot is built by
/// `xgettext`, which only sees string literals; `t(SOME_CONST)` extracts nothing and the message
/// silently stays untranslated in every language.
///
/// Builds with no OCR engine say less, because there is nothing for Enter to do there. The
/// marker is emitted either way, so only the wording changes.
#[must_use]
pub fn image_only_placeholder() -> String {
	#[cfg(any(target_os = "windows", target_os = "macos"))]
	{
		// TRANSLATORS: Line shown in place of a scanned PDF page with no text layer; Enter on it runs OCR
		t("[Image only. Press enter to OCR.]")
	}
	#[cfg(not(any(target_os = "windows", target_os = "macos")))]
	{
		// TRANSLATORS: Line shown in place of a scanned PDF page with no text layer, where the system has no OCR engine
		t("[Image only. This page has no text to read.]")
	}
}

/// Dots per inch to render at. OCR engines want 200 to 300 DPI; below that, small type starts
/// dropping characters. PDF user units are points (1/72 inch), which is what the ratio below
/// converts from.
const TARGET_DPI: f32 = 300.0;
const POINTS_PER_INCH: f32 = 72.0;

/// Fallback pixel width for a page whose reported size is unusable (zero or negative).
const FALLBACK_WIDTH: i32 = 1800;

/// An RGBA8 page render ready to hand to a platform OCR engine.
#[derive(Debug)]
pub struct RenderedPage {
	pub width: u32,
	pub height: u32,
	pub rgba: Vec<u8>,
}

/// A PDF held open for repeated page rendering.
///
/// Batch OCR renders many pages from one file, so the document is opened once here rather than
/// per page. Not `Send`: pdfium's handles are tied to the thread that made them, so a renderer
/// is created and used on the same thread. That thread need not be the UI thread, because every
/// call into pdfium takes the library's own global lock.
pub struct PageRenderer {
	document: PdfiumDocument,
}

impl PageRenderer {
	/// Opens `file_path` for rendering.
	///
	/// # Errors
	///
	/// Returns an error if pdfium cannot open the file, including a wrong or missing password.
	pub fn open(file_path: &str, password: Option<&str>) -> Result<Self> {
		Ok(Self { document: PdfiumDocument::new_from_path(file_path, password)? })
	}

	/// Renders page `page_index` (0-based) to an RGBA8 bitmap at [`TARGET_DPI`], scaled down to
	/// keep its longest side within `max_dimension` (engines silently downscale larger inputs
	/// themselves, and doing it here keeps the buffer smaller).
	///
	/// # Errors
	///
	/// Returns an error if the page is out of range or pdfium fails to rasterize it.
	pub fn render(&self, page_index: i32, max_dimension: u32) -> Result<RenderedPage> {
		let page = self.document.page(page_index)?;
		let width = pixel_width(page.width(), page.height(), max_dimension);
		let bitmap = page.render(&PdfiumRenderConfig::new().with_width(width))?;
		let (width, height) = (bitmap.width(), bitmap.height());
		let rgba = bitmap.as_rgba_bytes()?;
		Ok(RenderedPage { width: u32::try_from(width).unwrap_or(0), height: u32::try_from(height).unwrap_or(0), rgba })
	}
}

/// The pixel width to rasterize a page of `width_pt` by `height_pt` at [`TARGET_DPI`], reduced
/// so neither side exceeds `max_dimension`.
#[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss, clippy::cast_sign_loss)]
fn pixel_width(width_pt: f32, height_pt: f32, max_dimension: u32) -> i32 {
	if width_pt <= 0.0 || height_pt <= 0.0 {
		return FALLBACK_WIDTH;
	}
	let scale = TARGET_DPI / POINTS_PER_INCH;
	let longest = width_pt.max(height_pt) * scale;
	let capped = if longest > max_dimension as f32 { max_dimension as f32 / longest } else { 1.0 };
	((width_pt * scale * capped).round() as i32).max(1)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn pixel_width_renders_letter_at_target_dpi() {
		// 8.5 by 11 inches in points, comfortably inside the cap.
		assert_eq!(pixel_width(612.0, 792.0, 4000), 2550);
	}

	#[test]
	fn pixel_width_scales_down_to_the_cap() {
		// The same page at a 2600 px cap: the 11 inch side is the one that binds.
		assert_eq!(pixel_width(612.0, 792.0, 2600), 2009);
	}

	#[test]
	fn pixel_width_falls_back_for_an_unusable_page_size() {
		assert_eq!(pixel_width(0.0, 792.0, 2600), FALLBACK_WIDTH);
		assert_eq!(pixel_width(612.0, -1.0, 2600), FALLBACK_WIDTH);
	}
}
