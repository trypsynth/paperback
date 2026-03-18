use std::{
	collections::HashMap,
	fs::File,
	io::{Cursor, Read},
	path::Path,
};

use anyhow::{Context, Result};
use cfb::CompoundFile;
use roxmltree::{Document as XmlDocument, Node, NodeType};
use zip::ZipArchive;

use crate::{
	document::{Document, DocumentBuffer, Marker, MarkerType, ParserContext, ParserFlags, TocItem},
	parser::{
		Parser, ooxml::read_ooxml_relationships, path::extract_title_from_path, word::try_decrypt_office_file,
		xml::collect_text_from_tagged_elements,
	},
	types::LinkInfo,
	zip::read_zip_entry_by_name,
};

const PPT_RECORD_HEADER_SIZE: usize = 8;
const PPT_REC_SLIDE: u16 = 1006;
const PPT_REC_TEXT_CHARS_ATOM: u16 = 4000;
const PPT_REC_TEXT_BYTES_ATOM: u16 = 4008;
const PPT_REC_CSTRING: u16 = 4026;

pub struct PowerpointParser;

impl Parser for PowerpointParser {
	fn name(&self) -> &'static str {
		"PowerPoint Presentations"
	}

	fn extensions(&self) -> &[&str] {
		&["pptx", "pptm", "ppt"]
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
		if extension == "ppt" {
			return parse_legacy_ppt(context);
		}
		parse_pptx(context)
	}
}

fn parse_pptx(context: &ParserContext) -> Result<Document> {
	let bytes = match try_decrypt_office_file(&context.file_path, context.password.as_deref())? {
		Some(decrypted) => decrypted,
		None => std::fs::read(&context.file_path)
			.with_context(|| format!("Failed to read PPTX file '{}'", context.file_path))?,
	};
	let mut archive = ZipArchive::new(Cursor::new(bytes))
		.with_context(|| format!("Failed to read PPTX as zip '{}'", context.file_path))?;
	let mut slides = (0..archive.len())
		.filter_map(|i| archive.by_index(i).ok().map(|entry| entry.name().to_string()))
		.filter(|name| {
			name.starts_with("ppt/slides/slide")
				&& Path::new(name).extension().is_some_and(|ext| ext.eq_ignore_ascii_case("xml"))
				&& !name.contains("_rels")
		})
		.collect::<Vec<_>>();
	if slides.is_empty() {
		anyhow::bail!("PPTX file contains no slides");
	}
	slides.sort_by_key(|name| extract_slide_number(name));
	let mut buffer = DocumentBuffer::new();
	let id_positions = HashMap::new();
	let mut toc_items = Vec::new();
	for (index, slide_name) in slides.iter().enumerate() {
		let slide_content = read_zip_entry_by_name(&mut archive, slide_name)?;
		let slide_doc =
			XmlDocument::parse(&slide_content).with_context(|| format!("Failed to parse slide '{slide_name}'"))?;
		let slide_base = slide_name.rsplit('/').next().unwrap_or("");
		let rels_name = format!("ppt/slides/_rels/{slide_base}.rels");
		let rels = read_ooxml_relationships(&mut archive, &rels_name);
		let slide_title = extract_slide_title(slide_doc.root());
		let slide_start = buffer.current_position();
		let mut links = Vec::new();
		let slide_text = extract_slide_text(slide_doc.root(), &mut links, slide_start, &rels);
		if !slide_text.trim().is_empty() {
			buffer.append(&slide_text);
			if !buffer.content.ends_with('\n') {
				buffer.append("\n");
			}
			if index + 1 < slides.len() {
				buffer.append("\n");
			}
			buffer
				.add_marker(Marker::new(MarkerType::PageBreak, slide_start).with_text(format!("Slide {}", index + 1)));
			for link in links {
				buffer.add_marker(
					Marker::new(MarkerType::Link, link.offset).with_text(link.text).with_reference(link.reference),
				);
			}
			let toc_name = if slide_title.is_empty() { format!("Slide {}", index + 1) } else { slide_title.clone() };
			toc_items.push(TocItem::new(toc_name, String::new(), slide_start));
		}
	}
	let title = extract_title_from_path(&context.file_path);
	let mut document = Document::new().with_title(title);
	document.set_buffer(buffer);
	document.id_positions = id_positions;
	document.toc_items = toc_items;
	Ok(document)
}

