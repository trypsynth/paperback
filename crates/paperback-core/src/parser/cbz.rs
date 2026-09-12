//! Comic book archives (`.cbz`): a zip holding one image per page.
//!
//! There is no text in one, so every page comes out as an OCR placeholder carrying the same
//! [`MarkerType::ImageOnlyPage`] marker a scanned PDF page gets. That is what puts comic
//! archives in reach of the reader at all: pressing Enter on a page runs it through the
//! platform OCR engine exactly as it would a scan.

use std::{cmp::Ordering, fs::File, io::BufReader};

use anyhow::{Context, Result};
use zip::ZipArchive;

use crate::{
	document::{Document, DocumentBuffer, Marker, MarkerType, ParserContext},
	ocr::image_only_placeholder,
	parser::{Parser, util::path::extract_title_from_path},
	t,
};

/// File extensions inside the archive that count as a page.
///
/// Comic archives routinely carry a `ComicInfo.xml`, a `.nfo`, or the odd thumbnail
/// database alongside the artwork, and none of those is a page.
const PAGE_EXTENSIONS: [&str; 7] = ["jpg", "jpeg", "png", "gif", "webp", "bmp", "avif"];

pub struct CbzParser;

impl Parser for CbzParser {
	fn parse(&self, context: &ParserContext) -> Result<Document> {
		tracing::debug!(path = %context.file_path, "parsing cbz file");
		let file = File::open(&context.file_path)
			.with_context(|| format!("Failed to open comic archive '{}'", context.file_path))?;
		let mut archive = ZipArchive::new(BufReader::new(file)).map_err(|e| {
			tracing::warn!(path = %context.file_path, error = %e, "failed to open comic archive");
			// TRANSLATORS: Error shown when a comic book archive (.cbz) cannot be opened; {} is the underlying error
			anyhow::anyhow!(t("Failed to parse comic archive: {}").replace("{}", &e.to_string()))
		})?;
		let pages = page_names(&mut archive);
		if pages.is_empty() {
			// TRANSLATORS: Error shown when a comic book archive holds no page images at all
			return Err(anyhow::anyhow!(t("This comic archive contains no pages.")));
		}
		tracing::debug!(path = %context.file_path, pages = pages.len(), "cbz structure read");
		let buffer = build(&pages);
		let mut document = Document::new().with_title(extract_title_from_path(&context.file_path));
		document.set_buffer(buffer);
		tracing::debug!(path = %context.file_path, "parsed cbz file successfully");
		Ok(document)
	}
}

/// The archive's page images, in reading order.
///
/// Shared with [`crate::ocr`], which reopens the archive to render a page and has to number
/// the pages exactly as this parser did.
pub fn page_names<R: std::io::Read + std::io::Seek>(archive: &mut ZipArchive<R>) -> Vec<String> {
	let mut names = Vec::new();
	for i in 0..archive.len() {
		let Ok(entry) = archive.by_index_raw(i) else { continue };
		if !entry.is_file() {
			continue;
		}
		let Ok(name) = entry.name() else { continue };
		if is_page(&name) {
			names.push(name.into_owned());
		}
	}
	names.sort_by(|a, b| natural_cmp(a, b));
	names
}

/// Whether an archive entry is one of the page images rather than metadata.
fn is_page(name: &str) -> bool {
	// A leading dot is a resource fork or a macOS metadata file, never artwork.
	let file_name = name.rsplit(['/', '\\']).next().unwrap_or(name);
	if file_name.starts_with('.') || name.contains("__MACOSX") {
		return false;
	}
	let Some((_, extension)) = file_name.rsplit_once('.') else { return false };
	PAGE_EXTENSIONS.iter().any(|known| extension.eq_ignore_ascii_case(known))
}

/// Lay one placeholder line out per page, marked so the OCR flow can find it.
fn build(pages: &[String]) -> DocumentBuffer {
	let mut buffer = DocumentBuffer::new();
	let placeholder = image_only_placeholder();
	for (index, name) in pages.iter().enumerate() {
		let position = buffer.current_position();
		buffer.add_marker(Marker::new(MarkerType::PageBreak, position).with_text(format!("Page {}", index + 1)));
		// The archive entry rides along as the marker's reference so a caller that wants the
		// artwork itself knows which one to pull out.
		buffer.add_marker(Marker::new(MarkerType::Image, position).with_reference(name.clone()));
		buffer.add_marker(Marker::new(MarkerType::ImageOnlyPage, position));
		buffer.append(&placeholder);
		buffer.append("\n");
	}
	buffer
}

/// Compare two names with runs of digits ordered by value rather than by character.
///
/// Comic archives number their pages without padding often enough that plain sorting puts
/// page 10 in front of page 2, which shuffles the whole book.
fn natural_cmp(a: &str, b: &str) -> Ordering {
	let (mut a, mut b) = (a.as_bytes(), b.as_bytes());
	loop {
		match (a.first(), b.first()) {
			(None, None) => return Ordering::Equal,
			(None, Some(_)) => return Ordering::Less,
			(Some(_), None) => return Ordering::Greater,
			(Some(x), Some(y)) if x.is_ascii_digit() && y.is_ascii_digit() => {
				let (left, rest_a) = split_digits(a);
				let (right, rest_b) = split_digits(b);
				// Compare by length first so an arbitrarily long run needs no parsing, then
				// by digits, which settles equal-length runs and ignores leading zeroes.
				match left.len().cmp(&right.len()).then_with(|| left.cmp(right)) {
					Ordering::Equal => (a, b) = (rest_a, rest_b),
					other => return other,
				}
			}
			(Some(x), Some(y)) => match x.to_ascii_lowercase().cmp(&y.to_ascii_lowercase()) {
				Ordering::Equal => (a, b) = (&a[1..], &b[1..]),
				other => return other,
			},
		}
	}
}

