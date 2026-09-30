use std::{io::Write as _, path::Path};

use flate2::{Compression, write::GzEncoder};

use super::*;
use crate::{
	document::{Document, MarkerType},
	util::test_support::TempDir,
};

fn parse_source(source: &str) -> Document {
	let dir = TempDir::new("man-parser");
	let path = dir.write_str("page.1", source);
	ManParser.parse(&ParserContext::new(path)).expect("parse manual page")
}

fn text_of(source: &str) -> String {
	parse_source(source).buffer.content
}

/// The macros every page opens with, so a test can be about the one thing it is testing.
fn page(body: &str) -> String {
	format!(".TH TEST 1 \"1 January 2026\" \"Paperback\" \"Test Pages\"\n.SH NAME\ntest \\- a page\n{body}")
}

fn markers_of(document: &Document, mtype: MarkerType) -> Vec<(usize, String, usize)> {
	document
		.buffer
		.markers
		.iter()
		.filter(|marker| marker.mtype == mtype)
		.map(|marker| (marker.position, marker.text.clone(), marker.length))
		.collect()
}

/// The text a marker covers, which is what a reader would hear announced.
fn covered(document: &Document, mtype: MarkerType) -> Vec<String> {
	document
		.buffer
		.markers
		.iter()
		.filter(|marker| marker.mtype == mtype)
		.map(|marker| document.buffer.content.chars().skip(marker.position).take(marker.length).collect())
		.collect()
}

/// Roff fills text: the line breaks in the source are where the author stopped typing, not where
/// the paragraph breaks. A page written one sentence to a line has to read as prose.
#[test]
fn text_lines_join_into_one_paragraph() {
	let text = text_of(&page(".SH DESCRIPTION\nThe first sentence.\nThe second sentence.\n"));
	assert!(text.contains("The first sentence. The second sentence."), "{text}");
}

/// A blank line ends a paragraph, as it does in roff.
#[test]
fn a_blank_line_ends_the_paragraph() {
	let text = text_of(&page(".SH DESCRIPTION\nFirst paragraph.\n\nSecond paragraph.\n"));
	assert!(text.contains("First paragraph.\n\nSecond paragraph."), "{text}");
}

/// `.nf` turns filling off, and there the line breaks are the content: a synopsis, an example,
/// or anything else laid out by hand.
#[test]
fn a_no_fill_block_keeps_its_lines() {
	let text = text_of(&page(".SH EXAMPLE\n.nf\nint main(void)\n{\n    return 0;\n}\n.fi\nBack to prose.\n"));
	assert!(text.contains("int main(void)\n{\n    return 0;\n}"), "{text}");
	assert!(text.contains("Back to prose."), "{text}");
}

/// `.EX` and `.EE` are the same thing under another name, and are what a modern page uses.
#[test]
fn an_example_block_keeps_its_lines() {
	let text = text_of(&page(".SH EXAMPLE\n.EX\nfirst line\nsecond line\n.EE\n"));
	assert!(text.contains("first line\nsecond line"), "{text}");
}

#[test]
fn sections_and_subsections_become_headings() {
	let document = parse_source(&page(".SH OPTIONS\nSome options.\n.SS Long options\nMore.\n"));
	let sections: Vec<String> = markers_of(&document, MarkerType::Heading1).into_iter().map(|(_, t, _)| t).collect();
	assert_eq!(sections, ["NAME", "OPTIONS"]);
	let subsections: Vec<String> = markers_of(&document, MarkerType::Heading2).into_iter().map(|(_, t, _)| t).collect();
	assert_eq!(subsections, ["Long options"]);
}

/// A subsection belongs under the section it falls in, so the contents of a page reads the way
/// the page is laid out.
#[test]
fn the_table_of_contents_nests_subsections() {
	let document = parse_source(&page(".SH OPTIONS\nSome options.\n.SS Long options\nMore.\n.SH FILES\nSome files.\n"));
	let names: Vec<&str> = document.toc_items.iter().map(|item| item.name.as_str()).collect();
	assert_eq!(names, ["NAME", "OPTIONS", "FILES"]);
	let children: Vec<&str> = document.toc_items[1].children.iter().map(|item| item.name.as_str()).collect();
	assert_eq!(children, ["Long options"]);
}

/// `.SH` with no argument takes the line after it as the heading.
#[test]
fn a_heading_can_be_on_the_line_after_its_macro() {
	let document = parse_source(&page(".SH\nSEE ALSO\nls(1)\n"));
	let sections: Vec<String> = markers_of(&document, MarkerType::Heading1).into_iter().map(|(_, t, _)| t).collect();
	assert_eq!(sections, ["NAME", "SEE ALSO"]);
}

