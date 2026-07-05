use std::{
	collections::HashMap,
	fs::File,
	io::{BufReader, Cursor, Read},
	path::Path,
};

use anyhow::{Context, Result};
use cfb::CompoundFile;
use encoding_rs::WINDOWS_1252;
use office_crypto::decrypt_from_file;
use roxmltree::{Document as XmlDocument, Node, NodeType};
use zip::ZipArchive;

use crate::{
	document::{Document, DocumentBuffer, Marker, MarkerType, ParserContext, ParserFlags, format_marker_types},
	parser::{
		PASSWORD_REQUIRED_ERROR_PREFIX, Parser,
		util::{
			ooxml::{collect_ooxml_run_text, read_ooxml_relationships},
			path::extract_title_from_path,
			toc::{build_toc_from_buffer, heading_level_to_marker_type},
			xml::find_child_element,
		},
	},
	types::HeadingInfo,
	util::{encoding::convert_to_utf8, text::display_len, zip::read_zip_entry_by_name},
};

const FIB_MAGIC_DOC: u16 = 0xA5EC;
const FIB_MAGIC_DOC_OLD: u16 = 0xA5DC;
const FIB_FLAGS_OFFSET: usize = 0x0A;
const FIB_FCCLX_OFFSET: usize = 0x1A2;
const FIB_LCBCLX_OFFSET: usize = 0x1A6;
const FIB_FLAG_ENCRYPTED: u16 = 0x0100;
const FIB_FLAG_USE_1_TABLE: u16 = 0x0200;

pub struct WordParser;

impl Parser for WordParser {
	fn name(&self) -> &'static str {
		"Word Documents"
	}

	fn extensions(&self) -> &[&str] {
		&["docx", "docm", "doc", "zip"]
	}

	fn supported_flags(&self) -> ParserFlags {
		ParserFlags::SUPPORTS_TOC | ParserFlags::SUPPORTS_SECTIONS
	}

	fn parse(&self, context: &ParserContext) -> Result<Document> {
		let extension = context.forced_extension.as_ref().map_or_else(
			|| {
				Path::new(&context.file_path)
					.extension()
					.and_then(|ext| ext.to_str())
					.unwrap_or_default()
					.to_ascii_lowercase()
			},
			|ext| ext.to_ascii_lowercase(),
		);
		let render_tables_inline = context.render_tables_inline;
		if extension == "zip" {
			return parse_word_zip(context, render_tables_inline);
		}
		if extension == "doc" {
			match parse_legacy_doc(context) {
				Ok(document) => return Ok(document),
				Err(legacy_err) => match parse_ooxml_doc(context, render_tables_inline) {
					Ok(document) => return Ok(document),
					Err(ooxml_err) => {
						if let Ok(document) = parse_text_like_doc(context) {
							return Ok(document);
						}
						return Err(anyhow::anyhow!(
							"Legacy DOC parse failed: {legacy_err}. OOXML fallback failed: {ooxml_err}"
						));
					}
				},
			}
		}
		parse_ooxml_doc(context, render_tables_inline)
	}
}

fn parse_word_zip(context: &ParserContext, render_tables_inline: bool) -> Result<Document> {
	let file =
		File::open(&context.file_path).with_context(|| format!("Failed to open ZIP file '{}'", context.file_path))?;
	let mut archive = ZipArchive::new(BufReader::new(file))
		.with_context(|| format!("Failed to read ZIP archive '{}'", context.file_path))?;

	let mut docx_names: Vec<String> =
		archive.file_names().filter(|name| name.to_ascii_lowercase().ends_with(".docx")).map(String::from).collect();

	if docx_names.is_empty() {
		anyhow::bail!("No readable content found in the ZIP archive");
	}

	docx_names.sort();

	let mut buffer = DocumentBuffer::new();
	let mut id_positions = HashMap::new();
	let mut headings = Vec::new();

	for docx_name in &docx_names {
		let mut inner_file_data = Vec::new();
		{
			let mut inner_file = archive.by_name(docx_name)?;
			inner_file.read_to_end(&mut inner_file_data)?;
		}

		if !buffer.content.is_empty() {
			buffer.add_marker(Marker::new(MarkerType::SectionBreak, buffer.current_position()));
		}

		let mut inner_archive = ZipArchive::new(Cursor::new(inner_file_data))
			.with_context(|| format!("Failed to parse inner DOCX '{docx_name}' as zip"))?;

		parse_ooxml_from_archive(
			&mut inner_archive,
			&mut buffer,
			&mut id_positions,
			&mut headings,
			render_tables_inline,
		)
		.with_context(|| format!("Failed to parse DOCX contents of '{docx_name}'"))?;
	}

	let title = extract_title_from_path(&context.file_path);
	let toc_items = build_toc_from_buffer(&buffer);
	let mut document = Document::new().with_title(title);
	document.set_buffer(buffer);
	document.id_positions = id_positions;
	document.toc_items = toc_items;
	Ok(document)
}

