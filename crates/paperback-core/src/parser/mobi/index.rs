//! One tagged index format serves the NCX and KF8's skeleton and fragment tables, so this hands back raw tag values and leaves their meaning to the caller.

use std::collections::HashMap;

use super::varint::decode_vwi;

/// Bytes of `INDX` header before the per-record data, and the smallest a header can be.
const INDX_HEADER_LEN: usize = 192;

/// The entry's own name is not kept: the indices this crate uses carry everything they mean in their tags.
#[derive(Debug, Clone)]
pub(super) struct IndexEntry {
	/// A tag can carry several values, which is how tag 6 gives a start position and a length together.
	pub tags: HashMap<u8, Vec<usize>>,
}

#[derive(Debug, Default)]
pub(super) struct Index {
	pub entries: Vec<IndexEntry>,
	/// A tag value naming a string is a byte offset into this.
	cncx: Vec<u8>,
}

impl Index {
	pub fn string_at(&self, offset: usize) -> Option<String> {
		if offset >= self.cncx.len() {
			return None;
		}
		let (len, start) = decode_vwi(&self.cncx, offset);
		let end = start.checked_add(len)?;
		self.cncx.get(start..end).map(|bytes| String::from_utf8_lossy(bytes).into_owned())
	}
}

/// `None` is the ordinary answer for a book that simply has no such index.
pub(super) fn read_index(data: &[u8], records: &[usize], index_record: usize) -> Option<Index> {
	if index_record == 0 || index_record == 0xFFFF_FFFF || index_record + 1 >= records.len() {
		return None;
	}
	let header = data.get(records[index_record]..records[index_record + 1])?;
	if header.len() < INDX_HEADER_LEN || &header[0..4] != b"INDX" {
		return None;
	}
	let count = u32::from_be_bytes(header[24..28].try_into().ok()?) as usize;
	let cncx_count = u32::from_be_bytes(header[52..56].try_into().ok()?) as usize;
	let tags = parse_tagx(header)?;
	// The CNCX records follow the index's own records, so the first sits that far along.
	let mut cncx = Vec::new();
	for i in 0..cncx_count {
		let record = index_record + count + 1 + i;
		if record + 1 >= records.len() {
			break;
		}
		if let Some(bytes) = data.get(records[record]..records[record + 1]) {
			cncx.extend_from_slice(bytes);
		}
	}
	let mut entries = Vec::new();
	for i in 1..=count {
		let record = index_record + i;
		if record + 1 >= records.len() {
			break;
		}
		let Some(bytes) = data.get(records[record]..records[record + 1]) else { break };
		read_record(bytes, &tags.tags, tags.control_bytes, &mut entries);
	}
	Some(Index { entries, cncx })
}

struct Tagx {
	tags: Vec<TagDescriptor>,
	control_bytes: usize,
}

struct TagDescriptor {
	tag: u8,
	values_per_entry: usize,
	/// Which bits of the control byte say whether this tag is present, and how often.
	mask: u32,
	/// Set on the last descriptor sharing a control byte, so the reader steps to the next.
	ends_control_byte: bool,
}

fn parse_tagx(header: &[u8]) -> Option<Tagx> {
	let start = u32::from_be_bytes(header[4..8].try_into().ok()?) as usize;
	if start + 12 > header.len() || header.get(start..start + 4)? != b"TAGX" {
		return None;
	}
	let length = u32::from_be_bytes(header.get(start + 4..start + 8)?.try_into().ok()?) as usize;
	let control_bytes = u32::from_be_bytes(header.get(start + 8..start + 12)?.try_into().ok()?) as usize;
	let mut tags = Vec::new();
	for i in (12..length).step_by(4) {
		let at = start + i;
		let Some(row) = header.get(at..at + 4) else { break };
		tags.push(TagDescriptor {
			tag: row[0],
			values_per_entry: usize::from(row[1]),
			mask: u32::from(row[2]),
			ends_control_byte: row[3] == 1,
		});
	}
	Some(Tagx { tags, control_bytes })
}

