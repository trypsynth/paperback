use std::collections::HashMap;

use roxmltree::{Node, NodeType};

use crate::{
	document::{DocumentBuffer, Marker, MarkerType, format_marker_types},
	parser::util::{ooxml::collect_ooxml_run_text, toc::heading_level_to_marker_type, xml::find_child_element},
	types::HeadingInfo,
	util::text::display_len,
};

pub(super) fn process_paragraph(
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

pub(super) fn extract_number_from_string(s: &str) -> Option<i32> {
	let digits: String = s.chars().filter(char::is_ascii_digit).collect();
	digits.parse().ok()
}
