use std::{collections::HashMap, fs, fs::File, io::BufReader};

use anyhow::{Context, Result};
use roxmltree::{Document as XmlDocument, Node, NodeType};
use zip::ZipArchive;

use crate::{
	document::{Document, DocumentBuffer, Marker, MarkerType, ParserContext, ParserFlags},
	parser::{
		Parser,
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
	fn name(&self) -> &'static str {
		"OpenDocument Text Files"
	}

	fn extensions(&self) -> &[&str] {
		&["odt"]
	}

	fn supported_flags(&self) -> ParserFlags {
		ParserFlags::SUPPORTS_TOC
	}

	fn parse(&self, context: &ParserContext) -> Result<Document> {
		let file = File::open(&context.file_path)
			.with_context(|| format!("Failed to open ODT file '{}'", context.file_path))?;
		let mut archive = ZipArchive::new(BufReader::new(file))
			.with_context(|| format!("Failed to read ODT as zip '{}'", context.file_path))?;
		let content_str = read_zip_entry_by_name(&mut archive, "content.xml")
			.context("ODT file does not contain content.xml or it is empty")?;
		let xml_doc = XmlDocument::parse(&content_str).context("Invalid ODT content.xml")?;
		let mut buffer = DocumentBuffer::new();
		let mut id_positions = HashMap::new();
		traverse(xml_doc.root(), &mut buffer, &mut id_positions, context.render_tables_inline);
		let title = extract_title_from_path(&context.file_path);
		let toc_items = build_toc_from_buffer(&buffer);
		let mut document = Document::new().with_title(title);
		document.set_buffer(buffer);
		document.id_positions = id_positions;
		document.toc_items = toc_items;
		Ok(document)
	}
}

pub struct FodtParser;

impl Parser for FodtParser {
	fn name(&self) -> &'static str {
		"Flat OpenDocument Text Files"
	}

	fn extensions(&self) -> &[&str] {
		&["fodt"]
	}

	fn supported_flags(&self) -> ParserFlags {
		ParserFlags::SUPPORTS_TOC
	}

	fn parse(&self, context: &ParserContext) -> Result<Document> {
		let content_str = fs::read_to_string(&context.file_path)
			.with_context(|| format!("Failed to open FODT file '{}'", context.file_path))?;
		let xml_doc = XmlDocument::parse(&content_str).context("Invalid FODT document")?;
		let mut buffer = DocumentBuffer::new();
		let mut id_positions = HashMap::new();
		traverse(xml_doc.root(), &mut buffer, &mut id_positions, context.render_tables_inline);
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
	id_positions: &mut HashMap<String, usize>,
	render_tables_inline: bool,
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
			traverse_children(node, buffer, id_positions, render_tables_inline);
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
	traverse_children(node, buffer, id_positions, render_tables_inline);
}

