use std::{fs::File, io::BufReader, path::Path};

use anyhow::{Context, Result};
use zip::ZipArchive;

use crate::{
	parser::{cbr, cbz},
	pdf_engine::PdfDocument,
	t,
};

mod layout;
mod pool;
mod saved;

pub use layout::{RecognizedLine, RecognizedWord, arrange, page_text};
pub use pool::{Run, default_workers, recognize_pages};
pub use saved::{load_pages, save_pages, saved_pages_file};

/// OCR finds these pages by their `ImageOnlyPage` marker, so this text is free to change and be translated.
/// Literal [`t`] calls rather than a translated constant, because `xgettext` only sees string literals.
#[must_use]
pub fn image_only_placeholder() -> String {
	#[cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
	{
		// TRANSLATORS: Line shown in place of a scanned PDF page with no text layer; Enter on it runs OCR
		t("[Image only. Press enter to OCR.]")
	}
	#[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
	{
		// TRANSLATORS: Line shown in place of a scanned PDF page with no text layer, where the system has no OCR engine
		t("[Image only. This page has no text to read.]")
	}
}

/// OCR engines want 200 to 300 DPI; below that, small type starts dropping characters.
const TARGET_DPI: f32 = 300.0;
const POINTS_PER_INCH: f32 = 72.0;

const FALLBACK_WIDTH: i32 = 1800;

#[derive(Debug)]
pub struct RenderedPage {
	pub width: u32,
	pub height: u32,
	pub rgba: Vec<u8>,
}

/// Each pool worker opens its own rather than sharing one, so the workers rasterize in parallel.
pub struct PageRenderer {
	source: Source,
}

enum Source {
	Pdf(PdfDocument),
	Comic {
		archive: ZipArchive<BufReader<File>>,
		pages: Vec<String>,
	},
	/// A RAR is read from the front rather than opened at any member, so each page reopens it.
	ComicRar {
		file_path: String,
		pages: Vec<String>,
	},
}

impl PageRenderer {
	pub fn open(file_path: &str, password: Option<&str>) -> Result<Self> {
		let extension = Path::new(file_path).extension();
		if extension.is_some_and(|e| e.eq_ignore_ascii_case("cbz")) {
			return Self::open_comic(file_path);
		}
		if extension.is_some_and(|e| e.eq_ignore_ascii_case("cbr")) {
			return Self::open_comic_rar(file_path);
		}
		Ok(Self { source: Source::Pdf(PdfDocument::open(file_path, password)?) })
	}

	fn open_comic(file_path: &str) -> Result<Self> {
		let file = File::open(file_path).with_context(|| format!("Failed to open '{file_path}'"))?;
		let mut archive = ZipArchive::new(BufReader::new(file))?;
		// Numbered exactly as the parser numbered them, so page N here is the page the reader is on.
		let pages = cbz::page_names(&mut archive);
		if pages.is_empty() {
			anyhow::bail!("comic archive '{file_path}' has no pages to render");
		}
		Ok(Self { source: Source::Comic { archive, pages } })
	}

	// A .cbr that is really a zip reads as one, as it does in the parser.
	fn open_comic_rar(file_path: &str) -> Result<Self> {
		if cbz::is_zip(file_path) {
			return Self::open_comic(file_path);
		}
		let pages = cbr::page_names(file_path)?;
		if pages.is_empty() {
			anyhow::bail!("comic archive '{file_path}' has no pages to render");
		}
		Ok(Self { source: Source::ComicRar { file_path: file_path.to_string(), pages } })
	}

