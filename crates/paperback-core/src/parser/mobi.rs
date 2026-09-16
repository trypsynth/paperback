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
mod index;
mod kf8;
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
		// A KF8 book was an EPUB before it was compiled, and its skeleton index still records
		// where each of those files began. Those are the book's sections.
		let section_starts = if header.is_kf8 {
			kf8::section_starts(&data, &header.record_offsets, header.mobi_header()).unwrap_or_default()
		} else {
			Vec::new()
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
		// Section starts ride along as link targets, which gets each one an anchor in the
		// HTML and so an entry in `id_positions` once the text is converted. That is the only
		// way to learn where a raw-text position ended up, since rewriting and cleaning move
		// everything around before the converter ever sees it.
		extra_targets.extend(section_starts.iter().copied());
		let mut text = rewrite_internal_links(&text, &frag_offsets, &extra_targets);
		// Everything the converter has no use for comes out in one pass rather than five: each one
		// was a full scan and a full copy of a string that runs to a hundred megabytes on the
		// largest books. A title or style element wins over an `aid`/`cid` attribute inside it,
		// since the element starts first and the leftmost match is the one taken.
		//
		// KF8 / AZW3 files concatenate the skeleton and fragments, often leaving `</body></html>`
		// inside unclosed tags at insertion points, which `scraper` cannot parse the fragments
		// around, so those go too.
		static RE_STRIP: LazyLock<regex::Regex> = LazyLock::new(|| {
			regex::Regex::new(
				r#"(?is)<title[^>]*>.*?</title>|<style[^>]*>.*?</style>|</body>|</html>|@page\s*\{[^<]+|\s[ac]id\s*=\s*["'][^"']*["']"#,
			)
			.unwrap()
		});
		text = RE_STRIP.replace_all(&text, "").into_owned();
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
		// The chunks are independent by construction, so they convert on every core at once. Only
		// the stitching below has to stay in order, and it is a fraction of the work.
		let converted: Vec<HtmlToText> = split_html_chunks(&text)
			.par_iter()
			.map(|piece| {
				let mut html_converter = HtmlToText::with_render_tables_inline(context.render_tables_inline);
				html_converter.convert(piece, HtmlSourceMode::NativeHtml);
				html_converter
			})
			.collect();
		for html_converter in converted {
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
		for (index, start) in section_starts.iter().enumerate() {
			let Some(&position) = id_positions.get(&format!("fp{start:010}")) else { continue };
			buffer.add_marker(
				Marker::new(MarkerType::SectionBreak, position).with_text(format!("Section {}", index + 1)),
			);
		}
		document.id_positions = id_positions;
		let ncx_item_count = ncx_toc.len();
		let mut toc_items = build_toc_from_headings(&headings);
		// Whichever of the two names more of the book. A novel whose text marks up its title and
		// nothing else gives a one entry table of contents while its index quietly lists all forty
		// chapters, and taking the longer list is what puts those chapters back. A single entry
		// index is a stub rather than a table of contents and never wins.
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
		// The book may name every chapter in its index and mark up none of them in its text, which
		// leaves the reader a table of contents and nothing to move between by heading. Asked one
		// entry at a time so that a chapter which did write its own heading is not announced twice.
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
