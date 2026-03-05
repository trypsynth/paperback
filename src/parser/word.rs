use std::{
	collections::HashMap,
	fs::File,
	io::{BufReader, Read},
	path::Path,
};

use anyhow::{Context, Result};
use cfb::CompoundFile;
use encoding_rs::WINDOWS_1252;
use roxmltree::{Document as XmlDocument, Node, NodeType};
use zip::ZipArchive;

use crate::{
	document::{Document, DocumentBuffer, Marker, MarkerType, ParserContext, ParserFlags},
	parser::{
		Parser, PASSWORD_REQUIRED_ERROR_PREFIX,
		ooxml::{collect_ooxml_run_text, read_ooxml_relationships},
		path::extract_title_from_path,
		toc::{build_toc_from_buffer, heading_level_to_marker_type},
		xml::find_child_element,
	},
	types::HeadingInfo,
	zip::read_zip_entry_by_name,
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
		&["docx", "docm", "doc"]
	}

	fn supported_flags(&self) -> ParserFlags {
		ParserFlags::SUPPORTS_TOC
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
		if extension == "doc" {
			return parse_legacy_doc(context);
		}
		parse_ooxml_doc(context)
	}
}

fn parse_ooxml_doc(context: &ParserContext) -> Result<Document> {
	let file = File::open(&context.file_path).with_context(|| format!("Failed to open DOCX file '{}'", context.file_path))?;
	let mut archive = ZipArchive::new(BufReader::new(file)).with_context(|| format!("Failed to read DOCX as zip '{}'", context.file_path))?;
	let rels = read_ooxml_relationships(&mut archive, "word/_rels/document.xml.rels");
	let doc_content = read_zip_entry_by_name(&mut archive, "word/document.xml")?;
	let doc_xml = XmlDocument::parse(&doc_content).context("Failed to parse word/document.xml")?;
	let mut buffer = DocumentBuffer::new();
	let mut id_positions = HashMap::new();
	let mut headings = Vec::new();
	traverse(doc_xml.root(), &mut buffer, &mut headings, &mut id_positions, &rels);
	let title = extract_title_from_path(&context.file_path);
	let toc_items = build_toc_from_buffer(&buffer);
	let mut document = Document::new().with_title(title);
	document.set_buffer(buffer);
	document.id_positions = id_positions;
	document.toc_items = toc_items;
	Ok(document)
}

