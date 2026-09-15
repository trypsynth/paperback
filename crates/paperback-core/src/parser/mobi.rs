use std::{
	collections::{BTreeSet, HashMap},
	fs::File,
	io::Read,
	sync::LazyLock,
};

use anyhow::Result;
use encoding_rs::WINDOWS_1252;

use crate::{
	document::{Document, DocumentBuffer, Marker, MarkerType, ParserContext, TocItem, is_heading_marker},
	parser::{
		Parser, add_converter_markers, add_heading_markers_where,
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
		let headed = heading_positions(&buffer);
		let spans = toc_spans(&toc_items);
		add_heading_markers_where(&mut buffer, &toc_items, 1, &|offset| !span_has_heading(&spans, &headed, offset));
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

/// Where the book's own text already carries a heading.
fn heading_positions(buffer: &DocumentBuffer) -> Vec<usize> {
	let mut positions: Vec<usize> =
		buffer.markers.iter().filter(|marker| is_heading_marker(marker.mtype)).map(|marker| marker.position).collect();
	positions.sort_unstable();
	positions
}

/// Every table of contents entry's offset, in order, so that the stretch of the book an entry
/// speaks for can be worked out from where the next one starts.
fn toc_spans(items: &[TocItem]) -> Vec<usize> {
	let mut offsets = Vec::new();
	collect_offsets(items, &mut offsets);
	offsets.sort_unstable();
	offsets
}

fn collect_offsets(items: &[TocItem], out: &mut Vec<usize>) {
	for item in items {
		out.push(item.offset);
		collect_offsets(&item.children, out);
	}
}

/// Whether the stretch of the book starting at `offset` already carries a heading of its own.
fn span_has_heading(spans: &[usize], headed: &[usize], offset: usize) -> bool {
	let end = spans.iter().copied().find(|start| *start > offset).unwrap_or(usize::MAX);
	headed.iter().any(|position| *position >= offset && *position < end)
}

#[cfg(test)]
mod tests {
	use super::{span_has_heading, toc_spans};
	use crate::document::TocItem;

	fn item(offset: usize) -> TocItem {
		TocItem::new(format!("at {offset}"), String::new(), offset)
	}

	/// The stretch an entry speaks for runs to wherever the next one starts, and the last entry's
	/// runs to the end of the book.
	#[test]
	fn a_chapter_that_wrote_its_own_heading_is_recognised() {
		let spans = toc_spans(&[item(0), item(100), item(200)]);
		let headed = [150];
		assert!(!span_has_heading(&spans, &headed, 0), "nothing between 0 and 100");
		assert!(span_has_heading(&spans, &headed, 100), "the heading at 150 belongs to this one");
		assert!(!span_has_heading(&spans, &headed, 200), "nothing at or after 200");
	}

	/// A heading past the last entry still belongs to it, since nothing follows to end its stretch.
	#[test]
	fn the_last_entry_runs_to_the_end_of_the_book() {
		let spans = toc_spans(&[item(0), item(100)]);
		assert!(span_has_heading(&spans, &[9_000], 100));
	}

	/// The spans are gathered from the whole tree, not only its top level, or a chapter's
	/// subsections would all be measured against the chapter after it.
	#[test]
	fn spans_are_gathered_from_the_whole_tree() {
		let mut parent = item(0);
		parent.children = vec![item(50)];
		assert_eq!(toc_spans(&[parent, item(100)]), vec![0, 50, 100]);
	}
}
