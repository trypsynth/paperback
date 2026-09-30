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

use super::persist::Presentation;
use crate::{
	document::{Document, DocumentBuffer, Marker, MarkerType, ParserContext, TocItem},
	parser::{PASSWORD_REQUIRED_ERROR_PREFIX, util::path::extract_title_from_path},
	t,
};

const PPT_RECORD_HEADER_SIZE: usize = 8;
/// How much of a string has to be ordinary text before it is taken for text and not for the
/// binary that happens to sit next to it.
const PRINTABLE_SHARE_OF_A_STRING: f32 = 0.8;
/// What the names of the tags PowerPoint keeps its own settings under begin with, such as
/// "___PPT9" and "___PPT10". They are not text of the presentation.
const PROG_TAG_NAME_PREFIX: &str = "___PPT";
const PPT_REC_SLIDE: u16 = 1006;
const PPT_REC_TEXT_CHARS_ATOM: u16 = 4000;
const PPT_REC_TEXT_BYTES_ATOM: u16 = 4008;
const PPT_REC_CSTRING: u16 = 4026;
/// Names the slide whose text follows it in the document's list of slides.
const PPT_REC_SLIDE_PERSIST_ATOM: u16 = 1011;
/// The document's lists of text: the slides' own, then the masters', then the notes'.
const PPT_REC_SLIDE_LIST_WITH_TEXT: u16 = 4080;

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
	let (ppt_document_stream, current_user) = match decrypted_compound.as_mut() {
		Some(compound) => (read_ppt_document_stream(compound), read_current_user_stream(compound)),
		None => (read_ppt_document_stream(&mut compound), read_current_user_stream(&mut compound)),
	};
	let ppt_document_stream = ppt_document_stream
		.inspect_err(
			|e| tracing::warn!(path = %context.file_path, error = %e, "failed to read powerpoint document stream"),
		)
		.with_context(|| format!("Failed to read PowerPoint Document stream from '{}'", context.file_path))?;
	let slide_texts = collect_legacy_slide_texts(&ppt_document_stream, &current_user);
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

/// The Current User stream, which points at the save that is the presentation. An empty
/// result is not an error: a file without one is read as it lies.
fn read_current_user_stream<F: Read + std::io::Seek>(compound: &mut CompoundFile<F>) -> Vec<u8> {
	for stream_path in ["Current User", "/Current User", "PP97_DUALSTORAGE/Current User"] {
		if let Ok(mut stream) = compound.open_stream(stream_path) {
			let mut bytes = Vec::new();
			if stream.read_to_end(&mut bytes).is_ok() && !bytes.is_empty() {
				return bytes;
			}
		}
	}
	Vec::new()
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

/// The text of every slide, in the order the presentation lists them.
///
/// The text of a slide is not always kept in the slide. A presentation also carries a list of
/// its slides in the document itself, the SlideListWithText, and that is where PowerPoint puts
/// the outline text of a slide whose own record holds none. A file whose slides are all empty
/// reads as a file of empty slides unless both places are looked at.
fn collect_legacy_slide_texts(stream_data: &[u8], current_user: &[u8]) -> Vec<String> {
	if let Some(presentation) = Presentation::read(stream_data, current_user) {
		let outlines = presentation.document().map(slide_outline_texts).unwrap_or_default();
		// The list in the document is the order the slides are shown in, which is not the order
		// they were saved in. A presentation with no such list falls back to its slide objects,
		// which are at least all of the slides and usually in the right order.
		let listed: Vec<(u32, String)> =
			outlines.into_iter().filter(|(id, _)| presentation.object(*id).is_some()).collect();
		let slides: Vec<(u32, String)> = if listed.is_empty() {
			presentation.objects_of_type(PPT_REC_SLIDE).into_iter().map(|(id, _)| (id, String::new())).collect()
		} else {
			listed
		};
		tracing::debug!(slides = slides.len(), "read the slides the presentation lists");
		return slides
			.into_iter()
			.map(|(id, outline)| {
				let own = presentation.object(id).map(extract_legacy_text).unwrap_or_default();
				if own.is_empty() { outline } else { own }
			})
			.collect();
	}
	// A file that does not lead anywhere is read as it lies, which is what this always did.
	tracing::debug!("no persist directory, reading the stream as it lies");
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

/// The outline text the document keeps for each slide, in the order the document lists them.
///
/// The list holds an atom naming a slide followed by that slide's text, then the next slide,
/// and so on, so the atoms are read in order and each one belongs to the slide named most
/// recently.
/// <https://docs.microsoft.com/en-us/openspecs/office_file_formats/ms-ppt/1fc22d56-28f9-4818-bd45-67c2bf721ccf>
fn slide_outline_texts(document: &[u8]) -> Vec<(u32, String)> {
	let Some(list) = slide_list_with_text(document) else {
		return Vec::new();
	};
	let mut slides: Vec<(u32, Vec<String>)> = Vec::new();
	walk_ppt_records(list, &mut |record_type, _header_flags, payload| match record_type {
		PPT_REC_SLIDE_PERSIST_ATOM => {
			if let Some(bytes) = payload.get(..4) {
				slides.push((u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]), Vec::new()));
			}
		}
		PPT_REC_TEXT_CHARS_ATOM | PPT_REC_TEXT_BYTES_ATOM => {
			let text = if record_type == PPT_REC_TEXT_CHARS_ATOM {
				parse_text_chars_atom(payload)
			} else {
				parse_text_bytes_atom(payload)
			};
			if let (Some((_, parts)), Some(text)) = (slides.last_mut(), text) {
				let trimmed = text.trim();
				if !trimmed.is_empty() {
					parts.push(trimmed.to_string());
				}
			}
		}
		_ => {}
	});
	slides.into_iter().map(|(id, parts)| (id, normalize_legacy_slide_text(&parts.join("\n")))).collect()
}

