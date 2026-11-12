use std::{io::Write as _, path::Path};

use flate2::{Compression, write::GzEncoder};

use super::*;
use crate::{document::MarkerType, util::test_support::TempDir};

const PREAMBLE: &str = "This is test.info, produced by makeinfo version 7.1 from test.texi.\n\nINFO-DIR-SECTION Test\nSTART-INFO-DIR-ENTRY\n* Test: (test).   A test manual.\nEND-INFO-DIR-ENTRY\n\n";

const TOP: &str = "\x1f\nFile: test.info,  Node: Top,  Next: Intro,  Up: (dir)\n\nTest Manual\n***********\n\nWelcome to the manual.\n\n* Menu:\n\n* Intro::            Getting started.\n* Usage: Using it.   How to use it,\n                     step by step.\n* Other: (other)Elsewhere.   In another manual.\n\n";

const INTRO: &str = "\x1f\nFile: test.info,  Node: Intro,  Next: Using it,  Prev: Top,  Up: Top\n\n1 Introduction\n**************\n\nRead this first.  *Note Using it::, for more.  Also (*note Usage:\nUsing it.).  And *note intro::.\n\n";

const USING: &str = "\x1f\nFile: test.info,  Node: Using it,  Prev: Intro,  Up: Top\n\n2 Using it\n**********\n\n2.1 Details\n===========\n\nLine one.\nLine two \0\u{8}[image src=\"a.png\" alt=\"A \\\"quoted\\\" diagram\"\0\u{8}] here.\n\n\0\u{8}[index\0\u{8}]\n* Menu:\n\n* details:                               Using it.\n                                                              (line  9)\n\n";

fn manual() -> String {
	format!("{PREAMBLE}{TOP}{INTRO}{USING}\x1f\nTag Table:\nNode: Top\x7f1\n\x1f\nEnd Tag Table\n")
}

fn parse_path(path: &str) -> Document {
	InfoParser.parse(&ParserContext::new(path.to_owned())).expect("parse Info manual")
}

fn parse(source: &str) -> Document {
	let dir = TempDir::new("info-parser");
	parse_path(&dir.write_str("test.info", source))
}

fn markers(document: &Document, mtype: MarkerType) -> Vec<&Marker> {
	document.buffer.markers.iter().filter(|marker| marker.mtype == mtype).collect()
}

fn line_at(document: &Document, position: DocPos) -> &str {
	let content = &document.buffer.content;
	let start = content.char_indices().nth(position.get()).map_or(content.len(), |(i, _)| i);
	content[start..].lines().next().unwrap_or_default()
}

fn link_target<'a>(document: &'a Document, text: &str) -> Option<&'a str> {
	markers(document, MarkerType::Link).into_iter().find(|link| link.text == text).map(|link| link.reference.as_str())
}

fn resolves_to<'a>(document: &'a Document, reference: &str) -> &'a str {
	let key = reference.strip_prefix('#').expect("an internal link");
	line_at(document, *document.id_positions.get(key).expect("the link's target is known"))
}

#[test]
fn nodes_read_without_their_headers_and_underlines() {
	let document = parse(&manual());
	let text = &document.buffer.content;
	assert!(!text.contains("File: test.info"), "{text}");
	assert!(!text.contains("****"), "{text}");
	assert!(!text.contains("INFO-DIR-SECTION"), "the preamble is not part of the manual: {text}");
	assert!(text.starts_with("Test Manual\n\nWelcome to the manual.\n\n"), "{text}");
	assert_eq!(document.title, "Test Manual");
}

#[test]
fn underlined_titles_are_headings_by_their_underline() {
	let document = parse(&manual());
	let headings: Vec<(i32, &str)> = document
		.buffer
		.markers
		.iter()
		.filter(|marker| (MarkerType::Heading1 as i32..=MarkerType::Heading6 as i32).contains(&(marker.mtype as i32)))
		.map(|marker| (marker.level, marker.text.as_str()))
		.collect();
	assert_eq!(headings, [(1, "Test Manual"), (1, "1 Introduction"), (1, "2 Using it"), (2, "2.1 Details")]);
}

