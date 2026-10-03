//! Legacy binary `.doc` parsing: the OLE compound-file container, its FIB header, the
//! piece table that locates run text, and the plain-text fallback used when neither this
//! nor `super::ooxml` can make sense of the file.

use std::{
	fs::{self, File},
	io::{Cursor, Read, Seek},
};

use anyhow::{Context, Result};
use cfb::CompoundFile;
use encoding_rs::WINDOWS_1252;
use office_crypto::decrypt_from_file;

use crate::{
	document::{Document, DocumentBuffer, Marker, MarkerType, ParserContext},
	parser::{PASSWORD_REQUIRED_ERROR_PREFIX, util::path::extract_title_from_path},
	t,
	util::encoding::convert_to_utf8,
};

const FIB_MAGIC_DOC: u16 = 0xA5EC;
const FIB_MAGIC_DOC_OLD: u16 = 0xA5DC;
const FIB_NFIB_OFFSET: usize = 0x02;
const FIB_FLAGS_OFFSET: usize = 0x0A;
const FIB_FCMIN_OFFSET: usize = 0x18;
const FIB_FCMAC_OFFSET: usize = 0x1C;
const FIB_FCCLX_OFFSET: usize = 0x1A2;
const FIB_LCBCLX_OFFSET: usize = 0x1A6;
const FIB_FLAG_ENCRYPTED: u16 = 0x0100;
const FIB_FLAG_USE_1_TABLE: u16 = 0x0200;
/// `nFib` values below this are Word 6.0 and Word 95, which predate the Word 97 file format. They
/// keep their text in the `WordDocument` stream itself and write no `0Table`/`1Table`, so the
/// Word 97 path below cannot read them. Word 97 is `nFib` 0x00C1 and up.
const FIB_NFIB_WORD97: u16 = 0x00C1;

mod lists;

use lists::DocLists;

/// Marks where a list item starts in the extracted text, through normalization, until
/// [`finish_doc`] turns it into a list-item marker. Level N (1-9) is `U+FDD0 + N - 1`:
/// noncharacters, which no document can contain and which normalization leaves alone.
const LIST_ITEM_MARK: u32 = 0xFDD0;

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
		return finish_doc(text, &context.file_path);
	}
	// Word 6.0 and Word 95 keep their text in the WordDocument stream and write no table stream, so
	// the Word 97 path below would fail at the very first step. Their text is the run between fcMin
	// and fcMac; read it straight out rather than turning a thirty-year-old file into an error.
	let nfib = read_u16_le(&word_document, FIB_NFIB_OFFSET);
	if nfib < FIB_NFIB_WORD97 {
		return finish_doc(extract_word6_text(&word_document), &context.file_path);
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
	finish_doc(text, &context.file_path)
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
	let (units, pieces) = clx_pieces(clx, word_document)?;
	let lists = DocLists::read(word_document, table_stream);
	Some(label_list_paragraphs(&units, &pieces, lists, word_document))
}

/// One piece of the piece table: the character positions it covers and where its text is.
struct Piece {
	cp_start: usize,
	cp_end: usize,
	fc: usize,
	ansi: bool,
}

impl Piece {
	/// The file offset of character position `cp`, which this piece covers.
	const fn fc_of(&self, cp: usize) -> usize {
		self.fc + (cp - self.cp_start) * if self.ansi { 1 } else { 2 }
	}
}

/// The document's text with each list paragraph's label in front of it (and a [`LIST_ITEM_MARK`]
/// before that), in the order Word shows them. `units` is the text by character position, one
/// UTF-16 unit each, as the piece table lays it out.
fn label_list_paragraphs(units: &[u16], pieces: &[Piece], lists: Option<DocLists>, word_document: &[u8]) -> String {
	let Some(mut lists) = lists else { return String::from_utf16_lossy(units) };
	let mut out: Vec<u16> = Vec::with_capacity(units.len());
	let mut start = 0;
	while start < units.len() {
		// A paragraph runs to its paragraph mark, or to a table cell's end mark.
		let end = units[start..].iter().position(|&u| u == 0x0D || u == 0x07).map_or(units.len(), |p| start + p + 1);
		let mark = end - 1;
		let label = pieces
			.iter()
			.find(|p| p.cp_start <= mark && mark < p.cp_end)
			.and_then(|p| u32::try_from(p.fc_of(mark)).ok())
			.and_then(|fc| lists.label_for_paragraph(word_document, fc));
		if let Some(label) = label {
			let level = u32::try_from(label.level.clamp(1, 9) - 1).unwrap_or(0);
			out.extend(char::from_u32(LIST_ITEM_MARK + level).unwrap_or(' ').encode_utf16(&mut [0; 2]).iter());
			out.extend(label.text.encode_utf16());
			out.push(u16::from(b' '));
		}
		out.extend_from_slice(&units[start..end]);
		start = end;
	}
	String::from_utf16_lossy(&out)
}

#[cfg(test)]
fn parse_doc_clx(clx: &[u8], word_document: &[u8]) -> Option<String> {
	clx_pieces(clx, word_document).map(|(units, _)| String::from_utf16_lossy(&units))
}

