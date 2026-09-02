//! Legacy binary `.ppt` parsing: walks the OLE `PowerPoint Document` stream's record tree
//! directly, since there is no XML to hand off to a library.

use std::{collections::HashMap, fs::File, io::Read};

use anyhow::{Context, Result};
use cfb::CompoundFile;

use crate::{
	document::{Document, DocumentBuffer, Marker, MarkerType, ParserContext, TocItem},
	parser::util::path::extract_title_from_path,
	t,
};

const PPT_RECORD_HEADER_SIZE: usize = 8;
const PPT_REC_SLIDE: u16 = 1006;
const PPT_REC_TEXT_CHARS_ATOM: u16 = 4000;
const PPT_REC_TEXT_BYTES_ATOM: u16 = 4008;
const PPT_REC_CSTRING: u16 = 4026;

pub(super) fn parse_legacy_ppt(context: &ParserContext) -> Result<Document> {
	tracing::debug!(path = %context.file_path, "parsing legacy ppt file");
	let file =
		File::open(&context.file_path).with_context(|| format!("Failed to open PPT file '{}'", context.file_path))?;
	let mut compound =
		CompoundFile::open(file).with_context(|| format!("Failed to parse OLE container '{}'", context.file_path))?;
	// Encrypted PPT files have an EncryptionInfo stream. We can detect but not decrypt them.
	if compound.entry("/EncryptionInfo").is_ok() {
		tracing::warn!(path = %context.file_path, "legacy ppt file is encrypted, not supported");
		// TRANSLATORS: Error shown when a legacy PPT file is password-protected, which this parser cannot handle
		anyhow::bail!(t(
			"Password-protected PPT files are not currently supported. Try saving the file as PPTX and opening that instead."
		));
	}
	let ppt_document_stream = read_ppt_document_stream(&mut compound)
		.inspect_err(
			|e| tracing::warn!(path = %context.file_path, error = %e, "failed to read powerpoint document stream"),
		)
		.with_context(|| format!("Failed to read PowerPoint Document stream from '{}'", context.file_path))?;
	let slide_texts = collect_legacy_slide_texts(&ppt_document_stream);
	if slide_texts.is_empty() {
		tracing::warn!(path = %context.file_path, "legacy ppt file has no slides");
		// TRANSLATORS: Error shown when a legacy PPT presentation file has no slides
		anyhow::bail!(t("PPT file contains no slides"));
	}
	let mut buffer = DocumentBuffer::new();
	let mut toc_items = Vec::with_capacity(slide_texts.len());
	let mut id_positions = HashMap::new();
	for (index, slide_text) in slide_texts.iter().enumerate() {
		let slide_number = index + 1;
		let slide_start = buffer.current_position();
		let label = format!("Slide {slide_number}");
		id_positions.insert(format!("slide_{slide_number}"), slide_start);
		buffer.add_marker(Marker::new(MarkerType::PageBreak, slide_start).with_text(label.clone()));
		if !slide_text.is_empty() {
			buffer.append(slide_text);
			buffer.append("\n");
		}
		if slide_number < slide_texts.len() {
			buffer.append("\n");
		}
		toc_items.push(TocItem::new(first_non_empty_line(slide_text).unwrap_or(label), String::new(), slide_start));
	}
	let title = extract_title_from_path(&context.file_path);
	let mut document = Document::new().with_title(title);
	document.set_buffer(buffer);
	document.id_positions = id_positions;
	document.toc_items = toc_items;
	Ok(document)
}

fn read_ppt_document_stream(compound: &mut CompoundFile<File>) -> Result<Vec<u8>> {
	for stream_path in [
		"PowerPoint Document",
		"/PowerPoint Document",
		"PP97_DUALSTORAGE/PowerPoint Document",
		"/PP97_DUALSTORAGE/PowerPoint Document",
	] {
		if let Ok(mut stream) = compound.open_stream(stream_path) {
			let mut bytes = Vec::new();
			stream.read_to_end(&mut bytes)?;
			if !bytes.is_empty() {
				tracing::debug!(stream_path, "found powerpoint document stream");
				return Ok(bytes);
			}
		}
	}
	tracing::warn!("no powerpoint document stream found under any known path");
	// TRANSLATORS: Error shown when a legacy PPT file's OLE container has no PowerPoint Document stream
	anyhow::bail!(t("PowerPoint Document stream not found"))
}

fn collect_legacy_slide_texts(stream_data: &[u8]) -> Vec<String> {
	let mut slide_texts = Vec::new();
	walk_ppt_records(stream_data, &mut |record_type, _header_flags, payload| {
		if record_type == PPT_REC_SLIDE {
			slide_texts.push(extract_legacy_text(payload));
		}
	});
	if slide_texts.is_empty() {
		let fallback = extract_legacy_text(stream_data);
		if !fallback.is_empty() {
			tracing::warn!("no slide records found, falling back to flattened text extraction");
			slide_texts.push(fallback);
		}
	}
	slide_texts
}

