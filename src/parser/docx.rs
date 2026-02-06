use std::{collections::HashMap, fs::File, io::BufReader};

use anyhow::{Context, Result};
use roxmltree::{Document as XmlDocument, Node, NodeType};
use zip::ZipArchive;

use crate::{
	document::{Document, DocumentBuffer, Marker, MarkerType, ParserContext, ParserFlags},
	parser::{
		Parser,
		ooxml::{collect_ooxml_run_text, read_ooxml_relationships},
		path::extract_title_from_path,
		toc::{build_toc_from_buffer, heading_level_to_marker_type},
		xml::find_child_element,
	},
	types::HeadingInfo,
	zip::read_zip_entry_by_name,
};

pub struct DocxParser;

impl Parser for DocxParser {
	fn name(&self) -> &'static str {
		"Word Documents"
	}

	fn extensions(&self) -> &[&str] {
		&["docx", "docm"]
	}

	fn supported_flags(&self) -> ParserFlags {
		ParserFlags::SUPPORTS_TOC
	}

	fn parse(&self, context: &ParserContext) -> Result<Document> {
		let file = File::open(&context.file_path)
			.with_context(|| format!("Failed to open DOCX file '{}'", context.file_path))?;
		let mut archive = ZipArchive::new(BufReader::new(file))
			.with_context(|| format!("Failed to read DOCX as zip '{}'", context.file_path))?;
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