/// The piece table inside a Clx, read into text by character position and the pieces that lay it out.
fn clx_pieces(clx: &[u8], word_document: &[u8]) -> Option<(Vec<u16>, Vec<Piece>)> {
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
		return read_piece_table(&clx[offset..offset + piece_table_size], word_document);
	}
	None
}

#[cfg(test)]
fn parse_doc_piece_table(piece_table: &[u8], word_document: &[u8]) -> Option<String> {
	read_piece_table(piece_table, word_document).map(|(units, _)| String::from_utf16_lossy(&units))
}

fn read_piece_table(piece_table: &[u8], word_document: &[u8]) -> Option<(Vec<u16>, Vec<Piece>)> {
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
	let mut units: Vec<u16> = Vec::new();
	let mut pieces = Vec::with_capacity(piece_count);
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
		let cp_start = units.len();
		if is_ansi {
			// Every byte of a compressed piece is one character position, and Windows-1252 maps
			// each to one character in the Basic Multilingual Plane, so positions stay aligned.
			let (decoded, _, _) = WINDOWS_1252.decode(slice);
			units.extend(decoded.encode_utf16());
		} else {
			units.extend(slice.as_chunks::<2>().0.iter().map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]])));
		}
		pieces.push(Piece { cp_start, cp_end: units.len(), fc, ansi: is_ansi });
	}
	Some((units, pieces))
}

/// The main text of a Word 6.0 / Word 95 document, which is the byte run between `fcMin` and
/// `fcMac` in the WordDocument stream. Those bytes are 8-bit codepage text, not the UTF-16 Word 97
/// switched to, so they are decoded with the same detector the plain-text and HTML parsers use,
/// which recovers a Cyrillic or Central European document rather than turning it into question
/// marks. A fast-saved file's edits may sit outside this run, so the order can be imperfect, but
/// the words are there, which is the whole point against a hard error.
fn extract_word6_text(word_document: &[u8]) -> String {
	let fc_min = usize::try_from(read_u32_le(word_document, FIB_FCMIN_OFFSET)).unwrap_or(0);
	let fc_mac = usize::try_from(read_u32_le(word_document, FIB_FCMAC_OFFSET)).unwrap_or(0);
	if fc_mac <= fc_min || fc_min >= word_document.len() {
		// A file whose bounds make no sense still has the simple scan from 0x200 to fall back on.
		return extract_doc_text_simple(word_document);
	}
	let end = fc_mac.min(word_document.len());
	convert_to_utf8(&word_document[fc_min..end])
}

/// Builds the finished document from extracted DOC text: normalize field codes, drop it into a
/// buffer with a trailing newline, and title it from the path. Shared by the extraction paths.
fn finish_doc(text: String, file_path: &str) -> Result<Document> {
	let normalized = normalize_doc_text(&text);
	let mut buffer = DocumentBuffer::new();
	if !normalized.is_empty() {
		append_with_list_items(&mut buffer, &normalized);
		if !buffer.content.ends_with('\n') {
			buffer.append("\n");
		}
	}
	let title = extract_title_from_path(file_path);
	let mut document = Document::new().with_title(title);
	document.set_buffer(buffer);
	Ok(document)
}

/// Appends the normalized text, turning each [`LIST_ITEM_MARK`] at the start of a line into a
/// list-item marker, and each run of list items into one list the reader can step over, as HTML
/// and `.docx` lists are.
fn append_with_list_items(buffer: &mut DocumentBuffer, text: &str) {
	let mut run: Option<(usize, i32, usize)> = None; // start, items, end of last item
	let close = |buffer: &mut DocumentBuffer, run: &mut Option<(usize, i32, usize)>| {
		if let Some((start, items, end)) = run.take()
			&& end > start
		{
			buffer.add_marker(Marker::new(MarkerType::List, start).with_level(items).with_length(end - start));
		}
	};
	for line in text.split_inclusive('\n') {
		let mut chars = line.chars();
		let level = chars
			.next()
			.and_then(|c| (u32::from(c).wrapping_sub(LIST_ITEM_MARK) < 9).then(|| u32::from(c) - LIST_ITEM_MARK));
		let body: String = if level.is_some() { chars.as_str() } else { line }
			.chars()
			.filter(|&c| !(LIST_ITEM_MARK..LIST_ITEM_MARK + 9).contains(&u32::from(c)))
			.collect();
		let start = buffer.current_position();
		buffer.append(&body);
		match level {
			Some(level) if !body.trim().is_empty() => {
				let level = i32::try_from(level).unwrap_or(0) + 1;
				buffer.add_marker(
					Marker::new(MarkerType::ListItem, start).with_text(body.trim().to_string()).with_level(level),
				);
				let (_, items, end) = run.get_or_insert((start, 0, start));
				*items += 1;
				*end = buffer.current_position();
			}
			_ if body.trim().is_empty() => {}
			_ => close(buffer, &mut run),
		}
	}
	close(buffer, &mut run);
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
	let bytes = fs::read(&context.file_path)
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
mod tests;
