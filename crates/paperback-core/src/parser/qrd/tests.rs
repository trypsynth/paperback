use rusqlite::{Connection, params};
use serde_json::{Value, json};

use super::QrdParser;
use crate::{
	document::{Document, MarkerType, ParserContext},
	parser::Parser,
	util::test_support::TempDir,
};

/// As Python's `sqlhash` creates its shelf.
fn write_qrd(dir: &TempDir, name: &str, entries: &[(&str, String)]) -> String {
	let path = dir.path().join(name);
	let connection = Connection::open(&path).unwrap();
	connection.execute("CREATE TABLE shelf (key TEXT NOT NULL, value TEXT NOT NULL)", []).unwrap();
	connection.execute("CREATE UNIQUE INDEX keyndx ON shelf (key)", []).unwrap();
	for (key, value) in entries {
		connection.execute("INSERT INTO shelf VALUES (?1, ?2)", params![key, value]).unwrap();
	}
	path.to_string_lossy().into_owned()
}

fn book(text: &str, tree: &Value, metadata: &Value, original_path: &str) -> Vec<(&'static str, String)> {
	let config = json!({"text_location": "raw.text", "tree_location": "tree.json", "metadata_location": "metadata.json", "original_path": original_path, "format_version": 2, "demo_mode": false});
	vec![
		("book.json", config.to_string()),
		("metadata.json", metadata.to_string()),
		("tree.json", json!({"tree": tree, "tree_indexes": {}}).to_string()),
		("raw.text", text.to_string()),
	]
}

/// Where QRead put `needle`: in UTF-16 units, as Python 2 on Windows counted.
fn units(text: &str, needle: &str) -> (usize, usize) {
	let start = text[..text.find(needle).unwrap()].encode_utf16().count();
	(start, start + needle.encode_utf16().count())
}

fn chars(text: &str, needle: &str) -> usize {
	text[..text.find(needle).unwrap()].chars().count()
}

fn open(path: String) -> Document {
	QrdParser.parse(&ParserContext::new(path)).expect("read the book")
}

fn markers(document: &Document, kind: MarkerType) -> Vec<&crate::document::Marker> {
	document.buffer.markers.iter().filter(|marker| marker.mtype == kind).collect()
}

/// An emoji and a CRLF come before most of the book, so every offset after them has to be converted.
const STORY: &str =
	"Chapter One\nSee the next part, or mail us \u{1F600}.\r\nPart Two\nName\tUse\nA\tFirst\nGo back.\n";

fn story_tree() -> Value {
	let at = |needle| units(STORY, needle);
	let (one, one_end) = at("Chapter One");
	let (next, next_end) = at("the next part");
	let (mail, mail_end) = at("mail us");
	let (two, two_end) = at("Part Two");
	let (table, _) = at("Name");
	let table_end = at("First").1 + 1;
	let (back, back_end) = at("Go back");
	let cell = |id, parent, needle| {
		let (start, end) = at(needle);
		json!({"type": "td", "id": id, "parent": parent, "start": start, "end": end, "name": null, "attrs": {}})
	};
	json!([
		{"type": "id", "id": 0, "parent": null, "start": 0, "name": "ch1.xhtml"},
		{"type": "heading", "id": 1, "parent": null, "start": one, "end": one_end, "name": null, "level": "1", "tag": "h1"},
		{"type": "link", "id": 2, "parent": null, "start": next, "end": next_end, "name": "the next part", "href": "ch2.xhtml#two"},
		{"type": "link", "id": 3, "parent": null, "start": mail, "end": mail_end, "name": "mail us", "href": "text/mailto:us@example.com"},
		{"type": "id", "id": 4, "parent": null, "start": two, "name": "ch2.xhtml"},
		{"type": "id", "id": 5, "parent": null, "start": two, "name": "ch2.xhtml#two"},
		{"type": "heading", "id": 6, "parent": null, "start": two, "end": two_end, "name": "Part Two\n", "level": "2", "tag": "h2"},
		{"type": "table", "id": 7, "parent": null, "start": table, "end": table_end, "name": null, "attrs": {}},
		{"type": "tbody", "id": 8, "parent": 7, "start": table, "end": table_end, "name": null, "attrs": {}},
		{"type": "tr", "id": 9, "parent": 8, "start": table, "end": table_end, "name": null, "attrs": {}},
		cell(10, 9, "Name"),
		cell(11, 9, "Use"),
		{"type": "tr", "id": 12, "parent": 8, "start": table, "end": table_end, "name": null, "attrs": {}},
		cell(13, 12, "A\t"),
		cell(14, 12, "First"),
		{"type": "link", "id": 15, "parent": null, "start": back, "end": back_end, "name": "Go back", "href": "ch2.xhtml#missing"},
		{"type": "toc", "id": 16, "parent": null, "start": 0, "name": "Chapter One", "src": "ch1.xhtml"},
		{"type": "toc", "id": 17, "parent": 16, "start": two, "name": "Part Two", "src": "ch2.xhtml#two"},
		{"type": "toc", "id": 18, "parent": 99, "start": back, "name": "Afterword", "src": "ch2.xhtml"},
	])
}

