use std::{fs::File, io::Read};

use anyhow::Result;
use encoding_rs::WINDOWS_1252;

use crate::{
	document::{Document, DocumentBuffer, ParserContext, ParserFlags},
	html_to_text::{HtmlSourceMode, HtmlToText},
	parser::{Parser, add_converter_markers, path::extract_title_from_path, toc::build_toc_from_headings},
};

pub struct MobiParser;

impl Parser for MobiParser {
	fn name(&self) -> &'static str {
		"MOBI Books"
	}

	fn extensions(&self) -> &[&str] {
		&["mobi"]
	}

	fn supported_flags(&self) -> ParserFlags {
		ParserFlags::SUPPORTS_TOC | ParserFlags::SUPPORTS_LISTS
	}

	fn parse(&self, context: &ParserContext) -> Result<Document> {
		let mut file = File::open(&context.file_path)?;
		let mut data = Vec::new();
		file.read_to_end(&mut data)?;
		if data.len() < 78 {
			anyhow::bail!("File too short");
		}
		let title_bytes = &data[0..32];
		let num_records = u16::from_be_bytes([data[76], data[77]]) as usize;
		let mut record_offsets = Vec::new();
		for i in 0..num_records {
			let start = 78 + i * 8;
			if start + 4 > data.len() {
				anyhow::bail!("Invalid record offsets");
			}
			let offset = u32::from_be_bytes([data[start], data[start + 1], data[start + 2], data[start + 3]]) as usize;
			record_offsets.push(offset);
		}
		if record_offsets.is_empty() {
			anyhow::bail!("No records found");
		}
		let rec0_offset = record_offsets[0];
		let rec1_offset = if record_offsets.len() > 1 { record_offsets[1] } else { data.len() };
		if rec1_offset <= rec0_offset || rec1_offset > data.len() {
			anyhow::bail!("Invalid Record 0 offsets");
		}
		let rec0 = &data[rec0_offset..rec1_offset];
		if rec0.len() < 16 {
			anyhow::bail!("Invalid Record 0");
		}
		let compression = u16::from_be_bytes([rec0[0], rec0[1]]);
		let mobi_header_offset = 16;
		if mobi_header_offset + 8 > rec0.len() {
			anyhow::bail!("No MOBI header");
		}
		if &rec0[mobi_header_offset..mobi_header_offset + 4] != b"MOBI" {
			anyhow::bail!("Invalid MOBI identifier");
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
			anyhow::bail!("Invalid content record range");
		}
		let mut document_title = if name_offset > 0 && name_length > 0 && name_offset + name_length <= rec0.len() {
			String::from_utf8_lossy(&rec0[name_offset..name_offset + name_length]).into_owned()
		} else {
			String::from_utf8_lossy(title_bytes).into_owned()
		};
		document_title = document_title.replace('\0', "").trim().replace('_', " ");
		let mut document_author = String::new();
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
					anyhow::bail!("Invalid HUFF/CDIC records");
				}
			} else {
				anyhow::bail!("Missing HUFF parameters in header");
			}
		}
		let mut extra_data_flags = 0u32;
		if header_length >= 244 && mobi_header_offset + 244 <= rec0.len() {
			extra_data_flags = u32::from_be_bytes([
				rec0[mobi_header_offset + 240],
				rec0[mobi_header_offset + 241],
				rec0[mobi_header_offset + 242],
				rec0[mobi_header_offset + 243],
			]);
			if extra_data_flags == 0xFFFFFFFF {
				extra_data_flags = 0;
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
			if trailing_entries > 0 && !record_data.is_empty() {
				let mut stripped_len = record_data.len();
				for _ in 0..trailing_entries {
					if stripped_len == 0 {
						break;
					}
					let size = get_trailing_size(&record_data[..stripped_len]);
					stripped_len = stripped_len.saturating_sub(size);
				}
				if extra_data_flags & 1 != 0 && stripped_len > 0 {
					let overlap_size = (record_data[stripped_len - 1] & 0x07) as usize;
					stripped_len = stripped_len.saturating_sub(overlap_size + 1);
				}
				record_data = &record_data[..stripped_len];
			}
			match compression {
				1 => content.extend_from_slice(record_data),
				2 => content.extend_from_slice(&decompress_palmdoc(record_data)),
				17480 => {
					if let Some(ref mut decoder) = huff_decoder {
						content.extend_from_slice(&decoder.decode(record_data)?);
					}
				}
				other => anyhow::bail!("Unsupported compression mode ({})", other),
			}
		}
		const MAX_MOBI_TEXT_BYTES: usize = 20 * 1024 * 1024;
		if content.len() > MAX_MOBI_TEXT_BYTES {
			content.truncate(MAX_MOBI_TEXT_BYTES);
		}
		let mut text = if text_encoding == 65001 {
			String::from_utf8_lossy(&content).into_owned()
		} else {
			WINDOWS_1252.decode(&content).0.into_owned()
		};
		if let Ok(re) = regex::Regex::new(r"(?is)<title[^>]*>.*?</title>") {
			text = re.replace_all(&text, "").into_owned();
		}
		if let Ok(re) = regex::Regex::new(r"(?is)<style[^>]*>.*?</style>") {
			text = re.replace_all(&text, "").into_owned();
		}
		if let Ok(re) = regex::Regex::new(r"(?is)@page\s*\{[^<]+") {
			text = re.replace_all(&text, "").into_owned();
		}
		let mut html_converter = HtmlToText::new();
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
		let toc_items = build_toc_from_headings(html_converter.get_headings());
		document.toc_items = toc_items;
		Ok(document)
	}
}

