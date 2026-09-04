use std::{iter, ptr};

use rstest::rstest;

use super::*;
use crate::types::{FormatInfo, HeadingInfo, LinkInfo, ListInfo, ListItemInfo, SeparatorInfo, TableInfo};

struct MockConverter {
	headings: Vec<HeadingInfo>,
	links: Vec<LinkInfo>,
	images: Vec<ImageInfo>,
	figures: Vec<ImageInfo>,
	tables: Vec<TableInfo>,
	separators: Vec<SeparatorInfo>,
	lists: Vec<ListInfo>,
	list_items: Vec<ListItemInfo>,
	bolds: Vec<FormatInfo>,
	italics: Vec<FormatInfo>,
	underlines: Vec<FormatInfo>,
}

impl ConverterOutput for MockConverter {
	fn get_headings(&self) -> &[HeadingInfo] {
		&self.headings
	}

	fn get_links(&self) -> &[LinkInfo] {
		&self.links
	}

	fn get_images(&self) -> &[ImageInfo] {
		&self.images
	}

	fn get_figures(&self) -> &[ImageInfo] {
		&self.figures
	}

	fn get_tables(&self) -> &[TableInfo] {
		&self.tables
	}

	fn get_separators(&self) -> &[SeparatorInfo] {
		&self.separators
	}

	fn get_lists(&self) -> &[ListInfo] {
		&self.lists
	}

	fn get_list_items(&self) -> &[ListItemInfo] {
		&self.list_items
	}

	fn get_bolds(&self) -> &[FormatInfo] {
		&self.bolds
	}

	fn get_italics(&self) -> &[FormatInfo] {
		&self.italics
	}

	fn get_underlines(&self) -> &[FormatInfo] {
		&self.underlines
	}
}
fn sample_converter() -> MockConverter {
	MockConverter {
		headings: vec![HeadingInfo { offset: 1, level: 2, text: "Heading".to_string() }],
		links: vec![LinkInfo { offset: 2, text: "Link".to_string(), reference: "#a".to_string() }],
		images: vec![],
		figures: vec![],
		tables: vec![TableInfo { offset: 3, text: "T".to_string(), html_content: "<table/>".to_string(), length: 11 }],
		separators: vec![SeparatorInfo { offset: 4, length: 7 }],
		lists: vec![ListInfo { offset: 5, item_count: 3, length: 4 }],
		list_items: vec![ListItemInfo { offset: 6, level: 1, text: "Item".to_string() }],
		bolds: vec![],
		italics: vec![],
		underlines: vec![],
	}
}

#[test]
fn join_extensions_formats_and_skips_empty_entries() {
	let joined = join_extensions(["txt", "", "md"]);
	assert_eq!(joined, "*.txt;*.md");
}

#[test]
fn join_extensions_returns_empty_for_empty_input() {
	let joined = join_extensions(iter::empty::<&str>());
	assert_eq!(joined, "");
}

#[rstest]
#[case("http://example.com", true)]
#[case("HTTPS://example.com", true)]
#[case("MailTo:test@example.com", true)]
#[case("ftp://example.com", false)]
#[case("#local", false)]
#[case("https//example.com", false)]
#[case("mailtox:test@example.com", false)]
#[case("httpx://example.com", false)]
fn is_external_url_classifies_schemes(#[case] url: &str, #[case] expected: bool) {
	assert_eq!(is_external_url(url), expected);
}

#[rstest]
#[case("txt", true)]
#[case(".TXT", true)]
#[case("log", true)]
#[case("m4b", true)]
#[case(".M4B", true)]
#[case("m4a", false)]
#[case("", false)]
#[case(".", false)]
#[case("notarealextension", false)]
#[case(" txt", false)]
#[case("txt ", false)]
#[case("..txt", true)]
#[case("...log", true)]
fn parser_supports_extension_classifies_inputs(#[case] extension: &str, #[case] expected: bool) {
	assert_eq!(parser_supports_extension(extension), expected);
}

