use super::*;
use crate::util::test_support::TempDir;

fn parse_with(
	parser: &dyn Parser,
	name: &str,
	contents: impl AsRef<[u8]>,
	context: impl FnOnce(ParserContext) -> ParserContext,
) -> Result<Document> {
	let dir = TempDir::new("braille-parser");
	let path = dir.write_str(name, contents);
	parser.parse(&context(ParserContext::new(path)))
}

fn parse(parser: &dyn Parser, name: &str, contents: impl AsRef<[u8]>) -> Document {
	parse_with(parser, name, contents, |context| context).expect("parse braille document")
}

fn page_markers(document: &Document) -> Vec<(usize, String)> {
	document
		.buffer
		.markers
		.iter()
		.filter(|marker| marker.mtype == MarkerType::PageBreak)
		.map(|marker| (marker.position.get(), marker.text.clone()))
		.collect()
}

fn cells(ascii: &str) -> String {
	ascii.bytes().map(ascii_to_cell).collect()
}

fn layout(pages: &[&[&str]], strip_running_heads: bool) -> Layout {
	let pages: Vec<Page> = pages.iter().map(|page| page.iter().map(|line| cells(line)).collect()).collect();
	Layout::of(&pages, strip_running_heads)
}

#[test]
fn braille_ascii_names_each_of_the_64_cells_once() {
	let mut characters = BRAILLE_ASCII.to_vec();
	characters.sort_unstable();
	characters.dedup();
	assert_eq!(characters.len(), 64);
	for (dots, &character) in BRAILLE_ASCII.iter().enumerate() {
		assert_eq!(u32::from(ascii_to_cell(character)), 0x2800 + u32::try_from(dots).unwrap());
	}
}

/// Many BRF files write the letters, and the four signs above them, in lowercase.
#[test]
fn reads_lowercase_as_the_same_cells() {
	assert_eq!(cells("abc `{|}~"), cells("ABC @[\\]^"));
}

#[test]
fn back_translates_with_the_default_table() {
	let document = parse(&BrfParser, "orbit.brf", ",! ,ORBIT ,R1D] #BJ IS ! _W'S MO/\n");
	assert_eq!(document.buffer.content, "The Orbit Reader 20 is the world's most\n");
}

#[test]
fn back_translates_with_the_table_asked_for() {
	let document =
		parse_with(&BrfParser, "g1.brf", "AB\n", |context| ParserContext { braille_table: "en-ueb-g1.ctb", ..context })
			.unwrap();
	assert_eq!(document.buffer.content, "ab\n");
	assert_eq!(parse(&BrfParser, "g2.brf", "AB\n").buffer.content, "about\n");
}

#[test]
fn refuses_a_table_it_does_not_have() {
	let error = parse_with(&BrfParser, "book.brf", "AB\n", |context| ParserContext {
		braille_table: "no-such-table.ctb",
		..context
	})
	.unwrap_err();
	assert!(error.to_string().contains("no-such-table.ctb"), "{error}");
}

/// A paragraph that runs onto the next page is one paragraph, and that page's marker falls on its first word there.
#[test]
fn joins_a_paragraph_across_a_page_and_marks_where_the_page_begins() {
	let document = parse(&BrfParser, "pages.brf", "  HELLO\r\n\x0c_W\r\n\x0c");
	assert_eq!(document.buffer.content, "hello world\n");
	assert_eq!(page_markers(&document), [(0, "Page 1".to_string()), (6, "Page 2".to_string())]);
}

/// Older BRF files, such as the Internet Archive's, end their lines with a carriage return alone.
#[test]
fn reads_lines_ended_by_a_carriage_return_alone() {
	let document = parse(&BrfParser, "cr.brf", "  HELLO\r_W\r\r  ,! ,ORBIT\r");
	assert_eq!(document.buffer.content, "hello world\nThe Orbit\n");
}