fn parse_legacy_doc(context: &ParserContext) -> Result<Document> {
	let file = File::open(&context.file_path).with_context(|| format!("Failed to open DOC file '{}'", context.file_path))?;
	let mut compound = CompoundFile::open(file).with_context(|| format!("Failed to parse OLE container '{}'", context.file_path))?;
	let word_document = read_stream(&mut compound, "WordDocument").or_else(|_| read_stream(&mut compound, "/WordDocument"))?;
	if word_document.len() < FIB_LCBCLX_OFFSET + 4 {
		anyhow::bail!("DOC file is missing required FIB fields");
	}
	let fib_magic = read_u16_le(&word_document, 0);
	if fib_magic != FIB_MAGIC_DOC && fib_magic != FIB_MAGIC_DOC_OLD {
		anyhow::bail!("Not a valid DOC file (invalid FIB magic)");
	}
	let fib_flags = read_u16_le(&word_document, FIB_FLAGS_OFFSET);
	if (fib_flags & FIB_FLAG_ENCRYPTED) != 0 {
		anyhow::bail!("{PASSWORD_REQUIRED_ERROR_PREFIX} DOC file is encrypted and requires a password");
	}
	let table_stream_name = if (fib_flags & FIB_FLAG_USE_1_TABLE) != 0 { "1Table" } else { "0Table" };
	let table_stream = read_stream(&mut compound, table_stream_name)
		.or_else(|_| read_stream(&mut compound, &format!("/{table_stream_name}")))
		.with_context(|| format!("Failed to open DOC table stream '{table_stream_name}'"))?;
	let mut text = extract_doc_text_from_piece_table(&word_document, &table_stream).unwrap_or_else(|| extract_doc_text_simple(&word_document));
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

fn read_stream(compound: &mut CompoundFile<File>, path: &str) -> Result<Vec<u8>> {
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
	let mut normalized = text.replace("\r\n", "\n").replace('\r', "\n");
	normalized = normalized.replace('\u{13}', "").replace('\u{14}', "").replace('\u{15}', "");
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
) {
	if node.node_type() == NodeType::Element {
		let tag_name = node.tag_name().name();
		if let Some(id) = node.attribute("id") {
			id_positions.insert(id.to_string(), buffer.current_position());
		}
		if tag_name == "p" {
			process_paragraph(node, buffer, headings, id_positions, rels);
			return;
		} else if tag_name == "tbl" {
			process_table(node, buffer, rels);
			return;
		}
	}
	for child in node.children() {
		traverse(child, buffer, headings, id_positions, rels);
	}
}

fn process_table(element: Node, buffer: &mut DocumentBuffer, _rels: &HashMap<String, String>) {
	let table_start = buffer.current_position();
	let mut html_content = String::from("<table border=\"1\">");
	let mut table_caption = String::from("table: ");
	let mut first_row = true;
	for child in element.children() {
		if child.node_type() == NodeType::Element && child.tag_name().name() == "tr" {
			html_content.push_str("<tr>");
			for tc in child.children() {
				if tc.node_type() == NodeType::Element && tc.tag_name().name() == "tc" {
					html_content.push_str("<td>");
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
					let trimmed_cell = cell_text.trim();
					html_content.push_str(trimmed_cell);
					html_content.push_str("</td>");
					if first_row {
						table_caption.push_str(trimmed_cell);
						table_caption.push(' ');
					}
				}
			}
			html_content.push_str("</tr>");
			first_row = false;
		}
	}
	html_content.push_str("</table>");
	let final_caption = table_caption.trim().to_string();
	buffer.append(&final_caption);
	buffer.append("\n");
	buffer.add_marker(
		Marker::new(MarkerType::Table, table_start)
			.with_text(final_caption.clone())
			.with_reference(html_content)
			.with_length(final_caption.len()),
	);
}

fn process_paragraph(
	element: Node,
	buffer: &mut DocumentBuffer,
	headings: &mut Vec<HeadingInfo>,
	id_positions: &mut HashMap<String, usize>,
	rels: &HashMap<String, String>,
) {
	let paragraph_start = buffer.current_position();
	let mut paragraph_text = String::new();
	let mut heading_level = 0;
	let mut is_paragraph_style_heading = false;
	for child in element.children() {
		if child.node_type() != NodeType::Element {
			continue;
		}
		let tag_name = child.tag_name().name();
		if tag_name == "pPr" {
			heading_level = get_paragraph_heading_level(child);
			if heading_level > 0 {
				is_paragraph_style_heading = true;
			}
		} else if tag_name == "bookmarkStart" {
			if let Some(name) = child.attribute("name") {
				id_positions.insert(name.to_string(), paragraph_start + paragraph_text.len());
			}
		} else if tag_name == "hyperlink" {
			process_hyperlink(child, &mut paragraph_text, buffer, rels, paragraph_start);
		} else if tag_name == "r" {
			if heading_level == 0 {
				if let Some(rpr_node) = find_child_element(child, "rPr") {
					heading_level = get_run_heading_level(rpr_node);
				}
			}
			if let Some(instr_text_node) = find_child_element(child, "instrText") {
				if let Some(instruction) = instr_text_node.text() {
					if instruction.contains("HYPERLINK") {
						let link_target = parse_hyperlink_instruction(instruction);
						if !link_target.is_empty() {
							let (display_text, _) = extract_field_display_text(element, child);
							if !display_text.is_empty() {
								let link_offset = paragraph_start + paragraph_text.len();
								paragraph_text.push_str(&display_text);
								buffer.add_marker(
									Marker::new(MarkerType::Link, link_offset)
										.with_text(display_text.clone())
										.with_reference(link_target),
								);
							}
						}
					}
				}
			}
			paragraph_text.push_str(&collect_ooxml_run_text(child));
		}
	}
	let trimmed = paragraph_text.trim();
	buffer.append(trimmed);
	buffer.append("\n");
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

fn process_hyperlink(
	element: Node,
	paragraph_text: &mut String,
	buffer: &mut DocumentBuffer,
	rels: &HashMap<String, String>,
	paragraph_start: usize,
) {
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
		return;
	}
	let link_offset = paragraph_start + paragraph_text.len();
	paragraph_text.push_str(&link_text);
	if !link_target.is_empty() {
		buffer.add_marker(
			Marker::new(MarkerType::Link, link_offset).with_text(link_text.clone()).with_reference(link_target),
		);
	}
}

fn get_paragraph_heading_level(pr_element: Node) -> i32 {
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
					if let Some(level) = extract_number_from_string(style) {
						if level > 0 && level <= MAX_HEADING_LEVEL {
							return level;
						}
					}
				}
			}
		} else if tag_name == "outlineLvl" {
			if let Some(level_str) = child.attribute("val") {
				if let Ok(level) = level_str.parse::<i32>() {
					let actual_level = level + 1;
					if actual_level > 0 && actual_level <= MAX_HEADING_LEVEL {
						return actual_level;
					}
				}
			}
		}
	}
	0
}

fn get_run_heading_level(rpr_element: Node) -> i32 {
	const MAX_HEADING_LEVEL: i32 = 9;
	if let Some(rstyle_node) = find_child_element(rpr_element, "rStyle") {
		if let Some(style) = rstyle_node.attribute("val") {
			let style_lower = style.to_lowercase();
			if style_lower.starts_with("heading") && style_lower.ends_with("char") {
				if let Some(level) = extract_number_from_string(style) {
					if level > 0 && level <= MAX_HEADING_LEVEL {
						return level;
					}
				}
			}
		}
	}
	0
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
	if let (Some(first), Some(last)) = (first_quote, last_quote) {
		if first != last {
			let target = &instruction[first + 1..last];
			if instruction.contains("\\l") {
				return format!("#{target}");
			}
			return target.to_string();
		}
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

#[cfg(test)]
mod tests {
	use super::{normalize_doc_text, parse_doc_clx, parse_doc_piece_table};

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
		let text = "A\r\nB\u{13}C\u{14}\u{15}\n\n\nD";
		assert_eq!(normalize_doc_text(text), "A\nBC\n\nD");
	}
}
