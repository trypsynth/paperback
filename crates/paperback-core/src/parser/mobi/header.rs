use std::collections::HashMap;

use anyhow::Result;

use crate::t;

/// Fields pulled out of a MOBI file's PalmDOC/MOBI header (record 0) that the rest of parsing
/// needs. `rec0` is kept as an owned copy (record 0 is at most a few KB) so this struct does not
/// borrow the file's raw bytes.
pub(super) struct MobiHeader {
	pub rec0: Vec<u8>,
	pub record_offsets: Vec<usize>,
	pub mobi_header_offset: usize,
	pub header_length: usize,
	pub text_encoding: u32,
	pub compression: u16,
	pub first_content_record: usize,
	pub last_content_record: usize,
	pub extra_data_flags: u32,
	pub fdst_html_end: Option<usize>,
	pub document_title: String,
	pub is_kf8: bool,
}

impl MobiHeader {
	pub fn mobi_header(&self) -> &[u8] {
		&self.rec0[self.mobi_header_offset..]
	}
}

pub(super) fn parse_header(data: &[u8], title_bytes: &[u8]) -> Result<MobiHeader> {
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
	let rec0 = data[rec0_offset..rec1_offset].to_vec();
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
		tracing::warn!(first_content_record, last_content_record, num_records, "mobi content record range is invalid");
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
	let mut extra_data_flags = 0u32;
	// The trailing-data flags live at absolute record-0 offset 0xF2 (242), a fixed 2-byte
	// field regardless of MOBI version; that's 226 bytes into the mobi header, which starts
	// at record-0 offset 16.
	if rec0.len() >= mobi_header_offset + 228 {
		let mobi_header = &rec0[mobi_header_offset..];
		extra_data_flags = u32::from(u16::from_be_bytes([mobi_header[226], mobi_header[227]]));
		if extra_data_flags == 0xFFFFFFFF {
			extra_data_flags = 0;
		}
	}
	let mut fdst_html_end = None;
	if rec0.len() >= mobi_header_offset + 180 {
		let mobi_header = &rec0[mobi_header_offset..];
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
	let is_kf8 = {
		let mobi_header = &rec0[mobi_header_offset..];
		let mobi_version = if mobi_header.len() >= 24 {
			u32::from_be_bytes([mobi_header[20], mobi_header[21], mobi_header[22], mobi_header[23]])
		} else {
			0
		};
		mobi_version == 8
	};
	tracing::debug!(is_kf8, "detected mobi format version");
	Ok(MobiHeader {
		rec0,
		record_offsets,
		mobi_header_offset,
		header_length,
		text_encoding,
		compression,
		first_content_record,
		last_content_record,
		extra_data_flags,
		fdst_html_end,
		document_title,
		is_kf8,
	})
}

/// Reads the EXTH metadata block (if present) following the MOBI header, returning the raw
/// record map plus any author/title overrides it carries.
pub(super) fn parse_exth(header: &MobiHeader) -> (HashMap<u32, Vec<u8>>, Option<String>, Option<String>) {
	let mut exth_map = HashMap::new();
	let mut author = None;
	let mut title = None;
	let rec0 = &header.rec0;
	let exth_offset = header.mobi_header_offset + header.header_length;
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
					author = Some(exth_author);
				}
			} else if rec_type == 503 {
				let exth_title = String::from_utf8_lossy(&rec0[p + 8..p + rec_len]).into_owned();
				if !exth_title.trim().is_empty() {
					title = Some(exth_title);
				}
			}
			p += rec_len;
		}
	}
	(exth_map, author, title)
}
