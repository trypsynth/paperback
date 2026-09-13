//! Legacy binary `.ppt` parsing: walks the OLE `PowerPoint Document` stream's record tree
//! directly, since there is no XML to hand off to a library.

use std::{
	collections::HashMap,
	fs::File,
	io::{Cursor, Read},
};

use anyhow::{Context, Result};
use cfb::CompoundFile;
use office_crypto::decrypt_from_file;

use crate::{
	document::{Document, DocumentBuffer, Marker, MarkerType, ParserContext, TocItem},
	parser::{PASSWORD_REQUIRED_ERROR_PREFIX, util::path::extract_title_from_path},
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
	// A password-protected presentation is decrypted first, and the records are read out of
	// what comes back rather than off the disk.
	let decrypted = decrypt_if_encrypted(&mut compound, context)?;
	let mut decrypted_compound = match decrypted {
		Some(bytes) => Some(
			CompoundFile::open(Cursor::new(bytes))
				.with_context(|| format!("Failed to parse decrypted PPT file '{}'", context.file_path))?,
		),
		None => None,
	};
	let ppt_document_stream = match decrypted_compound.as_mut() {
		Some(compound) => read_ppt_document_stream(compound),
		None => read_ppt_document_stream(&mut compound),
	}
	.inspect_err(|e| tracing::warn!(path = %context.file_path, error = %e, "failed to read powerpoint document stream"))
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

/// Offset of the CurrentUserAtom's headerToken, past the record header and the size field.
const HEADER_TOKEN_OFFSET: usize = 12;

/// The token a presentation carries when it is password-protected. A legacy file says so here
/// rather than in a stream of its own, which is what an OOXML file does.
/// <https://docs.microsoft.com/en-us/openspecs/office_file_formats/ms-ppt/940d5700-e4d7-4fc0-ab48-fed5dbc48bc1>
const HEADER_TOKEN_ENCRYPTED: u32 = 0xF3D1_C4DF;

/// Decrypts a password-protected presentation, or reports that there is nothing to decrypt.
///
/// Returns the whole file rather than the one stream, since decryption rewrites several of
/// them and the caller reads the records out of the result.
fn decrypt_if_encrypted(compound: &mut CompoundFile<File>, context: &ParserContext) -> Result<Option<Vec<u8>>> {
	let Ok(mut stream) = compound.open_stream("Current User").or_else(|_| compound.open_stream("/Current User")) else {
		return Ok(None);
	};
	let mut current_user = Vec::new();
	stream.read_to_end(&mut current_user)?;
	let Some(token) = current_user.get(HEADER_TOKEN_OFFSET..HEADER_TOKEN_OFFSET + 4) else {
		return Ok(None);
	};
	if u32::from_le_bytes([token[0], token[1], token[2], token[3]]) != HEADER_TOKEN_ENCRYPTED {
		return Ok(None);
	}
	let Some(password) = context.password.as_deref() else {
		tracing::debug!(path = %context.file_path, "legacy ppt file is encrypted, asking for a password");
		// TRANSLATORS: Error detail shown when an encrypted PowerPoint file needs a password (the internal sentinel prefix before it is not translated)
		anyhow::bail!("{PASSWORD_REQUIRED_ERROR_PREFIX} {}", t("File is encrypted and requires a password"));
	};
	let decrypted = decrypt_from_file(&context.file_path, password)
		// TRANSLATORS: Error shown when decrypting an encrypted Office file fails; {} is the underlying error
		.map_err(|e| anyhow::anyhow!(t("Decryption failed (wrong password?): {}").replace("{}", &e.to_string())))?;
	tracing::debug!(path = %context.file_path, bytes = decrypted.len(), "decrypted legacy ppt file");
	Ok(Some(decrypted))
}

fn read_ppt_document_stream<F: Read + std::io::Seek>(compound: &mut CompoundFile<F>) -> Result<Vec<u8>> {
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
	use std::io::Write;

	use super::{
		HEADER_TOKEN_ENCRYPTED, extract_legacy_text, normalize_legacy_slide_text, parse_cstring, parse_text_bytes_atom,
		parse_text_chars_atom,
	};
	use crate::{
		document::ParserContext,
		parser::{PASSWORD_REQUIRED_ERROR_PREFIX, powerpoint::legacy::parse_legacy_ppt},
		util::test_support::TempDir,
	};

	/// Writes a presentation carrying nothing but the CurrentUserAtom that says whether it is
	/// encrypted, which is the only part of the file the check reads.
	fn presentation_with_token(dir: &TempDir, name: &str, token: u32) -> String {
		let mut current_user = Vec::new();
		// A CurrentUserAtom: its record header, its size, the token, and the offset after it.
		current_user.extend_from_slice(&0x0000u16.to_le_bytes());
		current_user.extend_from_slice(&0x0FF6u16.to_le_bytes());
		current_user.extend_from_slice(&0x0000_0014u32.to_le_bytes());
		current_user.extend_from_slice(&0x0000_0014u32.to_le_bytes());
		current_user.extend_from_slice(&token.to_le_bytes());
		current_user.extend_from_slice(&0u32.to_le_bytes());
		let path = dir.join_str(name);
		let mut compound = cfb::create(&path).expect("create compound file");
		compound.create_stream("/Current User").expect("stream").write_all(&current_user).expect("write");
		compound.create_stream("/PowerPoint Document").expect("stream").write_all(&[0u8; 8]).expect("write");
		compound.flush().expect("flush");
		path
	}

	/// A legacy presentation says it is encrypted in its CurrentUserAtom, where an OOXML file
	/// has a stream of its own for it. Opening one without a password has to ask for the
	/// password rather than report a broken file.
	#[test]
	fn an_encrypted_presentation_asks_for_a_password() {
		let dir = TempDir::new("ppt-encrypted");
		let path = presentation_with_token(&dir, "protected.ppt", HEADER_TOKEN_ENCRYPTED);
		let error = parse_legacy_ppt(&ParserContext::new(path)).expect_err("a password is needed");
		assert!(error.to_string().starts_with(PASSWORD_REQUIRED_ERROR_PREFIX), "{error}");
	}

	/// A presentation that is not encrypted is read as it stands, and fails later for having no
	/// slides rather than for wanting a password.
	#[test]
	fn a_plain_presentation_is_not_taken_for_an_encrypted_one() {
		let dir = TempDir::new("ppt-plain");
		let path = presentation_with_token(&dir, "plain.ppt", 0xE391_C05F);
		let error = parse_legacy_ppt(&ParserContext::new(path)).expect_err("no slides in it");
		assert!(!error.to_string().starts_with(PASSWORD_REQUIRED_ERROR_PREFIX), "{error}");
	}

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
