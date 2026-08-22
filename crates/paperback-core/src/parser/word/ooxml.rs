//! Modern OOXML `.docx` parsing (including the batch-`.zip`-of-`.docx` and single-file
//! password-protected cases), plus [`try_decrypt_office_file`], the shared OLE-container
//! decryption check reused by `powerpoint`'s `.pptx` parsing.

use std::{
	collections::HashMap,
	fs::File,
	io::{BufReader, Cursor, Read, Seek},
};

use anyhow::{Context, Result};
use cfb::CompoundFile;
use office_crypto::decrypt_from_file;
use roxmltree::{Document as XmlDocument, Node, NodeType};
use zip::ZipArchive;

use crate::{
	document::{Document, DocumentBuffer, Marker, MarkerType, ParserContext, format_marker_types},
	parser::{
		PASSWORD_REQUIRED_ERROR_PREFIX,
		convert::table_text::{build_html_table_from_grid, html_table_to_display, table_caption_from_html},
		util::{
			ooxml::{collect_ooxml_run_text, read_ooxml_relationships},
			path::extract_title_from_path,
			toc::{build_toc_from_buffer, heading_level_to_marker_type},
			xml::find_child_element,
		},
	},
	t,
	types::HeadingInfo,
	util::{text::display_len, zip::read_zip_entry_by_name},
};

pub(super) fn parse_word_zip(context: &ParserContext, render_tables_inline: bool) -> Result<Document> {
	let file =
		File::open(&context.file_path).with_context(|| format!("Failed to open ZIP file '{}'", context.file_path))?;
	let mut archive = ZipArchive::new(BufReader::new(file))
		.with_context(|| format!("Failed to read ZIP archive '{}'", context.file_path))?;

	tracing::debug!(path = %context.file_path, entries = archive.len(), "scanning zip archive for embedded docx entries");

	let mut docx_names: Vec<String> =
		archive.file_names().filter(|name| name.to_ascii_lowercase().ends_with(".docx")).map(String::from).collect();

	if docx_names.is_empty() {
		tracing::warn!(path = %context.file_path, "no docx entries found in zip archive");
		// TRANSLATORS: Error shown when a ZIP file contains no readable Word document content
		anyhow::bail!(t("No readable content found in the ZIP archive"));
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
	tracing::debug!(path = %context.file_path, documents = docx_names.len(), "extracted docx documents from zip batch");
	Ok(document)
}

pub(super) fn parse_ooxml_doc(context: &ParserContext, render_tables_inline: bool) -> Result<Document> {
	tracing::debug!(path = %context.file_path, "parsing ooxml document");
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
	tracing::debug!(path = %context.file_path, "parsed ooxml document successfully");
	Ok(document)
}

/// Read a DOCX/OOXML file's raw bytes, decrypting first if the file is an encrypted OLE container.
fn load_ooxml_bytes(path: &str, password: Option<&str>) -> Result<Vec<u8>> {
	try_decrypt_office_file(path, password)?
		.map_or_else(|| std::fs::read(path).with_context(|| format!("Failed to read '{path}'")), Ok)
}

fn parse_ooxml_from_archive<R: Read + Seek>(
	archive: &mut ZipArchive<R>,
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
fn build_style_heading_map<R: Read + Seek>(archive: &mut ZipArchive<R>) -> HashMap<String, i32> {
	let mut map = HashMap::new();
	let Ok(content) = read_zip_entry_by_name(archive, "word/styles.xml") else {
		tracing::debug!("word/styles.xml not present, skipping style based heading detection");
		return map;
	};
	let Ok(xml) = XmlDocument::parse(&content) else {
		tracing::warn!("word/styles.xml present but failed to parse, skipping style based heading detection");
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
	let html_content = build_html_table_from_grid(&rows);
	// Derive the caption the same way HTML/XML do (first-row text, no prefix) for consistent
	// labels across formats; fall back to "table" for an empty table like `table_caption_from_tsv`.
	let final_caption = table_caption_from_html(&html_content).unwrap_or_else(|| "table".to_string());
	let display_text = html_table_to_display(&html_content, render_tables_inline);
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
	// Not a compound file at all
	let Ok(compound) = CompoundFile::open(file) else { return Ok(None) };
	// Encrypted OOXML files always contain an EncryptionInfo stream.
	if compound.entry("/EncryptionInfo").is_err() {
		return Ok(None); // Compound file but not encrypted Office format
	}
	let Some(password) = password else {
		// TRANSLATORS: Error detail shown when an encrypted Office (OOXML) file needs a password (the internal sentinel prefix before it is not translated)
		anyhow::bail!("{PASSWORD_REQUIRED_ERROR_PREFIX} {}", t("File is encrypted and requires a password"));
	};
	let decrypted = decrypt_from_file(path, password)
		// TRANSLATORS: Error shown when decrypting an encrypted Office (OOXML) file fails; {} is the underlying error
		.map_err(|e| anyhow::anyhow!(t("Decryption failed (wrong password?): {}").replace("{}", &e.to_string())))?;
	Ok(Some(decrypted))
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

	/// Parse a Word table. The second cell contains U+1D11E (MUSICAL SYMBOL G CLEF, non-BMP,
	/// UTF-16 width 2) to lock the display-unit arithmetic. OFF mode emits the placeholder; ON mode
	/// emits the full TSV. In both cases the Table marker keeps the caption as text and its length
	/// equals the emitted display extent.
	#[test]
	fn word_table_emits_placeholder_or_tsv_by_flag() {
		// Minimal OOXML XML: one table with one row, two cells.
		let xml = r"<document><body>
			<tbl>
				<tr>
					<tc><p><r><t>Kop</t></r></p></tc>
					<tc><p><r><t>&#x1D11E;</t></r></p></tc>
				</tr>
			</tbl>
		</body></document>";
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
