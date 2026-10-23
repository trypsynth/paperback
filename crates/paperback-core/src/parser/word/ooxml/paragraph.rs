use std::collections::HashMap;

use roxmltree::{Node, NodeType};

use super::{
	super::numbering::Numbering,
	notes::{Citations, NoteKind},
};
use crate::{
	document::{DocumentBuffer, Marker, MarkerType, format_marker_types},
	parser::util::{ooxml::collect_ooxml_run_text, toc::heading_level_to_marker_type, xml::find_child_element},
	types::HeadingInfo,
	util::text::display_len,
};

/// Where a document's pages come from.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum PageSource {
	/// Where Word last laid the pages out, which it records with `lastRenderedPageBreak`. Explicit breaks are not counted on top, because Word records one of these after each of those too.
	Rendered,
	/// The breaks written into the file, for one Word never laid out, such as a file from Google Docs, pandoc or python-docx: page breaks, paragraphs set to start a page, and section breaks that start a new page.
	Explicit,
}

impl PageSource {
	/// Rendered breaks when the document has any, since Word wrote them, and the explicit ones otherwise.
	pub(super) fn for_document(document_xml: &str) -> Self {
		if document_xml.contains("lastRenderedPageBreak") { Self::Rendered } else { Self::Explicit }
	}
}