/// Split the leading run of digits off `s`, with any leading zeroes dropped.
fn split_digits(s: &[u8]) -> (&[u8], &[u8]) {
	let end = s.iter().position(|c| !c.is_ascii_digit()).unwrap_or(s.len());
	let (digits, rest) = s.split_at(end);
	let start = digits.iter().position(|&c| c != b'0').unwrap_or(digits.len());
	(&digits[start..], rest)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn artwork_is_told_apart_from_the_metadata_beside_it() {
		assert!(is_page("001.jpg"));
		assert!(is_page("pages/002.PNG"));
		assert!(is_page("a.webp"));
		assert!(!is_page("ComicInfo.xml"));
		assert!(!is_page("readme.txt"));
		assert!(!is_page("noextension"));
	}

	// A zip made on a Mac carries a shadow copy of every file under __MACOSX, and those
	// would otherwise double every page in the book.
	#[test]
	fn macos_metadata_is_not_a_page() {
		assert!(!is_page("__MACOSX/._001.jpg"));
		assert!(!is_page(".DS_Store"));
		assert!(!is_page("pages/._003.jpg"));
	}

	// Page numbers in comic archives are often unpadded, and plain string order puts 10
	// before 2, which shuffles the book.
	#[test]
	fn digits_sort_by_value_rather_than_by_character() {
		let mut names = vec!["p10.jpg".to_string(), "p2.jpg".to_string(), "p1.jpg".to_string()];
		names.sort_by(|a, b| natural_cmp(a, b));
		assert_eq!(names, ["p1.jpg", "p2.jpg", "p10.jpg"]);
	}

	#[test]
	fn padded_and_unpadded_numbers_sort_together() {
		let mut names = vec!["009.jpg".to_string(), "10.jpg".to_string(), "0008.jpg".to_string()];
		names.sort_by(|a, b| natural_cmp(a, b));
		assert_eq!(names, ["0008.jpg", "009.jpg", "10.jpg"]);
	}

	// Archives usually put their pages in folders, and the folder has to order before the
	// numbers inside it or chapters interleave.
	#[test]
	fn folders_order_before_the_pages_inside_them() {
		let mut names = vec![
			"ch2/01.jpg".to_string(),
			"ch10/01.jpg".to_string(),
			"ch1/02.jpg".to_string(),
			"ch1/01.jpg".to_string(),
		];
		names.sort_by(|a, b| natural_cmp(a, b));
		assert_eq!(names, ["ch1/01.jpg", "ch1/02.jpg", "ch2/01.jpg", "ch10/01.jpg"]);
	}

	#[test]
	fn sorting_ignores_case() {
		let mut names = vec!["B.jpg".to_string(), "a.jpg".to_string()];
		names.sort_by(|a, b| natural_cmp(a, b));
		assert_eq!(names, ["a.jpg", "B.jpg"]);
	}

	#[test]
	fn every_page_gets_a_break_and_an_ocr_placeholder() {
		let buffer = build(&["001.jpg".to_string(), "002.jpg".to_string()]);
		let count = |mtype| buffer.markers.iter().filter(|m| m.mtype == mtype).count();
		assert_eq!(count(MarkerType::PageBreak), 2);
		assert_eq!(count(MarkerType::ImageOnlyPage), 2);
		assert_eq!(count(MarkerType::Image), 2);
		assert_eq!(buffer.content.lines().count(), 2);
	}

	// The OCR flow replaces the placeholder line at the marker's own position, so the two
	// have to sit together or Enter finds nothing to swap.
	#[test]
	fn the_ocr_marker_sits_at_the_start_of_its_placeholder_line() {
		let buffer = build(&["001.jpg".to_string(), "002.jpg".to_string()]);
		let line_starts: Vec<usize> = std::iter::once(0)
			.chain(buffer.content.char_indices().filter(|(_, c)| *c == '\n').map(|(i, _)| i + 1))
			.collect();
		for marker in buffer.markers.iter().filter(|m| m.mtype == MarkerType::ImageOnlyPage) {
			assert!(line_starts.contains(&marker.position), "marker at {} is not a line start", marker.position);
		}
	}

	// A caller wanting the artwork has only the marker to go on, so it has to name the entry.
	#[test]
	fn each_page_marker_names_its_archive_entry() {
		let buffer = build(&["cover.png".to_string(), "002.jpg".to_string()]);
		let refs: Vec<&str> =
			buffer.markers.iter().filter(|m| m.mtype == MarkerType::Image).map(|m| m.reference.as_str()).collect();
		assert_eq!(refs, ["cover.png", "002.jpg"]);
	}
}