fn parse_ooxml_doc(context: &ParserContext, render_tables_inline: bool) -> Result<Document> {
	let bytes = load_ooxml_bytes(&context.file_path, context.password.as_deref())?;
	let mut archive = ZipArchive::new(Cursor::new(bytes))
		.with_context(|| format!("Failed to read DOCX as zip '{}'", context.file_path))?;
	let mut buffer = DocumentBuffer::new();
	let mut id_positions = HashMap::new();
	let mut headings = Vec::new();
	parse_ooxml_from_archive(&mut archive, &mut buffer, &mut id_positions, &mut headings, render_tables_inline)?;
	let title = extract_title_from_path(&context.file_path);
	let toc_items = build_toc_from_buffer(&buffer);
	let mut document = Document::new().with_title(title);
	document.set_buffer(buffer);
	document.id_positions = id_positions;
	document.toc_items = toc_items;
	Ok(document)
}

/// Read a DOCX/OOXML file's raw bytes, decrypting first if the file is an encrypted OLE container.
fn load_ooxml_bytes(path: &str, password: Option<&str>) -> Result<Vec<u8>> {
	match try_decrypt_office_file(path, password)? {
		Some(decrypted) => Ok(decrypted),
		None => std::fs::read(path).with_context(|| format!("Failed to read '{path}'")),
	}
}

pub fn parse_ooxml_from_archive<R: std::io::Read + std::io::Seek>(
	archive: &mut zip::ZipArchive<R>,
	buffer: &mut DocumentBuffer,
	id_positions: &mut HashMap<String, usize>,
	headings: &mut Vec<HeadingInfo>,
	render_tables_inline: bool,
) -> Result<()> {
	let style_heading_map = build_style_heading_map(archive);
	let rels = read_ooxml_relationships(archive, "word/_rels/document.xml.rels");
	let doc_content = read_zip_entry_by_name(archive, "word/document.xml")?;
	let doc_xml = XmlDocument::parse(&doc_content).context("Failed to parse word/document.xml")?;
	traverse(doc_xml.root(), buffer, headings, id_positions, &rels, &style_heading_map, render_tables_inline);
	Ok(())
}

/// Reads `word/styles.xml` and returns a map of style ID → heading level (1–9).
/// Detects headings via `<w:name w:val="heading N"/>` (the canonical semantic name
/// Word assigns regardless of locale) or a fallback `<w:outlineLvl>` in the style's pPr.
fn build_style_heading_map<R: std::io::Read + std::io::Seek>(archive: &mut zip::ZipArchive<R>) -> HashMap<String, i32> {
	let mut map = HashMap::new();
	let Ok(content) = read_zip_entry_by_name(archive, "word/styles.xml") else {
		return map;
	};
	let Ok(xml) = XmlDocument::parse(&content) else {
		return map;
	};
	for node in xml.root().descendants() {
		if node.node_type() != NodeType::Element || node.tag_name().name() != "style" {
			continue;
		}
		let Some(style_id) = node.attribute("styleId") else { continue };
		let mut heading_level: Option<i32> = None;
		for child in node.children() {
			if child.node_type() != NodeType::Element {
				continue;
			}
			match child.tag_name().name() {
				"name" => {
					if let Some(val) = child.attribute("val") {
						let lower = val.to_lowercase();
						if lower.starts_with("heading")
							&& let Some(n) = extract_number_from_string(val)
							&& n > 0 && n <= 9
						{
							heading_level = Some(n);
						}
					}
				}
				"pPr" if heading_level.is_none() => {
					for ppr_child in child.children() {
						if ppr_child.node_type() == NodeType::Element
							&& ppr_child.tag_name().name() == "outlineLvl"
							&& let Some(val) = ppr_child.attribute("val")
							&& let Ok(n) = val.parse::<i32>()
							&& (0..9).contains(&n)
						{
							heading_level = Some(n + 1);
						}
					}
				}
				_ => {}
			}
		}
		if let Some(level) = heading_level {
			map.insert(style_id.to_string(), level);
		}
	}
	map
}

