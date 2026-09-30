use anyhow::Result;

use super::{
	compression::{decompress_palmdoc, get_trailing_size},
	header::MobiHeader,
	huffman::HuffmanDecoder,
};
use crate::t;

fn build_huffman_decoder(header: &MobiHeader, data: &[u8]) -> Result<HuffmanDecoder> {
	let mobi_header = header.mobi_header();
	if header.header_length < 104 || header.mobi_header_offset + 104 > header.rec0.len() {
		tracing::warn!("mobi header is missing huff compression parameters");
		// TRANSLATORS: Error shown when a MOBI file's header is missing Huffman compression parameters
		anyhow::bail!(t("Missing HUFF parameters in header"));
	}
	let huff_record_index =
		u32::from_be_bytes([mobi_header[96], mobi_header[97], mobi_header[98], mobi_header[99]]) as usize;
	let huff_record_count =
		u32::from_be_bytes([mobi_header[100], mobi_header[101], mobi_header[102], mobi_header[103]]) as usize;
	let num_records = header.record_offsets.len();
	if huff_record_index == 0 || huff_record_count == 0 || huff_record_index + huff_record_count > num_records {
		tracing::warn!("mobi huff/cdic records are invalid");
		// TRANSLATORS: Error shown when a MOBI file's Huffman/CDIC compression records are invalid
		anyhow::bail!(t("Invalid HUFF/CDIC records"));
	}
	let mut huffs = Vec::new();
	for i in huff_record_index..huff_record_index + huff_record_count {
		let start = header.record_offsets[i];
		let end = if i + 1 < num_records { header.record_offsets[i + 1] } else { data.len() };
		huffs.push(&data[start..end]);
	}
	HuffmanDecoder::init(&huffs)
}

/// Decodes and concatenates every content record into the book's raw HTML bytes, undoing
/// PalmDOC/Huffman compression and stripping each record's trailing multibyte-boundary data.
pub(super) fn decode_content(data: &[u8], header: &MobiHeader) -> Result<Vec<u8>> {
	let mut huff_decoder = if header.compression == 17480 { Some(build_huffman_decoder(header, data)?) } else { None };
	let num_records = header.record_offsets.len();
	let mut content = Vec::new();
	let mut trailing_entry_fallback_count = 0usize;
	let mut huff_decoder_missing_count = 0usize;
	for i in header.first_content_record..=header.last_content_record {
		let start = header.record_offsets[i];
		let end = if i + 1 < num_records { header.record_offsets[i + 1] } else { data.len() };
		if start >= data.len() || end > data.len() || start >= end {
			continue;
		}
		let mut record_data = &data[start..end];
		let trailing_entries = (header.extra_data_flags >> 1).count_ones();
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
		if header.extra_data_flags & 1 != 0 && stripped_len > 0 {
			let overlap_size = (record_data[stripped_len - 1] & 0x03) as usize;
			stripped_len = stripped_len.saturating_sub(overlap_size + 1);
		}
		if stripped_len != record_data.len() {
			record_data = &record_data[..stripped_len];
		}
		match header.compression {
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
	if let Some(html_end) = header.fdst_html_end
		&& html_end < content.len()
	{
		content.truncate(html_end);
	}
	Ok(content)
}