#[test]
fn file_filter_string_contains_supported_and_fallback_groups() {
	let filter = build_file_filter_string();
	assert!(filter.contains("All Supported Files ("));
	assert!(filter.contains("*.txt"));
	assert!(filter.contains("*.epub"));
	#[cfg(not(target_os = "macos"))]
	assert!(filter.ends_with("All Files (*.*)|*.*"));
}

#[test]
fn add_converter_markers_transfers_all_marker_types_with_offset() {
	let converter = sample_converter();
	let mut buffer = DocumentBuffer::new();
	add_converter_markers(&mut buffer, &converter, 100);
	assert_eq!(buffer.markers.len(), 6);
	assert_eq!(buffer.markers[0].mtype, MarkerType::Heading2);
	assert_eq!(buffer.markers[0].position, 101);
	assert_eq!(buffer.markers[0].text, "Heading");
	assert_eq!(buffer.markers[1].mtype, MarkerType::Link);
	assert_eq!(buffer.markers[1].position, 102);
	assert_eq!(buffer.markers[1].reference, "#a");
	assert_eq!(buffer.markers[2].mtype, MarkerType::Table);
	assert_eq!(buffer.markers[2].length, 11);
	assert_eq!(buffer.markers[3].mtype, MarkerType::Separator);
	assert_eq!(buffer.markers[3].length, 7);
	assert_eq!(buffer.markers[4].mtype, MarkerType::List);
	assert_eq!(buffer.markers[4].level, 3);
	assert_eq!(buffer.markers[5].mtype, MarkerType::ListItem);
	assert_eq!(buffer.markers[5].level, 1);
}

#[test]
fn add_converter_markers_excluding_links_skips_link_markers() {
	let converter = sample_converter();
	let mut buffer = DocumentBuffer::new();
	add_converter_markers_excluding_links(&mut buffer, &converter, 10);
	assert_eq!(buffer.markers.len(), 5);
	assert!(buffer.markers.iter().all(|marker| marker.mtype != MarkerType::Link));
}

#[test]
fn add_converter_markers_handles_empty_converter_output() {
	let converter = MockConverter {
		headings: vec![],
		links: vec![],
		images: vec![],
		figures: vec![],
		tables: vec![],
		separators: vec![],
		lists: vec![],
		list_items: vec![],
		bolds: vec![],
		italics: vec![],
		underlines: vec![],
	};
	let mut buffer = DocumentBuffer::new();
	add_converter_markers(&mut buffer, &converter, 0);
	assert!(buffer.markers.is_empty());
}

#[test]
fn resolve_extension_routes_loose_ncc_html_to_opf() {
	let book_directory = Path::new("books").join("daisy2");
	assert_eq!(resolve_extension(&book_directory.join("ncc.html")), Some("opf"));
	assert_eq!(resolve_extension(&book_directory.join("NCC.HTML")), Some("opf"));
}

#[test]
fn resolve_extension_leaves_other_html_files_alone() {
	let chapter = Path::new("books").join("chapter1.html");
	assert_eq!(resolve_extension(&chapter), Some("html"));
}

#[test]
fn parse_document_errors_when_missing_extension() {
	let context = ParserContext::new("no_extension".to_string());
	let err = parse_document(&context).expect_err("expected missing extension error");
	assert!(err.to_string().contains("No file extension found"));
}

#[test]
fn parse_document_errors_for_unknown_forced_extension() {
	let context = ParserContext::new("anything".to_string()).with_forced_extension("unknown_ext".to_string());
	let err = parse_document(&context).expect_err("expected unknown parser error");
	assert!(err.to_string().contains("No parser found for extension"));
}

#[test]
fn get_parser_flags_for_context_returns_none_for_unknown_extension() {
	let context = ParserContext::new("doc.unknown_ext".to_string());
	assert_eq!(get_parser_flags_for_context(&context), ParserFlags::NONE);
}