/// The `IDXT` offset and count come from the record's own header rather than a marker search, since the bytes after the table would read as garbage entries.
fn read_record(record: &[u8], tags: &[TagDescriptor], control_bytes: usize, out: &mut Vec<IndexEntry>) {
	if record.len() < 28 || &record[0..4] != b"INDX" {
		return;
	}
	let idxt = u32::from_be_bytes(record[20..24].try_into().unwrap_or([0; 4])) as usize;
	let count = u32::from_be_bytes(record[24..28].try_into().unwrap_or([0; 4])) as usize;
	for i in 0..count {
		let at = idxt + 4 + i * 2;
		let Some(bytes) = record.get(at..at + 2) else { break };
		let start = usize::from(u16::from_be_bytes(bytes.try_into().unwrap_or([0; 2])));
		if let Some(entry) = read_entry(record, start, tags, control_bytes) {
			out.push(entry);
		}
	}
}

fn read_entry(record: &[u8], start: usize, tags: &[TagDescriptor], control_bytes: usize) -> Option<IndexEntry> {
	let name_len = usize::from(*record.get(start)?);
	let name_end = start + 1 + name_len;
	let control = record.get(name_end..name_end + control_bytes)?;
	let mut at = name_end + control_bytes;
	let mut values: HashMap<u8, Vec<usize>> = HashMap::new();
	let mut control_index = 0;
	for descriptor in tags {
		// A row marking the end of a control byte is a separator, not a tag: it steps the reader to the next byte and carries nothing of its own.
		if descriptor.ends_control_byte {
			control_index += 1;
			continue;
		}
		if descriptor.tag == 0 {
			continue;
		}
		let byte = u32::from(control.get(control_index).copied().unwrap_or(0));
		let present = byte & descriptor.mask;
		if present == 0 {
			continue;
		}
		// A tag whose mask is fully set either repeats a fixed number of times or states how many bytes of values follow; a partial mask holds the repeat count itself.
		let mut count = 0;
		let mut byte_length = 0;
		if present == descriptor.mask {
			if descriptor.mask.count_ones() > 1 {
				if at < record.len() {
					let (length, next) = decode_vwi(record, at);
					byte_length = length;
					at = next;
				}
			} else {
				count = 1;
			}
		} else {
			count = (present >> descriptor.mask.trailing_zeros()) as usize;
		}
		let mut read = Vec::new();
		if count > 0 {
			for _ in 0..count * descriptor.values_per_entry {
				if at >= record.len() {
					break;
				}
				let (value, next) = decode_vwi(record, at);
				read.push(value);
				at = next;
			}
		} else if byte_length > 0 {
			let mut consumed = 0;
			while consumed < byte_length && at < record.len() {
				let (value, next) = decode_vwi(record, at);
				read.push(value);
				consumed += next - at;
				at = next;
			}
		}
		if !read.is_empty() {
			values.insert(descriptor.tag, read);
		}
	}
	Some(IndexEntry { tags: values })
}

#[cfg(test)]
mod tests {
	#![allow(clippy::cast_possible_truncation)]

	use super::*;

	/// Every value the tests use fits in one byte.
	fn vwi(value: u8) -> u8 {
		value | 0x80
	}

	/// `entries` are `(name, control byte, values)` triples, the values already encoded.
	fn indx_record(entries: &[(&str, u8, &[u8])]) -> Vec<u8> {
		let mut body = Vec::new();
		let mut offsets = Vec::new();
		for (name, control, values) in entries {
			offsets.push(192 + body.len());
			body.push(name.len() as u8);
			body.extend_from_slice(name.as_bytes());
			body.push(*control);
			body.extend_from_slice(values);
		}
		let idxt = 192 + body.len();
		let mut record = vec![0u8; 192];
		record[0..4].copy_from_slice(b"INDX");
		record[20..24].copy_from_slice(&(idxt as u32).to_be_bytes());
		record[24..28].copy_from_slice(&(entries.len() as u32).to_be_bytes());
		record.extend_from_slice(&body);
		record.extend_from_slice(b"IDXT");
		for offset in offsets {
			record.extend_from_slice(&(offset as u16).to_be_bytes());
		}
		record
	}

	/// `tagx` rows are `(tag, values per entry, mask, ends control byte)`.
	fn index_of(tagx: &[(u8, u8, u8, u8)], entries: &[(&str, u8, &[u8])]) -> (Vec<u8>, Vec<usize>) {
		let mut header = vec![0u8; 192];
		header[0..4].copy_from_slice(b"INDX");
		header[4..8].copy_from_slice(&192u32.to_be_bytes());
		header[24..28].copy_from_slice(&1u32.to_be_bytes());
		header.extend_from_slice(b"TAGX");
		header.extend_from_slice(&((12 + tagx.len() * 4) as u32).to_be_bytes());
		header.extend_from_slice(&1u32.to_be_bytes());
		for (tag, per, mask, end) in tagx {
			header.extend_from_slice(&[*tag, *per, *mask, *end]);
		}
		let entry_record = indx_record(entries);
		// Record 0 is the book's own header in a real file, so the reader rejects an index there and the test layout leaves room for it.
		let mut data = vec![0u8; 16];
		let records = vec![0, 16, 16 + header.len(), 16 + header.len() + entry_record.len()];
		data.extend_from_slice(&header);
		data.extend_from_slice(&entry_record);
		(data, records)
	}

