//! Legacy binary `.doc` parsing: the OLE compound-file container, its FIB header, the
//! piece table that locates run text, and the plain-text fallback used when neither this
//! nor `super::ooxml` can make sense of the file.

use std::{
	fs::File,
	io::{Cursor, Read, Seek},
};

use anyhow::{Context, Result};
use cfb::CompoundFile;
use encoding_rs::WINDOWS_1252;
use office_crypto::decrypt_from_file;

use crate::{
	document::{Document, DocumentBuffer, ParserContext},
	parser::{PASSWORD_REQUIRED_ERROR_PREFIX, util::path::extract_title_from_path},
	t,
	util::encoding::convert_to_utf8,
};

const FIB_MAGIC_DOC: u16 = 0xA5EC;
const FIB_MAGIC_DOC_OLD: u16 = 0xA5DC;
const FIB_FLAGS_OFFSET: usize = 0x0A;
const FIB_FCCLX_OFFSET: usize = 0x1A2;
const FIB_LCBCLX_OFFSET: usize = 0x1A6;
const FIB_FLAG_ENCRYPTED: u16 = 0x0100;
const FIB_FLAG_USE_1_TABLE: u16 = 0x0200;

pub(super) fn parse_legacy_doc(context: &ParserContext) -> Result<Document> {
	let file =
		File::open(&context.file_path).with_context(|| format!("Failed to open DOC file '{}'", context.file_path))?;
	let mut compound =
		CompoundFile::open(file).with_context(|| format!("Failed to parse OLE container '{}'", context.file_path))?;
	let word_document =
		read_stream(&mut compound, "WordDocument").or_else(|_| read_stream(&mut compound, "/WordDocument"))?;
	if word_document.len() < FIB_LCBCLX_OFFSET + 4 {
		tracing::warn!(path = %context.file_path, "doc file is missing required fib fields");
		// TRANSLATORS: Error shown when a legacy DOC file is missing required header fields
		anyhow::bail!(t("DOC file is missing required FIB fields"));
	}
	let fib_magic = read_u16_le(&word_document, 0);
	if fib_magic != FIB_MAGIC_DOC && fib_magic != FIB_MAGIC_DOC_OLD {
		tracing::warn!(path = %context.file_path, magic = fib_magic, "doc file has an invalid fib magic number");
		// TRANSLATORS: Error shown when a legacy DOC file's header signature is invalid
		anyhow::bail!(t("Not a valid DOC file (invalid FIB magic)"));
	}
	let fib_flags = read_u16_le(&word_document, FIB_FLAGS_OFFSET);
	if (fib_flags & FIB_FLAG_ENCRYPTED) != 0 {
		let Some(password) = context.password.as_deref() else {
			tracing::debug!(path = %context.file_path, "encrypted doc file requires a password");
			// TRANSLATORS: Error detail shown when an encrypted legacy DOC file needs a password (the internal sentinel prefix before it is not translated)
			anyhow::bail!("{PASSWORD_REQUIRED_ERROR_PREFIX} {}", t("DOC file is encrypted and requires a password"));
		};
		let decrypted = decrypt_from_file(&context.file_path, password).map_err(|e| {
			tracing::warn!(path = %context.file_path, error = %e, "doc decryption failed");
			// TRANSLATORS: Error detail shown when decrypting a legacy DOC file fails (the internal sentinel prefix before it is not translated); {} is the underlying error
			let msg = t("DOC decryption failed (wrong password?): {}").replace("{}", &e.to_string());
			anyhow::anyhow!("{PASSWORD_REQUIRED_ERROR_PREFIX} {msg}")
		})?;
		let mut dec_compound =
			CompoundFile::open(Cursor::new(decrypted)).context("Decrypted DOC data is not a valid compound file")?;
		let word_document = read_stream(&mut dec_compound, "WordDocument")
			.or_else(|_| read_stream(&mut dec_compound, "/WordDocument"))?;
		let fib_flags2 = read_u16_le(&word_document, FIB_FLAGS_OFFSET);
		let table_stream_name2 = if (fib_flags2 & FIB_FLAG_USE_1_TABLE) != 0 { "1Table" } else { "0Table" };
		let table_stream2 = read_stream(&mut dec_compound, table_stream_name2)
			.or_else(|_| read_stream(&mut dec_compound, &format!("/{table_stream_name2}")))?;
		let piece_table_text = extract_doc_text_from_piece_table(&word_document, &table_stream2);
		if piece_table_text.is_none() {
			tracing::warn!(path = %context.file_path, "piece table extraction failed, using simple fallback extraction");
		}
		let mut text = piece_table_text.unwrap_or_else(|| extract_doc_text_simple(&word_document));
		if text.trim().is_empty() {
			tracing::warn!(path = %context.file_path, "doc text extraction produced no content, simple fallback extraction also found nothing");
			text = extract_doc_text_simple(&word_document);
		}
		let normalized = normalize_doc_text(&text);
		let mut buffer = DocumentBuffer::new();
		if !normalized.is_empty() {
			buffer.append(&normalized);
			if !buffer.content.ends_with('\n') {
				buffer.append("\n");
			}
		}
		let title = extract_title_from_path(&context.file_path);
		let mut document = Document::new().with_title(title);
		document.set_buffer(buffer);
		return Ok(document);
	}
	let table_stream_name = if (fib_flags & FIB_FLAG_USE_1_TABLE) != 0 { "1Table" } else { "0Table" };
	let table_stream = read_stream(&mut compound, table_stream_name)
		.or_else(|_| read_stream(&mut compound, &format!("/{table_stream_name}")))
		.with_context(|| format!("Failed to open DOC table stream '{table_stream_name}'"))?;
	let piece_table_text = extract_doc_text_from_piece_table(&word_document, &table_stream);
	if piece_table_text.is_none() {
		tracing::warn!(path = %context.file_path, "piece table extraction failed, using simple fallback extraction");
	}
	let mut text = piece_table_text.unwrap_or_else(|| extract_doc_text_simple(&word_document));
	if text.trim().is_empty() {
		tracing::warn!(path = %context.file_path, "doc text extraction produced no content, simple fallback extraction also found nothing");
		text = extract_doc_text_simple(&word_document);
	}
	let normalized = normalize_doc_text(&text);
	let mut buffer = DocumentBuffer::new();
	if !normalized.is_empty() {
		buffer.append(&normalized);
		if !buffer.content.ends_with('\n') {
			buffer.append("\n");
		}
	}
	let title = extract_title_from_path(&context.file_path);
	let mut document = Document::new().with_title(title);
	document.set_buffer(buffer);
	Ok(document)
}

