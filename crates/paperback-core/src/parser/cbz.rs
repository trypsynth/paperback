//! Comic book archives (`.cbz`): a zip holding one image per page.
//!
//! There is no text in one, so every page comes out as an OCR placeholder carrying the same
//! [`MarkerType::ImageOnlyPage`] marker a scanned PDF page gets. That is what puts comic
//! archives in reach of the reader at all: pressing Enter on a page runs it through the
//! platform OCR engine exactly as it would a scan.

use std::{
	cmp::Ordering,
	collections::HashMap,
	fs::File,
	io::{BufReader, Read},
};

use anyhow::{Context, Result};
use zip::ZipArchive;

use crate::{
	document::{Document, DocumentBuffer, Marker, MarkerType, ParserContext, TocItem},
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
		let info = comic_info(&mut archive);
		tracing::debug!(path = %context.file_path, pages = pages.len(), has_info = info.is_some(), "cbz structure read");
		comic_document(&pages, info.unwrap_or_default(), &context.file_path)
	}
}

/// What a comic's `ComicInfo.xml` says about it: the metadata file `ComicRack` defined and most comic tools now write beside the pages.
#[derive(Debug, Default)]
pub(super) struct ComicInfo {
	title: Option<String>,
	author: Option<String>,
	/// Keyed by the page's index among the archive's pages in reading order, which is how `<Page Image="...">` counts.
	pages: HashMap<usize, ComicPage>,
}

#[derive(Debug, Default)]
struct ComicPage {
	/// The page type, already worded for the reader, such as "Front cover". `None` for an ordinary story page.
	kind: Option<String>,
	bookmark: Option<String>,
}

/// Reads `ComicInfo.xml` from the archive's root. Only the root counts: an omnibus zip can carry an issue's own file in each folder.
fn comic_info<R: std::io::Read + std::io::Seek>(archive: &mut ZipArchive<R>) -> Option<ComicInfo> {
	let name = archive
		.file_names()
		.filter_map(Result::ok)
		.find(|name| name.eq_ignore_ascii_case("ComicInfo.xml"))?
		.into_owned();
	let mut xml = String::new();
	archive.by_name(&name).ok()?.read_to_string(&mut xml).ok()?;
	parse_comic_info(&xml)
}

fn parse_comic_info(xml: &str) -> Option<ComicInfo> {
	let document = roxmltree::Document::parse(xml).ok()?;
	let root = document.root_element();
	let field = |name: &str| {
		root.children()
			.find(|child| child.has_tag_name(name))
			.and_then(|child| child.text())
			.map(str::trim)
			.filter(|text| !text.is_empty())
			.map(str::to_string)
	};
	// A series issue is known by its series and number more than by its own title, which is often missing or a story name.
	let series = field("Series").map(|series| match field("Number") {
		Some(number) => format!("{series} #{number}"),
		None => series,
	});
	let title = match (series, field("Title")) {
		(Some(series), Some(title)) => Some(format!("{series}: {title}")),
		(series, title) => series.or(title),
	};
	let mut pages = HashMap::new();
	for page in root.descendants().filter(|node| node.has_tag_name("Page")) {
		let Some(index) = page.attribute("Image").and_then(|image| image.trim().parse::<usize>().ok()) else {
			continue;
		};
		let kind = page.attribute("Type").and_then(page_kind);
		let bookmark =
			page.attribute("Bookmark").map(str::trim).filter(|bookmark| !bookmark.is_empty()).map(str::to_string);
		if kind.is_some() || bookmark.is_some() {
			pages.insert(index, ComicPage { kind, bookmark });
		}
	}
	Some(ComicInfo { title, author: field("Writer"), pages })
}

/// A `<Page Type>` as the reader hears it. Story pages, and types this does not know, are ordinary pages and say nothing.
fn page_kind(kind: &str) -> Option<String> {
	Some(match kind {
		// TRANSLATORS: Spoken for a comic book page that is its front cover
		"FrontCover" => t("Front cover"),
		// TRANSLATORS: Spoken for a comic book page that is an inside cover
		"InnerCover" => t("Inside cover"),
		// TRANSLATORS: Spoken for a comic book page that recaps the story so far
		"Roundup" => t("Recap"),
		// TRANSLATORS: Spoken for a comic book page that is an advertisement
		"Advertisement" => t("Advertisement"),
		// TRANSLATORS: Spoken for a comic book page written by the editors
		"Editorial" => t("Editorial"),
		// TRANSLATORS: Spoken for a comic book page of letters from readers
		"Letters" => t("Letters"),
		// TRANSLATORS: Spoken for a comic book page previewing another comic
		"Preview" => t("Preview"),
		// TRANSLATORS: Spoken for a comic book page that is its back cover
		"BackCover" => t("Back cover"),
		_ => return None,
	})
}

