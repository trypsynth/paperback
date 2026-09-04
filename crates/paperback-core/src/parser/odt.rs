use std::{collections::HashMap, fs, fs::File, io::BufReader};

use anyhow::{Context, Result};
use roxmltree::{Document as XmlDocument, Node, NodeType};
use zip::ZipArchive;

use crate::{
	document::{Document, DocumentBuffer, Marker, MarkerType, ParserContext, format_marker_types},
	parser::{
		Parser,
		convert::table_text::{build_html_table_from_grid, html_table_to_display, table_caption_from_html},
		util::{
			path::extract_title_from_path,
			toc::{build_toc_from_buffer, heading_level_to_marker_type},
			xml::collect_element_text,
		},
	},
	util::zip::read_zip_entry_by_name,
};

pub struct OdtParser;

impl Parser for OdtParser {
	fn parse(&self, context: &ParserContext) -> Result<Document> {
		tracing::debug!(path = %context.file_path, "parsing odt file");
		let file = File::open(&context.file_path)
			.with_context(|| format!("Failed to open ODT file '{}'", context.file_path))?;
		let mut archive = ZipArchive::new(BufReader::new(file))
			.with_context(|| format!("Failed to read ODT as zip '{}'", context.file_path))?;
		let content_str = read_zip_entry_by_name(&mut archive, "content.xml")
			.context("ODT file does not contain content.xml or it is empty")?;
		let xml_doc = XmlDocument::parse(&content_str).context("Invalid ODT content.xml")?;
		let format_style_map = build_odt_format_style_map(xml_doc.root());
		let mut buffer = DocumentBuffer::new();
		let mut id_positions = HashMap::new();
		traverse(xml_doc.root(), &mut buffer, &mut id_positions, context.render_tables_inline, &format_style_map);
		let title = extract_title_from_path(&context.file_path);
		let toc_items = build_toc_from_buffer(&buffer);
		let mut document = Document::new().with_title(title);
		document.set_buffer(buffer);
		document.id_positions = id_positions;
		document.toc_items = toc_items;
		tracing::debug!(path = %context.file_path, "parsed odt file successfully");
		Ok(document)
	}
}

pub struct FodtParser;

impl Parser for FodtParser {
	fn parse(&self, context: &ParserContext) -> Result<Document> {
		tracing::debug!(path = %context.file_path, "parsing fodt file");
		let content_str = fs::read_to_string(&context.file_path)
			.with_context(|| format!("Failed to open FODT file '{}'", context.file_path))?;
		let xml_doc = XmlDocument::parse(&content_str).context("Invalid FODT document")?;
		let format_style_map = build_odt_format_style_map(xml_doc.root());
		let mut buffer = DocumentBuffer::new();
		let mut id_positions = HashMap::new();
		traverse(xml_doc.root(), &mut buffer, &mut id_positions, context.render_tables_inline, &format_style_map);
		let title = extract_title_from_path(&context.file_path);
		let toc_items = build_toc_from_buffer(&buffer);
		let mut document = Document::new().with_title(title);
		document.set_buffer(buffer);
		document.id_positions = id_positions;
		document.toc_items = toc_items;
		tracing::debug!(path = %context.file_path, "parsed fodt file successfully");
		Ok(document)
	}
}

/// Builds a style-name → `(bold, italic, underline)` map from `<office:automatic-styles>` /
/// `<style:style style:family="text">` entries, so that `<text:span text:style-name="...">`
/// elements encountered during traversal can be resolved to direct character formatting.
fn build_odt_format_style_map(root: Node) -> HashMap<String, (bool, bool, bool)> {
	let mut map = HashMap::new();
	let Some(automatic_styles) =
		root.descendants().find(|n| n.is_element() && n.tag_name().name() == "automatic-styles")
	else {
		return map;
	};
	for style_node in automatic_styles.children() {
		if !style_node.is_element() || style_node.tag_name().name() != "style" {
			continue;
		}
		if style_node.attribute("family") != Some("text") {
			continue;
		}
		let Some(name) = style_node.attribute("name") else { continue };
		let Some(text_props) =
			style_node.children().find(|n| n.is_element() && n.tag_name().name() == "text-properties")
		else {
			continue;
		};
		let bold = text_props.attribute("font-weight").is_some_and(|v| v == "bold");
		let italic = text_props.attribute("font-style").is_some_and(|v| v == "italic");
		let underline = text_props.attribute("text-underline-style").is_some_and(|v| v != "none");
		if bold || italic || underline {
			map.insert(name.to_string(), (bold, italic, underline));
		}
	}
	map
}

