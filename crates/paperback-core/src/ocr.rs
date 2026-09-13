//! Page rendering for OCR of pages that carry a picture and no text.
//!
//! The PDF parser tags every page that has an image but no extractable text with a
//! [`MarkerType::ImageOnlyPage`](crate::document::MarkerType::ImageOnlyPage) marker and an
//! [`image_only_placeholder`] line, and the comic archive parser tags every page that way,
//! since none of them holds text at all. At runtime the desktop app renders those pages to
//! RGBA bitmaps through [`PageRenderer`] and hands the pixels to the platform OCR engine
//! (`Windows.Media.Ocr`, or Vision on macOS), which lives in the desktop crate so this one
//! stays platform-neutral. The recognized text then replaces the placeholder in the buffer.

use std::{fs::File, io::BufReader, path::Path};

use anyhow::{Context, Result};
use pdfium::{PdfiumDocument, PdfiumRenderConfig};
use zip::ZipArchive;

use crate::{parser::cbz, t};

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

/// A document held open for repeated page rendering.
///
/// Batch OCR renders many pages from one file, so the file is opened once here rather than per
/// page. Not `Send`: pdfium's handles are tied to the thread that made them, so a renderer is
/// created and used on the same thread. That thread need not be the UI thread, because every
/// call into pdfium takes the library's own global lock.
pub struct PageRenderer {
	source: Source,
}

/// What the renderer draws pages out of. A PDF page has to be rasterized; a comic archive's
/// page is already a picture and only needs decoding.
enum Source {
	Pdf(PdfiumDocument),
	Comic { archive: ZipArchive<BufReader<File>>, pages: Vec<String> },
}

impl PageRenderer {
	/// Opens `file_path` for rendering, choosing the backend from its extension.
	///
	/// # Errors
	///
	/// Returns an error if the file cannot be opened, including a wrong or missing PDF
	/// password, or if a comic archive holds no pages.
	pub fn open(file_path: &str, password: Option<&str>) -> Result<Self> {
		if Path::new(file_path).extension().is_some_and(|e| e.eq_ignore_ascii_case("cbz")) {
			return Self::open_comic(file_path);
		}
		Ok(Self { source: Source::Pdf(PdfiumDocument::new_from_path(file_path, password)?) })
	}

	fn open_comic(file_path: &str) -> Result<Self> {
		let file = File::open(file_path).with_context(|| format!("Failed to open '{file_path}'"))?;
		let mut archive = ZipArchive::new(BufReader::new(file))?;
		// Numbered exactly as the parser numbered them, so page N here is the page the
		// reader is sitting on.
		let pages = cbz::page_names(&mut archive);
		if pages.is_empty() {
			anyhow::bail!("comic archive '{file_path}' has no pages to render");
		}
		Ok(Self { source: Source::Comic { archive, pages } })
	}

	/// Renders page `page_index` (0-based) to an RGBA8 bitmap, scaled down to keep its longest
	/// side within `max_dimension` (engines silently downscale larger inputs themselves, and
	/// doing it here keeps the buffer smaller). A PDF page is rasterized at [`TARGET_DPI`];
	/// a comic archive's page is decoded at whatever resolution it was scanned.
	///
	/// # Errors
	///
	/// Returns an error if the page is out of range, or cannot be rasterized or decoded.
	pub fn render(&mut self, page_index: i32, max_dimension: u32) -> Result<RenderedPage> {
		match &mut self.source {
			Source::Pdf(document) => {
				let page = document.page(page_index)?;
				let width = pixel_width(page.width(), page.height(), max_dimension);
				let bitmap = page.render(&PdfiumRenderConfig::new().with_width(width))?;
				let (width, height) = (bitmap.width(), bitmap.height());
				let rgba = bitmap.as_rgba_bytes()?;
				Ok(RenderedPage {
					width: u32::try_from(width).unwrap_or(0),
					height: u32::try_from(height).unwrap_or(0),
					rgba,
				})
			}
			Source::Comic { archive, pages } => {
				let index = usize::try_from(page_index).unwrap_or(usize::MAX);
				let name = pages.get(index).ok_or_else(|| anyhow::anyhow!("page {page_index} is out of range"))?;
				let mut entry = archive.by_name(name)?;
				let mut bytes = Vec::new();
				std::io::copy(&mut entry, &mut bytes)?;
				decode_page(&bytes, max_dimension)
			}
		}
	}
}