#[test]
fn m4b_parser_advertises_audio_book_navigation() {
	let context = ParserContext::new("book.m4b".to_string());
	assert_eq!(
		get_parser_flags_for_context(&context),
		ParserFlags::SUPPORTS_SECTIONS | ParserFlags::SUPPORTS_TOC | ParserFlags::SUPPORTS_AUDIO
	);
}

/// Guards against a format being declared in `paperback-formats` with no parser wired up
/// in `parser_registry!` (it would show up in the installer and open dialog but fail to
/// open), and against a parser being registered twice.
#[test]
fn every_declared_format_has_exactly_one_parser() {
	let registered: Vec<&'static FormatMeta> =
		ParserRegistry::global().all_parsers().iter().map(RegisteredParser::format).collect();
	for format in paperback_formats::ALL {
		let count = registered.iter().filter(|candidate| ptr::eq(**candidate, *format)).count();
		assert_eq!(count, 1, "{} must have exactly one registered parser, found {count}", format.name);
	}
	assert_eq!(registered.len(), paperback_formats::ALL.len(), "a parser was registered for an unlisted format");
}

/// Registration order decides which parser wins a shared extension, so it must track the
/// order of the format table rather than drifting from it.
#[test]
fn registration_order_matches_the_format_table() {
	let registered: Vec<&str> = ParserRegistry::global().all_parsers().iter().map(RegisteredParser::name).collect();
	let declared: Vec<&str> = paperback_formats::ALL.iter().map(|format| format.name).collect();
	assert_eq!(registered, declared);
}

/// `.zip` is claimed by both DAISY and Word, and `parse_document` tries claimants in
/// registration order, so DAISY must stay ahead of Word.
#[test]
fn zip_is_offered_to_daisy_before_word() {
	let names: Vec<_> = ParserRegistry::global().get_parsers_for_extension("zip").iter().map(|p| p.name()).collect();
	assert_eq!(names, vec![paperback_formats::DAISY.name, paperback_formats::WORD.name]);
}

/// The registry stores parsers in a `Vec`, so the file dialog's filter groups appear in a
/// stable order rather than whatever order a hash map happened to yield.
#[test]
fn file_filter_lists_groups_in_registration_order() {
	let filter = build_file_filter_string();
	// Groups are "Label (*.ext;…)|*.ext;…" pairs, so every other field is a label.
	let labels: Vec<&str> =
		filter.split('|').step_by(2).filter_map(|group| group.split_once(" (").map(|(label, _)| label)).collect();
	let expected: Vec<&str> = ParserRegistry::global().all_parsers().iter().map(RegisteredParser::name).collect();
	assert_eq!(labels.first().copied(), Some("All Supported Files"));
	assert_eq!(&labels[1..=expected.len()], expected.as_slice());
}

#[test]
fn file_filter_string_contains_text_files_group_name() {
	let filter = build_file_filter_string();
	assert!(filter.contains("Text Files ("));
}
/// `add_tables_separators_lists` sets the Table marker's `length` to `table.length`
/// (display units) and offsets it by the base offset.
#[test]
fn add_tables_separators_lists_sets_table_marker_length() {
	let converter = MockConverter {
		headings: vec![],
		links: vec![],
		images: vec![],
		figures: vec![],
		tables: vec![TableInfo {
			offset: 10,
			text: "T".to_string(),
			html_content: "<table/>".to_string(),
			length: 7, // display-unit field, must appear as marker length
		}],
		separators: vec![],
		lists: vec![],
		list_items: vec![],
		bolds: vec![],
		italics: vec![],
		underlines: vec![],
	};
	let mut buffer = DocumentBuffer::new();
	let base_offset = 100usize;
	add_converter_markers(&mut buffer, &converter, base_offset);
	let table_marker = buffer.markers.iter().find(|m| m.mtype == MarkerType::Table).expect("Table marker");
	assert_eq!(table_marker.position, base_offset + 10, "position = offset + table.offset");
	assert_eq!(table_marker.length, 7, "marker length must equal table length, not byte length");
	assert_eq!(table_marker.reference, "<table/>", "marker reference must be the table HTML");
}