fn read_stream<R: Read + Seek>(compound: &mut CompoundFile<R>, path: &str) -> Result<Vec<u8>> {
	let mut stream = compound.open_stream(path).with_context(|| format!("Stream not found: {path}"))?;
	let mut bytes = Vec::new();
	stream.read_to_end(&mut bytes)?;
	Ok(bytes)
}

fn extract_doc_text_from_piece_table(word_document: &[u8], table_stream: &[u8]) -> Option<String> {
	let fc_clx = usize::try_from(read_u32_le(word_document, FIB_FCCLX_OFFSET)).ok()?;
	let lcb_clx = usize::try_from(read_u32_le(word_document, FIB_LCBCLX_OFFSET)).ok()?;
	if lcb_clx == 0 || fc_clx.checked_add(lcb_clx)? > table_stream.len() {
		return None;
	}
	let clx = &table_stream[fc_clx..fc_clx + lcb_clx];
	parse_doc_clx(clx, word_document)
}

fn parse_doc_clx(clx: &[u8], word_document: &[u8]) -> Option<String> {
	let mut offset = 0usize;
	while offset < clx.len() {
		let section = clx[offset];
		offset += 1;
		if section == 0x01 {
			if offset + 2 > clx.len() {
				return None;
			}
			let size = usize::from(read_u16_le(clx, offset));
			offset = offset.checked_add(2 + size)?;
			continue;
		}
		if section != 0x02 {
			break;
		}
		if offset + 4 > clx.len() {
			return None;
		}
		let piece_table_size = usize::try_from(read_u32_le(clx, offset)).ok()?;
		offset += 4;
		if offset.checked_add(piece_table_size)? > clx.len() {
			return None;
		}
		return parse_doc_piece_table(&clx[offset..offset + piece_table_size], word_document);
	}
	None
}

