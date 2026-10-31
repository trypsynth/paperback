use std::{
	fs::{self, File},
	io::{Cursor, Read, Seek},
};

use anyhow::{Context, Result};
use cfb::CompoundFile;
use encoding_rs::{MACINTOSH, UTF_16LE, WINDOWS_1252};
use office_crypto::decrypt_from_file;

use crate::{
	document::{DocPos, Document, DocumentBuffer, Marker, MarkerType, ParserContext},
	parser::{
		PASSWORD_REQUIRED_ERROR_PREFIX,
		util::{
			path::extract_title_from_path,
			toc::{build_toc_from_buffer, heading_level_to_marker_type},
		},
	},
	t,
	util::encoding::{convert_to_utf8, decode_ansi},
};

const FIB_MAGIC_DOC: u16 = 0xA5EC;
const FIB_MAGIC_DOC_OLD: u16 = 0xA5DC;
/// The Word 6 and Word 95 editions for bidirectional and East Asian scripts keep their text as UTF-16.
const FIB_MAGIC_WORD6_UNICODE: u16 = 0xA697;
const FIB_MAGIC_WORD95_UNICODE: u16 = 0xA699;
const FIB_NFIB_OFFSET: usize = 0x02;
const FIB_FLAGS_OFFSET: usize = 0x0A;
const FIB_FCMIN_OFFSET: usize = 0x18;
const FIB_FCMAC_OFFSET: usize = 0x1C;
const FIB_FCCLX_OFFSET: usize = 0x1A2;
const FIB_LCBCLX_OFFSET: usize = 0x1A6;
const FIB_FLAG_ENCRYPTED: u16 = 0x0100;
const FIB_FLAG_USE_1_TABLE: u16 = 0x0200;
const FIB_NFIB_WORD97: u16 = 0x00C1;
const FIB_FLAG_COMPLEX: u16 = 0x0004;
const WINWORD1_MAGIC: u16 = 0xA59B;
const WINWORD2_MAGIC: u16 = 0xA5DB;
/// Word for the Macintosh 4.0 and 5.0, stored big-endian, so the bytes on disk are `FE 37`.
const WORD_MAC_MAGIC: u16 = 0x37FE;
/// Where its big-endian fcMin sits, with fcMac after it.
const WORD_MAC_FCMIN_OFFSET: usize = 0x14;
/// Where the clx's file offset sits in each FIB, a 16-bit length after it.
const WINWORD1_FCCLX_OFFSET: usize = 0x112;
const WINWORD2_FCCLX_OFFSET: usize = 0x11E;
/// The main text's character count; the footnotes' follows it.
const WINWORD_CCPTEXT_OFFSET: usize = 0x34;

mod lists;
mod properties;
mod styles;

use lists::DocLists;
use properties::Paragraphs;
use styles::{OUTLINE_BODY_TEXT, ParagraphStyle};

/// Level N is `U+FDD0 + N - 1`: noncharacters, which no document can contain and normalization leaves alone, so they survive until [`finish_doc`].
const LIST_ITEM_MARK: u32 = 0xFDD0;
/// Marks where a heading starts, as [`LIST_ITEM_MARK`] does a list item: level N is `U+FDE0 + N - 1`.
const HEADING_MARK: u32 = 0xFDE0;

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
	if !matches!(fib_magic, FIB_MAGIC_DOC | FIB_MAGIC_DOC_OLD | FIB_MAGIC_WORD6_UNICODE | FIB_MAGIC_WORD95_UNICODE) {
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
		return Ok(finish_doc(&text, &context.file_path));
	}
	// Word 6.0 and Word 95 write no table stream, so the Word 97 path would fail; their text is the fcMin-fcMac run of the WordDocument stream.
	let nfib = read_u16_le(&word_document, FIB_NFIB_OFFSET);
	if nfib < FIB_NFIB_WORD97 {
		return Ok(finish_doc(&extract_word6_text(&word_document), &context.file_path));
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
	Ok(finish_doc(&text, &context.file_path))
}

