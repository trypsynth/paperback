use std::{
	collections::HashSet,
	fs::{self, File},
	io::Read,
	path::Path,
};

use anyhow::{Context, Result};

use crate::{
	document::{DocPos, Document, DocumentBuffer, ParserContext},
	parser::{
		Parser, add_converter_markers,
		convert::{
			html_to_text::{HtmlSourceMode, HtmlToText},
			image_text,
		},
		is_data_uri, is_external_url,
		util::{path::extract_title_from_path, toc::build_toc_from_headings},
	},
	t,
	util::{encoding::decode_html, text::url_decode},
};

/// More than enough of a picture for its header to give its size, however much metadata comes first.
const PICTURE_HEADER_BYTES: u64 = 1 << 20;

pub struct HtmlParser;

impl Parser for HtmlParser {
	fn parse(&self, context: &ParserContext) -> Result<Document> {
		tracing::debug!(path = %context.file_path, "parsing html file");
		let bytes = fs::read(&context.file_path)
			.with_context(|| format!("Failed to open HTML file '{}'", context.file_path))?;
		if bytes.is_empty() {
			tracing::warn!(path = %context.file_path, "html file is empty");
			// TRANSLATORS: Error shown when an HTML file has no content; {} is the file path
			anyhow::bail!(t("HTML file is empty: {}").replace("{}", &context.file_path));
		}
		let html_content = decode_html(&bytes, None);
		let mut converter = HtmlToText::with_render_tables_inline(context.render_tables_inline)
			.with_content_images(content_images(&html_content, &context.file_path));
		converter.convert(&html_content, HtmlSourceMode::NativeHtml);
		let extracted_title = converter.get_title();
		let title = if extracted_title.is_empty() {
			tracing::debug!(path = %context.file_path, "html file has no title element, falling back to file name");
			extract_title_from_path(&context.file_path)
		} else {
			extracted_title.to_string()
		};
		let text = converter.take_text();
		let mut buffer = DocumentBuffer::with_content(text);
		let id_positions = converter.get_id_positions().clone();
		add_converter_markers(&mut buffer, &converter, DocPos::ZERO);
		let toc_items = build_toc_from_headings(converter.get_headings());
		let mut doc = Document::new().with_title(title);
		doc.set_buffer(buffer);
		doc.toc_items = toc_items;
		doc.id_positions = id_positions;
		tracing::debug!(path = %context.file_path, chars = doc.buffer.content.chars().count(), "parsed html file");
		Ok(doc)
	}
}

/// Pages give `alt=""` to pictures that are content as freely as ebooks do, so the page's pictures with one are measured beside the file, and those big enough to be content and placed only once, unlike a repeated ornament, are shown. A picture on the web isn't fetched for it.
fn content_images(html: &str, file_path: &str) -> HashSet<String> {
	let Some(directory) = Path::new(file_path).parent() else {
		return HashSet::new();
	};
	let sources = image_text::empty_alt_sources(html);
	sources
		.iter()
		.filter(|source| sources.iter().filter(|other| other == source).count() == 1)
		.filter(|source| !is_external_url(source) && !is_data_uri(source))
		.filter(|source| {
			let relative = url_decode(source.split(['?', '#']).next().unwrap_or_default());
			let mut header = Vec::new();
			File::open(directory.join(relative))
				.and_then(|file| file.take(PICTURE_HEADER_BYTES).read_to_end(&mut header))
				.is_ok_and(|_| image_text::is_content_sized(&header))
		})
		.cloned()
		.collect()
}

#[cfg(test)]
mod tests {
	use std::io::Cursor;

	use super::*;
	use crate::{document::MarkerType, util::test_support::TempDir};

