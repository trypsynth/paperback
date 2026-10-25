//! Keeping the pages OCR has read, so that a document opened again comes back with them rather than with its placeholders (#1160).
//!
//! What is kept is each line and where its words were read, not the text it was turned into, so the page is arranged again every time it comes back: by the joining the reader has set now, and by however [`super::arrange`] reads a page by then.
//!
//! Each document has one file, named for the same digest of its content that its settings are kept under, so a file that has changed since is never given text read from what it was. A page is appended to the file each time it is read, and a page read again is appended again; the last reading of a page is the one that counts.

use std::{
	collections::BTreeMap,
	fs::{self, File, OpenOptions},
	io::{self, BufRead, BufReader, Write},
	path::{Path, PathBuf},
};

use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use serde::{Deserialize, Serialize};

use super::{RecognizedLine, RecognizedWord};
use crate::config::compute_document_hash;

/// Written as the first line of every file, so a file written in a layout this build does not know is left alone rather than misread.
const VERSION: u32 = 1;

#[derive(Serialize, Deserialize)]
struct Header {
	version: u32,
}

/// One page as it is kept: its number and its lines.
#[derive(Serialize, Deserialize)]
struct Page {
	page: i32,
	lines: Vec<Line>,
}

/// A line and its words, each word with its box in whole pixels, which is as fine as an engine measures.
#[derive(Serialize, Deserialize)]
struct Line {
	text: String,
	words: Vec<(String, i32, i32, i32, i32)>,
}

/// Where the pages read off `document` are kept, in `dir`.
#[must_use]
pub fn saved_pages_file(dir: &Path, document: &Path) -> PathBuf {
	let digest = compute_document_hash(&document.to_string_lossy());
	dir.join(format!("{}.ocr", URL_SAFE_NO_PAD.encode(digest)))
}

/// Adds `pages` to what `file` keeps.
///
/// # Errors
///
/// Returns the error if the file or its folder cannot be written.
pub fn save_pages(file: &Path, pages: &[(i32, Vec<RecognizedLine>)]) -> io::Result<()> {
	if pages.is_empty() {
		return Ok(());
	}
	if let Some(dir) = file.parent() {
		fs::create_dir_all(dir)?;
	}
	let mut out = OpenOptions::new().create(true).append(true).open(file)?;
	let mut text = String::new();
	if out.metadata()?.len() == 0 {
		text.push_str(&serde_json::to_string(&Header { version: VERSION })?);
		text.push('\n');
	}
	for (page, lines) in pages {
		let page = Page { page: *page, lines: lines.iter().map(kept).collect() };
		text.push_str(&serde_json::to_string(&page)?);
		text.push('\n');
	}
	out.write_all(text.as_bytes())
}

/// The pages `file` keeps, in page order, the last reading of each. A file that is missing, or written in a layout this build does not know, keeps nothing; a line that cannot be read, such as one cut short when the app was closed while writing it, is passed over.
#[must_use]
pub fn load_pages(file: &Path) -> Vec<(i32, Vec<RecognizedLine>)> {
	let Ok(input) = File::open(file) else { return Vec::new() };
	let mut records = BufReader::new(input).lines().map_while(Result::ok);
	let known = records
		.next()
		.and_then(|header| serde_json::from_str::<Header>(&header).ok())
		.is_some_and(|header| header.version == VERSION);
	if !known {
		return Vec::new();
	}
	let mut pages = BTreeMap::new();
	for record in records {
		if let Ok(page) = serde_json::from_str::<Page>(&record) {
			pages.insert(page.page, page.lines.into_iter().map(read).collect());
		}
	}
	pages.into_iter().collect()
}

#[allow(clippy::cast_possible_truncation, reason = "a box in an image is far inside an i32")]
fn kept(line: &RecognizedLine) -> Line {
	let words = line
		.words
		.iter()
		.map(|word| {
			let (x, y, width, height) = (word.x.round(), word.y.round(), word.width.round(), word.height.round());
			(word.text.clone(), x as i32, y as i32, width as i32, height as i32)
		})
		.collect();
	Line { text: line.text.clone(), words }
}

#[allow(clippy::cast_precision_loss, reason = "a box in an image is far inside what an f32 counts exactly")]
fn read(line: Line) -> RecognizedLine {
	let words = line
		.words
		.into_iter()
		.map(|(text, x, y, width, height)| RecognizedWord {
			text,
			x: x as f32,
			y: y as f32,
			width: width as f32,
			height: height as f32,
		})
		.collect();
	RecognizedLine { text: line.text, words }
}

#[cfg(test)]
mod tests {
	use std::fs;

	use super::{load_pages, save_pages};
	use crate::ocr::{RecognizedLine, RecognizedWord};

	fn line(text: &str, x: f32) -> RecognizedLine {
		RecognizedLine {
			text: text.to_string(),
			words: vec![RecognizedWord { text: text.to_string(), x, y: 10.0, width: 40.0, height: 16.0 }],
		}
	}

	fn file(test: &str) -> std::path::PathBuf {
		let dir = std::env::temp_dir().join(format!("paperback-saved-ocr-{test}-{}", std::process::id()));
		let _ = fs::remove_dir_all(&dir);
		dir.join("pages.ocr")
	}

	#[test]
	fn pages_come_back_as_they_were_read() {
		let file = file("round-trip");
		save_pages(&file, &[(3, vec![line("three", 5.0)]), (1, vec![line("one", 7.0)])]).unwrap();
		assert_eq!(load_pages(&file), vec![(1, vec![line("one", 7.0)]), (3, vec![line("three", 5.0)])]);
	}

	/// A page read again replaces what it was read as before, and the pages read once are kept.
	#[test]
	fn the_last_reading_of_a_page_counts() {
		let file = file("last-reading");
		save_pages(&file, &[(1, vec![line("first try", 0.0)]), (2, vec![line("two", 0.0)])]).unwrap();
		save_pages(&file, &[(1, vec![line("second try", 0.0)])]).unwrap();
		assert_eq!(load_pages(&file), vec![(1, vec![line("second try", 0.0)]), (2, vec![line("two", 0.0)])]);
	}

	/// The app closed while a page was being written leaves a line cut short, which costs that page and no other.
	#[test]
	fn a_page_cut_short_is_passed_over() {
		let file = file("cut-short");
		save_pages(&file, &[(1, vec![line("one", 0.0)])]).unwrap();
		let mut text = fs::read_to_string(&file).unwrap();
		text.push_str("{\"page\":2,\"lines\":[{\"te");
		fs::write(&file, text).unwrap();
		assert_eq!(load_pages(&file), vec![(1, vec![line("one", 0.0)])]);
	}

	/// A file from a build that keeps pages some other way is left alone rather than misread.
	#[test]
	fn a_file_in_an_unknown_layout_keeps_nothing() {
		let file = file("unknown-layout");
		fs::create_dir_all(file.parent().unwrap()).unwrap();
		fs::write(&file, "{\"version\":99}\n{\"page\":1,\"lines\":[]}\n").unwrap();
		assert!(load_pages(&file).is_empty());
		assert!(load_pages(&file.with_extension("missing")).is_empty());
	}
}
