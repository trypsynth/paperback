use std::{
	collections::{BTreeSet, HashMap},
	fs::File,
	io::Read,
	sync::LazyLock,
};

use anyhow::Result;
use encoding_rs::WINDOWS_1252;
use rayon::prelude::*;

use crate::{
	document::{Document, DocumentBuffer, Marker, MarkerType, ParserContext, TocItem},
	parser::{
		Parser, add_converter_markers, add_toc_heading_markers,
		convert::html_to_text::{HtmlSourceMode, HtmlToText},
		text::text_document,
		util::{path::extract_title_from_path, toc::build_toc_from_headings},
	},
	t,
	types::HeadingInfo,
	util::encoding::decode_guessing_code_page,
};

mod chunk;
mod compression;
mod decompress;
mod header;
mod html;
mod huffman;
mod index;
mod kf8;
mod links;
pub mod pictures;
mod toc;
mod varint;

use chunk::split_html_chunks;
use decompress::decode_content;
use header::{parse_exth, parse_header};
use html::rewrite_font_size_headings;
use links::{build_fragment_offsets, resolve_ncx_offsets, rewrite_internal_links};
use toc::parse_ncx;

pub struct MobiParser;

impl Parser for MobiParser {
	fn parse(&self, context: &ParserContext) -> Result<Document> {
		tracing::debug!(path = %context.file_path, "parsing mobi file");
		let mut file = File::open(&context.file_path)?;
		let mut data = Vec::new();
		file.read_to_end(&mut data)?;
		if data.len() < 78 {
			tracing::warn!(len = data.len(), "mobi file too short to contain a valid header");
			// TRANSLATORS: Error shown when a MOBI file is too small to contain a valid header
			anyhow::bail!(t("File too short"));
		}
		let title_bytes = data[0..32].to_vec();
		let mut header = parse_header(&data, &title_bytes)?;
		let (exth_map, exth_author, exth_title) = parse_exth(&header);
		let mut document_author = exth_author.unwrap_or_default();
		if let Some(title) = exth_title {
			header.document_title = title;
		}
		// Each book-sized stage is freed as soon as the next is made, since the function's end is too late: by then every copy has sat through the conversion.
		let content = decode_content(&data, &header)?;
		// Early Mobipocket books are PalmDOC files holding HTML, and go on to be read as any other; the rest of PalmDOC is plain text.
		if header.is_palmdoc && !looks_like_html(&content) {
			let mut document = text_document(&context.file_path, decode_guessing_code_page(content));
			if !header.document_title.is_empty() {
				document.title = header.document_title;
			}
			tracing::debug!(path = %context.file_path, compression = header.compression, "parsed plain palmdoc file");
			return Ok(document);
		}
		let raw = {
			// PalmDOC names no encoding, and Russian books in Windows-1251 came out garbled when every one was taken as Western.
			if header.is_palmdoc {
				decode_guessing_code_page(content)
			} else if header.text_encoding == 65001 {
				tracing::debug!("decoding mobi content as utf-8");
				String::from_utf8(content)
					.unwrap_or_else(|error| String::from_utf8_lossy(error.as_bytes()).into_owned())
			} else {
				tracing::debug!(text_encoding = header.text_encoding, "decoding mobi content as windows-1252");
				WINDOWS_1252.decode(&content).0.into_owned()
			}
		};
		// A KF8 book was an EPUB before it was compiled, and its skeleton index still records where each file began.
		let section_starts = if header.is_kf8 {
			kf8::section_starts(&data, &header.record_offsets, header.mobi_header()).unwrap_or_default()
		} else {
			Vec::new()
		};
		// filepos values are byte offsets into the raw HTML, so links are rewritten before anything is stripped.
		let frag_offsets = build_fragment_offsets(&data, &header.record_offsets, header.mobi_header());
		let mut ncx_toc =
			parse_ncx(&data, &header.record_offsets, header.mobi_header(), &exth_map, header.is_kf8, &frag_offsets);
		fn extract_targets(items: &[TocItem], targets: &mut BTreeSet<usize>) {
			let mut stack = vec![items];
			while let Some(current_items) = stack.pop() {
				for item in current_items {
					if let Some(pos_str) = item.reference.strip_prefix("#fp")
						&& let Ok(pos) = pos_str.parse::<usize>()
					{
						targets.insert(pos);
					}
					if !item.children.is_empty() {
						stack.push(&item.children);
					}
				}
			}
		}
		let mut extra_targets = BTreeSet::new();
		extract_targets(&ncx_toc, &mut extra_targets);
		// Anchoring section starts as link targets is the only way to learn where a raw-text position ends up after rewriting and cleaning move everything.
		extra_targets.extend(section_starts.iter().copied());
		let mut text = rewrite_internal_links(&raw, &frag_offsets, &extra_targets);
		if header.is_palmdoc {
			// These books describe themselves in a Dublin Core block before the text, which would otherwise be read out as the book's first line.
			let (stripped, creator) = take_palmdoc_metadata(&text);
			text = stripped;
			if document_author.is_empty() {
				document_author = creator.unwrap_or_default();
			}
		}
		drop(raw);
		// One pass rather than five, since each was a full copy of a string reaching a hundred megabytes; a title or style element beats an `aid`/`cid` inside it as the leftmost match.
		// KF8 / AZW3 files often leave `</body></html>` inside unclosed tags at fragment insertion points, which `scraper` cannot parse around.
		static RE_STRIP: LazyLock<regex::Regex> = LazyLock::new(|| {
			regex::Regex::new(
				r#"(?is)<title[^>]*>.*?</title>|<style[^>]*>.*?</style>|</body>|</html>|@page\s*\{[^<]+|\s[ac]id\s*=\s*["'][^"']*["']"#,
			)
			.unwrap()
		});
		text = RE_STRIP.replace_all(&text, "").into_owned();
		// Old-style Mobipocket files use <font size="N"> instead of <h1>-<h6>.
		text = rewrite_font_size_headings(&text);
		text = pictures::point_pictures_at_records(&text, &data, &header.record_offsets, header.mobi_header());
		drop(data);
		let mut document_title = header.document_title;
		if document_title.trim().is_empty() {
			document_title = extract_title_from_path(&context.file_path);
		}
		let mut document = Document::new().with_author(document_author);
		document.title = document_title;
		// Chunked because one book-sized DOM ran huge MOBI/AZW3 files out of memory (#781); chunks end on block boundaries, and appending (not `DocumentBuffer::from_parts`, which adds separators) keeps output unchanged.
		let mut buffer = DocumentBuffer::new();
		let mut id_positions = HashMap::new();
		let mut headings: Vec<HeadingInfo> = Vec::new();
		let converted: Vec<HtmlToText> = split_html_chunks(&text)
			.par_iter()
			.map(|piece| {
				let mut html_converter = HtmlToText::with_render_tables_inline(context.render_tables_inline);
				html_converter.convert(piece, HtmlSourceMode::NativeHtml);
				html_converter
			})
			.collect();
		drop(text);
		for html_converter in converted {
			let offset = buffer.end();
			buffer.append(html_converter.get_text());
			for (id, &relative) in html_converter.get_id_positions() {
				id_positions.entry(id.clone()).or_insert_with(|| offset + relative.from_start());
			}
			for heading in html_converter.get_headings() {
				headings.push(HeadingInfo { offset: offset + heading.offset.from_start(), ..heading.clone() });
			}
			add_converter_markers(&mut buffer, &html_converter, offset);
		}
		for (index, start) in section_starts.iter().enumerate() {
			let Some(&position) = id_positions.get(&format!("fp{start:010}")) else { continue };
			buffer.add_marker(
				Marker::new(MarkerType::SectionBreak, position).with_text(format!("Section {}", index + 1)),
			);
		}
		document.id_positions = id_positions;
		let ncx_item_count = ncx_toc.len();
		let mut toc_items = build_toc_from_headings(&headings);
		// A novel marking up only its title gives one heading while its index lists all forty chapters; a single-entry index is a stub and never wins.
		let use_ncx = ncx_item_count > 1 && ncx_item_count > toc_items.len();
		let toc_source = if use_ncx {
			"ncx"
		} else if toc_items.is_empty() {
			if ncx_toc.is_empty() { "none" } else { "ncx" }
		} else {
			"headings"
		};
		if use_ncx || toc_items.is_empty() {
			resolve_ncx_offsets(&mut ncx_toc, &document.id_positions);
			if !ncx_toc.is_empty() {
				toc_items = ncx_toc;
			}
		}
		add_toc_heading_markers(&mut buffer, &toc_items);
		document.set_buffer(buffer);
		document.toc_items = toc_items;
		tracing::debug!(
			path = %context.file_path,
			compression = header.compression,
			is_kf8 = header.is_kf8,
			text_encoding = header.text_encoding,
			num_records = header.record_offsets.len(),
			sections = section_starts.len(),
			heading_count = headings.len(),
			ncx_count = ncx_item_count,
			toc_source,
			"parsed mobi file"
		);
		Ok(document)
	}
}

