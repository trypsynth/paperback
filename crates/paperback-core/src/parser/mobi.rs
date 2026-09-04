use std::{
	collections::{BTreeSet, HashMap},
	fs::File,
	io::Read,
	sync::LazyLock,
};

use anyhow::Result;
use encoding_rs::WINDOWS_1252;

use crate::{
	document::{Document, DocumentBuffer, ParserContext, TocItem},
	parser::{
		Parser, add_converter_markers,
		convert::html_to_text::{HtmlSourceMode, HtmlToText},
		util::{path::extract_title_from_path, toc::build_toc_from_headings},
	},
	t,
	types::HeadingInfo,
};

mod chunk;
mod compression;
mod decompress;
mod header;
mod html;
mod huffman;
mod links;
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
		let document_author = exth_author.unwrap_or_default();
		if let Some(title) = exth_title {
			header.document_title = title;
		}
		let content = decode_content(&data, &header)?;
		let text = if header.text_encoding == 65001 {
			tracing::debug!("decoding mobi content as utf-8");
			String::from_utf8_lossy(&content).into_owned()
		} else {
			tracing::debug!(text_encoding = header.text_encoding, "decoding mobi content as windows-1252");
			WINDOWS_1252.decode(&content).0.into_owned()
		};
		// Rewrite MOBI-style filepos links into standard href/id anchors before any
		// content is stripped, since filepos values are byte offsets into the raw HTML.
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
		let mut text = rewrite_internal_links(&text, &frag_offsets, &extra_targets);
		static RE_AID: LazyLock<regex::Regex> =
			LazyLock::new(|| regex::Regex::new(r#"(?i)\s[ac]id\s*=\s*["'][^"']*["']"#).unwrap());
		text = RE_AID.replace_all(&text, "").into_owned();
		// KF8 / AZW3 files concatenate the skeleton and fragments, often leaving
		// `</body></html>` inside unclosed tags at insertion points. We strip these
		// to allow `scraper` to parse the fragments cleanly.
		static RE_BODY: LazyLock<regex::Regex> = LazyLock::new(|| regex::Regex::new(r"(?is)</body>|</html>").unwrap());
		text = RE_BODY.replace_all(&text, "").into_owned();
		static RE_TITLE: LazyLock<regex::Regex> =
			LazyLock::new(|| regex::Regex::new(r"(?is)<title[^>]*>.*?</title>").unwrap());
		text = RE_TITLE.replace_all(&text, "").into_owned();
		static RE_STYLE: LazyLock<regex::Regex> =
			LazyLock::new(|| regex::Regex::new(r"(?is)<style[^>]*>.*?</style>").unwrap());
		text = RE_STYLE.replace_all(&text, "").into_owned();
		static RE_PAGE: LazyLock<regex::Regex> = LazyLock::new(|| regex::Regex::new(r"(?is)@page\s*\{[^<]+").unwrap());
		text = RE_PAGE.replace_all(&text, "").into_owned();
		// Old-style Mobipocket files use <font size="N"> instead of <h1>-<h6>.
		// Rewrite them so the heading-based TOC builder can pick them up.
		text = rewrite_font_size_headings(&text);
		let mut document_title = header.document_title;
		if document_title.trim().is_empty() {
			document_title = extract_title_from_path(&context.file_path);
		}
		let mut document = Document::new().with_author(document_author);
		document.title = document_title;
		// Converted chunk by chunk rather than as one whole-book DOM parse: `HtmlToText::convert`
		// builds a full `scraper`/`html5ever` tree for whatever it's handed, and a book-sized
		// single tree is what ran huge MOBI/AZW3 files out of memory (#781). Each chunk lands on
		// a block-element boundary (see `chunk::split_html_chunks`), so splitting costs nothing
		// beyond that boundary already being a line break in the rendered output; appending each
		// chunk's text in sequence (rather than via `DocumentBuffer::from_parts`, which inserts a
		// separator between parts) keeps a single-chunk book's output byte-identical to before
		// chunking existed.
		let mut buffer = DocumentBuffer::new();
		let mut id_positions = HashMap::new();
		let mut headings: Vec<HeadingInfo> = Vec::new();
		for piece in split_html_chunks(&text) {
			let mut html_converter = HtmlToText::with_render_tables_inline(context.render_tables_inline);
			html_converter.convert(piece, HtmlSourceMode::NativeHtml);
			let offset = buffer.current_position();
			buffer.append(&html_converter.get_text());
			for (id, &relative) in html_converter.get_id_positions() {
				id_positions.entry(id.clone()).or_insert(offset + relative);
			}
			for heading in html_converter.get_headings() {
				headings.push(HeadingInfo { offset: offset + heading.offset, ..heading.clone() });
			}
			add_converter_markers(&mut buffer, &html_converter, offset);
		}
		document.set_buffer(buffer);
		document.id_positions = id_positions;
		let mut toc_items = build_toc_from_headings(&headings);
		let toc_source = if !toc_items.is_empty() {
			"headings"
		} else if !ncx_toc.is_empty() {
			"ncx"
		} else {
			"none"
		};
		if toc_items.is_empty() && !ncx_toc.is_empty() {
			resolve_ncx_offsets(&mut ncx_toc, &document.id_positions);
			toc_items = ncx_toc;
		}
		document.toc_items = toc_items;
		tracing::debug!(
			path = %context.file_path,
			compression = header.compression,
			is_kf8 = header.is_kf8,
			text_encoding = header.text_encoding,
			num_records = header.record_offsets.len(),
			toc_source,
			"parsed mobi file"
		);
		Ok(document)
	}
}
