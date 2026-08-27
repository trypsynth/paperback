//! Modern OOXML `.pptx` parsing: reads each `ppt/slides/slideN.xml` part directly (`pptx` has no
//! single flattened text stream the way `.doc`x does), plus relationship-based hyperlink
//! resolution and `DrawingML` table extraction.

use std::{collections::HashMap, fs, io::Cursor, path::Path};

use anyhow::{Context, Result};
use roxmltree::{Document as XmlDocument, Node, NodeType};
use zip::ZipArchive;

use crate::{
	document::{Document, DocumentBuffer, Marker, MarkerType, ParserContext, TocItem},
	parser::{
		convert::table_text::{
			build_html_table_from_grid, display_lines_and_length, html_table_to_display, table_caption_from_html,
		},
		util::{
			ooxml::{read_ooxml_relationships, try_decrypt_office_file},
			path::extract_title_from_path,
			xml::collect_text_from_tagged_elements,
		},
	},
	t,
	types::LinkInfo,
	util::{text::display_len, zip::read_zip_entry_by_name},
};

/// A table found while traversing a slide. Markers are added after the slide text is appended to
/// the buffer (mirroring the deferred link-marker handling), so positions stay in display units.
struct TableData {
	offset: usize,
	caption: String,
	html: String,
	length: usize,
}

pub(super) fn parse_pptx(context: &ParserContext) -> Result<Document> {
	tracing::debug!(path = %context.file_path, "parsing pptx file");
	let bytes = match try_decrypt_office_file(&context.file_path, context.password.as_deref())? {
		Some(decrypted) => decrypted,
		None => {
			fs::read(&context.file_path).with_context(|| format!("Failed to read PPTX file '{}'", context.file_path))?
		}
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
		tracing::warn!(path = %context.file_path, "pptx file has no slides");
		// TRANSLATORS: Error shown when a PPTX presentation file has no slides
		anyhow::bail!(t("PPTX file contains no slides"));
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
		let mut tables = Vec::new();
		let slide_text = extract_slide_text(
			slide_doc.root(),
			&mut links,
			&mut tables,
			slide_start,
			&rels,
			context.render_tables_inline,
		);
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
			for table in tables {
				buffer.add_marker(
					Marker::new(MarkerType::Table, table.offset)
						.with_text(table.caption)
						.with_reference(table.html)
						.with_length(table.length),
				);
			}
			let toc_name = if slide_title.is_empty() { format!("Slide {}", index + 1) } else { slide_title.clone() };
			toc_items.push(TocItem::new(toc_name, String::new(), slide_start));
		} else {
			tracing::debug!(slide = index + 1, "skipped pptx slide with no text");
		}
	}
	let title = extract_title_from_path(&context.file_path);
	let mut document = Document::new().with_title(title);
	document.set_buffer(buffer);
	document.id_positions = id_positions;
	document.toc_items = toc_items;
	tracing::debug!(path = %context.file_path, "parsed pptx file successfully");
	Ok(document)
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
	tables: &mut Vec<TableData>,
	slide_start: usize,
	rels: &HashMap<String, String>,
	render_tables_inline: bool,
) -> String {
	let mut text = String::new();
	traverse_for_text(root, &mut text, links, tables, slide_start, rels, render_tables_inline);
	text
}

