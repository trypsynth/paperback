//! MOBI HUFF/CDIC decompression (`compression` mode 17480): builds a code table from the
//! `HUFF` record and a phrase dictionary from the `CDIC` record(s), then recursively
//! expands compressed text through them. A dictionary phrase can itself reference other
//! phrases, hence the explicit stack in `decode` rather than plain recursion.

use anyhow::Result;

use crate::t;

type HuffmanDictionary = Vec<Option<(Vec<u8>, bool)>>;
type CodeDictionary = [(u8, bool, u32); 256];
type MinCodesMapping = [u32; 33];
type MaxCodesMapping = [u32; 33];

pub(super) struct HuffmanDecoder {
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
	pub(super) fn init(huffs: &[&[u8]]) -> Result<Self> {
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

	pub(super) fn decode(&mut self, data: &[u8]) -> Result<Vec<u8>> {
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
						let (slice, flag) = self.dictionary[index].clone().unwrap_or_else(|| (Vec::new(), true));
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