#[test]
fn reads_a_book_converted_from_an_epub() {
	let dir = TempDir::new("qrd-story");
	let metadata = json!({"title": "Story", "authors": ["A. Writer", "B. Writer"]});
	let document = open(write_qrd(&dir, "story.qrd", &book(STORY, &story_tree(), &metadata, r"C:\Books\story.epub")));
	let text = document.buffer.content.clone();
	assert_eq!(text, STORY.replace("\r\n", "\n"));
	assert_eq!((document.title.as_str(), document.author.as_str()), ("Story", "A. Writer, B. Writer"));
	let headings: Vec<(usize, &str)> = document
		.buffer
		.markers
		.iter()
		.filter(|marker| matches!(marker.mtype, MarkerType::Heading1 | MarkerType::Heading2))
		.map(|marker| (marker.position.get(), marker.text.as_str()))
		.collect();
	assert_eq!(headings, [(0, "Chapter One"), (chars(&text, "Part Two"), "Part Two")]);
	assert_eq!(document.id_positions["ch2.xhtml#two"].get(), chars(&text, "Part Two"));
	assert_eq!(markers(&document, MarkerType::SectionBreak).len(), 2);
	let links: Vec<(usize, &str, &str)> = markers(&document, MarkerType::Link)
		.iter()
		.map(|marker| (marker.position.get(), marker.text.as_str(), marker.reference.as_str()))
		.collect();
	assert_eq!(
		links,
		[
			(chars(&text, "the next"), "the next part", "ch2.xhtml#two"),
			(chars(&text, "mail us"), "mail us", "mailto:us@example.com"),
			(chars(&text, "Go back"), "Go back", "ch2.xhtml"),
		]
	);
	let table = markers(&document, MarkerType::Table)[0];
	assert_eq!(table.position.get(), chars(&text, "Name"));
	assert_eq!(table.reference, "<table><tr><td>Name</td><td>Use</td></tr><tr><td>A</td><td>First</td></tr></table>");
	let toc: Vec<(&str, usize, usize)> =
		document.toc_items.iter().map(|item| (item.name.as_str(), item.offset.get(), item.children.len())).collect();
	assert_eq!(toc, [("Chapter One", 0, 1), ("Afterword", chars(&text, "Go back"), 0)]);
	assert_eq!(document.toc_items[0].children[0].offset.get(), chars(&text, "Part Two"));
}

#[test]
fn repairs_the_links_qread_joined_wrongly_in_help_files() {
	let text = "Formats\nSee zip and the index or the site.\nZip\n";
	let at = |needle| units(text, needle);
	let link = |id, needle, href| {
		let (start, end) = at(needle);
		json!({"type": "link", "id": id, "parent": null, "start": start, "end": end, "name": needle, "href": href})
	};
	let tree = json!([
		{"type": "id", "id": 0, "parent": null, "start": 0, "name": "/general/index.htm"},
		{"type": "id", "id": 1, "parent": null, "start": at("Zip").0, "name": "/general/index.htm#zip"},
		link(2, "zip", "/general/#zip"),
		link(3, "the index", "/GENERAL/Index.htm"),
		link(4, "the site", "/general/https://example.com/7z"),
	]);
	let dir = TempDir::new("qrd-help");
	let document = open(write_qrd(&dir, "7-zip.qrd", &book(text, &tree, &json!({"title": "7-Zip"}), r"C:\7-zip.chm")));
	let references: Vec<&str> =
		markers(&document, MarkerType::Link).iter().map(|marker| marker.reference.as_str()).collect();
	assert_eq!(references, ["/general/index.htm#zip", "/general/index.htm", "https://example.com/7z"]);
}