fn parse_doc_piece_table(piece_table: &[u8], word_document: &[u8]) -> Option<String> {
	if piece_table.len() < 4 {
		return None;
	}
	let piece_count = (piece_table.len().saturating_sub(4)) / 12;
	if piece_count == 0 {
		return None;
	}
	let cp_table_len = (piece_count + 1) * 4;
	if cp_table_len > piece_table.len() {
		return None;
	}
	let mut cps = Vec::with_capacity(piece_count + 1);
	for i in 0..=piece_count {
		cps.push(read_u32_le(piece_table, i * 4));
	}
	let mut text = String::new();
	for i in 0..piece_count {
		let pcd_offset = cp_table_len + (i * 8);
		if pcd_offset + 8 > piece_table.len() {
			break;
		}
		let cp_start = cps[i];
		let cp_end = cps[i + 1];
		if cp_end <= cp_start {
			continue;
		}
		let char_count = usize::try_from(cp_end - cp_start).ok()?;
		let mut fc_raw = read_u32_le(piece_table, pcd_offset + 2);
		let is_ansi = (fc_raw & 0x4000_0000) != 0;
		fc_raw &= 0x3FFF_FFFF;
		if is_ansi {
			fc_raw /= 2;
		}
		let fc = usize::try_from(fc_raw).ok()?;
		let byte_count = if is_ansi { char_count } else { char_count.saturating_mul(2) };
		if fc >= word_document.len() {
			continue;
		}
		let end = fc.saturating_add(byte_count).min(word_document.len());
		let slice = &word_document[fc..end];
		if is_ansi {
			let (decoded, _, _) = WINDOWS_1252.decode(slice);
			text.push_str(decoded.as_ref());
		} else {
			let utf16: Vec<u16> = slice.chunks_exact(2).map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]])).collect();
			text.push_str(&String::from_utf16_lossy(&utf16));
		}
	}
	Some(text)
}

fn extract_doc_text_simple(word_document: &[u8]) -> String {
	if word_document.len() <= 0x200 {
		return String::new();
	}
	let text_start = &word_document[0x200..];
	let text_end = text_start.iter().position(|&b| b == 0).unwrap_or(text_start.len());
	let (decoded, _, _) = WINDOWS_1252.decode(&text_start[..text_end]);
	decoded.to_string()
}

fn normalize_doc_text(text: &str) -> String {
	// Strip Word field codes: \u{13}=begin, \u{14}=separator (display text follows), \u{15}=end.
	// Keep only text outside fields or in the display portion of a field; discard instructions.
	let stripped = {
		let mut out = String::with_capacity(text.len());
		// Each entry on the stack is true when we have passed the \u{14} separator at that depth.
		let mut field_stack: Vec<bool> = Vec::new();
		for ch in text.chars() {
			match ch {
				'\u{13}' => field_stack.push(false),
				'\u{14}' => {
					if let Some(top) = field_stack.last_mut() {
						*top = true;
					}
				}
				'\u{15}' => {
					field_stack.pop();
				}
				_ => {
					if field_stack.is_empty() || field_stack.iter().all(|&past_sep| past_sep) {
						out.push(ch);
					}
				}
			}
		}
		out
	};
	let normalized = stripped.replace("\r\n", "\n").replace('\r', "\n");
	let mut out = String::with_capacity(normalized.len());
	let mut previous_was_newline = false;
	let mut newline_run = 0usize;
	for ch in normalized.chars() {
		if ch == '\n' {
			newline_run += 1;
			if newline_run > 2 {
				continue;
			}
			previous_was_newline = true;
			out.push(ch);
			continue;
		}
		newline_run = 0;
		if ch.is_control() && ch != '\t' {
			continue;
		}
		if previous_was_newline && ch == ' ' {
			continue;
		}
		previous_was_newline = false;
		out.push(ch);
	}
	out.trim().to_string()
}