fn traverse_children(
	node: Node,
	buffer: &mut DocumentBuffer,
	id_positions: &mut HashMap<String, usize>,
	render_tables_inline: bool,
) {
	for child in node.children() {
		traverse(child, buffer, id_positions, render_tables_inline);
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
	let mut html_content = String::from("<table border=\"1\">");
	let mut table_caption = String::new();
	let mut found_first_row = false;
	let mut has_content = false;
	// Build the table HTML and caption from the XML nodes directly. Cell text is collected via
	// `collect_element_text` (operating on the XML tree), NOT by slicing the display buffer — the
	// display buffer is indexed in display units, so slicing it with those offsets as byte indices
	// mis-sliced (and could panic) on non-ASCII cell content.
	for child in node.children() {
		if child.is_element() && child.tag_name().name() == "table-row" {
			html_content.push_str("<tr>");
			for cell in child.children() {
				if cell.is_element() && cell.tag_name().name() == "table-cell" {
					let cell_text = collect_element_text(cell);
					if !cell_text.trim().is_empty() {
						has_content = true;
					}
					if !found_first_row {
						table_caption.push_str(cell_text.trim());
						table_caption.push(' ');
					}
					html_content.push_str("<td>");
					html_content.push_str(&cell_text.replace('\n', "<br/>"));
					html_content.push_str("</td>");
				}
			}
			html_content.push_str("</tr>");
			found_first_row = true;
		}
	}
	html_content.push_str("</table>");
	if !has_content {
		return;
	}
	let marker_text =
		if table_caption.trim().is_empty() { "table".to_string() } else { table_caption.trim().to_string() };
	let display_text = crate::parser::table_text::html_table_to_display(&html_content, render_tables_inline);
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
mod tests {
	use std::collections::HashMap;

	use roxmltree::Document as XmlDocument;

	use super::traverse;
	use crate::{
		document::{DocumentBuffer, MarkerType},
		util::text::display_len,
	};

	/// OFF mode: an ODT table emits a `"[Table]: <first row>"` placeholder. The second cell holds a
	/// non-ASCII character (U+1D11E, G Clef, non-BMP) to prove the cell-text extraction no longer
	/// mis-slices the display buffer with display-unit offsets as byte indices.
	#[test]
	fn odt_table_emits_placeholder_when_off() {
		let xml = "<document><table><table-row><table-cell>Kop</table-cell><table-cell>\u{1D11E}</table-cell></table-row></table></document>";
		let xml_doc = XmlDocument::parse(xml).expect("valid xml");
		let mut buffer = DocumentBuffer::new();
		let mut id_positions = HashMap::new();
		traverse(xml_doc.root(), &mut buffer, &mut id_positions, false);

		assert_eq!(buffer.content, "[Table]: Kop \u{1D11E}\n");
		let table_marker = buffer.markers.iter().find(|m| m.mtype == MarkerType::Table).expect("Table marker");
		assert_eq!(table_marker.position, 0, "marker starts at buffer start");
		assert_eq!(table_marker.length, display_len("[Table]: Kop \u{1D11E}") + 1, "marker length in display units");
		assert_eq!(table_marker.text, "Kop \u{1D11E}", "marker keeps the first-row caption");
		assert!(table_marker.reference.contains("<table"), "marker reference is the table HTML");
	}

	/// An `id` attribute on an element nested inside a table cell must be registered in
	/// `id_positions` at the table's start position, so internal links to that anchor navigate to
	/// the table. Holds in both OFF and ON modes (registration happens before the cells collapse).
	#[test]
	fn odt_table_cell_id_registered_at_table_start() {
		let xml = "<document><p>before</p><table><table-row><table-cell><span id=\"anchor1\">Kop</span></table-cell></table-row></table></document>";
		let xml_doc = XmlDocument::parse(xml).expect("valid xml");
		for inline in [false, true] {
			let mut buffer = DocumentBuffer::new();
			let mut id_positions = HashMap::new();
			traverse(xml_doc.root(), &mut buffer, &mut id_positions, inline);
			let table_marker = buffer.markers.iter().find(|m| m.mtype == MarkerType::Table).expect("Table marker");
			assert_eq!(
				id_positions.get("anchor1"),
				Some(&table_marker.position),
				"in-cell anchor id maps to the table start (inline={inline})"
			);
		}
	}

	/// ON mode: the same ODT table emits the full TSV instead of the placeholder.
	#[test]
	fn odt_table_emits_tsv_when_inline() {
		let xml = "<document><table><table-row><table-cell>Kop</table-cell><table-cell>\u{1D11E}</table-cell></table-row></table></document>";
		let xml_doc = XmlDocument::parse(xml).expect("valid xml");
		let mut buffer = DocumentBuffer::new();
		let mut id_positions = HashMap::new();
		traverse(xml_doc.root(), &mut buffer, &mut id_positions, true);

		assert_eq!(buffer.content, "Kop\t\u{1D11E}\n");
		let table_marker = buffer.markers.iter().find(|m| m.mtype == MarkerType::Table).expect("Table marker");
		assert_eq!(table_marker.length, display_len("Kop\t\u{1D11E}") + 1, "marker length spans the TSV");
	}
}