/// Decode one comic page and shrink it to fit `max_dimension`.
fn decode_page(bytes: &[u8], max_dimension: u32) -> Result<RenderedPage> {
	let image = image::load_from_memory(bytes).context("failed to decode a comic archive page")?;
	// Scanned artwork is routinely bigger than the engines accept, and they downscale it
	// themselves anyway, so shrinking here only saves carrying the pixels around.
	let longest = image.width().max(image.height());
	let image = if max_dimension > 0 && longest > max_dimension {
		image.thumbnail(max_dimension, max_dimension)
	} else {
		image
	};
	let rgba = image.into_rgba8();
	Ok(RenderedPage { width: rgba.width(), height: rgba.height(), rgba: rgba.into_raw() })
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
	use std::{env, fs, io::Write as _};

	use zip::{ZipWriter, write::SimpleFileOptions};

	use super::*;

	/// A greyscale PNG of `width` by `height`, all white. Small but a real decodable image.
	fn png(width: u32, height: u32) -> Vec<u8> {
		let mut buffer = Vec::new();
		let encoder = image::codecs::png::PngEncoder::new(&mut buffer);
		let pixels = vec![255u8; (width * height) as usize];
		image::ImageEncoder::write_image(encoder, &pixels, width, height, image::ExtendedColorType::L8).unwrap();
		buffer
	}

	/// Write a comic archive holding `pages`, each `(name, width, height)`, and return its path.
	fn comic(test_name: &str, pages: &[(&str, u32, u32)]) -> String {
		let dir = env::temp_dir().join("paperback-cbz-render-tests");
		fs::create_dir_all(&dir).unwrap();
		let path = dir.join(format!("{test_name}.cbz"));
		let file = fs::File::create(&path).unwrap();
		let mut zip = ZipWriter::new(file);
		for (name, width, height) in pages {
			zip.start_file(*name, SimpleFileOptions::default()).unwrap();
			zip.write_all(&png(*width, *height)).unwrap();
		}
		zip.finish().unwrap();
		path.to_string_lossy().into_owned()
	}

	// The reader numbers comic pages in natural order, so the renderer has to agree or
	// running OCR on page 2 recognizes some other page entirely.
	#[test]
	fn a_comic_page_renders_in_the_order_the_reader_numbers_them() {
		let path = comic("order", &[("p10.png", 30, 10), ("p2.png", 20, 10), ("p1.png", 10, 10)]);
		let mut renderer = PageRenderer::open(&path, None).unwrap();
		assert_eq!(renderer.render(0, 4000).unwrap().width, 10);
		assert_eq!(renderer.render(1, 4000).unwrap().width, 20);
		assert_eq!(renderer.render(2, 4000).unwrap().width, 30);
	}

	#[test]
	fn a_rendered_comic_page_comes_back_as_rgba() {
		let path = comic("rgba", &[("001.png", 8, 4)]);
		let mut renderer = PageRenderer::open(&path, None).unwrap();
		let page = renderer.render(0, 4000).unwrap();
		assert_eq!((page.width, page.height), (8, 4));
		assert_eq!(page.rgba.len(), 8 * 4 * 4);
	}

	// Scanned artwork is routinely larger than the OCR engines accept, so an oversized page
	// has to come back shrunk rather than as tens of megabytes of pixels.
	#[test]
	fn an_oversized_comic_page_is_shrunk_to_the_cap() {
		let path = comic("cap", &[("001.png", 400, 200)]);
		let mut renderer = PageRenderer::open(&path, None).unwrap();
		let page = renderer.render(0, 100).unwrap();
		assert_eq!(page.width, 100);
		assert_eq!(page.height, 50);
	}

	#[test]
	fn a_page_past_the_end_of_a_comic_is_an_error() {
		let path = comic("range", &[("001.png", 8, 4)]);
		let mut renderer = PageRenderer::open(&path, None).unwrap();
		assert!(renderer.render(1, 4000).is_err());
		assert!(renderer.render(-1, 4000).is_err());
	}

	// A zip with nothing but metadata in it has no pages to OCR, and saying so beats
	// handing the engine an empty bitmap.
	#[test]
	fn a_comic_with_no_pages_will_not_open() {
		let path = comic("empty", &[("ComicInfo.xml", 1, 1)]);
		assert!(PageRenderer::open(&path, None).is_err());
	}

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