	// The shape the skeleton table's fragment count uses.
	#[test]
	fn a_single_bit_tag_reads_one_value() {
		let (data, records) = index_of(&[(1, 1, 0x01, 0), (0, 0, 0, 1)], &[("SKEL", 0x01, &[vwi(8)])]);
		let index = read_index(&data, &records, 1).expect("index");
		assert_eq!(index.entries.len(), 1);
		assert_eq!(index.entries[0].tags.get(&1), Some(&vec![8]));
	}

	#[test]
	fn a_tag_with_two_values_per_entry_gives_both() {
		let (data, records) = index_of(&[(6, 2, 0x01, 0), (0, 0, 0, 1)], &[("SKEL", 0x01, &[vwi(10), vwi(20)])]);
		let index = read_index(&data, &records, 1).expect("index");
		assert_eq!(index.entries[0].tags.get(&6), Some(&vec![10, 20]));
	}

	// Reading this separator row as a tag made every following tag take its values from the wrong control byte.
	#[test]
	fn the_row_ending_a_control_byte_carries_no_values() {
		let tagx = &[(1, 1, 0x01, 0), (6, 2, 0x02, 0), (0, 0, 0, 1)];
		let (data, records) = index_of(tagx, &[("SKEL", 0x03, &[vwi(7), vwi(10), vwi(20)])]);
		let index = read_index(&data, &records, 1).expect("index");
		let tags = &index.entries[0].tags;
		assert_eq!(tags.get(&1), Some(&vec![7]));
		assert_eq!(tags.get(&6), Some(&vec![10, 20]));
		assert_eq!(tags.len(), 2);
	}

	#[test]
	fn a_tag_whose_bit_is_clear_is_absent() {
		let tagx = &[(1, 1, 0x01, 0), (6, 2, 0x02, 0), (0, 0, 0, 1)];
		let (data, records) = index_of(tagx, &[("SKEL", 0x02, &[vwi(10), vwi(20)])]);
		let index = read_index(&data, &records, 1).expect("index");
		assert!(!index.entries[0].tags.contains_key(&1));
		assert_eq!(index.entries[0].tags.get(&6), Some(&vec![10, 20]));
	}

	// Several entries in one record all come back, rather than only whichever a marker search happened to reach.
	#[test]
	fn every_entry_in_a_record_is_read() {
		let entries: &[(&str, u8, &[u8])] = &[("A", 0x01, &[vwi(2)]), ("B", 0x01, &[vwi(4)]), ("C", 0x01, &[vwi(6)])];
		let (data, records) = index_of(&[(1, 1, 0x01, 0), (0, 0, 0, 1)], entries);
		let index = read_index(&data, &records, 1).expect("index");
		let values: Vec<usize> = index.entries.iter().filter_map(|e| e.tags.get(&1)?.first().copied()).collect();
		assert_eq!(values, [2, 4, 6]);
	}

	#[test]
	fn a_cncx_string_is_read_through_its_length_prefix() {
		let index = Index { entries: Vec::new(), cncx: vec![0x85, b'h', b'e', b'l', b'l', b'o', 0x82, b'h', b'i'] };
		assert_eq!(index.string_at(0).as_deref(), Some("hello"));
		assert_eq!(index.string_at(6).as_deref(), Some("hi"));
		assert!(index.string_at(99).is_none());
	}

	#[test]
	fn a_record_that_is_not_an_index_reads_as_none() {
		let data = vec![0u8; 512];
		assert!(read_index(&data, &[0, 256, 512], 1).is_none());
	}

	#[test]
	fn an_out_of_range_record_reads_as_none() {
		let data = vec![0u8; 512];
		assert!(read_index(&data, &[0, 256, 512], 9).is_none());
		assert!(read_index(&data, &[0, 256, 512], 0).is_none());
		assert!(read_index(&data, &[0, 256, 512], 0xFFFF_FFFF).is_none());
	}
}