	pub fn render(&mut self, page_index: i32, max_dimension: u32) -> Result<RenderedPage> {
		match &mut self.source {
			Source::Pdf(document) => {
				let page = document.page(page_index)?;
				let width = u32::try_from(pixel_width(page.width(), page.height(), max_dimension)).unwrap_or(1);
				let bitmap =
					page.render(width).ok_or_else(|| anyhow::anyhow!("page {page_index} could not be rasterized"))?;
				Ok(RenderedPage { width: bitmap.width, height: bitmap.height, rgba: bitmap.rgba })
			}
			Source::ComicRar { file_path, pages } => {
				let index = usize::try_from(page_index).unwrap_or(usize::MAX);
				let name = pages.get(index).ok_or_else(|| anyhow::anyhow!("page {page_index} is out of range"))?;
				let bytes = cbr::page_bytes(file_path, name)?;
				decode_page(&bytes, max_dimension)
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

fn decode_page(bytes: &[u8], max_dimension: u32) -> Result<RenderedPage> {
	decode_image(bytes, max_dimension).context("failed to decode a comic archive page")
}

pub fn decode_image(bytes: &[u8], max_dimension: u32) -> Result<RenderedPage> {
	let image = image::load_from_memory(bytes).context("failed to decode the image")?;
	// The engines downscale oversized input themselves anyway, so shrinking here only saves carrying the pixels around.
	let longest = image.width().max(image.height());
	let image = if max_dimension > 0 && longest > max_dimension {
		image.thumbnail(max_dimension, max_dimension)
	} else {
		image
	};
	let rgba = image.into_rgba8();
	Ok(RenderedPage { width: rgba.width(), height: rgba.height(), rgba: rgba.into_raw() })
}

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

	fn png(width: u32, height: u32) -> Vec<u8> {
		let mut buffer = Vec::new();
		let encoder = image::codecs::png::PngEncoder::new(&mut buffer);
		let pixels = vec![255u8; (width * height) as usize];
		image::ImageEncoder::write_image(encoder, &pixels, width, height, image::ExtendedColorType::L8).unwrap();
		buffer
	}

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

	// The reader numbers comic pages in natural order; disagreeing here would OCR some other page.
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

	#[test]
	fn an_oversized_comic_page_is_shrunk_to_the_cap() {
		let path = comic("cap", &[("001.png", 400, 200)]);
		let mut renderer = PageRenderer::open(&path, None).unwrap();
		let page = renderer.render(0, 100).unwrap();
		assert_eq!(page.width, 100);
		assert_eq!(page.height, 50);
	}

	#[test]
	fn a_picture_decodes_to_rgba_of_its_own_size() {
		let page = decode_image(&png(8, 4), 4000).unwrap();
		assert_eq!((page.width, page.height), (8, 4));
		assert_eq!(page.rgba.len(), 8 * 4 * 4);
	}

	#[test]
	fn an_oversized_picture_is_shrunk_to_the_cap() {
		let page = decode_image(&png(400, 200), 100).unwrap();
		assert_eq!((page.width, page.height), (100, 50));
	}

	#[test]
	fn a_picture_that_is_not_an_image_is_an_error() {
		assert!(decode_image(b"<svg xmlns=\"http://www.w3.org/2000/svg\"/>", 4000).is_err());
	}

	#[test]
	fn a_page_past_the_end_of_a_comic_is_an_error() {
		let path = comic("range", &[("001.png", 8, 4)]);
		let mut renderer = PageRenderer::open(&path, None).unwrap();
		assert!(renderer.render(1, 4000).is_err());
		assert!(renderer.render(-1, 4000).is_err());
	}

	#[test]
	fn a_comic_with_no_pages_will_not_open() {
		let path = comic("empty", &[("ComicInfo.xml", 1, 1)]);
		assert!(PageRenderer::open(&path, None).is_err());
	}

	#[test]
	fn pixel_width_renders_letter_at_target_dpi() {
		assert_eq!(pixel_width(612.0, 792.0, 4000), 2550);
	}

	#[test]
	fn pixel_width_scales_down_to_the_cap() {
		assert_eq!(pixel_width(612.0, 792.0, 2600), 2009);
	}

	#[test]
	fn pixel_width_falls_back_for_an_unusable_page_size() {
		assert_eq!(pixel_width(0.0, 792.0, 2600), FALLBACK_WIDTH);
		assert_eq!(pixel_width(612.0, -1.0, 2600), FALLBACK_WIDTH);
	}
}