fn walk_ppt_records(data: &[u8], visit: &mut impl FnMut(u16, u16, &[u8])) {
	let mut offset = 0usize;
	while offset + PPT_RECORD_HEADER_SIZE <= data.len() {
		let header_flags = u16::from_le_bytes([data[offset], data[offset + 1]]);
		let record_type = u16::from_le_bytes([data[offset + 2], data[offset + 3]]);
		let record_len = usize::try_from(u32::from_le_bytes([
			data[offset + 4],
			data[offset + 5],
			data[offset + 6],
			data[offset + 7],
		]))
		.unwrap_or(0);
		let available = data.len().saturating_sub(offset + PPT_RECORD_HEADER_SIZE);
		let payload_len = record_len.min(available);
		let payload_start = offset + PPT_RECORD_HEADER_SIZE;
		let payload_end = payload_start + payload_len;
		let payload = &data[payload_start..payload_end];
		visit(record_type, header_flags, payload);
		if is_ppt_container_record(header_flags, record_type) && !payload.is_empty() {
			walk_ppt_records(payload, visit);
		}
		let consumed = PPT_RECORD_HEADER_SIZE + payload_len;
		if consumed == 0 {
			break;
		}
		offset += consumed;
	}
}

const fn is_ppt_container_record(header_flags: u16, record_type: u16) -> bool {
	(header_flags & 0x000F) == 0x000F
		|| matches!(record_type, 1000 | 1006 | 1007 | 1008 | 1010 | 1016 | 1033 | 4057 | 4080 | 4082 | 4116)
}

fn extract_legacy_text(data: &[u8]) -> String {
	let mut text_parts = Vec::new();
	walk_ppt_records(data, &mut |record_type, _header_flags, payload| {
		let maybe_text = match record_type {
			PPT_REC_TEXT_CHARS_ATOM => parse_text_chars_atom(payload),
			PPT_REC_TEXT_BYTES_ATOM => parse_text_bytes_atom(payload),
			PPT_REC_CSTRING => parse_cstring(payload),
			_ => None,
		};
		if let Some(text) = maybe_text {
			let trimmed = text.trim();
			if !trimmed.is_empty() {
				text_parts.push(trimmed.to_string());
			}
		}
	});
	normalize_legacy_slide_text(&text_parts.join("\n"))
}

fn parse_text_chars_atom(data: &[u8]) -> Option<String> {
	if data.len() < 2 {
		return None;
	}
	let mut chars = Vec::with_capacity(data.len() / 2);
	for chunk in data.as_chunks::<2>().0 {
		let code_unit = u16::from_le_bytes([chunk[0], chunk[1]]);
		if code_unit == 0 {
			break;
		}
		if let Some(ch) = char::from_u32(u32::from(code_unit)) {
			chars.push(ch);
		}
	}
	let text: String = chars.into_iter().collect();
	let normalized = text.trim_end_matches('\r').trim_end_matches('\u{0}').trim().to_string();
	(!normalized.is_empty()).then_some(normalized)
}

fn parse_text_bytes_atom(data: &[u8]) -> Option<String> {
	if data.is_empty() {
		return None;
	}
	let text = data.iter().map(|b| char::from(*b)).collect::<String>();
	let normalized = text.trim_end_matches('\r').trim_end_matches('\u{0}').trim().to_string();
	(!normalized.is_empty()).then_some(normalized)
}

fn parse_cstring(data: &[u8]) -> Option<String> {
	let null_pos = data.iter().position(|&b| b == 0).unwrap_or(data.len());
	let text = String::from_utf8_lossy(&data[..null_pos]).trim_end_matches('\r').trim().to_string();
	if text.is_empty() || text == "___PPT10" || text == "Default Design" {
		return None;
	}
	let total_chars = text.chars().count();
	if total_chars == 0 {
		return None;
	}
	let printable_chars =
		text.chars().filter(|c| c.is_alphanumeric() || c.is_whitespace() || c.is_ascii_punctuation()).count();
	(((printable_chars as f32) / (total_chars as f32)) >= 0.8).then_some(text)
}

fn normalize_legacy_slide_text(text: &str) -> String {
	text.replace("\r\n", "\n").replace('\r', "\n").trim().to_string()
}

fn first_non_empty_line(text: &str) -> Option<String> {
	text.lines().map(str::trim).find(|line| !line.is_empty()).map(ToString::to_string)
}

#[cfg(test)]
mod tests {
	use super::{
		extract_legacy_text, normalize_legacy_slide_text, parse_cstring, parse_text_bytes_atom, parse_text_chars_atom,
	};

	#[test]
	fn parse_text_chars_atom_decodes_utf16le() {
		let atom_data = [0x48, 0x00, 0x69, 0x00, 0x00, 0x00];
		assert_eq!(parse_text_chars_atom(&atom_data), Some("Hi".to_string()));
	}

	#[test]
	fn parse_text_bytes_atom_decodes_bytes() {
		assert_eq!(parse_text_bytes_atom(b"Hello"), Some("Hello".to_string()));
	}

	#[test]
	fn parse_cstring_filters_known_noise() {
		assert_eq!(parse_cstring(b"___PPT10\0"), None);
		assert_eq!(parse_cstring(b"Default Design\0"), None);
		assert_eq!(parse_cstring(b"Agenda\0"), Some("Agenda".to_string()));
	}

	#[test]
	fn normalize_legacy_slide_text_normalizes_line_endings() {
		assert_eq!(normalize_legacy_slide_text(" a\r\nb\rc "), "a\nb\nc");
	}

	#[test]
	fn extract_legacy_text_reads_text_atoms() {
		let mut bytes = Vec::new();
		// TextBytesAtom header: [ver/inst=0][type=4008][len=5]
		bytes.extend_from_slice(&[0x00, 0x00, 0xA8, 0x0F, 0x05, 0x00, 0x00, 0x00]);
		bytes.extend_from_slice(b"Hello");
		assert_eq!(extract_legacy_text(&bytes), "Hello");
	}
}