/// French braille ASCII writes the capital sign as ¨ and é as itself, as `fr-bfu-comp6.utb` defines them.
#[test]
fn reads_the_braille_ascii_of_the_tables_country() {
	let french = |context: ParserContext| ParserContext { braille_table: "fr-bfu-comp6.utb", ..context };
	let utf8 = parse_with(&BrfParser, "fr.brf", "¨le ¨président\n", french).unwrap();
	assert_eq!(utf8.buffer.content, "Le Président\n");
	let windows_1252 = parse_with(&BrfParser, "fr.brf", b"\xa8le \xa8pr\xe9sident\n", french).unwrap();
	assert_eq!(windows_1252.buffer.content, "Le Président\n");
}

/// DAISY Pipeline writes Norwegian with Braillo's table, whose capital sign is `*` rather than liblouis's `´`.
#[test]
fn reads_norwegian_written_with_another_countrys_braille_ascii() {
	let norwegian = |context: ParserContext| ParserContext { braille_table: "no-no-g0.utb", ..context };
	let document = parse_with(&BrfParser, "no.brf", "  *HEI *VERDEN, DETTE ER EN BOK.\r\n", norwegian).unwrap();
	assert_eq!(document.buffer.content, "Hei Verden, dette er en bok.\n");
}

/// Danish files are written in code page 850, so æ, ø and å are bytes Windows-1252 has other characters for.
#[test]
fn reads_danish_written_in_code_page_850() {
	let danish = |context: ParserContext| ParserContext { braille_table: "da-dk-g16.ctb", ..context };
	let document =
		parse_with(&BrfParser, "da.brf", b"det skal v\x91re en bog om s\x9b og \x86er.\r\n", danish).unwrap();
	assert_eq!(document.buffer.content, "det skal være en bog om sø og åer.\n");
}

/// French braille ASCII writes the digits as themselves, as `fr-bfu-comp6.utb` defines them, rather than as the lowered North American cells.
#[test]
fn reads_digits_in_the_braille_ascii_of_the_tables_country() {
	let document = parse_with(&BrfParser, "fr.brf", "000 morts\n", |context| ParserContext {
		braille_table: "fr-bfu-comp6.utb",
		..context
	})
	.unwrap();
	assert_eq!(document.buffer.content, "000 morts\n");
}

/// Slovak braille files are written in North American Braille ASCII, although `sk-g1.ctb` defines `,` as dot 2 and `*` as dots 3 and 5 and has no `display` rules.
#[test]
fn reads_slovak_files_as_north_american_braille_ascii() {
	let document = parse_with(&BrfParser, "sk.brf", ",betsabe3 \\a!ko sk+:an* kr*sa\n", |context| ParserContext {
		braille_table: "sk-g1.ctb",
		..context
	})
	.unwrap();
	assert_eq!(document.buffer.content, "Betsabe: ťažko skúšaná krása\n");
}

/// A Czech file in North American Braille ASCII reads as one, rather than in Dotify's Czech braille ASCII, as `cs-g1.ctb` defines the same cells as `sk-g1.ctb` for it; the letters here are shared by both languages.
#[test]
fn reads_czech_files_as_north_american_braille_ascii() {
	let document = parse_with(&BrfParser, "cs.brf", ",autor3 ,angela ,hunt1 \\a!ko sk+:an* kr*sa\n", |context| {
		ParserContext { braille_table: "cs-g1.ctb", ..context }
	})
	.unwrap();
	assert_eq!(document.buffer.content, "Autor: Angela Hunt, ťažko skúšaná krása\n");
}

