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
};

mod compression;
mod html;
mod huffman;
mod links;
mod toc;
mod varint;

use compression::{decompress_palmdoc, get_trailing_size};
use html::rewrite_font_size_headings;
use huffman::HuffmanDecoder;
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
		let title_bytes = &data[0..32];
		let num_records = u16::from_be_bytes([data[76], data[77]]) as usize;
		let mut record_offsets = Vec::new();
		for i in 0..num_records {
			let start = 78 + i * 8;
			if start + 4 > data.len() {
				tracing::warn!(record_index = i, "mobi record offset table is truncated");
				// TRANSLATORS: Error shown when a MOBI file's record offset table is truncated/corrupt
				anyhow::bail!(t("Invalid record offsets"));
			}
			let offset = u32::from_be_bytes([data[start], data[start + 1], data[start + 2], data[start + 3]]) as usize;
			record_offsets.push(offset);
		}
		if record_offsets.is_empty() {
			tracing::warn!("mobi file has no records");
			// TRANSLATORS: Error shown when a MOBI file has no records
			anyhow::bail!(t("No records found"));
		}
		let rec0_offset = record_offsets[0];
		let rec1_offset = if record_offsets.len() > 1 { record_offsets[1] } else { data.len() };
		if rec1_offset <= rec0_offset || rec1_offset > data.len() {
			tracing::warn!(rec0_offset, rec1_offset, "mobi record 0 has an invalid offset range");
			// TRANSLATORS: Error shown when a MOBI file's first record has an invalid offset range
			anyhow::bail!(t("Invalid Record 0 offsets"));
		}
		let rec0 = &data[rec0_offset..rec1_offset];
		if rec0.len() < 16 {
			tracing::warn!(len = rec0.len(), "mobi record 0 is too small to be valid");
			// TRANSLATORS: Error shown when a MOBI file's first record is too small to be valid
			anyhow::bail!(t("Invalid Record 0"));
		}
		let compression = u16::from_be_bytes([rec0[0], rec0[1]]);
		tracing::debug!(compression, "detected mobi compression mode");
		let mobi_header_offset = 16;
		if mobi_header_offset + 8 > rec0.len() {
			tracing::warn!("mobi record 0 is missing the mobi header");
			// TRANSLATORS: Error shown when a MOBI file is missing its MOBI header
			anyhow::bail!(t("No MOBI header"));
		}
		if &rec0[mobi_header_offset..mobi_header_offset + 4] != b"MOBI" {
			tracing::warn!("mobi header signature does not match the expected identifier");
			// TRANSLATORS: Error shown when a MOBI file's header signature doesn't match the expected "MOBI" identifier
			anyhow::bail!(t("Invalid MOBI identifier"));
		}
		let header_length = u32::from_be_bytes([
			rec0[mobi_header_offset + 4],
			rec0[mobi_header_offset + 5],
			rec0[mobi_header_offset + 6],
			rec0[mobi_header_offset + 7],
		]) as usize;
		let text_encoding = u32::from_be_bytes([
			rec0[mobi_header_offset + 12],
			rec0[mobi_header_offset + 13],
			rec0[mobi_header_offset + 14],
			rec0[mobi_header_offset + 15],
		]);
		let mut name_offset = 0usize;
		let mut name_length = 0usize;
		if header_length >= 76 && mobi_header_offset + 76 <= rec0.len() {
			name_offset = u32::from_be_bytes([
				rec0[mobi_header_offset + 68],
				rec0[mobi_header_offset + 69],
				rec0[mobi_header_offset + 70],
				rec0[mobi_header_offset + 71],
			]) as usize;
			name_length = u32::from_be_bytes([
				rec0[mobi_header_offset + 72],
				rec0[mobi_header_offset + 73],
				rec0[mobi_header_offset + 74],
				rec0[mobi_header_offset + 75],
			]) as usize;
		}
		let first_content_record = 1;
		let mut last_content_record = num_records.saturating_sub(1);
		if rec0.len() >= 10 {
			let text_record_count = u16::from_be_bytes([rec0[8], rec0[9]]) as usize;
			if text_record_count > 0 {
				last_content_record = text_record_count;
			}
		}
		if last_content_record >= num_records || first_content_record > last_content_record {
			tracing::warn!(
				first_content_record,
				last_content_record,
				num_records,
				"mobi content record range is invalid"
			);
			// TRANSLATORS: Error shown when a MOBI file's content record range is invalid
			anyhow::bail!(t("Invalid content record range"));
		}
		let mut document_title = if name_offset > 0 && name_length > 0 && name_offset + name_length <= rec0.len() {
			tracing::debug!("using exth name offset for document title");
			String::from_utf8_lossy(&rec0[name_offset..name_offset + name_length]).into_owned()
		} else {
			tracing::debug!("using raw header title field for document title");
			String::from_utf8_lossy(title_bytes).into_owned()
		};
		document_title = document_title.replace('\0', "").trim().replace('_', " ");
		let mut document_author = String::new();
		let mut exth_map = HashMap::new();
		let exth_offset = mobi_header_offset + header_length;
		if exth_offset + 12 <= rec0.len() && &rec0[exth_offset..exth_offset + 4] == b"EXTH" {
			let exth_num_records = u32::from_be_bytes([
				rec0[exth_offset + 8],
				rec0[exth_offset + 9],
				rec0[exth_offset + 10],
				rec0[exth_offset + 11],
			]) as usize;
			let mut p = exth_offset + 12;
			for _ in 0..exth_num_records {
				if p + 8 > rec0.len() {
					break;
				}
				let rec_type = u32::from_be_bytes([rec0[p], rec0[p + 1], rec0[p + 2], rec0[p + 3]]);
				let rec_len = u32::from_be_bytes([rec0[p + 4], rec0[p + 5], rec0[p + 6], rec0[p + 7]]) as usize;
				if p + rec_len > rec0.len() {
					break;
				}
				exth_map.insert(rec_type, rec0[p + 8..p + rec_len].to_vec());
				if rec_type == 100 {
					let exth_author = String::from_utf8_lossy(&rec0[p + 8..p + rec_len]).into_owned();
					if !exth_author.trim().is_empty() {
						document_author = exth_author;
					}
				} else if rec_type == 503 {
					let exth_title = String::from_utf8_lossy(&rec0[p + 8..p + rec_len]).into_owned();
					if !exth_title.trim().is_empty() {
						document_title = exth_title;
					}
				}
				p += rec_len;
			}
		}
		let mut huff_decoder = None;
		if compression == 17480 {
			if header_length >= 104 && mobi_header_offset + 104 <= rec0.len() {
				let huff_record_index = u32::from_be_bytes([
					rec0[mobi_header_offset + 96],
					rec0[mobi_header_offset + 97],
					rec0[mobi_header_offset + 98],
					rec0[mobi_header_offset + 99],
				]) as usize;
				let huff_record_count = u32::from_be_bytes([
					rec0[mobi_header_offset + 100],
					rec0[mobi_header_offset + 101],
					rec0[mobi_header_offset + 102],
					rec0[mobi_header_offset + 103],
				]) as usize;
				if huff_record_index > 0
					&& huff_record_count > 0
					&& huff_record_index + huff_record_count <= num_records
				{
					let mut huffs = Vec::new();
					for i in huff_record_index..huff_record_index + huff_record_count {
						let start = record_offsets[i];
						let end = if i + 1 < num_records { record_offsets[i + 1] } else { data.len() };
						huffs.push(&data[start..end]);
					}
					huff_decoder = Some(HuffmanDecoder::init(&huffs)?);
				} else {
					tracing::warn!("mobi huff/cdic records are invalid");
					// TRANSLATORS: Error shown when a MOBI file's Huffman/CDIC compression records are invalid
					anyhow::bail!(t("Invalid HUFF/CDIC records"));
				}
			} else {
				tracing::warn!("mobi header is missing huff compression parameters");
				// TRANSLATORS: Error shown when a MOBI file's header is missing Huffman compression parameters
				anyhow::bail!(t("Missing HUFF parameters in header"));
			}
		}
		let mut extra_data_flags = 0u32;
		let mobi_header = &rec0[mobi_header_offset..];
		if mobi_header.len() >= 24 {
			let mobi_version = u32::from_be_bytes([mobi_header[20], mobi_header[21], mobi_header[22], mobi_header[23]]);
			if mobi_version == 8 && mobi_header.len() >= 244 {
				tracing::debug!("using kf8 extra data flags offset");
				extra_data_flags =
					u32::from_be_bytes([mobi_header[224], mobi_header[225], mobi_header[226], mobi_header[227]]);
			} else {
				tracing::debug!("using legacy extra data flags offset");
				extra_data_flags = u32::from(u16::from_be_bytes([mobi_header[242], mobi_header[243]]));
			}
			if extra_data_flags == 0xFFFFFFFF {
				extra_data_flags = 0;
			}
		}
		let mut fdst_html_end = None;
		if mobi_header.len() >= 180 {
			let fdst_idx =
				u32::from_be_bytes([mobi_header[176], mobi_header[177], mobi_header[178], mobi_header[179]]) as usize;
			if fdst_idx != 0xFFFFFFFF && fdst_idx < num_records {
				let start = record_offsets[fdst_idx];
				let end = if fdst_idx + 1 < num_records { record_offsets[fdst_idx + 1] } else { data.len() };
				if start < end && end <= data.len() {
					let fdst_rec = &data[start..end];
					if fdst_rec.starts_with(b"FDST") && fdst_rec.len() >= 20 {
						let html_flow_end =
							u32::from_be_bytes([fdst_rec[16], fdst_rec[17], fdst_rec[18], fdst_rec[19]]) as usize;
						fdst_html_end = Some(html_flow_end);
					} else {
						tracing::debug!("fdst record does not start with fdst signature, skipping html end truncation");
					}
				} else {
					tracing::debug!("fdst record offset is out of range, skipping html end truncation");
				}
			} else if fdst_idx != 0xFFFFFFFF {
				tracing::debug!(fdst_idx, num_records, "fdst index is out of range, skipping html end truncation");
			}
		}
		let mut content = Vec::new();
		let mut trailing_entry_fallback_count = 0usize;
		let mut huff_decoder_missing_count = 0usize;
		for i in first_content_record..=last_content_record {
			let start = record_offsets[i];
			let end = if i + 1 < num_records { record_offsets[i + 1] } else { data.len() };
			if start >= data.len() || end > data.len() || start >= end {
				continue;
			}
			let mut record_data = &data[start..end];
			let trailing_entries = (extra_data_flags >> 1).count_ones();
			let mut stripped_len = record_data.len();
			if trailing_entries > 0 && !record_data.is_empty() {
				let mut valid = true;
				for _ in 0..trailing_entries {
					if stripped_len == 0 {
						break;
					}
					let size = get_trailing_size(&record_data[..stripped_len]);
					if size > stripped_len {
						valid = false;
						break;
					}
					stripped_len -= size;
				}
				if !valid {
					stripped_len = record_data.len();
					trailing_entry_fallback_count += 1;
				}
			}
			if extra_data_flags & 1 != 0 && stripped_len > 0 {
				let overlap_size = (record_data[stripped_len - 1] & 0x03) as usize;
				stripped_len = stripped_len.saturating_sub(overlap_size + 1);
			}
			if stripped_len != record_data.len() {
				record_data = &record_data[..stripped_len];
			}
			match compression {
				1 => content.extend_from_slice(record_data),
				2 => content.extend_from_slice(&decompress_palmdoc(record_data)),
				17480 => {
					if let Some(ref mut decoder) = huff_decoder {
						let decoded = decoder.decode(record_data)?;
						content.extend_from_slice(&decoded);
					} else {
						huff_decoder_missing_count += 1;
					}
				}
				other => {
					tracing::warn!(mode = other, "unsupported mobi compression mode");
					// TRANSLATORS: Error shown when a MOBI file uses an unrecognized compression mode; {} is the numeric mode value
					anyhow::bail!(t("Unsupported compression mode ({})").replace("{}", &other.to_string()))
				}
			}
		}
		if trailing_entry_fallback_count > 0 {
			tracing::warn!(
				count = trailing_entry_fallback_count,
				"records fell back to untrimmed data due to invalid trailing entry sizes"
			);
		}
		if huff_decoder_missing_count > 0 {
			tracing::warn!(
				count = huff_decoder_missing_count,
				"huffman decoder unexpectedly missing, dropped record content"
			);
		}
		if let Some(html_end) = fdst_html_end
			&& html_end < content.len()
		{
			content.truncate(html_end);
		}
		const MAX_MOBI_TEXT_BYTES: usize = 20 * 1024 * 1024;
		if content.len() > MAX_MOBI_TEXT_BYTES {
			tracing::warn!(
				original_len = content.len(),
				cap = MAX_MOBI_TEXT_BYTES,
				"mobi content exceeded max text size, truncating"
			);
			content.truncate(MAX_MOBI_TEXT_BYTES);
		}
		let text = if text_encoding == 65001 {
			tracing::debug!("decoding mobi content as utf-8");
			String::from_utf8_lossy(&content).into_owned()
		} else {
			tracing::debug!(text_encoding, "decoding mobi content as windows-1252");
			WINDOWS_1252.decode(&content).0.into_owned()
		};
		// Rewrite MOBI-style filepos links into standard href/id anchors before any
		// content is stripped, since filepos values are byte offsets into the raw HTML.
		let frag_offsets = build_fragment_offsets(&data, &record_offsets, mobi_header);
		let is_kf8 = {
			let mobi_version = if mobi_header.len() >= 24 {
				u32::from_be_bytes([mobi_header[20], mobi_header[21], mobi_header[22], mobi_header[23]])
			} else {
				0
			};
			mobi_version == 8
		};
		tracing::debug!(is_kf8, "detected mobi format version");
		let mut ncx_toc = parse_ncx(&data, &record_offsets, mobi_header, &exth_map, is_kf8, &frag_offsets);
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
		let mut html_converter = HtmlToText::with_render_tables_inline(context.render_tables_inline);
		html_converter.convert(&text, HtmlSourceMode::NativeHtml);
		if document_title.trim().is_empty() {
			document_title = extract_title_from_path(&context.file_path);
		}
		let mut document = Document::new().with_author(document_author);
		document.title = document_title;
		let mut buffer = DocumentBuffer::new();
		buffer.append(&html_converter.get_text());
		add_converter_markers(&mut buffer, &html_converter, 0);
		document.set_buffer(buffer);
		document.id_positions = html_converter.get_id_positions().clone();
		let mut toc_items = build_toc_from_headings(html_converter.get_headings());
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
			compression,
			is_kf8,
			text_encoding,
			num_records,
			toc_source,
			"parsed mobi file"
		);
		Ok(document)
	}
}