	fn png(width: u32, height: u32) -> Vec<u8> {
		let mut bytes = Vec::new();
		image::DynamicImage::new_luma8(width, height)
			.write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Png)
			.expect("encode a png");
		bytes
	}

	/// A page gives every picture an empty `alt`: a photo worth reading, a divider that isn't, and an ornament set twice.
	#[test]
	fn shows_pictures_with_an_empty_alt_that_are_content() {
		let dir = TempDir::new("html-parser");
		dir.write("photo.png", png(640, 480));
		dir.write("rule.png", png(600, 8));
		dir.write("flourish.png", png(400, 300));
		let path = dir.write_str(
			"page.html",
			r#"<html><body><p>Before</p><img alt="" src="photo.png"><img alt="" src="rule.png"><img alt="" src="flourish.png"><p>Between</p><img alt="" src="flourish.png"><img alt="" src="missing.png"></body></html>"#,
		);
		let doc = HtmlParser.parse(&ParserContext::new(path)).expect("parse html document");
		assert_eq!(doc.buffer.content.matches("[Image]").count(), 1, "{:?}", doc.buffer.content);
		let images: Vec<_> = doc
			.buffer
			.markers
			.iter()
			.filter(|marker| marker.mtype == MarkerType::Image)
			.map(|marker| marker.reference.as_str())
			.collect();
		assert_eq!(images, ["photo.png"]);
	}

	fn parse_html(name: &str, contents: &str) -> Result<Document> {
		let dir = TempDir::new("html-parser");
		let path = dir.write_str(name, contents);
		HtmlParser.parse(&ParserContext::new(path))
	}

	fn parse_ok(contents: &str) -> Document {
		parse_html("page.html", contents).expect("parse html document")
	}

	#[test]
	fn extracts_visible_text_without_markup() {
		let doc = parse_ok("<html><body><p>Hello there</p><p>Second paragraph</p></body></html>");
		assert!(doc.buffer.content.contains("Hello there"), "text: {:?}", doc.buffer.content);
		assert!(doc.buffer.content.contains("Second paragraph"));
		assert!(!doc.buffer.content.contains('<'), "markup leaked into text: {:?}", doc.buffer.content);
	}

	#[test]
	fn prefers_the_title_element_over_the_file_name() {
		let doc = parse_ok("<html><head><title>Document Title</title></head><body><p>x</p></body></html>");
		assert_eq!(doc.title, "Document Title");
	}

	#[test]
	fn falls_back_to_the_file_name_when_there_is_no_title() {
		let doc = parse_html("Fallback Name.html", "<html><body><p>x</p></body></html>").expect("parse");
		assert_eq!(doc.title, "Fallback Name");
	}

	#[test]
	fn records_headings_as_markers_and_toc_entries() {
		let doc = parse_ok("<html><body><h1>Chapter One</h1><p>body</p><h2>Section</h2></body></html>");
		let headings: Vec<_> = doc
			.buffer
			.markers
			.iter()
			.filter(|marker| matches!(marker.mtype, MarkerType::Heading1 | MarkerType::Heading2))
			.map(|marker| marker.text.as_str())
			.collect();
		assert_eq!(headings, vec!["Chapter One", "Section"]);
		// The TOC is a tree, so the h2 hangs off the h1 rather than sitting beside it.
		assert_eq!(doc.toc_items.len(), 1);
		assert_eq!(doc.toc_items[0].name, "Chapter One");
		assert_eq!(doc.toc_items[0].children.len(), 1);
		assert_eq!(doc.toc_items[0].children[0].name, "Section");
	}

	#[test]
	fn reads_a_page_in_the_charset_it_declares() {
		let (bytes, _, _) = encoding_rs::WINDOWS_1251.encode("<html><head><meta http-equiv=\"Content-Type\" content=\"text/html; charset=windows-1251\"><title>Москва</title></head><body><p>Привет</p></body></html>");
		let dir = TempDir::new("html-parser");
		let doc =
			HtmlParser.parse(&ParserContext::new(dir.write_str("page.html", bytes))).expect("parse html document");
		assert_eq!(doc.title, "Москва");
		assert!(doc.buffer.content.contains("Привет"), "text: {:?}", doc.buffer.content);
	}

	#[test]
	fn records_links_with_their_targets() {
		let doc = parse_ok(r#"<html><body><p><a href="https://example.com">click me</a></p></body></html>"#);
		let link = doc.buffer.markers.iter().find(|marker| marker.mtype == MarkerType::Link).expect("link marker");
		assert_eq!(link.text, "click me");
		assert_eq!(link.reference, "https://example.com");
	}

	#[test]
	fn rejects_an_empty_file() {
		let err = parse_html("page.html", "").expect_err("empty html must fail");
		assert!(err.to_string().contains("empty"), "unexpected error: {err}");
	}
}