/// Hungarian braille files are written in North American Braille ASCII, although `hu-hu-g1.ctb` writes `1` as dots 1 and 6 and `@` as dots 4 and 5, and has no cell for `*`.
#[test]
fn reads_hungarian_files_as_north_american_braille_ascii() {
	let document = parse_with(
		&BrfParser,
		"hu.brf",
		".:*P ]:I *G3 .@D@M1 .*VA *S .?URI +JRA ITT VOLT1 DE NEM :[LT SEMMIT'\n",
		|context| ParserContext { braille_table: "hu-hu-g1.ctb", ..context },
	)
	.unwrap();
	assert_eq!(document.buffer.content, "Szép őszi ég: Ádám, Éva és Gyuri újra itt volt, de nem szólt semmit.\n");
}

/// A table's braille ASCII reads a NUL or a tab as the blank cell, so another country's has to as well, or a single one on a title page or a blank line would leave it a character unread and lose it the file.
#[test]
fn reads_nul_and_tabs_as_blank_in_another_countrys_braille_ascii() {
	let line = ".:*P ]:I *G3 .@D@M1 .*VA *S .?URI +JRA ITT VOLT1 DE NEM :[LT SEMMIT'\n";
	let document = parse_with(&BrfParser, "hu.brf", format!("\0\n{line}\t\n{line}"), |context| ParserContext {
		braille_table: "hu-hu-g1.ctb",
		..context
	})
	.unwrap();
	assert!(document.buffer.content.contains("Szép őszi ég: Ádám, Éva"), "{:?}", document.buffer.content);
}

/// UEB's tables display a capital A as the computer braille cell with dot 7, which is no six-dot braille file's.
#[test]
fn reads_capitals_as_north_american_braille_ascii_for_english() {
	assert_eq!(parse(&BrfParser, "upper.brf", "HELLO\n").buffer.content, "hello\n");
}

#[test]
fn reads_a_brf_written_in_unicode_braille() {
	let document = parse(&BrfParser, "unicode.brf", "  ⠓⠑⠇⠇⠕\r\n⠸⠺\r\n");
	assert_eq!(document.buffer.content, "hello world\n");
}

#[test]
fn joins_wrapped_lines_and_words_broken_at_a_hyphen() {
	let layout = layout(&[&["  SELF-", "3TA9$ BOOK", "R1D]"]], true);
	assert_eq!(layout.paragraphs, [cells("SELF-3TA9$ BOOK R1D]")]);
}

#[test]
fn keeps_the_space_after_a_dash_that_ends_a_line() {
	let layout = layout(&[&["  HAIR,-", ",CAN'T", "M]E--", "?AN"]], true);
	assert_eq!(layout.paragraphs, [cells("HAIR,- ,CAN'T M]E-- ?AN")]);
}

#[test]
fn starts_a_paragraph_at_an_indent_or_a_blank_line() {
	let layout = layout(&[&["  ONE", "TWO", "  ?REE", "", "F\\R"]], true);
	assert_eq!(layout.paragraphs, [cells("ONE TWO"), cells("?REE"), cells("F\\R")]);
}

#[test]
fn keeps_a_centered_heading_apart_from_the_text_after_it() {
	let layout = layout(&[&["          ,! ,ORBIT", ",! ,ORBIT IS"]], true);
	assert_eq!(layout.paragraphs, [cells(",! ,ORBIT"), cells(",! ,ORBIT IS")]);
}

#[test]
fn drops_page_numbers_and_padding() {
	let layout = layout(&[&["                                      #A", "  ONE  ", "TWO         #A#B", "", ""]], true);
	assert_eq!(layout.paragraphs, [cells("ONE TWO")]);
}

/// A continuation page numbers itself with a letter before the number, as in ⠁⠼⠁.
#[test]
fn drops_a_continuation_page_number() {
	assert_eq!(without_page_number(&cells("SERVICES HAVE SOME       A#A")), cells("SERVICES HAVE SOME"));
	assert_eq!(without_page_number(&cells("IN #ABC YE>S")), cells("IN #ABC YE>S"));
}