fn traverse(
	node: Node,
	buffer: &mut DocumentBuffer,
	id_positions: &mut HashMap<String, usize>,
	render_tables_inline: bool,
	format_style_map: &HashMap<String, (bool, bool, bool)>,
) {
	if node.node_type() == NodeType::Element {
		let tag_name = node.tag_name().name();
		if tag_name == "h" {
			let level = node.attribute("outline-level").and_then(|s| s.parse::<i32>().ok()).unwrap_or(1);
			let heading_offset = buffer.current_position();
			let heading_text = collect_element_text(node);
			if !heading_text.is_empty() {
				buffer.append(&heading_text);
				buffer.append("\n");
				let marker_type = heading_level_to_marker_type(level);
				buffer.add_marker(Marker::new(marker_type, heading_offset).with_text(heading_text).with_level(level));
			}
			return; // Don't traverse children, we already got the text
		}
		if tag_name == "p" {
			traverse_children(node, buffer, id_positions, render_tables_inline, format_style_map);
			buffer.append("\n");
			return;
		}
		if tag_name == "a" {
			if let Some(href) = node.attribute("href") {
				let link_offset = buffer.current_position();
				let link_text = collect_element_text(node);
				if !link_text.is_empty() {
					buffer.append(&link_text);
					buffer.add_marker(
						Marker::new(MarkerType::Link, link_offset)
							.with_text(link_text)
							.with_reference(href.to_string()),
					);
				}
			}
			return; // Don't traverse children, we already got the text
		}
		if let Some(id) = node.attribute("id") {
			id_positions.insert(id.to_string(), buffer.current_position());
		}
		if tag_name == "span"
			&& let Some(style_name) = node.attribute("style-name")
			&& let Some(&(bold, italic, underline)) = format_style_map.get(style_name)
			&& (bold || italic || underline)
		{
			let start = buffer.current_position();
			traverse_children(node, buffer, id_positions, render_tables_inline, format_style_map);
			let end = buffer.current_position();
			if end > start {
				for kind in format_marker_types(bold, italic, underline) {
					buffer.add_marker(Marker::new(kind, start).with_length(end - start));
				}
			}
			return;
		}
		if tag_name == "table" {
			process_table(node, buffer, id_positions, render_tables_inline);
			return;
		}
	} else if node.node_type() == NodeType::Text {
		if let Some(text) = node.text() {
			buffer.append(text);
		}
		return;
	}
	traverse_children(node, buffer, id_positions, render_tables_inline, format_style_map);
}

fn traverse_children(
	node: Node,
	buffer: &mut DocumentBuffer,
	id_positions: &mut HashMap<String, usize>,
	render_tables_inline: bool,
	format_style_map: &HashMap<String, (bool, bool, bool)>,
) {
	for child in node.children() {
		traverse(child, buffer, id_positions, render_tables_inline, format_style_map);
	}
}

fn process_table(
	node: Node,
	buffer: &mut DocumentBuffer,
	id_positions: &mut HashMap<String, usize>,
	render_tables_inline: bool,
) {
	let table_start = buffer.current_position();
	// The table collapses to a placeholder/TSV, so cells have no individual display offset. Register
	// every anchor `id` nested inside the table at the table's start position; internal links to a
	// bookmark/footnote/cross-ref target inside a cell then navigate to the table.
	for descendant in node.descendants() {
		if descendant.is_element()
			&& let Some(id) = descendant.attribute("id")
		{
			id_positions.insert(id.to_string(), table_start);
		}
	}
	let mut rows: Vec<Vec<String>> = Vec::new();
	let mut has_content = false;
	// Build the table grid from the XML nodes directly. Cell text is collected via
	// `collect_element_text` (operating on the XML tree), NOT by slicing the display buffer. The
	// display buffer is indexed in display units, so slicing it with those offsets as byte indices
	// mis-sliced (and could panic) on non-ASCII cell content.
	for child in node.children() {
		if child.is_element() && child.tag_name().name() == "table-row" {
			let mut cells: Vec<String> = Vec::new();
			for cell in child.children() {
				if cell.is_element() && cell.tag_name().name() == "table-cell" {
					let cell_text = collect_element_text(cell);
					if !cell_text.trim().is_empty() {
						has_content = true;
					}
					cells.push(cell_text.trim().to_string());
				}
			}
			rows.push(cells);
		}
	}
	if !has_content {
		tracing::debug!("dropped table with no non-blank cell content");
		return;
	}
	let html_content = build_html_table_from_grid(&rows);
	let marker_text = table_caption_from_html(&html_content).unwrap_or_else(|| "table".to_string());
	let display_text = html_table_to_display(&html_content, render_tables_inline);
	buffer.append(&display_text);
	buffer.append("\n");
	let display_len = buffer.current_position() - table_start;
	buffer.add_marker(
		Marker::new(MarkerType::Table, table_start)
			.with_text(marker_text)
			.with_reference(html_content)
			.with_length(display_len),
	);
}

#[cfg(test)]
mod tests;