fn parse_legacy_doc(context: &ParserContext) -> Result<Document> {
	let file =
		File::open(&context.file_path).with_context(|| format!("Failed to open DOC file '{}'", context.file_path))?;
	let mut compound =
		CompoundFile::open(file).with_context(|| format!("Failed to parse OLE container '{}'", context.file_path))?;
	let word_document =
		read_stream(&mut compound, "WordDocument").or_else(|_| read_stream(&mut compound, "/WordDocument"))?;
	if word_document.len() < FIB_LCBCLX_OFFSET + 4 {
		anyhow::bail!("DOC file is missing required FIB fields");
	}
	let fib_magic = read_u16_le(&word_document, 0);
	if fib_magic != FIB_MAGIC_DOC && fib_magic != FIB_MAGIC_DOC_OLD {
		anyhow::bail!("Not a valid DOC file (invalid FIB magic)");
	}
	let fib_flags = read_u16_le(&word_document, FIB_FLAGS_OFFSET);
	if (fib_flags & FIB_FLAG_ENCRYPTED) != 0 {
		let Some(password) = context.password.as_deref() else {
			anyhow::bail!("{PASSWORD_REQUIRED_ERROR_PREFIX} DOC file is encrypted and requires a password");
		};
		let decrypted = decrypt_from_file(&context.file_path, password).map_err(|e| {
			anyhow::anyhow!("{PASSWORD_REQUIRED_ERROR_PREFIX} DOC decryption failed (wrong password?): {e}")
		})?;
		let mut dec_compound =
			CompoundFile::open(Cursor::new(decrypted)).context("Decrypted DOC data is not a valid compound file")?;
		let word_document = read_stream(&mut dec_compound, "WordDocument")
			.or_else(|_| read_stream(&mut dec_compound, "/WordDocument"))?;
		let fib_flags2 = read_u16_le(&word_document, FIB_FLAGS_OFFSET);
		let table_stream_name2 = if (fib_flags2 & FIB_FLAG_USE_1_TABLE) != 0 { "1Table" } else { "0Table" };
		let table_stream2 = read_stream(&mut dec_compound, table_stream_name2)
			.or_else(|_| read_stream(&mut dec_compound, &format!("/{table_stream_name2}")))?;
		let mut text = extract_doc_text_from_piece_table(&word_document, &table_stream2)
			.unwrap_or_else(|| extract_doc_text_simple(&word_document));
		if text.trim().is_empty() {
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
	let mut text = extract_doc_text_from_piece_table(&word_document, &table_stream)
		.unwrap_or_else(|| extract_doc_text_simple(&word_document));
	if text.trim().is_empty() {
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

fn read_stream<R: std::io::Read + std::io::Seek>(compound: &mut CompoundFile<R>, path: &str) -> Result<Vec<u8>> {
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

fn parse_text_like_doc(context: &ParserContext) -> Result<Document> {
	let bytes = std::fs::read(&context.file_path)
		.with_context(|| format!("Failed to read potential text DOC '{}'", context.file_path))?;
	let decoded = convert_to_utf8(&bytes);
	if !looks_like_text_content(&decoded) {
		anyhow::bail!("File content does not look like plain text");
	}
	let normalized = normalize_doc_text(&decoded);
	if normalized.trim().is_empty() {
		anyhow::bail!("No readable text content found");
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

fn traverse(
	node: Node,
	buffer: &mut DocumentBuffer,
	headings: &mut Vec<HeadingInfo>,
	id_positions: &mut HashMap<String, usize>,
	rels: &HashMap<String, String>,
	style_heading_map: &HashMap<String, i32>,
	render_tables_inline: bool,
) {
	if node.node_type() == NodeType::Element {
		let tag_name = node.tag_name().name();
		if let Some(id) = node.attribute("id") {
			id_positions.insert(id.to_string(), buffer.current_position());
		}
		if tag_name == "p" {
			process_paragraph(node, buffer, headings, id_positions, rels, style_heading_map);
			return;
		} else if tag_name == "tbl" {
			process_table(node, buffer, rels, render_tables_inline);
			return;
		}
	}
	for child in node.children() {
		traverse(child, buffer, headings, id_positions, rels, style_heading_map, render_tables_inline);
	}
}

fn process_table(
	element: Node,
	buffer: &mut DocumentBuffer,
	_rels: &HashMap<String, String>,
	render_tables_inline: bool,
) {
	let table_start = buffer.current_position();
	let mut rows: Vec<Vec<String>> = Vec::new();
	for child in element.children() {
		if child.node_type() == NodeType::Element && child.tag_name().name() == "tr" {
			let mut cells: Vec<String> = Vec::new();
			for tc in child.children() {
				if tc.node_type() == NodeType::Element && tc.tag_name().name() == "tc" {
					let mut cell_text = String::new();
					for p in tc.children() {
						if p.node_type() == NodeType::Element && p.tag_name().name() == "p" {
							for r in p.children() {
								if r.node_type() == NodeType::Element && r.tag_name().name() == "r" {
									cell_text.push_str(&collect_ooxml_run_text(r));
								}
							}
							cell_text.push(' ');
						}
					}
					cells.push(cell_text.trim().to_string());
				}
			}
			rows.push(cells);
		}
	}
	let html_content = crate::parser::table_text::build_html_table_from_grid(&rows);
	// Derive the caption the same way HTML/XML do (first-row text, no prefix) for consistent
	// labels across formats; fall back to "table" for an empty table like `table_caption_from_tsv`.
	let final_caption =
		crate::parser::table_text::table_caption_from_html(&html_content).unwrap_or_else(|| "table".to_string());
	let display_text = crate::parser::table_text::html_table_to_display(&html_content, render_tables_inline);
	buffer.append(&display_text);
	buffer.append("\n");
	let display_len = buffer.current_position() - table_start;
	buffer.add_marker(
		Marker::new(MarkerType::Table, table_start)
			.with_text(final_caption)
			.with_reference(html_content)
			.with_length(display_len),
	);
}

fn process_paragraph(
	element: Node,
	buffer: &mut DocumentBuffer,
	headings: &mut Vec<HeadingInfo>,
	id_positions: &mut HashMap<String, usize>,
	rels: &HashMap<String, String>,
	style_heading_map: &HashMap<String, i32>,
) {
	let paragraph_start = buffer.current_position();
	let mut paragraph_text = String::new();
	let mut para_display_len = 0usize;
	let mut heading_level = 0;
	let mut is_paragraph_style_heading = false;
	let mut format_spans: Vec<(MarkerType, usize, usize)> = Vec::new();
	for child in element.children() {
		if child.node_type() != NodeType::Element {
			continue;
		}
		let tag_name = child.tag_name().name();
		if tag_name == "pPr" {
			heading_level = get_paragraph_heading_level(child, style_heading_map);
			if heading_level > 0 {
				is_paragraph_style_heading = true;
			}
		} else if tag_name == "bookmarkStart" {
			if let Some(name) = child.attribute("name") {
				id_positions.insert(name.to_string(), paragraph_start + paragraph_text.len());
			}
		} else if tag_name == "hyperlink" {
			para_display_len += process_hyperlink(child, &mut paragraph_text, buffer, rels, paragraph_start);
		} else if tag_name == "r" {
			if heading_level == 0
				&& let Some(rpr_node) = find_child_element(child, "rPr")
			{
				heading_level = get_run_heading_level(rpr_node);
			}
			if let Some(instr_text_node) = find_child_element(child, "instrText")
				&& let Some(instruction) = instr_text_node.text()
				&& instruction.contains("HYPERLINK")
			{
				let link_target = parse_hyperlink_instruction(instruction);
				if !link_target.is_empty() {
					let (display_text, _) = extract_field_display_text(element, child);
					if !display_text.is_empty() {
						let link_offset = paragraph_start + paragraph_text.len();
						paragraph_text.push_str(&display_text);
						para_display_len += display_len(&display_text);
						buffer.add_marker(
							Marker::new(MarkerType::Link, link_offset)
								.with_text(display_text.clone())
								.with_reference(link_target),
						);
					}
				}
			}
			let run_text = collect_ooxml_run_text(child);
			if !run_text.is_empty() {
				let run_start = paragraph_start + para_display_len;
				let run_len = display_len(&run_text);
				if let Some(rpr_node) = find_child_element(child, "rPr") {
					let (bold, italic, underline) = get_run_format_flags(rpr_node);
					let run_end = run_start + run_len;
					if run_end > run_start {
						format_spans.extend(
							format_marker_types(bold, italic, underline).map(|kind| (kind, run_start, run_end)),
						);
					}
				}
				paragraph_text.push_str(&run_text);
				para_display_len += run_len;
			}
		}
	}
	let trimmed = paragraph_text.trim();
	buffer.append(trimmed);
	buffer.append("\n");
	let leading_trim = display_len(&paragraph_text) - display_len(paragraph_text.trim_start());
	for (kind, start, end) in format_spans {
		let adj_start = start.saturating_sub(leading_trim);
		let adj_end = end.saturating_sub(leading_trim);
		if adj_end > adj_start {
			buffer.add_marker(Marker::new(kind, adj_start).with_length(adj_end - adj_start));
		}
	}
	if heading_level > 0 && !trimmed.is_empty() {
		let heading_text =
			if is_paragraph_style_heading { trimmed.to_string() } else { extract_heading_text(element, heading_level) };
		if !heading_text.is_empty() {
			let marker_type = heading_level_to_marker_type(heading_level);
			buffer.add_marker(
				Marker::new(marker_type, paragraph_start).with_text(heading_text.clone()).with_level(heading_level),
			);
			headings.push(HeadingInfo { offset: paragraph_start, level: heading_level, text: heading_text });
		}
	}
}

/// Appends the hyperlink's display text to `paragraph_text`, records a Link
/// marker, and returns the number of display units appended.
fn process_hyperlink(
	element: Node,
	paragraph_text: &mut String,
	buffer: &mut DocumentBuffer,
	rels: &HashMap<String, String>,
	paragraph_start: usize,
) -> usize {
	let r_id = element.attribute("id").unwrap_or("");
	let anchor = element.attribute("anchor").unwrap_or("");
	let link_target = if !r_id.is_empty() {
		rels.get(r_id).cloned().unwrap_or_default()
	} else if !anchor.is_empty() {
		format!("#{anchor}")
	} else {
		String::new()
	};
	let mut link_text = String::new();
	for child in element.children() {
		if child.node_type() == NodeType::Element && child.tag_name().name() == "r" {
			link_text.push_str(&collect_ooxml_run_text(child));
		}
	}
	if link_text.is_empty() {
		return 0;
	}
	let link_offset = paragraph_start + paragraph_text.len();
	paragraph_text.push_str(&link_text);
	if !link_target.is_empty() {
		buffer.add_marker(
			Marker::new(MarkerType::Link, link_offset).with_text(link_text.clone()).with_reference(link_target),
		);
	}
	display_len(&link_text)
}

fn get_paragraph_heading_level(pr_element: Node, style_heading_map: &HashMap<String, i32>) -> i32 {
	const MAX_HEADING_LEVEL: i32 = 9;
	for child in pr_element.children() {
		if child.node_type() != NodeType::Element {
			continue;
		}
		let tag_name = child.tag_name().name();
		if tag_name == "pStyle" {
			if let Some(style) = child.attribute("val") {
				let style_lower = style.to_lowercase();
				if style_lower.starts_with("heading") {
					if let Some(level) = extract_number_from_string(style)
						&& level > 0 && level <= MAX_HEADING_LEVEL
					{
						return level;
					}
				} else if let Some(&level) = style_heading_map.get(style) {
					return level;
				}
			}
		} else if tag_name == "outlineLvl"
			&& let Some(level_str) = child.attribute("val")
			&& let Ok(level) = level_str.parse::<i32>()
		{
			let actual_level = level + 1;
			if actual_level > 0 && actual_level <= MAX_HEADING_LEVEL {
				return actual_level;
			}
		}
	}
	0
}

fn get_run_heading_level(rpr_element: Node) -> i32 {
	const MAX_HEADING_LEVEL: i32 = 9;
	if let Some(rstyle_node) = find_child_element(rpr_element, "rStyle")
		&& let Some(style) = rstyle_node.attribute("val")
	{
		let style_lower = style.to_lowercase();
		if style_lower.starts_with("heading")
			&& style_lower.ends_with("char")
			&& let Some(level) = extract_number_from_string(style)
			&& level > 0
			&& level <= MAX_HEADING_LEVEL
		{
			return level;
		}
	}
	0
}

fn get_run_format_flags(rpr_element: Node) -> (bool, bool, bool) {
	let is_toggle_on = |tag: &str| {
		find_child_element(rpr_element, tag)
			.is_some_and(|node| node.attribute("val").is_none_or(|v| !matches!(v, "false" | "0")))
	};
	let bold = is_toggle_on("b");
	let italic = is_toggle_on("i");
	let underline =
		find_child_element(rpr_element, "u").is_some_and(|node| node.attribute("val").is_none_or(|v| v != "none"));
	(bold, italic, underline)
}

fn extract_heading_text(paragraph: Node, heading_level: i32) -> String {
	let mut text = String::new();
	for child in paragraph.children() {
		if child.node_type() != NodeType::Element {
			continue;
		}
		let tag_name = child.tag_name().name();
		if tag_name == "r" {
			let run_level = find_child_element(child, "rPr").map_or(0, get_run_heading_level);
			if run_level == heading_level {
				text.push_str(&collect_ooxml_run_text(child));
			}
		} else if tag_name == "hyperlink" {
			for link_child in child.children() {
				if link_child.node_type() == NodeType::Element && link_child.tag_name().name() == "r" {
					let run_level = find_child_element(link_child, "rPr").map_or(0, get_run_heading_level);
					if run_level == heading_level {
						text.push_str(&collect_ooxml_run_text(link_child));
					}
				}
			}
		}
	}
	text.trim().to_string()
}

fn parse_hyperlink_instruction(instruction: &str) -> String {
	let first_quote = instruction.find('"');
	let last_quote = instruction.rfind('"');
	if let (Some(first), Some(last)) = (first_quote, last_quote)
		&& first != last
	{
		let target = &instruction[first + 1..last];
		if instruction.contains("\\l") {
			return format!("#{target}");
		}
		return target.to_string();
	}
	String::new()
}

fn extract_field_display_text(paragraph: Node, instr_run: Node) -> (String, bool) {
	let mut display_text = String::new();
	let mut in_display_text = false;
	let mut found = false;
	let children: Vec<_> = paragraph.children().collect();
	let mut start_index = 0;
	for (i, child) in children.iter().enumerate() {
		if child.id() == instr_run.id() {
			start_index = i + 1;
			found = true;
			break;
		}
	}
	if !found {
		return (display_text, false);
	}
	for child in children.iter().skip(start_index) {
		if child.node_type() == NodeType::Element && child.tag_name().name() == "r" {
			if let Some(fld_char) = find_child_element(*child, "fldChar") {
				if let Some(fld_type) = fld_char.attribute("fldCharType") {
					if fld_type == "separate" {
						in_display_text = true;
					} else if fld_type == "end" {
						break;
					}
				}
			} else if in_display_text {
				display_text.push_str(&collect_ooxml_run_text(*child));
			}
		}
	}
	(display_text, true)
}

fn extract_number_from_string(s: &str) -> Option<i32> {
	let digits: String = s.chars().filter(char::is_ascii_digit).collect();
	digits.parse().ok()
}

/// If `path` looks like an encrypted OLE compound file (has an `EncryptionInfo` stream),
/// attempts to decrypt it with `password` and returns the decrypted bytes.
/// Returns `None` if the file is not a compound file or is not encrypted.
/// Returns an error if it is encrypted but decryption fails (wrong password, etc.).
pub fn try_decrypt_office_file(path: &str, password: Option<&str>) -> Result<Option<Vec<u8>>> {
	// Try opening as a CFB compound file. Plain ZIPs will fail here.
	let file = File::open(path).with_context(|| format!("Failed to open '{path}'"))?;
	let compound = match CompoundFile::open(file) {
		Ok(c) => c,
		Err(_) => return Ok(None), // Not a compound file at all
	};
	// Encrypted OOXML files always contain an EncryptionInfo stream.
	if compound.entry("/EncryptionInfo").is_err() {
		return Ok(None); // Compound file but not encrypted Office format
	}
	let Some(password) = password else {
		anyhow::bail!("{PASSWORD_REQUIRED_ERROR_PREFIX} File is encrypted and requires a password");
	};
	let decrypted =
		decrypt_from_file(path, password).map_err(|e| anyhow::anyhow!("Decryption failed (wrong password?): {e}"))?;
	Ok(Some(decrypted))
}

#[cfg(test)]
mod tests {
	use std::collections::HashMap;

	use roxmltree::Document as XmlDocument;

	use super::{looks_like_text_content, normalize_doc_text, parse_doc_clx, parse_doc_piece_table, traverse};
	use crate::{
		document::{DocumentBuffer, MarkerType},
		util::text::display_len,
	};

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

	/// Parse a Word table. The second cell contains U+1D11E (MUSICAL SYMBOL G CLEF, non-BMP,
	/// UTF-16 width 2) to lock the display-unit arithmetic. OFF mode emits the placeholder; ON mode
	/// emits the full TSV. In both cases the Table marker keeps the caption as text and its length
	/// equals the emitted display extent.
	#[test]
	fn word_table_emits_placeholder_or_tsv_by_flag() {
		// Minimal OOXML XML: one table with one row, two cells.
		let xml = r#"<document><body>
			<tbl>
				<tr>
					<tc><p><r><t>Kop</t></r></p></tc>
					<tc><p><r><t>&#x1D11E;</t></r></p></tc>
				</tr>
			</tbl>
		</body></document>"#;
		let xml_doc = XmlDocument::parse(xml).expect("valid xml");

		// OFF: placeholder "[Table]: Kop 𝄞".
		let mut buffer = DocumentBuffer::new();
		let mut headings = Vec::new();
		let mut id_positions = HashMap::new();
		let rels = HashMap::new();
		traverse(xml_doc.root(), &mut buffer, &mut headings, &mut id_positions, &rels, &HashMap::new(), false);
		assert_eq!(buffer.content, "[Table]: Kop \u{1D11E}\n");
		let table_marker = buffer.markers.iter().find(|m| m.mtype == MarkerType::Table).expect("Table marker");
		assert_eq!(table_marker.text, "Kop \u{1D11E}", "marker caption is the first-row text, no prefix");
		assert_eq!(table_marker.length, display_len("[Table]: Kop \u{1D11E}") + 1, "marker length in display units");
		assert!(table_marker.reference.contains("<td>Kop</td>"), "marker reference is the table HTML");

		// ON: full TSV "Kop\t𝄞".
		let mut buffer = DocumentBuffer::new();
		let mut headings = Vec::new();
		let mut id_positions = HashMap::new();
		traverse(xml_doc.root(), &mut buffer, &mut headings, &mut id_positions, &rels, &HashMap::new(), true);
		assert_eq!(buffer.content, "Kop\t\u{1D11E}\n");
		let table_marker = buffer.markers.iter().find(|m| m.mtype == MarkerType::Table).expect("Table marker");
		assert_eq!(table_marker.length, display_len("Kop\t\u{1D11E}") + 1, "marker length spans the TSV");
	}

	/// Parse a single paragraph and return the buffer, so run-property (`<w:rPr>`) format markers
	/// can be inspected. Test XML uses unnamespaced tags/attributes to match `attribute("val")`
	/// (roxmltree matches on the local name here, mirroring the existing table test fixtures).
	fn parse_run_props(xml: &str) -> DocumentBuffer {
		let xml_doc = XmlDocument::parse(xml).expect("valid xml");
		let mut buffer = DocumentBuffer::new();
		let mut headings = Vec::new();
		let mut id_positions = HashMap::new();
		let rels = HashMap::new();
		traverse(xml_doc.root(), &mut buffer, &mut headings, &mut id_positions, &rels, &HashMap::new(), false);
		buffer
	}

	#[test]
	fn run_bold_property_emits_bold_marker() {
		let buffer = parse_run_props(r"<document><body><p><r><rPr><b/></rPr><t>bold</t></r></p></body></document>");
		let marker = buffer.markers.iter().find(|m| m.mtype == MarkerType::Bold).expect("Bold marker");
		assert_eq!(marker.position, 0);
		assert_eq!(marker.length, display_len("bold"));
	}

	#[test]
	fn run_italic_property_emits_italic_marker() {
		let buffer = parse_run_props(r"<document><body><p><r><rPr><i/></rPr><t>italic</t></r></p></body></document>");
		let marker = buffer.markers.iter().find(|m| m.mtype == MarkerType::Italic).expect("Italic marker");
		assert_eq!(marker.position, 0);
		assert_eq!(marker.length, display_len("italic"));
	}

	#[test]
	fn run_underline_property_emits_underline_marker() {
		let buffer = parse_run_props(
			r#"<document><body><p><r><rPr><u val="single"/></rPr><t>under</t></r></p></body></document>"#,
		);
		let marker = buffer.markers.iter().find(|m| m.mtype == MarkerType::Underline).expect("Underline marker");
		assert_eq!(marker.position, 0);
		assert_eq!(marker.length, display_len("under"));
	}

	#[test]
	fn run_bold_and_italic_together_emit_both_spanning_same_range() {
		let buffer = parse_run_props(r"<document><body><p><r><rPr><b/><i/></rPr><t>both</t></r></p></body></document>");
		let bold = buffer.markers.iter().find(|m| m.mtype == MarkerType::Bold).expect("Bold marker");
		let italic = buffer.markers.iter().find(|m| m.mtype == MarkerType::Italic).expect("Italic marker");
		assert_eq!(bold.position, italic.position);
		assert_eq!(bold.length, italic.length);
		assert_eq!(bold.position, 0);
		assert_eq!(bold.length, display_len("both"));
	}

	#[test]
	fn run_underline_none_is_not_underlined() {
		let buffer = parse_run_props(
			r#"<document><body><p><r><rPr><u val="none"/></rPr><t>plain</t></r></p></body></document>"#,
		);
		assert!(
			!buffer.markers.iter().any(|m| m.mtype == MarkerType::Underline),
			"u val=none must not produce an Underline marker"
		);
	}

	#[test]
	fn run_bold_false_cancels_bold() {
		let buffer = parse_run_props(
			r#"<document><body><p><r><rPr><b val="false"/></rPr><t>plain</t></r></p></body></document>"#,
		);
		assert!(
			!buffer.markers.iter().any(|m| m.mtype == MarkerType::Bold),
			"b val=false must not produce a Bold marker"
		);
	}

	#[test]
	fn run_bold_zero_cancels_bold() {
		let buffer =
			parse_run_props(r#"<document><body><p><r><rPr><b val="0"/></rPr><t>plain</t></r></p></body></document>"#);
		assert!(!buffer.markers.iter().any(|m| m.mtype == MarkerType::Bold), "b val=0 must not produce a Bold marker");
	}

	/// The offset of a format marker must be computed in DISPLAY units, not byte length. A paragraph
	/// beginning with a multi-byte (but display-stable) character before the bold run would place the
	/// Bold marker at the wrong position if `String::len()` (bytes) were used instead of `display_len`.
	#[test]
	fn run_format_offset_uses_display_units_not_bytes() {
		// "é" is 2 bytes in UTF-8 but 1 display unit (single UTF-16 code unit / one char).
		let buffer = parse_run_props(
			r"<document><body><p><r><t>é</t></r><r><rPr><b/></rPr><t>bold</t></r></p></body></document>",
		);
		let marker = buffer.markers.iter().find(|m| m.mtype == MarkerType::Bold).expect("Bold marker");
		assert_eq!(marker.position, display_len("é"), "offset must be display-unit, not byte length");
		assert_ne!(marker.position, "é".len(), "byte length (2) would be the bug");
		assert_eq!(marker.length, display_len("bold"));
	}

	/// A paragraph starting with a whitespace-only unformatted run before a bold run must not
	/// desync the Bold marker's offset. `process_paragraph` only appends the TRIMMED paragraph
	/// text to the buffer, so the leading spaces never make it into the final content - the
	/// bold run's offset must be shifted left by the same amount that gets trimmed, or the
	/// marker ends up pointing past the start of "bold" into the wrong text.
	#[test]
	fn run_format_offset_accounts_for_leading_whitespace_trim() {
		let buffer = parse_run_props(
			r#"<document><body><p><r><t xml:space="preserve">  </t></r><r><rPr><b/></rPr><t>bold</t></r></p></body></document>"#,
		);
		assert_eq!(buffer.content, "bold\n", "leading whitespace run must be trimmed from the final content");
		let marker = buffer.markers.iter().find(|m| m.mtype == MarkerType::Bold).expect("Bold marker");
		assert_eq!(marker.position, 0, "Bold marker must point at the start of the trimmed content");
		assert_eq!(marker.length, display_len("bold"));
	}
}