/// `.TP` is how a page writes an option: the tag on one line, what it means under it. Each tag
/// is a list item, so the options of a page can be walked one at a time.
#[test]
fn tagged_paragraphs_become_list_items() {
	let document =
		parse_source(&page(".SH OPTIONS\n.TP\n.B \\-a\nDo the first thing.\n.TP\n.B \\-b\nDo the second thing.\n"));
	let tags: Vec<String> = markers_of(&document, MarkerType::ListItem).into_iter().map(|(_, t, _)| t).collect();
	assert_eq!(tags, ["-a", "-b"]);
	assert!(document.buffer.content.contains("-a\nDo the first thing."), "{}", document.buffer.content);
}

/// A run of tagged paragraphs is a list, which the reader can step over rather than through.
#[test]
fn a_run_of_tagged_paragraphs_is_one_list() {
	let document = parse_source(&page(".SH OPTIONS\n.TP\n.B \\-a\nFirst.\n.TP\n.B \\-b\nSecond.\n.SH FILES\nDone.\n"));
	let lists = markers_of(&document, MarkerType::List);
	assert_eq!(lists.len(), 1, "{lists:?}");
	assert!(lists[0].2 > 0, "the list covers the items in it");
}

/// A lone tagged paragraph is not a list worth navigating into.
#[test]
fn a_single_tagged_paragraph_is_not_a_list() {
	let document = parse_source(&page(".SH FILES\n.TP\n.I /etc/passwd\nThe password file.\n"));
	assert!(markers_of(&document, MarkerType::List).is_empty());
	assert_eq!(markers_of(&document, MarkerType::ListItem).len(), 1);
}

/// `.IP` with a tag is a list item too; without one it is only an indented paragraph.
#[test]
fn indented_paragraphs_are_items_only_when_tagged() {
	let document = parse_source(&page(".SH NOTES\n.IP \\(bu\nA bullet.\n.IP\nJust indented.\n"));
	let tags: Vec<String> = markers_of(&document, MarkerType::ListItem).into_iter().map(|(_, t, _)| t).collect();
	assert_eq!(tags, ["\u{2022}"]);
}

/// The alternating font macros put no space between their arguments, which is how a page writes
/// a cross-reference to another page.
#[test]
fn a_cross_reference_reads_as_one_word() {
	let document = parse_source(&page(".SH SEE ALSO\n.BR ls (1),\n.BR cat (1)\n"));
	assert!(document.buffer.content.contains("ls(1), cat(1)"), "{}", document.buffer.content);
	assert_eq!(covered(&document, MarkerType::Bold), ["ls", "cat"]);
}

#[test]
fn font_macros_and_escapes_mark_their_runs() {
	let document = parse_source(&page(".SH DESCRIPTION\n.B bold words\n.I italic words\nplain \\fBinline\\fP plain\n"));
	assert_eq!(covered(&document, MarkerType::Bold), ["bold words", "inline"]);
	assert_eq!(covered(&document, MarkerType::Italic), ["italic words"]);
}

/// `.B` with nothing after it sets the line below it instead.
#[test]
fn a_bare_font_macro_sets_the_next_line() {
	let document = parse_source(&page(".SH DESCRIPTION\n.B\nthe whole line\n"));
	assert_eq!(covered(&document, MarkerType::Bold), ["the whole line"]);
}

#[test]
fn escapes_become_the_characters_they_name() {
	let text = text_of(&page(".SH DESCRIPTION\n\\-\\-help \\(bu \\(em \\e \\(aq \\&nothing \\fR\n"));
	assert!(text.contains("--help \u{2022} \u{2014} \\ ' nothing"), "{text}");
}

#[test]
fn comments_and_unknown_macros_leave_nothing_behind() {
	let text = text_of(&page(".\\\" a comment\n.SH DESCRIPTION\n.ad l\n.hy 0\nOnly this.\n.\\\" another\n"));
	assert!(text.contains("Only this."), "{text}");
	assert!(!text.contains("comment"), "{text}");
	assert!(!text.contains("hy"), "{text}");
}

/// A macro definition is instructions for the typesetter, not text, and runs until `..`.
#[test]
fn a_macro_definition_is_skipped_whole() {
	let text = text_of(&page(".de XX\nthis is inside the definition\n..\n.SH DESCRIPTION\nOnly this.\n"));
	assert!(!text.contains("inside the definition"), "{text}");
	assert!(text.contains("Only this."), "{text}");
}

/// A line ending in a backslash continues onto the next one.
#[test]
fn a_continued_line_is_read_as_one() {
	let text = text_of(&page(".SH DESCRIPTION\n.B one \\\ntwo\n"));
	assert!(text.contains("one two"), "{text}");
}

#[test]
fn a_link_carries_its_target() {
	let document =
		parse_source(&page(".SH SEE ALSO\nThe\n.UR https://example.com/page\nproject page\n.UE\nhas more.\n"));
	let links: Vec<(String, String)> = document
		.buffer
		.markers
		.iter()
		.filter(|marker| marker.mtype == MarkerType::Link)
		.map(|marker| (marker.text.clone(), marker.reference.clone()))
		.collect();
	assert_eq!(links, [("project page".to_string(), "https://example.com/page".to_string())]);
	assert!(document.buffer.content.contains("The project page has more."), "{}", document.buffer.content);
}