/// Word for Windows 1.x and 2.0, and Word for the Macintosh 4.0 and 5.0, predate the OLE container: the file opens with the FIB, and the text is 8-bit, either the fcMin-fcMac run or, after a fast save, pieces at file offsets.
pub(super) fn parse_winword_doc(context: &ParserContext, data: &[u8]) -> Result<Document> {
	let Some(text) = winword_text(data) else {
		tracing::warn!(path = %context.file_path, "word for windows 1 or 2 file has no text run");
		// TRANSLATORS: Error shown when a DOC file from Word for Windows 1.x or 2.0 has no readable text
		anyhow::bail!(t("No readable text content found"));
	};
	Ok(finish_doc(&text, &context.file_path))
}

pub(super) const fn is_winword_magic(magic: u16) -> bool {
	matches!(magic, WINWORD1_MAGIC | WINWORD2_MAGIC | WORD_MAC_MAGIC)
}

fn winword_text(data: &[u8]) -> Option<String> {
	let clx_offset = match read_u16_le(data, 0) {
		WINWORD1_MAGIC => WINWORD1_FCCLX_OFFSET,
		WINWORD2_MAGIC => WINWORD2_FCCLX_OFFSET,
		WORD_MAC_MAGIC => return word_mac_text(data),
		_ => return None,
	};
	let fast_saved = read_u16_le(data, FIB_FLAGS_OFFSET) & FIB_FLAG_COMPLEX != 0;
	// A fast-saved file whose pieces don't make sense still has its last full save's run to read.
	let pieces = fast_saved.then(|| winword_clx(data, clx_offset).and_then(|clx| winword_pieces(data, clx))).flatten();
	let mut bytes = if let Some(bytes) = pieces { bytes } else { winword_run(data)? };
	// The main text comes first, then footnotes, then headers and footers, which would only repeat as stray lines after the body.
	let main = usize::try_from(read_u32_le(data, WINWORD_CCPTEXT_OFFSET)).ok()?;
	let footnotes = usize::try_from(read_u32_le(data, WINWORD_CCPTEXT_OFFSET + 4)).ok()?;
	if main > 0 {
		bytes.truncate(main.saturating_add(footnotes));
	}
	(!bytes.is_empty()).then(|| decode_ansi(&bytes))
}

/// Its fields are big-endian and its text Mac Roman.
fn word_mac_text(data: &[u8]) -> Option<String> {
	let field =
		|at: usize| data.get(at..at + 4).map(|bytes| u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]));
	let fc_min = usize::try_from(field(WORD_MAC_FCMIN_OFFSET)?).ok()?;
	let fc_mac = usize::try_from(field(WORD_MAC_FCMIN_OFFSET + 4)?).ok()?.min(data.len());
	let run = data.get(fc_min..fc_mac).filter(|run| !run.is_empty())?;
	Some(MACINTOSH.decode(run).0.into_owned())
}

fn winword_clx(data: &[u8], clx_offset: usize) -> Option<&[u8]> {
	let fc = usize::try_from(read_u32_le(data, clx_offset)).ok()?;
	let lcb = usize::from(read_u16_le(data, clx_offset + 4));
	data.get(fc..fc.checked_add(lcb)?)
}

fn winword_run(data: &[u8]) -> Option<Vec<u8>> {
	let fc_min = usize::try_from(read_u32_le(data, FIB_FCMIN_OFFSET)).ok()?;
	let fc_mac = usize::try_from(read_u32_le(data, FIB_FCMAC_OFFSET)).ok()?.min(data.len());
	Some(data.get(fc_min..fc_mac)?.to_vec())
}

/// The clx is a run of property blocks (`1`, then a 16-bit length) ending in the piece table (`2`, then a 16-bit length): CPs, then an 8-byte descriptor per piece with its file offset at byte 2.
fn winword_pieces(data: &[u8], clx: &[u8]) -> Option<Vec<u8>> {
	let mut at = 0;
	loop {
		match *clx.get(at)? {
			1 => at += 3 + usize::from(read_u16_le(clx, at + 1)),
			2 => {
				let len = usize::from(read_u16_le(clx, at + 1));
				let table = clx.get(at + 3..at + 3 + len)?;
				let pieces = len.checked_sub(4)? / 12;
				let mut bytes = Vec::new();
				for piece in 0..pieces {
					let count = usize::try_from(
						read_u32_le(table, 4 * (piece + 1)).checked_sub(read_u32_le(table, 4 * piece))?,
					)
					.ok()?;
					let fc = usize::try_from(read_u32_le(table, 4 * (pieces + 1) + 8 * piece + 2)).ok()?;
					bytes.extend_from_slice(data.get(fc..fc.checked_add(count)?)?);
				}
				return (!bytes.is_empty()).then_some(bytes);
			}
			_ => return None,
		}
	}
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
	let formatting = Paragraphs::read(word_document, table_stream).map(|paragraphs| Formatting {
		paragraphs,
		lists: DocLists::read(word_document, table_stream),
		styles: styles::paragraph_styles(word_document, table_stream),
	});
	Some(mark_paragraphs(&units, &pieces, formatting, word_document))
}

