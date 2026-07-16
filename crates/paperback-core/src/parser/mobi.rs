use std::{
	collections::{BTreeSet, HashMap},
	fs::File,
	io::Read,
	sync::LazyLock,
};

use anyhow::Result;
use encoding_rs::WINDOWS_1252;

use crate::{
	document::{Document, DocumentBuffer, ParserContext, ParserFlags, TocItem},
	parser::{
		Parser, add_converter_markers,
		html_to_text::{HtmlSourceMode, HtmlToText},
		util::{path::extract_title_from_path, toc::build_toc_from_headings},
	},
	t,
};

pub struct MobiParser;

impl Parser for MobiParser {
	fn name(&self) -> &'static str {
		"MOBI Books"
	}

	fn extensions(&self) -> &[&str] {
		&["mobi", "azw", "azw3"]
	}

	fn supported_flags(&self) -> ParserFlags {
		ParserFlags::SUPPORTS_TOC | ParserFlags::SUPPORTS_LISTS
	}

	fn parse(&self, context: &ParserContext) -> Result<Document> {
		let mut file = File::open(&context.file_path)?;
		let mut data = Vec::new();
		file.read_to_end(&mut data)?;
		if data.len() < 78 {
			// TRANSLATORS: Error shown when a MOBI file is too small to contain a valid header
			anyhow::bail!(t("File too short"));
		}
		let title_bytes = &data[0..32];
		let num_records = u16::from_be_bytes([data[76], data[77]]) as usize;
		let mut record_offsets = Vec::new();
		for i in 0..num_records {
			let start = 78 + i * 8;
			if start + 4 > data.len() {
				// TRANSLATORS: Error shown when a MOBI file's record offset table is truncated/corrupt
				anyhow::bail!(t("Invalid record offsets"));
			}
			let offset = u32::from_be_bytes([data[start], data[start + 1], data[start + 2], data[start + 3]]) as usize;
			record_offsets.push(offset);
		}
		if record_offsets.is_empty() {
			// TRANSLATORS: Error shown when a MOBI file has no records
			anyhow::bail!(t("No records found"));
		}
		let rec0_offset = record_offsets[0];
		let rec1_offset = if record_offsets.len() > 1 { record_offsets[1] } else { data.len() };
		if rec1_offset <= rec0_offset || rec1_offset > data.len() {
			// TRANSLATORS: Error shown when a MOBI file's first record has an invalid offset range
			anyhow::bail!(t("Invalid Record 0 offsets"));
		}
		let rec0 = &data[rec0_offset..rec1_offset];
		if rec0.len() < 16 {
			// TRANSLATORS: Error shown when a MOBI file's first record is too small to be valid
			anyhow::bail!(t("Invalid Record 0"));
		}
		let compression = u16::from_be_bytes([rec0[0], rec0[1]]);
		let mobi_header_offset = 16;
		if mobi_header_offset + 8 > rec0.len() {
			// TRANSLATORS: Error shown when a MOBI file is missing its MOBI header
			anyhow::bail!(t("No MOBI header"));
		}
		if &rec0[mobi_header_offset..mobi_header_offset + 4] != b"MOBI" {
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
			// TRANSLATORS: Error shown when a MOBI file's content record range is invalid
			anyhow::bail!(t("Invalid content record range"));
		}
		let mut document_title = if name_offset > 0 && name_length > 0 && name_offset + name_length <= rec0.len() {
			String::from_utf8_lossy(&rec0[name_offset..name_offset + name_length]).into_owned()
		} else {
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
					// TRANSLATORS: Error shown when a MOBI file's Huffman/CDIC compression records are invalid
					anyhow::bail!(t("Invalid HUFF/CDIC records"));
				}
			} else {
				// TRANSLATORS: Error shown when a MOBI file's header is missing Huffman compression parameters
				anyhow::bail!(t("Missing HUFF parameters in header"));
			}
		}
		let mut extra_data_flags = 0u32;
		let mobi_header = &rec0[mobi_header_offset..];
		if mobi_header.len() >= 24 {
			let mobi_version = u32::from_be_bytes([mobi_header[20], mobi_header[21], mobi_header[22], mobi_header[23]]);
			if mobi_version == 8 && mobi_header.len() >= 244 {
				extra_data_flags =
					u32::from_be_bytes([mobi_header[224], mobi_header[225], mobi_header[226], mobi_header[227]]);
			} else {
				extra_data_flags = u16::from_be_bytes([mobi_header[242], mobi_header[243]]) as u32;
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
					}
				}
			}
		}

		let mut content = Vec::new();
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
					}
				}
				// TRANSLATORS: Error shown when a MOBI file uses an unrecognized compression mode; {} is the numeric mode value
				other => anyhow::bail!(t("Unsupported compression mode ({})").replace("{}", &other.to_string())),
			}
		}

		if let Some(html_end) = fdst_html_end {
			if html_end < content.len() {
				content.truncate(html_end);
			}
		}

		const MAX_MOBI_TEXT_BYTES: usize = 20 * 1024 * 1024;
		if content.len() > MAX_MOBI_TEXT_BYTES {
			content.truncate(MAX_MOBI_TEXT_BYTES);
		}
		let text = if text_encoding == 65001 {
			String::from_utf8_lossy(&content).into_owned()
		} else {
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
		let mut ncx_toc = parse_ncx(&data, &record_offsets, mobi_header, &exth_map, is_kf8, &frag_offsets);

		fn extract_targets(items: &[TocItem], targets: &mut BTreeSet<usize>) {
			let mut stack = vec![items];
			while let Some(current_items) = stack.pop() {
				for item in current_items {
					if let Some(pos_str) = item.reference.strip_prefix("#fp") {
						if let Ok(pos) = pos_str.parse::<usize>() {
							targets.insert(pos);
						}
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
		if toc_items.is_empty() && !ncx_toc.is_empty() {
			resolve_ncx_offsets(&mut ncx_toc, &document.id_positions);
			toc_items = ncx_toc;
		}
		document.toc_items = toc_items;
		Ok(document)
	}
}

fn resolve_ncx_offsets(items: &mut [TocItem], id_positions: &HashMap<String, usize>) {
	let mut stack: Vec<&mut [TocItem]> = vec![items];
	while let Some(current) = stack.pop() {
		for item in current.iter_mut() {
			if item.offset == 0 && !item.reference.is_empty() {
				let key = if item.reference.starts_with('#') { &item.reference[1..] } else { &item.reference };
				if let Some(&pos) = id_positions.get(key) {
					item.offset = pos;
				}
			}
			if !item.children.is_empty() {
				stack.push(&mut item.children);
			}
		}
	}
}

// Old-style Mobipocket files use <font size="N"> for headings (size 7 = largest, 4-7 map to h1-h4).
// Only activated when the document contains no semantic h1-h6 tags.
fn rewrite_font_size_headings(html: &str) -> String {
	static RE_H1_6: LazyLock<regex::Regex> = LazyLock::new(|| regex::Regex::new(r"(?i)<h[1-6]\b").unwrap());
	if RE_H1_6.is_match(html) {
		return html.to_string();
	}
	let mut result = html.to_string();
	for (size, level) in [(7u8, 1u8), (6, 2), (5, 3), (4, 4)] {
		let Ok(re) = regex::Regex::new(&format!(r#"(?is)<font\b[^>]*\bsize=["']?{size}["']?[^>]*>(.*?)</font>"#))
		else {
			continue;
		};
		result = re
			.replace_all(&result, |caps: &regex::Captures<'_>| format!("<h{level}>{}</h{level}>", &caps[1]))
			.into_owned();
	}
	result
}

fn snap_to_char_boundary(s: &str, pos: usize) -> usize {
	let mut p = pos.min(s.len());
	while p > 0 && !s.is_char_boundary(p) {
		p -= 1;
	}
	p
}

// If pos falls inside or at the start of an HTML tag (<...>), advance it to
// just after the closing '>'. We look at the first '<' and '>' from pos
// onwards: if '>' comes before '<', we are inside a tag.
fn snap_past_open_tag(html: &str, pos: usize) -> usize {
	let bytes = &html.as_bytes()[pos..];
	let next_gt = bytes.iter().position(|&b| b == b'>');
	let next_lt = bytes.iter().position(|&b| b == b'<');
	match (next_gt, next_lt) {
		(Some(gt), Some(lt)) if gt < lt => return pos + gt + 1,
		(Some(gt), None) => return pos + gt + 1,
		(Some(0), _) | (None, Some(0)) => {
			if let Some(end) = bytes.iter().position(|&b| b == b'>') {
				return pos + end + 1;
			}
		}
		_ => {}
	}
	pos
}

fn decode_vwi(data: &[u8], mut pos: usize) -> (usize, usize) {
	let mut val: usize = 0;
	while pos < data.len() {
		let b = data[pos];
		pos += 1;
		val = (val << 7) | (b & 0x7F) as usize;
		if (b & 0x80) != 0 {
			break;
		}
	}
	(val, pos)
}

fn base32_decode(s: &str) -> usize {
	let mut val = 0;
	for c in s.chars() {
		val = (val << 5) | (c.to_digit(32).unwrap_or(0) as usize);
	}
	val
}

fn build_fragment_offsets(data: &[u8], records: &[usize], mobi_header: &[u8]) -> HashMap<usize, usize> {
	let mut frag_offsets = HashMap::new();
	if mobi_header.len() < 236 {
		return frag_offsets;
	}
	let frag_indx =
		u32::from_be_bytes([mobi_header[232], mobi_header[233], mobi_header[234], mobi_header[235]]) as usize;
	if frag_indx == 0xFFFFFFFF || frag_indx >= records.len() - 1 {
		return frag_offsets;
	}

	let prim_rec = &data[records[frag_indx]..records[frag_indx + 1]];
	if prim_rec.len() < 28 || &prim_rec[0..4] != b"INDX" {
		return frag_offsets;
	}
	let num_data_recs = u32::from_be_bytes([prim_rec[24], prim_rec[25], prim_rec[26], prim_rec[27]]) as usize;

	for i in 1..=num_data_recs {
		if frag_indx + i >= records.len() - 1 {
			break;
		}
		let data_rec = &data[records[frag_indx + i]..records[frag_indx + i + 1]];
		if data_rec.len() < 28 || &data_rec[0..4] != b"INDX" {
			continue;
		}

		let idxt_offset = u32::from_be_bytes([data_rec[20], data_rec[21], data_rec[22], data_rec[23]]) as usize;
		let num_entries = u32::from_be_bytes([data_rec[24], data_rec[25], data_rec[26], data_rec[27]]) as usize;

		if idxt_offset + 4 > data_rec.len() {
			continue;
		}
		let idxt = &data_rec[idxt_offset..];
		if &idxt[0..4] != b"IDXT" {
			continue;
		}

		for j in 0..num_entries {
			let entry_idx = 4 + j as usize * 2;
			if entry_idx + 2 > idxt.len() {
				break;
			}
			let entry_offset = u16::from_be_bytes([idxt[entry_idx], idxt[entry_idx + 1]]) as usize;
			if entry_offset >= data_rec.len() {
				continue;
			}

			let mut pos = entry_offset;
			let label_len = data_rec[pos] as usize;
			pos += 1;
			if pos + label_len > data_rec.len() {
				continue;
			}
			let label_str = match std::str::from_utf8(&data_rec[pos..pos + label_len]) {
				Ok(s) => s,
				Err(_) => continue,
			};
			let insert_offset = match label_str.parse::<usize>() {
				Ok(v) => v,
				Err(_) => continue,
			};
			pos += label_len;

			if pos >= data_rec.len() {
				continue;
			}
			let control = data_rec[pos];
			pos += 1;

			if (control & 1) != 0 {
				let (_, p) = decode_vwi(data_rec, pos);
				pos = p;
			}
			if (control & 2) != 0 {
				let (_, p) = decode_vwi(data_rec, pos);
				pos = p;
			}
			if (control & 4) != 0 {
				let (fid, _) = decode_vwi(data_rec, pos);
				frag_offsets.insert(fid, insert_offset);
			}
		}
	}
	frag_offsets
}

fn rewrite_internal_links(html: &str, frag_offsets: &HashMap<usize, usize>, extra_targets: &BTreeSet<usize>) -> String {
	static RE_LINKS: LazyLock<regex::Regex> = LazyLock::new(|| {
		regex::Regex::new(r#"(?i)<a\b[^>]*?(?:filepos\s*=\s*['"]?(\d+)|href\s*=\s*['"]?kindle:pos:(?:fid:([0-9A-Va-v]+):)?off:([0-9A-Va-v]+))[^>]*>"#).unwrap()
	});

	let mut links: Vec<(usize, usize, usize)> = Vec::new();
	let mut targets = extra_targets.clone();
	for cap in RE_LINKS.captures_iter(html) {
		let m = cap.get(0).unwrap();
		let mut filepos = None;
		if let Some(fpos) = cap.get(1) {
			filepos = fpos.as_str().parse::<usize>().ok();
		} else if let Some(off) = cap.get(3) {
			let off_val = base32_decode(off.as_str());
			if let Some(fid) = cap.get(2) {
				let f_idx = base32_decode(fid.as_str());
				if let Some(&base_offset) = frag_offsets.get(&f_idx) {
					filepos = Some(base_offset + off_val);
				}
			} else {
				filepos = Some(off_val);
			}
		}

		if let Some(filepos) = filepos {
			if filepos < html.len() {
				links.push((m.start(), m.end(), filepos));
				targets.insert(filepos);
			}
		}
	}
	if links.is_empty() && targets.is_empty() {
		return html.to_string();
	}
	// Build a sorted event list: inserts (kind=0) at target positions, replaces (kind=1) at link sites.
	// At equal positions inserts sort before replaces so the anchor lands before the link tag.
	let mut events: Vec<(usize, u8, usize, usize)> = Vec::new();
	for &target in &targets {
		events.push((target, 0, target, target));
	}
	for (start, end, filepos) in &links {
		events.push((*start, 1, *end, *filepos));
	}
	events.sort_unstable_by_key(|&(pos, kind, _, _)| (pos, kind));
	let mut result = String::with_capacity(html.len() + targets.len() * 30);
	let mut pos = 0usize;
	for (event_pos, kind, end, filepos) in events {
		let mut actual_pos = snap_to_char_boundary(html, event_pos);
		if kind == 0 {
			actual_pos = snap_past_open_tag(html, actual_pos);
		}
		if actual_pos < pos {
			continue;
		}
		result.push_str(&html[pos..actual_pos]);
		if kind == 0 {
			result.push_str(&format!("<a id=\"fp{filepos:010}\"></a>"));
			pos = actual_pos;
		} else {
			result.push_str(&format!("<a href=\"#fp{filepos:010}\">"));
			pos = end;
		}
	}
	result.push_str(&html[pos..]);
	result
}

fn get_trailing_size(data: &[u8]) -> usize {
	let mut size = 0usize;
	let mut pos = data.len() - 1;
	let mut shift = 0u32;
	loop {
		let b = data[pos];
		if shift < 32 {
			size |= ((b & 0x7f) as usize) << shift;
		}
		if b & 0x80 != 0 || pos == 0 {
			break;
		}
		pos -= 1;
		shift += 7;
	}
	size
}

fn decompress_palmdoc(data: &[u8]) -> Vec<u8> {
	let mut pos: usize = 0;
	let mut text: Vec<u8> = Vec::new();
	let mut prev: Option<u8> = None;
	while pos < data.len() {
		let byte = data[pos];
		pos += 1;
		match byte {
			new if prev.is_some() => {
				let old = prev.take().unwrap();
				let token = u16::from_be_bytes([old, new]) & 0x3fff;
				let offset = (token >> 3) as usize;
				let len = ((token & 0x0007) + 3) as usize;
				if offset == 0 || offset > text.len() {
					continue;
				}
				let start = text.len() - offset;
				for j in 0..len {
					let src = start + j;
					if src >= text.len() {
						break;
					}
					let b = text[src];
					text.push(b);
				}
			}
			0x00 | 0x09..=0x7f => text.push(byte),
			0x01..=0x08 => {
				let count = byte as usize;
				if pos + count <= data.len() {
					text.extend_from_slice(&data[pos..pos + count]);
					pos += count;
				}
			}
			0x80..=0xbf => {
				if pos >= data.len() {
					break;
				}
				prev = Some(byte);
			}
			_ => {
				text.push(b' ');
				text.push(byte ^ 0x80);
			}
		}
	}
	text
}

type HuffmanDictionary = Vec<Option<(Vec<u8>, bool)>>;
type CodeDictionary = [(u8, bool, u32); 256];
type MinCodesMapping = [u32; 33];
type MaxCodesMapping = [u32; 33];

struct HuffmanDecoder {
	dictionary: HuffmanDictionary,
	code_dict: CodeDictionary,
	min_codes: MinCodesMapping,
	max_codes: MaxCodesMapping,
}

impl Default for HuffmanDecoder {
	fn default() -> Self {
		Self { dictionary: vec![], code_dict: [(0, false, 0); 256], min_codes: [0; 33], max_codes: [u32::MAX; 33] }
	}
}

struct DecodeFrame {
	data: Vec<u8>,
	pos: usize,
	bits_left: usize,
	x: u64,
	n: i32,
	out: Vec<u8>,
	target_dict_index: Option<usize>,
}

impl HuffmanDecoder {
	fn init(huffs: &[&[u8]]) -> Result<Self> {
		let mut decoder = Self::default();
		decoder.load_huff(huffs[0])?;
		decoder.load_cdic_records(&huffs[1..])?;
		for i in 0..decoder.dictionary.len() {
			let (slice, flag) = decoder.dictionary[i].clone().unwrap();
			if !flag {
				decoder.dictionary[i] = None;
				let decoded = decoder.decode(&slice).unwrap_or(slice);
				decoder.dictionary[i] = Some((decoded, true));
			}
		}
		Ok(decoder)
	}

	fn load_huff(&mut self, huff: &[u8]) -> Result<()> {
		if huff.len() < 24 {
			// TRANSLATORS: Error shown when a MOBI file's Huffman compression record is too small to be valid
			anyhow::bail!(t("Invalid HUFF record"));
		}
		if &huff[0..4] != b"HUFF" {
			// TRANSLATORS: Error shown when a MOBI file's Huffman compression record has the wrong signature
			anyhow::bail!(t("Invalid HUFF header"));
		}
		let cache_offset = u32::from_be_bytes([huff[8], huff[9], huff[10], huff[11]]) as usize;
		let base_offset = u32::from_be_bytes([huff[12], huff[13], huff[14], huff[15]]) as usize;
		if cache_offset + 256 * 4 > huff.len() {
			// TRANSLATORS: Error shown when a MOBI file's Huffman cache table offset is out of bounds
			anyhow::bail!(t("Invalid HUFF cache offset"));
		}
		for i in 0..256 {
			let off = cache_offset + i * 4;
			let v = u32::from_be_bytes([huff[off], huff[off + 1], huff[off + 2], huff[off + 3]]);
			let code_len = (v & 0x1F) as u8;
			let term = (v & 0x80) == 0x80;
			let mut max_code = u64::from(v >> 8);
			if code_len == 0 {
				// TRANSLATORS: Error shown when a MOBI file's Huffman code length is invalid
				anyhow::bail!(t("Code len out of bounds"));
			}
			if code_len <= 8 && !term {
				// TRANSLATORS: Error shown when a MOBI file's Huffman table has an invalid terminal-code entry
				anyhow::bail!(t("Bad term"));
			}
			max_code = ((max_code + 1) << (32usize.saturating_sub(code_len as usize))).saturating_sub(1);
			self.code_dict[i] = (code_len, term, max_code as u32);
		}
		// Base table has 64 interleaved entries: [min1, max1, min2, max2, ... min32, max32]
		if base_offset + 64 * 4 > huff.len() {
			// TRANSLATORS: Error shown when a MOBI file's Huffman base table offset is out of bounds
			anyhow::bail!(t("Invalid HUFF base offset"));
		}
		for i in 1..=32usize {
			let min_off = base_offset + (i - 1) * 8;
			let max_off = base_offset + (i - 1) * 8 + 4;
			let min_val = if min_off + 4 <= huff.len() {
				u64::from(u32::from_be_bytes([huff[min_off], huff[min_off + 1], huff[min_off + 2], huff[min_off + 3]]))
			} else {
				0
			};
			let max_val = if max_off + 4 <= huff.len() {
				u64::from(u32::from_be_bytes([huff[max_off], huff[max_off + 1], huff[max_off + 2], huff[max_off + 3]]))
			} else {
				0
			};
			self.min_codes[i] = (min_val << (32 - i)) as u32;
			self.max_codes[i] = (((max_val + 1) << (32 - i)).saturating_sub(1)) as u32;
		}
		Ok(())
	}

	fn load_cdic_records(&mut self, records: &[&[u8]]) -> Result<()> {
		for cdic in records {
			if cdic.len() < 16 {
				continue;
			}
			if &cdic[0..4] != b"CDIC" {
				// TRANSLATORS: Error shown when a MOBI file's compressed dictionary record has the wrong signature
				anyhow::bail!(t("Invalid CDIC header"));
			}
			let num_phrases = u32::from_be_bytes([cdic[8], cdic[9], cdic[10], cdic[11]]);
			let bits = u32::from_be_bytes([cdic[12], cdic[13], cdic[14], cdic[15]]);
			let n = (1u32 << bits).min(num_phrases.saturating_sub(self.dictionary.len() as u32));
			let mut offsets = Vec::with_capacity(n as usize);
			for i in 0..n as usize {
				let off = 16 + i * 2;
				if off + 2 > cdic.len() {
					// TRANSLATORS: Error shown when a MOBI file's compressed dictionary offset table is out of bounds
					anyhow::bail!(t("Invalid CDIC offsets"));
				}
				offsets.push(u16::from_be_bytes([cdic[off], cdic[off + 1]]));
			}
			for offset in offsets {
				let off = 16 + offset as usize;
				if off + 2 > cdic.len() {
					// TRANSLATORS: Error shown when a MOBI file's compressed dictionary phrase offset is out of bounds
					anyhow::bail!(t("Invalid CDIC phrase offset"));
				}
				let num_bytes = u16::from_be_bytes([cdic[off], cdic[off + 1]]);
				let len = (num_bytes & 0x7FFF) as usize;
				if off + 2 + len > cdic.len() {
					// TRANSLATORS: Error shown when a MOBI file's compressed dictionary phrase length is out of bounds
					anyhow::bail!(t("Invalid CDIC phrase length"));
				}
				let bytes = cdic[off + 2..off + 2 + len].to_vec();
				self.dictionary.push(Some((bytes, (num_bytes & 0x8000) == 0x8000)));
			}
		}
		self.dictionary.reserve(4096);
		Ok(())
	}

	fn decode(&mut self, data: &[u8]) -> Result<Vec<u8>> {
		let mut stack: Vec<DecodeFrame> = Vec::with_capacity(32);
		let mut current = {
			let mut padded_data = Vec::with_capacity(data.len() + 8);
			padded_data.extend_from_slice(data);
			padded_data.extend_from_slice(&[0u8; 8]);
			let mut x_bytes = [0u8; 8];
			x_bytes.copy_from_slice(&padded_data[0..8]);
			DecodeFrame {
				data: padded_data,
				pos: 0,
				bits_left: data.len() * 8,
				x: u64::from_be_bytes(x_bytes),
				n: 32,
				out: Vec::new(),
				target_dict_index: None,
			}
		};
		loop {
			if current.n <= 0 {
				current.pos += 4;
				let mut x_bytes = [0u8; 8];
				if current.pos + 8 <= current.data.len() {
					x_bytes.copy_from_slice(&current.data[current.pos..current.pos + 8]);
				} else {
					// 1-3 remaining bytes: load zero-padded to 4 bytes
					let rem = current.data.len() - current.pos;
					x_bytes[..rem].copy_from_slice(&current.data[current.pos..]);
				}
				current.x = u64::from_be_bytes(x_bytes);
				current.n += 32;
			}
			let code = (current.x >> current.n.clamp(0, 32) as u32) as u32;
			let (code_len, term, mut max_code) = self.code_dict[(code >> 24) as usize];
			let mut code_len = code_len as usize;
			if !term {
				while code_len < 33 && code < self.min_codes[code_len] {
					code_len += 1;
				}
				if code_len < 33 {
					max_code = self.max_codes[code_len];
				}
			}
			if code_len == 0 || code_len > 32 {
				// TRANSLATORS: Error shown when a MOBI file's Huffman code length is out of range; {} is the invalid length value
				anyhow::bail!(t("Invalid code_len {}").replace("{}", &code_len.to_string()));
			}
			current.n -= code_len as i32;
			if current.bits_left < code_len {
				current.bits_left = 0;
			} else {
				current.bits_left -= code_len;
				if code > max_code {
					current.bits_left = 0;
				} else {
					let index = ((max_code - code) >> (32 - code_len)) as usize;
					if index >= self.dictionary.len() {
						current.bits_left = 0;
					} else {
						let (slice, flag) = match self.dictionary[index].clone() {
							Some(v) => v,
							None => (Vec::new(), true),
						};
						if flag {
							current.out.extend_from_slice(&slice);
						} else {
							self.dictionary[index] = None;
							stack.push(current);
							if stack.len() > 1024 {
								// TRANSLATORS: Error shown when a MOBI file's Huffman decoder recurses too deeply (likely corrupt data)
								anyhow::bail!(t("Decode stack overflow"));
							}
							current = {
								let mut padded_data = Vec::with_capacity(slice.len() + 8);
								padded_data.extend_from_slice(&slice);
								padded_data.extend_from_slice(&[0u8; 8]);
								let mut x_bytes = [0u8; 8];
								x_bytes.copy_from_slice(&padded_data[0..8]);
								DecodeFrame {
									data: padded_data,
									pos: 0,
									bits_left: slice.len() * 8,
									x: u64::from_be_bytes(x_bytes),
									n: 32,
									out: Vec::new(),
									target_dict_index: Some(index),
								}
							};
						}
					}
				}
			}
			while current.bits_left == 0 {
				let finished_out = current.out;
				if let Some(idx) = current.target_dict_index {
					self.dictionary[idx] = Some((finished_out.clone(), true));
				}
				if let Some(mut parent) = stack.pop() {
					parent.out.extend_from_slice(&finished_out);
					current = parent;
				} else {
					return Ok(finished_out);
				}
			}
		}
	}
}

fn parse_ncx(
	data: &[u8],
	records: &[usize],
	mobi_header: &[u8],
	exth: &HashMap<u32, Vec<u8>>,
	is_kf8: bool,
	frag_offsets: &HashMap<usize, usize>,
) -> Vec<TocItem> {
	let mut ncx_index = 0xFFFFFFFF;
	if is_kf8 && mobi_header.len() >= 232 {
		ncx_index = u32::from_be_bytes(mobi_header[228..232].try_into().unwrap_or([0; 4])) as usize;
	} else if !is_kf8 && mobi_header.len() >= 248 {
		ncx_index = u32::from_be_bytes(mobi_header[244..248].try_into().unwrap_or([0; 4])) as usize;
	}
	if ncx_index == 0xFFFFFFFF || ncx_index == 0 {
		if let Some(ext) = exth.get(&253) {
			if ext.len() >= 4 {
				ncx_index = u32::from_be_bytes([ext[0], ext[1], ext[2], ext[3]]) as usize;
			}
		}
	}
	if ncx_index == 0xFFFFFFFF || ncx_index == 0 || ncx_index >= records.len() - 1 {
		return Vec::new();
	}

	let indx_rec = &data[records[ncx_index]..records[ncx_index + 1]];
	if indx_rec.len() < 192 || &indx_rec[0..4] != b"INDX" {
		return Vec::new();
	}

	let count = u32::from_be_bytes(indx_rec[24..28].try_into().unwrap()) as usize;
	let cncx_count = u32::from_be_bytes(indx_rec[52..56].try_into().unwrap()) as usize;

	let cncx_start_rec = ncx_index + count + 1;
	let mut cncx_data = Vec::new();
	for i in 0..cncx_count {
		let rec_idx = cncx_start_rec + i;
		if rec_idx >= records.len() - 1 {
			break;
		}
		let rec = &data[records[rec_idx]..records[rec_idx + 1]];
		cncx_data.extend_from_slice(rec);
	}

	let tagx_start = u32::from_be_bytes(indx_rec[4..8].try_into().unwrap()) as usize;
	if tagx_start + 12 > indx_rec.len() || &indx_rec[tagx_start..tagx_start + 4] != b"TAGX" {
		return Vec::new();
	}

	let tagx_len = u32::from_be_bytes(indx_rec[tagx_start + 4..tagx_start + 8].try_into().unwrap()) as usize;
	let control_byte_count = u32::from_be_bytes(indx_rec[tagx_start + 8..tagx_start + 12].try_into().unwrap()) as usize;

	let mut tags = Vec::new();
	for i in (12..tagx_len).step_by(4) {
		let p = tagx_start + i;
		if p + 4 > indx_rec.len() {
			break;
		}
		let tag = indx_rec[p];
		let vpe = indx_rec[p + 1] as usize;
		let mask = indx_rec[p + 2] as u32;
		let end = indx_rec[p + 3];
		tags.push((tag, vpe, mask, end));
	}

	let mut idxt_start = 0;
	for i in (0..indx_rec.len().saturating_sub(4)).rev() {
		if &indx_rec[i..i + 4] == b"IDXT" {
			idxt_start = i;
			break;
		}
	}
	if idxt_start == 0 {
		return Vec::new();
	}

	let num_entries = (indx_rec.len() - idxt_start - 4) / 2;
	let mut offsets = Vec::new();
	for i in 0..num_entries {
		let p = idxt_start + 4 + i * 2;
		if p + 2 > indx_rec.len() {
			break;
		}
		let off = u16::from_be_bytes(indx_rec[p..p + 2].try_into().unwrap()) as usize;
		offsets.push(off);
	}

	let mut entries = Vec::new();
	for i in 0..=count {
		let rec_idx = ncx_index + i;
		if rec_idx >= records.len() - 1 {
			break;
		}
		let rec = &data[records[rec_idx]..records[rec_idx + 1]];

		let mut idxt = 0;
		for j in (0..rec.len().saturating_sub(4)).rev() {
			if &rec[j..j + 4] == b"IDXT" {
				idxt = j;
				break;
			}
		}
		if idxt == 0 {
			continue;
		}
		let num = (rec.len() - idxt - 4) / 2;
		for j in 0..num {
			let p = idxt + 4 + j * 2;
			if p + 2 > rec.len() {
				break;
			}
			let off = u16::from_be_bytes(rec[p..p + 2].try_into().unwrap()) as usize;

			if off >= rec.len() {
				continue;
			}

			let id_len = rec[off] as usize;
			let data_start = off + 1 + id_len;
			if data_start + control_byte_count > rec.len() {
				continue;
			}

			let mut cbytes = Vec::new();
			for k in 0..control_byte_count {
				cbytes.push(rec[data_start + k]);
			}

			let mut title_offset: Option<usize> = None;
			let mut pos: Option<usize> = None;
			let mut fid: Option<usize> = None;
			let mut lvl = 0;

			let mut vwi_offset = data_start + control_byte_count;
			let mut cbyte_idx = 0;

			for &(tag, vpe, mask, end_flag) in &tags {
				let cb = cbytes.get(cbyte_idx).copied().unwrap_or(0) as u32;
				if end_flag == 1 {
					cbyte_idx += 1;
				}
				if tag == 0 {
					continue;
				}

				let val = cb & mask;
				if val == 0 {
					continue;
				}

				let mut value_count = 0;
				let mut value_bytes = 0;

				if val == mask {
					if mask.count_ones() > 1 {
						if vwi_offset < rec.len() {
							let (v, next) = decode_vwi(rec, vwi_offset);
							value_bytes = v as usize;
							vwi_offset = next;
						}
					} else {
						value_count = 1;
					}
				} else {
					let mut m = mask;
					let mut v = val;
					while m & 1 == 0 {
						m >>= 1;
						v >>= 1;
					}
					value_count = v as usize;
				}

				let mut vals = Vec::new();
				if value_count > 0 {
					for _ in 0..(value_count * vpe) {
						if vwi_offset < rec.len() {
							let (v, next) = decode_vwi(rec, vwi_offset);
							vals.push(v);
							vwi_offset = next;
						}
					}
				} else if value_bytes > 0 {
					let mut total_consumed = 0;
					while total_consumed < value_bytes && vwi_offset < rec.len() {
						let (v, next) = decode_vwi(rec, vwi_offset);
						vals.push(v);
						total_consumed += next - vwi_offset;
						vwi_offset = next;
					}
				}

				if !vals.is_empty() {
					if tag == 1 {
						pos = Some(vals[0] as usize);
					}
					if tag == 3 {
						title_offset = Some(vals[0] as usize);
					}
					if tag == 4 {
						lvl = vals[0] as usize;
					}
					if tag == 6 {
						fid = Some(vals[0] as usize);
						if vals.len() > 1 {
							pos = Some(vals[1] as usize);
						}
					}
				}
			}

			if let (Some(toff), Some(p)) = (title_offset, pos) {
				if toff < cncx_data.len() {
					let (text_len, next) = decode_vwi(&cncx_data, toff);
					if next + text_len <= cncx_data.len() {
						let title_bytes = &cncx_data[next..next + text_len];
						let title = String::from_utf8_lossy(title_bytes).into_owned();
						let f = fid.unwrap_or(0);
						let filepos = frag_offsets.get(&f).copied().unwrap_or(0) + p;
						let lvl = if lvl == 0 { 1 } else { lvl as u32 };

						entries.push((title, lvl, format!("#fp{:010}", filepos)));
					}
				}
			}
		}
	}

	let mut toc: Vec<TocItem> = Vec::new();
	let mut stack: Vec<usize> = Vec::new();
	let mut levels: Vec<u32> = Vec::new();
	for (title, level, reference) in entries {
		if level == 0 {
			continue;
		}
		while let Some(&last_level) = levels.last() {
			if last_level < level {
				break;
			}
			stack.pop();
			levels.pop();
		}
		let item = TocItem::new(title, reference, 0);
		let mut current = &mut toc;
		for &idx in &stack {
			current = &mut current[idx].children;
		}
		current.push(item);
		stack.push(current.len() - 1);
		levels.push(level);
	}
	toc
}
