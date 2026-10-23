use std::collections::HashMap;

use anyhow::{Context, Result};
use roxmltree::{Document as XmlDocument, Node};

use crate::{
	document::{Document, DocumentBuffer, Marker, MarkerType, ParserContext, format_marker_types},
	parser::{
		Parser,
		convert::table_text::{build_html_table_from_grid, html_table_to_display, table_caption_from_html},
		odf_crypto::read_odf_content,
		util::{
			comments::{self, Comment},
			odf,
			path::extract_title_from_path,
			toc::{build_toc_from_buffer, heading_level_to_marker_type},
			xml::read_xml_to_string,
		},
	},
	t,
	util::text::display_len,
};

pub struct OdtParser;

impl Parser for OdtParser {
	fn parse(&self, context: &ParserContext) -> Result<Document> {
		tracing::debug!(path = %context.file_path, "parsing odt file");
		// No `.context()` here: it would wrap the `[password_required]` sentinel an encrypted file returns, and the layers above match on that sentinel at the top of the error.
		let content_str = read_odf_content(&context.file_path, "content.xml", context.password.as_deref())?;
		let xml_doc = XmlDocument::parse(&content_str).context("Invalid ODT content.xml")?;
		let format_style_map = build_odt_format_style_map(xml_doc.root());
		let mut buffer = DocumentBuffer::new();
		let mut id_positions = HashMap::new();
		traverse(xml_doc.root(), &mut buffer, &mut id_positions, context.render_tables_inline, &format_style_map);
		let (title, author) = odf::packaged_metadata(&context.file_path, context.password.as_deref());
		let title = title.unwrap_or_else(|| extract_title_from_path(&context.file_path));
		let toc_items = build_toc_from_buffer(&buffer);
		let mut document = Document::new().with_title(title).with_author(author.unwrap_or_default());
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
		let content_str = read_xml_to_string(&context.file_path)
			.with_context(|| format!("Failed to open FODT file '{}'", context.file_path))?;
		let xml_doc = XmlDocument::parse(&content_str).context("Invalid FODT document")?;
		let format_style_map = build_odt_format_style_map(xml_doc.root());
		let mut buffer = DocumentBuffer::new();
		let mut id_positions = HashMap::new();
		traverse(xml_doc.root(), &mut buffer, &mut id_positions, context.render_tables_inline, &format_style_map);
		let (title, author) = odf::metadata(&xml_doc);
		let title = title.unwrap_or_else(|| extract_title_from_path(&context.file_path));
		let toc_items = build_toc_from_buffer(&buffer);
		let mut document = Document::new().with_title(title).with_author(author.unwrap_or_default());
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

/// Writes the content under `root` into `buffer`, then the footnotes and endnotes it cites, each where its citation links to.
fn traverse(
	root: Node,
	buffer: &mut DocumentBuffer,
	id_positions: &mut HashMap<String, usize>,
	render_tables_inline: bool,
	format_style_map: &HashMap<String, (bool, bool, bool)>,
) {
	let mut walk =
		Walk { buffer, id_positions, render_tables_inline, format_style_map, notes: Vec::new(), comments: Vec::new() };
	walk.node(root);
	walk.write_notes();
	walk.write_comments();
}

/// What a walk of an ODT's content writes into and consults.
struct Walk<'w, 'a, 'input> {
	buffer: &'w mut DocumentBuffer,
	id_positions: &'w mut HashMap<String, usize>,
	render_tables_inline: bool,
	format_style_map: &'w HashMap<String, (bool, bool, bool)>,
	/// The notes cited so far, by the id their citation links to, with their number and body. Read in the middle of a sentence they interrupted it, so they are written after the text instead.
	notes: Vec<(String, String, Option<Node<'a, 'input>>)>,
	/// The comments met so far, each with the name its range's closing `office:annotation-end` uses, when it covers words.
	comments: Vec<(Comment, Option<String>)>,
}

impl<'a, 'input> Walk<'_, 'a, 'input> {
	fn node(&mut self, node: Node<'a, 'input>) {
		if node.is_text() {
			self.buffer.append(node.text().unwrap_or_default());
			return;
		}
		if !node.is_element() {
			self.children(node);
			return;
		}
		if let Some(spaces) = odf::spacing(node) {
			self.buffer.append(&spaces);
			return;
		}
		let tag_name = node.tag_name().name();
		if tag_name == "annotation" {
			self.keep_comment(node);
			return;
		}
		if tag_name == "annotation-end" {
			let position = self.buffer.current_position();
			if let Some((comment, _)) =
				self.comments.iter_mut().rev().find(|(_, name)| name.as_deref() == node.attribute("name"))
			{
				comment.end = Some(position);
			}
			return;
		}
		if odf::is_hidden(node) {
			return;
		}
		if tag_name == "h" {
			let level = node.attribute("outline-level").and_then(|s| s.parse::<i32>().ok()).unwrap_or(1);
			let heading_offset = self.buffer.current_position();
			let heading_text = odf::plain_text(node);
			if !heading_text.is_empty() {
				self.buffer.append(&heading_text);
				self.buffer.append("\n");
				let marker_type = heading_level_to_marker_type(level);
				self.buffer
					.add_marker(Marker::new(marker_type, heading_offset).with_text(heading_text).with_level(level));
			}
			self.keep_notes_inside(node);
			return;
		}
		if tag_name == "p" {
			self.children(node);
			self.buffer.append("\n");
			return;
		}
		if tag_name == "a" {
			if let Some(href) = node.attribute("href") {
				let link_offset = self.buffer.current_position();
				let link_text = odf::plain_text(node);
				if !link_text.is_empty() {
					self.buffer.append(&link_text);
					self.buffer.add_marker(
						Marker::new(MarkerType::Link, link_offset)
							.with_text(link_text)
							.with_reference(href.to_string()),
					);
				}
			}
			self.keep_notes_inside(node);
			return;
		}
		if tag_name == "note" {
			let id = node.attribute("id").map_or_else(|| format!("note-{}", self.notes.len() + 1), str::to_string);
			let citation = odf::note_citation(node);
			let cited = format!("[{citation}]");
			let offset = self.buffer.current_position();
			self.buffer.append(&cited);
			self.buffer
				.add_marker(Marker::new(MarkerType::Link, offset).with_text(cited).with_reference(format!("#{id}")));
			let body = node.children().find(|child| child.is_element() && child.tag_name().name() == "note-body");
			self.notes.push((id, citation, body));
			return;
		}
		if let Some(id) = node.attribute("id") {
			self.id_positions.insert(id.to_string(), self.buffer.current_position());
		}
		if let Some((placeholder, description)) = odf::picture(node) {
			let offset = self.buffer.current_position();
			self.buffer.append(&placeholder);
			self.buffer.add_marker(
				Marker::new(MarkerType::Image, offset).with_text(description).with_length(display_len(&placeholder)),
			);
			return;
		}
		if tag_name == "span"
			&& let Some(style_name) = node.attribute("style-name")
			&& let Some(&(bold, italic, underline)) = self.format_style_map.get(style_name)
			&& (bold || italic || underline)
		{
			let start = self.buffer.current_position();
			self.children(node);
			let end = self.buffer.current_position();
			if end > start {
				for kind in format_marker_types(bold, italic, underline) {
					self.buffer.add_marker(Marker::new(kind, start).with_length(end - start));
				}
			}
			return;
		}
		if tag_name == "table" {
			process_table(node, self.buffer, self.id_positions, self.render_tables_inline);
			self.keep_notes_inside(node);
			return;
		}
		self.children(node);
	}