#[test]
fn drops_a_running_head_only_when_asked() {
	let pages: &[&[&str]] = &[&["  ONE"], &["   ,HEAD", "TWO"], &["   ,HEAD", "?REE"], &["   ,HEAD", "F\\R"]];
	assert_eq!(layout(pages, true).paragraphs, [cells("ONE TWO ?REE F\\R")]);
	assert_eq!(
		layout(pages, false).paragraphs,
		[cells("ONE"), cells(",HEAD TWO"), cells(",HEAD ?REE"), cells(",HEAD F\\R")]
	);
}

#[test]
fn marks_pages_without_text_where_the_next_text_starts() {
	let layout = layout(&[&["  ONE"], &[], &["TWO"], &[]], true);
	assert_eq!(layout.page_starts, [Some((0, 0)), Some((0, 4)), Some((0, 4)), None]);
}

#[test]
fn separates_a_print_page_line_from_the_text_around_it() {
	let layout = layout(&[&["  ONE", "-----------------------#BC", "TWO"]], true);
	assert_eq!(layout.paragraphs, [cells("ONE"), cells("-----------------------#BC"), cells("TWO")]);
}

#[test]
fn titles_a_brf_after_its_file() {
	let document = parse(&BrfParser, "The Orbit Reader.BRF", "A\n");
	assert_eq!(document.title, "The Orbit Reader");
}

fn pef(meta: &str, volumes: &str) -> String {
	format!(
		r#"<?xml version="1.0" encoding="UTF-8"?><pef xmlns="http://www.daisy.org/ns/2008/pef" version="2008-1"><head><meta xmlns:dc="http://purl.org/dc/elements/1.1/"><dc:format>application/x-pef+xml</dc:format>{meta}</meta></head><body>{volumes}</body></pef>"#
	)
}