struct Formatting {
	paragraphs: Paragraphs,
	lists: Option<DocLists>,
	styles: Vec<ParagraphStyle>,
}

struct Piece {
	cp_start: usize,
	cp_end: usize,
	fc: usize,
	ansi: bool,
}

impl Piece {
	const fn fc_of(&self, cp: usize) -> usize {
		self.fc + (cp - self.cp_start) * if self.ansi { 1 } else { 2 }
	}
}

/// A numbered heading keeps its number but is a heading rather than a list item; `units` holds one UTF-16 unit per character position.
fn mark_paragraphs(units: &[u16], pieces: &[Piece], formatting: Option<Formatting>, word_document: &[u8]) -> String {
	let Some(mut formatting) = formatting else { return String::from_utf16_lossy(units) };
	let mut out: Vec<u16> = Vec::with_capacity(units.len());
	let mut start = 0;
	while start < units.len() {
		// A paragraph runs to its paragraph mark, or to a table cell's end mark.
		let end = units[start..].iter().position(|&u| u == 0x0D || u == 0x07).map_or(units.len(), |p| start + p + 1);
		let mark = end - 1;
		let properties = pieces
			.iter()
			.find(|p| p.cp_start <= mark && mark < p.cp_end)
			.and_then(|p| u32::try_from(p.fc_of(mark)).ok())
			.and_then(|fc| formatting.paragraphs.at(word_document, fc));
		if let Some(properties) = properties {
			let style = formatting.styles.get(usize::from(properties.istd)).copied().unwrap_or_default();
			let heading = properties
				.outline
				.map_or(style.heading, |outline| (outline < OUTLINE_BODY_TEXT).then_some(outline + 1));
			let ilvl = properties.ilvl.or(style.ilvl).unwrap_or(0);
			let label = properties
				.ilfo
				.or(style.ilfo)
				.filter(|&ilfo| ilfo > 0)
				.and_then(|ilfo| formatting.lists.as_mut().and_then(|lists| lists.label(ilfo, ilvl)));
			if let Some(level) = heading {
				push_mark(&mut out, HEADING_MARK, i32::from(level));
			} else if let Some(label) = &label {
				push_mark(&mut out, LIST_ITEM_MARK, label.level);
			}
			if let Some(label) = label {
				out.extend(label.text.encode_utf16());
				out.push(u16::from(b' '));
			}
		}
		out.extend_from_slice(&units[start..end]);
		start = end;
	}
	String::from_utf16_lossy(&out)
}

fn push_mark(out: &mut Vec<u16>, base: u32, level: i32) {
	let level = u32::try_from(level.clamp(1, 9) - 1).unwrap_or(0);
	out.extend(char::from_u32(base + level).unwrap_or(' ').encode_utf16(&mut [0; 2]).iter());
}

#[cfg(test)]
fn parse_doc_clx(clx: &[u8], word_document: &[u8]) -> Option<String> {
	clx_pieces(clx, word_document).map(|(units, _)| String::from_utf16_lossy(&units))
}

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
			// Every byte of a compressed piece is one character position, and Windows-1252 maps each to one BMP character, so positions stay aligned.
			let (decoded, _, _) = WINDOWS_1252.decode(slice);
			units.extend(decoded.encode_utf16());
		} else {
			units.extend(slice.as_chunks::<2>().0.iter().map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]])));
		}
		pieces.push(Piece { cp_start, cp_end: units.len(), fc, ansi: is_ansi });
	}
	Some((units, pieces))
}

