//! Documents the UI tests open, generated at test time.

use std::path::{Path, PathBuf};

pub const HTML_NAME: &str = "ui-test.html";
pub const TITLE: &str = "UI test book";
pub const H1: &str = "Paperback UI test";
pub const H2_FIRST: &str = "Introduction";
pub const H2_LAST: &str = "Conclusion";

/// Writes a small HTML book into `dir`: an opening paragraph, then an `h1` and two `h2`s,
/// each followed by text, plus a link and a list.
pub fn write_html(dir: &Path) -> PathBuf {
	std::fs::create_dir_all(dir).expect("create fixture dir");
	let path = dir.join(HTML_NAME);
	let html = format!(
		"<!DOCTYPE html>\n<html><head><title>{TITLE}</title></head><body>\n\
		<p>This book exists for the UI tests.</p>\n\
		<h1>{H1}</h1>\n\
		<p>The first chapter follows.</p>\n\
		<h2>{H2_FIRST}</h2>\n\
		<p>Some introductory text with <a href=\"https://example.com\">a link</a>.</p>\n\
		<ul><li>First item</li><li>Second item</li></ul>\n\
		<h2>{H2_LAST}</h2>\n\
		<p>The last words of the book.</p>\n\
		</body></html>\n"
	);
	std::fs::write(&path, html).expect("write fixture");
	path
}