fn get_trailing_size(data: &[u8]) -> usize {
	if data.is_empty() {
		return 0;
	}
	// If the last byte doesn't have bit 7 set it's not a valid VLQ terminator —
	// this happens when the trailing-entry count from extra_data_flags exceeds the
	// entries actually present. Treat the entry as absent.
	if data[data.len() - 1] & 0x80 == 0 {
		return 0;
	}
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
			anyhow::bail!("Invalid HUFF record");
		}
		if &huff[0..4] != b"HUFF" {
			anyhow::bail!("Invalid HUFF header");
		}
		let cache_offset = u32::from_be_bytes([huff[8], huff[9], huff[10], huff[11]]) as usize;
		let base_offset = u32::from_be_bytes([huff[12], huff[13], huff[14], huff[15]]) as usize;
		if cache_offset + 256 * 4 > huff.len() {
			anyhow::bail!("Invalid HUFF cache offset");
		}
		for i in 0..256 {
			let off = cache_offset + i * 4;
			let v = u32::from_be_bytes([huff[off], huff[off + 1], huff[off + 2], huff[off + 3]]);
			let code_len = (v & 0x1F) as u8;
			let term = (v & 0x80) == 0x80;
			let mut max_code = (v >> 8) as u64;
			if code_len == 0 {
				anyhow::bail!("Code len out of bounds");
			}
			if code_len <= 8 && !term {
				anyhow::bail!("Bad term");
			}
			max_code = ((max_code + 1) << (32usize.saturating_sub(code_len as usize))).saturating_sub(1);
			self.code_dict[i] = (code_len, term, max_code as u32);
		}
		// Base table has 64 interleaved entries: [min1, max1, min2, max2, ... min32, max32]
		if base_offset + 64 * 4 > huff.len() {
			anyhow::bail!("Invalid HUFF base offset");
		}
		for i in 1..=32usize {
			let min_off = base_offset + (i - 1) * 8;
			let max_off = base_offset + (i - 1) * 8 + 4;
			let min_val = if min_off + 4 <= huff.len() {
				u32::from_be_bytes([huff[min_off], huff[min_off + 1], huff[min_off + 2], huff[min_off + 3]]) as u64
			} else {
				0
			};
			let max_val = if max_off + 4 <= huff.len() {
				u32::from_be_bytes([huff[max_off], huff[max_off + 1], huff[max_off + 2], huff[max_off + 3]]) as u64
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
				anyhow::bail!("Invalid CDIC header");
			}
			let num_phrases = u32::from_be_bytes([cdic[8], cdic[9], cdic[10], cdic[11]]);
			let bits = u32::from_be_bytes([cdic[12], cdic[13], cdic[14], cdic[15]]);
			let n = (1u32 << bits).min(num_phrases.saturating_sub(self.dictionary.len() as u32));
			let mut offsets = Vec::with_capacity(n as usize);
			for i in 0..n as usize {
				let off = 16 + i * 2;
				if off + 2 > cdic.len() {
					anyhow::bail!("Invalid CDIC offsets");
				}
				offsets.push(u16::from_be_bytes([cdic[off], cdic[off + 1]]));
			}
			for offset in offsets {
				let off = 16 + offset as usize;
				if off + 2 > cdic.len() {
					anyhow::bail!("Invalid CDIC phrase offset");
				}
				let num_bytes = u16::from_be_bytes([cdic[off], cdic[off + 1]]);
				let len = (num_bytes & 0x7FFF) as usize;
				if off + 2 + len > cdic.len() {
					anyhow::bail!("Invalid CDIC phrase length");
				}
				let bytes = cdic[off + 2..off + 2 + len].to_vec();
				self.dictionary.push(Some((bytes, (num_bytes & 0x8000) == 0x8000)));
			}
		}
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
				anyhow::bail!("Invalid code_len {}", code_len);
			}
			current.n -= code_len as i32;
			if current.bits_left < code_len {
				break;
			}
			current.bits_left -= code_len;
			if code > max_code {
				break;
			}
			let index = ((max_code - code) >> (32 - code_len)) as usize;
			if index >= self.dictionary.len() {
				break;
			}
			let (slice, flag) = match self.dictionary[index].clone() {
				Some(v) => v,
				None => {
					// Cycle detected: this entry is already being decoded up the stack.
					// Break the cycle by emitting nothing for this reference.
					break;
				}
			};
			if flag {
				current.out.extend_from_slice(&slice);
			} else {
				self.dictionary[index] = None;
				stack.push(current);
				if stack.len() > 1024 {
					anyhow::bail!("Decode stack overflow");
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
		while let Some(mut parent) = stack.pop() {
			let finished_out = current.out;
			if let Some(idx) = current.target_dict_index {
				self.dictionary[idx] = Some((finished_out.clone(), true));
			}
			parent.out.extend_from_slice(&finished_out);
			current = parent;
		}
		Ok(current.out)
	}
}
