//! MOBI/KF8 NCX table-of-contents parsing: walks the `INDX`/`TAGX`/`CNCX` index structure
//! (Mobipocket's generic tagged-index format, also used for other index types) to
//! reconstruct a hierarchical table of contents from the book's NCX index.

use std::collections::HashMap;

use super::varint::decode_vwi;
use crate::document::TocItem;

pub(super) fn parse_ncx(
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
	if (ncx_index == 0xFFFFFFFF || ncx_index == 0)
		&& let Some(ext) = exth.get(&253)
		&& ext.len() >= 4
	{
		ncx_index = u32::from_be_bytes([ext[0], ext[1], ext[2], ext[3]]) as usize;
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
		let mask = u32::from(indx_rec[p + 2]);
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
				let cb = u32::from(cbytes.get(cbyte_idx).copied().unwrap_or(0));
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
							value_bytes = v;
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
						pos = Some(vals[0]);
					}
					if tag == 3 {
						title_offset = Some(vals[0]);
					}
					if tag == 4 {
						lvl = vals[0];
					}
					if tag == 6 {
						fid = Some(vals[0]);
						if vals.len() > 1 {
							pos = Some(vals[1]);
						}
					}
				}
			}
			if let (Some(toff), Some(p)) = (title_offset, pos)
				&& toff < cncx_data.len()
			{
				let (text_len, next) = decode_vwi(&cncx_data, toff);
				if next + text_len <= cncx_data.len() {
					let title_bytes = &cncx_data[next..next + text_len];
					let title = String::from_utf8_lossy(title_bytes).into_owned();
					let f = fid.unwrap_or(0);
					let filepos = frag_offsets.get(&f).copied().unwrap_or(0) + p;
					let lvl = if lvl == 0 { 1 } else { lvl as u32 };
					entries.push((title, lvl, format!("#fp{filepos:010}")));
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