#[test]
fn an_address_becomes_a_mailto_link() {
	let document = parse_source(&page(".SH AUTHOR\nWritten by\n.MT someone@example.com\nSomeone\n.ME .\n"));
	let targets: Vec<String> = document
		.buffer
		.markers
		.iter()
		.filter(|marker| marker.mtype == MarkerType::Link)
		.map(|marker| marker.reference.clone())
		.collect();
	assert_eq!(targets, ["mailto:someone@example.com"]);
}

/// A page is named by its name and section together, the way one page refers to another.
#[test]
fn the_title_is_the_name_and_section() {
	assert_eq!(parse_source(&page("")).title, "TEST(1)");
}

#[test]
fn a_page_without_a_title_header_falls_back_to_its_file_name() {
	let dir = TempDir::new("man-parser");
	let path = dir.write_str("mytool.1", ".SH NAME\nmytool \\- does things\n");
	let document = ManParser.parse(&ParserContext::new(path)).expect("parse manual page");
	assert_eq!(document.title, "mytool");
}

/// An installed page is gzipped, and nothing in the name says so beyond the extension.
#[test]
fn a_gzipped_page_is_unpacked() {
	let dir = TempDir::new("man-parser");
	let mut encoder = GzEncoder::new(Vec::new(), Compression::fast());
	encoder.write_all(page(".SH DESCRIPTION\nGzipped prose.\n").as_bytes()).unwrap();
	let path = dir.write_str("page.1.gz", encoder.finish().unwrap());
	let document = ManParser.parse(&ParserContext::new(path)).expect("parse gzipped manual page");
	assert_eq!(document.title, "TEST(1)");
	assert!(document.buffer.content.contains("Gzipped prose."), "{}", document.buffer.content);
}

/// `gz` is claimed so that an installed page can be offered in an open dialog at all, so a
/// gzipped anything else reaches this parser and has to be turned away rather than read.
#[test]
fn a_gzipped_file_that_is_not_a_page_is_refused() {
	let dir = TempDir::new("man-parser");
	let mut encoder = GzEncoder::new(Vec::new(), Compression::fast());
	encoder.write_all(b"just some prose in a text file\nwith no roff in it at all\n").unwrap();
	let path = dir.write_str("notes.txt.gz", encoder.finish().unwrap());
	let error = ManParser.parse(&ParserContext::new(path)).expect_err("not a manual page");
	assert!(error.to_string().contains("not a manual page"), "{error}");
}

#[test]
fn a_section_number_names_a_manual_page() {
	assert!(is_manual_page_name(Path::new("/usr/share/man/man1/ls.1")));
	assert!(is_manual_page_name(Path::new("printf.3.gz")));
	assert!(is_manual_page_name(Path::new("Tcl_Init.3tcl")));
	assert!(is_manual_page_name(Path::new("perlfunc.1p")));
	assert!(!is_manual_page_name(Path::new("notes.txt")));
	assert!(!is_manual_page_name(Path::new("archive.tar.gz")));
	assert!(!is_manual_page_name(Path::new("README")));
	assert!(!is_manual_page_name(Path::new("page.man")));
}

/// A table is a table: the same grid every other format hands to the shared renderer, so a
/// page's table can be stepped into and read a row at a time. The lines describing the shape
/// of it are not content and do not survive.
#[test]
fn a_table_becomes_a_table() {
	let document = parse_source(&page(
		".SH TABLE
.TS
allbox;
lb lb
l l.
Interface\tValue
T{
.B printf
T}\tsafe
.TE
After.
",
	));
	let tables = markers_of(&document, MarkerType::Table);
	assert_eq!(tables.len(), 1, "{tables:?}");
	assert_eq!(tables[0].1, "Interface Value", "the caption names the table by its first row");
	let text = &document.buffer.content;
	assert!(text.contains("Interface\tValue"), "{text}");
	assert!(text.contains("printf\tsafe"), "{text}");
	assert!(!text.contains("allbox"), "{text}");
	assert!(!text.contains("T{"), "{text}");
	assert!(text.contains("After."), "{text}");
}

/// With tables set to stand as placeholders, a page's table does too.
#[test]
fn a_table_can_stand_as_a_placeholder() {
	let dir = TempDir::new("man-parser");
	let path = dir.write_str(
		"page.1",
		page(
			".SH TABLE
.TS
l l.
One\tTwo
.TE
",
		),
	);
	let context = ParserContext::new(path).with_render_tables_inline(false);
	let document = ManParser.parse(&context).expect("parse manual page");
	assert!(document.buffer.content.contains("[Table]"), "{}", document.buffer.content);
	assert_eq!(markers_of(&document, MarkerType::Table).len(), 1);
}

/// A separator other than the tab is named in the options line.
#[test]
fn a_table_can_name_its_own_separator() {
	let text = text_of(&page(
		".SH TABLE
.TS
center tab(:);
l l.
One:Two
.TE
",
	));
	assert!(text.contains("One\tTwo"), "{text}");
}
