use std::path::{Path, PathBuf};

pub const HTML_NAME: &str = "ui-test.html";
pub const TITLE: &str = "UI test book";
pub const H1: &str = "Paperback UI test";
pub const H2_FIRST: &str = "Introduction";
pub const H2_LAST: &str = "Conclusion";

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

pub const COMIC_NAME: &str = "ui-test.cbz";
pub const COMIC_WORDS: &str = "quick brown fox";

pub fn write_comic(dir: &Path) -> PathBuf {
	use std::io::Write as _;
	std::fs::create_dir_all(dir).expect("create fixture dir");
	let path = dir.join(COMIC_NAME);
	let mut zip = zip::ZipWriter::new(std::fs::File::create(&path).expect("create comic"));
	zip.start_file("001.png", zip::write::SimpleFileOptions::default()).expect("start page");
	zip.write_all(include_bytes!("ocr-sample.png")).expect("write page");
	zip.finish().expect("finish comic");
	path
}