fn parse_legacy_ppt(context: &ParserContext) -> Result<Document> {
	let file =
		File::open(&context.file_path).with_context(|| format!("Failed to open PPT file '{}'", context.file_path))?;
	let mut compound =
		CompoundFile::open(file).with_context(|| format!("Failed to parse OLE container '{}'", context.file_path))?;

	// Encrypted PPT files have an EncryptionInfo stream. We can detect but not decrypt them.
	if compound.entry("/EncryptionInfo").is_ok() {
		anyhow::bail!("Password-protected PPT files are not currently supported. Try saving the file as PPTX and opening that instead.");
	}

	let ppt_document_stream = read_ppt_document_stream(&mut compound)
		.with_context(|| format!("Failed to read PowerPoint Document stream from '{}'", context.file_path))?;
	let slide_texts = collect_legacy_slide_texts(&ppt_document_stream);
	if slide_texts.is_empty() {
		anyhow::bail!("PPT file contains no slides");
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
				return Ok(bytes);
			}
		}
	}
	anyhow::bail!("PowerPoint Document stream not found")
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
	for chunk in data.chunks_exact(2) {
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

fn extract_slide_number(slide_name: &str) -> usize {
	slide_name.chars().filter(char::is_ascii_digit).collect::<String>().parse().unwrap_or(0)
}

fn extract_slide_title(root: Node) -> String {
	root.descendants()
		.filter(|node| node.node_type() == NodeType::Element && node.tag_name().name() == "sp")
		.find_map(|shape| {
			if is_title_shape(shape) {
				let text = collect_text_from_tagged_elements(shape, "t");
				let trimmed = text.trim();
				if trimmed.is_empty() { None } else { Some(trimmed.to_string()) }
			} else {
				None
			}
		})
		.unwrap_or_default()
}

fn is_title_shape(node: Node) -> bool {
	for child in node.descendants() {
		if child.node_type() == NodeType::Element
			&& child.tag_name().name() == "ph"
			&& let Some(ph_type) = child.attribute("type")
			&& (ph_type == "title" || ph_type == "ctrTitle")
		{
			return true;
		}
	}
	false
}

fn extract_slide_text(
	root: Node,
	links: &mut Vec<LinkInfo>,
	slide_start: usize,
	rels: &HashMap<String, String>,
) -> String {
	let mut text = String::new();
	traverse_for_text(root, &mut text, links, slide_start, rels);
	text
}

fn traverse_for_text(
	node: Node,
	text: &mut String,
	links: &mut Vec<LinkInfo>,
	slide_start: usize,
	rels: &HashMap<String, String>,
) {
	match node.node_type() {
		NodeType::Element => {
			let tag_name = node.tag_name().name();
			match tag_name {
				"t" => {
					if let Some(t) = node.text() {
						text.push_str(t);
					}
					return;
				}
				"br" => {
					text.push('\n');
					return;
				}
				"p" => {
					for child in node.children() {
						traverse_for_text(child, text, links, slide_start, rels);
					}
					if !text.ends_with('\n') {
						text.push('\n');
					}
					return;
				}
				"hlinkClick" => {
					if let Some(r_id) = node.attribute("id")
						&& let Some(link_target) = rels.get(r_id)
						&& let Some(parent) = node.parent()
					{
						let link_text = collect_text_from_tagged_elements(parent, "t");
						if !link_text.is_empty() {
							let link_offset = slide_start + text.len();
							text.push_str(&link_text);
							links.push(LinkInfo {
								offset: link_offset,
								text: link_text,
								reference: link_target.clone(),
							});
						}
					}
					return;
				}
				_ => {}
			}
		}
		NodeType::Text => return,
		_ => {}
	}
	for child in node.children() {
		traverse_for_text(child, text, links, slide_start, rels);
	}
}

#[cfg(test)]
mod tests {
	use std::collections::HashMap;

	use roxmltree::Document as XmlDocument;
	use rstest::rstest;

	use super::{
		extract_legacy_text, extract_slide_number, extract_slide_text, extract_slide_title, is_title_shape,
		normalize_legacy_slide_text, parse_cstring, parse_text_bytes_atom, parse_text_chars_atom,
	};

	#[rstest]
	#[case("ppt/slides/slide1.xml", 1)]
	#[case("ppt/slides/slide12.xml", 12)]
	#[case("slide007.xml", 7)]
	#[case("ppt/slides/custom.xml", 0)]
	fn extract_slide_number_parses_digits(#[case] name: &str, #[case] expected: usize) {
		assert_eq!(extract_slide_number(name), expected);
	}

	#[test]
	fn is_title_shape_true_for_title_and_center_title() {
		let xml = r#"
			<root>
				<sp><nvSpPr><nvPr><ph type="title" /></nvPr></nvSpPr></sp>
				<sp><nvSpPr><nvPr><ph type="ctrTitle" /></nvPr></nvSpPr></sp>
			</root>
		"#;
		let doc = XmlDocument::parse(xml).expect("xml parse");
		let shapes: Vec<_> = doc.descendants().filter(|n| n.tag_name().name() == "sp").collect();
		assert!(is_title_shape(shapes[0]));
		assert!(is_title_shape(shapes[1]));
	}

	#[test]
	fn is_title_shape_false_without_title_placeholder() {
		let xml = r#"<root><sp><nvSpPr><nvPr><ph type="body" /></nvPr></nvSpPr></sp></root>"#;
		let doc = XmlDocument::parse(xml).expect("xml parse");
		let shape = doc.descendants().find(|n| n.tag_name().name() == "sp").expect("shape");
		assert!(!is_title_shape(shape));
	}

	#[test]
	fn extract_slide_title_uses_first_non_empty_title_shape() {
		let xml = r#"
			<root>
				<sp><nvSpPr><nvPr><ph type="title" /></nvPr></nvSpPr><txBody><p><r><t>  </t></r></p></txBody></sp>
				<sp><nvSpPr><nvPr><ph type="title" /></nvPr></nvSpPr><txBody><p><r><t>Agenda</t></r></p></txBody></sp>
			</root>
		"#;
		let doc = XmlDocument::parse(xml).expect("xml parse");
		assert_eq!(extract_slide_title(doc.root()), "Agenda");
	}

	#[test]
	fn extract_slide_title_returns_empty_when_missing() {
		let xml = r#"<root><sp><txBody><p><r><t>Body text</t></r></p></txBody></sp></root>"#;
		let doc = XmlDocument::parse(xml).expect("xml parse");
		assert!(extract_slide_title(doc.root()).is_empty());
	}

	#[test]
	fn extract_slide_text_collects_paragraphs_and_breaks() {
		let xml = r#"
			<root>
				<p><r><t>Hello</t></r><br/><r><t>World</t></r></p>
				<p><r><t>Next</t></r></p>
			</root>
		"#;
		let doc = XmlDocument::parse(xml).expect("xml parse");
		let mut links = Vec::new();
		let rels = HashMap::new();
		let text = extract_slide_text(doc.root(), &mut links, 0, &rels);
		assert_eq!(text, "Hello\nWorld\nNext\n");
		assert!(links.is_empty());
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