/// The document's list of slide text, which is the one whose instance is zero. The other two
/// lists hold the text of the masters and of the notes.
fn slide_list_with_text(document: &[u8]) -> Option<&[u8]> {
	let mut offset = PPT_RECORD_HEADER_SIZE;
	while offset + PPT_RECORD_HEADER_SIZE <= document.len() {
		let header_flags = u16::from_le_bytes([document[offset], document[offset + 1]]);
		let record_type = u16::from_le_bytes([document[offset + 2], document[offset + 3]]);
		let record_len = usize::try_from(u32::from_le_bytes([
			document[offset + 4],
			document[offset + 5],
			document[offset + 6],
			document[offset + 7],
		]))
		.unwrap_or(0);
		let payload_start = offset + PPT_RECORD_HEADER_SIZE;
		let payload_end = (payload_start + record_len).min(document.len());
		if record_type == PPT_REC_SLIDE_LIST_WITH_TEXT && (header_flags >> 4) == 0 {
			return Some(&document[payload_start..payload_end]);
		}
		if payload_end <= offset {
			break;
		}
		offset = payload_end;
	}
	None
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
	let text = parse_text_chars_atom(data)?.trim_end_matches('\r').trim().to_string();
	if text.is_empty() || text.starts_with(PROG_TAG_NAME_PREFIX) || text == "Default Design" {
		return None;
	}
	let total_chars = text.chars().count();
	let printable_chars =
		text.chars().filter(|c| c.is_alphanumeric() || c.is_whitespace() || c.is_ascii_punctuation()).count();
	(((printable_chars as f32) / (total_chars as f32)) >= PRINTABLE_SHARE_OF_A_STRING).then_some(text)
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
		HEADER_TOKEN_ENCRYPTED, PPT_REC_SLIDE, PPT_REC_SLIDE_LIST_WITH_TEXT, PPT_RECORD_HEADER_SIZE,
		collect_legacy_slide_texts, extract_legacy_text, normalize_legacy_slide_text, parse_cstring,
		parse_text_bytes_atom, parse_text_chars_atom,
	};
	use crate::{
		document::ParserContext,
		parser::{PASSWORD_REQUIRED_ERROR_PREFIX, powerpoint::legacy::parse_legacy_ppt},
		util::test_support::TempDir,
	};

	fn utf16(text: &str) -> Vec<u8> {
		text.encode_utf16().flat_map(u16::to_le_bytes).collect()
	}

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

	/// A record: its header, then whatever it holds.
	fn record(version_and_instance: u16, record_type: u16, payload: &[u8]) -> Vec<u8> {
		let mut bytes = Vec::with_capacity(PPT_RECORD_HEADER_SIZE + payload.len());
		bytes.extend_from_slice(&version_and_instance.to_le_bytes());
		bytes.extend_from_slice(&record_type.to_le_bytes());
		bytes.extend_from_slice(&u32::try_from(payload.len()).expect("record fits").to_le_bytes());
		bytes.extend_from_slice(payload);
		bytes
	}

	/// A CurrentUserAtom naming the save at `offset_to_current_edit`.
	fn current_user_stream(offset_to_current_edit: u32) -> Vec<u8> {
		let mut payload = Vec::new();
		payload.extend_from_slice(&0x0000_0014u32.to_le_bytes());
		payload.extend_from_slice(&0xE391_C05Fu32.to_le_bytes());
		payload.extend_from_slice(&offset_to_current_edit.to_le_bytes());
		payload.extend_from_slice(&[0u8; 8]);
		record(0, 0x0FF6, &payload)
	}

	/// A UserEditAtom naming its persist directory and the object the document is.
	fn user_edit_atom(persist_directory_at: u32, document_id: u32) -> Vec<u8> {
		let mut payload = Vec::new();
		payload.extend_from_slice(&0u32.to_le_bytes());
		payload.extend_from_slice(&0u32.to_le_bytes());
		payload.extend_from_slice(&0u32.to_le_bytes());
		payload.extend_from_slice(&persist_directory_at.to_le_bytes());
		payload.extend_from_slice(&document_id.to_le_bytes());
		payload.extend_from_slice(&[0u8; 8]);
		record(0, 0x0FF5, &payload)
	}

	/// A PersistDirectoryAtom, one entry for each object.
	fn persist_directory(entries: &[(u32, u32)]) -> Vec<u8> {
		let mut payload = Vec::new();
		for (id, offset) in entries {
			payload.extend_from_slice(&((1u32 << 20) | *id).to_le_bytes());
			payload.extend_from_slice(&offset.to_le_bytes());
		}
		record(0, 0x1772, &payload)
	}

	/// A slide of the document's list of slide text: the atom naming the slide, then its text.
	fn listed_slide(slide_id: u32, text: &str) -> Vec<u8> {
		let mut payload = Vec::new();
		payload.extend_from_slice(&slide_id.to_le_bytes());
		payload.extend_from_slice(&[0u8; 16]);
		let mut bytes = record(0, 1011, &payload);
		bytes.extend_from_slice(&record(0, 4008, text.as_bytes()));
		bytes
	}

	/// The text of a slide is not always in the slide. Reading only the slide containers leaves
	/// a presentation that keeps its text in the document's list looking empty, which is what
	/// happened to the presentations in issue 839.
	#[test]
	fn slide_text_comes_from_the_document_list_when_the_slide_holds_none() {
		let mut stream = Vec::new();
		// An empty slide, and a second one, laid down before the document they belong to.
		let first_slide_at = 0u32;
		stream.extend_from_slice(&record(0x000F, PPT_REC_SLIDE, &[]));
		let second_slide_at = u32::try_from(stream.len()).expect("offset fits");
		stream.extend_from_slice(&record(0x000F, PPT_REC_SLIDE, &[]));
		let document_at = u32::try_from(stream.len()).expect("offset fits");
		let mut list = listed_slide(2, "The first slide");
		list.extend_from_slice(&listed_slide(3, "The second slide"));
		let document = record(0x000F, 1000, &record(0x0000, PPT_REC_SLIDE_LIST_WITH_TEXT, &list));
		stream.extend_from_slice(&document);
		let directory_at = u32::try_from(stream.len()).expect("offset fits");
		stream.extend_from_slice(&persist_directory(&[(1, document_at), (2, first_slide_at), (3, second_slide_at)]));
		let user_edit_at = u32::try_from(stream.len()).expect("offset fits");
		stream.extend_from_slice(&user_edit_atom(directory_at, 1));
		let current_user = current_user_stream(user_edit_at);
		assert_eq!(
			collect_legacy_slide_texts(&stream, &current_user),
			vec!["The first slide".to_string(), "The second slide".to_string()]
		);
	}

	/// A presentation says which objects are its own. Reading the stream from the front instead
	/// picks up whatever else is left in it, which on these files is text from earlier saves.
	#[test]
	fn superseded_saves_are_left_out() {
		let mut stream = Vec::new();
		// An older save's slide, which the presentation no longer names.
		stream.extend_from_slice(&record(0x000F, PPT_REC_SLIDE, &record(0, 4008, b"An older save")));
		let slide_at = u32::try_from(stream.len()).expect("offset fits");
		stream.extend_from_slice(&record(0x000F, PPT_REC_SLIDE, &record(0, 4008, b"The current save")));
		let document_at = u32::try_from(stream.len()).expect("offset fits");
		stream.extend_from_slice(&record(0x000F, 1000, &[]));
		let directory_at = u32::try_from(stream.len()).expect("offset fits");
		stream.extend_from_slice(&persist_directory(&[(1, document_at), (2, slide_at)]));
		let user_edit_at = u32::try_from(stream.len()).expect("offset fits");
		stream.extend_from_slice(&user_edit_atom(directory_at, 1));
		let current_user = current_user_stream(user_edit_at);
		assert_eq!(collect_legacy_slide_texts(&stream, &current_user), vec!["The current save".to_string()]);
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
		assert_eq!(parse_cstring(&utf16("___PPT9")), None);
		assert_eq!(parse_cstring(&utf16("___PPT10")), None);
		assert_eq!(parse_cstring(&utf16("Default Design")), None);
		assert_eq!(parse_cstring(&utf16("Agenda")), Some("Agenda".to_string()));
	}

	/// A CString holds UTF-16. Reading one as bytes turns "___PPT10" into a lone underscore,
	/// which then stands on the slide in place of the text the slide really has.
	#[test]
	fn parse_cstring_reads_utf16() {
		assert_eq!(parse_cstring(&utf16("Agenda")), Some("Agenda".to_string()));
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