/// Writes one paragraph into the buffer. Returns whether it is an item of a numbered or
/// bulleted list, so the caller can mark where the list it belongs to starts and ends.
#[allow(clippy::too_many_arguments)]
pub(super) fn process_paragraph(
	element: Node,
	buffer: &mut DocumentBuffer,
	headings: &mut Vec<HeadingInfo>,
	id_positions: &mut HashMap<String, usize>,
	rels: &HashMap<String, String>,
	style_heading_map: &HashMap<String, i32>,
	numbering: &mut Numbering,
	page_source: PageSource,
	citations: &mut Citations,
) -> bool {
	let paragraph_start = buffer.current_position();
	// Word keeps list numbers and bullets out of the text and generates them; without this a
	// numbered procedure reads as a run of unnumbered paragraphs.
	let label = numbering.label_for(find_child_element(element, "pPr"));
	let mut paragraph_text = label.as_ref().map_or_else(String::new, |label| format!("{} ", label.text));
	let mut para_display_len = display_len(&paragraph_text);
	let label_len = para_display_len;
	let mut page_starts: Vec<usize> = Vec::new();
	// Explicit breaks fall after their own text, so one at the end of the paragraph starts the page on the next line instead.
	let mut breaks_after: Vec<usize> = Vec::new();
	let mut next_starts_page = false;
	let mut bookmarks: Vec<(String, usize)> = Vec::new();
	let mut links: Vec<(usize, String, String)> = Vec::new();
	let mut heading_level = 0;
	let mut is_paragraph_style_heading = false;
	let mut format_spans: Vec<(MarkerType, usize, usize)> = Vec::new();
	// A hyperlink field's displayed text was already taken whole when its instruction was read, so its runs are skipped until the field ends rather than read a second time.
	let mut in_taken_field = false;
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
			if page_source == PageSource::Explicit {
				if find_child_element(child, "pageBreakBefore").is_some_and(is_on) {
					page_starts.push(0);
				}
				next_starts_page = find_child_element(child, "sectPr").is_some_and(section_starts_page);
			}
		} else if tag_name == "bookmarkStart" {
			if let Some(name) = child.attribute("name") {
				bookmarks.push((name.to_string(), para_display_len));
			}
		} else if tag_name == "hyperlink" {
			if child.children().any(|run| run.tag_name().name() == "r" && starts_page(run, page_source)) {
				page_starts.push(para_display_len);
			}
			para_display_len += process_hyperlink(child, &mut paragraph_text, &mut links, rels, para_display_len);
		} else if tag_name == "r" {
			match page_source {
				PageSource::Rendered if starts_page(child, page_source) => page_starts.push(para_display_len),
				PageSource::Explicit => {
					breaks_after.extend(page_break_offsets(child).map(|offset| para_display_len + offset));
				}
				PageSource::Rendered => {}
			}
			if in_taken_field {
				in_taken_field = find_child_element(child, "fldChar").and_then(|field| field.attribute("fldCharType"))
					!= Some("end");
				continue;
			}
			if heading_level == 0
				&& let Some(rpr_node) = find_child_element(child, "rPr")
			{
				heading_level = get_run_heading_level(rpr_node);
			}
			if let Some((kind, reference)) = NoteKind::cited_by(child) {
				// A custom mark is typed in the run itself, in place of the number Word would generate.
				let custom = matches!(reference.attribute("customMarkFollows"), Some("1" | "true" | "on"))
					.then(|| collect_ooxml_run_text(child).trim().to_string());
				let citation = citations.cite(kind, reference.attribute("id").unwrap_or_default(), custom);
				let cited = format!("[{}]", citation.label);
				links.push((para_display_len, cited.clone(), format!("#{}", citation.key)));
				para_display_len += display_len(&cited);
				paragraph_text.push_str(&cited);
				continue;
			}
			if let Some(instr_text_node) = find_child_element(child, "instrText")
				&& let Some(instruction) = instr_text_node.text()
				&& instruction.contains("HYPERLINK")
			{
				let link_target = parse_hyperlink_instruction(instruction);
				if !link_target.is_empty() {
					let (display_text, _) = extract_field_display_text(element, child);
					if !display_text.is_empty() {
						links.push((para_display_len, display_text.clone(), link_target));
						paragraph_text.push_str(&display_text);
						para_display_len += display_len(&display_text);
						in_taken_field = true;
					}
				}
			}
			let run_text = collect_ooxml_run_text(child);
			if !run_text.is_empty() {
				let run_start = para_display_len;
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
	let trimmed_len = display_len(trimmed);
	// Offsets were taken in the untrimmed text, so they move back by what the trim removed, and never out of this paragraph.
	let in_trimmed = |offset: usize| paragraph_start + offset.saturating_sub(leading_trim).min(trimmed_len);
	// A page or bookmark that starts on the paragraph's first words starts on the paragraph, not after its list label.
	let at_words = |offset: usize| if offset <= label_len { paragraph_start } else { in_trimmed(offset) };
	for offset in page_starts {
		buffer.add_marker(Marker::new(MarkerType::PageBreak, at_words(offset)));
	}
	let next_line = buffer.current_position();
	for offset in breaks_after {
		let position = if offset.saturating_sub(leading_trim) >= trimmed_len { next_line } else { at_words(offset) };
		buffer.add_marker(Marker::new(MarkerType::PageBreak, position));
	}
	if next_starts_page {
		buffer.add_marker(Marker::new(MarkerType::PageBreak, next_line));
	}
	for (name, offset) in bookmarks {
		id_positions.insert(name, at_words(offset));
	}
	for (offset, text, reference) in links {
		buffer.add_marker(Marker::new(MarkerType::Link, in_trimmed(offset)).with_text(text).with_reference(reference));
	}
	for (kind, start, end) in format_spans {
		let adj_start = in_trimmed(start);
		let adj_end = in_trimmed(end);
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
	// A numbered heading keeps its number but is a heading, not a list item.
	let Some(label) = label.filter(|_| heading_level == 0 && !trimmed.is_empty()) else { return false };
	buffer.add_marker(
		Marker::new(MarkerType::ListItem, paragraph_start).with_text(trimmed.to_string()).with_level(label.level),
	);
	true
}

/// Whether a page begins at `run`.
///
/// A .docx has no pages of its own: Word paginates as it lays the document out, and saves where each page after the first began with `lastRenderedPageBreak`, so for a file Word laid out that is the closest it comes to Word's own page numbers.
pub(super) fn starts_page(run: Node, page_source: PageSource) -> bool {
	match page_source {
		PageSource::Rendered => find_child_element(run, "lastRenderedPageBreak").is_some(),
		PageSource::Explicit => page_break_offsets(run).next().is_some(),
	}
}

/// Where in `run`'s text each explicit page break leaves off, counted in display units the way [`collect_ooxml_run_text`] writes the run, the break's own newline included.
fn page_break_offsets(run: Node) -> impl Iterator<Item = usize> {
	let mut offset = 0;
	run.children().filter(|child| child.node_type() == NodeType::Element).filter_map(move |child| {
		match child.tag_name().name() {
			"t" => offset += child.text().map_or(0, display_len),
			"tab" => offset += 1,
			"br" => {
				offset += 1;
				if child.attribute("type") == Some("page") {
					return Some(offset);
				}
			}
			_ => {}
		}
		None
	})
}

/// Whether an on-off property such as `<w:pageBreakBefore/>` is on: present with no value, or with one that is not off.
fn is_on(property: Node) -> bool {
	!matches!(property.attribute("val"), Some("0" | "false" | "off"))
}

/// Whether a section ending at this paragraph starts the next one on a new page, which every kind but a continuous section does, including one that names no kind.
fn section_starts_page(section: Node) -> bool {
	find_child_element(section, "type").and_then(|kind| kind.attribute("val")) != Some("continuous")
}

/// Appends the hyperlink's display text to `paragraph_text`, records the link at `offset` into the paragraph, and returns the number of display units appended.
fn process_hyperlink(
	element: Node,
	paragraph_text: &mut String,
	links: &mut Vec<(usize, String, String)>,
	rels: &HashMap<String, String>,
	offset: usize,
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
	paragraph_text.push_str(&link_text);
	if !link_target.is_empty() {
		links.push((offset, link_text.clone(), link_target));
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
						&& level > 0
						&& level <= MAX_HEADING_LEVEL
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