/// The `<metadata>` block and its `dc:Creator`, which stands in for the EXTH author these books lack.
fn take_palmdoc_metadata(html: &str) -> (String, Option<String>) {
	static METADATA: LazyLock<regex::Regex> =
		LazyLock::new(|| regex::Regex::new(r"(?is)<metadata\b.*?</metadata\s*>").expect("metadata pattern is valid"));
	static CREATOR: LazyLock<regex::Regex> = LazyLock::new(|| {
		regex::Regex::new(r"(?is)<dc:creator\b[^>]*>(.*?)</dc:creator\s*>").expect("creator pattern is valid")
	});
	let Some(block) = METADATA.find(html) else { return (html.to_string(), None) };
	let creator = CREATOR
		.captures(block.as_str())
		.map(|captures| captures[1].trim().to_string())
		.filter(|creator| !creator.is_empty());
	(format!("{}{}", &html[..block.start()], &html[block.end()..]), creator)
}

/// Only the start is checked: a Mobipocket book opens with its `<html>`, and plain text that happens to hold a tag further in stays text.
fn looks_like_html(content: &[u8]) -> bool {
	let start = String::from_utf8_lossy(&content[..content.len().min(2048)]).to_ascii_lowercase();
	start.trim_start().starts_with('<') && (start.contains("<html") || start.contains("<body"))
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::util::test_support::TempDir;

	/// A Palm database holding `texts` as uncompressed PalmDOC records.
	fn palmdoc(name: &str, texts: &[&str]) -> Vec<u8> {
		let records = texts.len() + 1;
		let mut header = vec![0u8; 78];
		header[..name.len()].copy_from_slice(name.as_bytes());
		header[60..68].copy_from_slice(b"TEXtREAd");
		header[76..78].copy_from_slice(&u16::try_from(records).unwrap().to_be_bytes());
		let mut record0 = Vec::new();
		record0.extend_from_slice(&1u16.to_be_bytes());
		record0.extend_from_slice(&[0, 0]);
		record0.extend_from_slice(&u32::try_from(texts.iter().map(|t| t.len()).sum::<usize>()).unwrap().to_be_bytes());
		record0.extend_from_slice(&u16::try_from(texts.len()).unwrap().to_be_bytes());
		record0.extend_from_slice(&4096u16.to_be_bytes());
		record0.extend_from_slice(&[0; 4]);
		let bodies: Vec<&[u8]> =
			std::iter::once(record0.as_slice()).chain(texts.iter().map(|t| t.as_bytes())).collect();
		let mut offset = 78 + records * 8;
		let mut list = Vec::new();
		for body in &bodies {
			list.extend_from_slice(&u32::try_from(offset).unwrap().to_be_bytes());
			list.extend_from_slice(&[0; 4]);
			offset += body.len();
		}
		let mut out = header;
		out.extend_from_slice(&list);
		for body in bodies {
			out.extend_from_slice(body);
		}
		out
	}

	#[test]
	fn a_palmdoc_book_holding_html_is_read_as_html() {
		let dir = TempDir::new("palmdoc-html");
		let path = dir.write_str(
			"book.mobi",
			palmdoc(
				"Html_Book",
				&["<HTML><HEAD><metadata><dc-metadata><dc:Title>Html Book</dc:Title><dc:Creator>Jane Writer</dc:Creator></dc-metadata></metadata></HEAD><BODY><h1>Chapter</h1><p>One &amp; two</p></BODY></HTML>"],
			),
		);
		let document = MobiParser.parse(&ParserContext::new(path)).expect("a PalmDOC book holding HTML");
		assert!(document.buffer.content.contains("One & two"), "{}", document.buffer.content);
		assert!(!document.buffer.content.contains("<p>"), "{}", document.buffer.content);
		assert_eq!(document.toc_items.first().map(|item| item.name.as_str()), Some("Chapter"));
		assert!(
			document.buffer.content.trim_start().starts_with("Chapter"),
			"metadata read as text: {}",
			document.buffer.content
		);
		assert_eq!(document.author, "Jane Writer");
	}

	#[test]
	fn a_palmdoc_book_reads_as_its_plain_text() {
		let dir = TempDir::new("palmdoc");
		let path = dir
			.write_str("book.mobi", palmdoc("My_Palm_Book", &["First line\r\nstill ", "the first <b>paragraph</b>\n"]));
		let document = MobiParser.parse(&ParserContext::new(path)).expect("a PalmDOC book");
		assert_eq!(
			document.buffer.content, "First line\nstill the first <b>paragraph</b>\n",
			"kept as text, not read as HTML"
		);
		assert_eq!(document.title, "My Palm Book");
	}
}