fn volume(pages: &[&[&str]]) -> String {
	let mut markup = String::new();
	for rows in pages {
		markup.push_str("<page>");
		for row in *rows {
			markup.push_str("<row>");
			markup.push_str(&cells(row));
			markup.push_str("</row>");
		}
		markup.push_str("</page>");
	}
	let pages = markup;
	format!(r#"<volume cols="28" rows="4" rowgap="0" duplex="false"><section>{pages}</section></volume>"#)
}

#[test]
fn reads_pef_rows_as_lines_and_pages() {
	let document = parse(
		&PefParser,
		"ncbi.pef",
		pef("", &volume(&[&[",ABOUT ,,NCBI       #A", "  ,SOME FACTS"], &["  HELLO       A#A"]])),
	);
	assert_eq!(document.buffer.content, "About NCBI\nSome facts\nhello\n");
	assert_eq!(page_markers(&document), [(0, "Page 1".to_string()), (22, "Page 2".to_string())]);
	assert!(document.toc_items.is_empty(), "a single volume needs no table of contents");
}

#[test]
fn ignores_line_breaks_inside_pef_rows() {
	let row = |text: &str| format!("<row>{}\n</row>\n", cells(text));
	let volumes = format!(
		r#"<volume cols="28" rows="4" rowgap="0" duplex="false"><section><page>{}{}</page></section></volume>"#,
		row("  HELLO      #A"),
		row("_W")
	);
	assert_eq!(parse(&PefParser, "rows.pef", pef("", &volumes)).buffer.content, "hello world\n");
}

#[test]
fn lists_pef_volumes_in_the_table_of_contents() {
	let document =
		parse(&PefParser, "two.pef", pef("", &(volume(&[&["  ONE"], &["  TWO"]]) + &volume(&[&["  ?REE"]]))));
	let toc: Vec<_> = document.toc_items.iter().map(|item| (item.name.as_str(), item.offset.get())).collect();
	assert_eq!(toc, [("Volume 1", 0), ("Volume 2", 8)]);
}

#[test]
fn takes_title_and_author_from_pef_metadata() {
	let meta = "<dc:title>The Fall of the House of Usher</dc:title><dc:creator>Edgar Allan Poe</dc:creator>";
	let document = parse(&PefParser, "usher.pef", pef(meta, &volume(&[&["A"]])));
	assert_eq!(document.title, "The Fall of the House of Usher");
	assert_eq!(document.author, "Edgar Allan Poe");
}

#[test]
fn titles_a_pef_without_one_after_its_file() {
	let document = parse(&PefParser, "about ncbi.pef", pef("", ""));
	assert_eq!(document.title, "about ncbi");
}

#[test]
fn refuses_xml_that_is_not_pef() {
	let result = parse_with(
		&PefParser,
		"other.pef",
		r#"<?xml version="1.0"?><html xmlns="http://www.w3.org/1999/xhtml"/>"#,
		|context| context,
	);
	assert!(result.is_err());
}

#[test]
fn lists_the_tables_people_choose_by_name() {
	let tables = braille_tables();
	let ueb = tables.iter().find(|table| table.file_name == DEFAULT_TABLE).expect("the default table is listed");
	assert!(ueb.display_name.contains("nified"), "{}", ueb.display_name);
	assert!(
		tables.iter().all(|table| {
			!std::path::Path::new(table.file_name)
				.extension()
				.is_some_and(|extension| extension.eq_ignore_ascii_case("cti") || extension.eq_ignore_ascii_case("uti"))
		}),
		"included pieces are not offered"
	);
	let us =
		tables.iter().find(|table| table.file_name == "en-us-g2.ctb").expect("tables without metadata are listed too");
	assert_eq!(us.display_name, "English, U.S. Grade 2 (ABAE) table");
	assert!(tables.windows(2).all(|pair| pair[0].display_name.to_lowercase() <= pair[1].display_name.to_lowercase()));
	let slovak = tables.iter().position(|table| table.file_name == "sk-g1.ctb").expect("Slovak grade 1 is listed");
	let unnamed = tables.iter().position(|table| table.file_name == "sk-sk-g1.utb").expect("unnamed tables are listed");
	assert!(unnamed.abs_diff(slovak) < 5, "a table named by its file sits among the named ones for its language");
	assert_eq!(tables[slovak].label(), "Slovak braille (sk-g1.ctb)");
	assert_eq!(tables[unnamed].label(), "sk-sk-g1.utb");
	assert_eq!(table_name(DEFAULT_TABLE), Some(DEFAULT_TABLE));
	assert_eq!(table_name("no-such-table.ctb"), None);
}

#[rstest::rstest]
#[case("en-Brai-US", "UEB grade2", Some(6), Some("en-ueb-g2.ctb"))]
#[case("en-Brai-US", "UEB grade1", Some(6), Some("en-ueb-g1.ctb"))]
#[case("en-Brai-US", "EBAE grade2", None, Some("en-us-g2.ctb"))]
#[case("de-Brai", "German grade2", Some(6), Some("de-g2.ctb"))]
// liblouis marks only the detailed German grade 0 and 1 tables as reading both ways.
#[case("de-Brai", "German grade1", None, Some("de-g1-detailed.ctb"))]
#[case("de-Brai", "German grade0", None, Some("de-g0-detailed.utb"))]
#[case("fr-Brai", "French grade2", None, Some("fr-bfu-g2.ctb"))]
#[case("fr-Brai", "French uncontracted no-grade", None, Some("fr-bfu-comp6.utb"))]
#[case("nl-Brai-NL", "Dutch grade0", Some(6), Some("nl-NL-g0.utb"))]
#[case("nl-Brai", "Dutch no-grade", None, Some("nl-NL-g0.utb"))]
#[case("xx-Brai", "Unknown grade2", None, None)]
fn finds_the_table_an_ebraille_book_names(
	#[case] language: &str,
	#[case] system: &str,
	#[case] cells: Option<u8>,
	#[case] table: Option<&str>,
) {
	assert_eq!(table_for_code(BrailleCode { language, system, cells }), table);
}
