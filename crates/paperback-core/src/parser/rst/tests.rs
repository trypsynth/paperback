use super::*;
use crate::{document::DocumentHandle, reader_core::resolve_link, util::test_support::TempDir};

fn parse_bytes(name: &str, contents: impl AsRef<[u8]>) -> Document {
	let dir = TempDir::new("rst-parser");
	let path = dir.write_str(name, contents);
	RstParser.parse(&ParserContext::new(path)).expect("parse reStructuredText document")
}

#[test]
fn extracts_headings_and_paragraph_text() {
	let doc = parse_bytes(
		"book.rst",
		"Chapter One\n===========\n\nFirst paragraph.\n\nSection\n-------\n\nSecond paragraph.\n",
	);
	assert!(doc.buffer.content.contains("Chapter One"), "got: {:?}", doc.buffer.content);
	assert!(doc.buffer.content.contains("First paragraph."));
	assert!(doc.buffer.content.contains("Second paragraph."));
	assert_eq!(doc.toc_items.len(), 1, "top-level heading becomes a single TOC root: {:?}", doc.toc_items);
	assert_eq!(doc.toc_items[0].name, "Chapter One");
	assert_eq!(doc.toc_items[0].children.len(), 1);
	assert_eq!(doc.toc_items[0].children[0].name, "Section");
}

#[test]
fn cross_reference_to_a_section_resolves_as_an_internal_link() {
	let doc = parse_bytes("book.rst", "See Details_ below.\n\nDetails\n=======\n\nBody text here.\n");
	let handle = DocumentHandle::new(doc);
	let heading_offset = handle.document().buffer.content.find("Details\n").expect("heading text present");
	let nav = resolve_link(&handle, "#details", DocPos::ZERO);
	assert!(nav.found, "reference to the section must resolve");
	assert!(!nav.is_external, "an in-book cross-reference must not be treated as external");
	assert_eq!(nav.offset, heading_offset);
}

#[test]
fn embedded_external_link_resolves_as_external() {
	let doc = parse_bytes("book.rst", "See `the docs <https://example.com/>`_ for details.\n");
	let handle = DocumentHandle::new(doc);
	let nav = resolve_link(&handle, "https://example.com/", DocPos::ZERO);
	assert!(nav.found);
	assert!(nav.is_external);
	assert_eq!(nav.url, "https://example.com/");
}

#[test]
fn reports_the_path_when_the_file_is_missing() {
	let dir = TempDir::new("rst-parser");
	let missing = dir.join_str("nope.rst");
	let err = RstParser.parse(&ParserContext::new(missing.clone())).expect_err("missing file must fail");
	assert!(err.to_string().contains(&missing), "error should name the file: {err}");
}

#[test]
fn decodes_non_utf8_content() {
	let mut bytes = vec![0xFF, 0xFE];
	for unit in "héllo\n=====\n".encode_utf16() {
		bytes.extend_from_slice(&unit.to_le_bytes());
	}
	let doc = parse_bytes("book.rst", bytes);
	assert!(doc.buffer.content.contains("héllo"), "got: {:?}", doc.buffer.content);
}

/// See the module doc for why a panic matters more here than for most formats.
#[test]
fn never_panics_on_malformed_input() {
	let samples = [
		"",
		"   \n\n\t\n",
		"**unterminated bold\n",
		"``unterminated literal\n",
		"`unterminated ref\n",
		":role:`unterminated\n",
		".. \n",
		".. directive::\n",
		".. _label:\n",
		"===\n===\n===\n",
		"- \n- \n",
		"1.\n",
		":\n",
		"|\n",
		"[",
		"\\",
		&"=".repeat(5000),
		"Title\n=====\n\n   deeply\n      nested\n         quotes\n            forever\n",
		"**bold with ``literal`` nested inside it**\n",
		"1. one\n   - nested bullet\n     continued\n2. two\n",
	];
	for sample in samples {
		let dir = TempDir::new("rst-parser-fuzz");
		let path = dir.write_str("book.rst", sample);
		let result = RstParser.parse(&ParserContext::new(path));
		assert!(result.is_ok(), "sample must parse without error: {sample:?}");
	}
}

#[test]
fn an_opening_title_and_author_field_name_the_document() {
	let doc = parse_bytes(
		"notes.rst",
		"Field Notes
===========

:Author: Ann Smith
:Date: 2026

Text.
",
	);
	assert_eq!(doc.title, "Field Notes");
	assert_eq!(doc.author, "Ann Smith");
	let untitled = parse_bytes(
		"notes.rst",
		"Just text.

Section
=======
",
	);
	assert_eq!(untitled.title, "notes");
	assert_eq!(untitled.author, "");
}
