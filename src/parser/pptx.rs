use std::{collections::HashMap, fs::File, io::BufReader, path::Path};

use anyhow::{Context, Result};
use roxmltree::{Document as XmlDocument, Node, NodeType};
use zip::ZipArchive;

use crate::{
	document::{Document, DocumentBuffer, Marker, MarkerType, ParserContext, ParserFlags, TocItem},
	parser::{
		Parser, ooxml::read_ooxml_relationships, path::extract_title_from_path, xml::collect_text_from_tagged_elements,
	},
	types::LinkInfo,
	zip::read_zip_entry_by_name,
};

pub struct PptxParser;

impl Parser for PptxParser {
	fn name(&self) -> &'static str {
		"PowerPoint Presentations"
	}

	fn extensions(&self) -> &[&str] {
		&["pptx", "pptm"]
	}

	fn supported_flags(&self) -> ParserFlags {
		ParserFlags::SUPPORTS_TOC
	}

	fn parse(&self, context: &ParserContext) -> Result<Document> {
		let file = File::open(&context.file_path)
			.with_context(|| format!("Failed to open PPTX file '{}'", context.file_path))?;
		let mut archive = ZipArchive::new(BufReader::new(file))
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
				buffer.add_marker(
					Marker::new(MarkerType::PageBreak, slide_start).with_text(format!("Slide {}", index + 1)),
				);
				for link in links {
					buffer.add_marker(
						Marker::new(MarkerType::Link, link.offset).with_text(link.text).with_reference(link.reference),
					);
				}
				let toc_name =
					if slide_title.is_empty() { format!("Slide {}", index + 1) } else { slide_title.clone() };
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
		if child.node_type() == NodeType::Element && child.tag_name().name() == "ph" {
			if let Some(ph_type) = child.attribute("type") {
				if ph_type == "title" || ph_type == "ctrTitle" {
					return true;
				}
			}
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
					if let Some(r_id) = node.attribute("id") {
						if let Some(link_target) = rels.get(r_id) {
							if let Some(parent) = node.parent() {
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