#[test]
fn a_menu_is_a_list_of_links_to_its_nodes() {
	let document = parse(&manual());
	let text = &document.buffer.content;
	assert!(text.contains("Intro: Getting started.\n"), "{text}");
	assert!(text.contains("Usage: How to use it, step by step.\n"), "{text}");
	assert!(!text.contains("* Menu:"), "{text}");
	assert_eq!(link_target(&document, "Intro"), Some("#Intro"));
	assert_eq!(link_target(&document, "Usage"), Some("#Using it"));
	assert_eq!(resolves_to(&document, "#Using it"), "2 Using it");
	let items: Vec<&str> = markers(&document, MarkerType::ListItem).iter().map(|item| item.text.as_str()).collect();
	assert_eq!(items, ["Intro", "Usage", "Other", "details"]);
	assert_eq!(markers(&document, MarkerType::List).len(), 2);
}

#[test]
fn an_entry_in_another_manual_is_not_a_link() {
	let document = parse(&manual());
	assert!(document.buffer.content.contains("Other: In another manual.\n"));
	assert_eq!(link_target(&document, "Other"), None);
}

#[test]
fn cross_references_read_as_see_and_link_their_node() {
	let document = parse(&manual());
	let text = &document.buffer.content;
	assert!(text.contains("Read this first.  See Using it, for more.  Also (see Usage).  And see intro.\n"), "{text}");
	assert_eq!(link_target(&document, "Using it"), Some("#Using it"));
	// `info` matches a node name without regard to case when nothing matches exactly.
	assert_eq!(link_target(&document, "intro"), Some("#Intro"));
}

#[test]
fn emphasis_that_starts_like_a_reference_keeps_its_lines() {
	let node = "\x1f\nFile: test.info,  Node: Top,  Up: (dir)\n\n*Note* that this is\nnot a reference.\n";
	let document = parse(&format!("{PREAMBLE}{node}"));
	assert_eq!(document.buffer.content, "*Note* that this is\nnot a reference.\n");
}

#[test]
fn a_reference_keyword_at_the_end_of_a_line_takes_the_next() {
	let node = "\x1f\nFile: test.info,  Node: Top,  Up: (dir)\n\nRead *note\nTop::.\n";
	let document = parse(&format!("{PREAMBLE}{node}"));
	assert_eq!(document.buffer.content, "Read see Top.\n");
}

#[test]
fn an_index_entry_links_to_its_line() {
	let document = parse(&manual());
	assert!(document.buffer.content.contains("details: Using it\n"), "{}", document.buffer.content);
	let reference = link_target(&document, "details").expect("the index entry is a link");
	assert_eq!(resolves_to(&document, reference), "Line one.");
}

#[test]
fn an_image_reads_as_its_description() {
	let document = parse(&manual());
	assert!(document.buffer.content.contains("Line two A \"quoted\" diagram here.\n"), "{}", document.buffer.content);
	assert!(!document.buffer.content.contains('\0'));
}

#[test]
fn the_contents_follow_each_nodes_up() {
	let document = parse(&manual());
	let names: Vec<&str> = document.toc_items.iter().map(|item| item.name.as_str()).collect();
	assert_eq!(names, ["Test Manual", "1 Introduction", "2 Using it"]);
	assert_eq!(line_at(&document, document.toc_items[1].offset), "1 Introduction");
}

#[test]
fn nested_nodes_nest_in_the_contents() {
	let child =
		"\x1f\nFile: test.info,  Node: Deeper,  Prev: Using it,  Up: Using it\n\n2.2 Deeper\n==========\n\nMore.\n";
	let document = parse(&format!("{PREAMBLE}{TOP}{INTRO}{USING}{child}"));
	let using = &document.toc_items[2];
	assert_eq!(using.children.iter().map(|item| item.name.as_str()).collect::<Vec<_>>(), ["2.2 Deeper"]);
}

fn gzip(text: &str) -> Vec<u8> {
	let mut encoder = GzEncoder::new(Vec::new(), Compression::fast());
	encoder.write_all(text.as_bytes()).unwrap();
	encoder.finish().unwrap()
}

