//! Modern OOXML `.docx` parsing, including the batch-`.zip`-of-`.docx` and single-file
//! password-protected cases (the latter via [`util::ooxml::try_decrypt_office_file`], the
//! shared OLE-container decryption check also reused by `powerpoint`'s `.pptx` parsing).

use std::{
	collections::HashMap,
	fs::{self, File},
	io::{BufReader, Cursor, Read, Seek},
};

use anyhow::{Context, Result};
use roxmltree::{Document as XmlDocument, Node, NodeType};
use zip::ZipArchive;

use crate::{
	document::{Document, DocumentBuffer, Marker, MarkerType, ParserContext},
	parser::{
		convert::table_text::{build_html_table_from_grid, html_table_to_display, table_caption_from_html},
		util::{
			ooxml::{collect_ooxml_run_text, read_ooxml_relationships, try_decrypt_office_file},
			path::extract_title_from_path,
			toc::build_toc_from_buffer,
		},
	},
	t,
	types::HeadingInfo,
	util::zip::read_zip_entry_by_name,
};

mod paragraph;

use paragraph::{extract_number_from_string, process_paragraph};

pub(super) fn parse_word_zip(context: &ParserContext, render_tables_inline: bool) -> Result<Document> {
	let file =
		File::open(&context.file_path).with_context(|| format!("Failed to open ZIP file '{}'", context.file_path))?;
	let mut archive = ZipArchive::new(BufReader::new(file))
		.with_context(|| format!("Failed to read ZIP archive '{}'", context.file_path))?;
	tracing::debug!(path = %context.file_path, entries = archive.len(), "scanning zip archive for embedded docx entries");
	// zip 9 hands back a Result per name, since decoding one can fail. A name that will not
	// decode cannot match what this scan is looking for, so drop those rather than fail the file.
	let mut docx_names: Vec<String> = archive
		.file_names()
		.flatten()
		.filter(|name| name.to_ascii_lowercase().ends_with(".docx"))
		.map(String::from)
		.collect();
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
		.map_or_else(|| fs::read(path).with_context(|| format!("Failed to read '{path}'")), Ok)
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

/// Reads `word/styles.xml` and returns a map of style ID → heading level (1-9).
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

#[cfg(test)]
mod tests;
