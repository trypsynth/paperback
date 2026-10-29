use super::*;
use crate::{
	document::{Document, MarkerType},
	util::test_support::TempDir,
};

fn parse_source(source: &str) -> Document {
	let dir = TempDir::new("mdoc-parser");
	let path = dir.write_str("page.1", source);
	ManParser.parse(&ParserContext::new(path)).expect("parse manual page")
}

fn text_of(source: &str) -> String {
	parse_source(source).buffer.content
}

fn page(body: &str) -> String {
	format!(".Dd January 1, 2026\n.Dt TEST 1\n.Os\n.Sh NAME\n.Nm test\n.Nd a page\n{body}")
}

fn covered(document: &Document, mtype: MarkerType) -> Vec<String> {
	document
		.buffer
		.markers
		.iter()
		.filter(|marker| marker.mtype == mtype)
		.map(|marker| document.buffer.text(marker.position..marker.position + marker.length).to_string())
		.collect()
}

fn tags_of(document: &Document) -> Vec<String> {
	document
		.buffer
		.markers
		.iter()
		.filter(|marker| marker.mtype == MarkerType::ListItem)
		.map(|marker| marker.text.clone())
		.collect()
}

/// The two packages share their file names, so which one a page is written in is read off its first macro.
#[test]
fn a_page_is_recognised_by_its_first_macro() {
	assert!(mdoc::is_mdoc(".\\\" a comment\n.Dd January 1, 2026\n.Dt TEST 1\n"));
	assert!(!mdoc::is_mdoc(".\\\" a comment\n.TH TEST 1\n.SH NAME\n"));
	assert!(!mdoc::is_mdoc("no macros at all\n"));
}

#[test]
fn the_name_section_reads_as_one_line() {
	let text = text_of(&page(""));
	assert!(text.contains("test - a page"), "{text}");
}

#[test]
fn the_title_is_the_name_and_section() {
	assert_eq!(parse_source(&page("")).title, "TEST(1)");
}

#[test]
fn macros_nest_inside_one_another() {
	let text = text_of(&page(".Sh SYNOPSIS\n.Nm\n.Op Fl f Ar file\n.Op Ar\n"));
	assert!(text.contains("test [-f file] [file ...]"), "{text}");
}

#[test]
fn flags_are_written_with_their_dash() {
	let document = parse_source(&page(".Sh DESCRIPTION\nUse\n.Fl b\nto number lines.\n"));
	assert!(document.buffer.content.contains("Use -b to number lines."), "{}", document.buffer.content);
	assert_eq!(covered(&document, MarkerType::Bold), ["-b"]);
}

#[test]
fn a_bare_name_stands_for_the_name_of_the_page() {
	let text = text_of(&page(".Sh DESCRIPTION\nThe\n.Nm\nutility does things.\n"));
	assert!(text.contains("The test utility does things."), "{text}");
}

#[test]
fn punctuation_sits_against_the_word_before_it() {
	let text = text_of(&page(".Sh SEE ALSO\n.Xr cat 1 ,\n.Xr ls 1\n"));
	assert!(text.contains("cat(1), ls(1)"), "{text}");
}

#[test]
fn punctuation_falls_outside_an_enclosure() {
	let text = text_of(&page(".Sh DESCRIPTION\nPrint tabs as\n.Ql ^I .\nDone.\n"));
	assert!(text.contains("Print tabs as \u{2018}^I\u{2019}. Done."), "{text}");
}

/// Every BSD page writes the current directory as `\&.`.
#[test]
fn an_escaped_full_stop_is_text() {
	let text = text_of(&page(".Sh DESCRIPTION\nEverything except\n.Sq \\&.\nand\n.Sq \\&.. .\n"));
	assert!(text.contains("except \u{2018}.\u{2019} and \u{2018}..\u{2019}."), "{text}");
}

#[test]
fn enclosures_put_their_marks_around_what_they_cover() {
	let text = text_of(&page(".Sh DESCRIPTION\n.Pq Ql x\n.Bq y\n.Dq z\n"));
	assert!(text.contains("(\u{2018}x\u{2019}) [y] \u{201C}z\u{201D}"), "{text}");
}

#[test]
fn a_tagged_list_gives_one_item_per_option() {
	let document = parse_source(&page(
		".Sh DESCRIPTION\n.Bl -tag -width Ds\n.It Fl a\nThe first.\n.It Fl b Ar file\nThe second.\n.El\n",
	));
	assert_eq!(tags_of(&document), ["-a", "-b file"]);
	assert!(document.buffer.content.contains("-a\nThe first."), "{}", document.buffer.content);
}

#[test]
fn a_bulleted_list_marks_each_item() {
	let document = parse_source(&page(".Sh DESCRIPTION\n.Bl -bullet\n.It\nFirst thing.\n.It\nSecond thing.\n.El\n"));
	assert_eq!(tags_of(&document), ["\u{2022}", "\u{2022}"]);
}

#[test]
fn a_numbered_list_counts_its_items() {
	let document = parse_source(&page(".Sh DESCRIPTION\n.Bl -enum\n.It\nFirst.\n.It\nSecond.\n.It\nThird.\n.El\n"));
	assert_eq!(tags_of(&document), ["1.", "2.", "3."]);
}

#[test]
fn sections_and_subsections_become_headings() {
	let document = parse_source(&page(".Sh DESCRIPTION\nSome prose.\n.Ss A subsection\nMore.\n"));
	let names: Vec<&str> = document.toc_items.iter().map(|item| item.name.as_str()).collect();
	assert_eq!(names, ["NAME", "DESCRIPTION"]);
	let children: Vec<&str> = document.toc_items[1].children.iter().map(|item| item.name.as_str()).collect();
	assert_eq!(children, ["A subsection"]);
}

#[test]
fn a_literal_display_keeps_its_lines() {
	let text = text_of(&page(".Sh EXAMPLES\n.Bd -literal -offset indent\nfirst line\nsecond line\n.Ed\n"));
	assert!(text.contains("first line\nsecond line"), "{text}");
}

#[test]
fn the_exit_status_macro_writes_its_sentence() {
	let text = text_of(&page(".Sh EXIT STATUS\n.Ex -std\n"));
	assert!(text.contains("The test utility exits 0 on success, and >0 if an error occurs."), "{text}");
}

#[test]
fn text_lines_join_into_one_paragraph() {
	let text = text_of(&page(".Sh DESCRIPTION\nThe first sentence.\nThe second sentence.\n"));
	assert!(text.contains("The first sentence. The second sentence."), "{text}");
}