/// One comic's worth of placeholder pages, however the archive they came out of was packed.
///
/// # Errors
///
/// Returns an error if the archive holds no pages at all.
pub(super) fn comic_document(pages: &[String], info: ComicInfo, file_path: &str) -> Result<Document> {
	if pages.is_empty() {
		// TRANSLATORS: Error shown when a comic book archive holds no page images at all
		return Err(anyhow::anyhow!(t("This comic archive contains no pages.")));
	}
	let title = info.title.clone().unwrap_or_else(|| extract_title_from_path(file_path));
	let mut document = Document::new().with_title(title).with_author(info.author.clone().unwrap_or_default());
	let (buffer, toc_items) = build(pages, &info);
	document.set_buffer(buffer);
	document.toc_items = toc_items;
	tracing::debug!(path = %file_path, "parsed comic archive successfully");
	Ok(document)
}

/// Whether a file is a zip, whatever its name says.
///
/// Comic archives get renamed between .cbz and .cbr by people sorting their shelves, so the
/// name is a hint and the first four bytes are the answer.
pub(crate) fn is_zip(file_path: &str) -> bool {
	let Ok(mut file) = File::open(file_path) else { return false };
	let mut magic = [0u8; 4];
	file.read_exact(&mut magic).is_ok() && magic == *b"PK"
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
pub(super) fn is_page(name: &str) -> bool {
	// A leading dot is a resource fork or a macOS metadata file, never artwork.
	let file_name = name.rsplit(['/', '\\']).next().unwrap_or(name);
	if file_name.starts_with('.') || name.contains("__MACOSX") {
		return false;
	}
	let Some((_, extension)) = file_name.rsplit_once('.') else { return false };
	PAGE_EXTENSIONS.iter().any(|known| extension.eq_ignore_ascii_case(known))
}

/// Lay one placeholder line out per page, marked so the OCR flow can find it, with the table of contents the comic's page bookmarks make.
fn build(pages: &[String], info: &ComicInfo) -> (DocumentBuffer, Vec<TocItem>) {
	let mut buffer = DocumentBuffer::new();
	let mut toc_items = Vec::new();
	let placeholder = image_only_placeholder();
	for (index, name) in pages.iter().enumerate() {
		let position = buffer.current_position();
		let page = info.pages.get(&index);
		// The label stays "Page N" whatever the page is: the OCR pass reads the page number back out of it.
		buffer.add_marker(Marker::new(MarkerType::PageBreak, position).with_text(format!("Page {}", index + 1)));
		// The archive entry rides along as the marker's reference so a caller that wants the
		// artwork itself knows which one to pull out.
		let kind = page.and_then(|page| page.kind.clone()).unwrap_or_default();
		buffer.add_marker(Marker::new(MarkerType::Image, position).with_reference(name.clone()).with_text(kind));
		buffer.add_marker(Marker::new(MarkerType::ImageOnlyPage, position));
		if let Some(bookmark) = page.and_then(|page| page.bookmark.clone()) {
			toc_items.push(TocItem::new(bookmark, String::new(), position));
		}
		buffer.append(&placeholder);
		buffer.append("\n");
	}
	(buffer, toc_items)
}

/// Compare two names with runs of digits ordered by value rather than by character.
///
/// Comic archives number their pages without padding often enough that plain sorting puts
/// page 10 in front of page 2, which shuffles the whole book.
pub(super) fn natural_cmp(a: &str, b: &str) -> Ordering {
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

	fn build_plain(pages: &[String]) -> DocumentBuffer {
		build(pages, &ComicInfo::default()).0
	}

	const COMIC_INFO: &str = r#"<?xml version="1.0"?>
<ComicInfo xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
	<Title>The Big Heist</Title>
	<Series>Night Patrol</Series>
	<Number>12</Number>
	<Writer>A. Writer, B. Writer</Writer>
	<Pages>
		<Page Image="0" Type="FrontCover" />
		<Page Image="1" Type="Story" Bookmark="Chapter 1" />
		<Page Image="2" Type="Advertisement" />
	</Pages>
</ComicInfo>"#;

	#[test]
	fn comic_info_names_the_issue_by_series_number_and_title() {
		let info = parse_comic_info(COMIC_INFO).unwrap();
		assert_eq!(info.title.as_deref(), Some("Night Patrol #12: The Big Heist"));
		assert_eq!(info.author.as_deref(), Some("A. Writer, B. Writer"));
	}

	#[test]
	fn a_comic_without_a_series_is_named_by_its_title_alone() {
		let info = parse_comic_info("<ComicInfo><Title>One Shot</Title></ComicInfo>").unwrap();
		assert_eq!(info.title.as_deref(), Some("One Shot"));
		assert_eq!(info.author, None);
	}

	/// The page type goes on the picture rather than the page label, which the OCR pass parses.
	#[test]
	fn page_types_name_the_picture_and_bookmarks_make_the_contents() {
		let info = parse_comic_info(COMIC_INFO).unwrap();
		let pages: Vec<String> = ["001.jpg", "002.jpg", "003.jpg"].iter().map(ToString::to_string).collect();
		let (buffer, toc) = build(&pages, &info);
		let image_texts: Vec<&str> =
			buffer.markers.iter().filter(|m| m.mtype == MarkerType::Image).map(|m| m.text.as_str()).collect();
		assert_eq!(image_texts, ["Front cover", "", "Advertisement"]);
		let labels: Vec<&str> =
			buffer.markers.iter().filter(|m| m.mtype == MarkerType::PageBreak).map(|m| m.text.as_str()).collect();
		assert_eq!(labels, ["Page 1", "Page 2", "Page 3"]);
		assert_eq!(toc.len(), 1);
		assert_eq!(toc[0].name, "Chapter 1");
		let second_page = buffer.markers.iter().filter(|m| m.mtype == MarkerType::PageBreak).nth(1).unwrap().position;
		assert_eq!(toc[0].offset, second_page);
	}

	#[test]
	fn a_cbz_takes_its_title_author_and_contents_from_its_comic_info() {
		use std::io::Write;
		let dir = crate::util::test_support::TempDir::new("cbz-comic-info");
		let path = dir.join_str("issue.cbz");
		let mut zip = zip::ZipWriter::new(File::create(&path).unwrap());
		for (name, contents) in [("ComicInfo.xml", COMIC_INFO), ("001.jpg", "a"), ("002.jpg", "b"), ("003.jpg", "c")] {
			zip.start_file(name, zip::write::SimpleFileOptions::default()).unwrap();
			zip.write_all(contents.as_bytes()).unwrap();
		}
		zip.finish().unwrap();
		let document = CbzParser.parse(&ParserContext::new(path)).unwrap();
		assert_eq!(document.title, "Night Patrol #12: The Big Heist");
		assert_eq!(document.author, "A. Writer, B. Writer");
		assert_eq!(document.toc_items.iter().map(|item| item.name.as_str()).collect::<Vec<_>>(), ["Chapter 1"]);
	}

	#[test]
	fn broken_comic_info_is_ignored() {
		assert!(parse_comic_info("<ComicInfo><Title>").is_none());
	}

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
		let buffer = build_plain(&["001.jpg".to_string(), "002.jpg".to_string()]);
		let count = |mtype| buffer.markers.iter().filter(|m| m.mtype == mtype).count();
		assert_eq!(count(MarkerType::PageBreak), 2);
		assert_eq!(count(MarkerType::ImageOnlyPage), 2);
		assert_eq!(count(MarkerType::Image), 2);
		assert_eq!(buffer.content.lines().count(), 2);
	}

	/// The OCR pass reads a page's number off its break's label to know which page of the file to
	/// render: `--pages` renumbers the pages of an extract from one, so the position in the document
	/// is no longer the page of the file, and the label is what still says which sheet it is. A
	/// label that stopped naming its page would send `pb --pages ... --ocr-image-pages` back to
	/// rendering page 1 for every page of the range, with nothing to show for it but a scan that
	/// reads as if the engine had misread it.
	///
	/// The CBR parser goes through this one, so both comic formats are pinned here.
	#[test]
	fn a_page_break_names_the_page_of_the_file_it_heads() {
		let buffer = build_plain(&["001.jpg".to_string(), "002.jpg".to_string()]);
		let labels: Vec<&str> = buffer
			.markers
			.iter()
			.filter(|marker| marker.mtype == MarkerType::PageBreak)
			.map(|marker| marker.text.as_str())
			.collect();
		assert_eq!(labels, ["Page 1", "Page 2"]);
	}

	// The OCR flow replaces the placeholder line at the marker's own position, so the two
	// have to sit together or Enter finds nothing to swap.
	#[test]
	fn the_ocr_marker_sits_at_the_start_of_its_placeholder_line() {
		let buffer = build_plain(&["001.jpg".to_string(), "002.jpg".to_string()]);
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
		let buffer = build_plain(&["cover.png".to_string(), "002.jpg".to_string()]);
		let refs: Vec<&str> =
			buffer.markers.iter().filter(|m| m.mtype == MarkerType::Image).map(|m| m.reference.as_str()).collect();
		assert_eq!(refs, ["cover.png", "002.jpg"]);
	}
}