pub(super) fn parse_text_like_doc(context: &ParserContext) -> Result<Document> {
	let bytes = std::fs::read(&context.file_path)
		.with_context(|| format!("Failed to read potential text DOC '{}'", context.file_path))?;
	let decoded = convert_to_utf8(&bytes);
	if !looks_like_text_content(&decoded) {
		tracing::warn!(path = %context.file_path, "doc fallback content does not look like plain text");
		// TRANSLATORS: Error shown when a DOC file's fallback content doesn't look like plain text
		anyhow::bail!(t("File content does not look like plain text"));
	}
	let normalized = normalize_doc_text(&decoded);
	if normalized.trim().is_empty() {
		tracing::warn!(path = %context.file_path, "doc fallback text-like content normalized to empty text");
		// TRANSLATORS: Error shown when a DOC file's fallback content has no readable text
		anyhow::bail!(t("No readable text content found"));
	}
	let mut buffer = DocumentBuffer::new();
	buffer.append(&normalized);
	if !buffer.content.ends_with('\n') {
		buffer.append("\n");
	}
	let title = extract_title_from_path(&context.file_path);
	let mut document = Document::new().with_title(title);
	document.set_buffer(buffer);
	Ok(document)
}

fn looks_like_text_content(content: &str) -> bool {
	let sample: String = content.chars().take(4096).collect();
	if sample.trim().is_empty() {
		return false;
	}
	let total = sample.chars().count();
	if total == 0 {
		return false;
	}
	let printable = sample.chars().filter(|c| !c.is_control() || *c == '\n' || *c == '\r' || *c == '\t').count();
	(printable as f32) / (total as f32) >= 0.85
}

fn read_u16_le(data: &[u8], offset: usize) -> u16 {
	if offset + 2 > data.len() {
		return 0;
	}
	u16::from_le_bytes([data[offset], data[offset + 1]])
}

fn read_u32_le(data: &[u8], offset: usize) -> u32 {
	if offset + 4 > data.len() {
		return 0;
	}
	u32::from_le_bytes([data[offset], data[offset + 1], data[offset + 2], data[offset + 3]])
}

#[cfg(test)]
mod tests {
	use super::{looks_like_text_content, normalize_doc_text, parse_doc_clx, parse_doc_piece_table};

	#[test]
	fn parse_doc_piece_table_extracts_ansi_text() {
		let mut word_document = vec![0u8; 64];
		word_document[16..21].copy_from_slice(b"Hello");
		let mut piece_table = Vec::new();
		piece_table.extend_from_slice(&0u32.to_le_bytes());
		piece_table.extend_from_slice(&5u32.to_le_bytes());
		piece_table.extend_from_slice(&0u16.to_le_bytes());
		let fc_raw = 0x4000_0000u32 | 32u32;
		piece_table.extend_from_slice(&fc_raw.to_le_bytes());
		piece_table.extend_from_slice(&0u16.to_le_bytes());
		let text = parse_doc_piece_table(&piece_table, &word_document).expect("text");
		assert_eq!(text, "Hello");
	}

	#[test]
	fn parse_doc_clx_extracts_piece_table_text() {
		let mut word_document = vec![0u8; 64];
		word_document[16..21].copy_from_slice(b"Hello");
		let mut piece_table = Vec::new();
		piece_table.extend_from_slice(&0u32.to_le_bytes());
		piece_table.extend_from_slice(&5u32.to_le_bytes());
		piece_table.extend_from_slice(&0u16.to_le_bytes());
		let fc_raw = 0x4000_0000u32 | 32u32;
		piece_table.extend_from_slice(&fc_raw.to_le_bytes());
		piece_table.extend_from_slice(&0u16.to_le_bytes());
		let mut clx = Vec::new();
		clx.push(0x02);
		clx.extend_from_slice(&(piece_table.len() as u32).to_le_bytes());
		clx.extend_from_slice(&piece_table);
		let text = parse_doc_clx(&clx, &word_document).expect("clx text");
		assert_eq!(text, "Hello");
	}

	#[test]
	fn normalize_doc_text_cleans_control_markers() {
		let text = "A\r\nB\u{13}\u{14}C\u{15}\n\n\nD";
		assert_eq!(normalize_doc_text(text), "A\nBC\n\nD");
	}

	#[test]
	fn looks_like_text_content_detects_textual_data() {
		assert!(looks_like_text_content("Manual Title\nLine 2\nLine 3"));
		assert!(!looks_like_text_content("\u{0}\u{1}\u{2}\u{3}\u{4}\u{5}"));
	}
}