#[test]
fn reads_a_book_converted_from_a_pdf() {
	let text = "First page.\nSecond page.\n";
	let page = |id, number, needle| {
		let (start, end) = units(text, needle);
		json!({"type": "page", "id": id, "parent": null, "start": start, "end": end, "name": format!("page {number}"), "pagenum": number})
	};
	let tree = json!([page(0, 1, "First page.\n"), page(1, 2, "Second page.\n")]);
	let dir = TempDir::new("qrd-pdf");
	let document = open(write_qrd(
		&dir,
		"notes.qrd",
		&book(text, &tree, &json!({"title": "notes.pdf"}), r"C:\Users\me\notes.pdf"),
	));
	assert_eq!(document.title, "notes");
	let pages: Vec<(usize, &str)> = markers(&document, MarkerType::PageBreak)
		.iter()
		.map(|marker| (marker.position.get(), marker.text.as_str()))
		.collect();
	assert_eq!(pages, [(0, "Page 1"), (12, "Page 2")]);
}

#[test]
fn turns_away_what_is_not_a_qread_book() {
	let dir = TempDir::new("qrd-not");
	let other = dir.path().join("other.qrd");
	Connection::open(&other).unwrap().execute("CREATE TABLE Notes (Id INTEGER PRIMARY KEY)", []).unwrap();
	assert!(QrdParser.parse(&ParserContext::new(other.to_string_lossy().into_owned())).is_err());
	let no_config = write_qrd(&dir, "no-config.qrd", &[("raw.text", "Text".to_string())]);
	assert!(QrdParser.parse(&ParserContext::new(no_config)).is_err());
	let garbage = dir.write("garbage.qrd", b"this is not an SQLite database at all, not even close");
	assert!(QrdParser.parse(&ParserContext::new(garbage.to_string_lossy().into_owned())).is_err());
}

/// As CHM help pages are laid out: a heading in one cell, the text in another, read as a page rather than a data table.
#[test]
fn a_table_laying_out_a_page_is_read_as_text() {
	let text = "Run\nRuns a program.\n";
	let at = |needle| units(text, needle);
	let end = at("program.").1;
	let tree = json!([
		{"type": "table", "id": 0, "parent": null, "start": 0, "end": end, "name": null, "attrs": {"width": "100%"}},
		{"type": "tr", "id": 1, "parent": 0, "start": 0, "end": end, "name": null, "attrs": {}},
		{"type": "td", "id": 2, "parent": 1, "start": 0, "end": at("Run\n").1, "name": null, "attrs": {}},
		{"type": "heading", "id": 3, "parent": null, "start": 0, "end": at("Run").1, "name": null, "level": "1", "tag": "h1"},
		{"type": "td", "id": 4, "parent": 1, "start": at("Runs").0, "end": end, "name": null, "attrs": {}},
		{"type": "tr", "id": 5, "parent": 0, "start": at("Runs").0, "end": end, "name": null, "attrs": {}},
		{"type": "td", "id": 6, "parent": 5, "start": at("Runs").0, "end": end, "name": null, "attrs": {}},
		{"type": "td", "id": 7, "parent": 5, "start": end, "end": end, "name": null, "attrs": {}},
	]);
	let dir = TempDir::new("qrd-layout");
	let document = open(write_qrd(&dir, "help.qrd", &book(text, &tree, &json!({"title": "Help"}), r"C:\help.chm")));
	assert!(markers(&document, MarkerType::Table).is_empty());
	assert_eq!(markers(&document, MarkerType::Heading1)[0].text, "Run");
}