	fn children(&mut self, node: Node<'a, 'input>) {
		for child in node.children() {
			self.node(child);
		}
	}

	/// Keeps the notes cited inside an element that is read as plain text, such as a heading or a table, whose citations show as text rather than links, so their bodies are still written with the rest.
	fn keep_notes_inside(&mut self, node: Node<'a, 'input>) {
		for note in
			node.descendants().filter(|descendant| descendant.is_element() && descendant.tag_name().name() == "note")
		{
			let id = note.attribute("id").map_or_else(|| format!("note-{}", self.notes.len() + 1), str::to_string);
			let body = note.children().find(|child| child.is_element() && child.tag_name().name() == "note-body");
			self.notes.push((id, odf::note_citation(note), body));
		}
	}

	/// Writes the notes under a heading of their own after the text, each where its citation's link lands. A note can cite another, so this runs until none are left.
	fn keep_comment(&mut self, annotation: Node) {
		let child_text = |name: &str| {
			annotation
				.children()
				.filter(|child| child.is_element() && child.tag_name().name() == name)
				.map(|child| odf::plain_text(child).trim().to_string())
				.filter(|text| !text.is_empty())
				.collect::<Vec<_>>()
		};
		let text = child_text("p").join(" ");
		if text.is_empty() {
			return;
		}
		let key = format!("comment-{}", self.comments.len() + 1);
		let start = self.buffer.current_position();
		self.id_positions.insert(key.clone(), start);
		let author = child_text("creator").join(" ");
		let name = annotation.attribute("name").map(str::to_string);
		self.comments.push((Comment { key, author, text, start, end: None }, name));
	}

	fn write_comments(&mut self) {
		let comments = std::mem::take(&mut self.comments).into_iter().map(|(comment, _)| comment).collect();
		comments::write_comments(self.buffer, comments);
	}

	fn write_notes(&mut self) {
		if self.notes.is_empty() {
			return;
		}
		if !self.buffer.content.is_empty() && !self.buffer.content.ends_with('\n') {
			self.buffer.append("\n");
		}
		// TRANSLATORS: Heading over the footnotes and endnotes of a document, written after its text
		let heading = t("Notes");
		let heading_offset = self.buffer.current_position();
		self.buffer.append(&heading);
		self.buffer.append("\n");
		self.buffer
			.add_marker(Marker::new(heading_level_to_marker_type(1), heading_offset).with_text(heading).with_level(1));
		while !self.notes.is_empty() {
			for (id, citation, body) in std::mem::take(&mut self.notes) {
				self.id_positions.insert(id, self.buffer.current_position());
				self.buffer.append(&format!("{citation}. "));
				if let Some(body) = body {
					self.children(body);
				}
				if !self.buffer.content.ends_with('\n') {
					self.buffer.append("\n");
				}
			}
		}
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
					let cell_text = odf::plain_text(cell);
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