fn extract_word6_text(word_document: &[u8]) -> String {
	let fc_min = usize::try_from(read_u32_le(word_document, FIB_FCMIN_OFFSET)).unwrap_or(0);
	let fc_mac = usize::try_from(read_u32_le(word_document, FIB_FCMAC_OFFSET)).unwrap_or(0);
	if fc_mac <= fc_min || fc_min >= word_document.len() {
		// A file whose bounds make no sense still has the simple scan from 0x200 to fall back on.
		return extract_doc_text_simple(word_document);
	}
	let end = fc_mac.min(word_document.len());
	if matches!(read_u16_le(word_document, 0), FIB_MAGIC_WORD6_UNICODE | FIB_MAGIC_WORD95_UNICODE) {
		return UTF_16LE.decode_without_bom_handling(&word_document[fc_min..end]).0.into_owned();
	}
	convert_to_utf8(&word_document[fc_min..end])
}

fn finish_doc(text: &str, file_path: &str) -> Document {
	let normalized = normalize_doc_text(text);
	let mut buffer = DocumentBuffer::new();
	if !normalized.is_empty() {
		append_marked(&mut buffer, &normalized);
		if !buffer.content.ends_with('\n') {
			buffer.append("\n");
		}
	}
	let title = extract_title_from_path(file_path);
	let toc_items = build_toc_from_buffer(&buffer);
	let mut document = Document::new().with_title(title);
	document.set_buffer(buffer);
	document.toc_items = toc_items;
	document
}

fn append_marked(buffer: &mut DocumentBuffer, text: &str) {
	let mut run: Option<(DocPos, i32, DocPos)> = None; // start, items, end of last item
	let close = |buffer: &mut DocumentBuffer, run: &mut Option<(DocPos, i32, DocPos)>| {
		if let Some((start, items, end)) = run.take()
			&& end > start
		{
			buffer.add_marker(Marker::new(MarkerType::List, start).with_level(items).with_length(end - start));
		}
	};
	for line in text.split_inclusive('\n') {
		let mut chars = line.chars();
		let first = chars.next().map_or(0, u32::from);
		let level_of = |base: u32| (first.wrapping_sub(base) < 9).then(|| i32::try_from(first - base).unwrap_or(0) + 1);
		let (heading, item) = (level_of(HEADING_MARK), level_of(LIST_ITEM_MARK));
		let body: String =
			if heading.or(item).is_some() { chars.as_str() } else { line }.chars().filter(|&c| !is_mark(c)).collect();
		let start = buffer.end();
		buffer.append(&body);
		if let Some(level) = heading.filter(|_| !body.trim().is_empty()) {
			close(buffer, &mut run);
			buffer.add_marker(
				Marker::new(heading_level_to_marker_type(level), start)
					.with_text(body.trim().to_string())
					.with_level(level),
			);
			continue;
		}
		match item {
			Some(level) if !body.trim().is_empty() => {
				buffer.add_marker(
					Marker::new(MarkerType::ListItem, start).with_text(body.trim().to_string()).with_level(level),
				);
				let (_, items, end) = run.get_or_insert((start, 0, start));
				*items += 1;
				*end = buffer.end();
			}
			_ if body.trim().is_empty() => {}
			_ => close(buffer, &mut run),
		}
	}
	close(buffer, &mut run);
}

fn is_mark(c: char) -> bool {
	let c = u32::from(c);
	(LIST_ITEM_MARK..LIST_ITEM_MARK + 9).contains(&c) || (HEADING_MARK..HEADING_MARK + 9).contains(&c)
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
	// Word field codes: \u{13} begin, \u{14} separator (display text follows), \u{15} end; only the display text is kept, never the instructions.
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
	// An OLE or zip container its own parsers turned down is still binary, however much of it decodes to letters.
	let container = bytes.starts_with(&[0xD0, 0xCF, 0x11, 0xE0]) || bytes.starts_with(b"PK\x03\x04");
	let decoded = convert_to_utf8(&bytes);
	if container || !looks_like_text_content(&decoded) {
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
	// Text never holds NULs, while a binary header is full of them even when the text after it makes up most of the sample.
	if total == 0 || sample.chars().filter(|&c| c == '\0').count() > 8 {
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