/// A manual split as makeinfo splits a large one: the main file has only the tables, each piece repeats the preamble, and offsets count the pieces as one file without their preambles.
fn write_split_manual(dir: &TempDir) {
	let anchor_line = "Here is the anchor line.\n";
	let using = USING.replace("Line one.\n", &format!("Line one.\n{anchor_line}"));
	let first = format!("{PREAMBLE}{TOP}{INTRO}");
	let second = format!("{PREAMBLE}{using}");
	let first_start = 100;
	let second_start = first_start + (first.len() - PREAMBLE.len());
	let anchor = second_start + using.find(anchor_line).unwrap();
	let main = format!(
		"{PREAMBLE}\x1f\nIndirect:\ntest.info-1: {first_start}\ntest.info-2: {second_start}\n\x1f\nTag Table:\n(Indirect)\nNode: Top\x7f{first_start}\nRef: The anchor\x7f{anchor}\n\x1f\nEnd Tag Table\n"
	);
	dir.write("test.info.gz", gzip(&main));
	dir.write("test.info-1.gz", gzip(&first));
	dir.write("test.info-2.gz", gzip(&second));
}

#[test]
fn a_split_manual_reads_all_its_pieces() {
	let dir = TempDir::new("info-split");
	write_split_manual(&dir);
	let document = parse_path(&dir.join_str("test.info.gz"));
	let names: Vec<&str> = document.toc_items.iter().map(|item| item.name.as_str()).collect();
	assert_eq!(names, ["Test Manual", "1 Introduction", "2 Using it"]);
	let anchor = *document.id_positions.get("The anchor").expect("the anchor is placed");
	assert_eq!(line_at(&document, anchor), "Here is the anchor line.");
}

#[test]
fn opening_a_piece_opens_its_whole_manual() {
	let dir = TempDir::new("info-piece");
	write_split_manual(&dir);
	let document = parse_path(&dir.join_str("test.info-2.gz"));
	assert!(document.buffer.content.starts_with("Test Manual\n"), "{}", document.buffer.content);
}

#[test]
fn a_missing_piece_is_an_error_naming_it() {
	let dir = TempDir::new("info-missing-piece");
	write_split_manual(&dir);
	std::fs::remove_file(dir.path().join("test.info-2.gz")).unwrap();
	let error = InfoParser.parse(&ParserContext::new(dir.join_str("test.info.gz"))).unwrap_err();
	assert!(format!("{error:#}").contains("test.info-2"), "{error:#}");
}

#[test]
fn text_that_is_not_a_manual_is_turned_away() {
	let dir = TempDir::new("info-not");
	let path = dir.write_str("notes.info", "Just some notes.\nNo nodes here.\n");
	assert!(InfoParser.parse(&ParserContext::new(path)).is_err());
}

#[test]
fn info_files_and_their_pieces_are_known_by_name() {
	for name in ["sed.info", "sed.info.gz", "find.info-2.gz", "find.info-12", "GNUPG.INFO"] {
		assert!(is_info_name(Path::new(name)), "{name}");
	}
	for name in ["info", "sed.gz", "notes.info-x", "readme.txt", ".info-1"] {
		assert!(!is_info_name(Path::new(name)), "{name}");
	}
}

#[test]
fn a_node_name_with_a_full_stop_inside_stays_whole() {
	assert_eq!(split_node("sed.info here.  Rest"), ("sed.info here".to_owned(), "  Rest"));
	assert_eq!(split_node("Node, rest"), ("Node".to_owned(), " rest"));
	assert_eq!(split_node("\x7fa: b.c\x7f.  Rest"), ("a: b.c".to_owned(), "  Rest"));
}

#[test]
fn header_fields_are_read_by_name() {
	let header = "File: sed.info,  Node: Top,  Next: Introduction,  Up: (dir)";
	assert_eq!(header_field(header, "Node").as_deref(), Some("Top"));
	assert_eq!(header_field(header, "Next").as_deref(), Some("Introduction"));
	assert_eq!(header_field(header, "Up").as_deref(), Some("(dir)"));
	assert_eq!(header_field(header, "Prev"), None);
	assert_eq!(header_field("File: a,  Node: \x7fx, y\x7f,  Up: Top", "Node").as_deref(), Some("x, y"));
}