fn traverse_for_text(
	node: Node,
	text: &mut String,
	links: &mut Vec<LinkInfo>,
	tables: &mut Vec<TableData>,
	slide_start: usize,
	rels: &HashMap<String, String>,
	render_tables_inline: bool,
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
				"tbl" => {
					// Handle the table here and skip generic recursion: otherwise the walk below would
					// re-emit every cell's `<a:t>` text as flat paragraph lines, duplicating the table.
					process_pptx_table(node, text, tables, slide_start, render_tables_inline);
					return;
				}
				"p" => {
					for child in node.children() {
						traverse_for_text(child, text, links, tables, slide_start, rels, render_tables_inline);
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
		traverse_for_text(child, text, links, tables, slide_start, rels, render_tables_inline);
	}
}

/// Convert a `DrawingML` `<a:tbl>` into the shared HTML-table representation, append its display text
/// to `text`, and record a [`TableData`] for later marker creation. pptx cells nest one level
/// deeper than Word (`tc > txBody > p`), so cell text is gathered from every descendant `<a:t>`.
fn process_pptx_table(
	node: Node,
	text: &mut String,
	tables: &mut Vec<TableData>,
	slide_start: usize,
	render_tables_inline: bool,
) {
	let mut rows: Vec<Vec<String>> = Vec::new();
	for tr in node.children() {
		if tr.node_type() != NodeType::Element || tr.tag_name().name() != "tr" {
			continue;
		}
		let mut cells: Vec<String> = Vec::new();
		for tc in tr.children() {
			if tc.node_type() != NodeType::Element || tc.tag_name().name() != "tc" {
				continue;
			}
			// Join paragraphs within the cell with a space so multi-paragraph cells stay readable.
			let mut cell_text = String::new();
			for p in tc.descendants() {
				if p.node_type() == NodeType::Element && p.tag_name().name() == "p" {
					let para = collect_text_from_tagged_elements(p, "t");
					if !para.is_empty() {
						if !cell_text.is_empty() {
							cell_text.push(' ');
						}
						cell_text.push_str(&para);
					}
				}
			}
			cells.push(cell_text.trim().to_string());
		}
		rows.push(cells);
	}
	let html = build_html_table_from_grid(&rows);
	let caption = table_caption_from_html(&html).unwrap_or_else(|| "table".to_string());
	let display_text = html_table_to_display(&html, render_tables_inline);
	let (_, length) = display_lines_and_length(&display_text);
	let offset = slide_start + display_len(text);
	text.push_str(&display_text);
	text.push('\n');
	tables.push(TableData { offset, caption, html, length });
}

#[cfg(test)]
mod tests {
	use std::collections::HashMap;

	use roxmltree::Document as XmlDocument;
	use rstest::rstest;

	use super::{display_len, extract_slide_number, extract_slide_text, extract_slide_title, is_title_shape};

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
		let xml = r"<root><sp><txBody><p><r><t>Body text</t></r></p></txBody></sp></root>";
		let doc = XmlDocument::parse(xml).expect("xml parse");
		assert!(extract_slide_title(doc.root()).is_empty());
	}

	#[test]
	fn extract_slide_text_collects_paragraphs_and_breaks() {
		let xml = r"
			<root>
				<p><r><t>Hello</t></r><br/><r><t>World</t></r></p>
				<p><r><t>Next</t></r></p>
			</root>
		";
		let doc = XmlDocument::parse(xml).expect("xml parse");
		let mut links = Vec::new();
		let mut tables = Vec::new();
		let rels = HashMap::new();
		let text = extract_slide_text(doc.root(), &mut links, &mut tables, 0, &rels, true);
		assert_eq!(text, "Hello\nWorld\nNext\n");
		assert!(links.is_empty());
		assert!(tables.is_empty());
	}

	const TABLE_SLIDE_XML: &str = r"
		<root>
			<graphicFrame><graphic><graphicData>
				<tbl>
					<tblPr/>
					<tblGrid><gridCol/><gridCol/></tblGrid>
					<tr><tc><txBody><p><r><t>One</t></r></p></txBody></tc><tc><txBody><p><r><t>Two</t></r></p></txBody></tc></tr>
					<tr><tc><txBody><p><r><t>Three</t></r></p></txBody></tc><tc><txBody><p><r><t>Four</t></r></p></txBody></tc></tr>
				</tbl>
			</graphicData></graphic></graphicFrame>
		</root>
	";

	#[test]
	fn extract_slide_text_renders_table_inline_with_marker_data() {
		let doc = XmlDocument::parse(TABLE_SLIDE_XML).expect("xml parse");
		let mut links = Vec::new();
		let mut tables = Vec::new();
		let rels = HashMap::new();
		let text = extract_slide_text(doc.root(), &mut links, &mut tables, 0, &rels, true);
		// Tab-separated rows, no flat duplication of the cell text.
		assert_eq!(text, "One\tTwo\nThree\tFour\n");
		assert_eq!(tables.len(), 1);
		let table = &tables[0];
		assert_eq!(table.offset, 0);
		assert_eq!(table.caption, "One Two");
		assert!(table.html.contains("<table"));
		assert!(table.html.contains("Four"));
		// Two rows, each contributing its display width + a trailing newline.
		assert_eq!(table.length, display_len("One\tTwo") + 1 + display_len("Three\tFour") + 1);
		assert!(links.is_empty());
	}

	#[test]
	fn extract_slide_text_renders_table_placeholder() {
		let doc = XmlDocument::parse(TABLE_SLIDE_XML).expect("xml parse");
		let mut links = Vec::new();
		let mut tables = Vec::new();
		let rels = HashMap::new();
		let text = extract_slide_text(doc.root(), &mut links, &mut tables, 0, &rels, false);
		assert_eq!(text, "[Table]: One Two\n");
		assert_eq!(tables.len(), 1);
		assert_eq!(tables[0].caption, "One Two");
	}
}
