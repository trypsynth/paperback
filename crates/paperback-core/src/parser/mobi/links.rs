//! Rewriting MOBI/KF8 internal links: resolving `filepos`/`kindle:pos` anchors and NCX
//! fragment-id offsets into real `<a href="#fp...">`/`<a id="fp...">` anchors, since MOBI
//! stores links and TOC targets as raw byte offsets into the decoded content rather than
//! real element ids.

use std::{
	collections::{BTreeSet, HashMap},
	fmt::Write as _,
	sync::LazyLock,
};

use super::varint::{base32_decode, decode_vwi};
use crate::document::TocItem;

/// Fills in [`TocItem::offset`] for NCX entries whose `#fp...` reference couldn't be
/// resolved to a byte offset while parsing the NCX (no matching fragment-id), by looking
/// the reference up in the final HTML's id positions instead.
pub(super) fn resolve_ncx_offsets(items: &mut [TocItem], id_positions: &HashMap<String, usize>) {
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

pub(super) fn build_fragment_offsets(data: &[u8], records: &[usize], mobi_header: &[u8]) -> HashMap<usize, usize> {
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
			let entry_idx = 4 + j * 2;
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
			let Ok(label_str) = std::str::from_utf8(&data_rec[pos..pos + label_len]) else { continue };
			let Ok(insert_offset) = label_str.parse::<usize>() else { continue };
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

pub(super) fn rewrite_internal_links(
	html: &str,
	frag_offsets: &HashMap<usize, usize>,
	extra_targets: &BTreeSet<usize>,
) -> String {
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

		if let Some(filepos) = filepos
			&& filepos < html.len()
		{
			links.push((m.start(), m.end(), filepos));
			targets.insert(filepos);
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
			let _ = write!(result, "<a id=\"fp{filepos:010}\"></a>");
			pos = actual_pos;
		} else {
			let _ = write!(result, "<a href=\"#fp{filepos:010}\">");
			pos = end;
		}
	}
	result.push_str(&html[pos..]);
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
