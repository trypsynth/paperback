use std::{
	fs::File,
	io::{BufReader, Read},
	path::Path,
};

use anyhow::{Context, Result};
use roxmltree::{Document as XmlDocument, Node, NodeType, ParsingOptions};
use zip::ZipArchive;

use crate::{
	document::{Document, DocumentBuffer, Marker, MarkerType, ParserContext, ParserFlags},
	html_to_text::{HtmlSourceMode, HtmlToText},
	parser::{Parser, add_converter_markers, path::extract_title_from_path, toc::build_toc_from_headings},
	xml_to_text::XmlToText,
	zip::read_zip_entry_by_name_with_password,
};

pub struct DaisyParser;

impl Parser for DaisyParser {
	fn name(&self) -> &'static str {
		"DAISY Books"
	}

	fn extensions(&self) -> &[&str] {
		&["opf", "zip"]
	}

	fn supported_flags(&self) -> ParserFlags {
		ParserFlags::SUPPORTS_SECTIONS
			| ParserFlags::SUPPORTS_TOC
			| ParserFlags::SUPPORTS_LISTS
			| ParserFlags::SUPPORTS_PAGES
	}

	fn parse(&self, context: &ParserContext) -> Result<Document> {
		let path = Path::new(&context.file_path);
		let mut title = extract_title_from_path(&context.file_path);
		let mut author = String::new();
		let mut buffer;
		let is_zip = path.extension().is_some_and(|ext| ext.eq_ignore_ascii_case("zip"))
			|| File::open(path)
				.and_then(|f| {
					let mut header = [0; 4];
					let mut reader = BufReader::new(f);
					reader.read_exact(&mut header)?;
					Ok(header == [0x50, 0x4b, 0x03, 0x04])
				})
				.unwrap_or(false);
		if is_zip {
			let file = File::open(path).context("Failed to open zip file")?;
			let mut archive = ZipArchive::new(BufReader::new(file)).context("Failed to read zip archive")?;
			let opf_path = archive
				.file_names()
				.find(|n| Path::new(n).extension().is_some_and(|ext| ext.eq_ignore_ascii_case("opf")))
				.map(String::from);
			if let Some(opf_name) = opf_path {
				let (manifest_xml, metadata) = {
					let opf_content =
						read_zip_entry_by_name_with_password(&mut archive, &opf_name, context.password.as_deref())
							.map_err(|e| {
								if e.to_string().starts_with(crate::parser::PASSWORD_REQUIRED_ERROR_PREFIX) {
									e
								} else {
									e.context("Failed to read OPF file")
								}
							})?;
					parse_opf_metadata_and_manifest(&opf_content)?
				};
				if let Some(t) = metadata.0 {
					title = t;
				}
				if let Some(a) = metadata.1 {
					author = a;
				}
				if let Some(dtbook_path) = manifest_xml {
					let base_dir = Path::new(&opf_name).parent().unwrap_or_else(|| Path::new(""));
					let xml_full_path = if base_dir.as_os_str().is_empty() {
						dtbook_path
					} else {
						base_dir.join(&dtbook_path).to_string_lossy().to_string().replace('\\', "/")
					};
					let xml_content =
						read_zip_entry_by_name_with_password(&mut archive, &xml_full_path, context.password.as_deref())
							.map_err(|e| {
								if e.to_string().starts_with(crate::parser::PASSWORD_REQUIRED_ERROR_PREFIX) {
									e
								} else {
									e.context("Failed to read XML file from zip")
								}
							})?;
					let mut converter = XmlToText::new();
					if converter.convert(&xml_content) {
						buffer = DocumentBuffer::with_content(converter.get_text());
						add_converter_markers(&mut buffer, &converter, 0);
						for pb in converter.get_page_breaks() {
							buffer.add_marker(Marker::new(MarkerType::PageBreak, pb.offset).with_text(pb.text.clone()));
						}
					} else {
						anyhow::bail!("Failed to convert DTBook XML to text");
					}
					let mut toc_items = None;
					let ncx_path = archive
						.file_names()
						.find(|n| Path::new(n).extension().is_some_and(|ext| ext.eq_ignore_ascii_case("ncx")))
						.map(String::from);
					if let Some(ncx_name) = ncx_path {
						if let Ok(ncx_content) =
							read_zip_entry_by_name_with_password(&mut archive, &ncx_name, context.password.as_deref())
						{
							if !ncx_content.is_empty() {
								if let Some(ncx_toc) = parse_daisy_ncx(&ncx_content, converter.get_id_positions()) {
									if !ncx_toc.is_empty() {
										toc_items = Some(ncx_toc);
									}
								}
							}
						}
					}
					let toc_items = toc_items.unwrap_or_else(|| build_toc_from_headings(converter.get_headings()));
					return Ok(Document {
						title,
						author,
						buffer,
						toc_items,
						id_positions: converter.get_id_positions().clone(),
						..Document::default()
					});
				}
			}
			let ncc_path =
				archive.file_names().find(|n| n.ends_with("ncc.html") || n.ends_with("NCC.html")).map(String::from);
			if let Some(ncc_name) = ncc_path {
				let ncc_content =
					read_zip_entry_by_name_with_password(&mut archive, &ncc_name, context.password.as_deref())
						.map_err(|e| {
							if e.to_string().starts_with(crate::parser::PASSWORD_REQUIRED_ERROR_PREFIX) {
								e
							} else {
								e.context("Failed to read ncc.html")
							}
						})?;
				let links = extract_daisy2_links(&ncc_content);
				let mut combined_html = String::new();
				let base_dir = Path::new(&ncc_name).parent().unwrap_or_else(|| Path::new(""));
				for link in links {
					let link_path = if base_dir.as_os_str().is_empty() {
						link.clone()
					} else {
						base_dir.join(&link).to_string_lossy().to_string().replace('\\', "/")
					};
					if let Ok(c) =
						read_zip_entry_by_name_with_password(&mut archive, &link_path, context.password.as_deref())
					{
						combined_html.push_str(&c);
						combined_html.push_str("\n\n");
					}
				}
				let mut converter = HtmlToText::new();
				if converter.convert(&combined_html, HtmlSourceMode::NativeHtml) {
					buffer = DocumentBuffer::with_content(converter.get_text());
					add_converter_markers(&mut buffer, &converter, 0);
					let toc_items = build_toc_from_headings(converter.get_headings());
					return Ok(Document {
						title,
						author,
						buffer,
						toc_items,
						id_positions: converter.get_id_positions().clone(),
						..Document::default()
					});
				}
			}
			anyhow::bail!("ZIP archive does not appear to be a valid DAISY 3 or DAISY 2.02 book");
		}
		let file_content = std::fs::read_to_string(path)?;
		let (manifest_xml, metadata) = parse_opf_metadata_and_manifest(&file_content)?;
		if let Some(t) = metadata.0 {
			title = t;
		}
		if let Some(a) = metadata.1 {
			author = a;
		}
		if let Some(dtbook_path) = manifest_xml {
			let base_dir = path.parent().unwrap_or_else(|| Path::new(""));
			let xml_full_path = base_dir.join(&dtbook_path);
			let xml_content = std::fs::read_to_string(&xml_full_path)
				.with_context(|| format!("Failed to read DTBook XML file at {}", xml_full_path.display()))?;
			let mut converter = XmlToText::new();
			if converter.convert(&xml_content) {
				buffer = DocumentBuffer::with_content(converter.get_text());
				add_converter_markers(&mut buffer, &converter, 0);
				for pb in converter.get_page_breaks() {
					buffer.add_marker(Marker::new(MarkerType::PageBreak, pb.offset).with_text(pb.text.clone()));
				}
				let mut toc_items = None;
				if let Ok(entries) = std::fs::read_dir(base_dir) {
					for entry in entries.flatten() {
						let path = entry.path();
						if path.is_file() && path.extension().is_some_and(|e| e.eq_ignore_ascii_case("ncx")) {
							if let Ok(ncx_content) = std::fs::read_to_string(&path) {
								if let Some(ncx_toc) = parse_daisy_ncx(&ncx_content, converter.get_id_positions()) {
									if !ncx_toc.is_empty() {
										toc_items = Some(ncx_toc);
										break;
									}
								}
							}
						}
					}
				}
				let toc_items = toc_items.unwrap_or_else(|| build_toc_from_headings(converter.get_headings()));
				return Ok(Document {
					title,
					author,
					buffer,
					toc_items,
					id_positions: converter.get_id_positions().clone(),
					..Document::default()
				});
			}
		}
		anyhow::bail!("Invalid DAISY .opf file or could not find DTBook XML in manifest");
	}
}

type OpfMetadataResult = Result<(Option<String>, (Option<String>, Option<String>))>;

fn parse_opf_metadata_and_manifest(opf_content: &str) -> OpfMetadataResult {
	let doc =
		XmlDocument::parse_with_options(opf_content, ParsingOptions { allow_dtd: true, ..ParsingOptions::default() })
			.context("Failed to parse OPF XML")?;
	let mut dtbook_href = None;
	let mut title = None;
	let mut author = None;
	if let Some(package) =
		doc.descendants().find(|n| n.node_type() == NodeType::Element && n.tag_name().name() == "package")
	{
		for child in package.children() {
			if child.is_element() {
				if child.tag_name().name() == "metadata" {
					for meta_child in child.children() {
						if meta_child.is_element() {
							let name = meta_child.tag_name().name();
							if name == "Title" || name == "title" {
								title = meta_child.text().map(|s| s.trim().to_string());
							} else if name == "Creator" || name == "creator" {
								author = meta_child.text().map(|s| s.trim().to_string());
							}
						}
					}
					for meta_child in child.descendants() {
						if meta_child.is_element() {
							let name = meta_child.tag_name().name();
							if name == "Title" || name == "title" {
								if title.is_none() {
									title = meta_child.text().map(|s| s.trim().to_string());
								}
							} else if (name == "Creator" || name == "creator") && author.is_none() {
								author = meta_child.text().map(|s| s.trim().to_string());
							}
						}
					}
				} else if child.tag_name().name() == "manifest" {
					for item in child.children() {
						if item.is_element() && item.tag_name().name() == "item" {
							let media_type = item.attribute("media-type").unwrap_or("");
							if media_type == "application/x-dtbook+xml" || media_type == "text/xml" {
								let href = item.attribute("href").map(std::string::ToString::to_string);
								if media_type == "application/x-dtbook+xml" {
									dtbook_href = href;
									break;
								} else if dtbook_href.is_none()
									&& href.as_ref().is_some_and(|h| {
										Path::new(h).extension().is_some_and(|ext| ext.eq_ignore_ascii_case("xml"))
									}) {
									dtbook_href = href;
								}
							}
						}
					}
				}
			}
		}
	}
	Ok((dtbook_href, (title, author)))
}

fn extract_daisy2_links(ncc_content: &str) -> Vec<String> {
	let mut links = Vec::new();
	let scraper = scraper::Html::parse_document(ncc_content);
	let selector = scraper::Selector::parse("a[href]").unwrap();
	for element in scraper.select(&selector) {
		if let Some(href) = element.value().attr("href") {
			let file_path = href.split('#').next().unwrap_or(href);
			if !file_path.is_empty() && !links.contains(&file_path.to_string()) {
				links.push(file_path.to_string());
			}
		}
	}
	links
}

fn parse_daisy_ncx(
	ncx_content: &str,
	id_positions: &std::collections::HashMap<String, usize>,
) -> Option<Vec<crate::document::TocItem>> {
	let ncx_doc =
		XmlDocument::parse_with_options(ncx_content, ParsingOptions { allow_dtd: true, ..ParsingOptions::default() })
			.ok()?;
	let nav_map =
		ncx_doc.descendants().find(|n| n.node_type() == NodeType::Element && n.tag_name().name() == "navMap")?;
	let mut items = Vec::new();
	for navpoint in nav_map.children() {
		if navpoint.node_type() == NodeType::Element && navpoint.tag_name().name() == "navPoint" {
			if let Some(item) = convert_daisy_navpoint(navpoint, id_positions) {
				items.push(item);
			}
		}
	}
	if items.is_empty() { None } else { Some(items) }
}

fn convert_daisy_navpoint(
	nav: Node,
	id_positions: &std::collections::HashMap<String, usize>,
) -> Option<crate::document::TocItem> {
	let label = nav
		.children()
		.find(|n| n.node_type() == NodeType::Element && n.tag_name().name() == "navLabel")
		.and_then(|label| {
			label
				.children()
				.find(|t| t.node_type() == NodeType::Element && t.tag_name().name() == "text")
				.and_then(|t| t.text())
		})
		.unwrap_or("")
		.to_string();
	let content_src = nav
		.children()
		.find(|n| n.node_type() == NodeType::Element && n.tag_name().name() == "content")
		.and_then(|c| c.attribute("src"))?;
	if label.trim().is_empty() {
		return None;
	}
	let target_id =
		content_src.find('#').map_or_else(|| nav.attribute("id").unwrap_or(content_src), |idx| &content_src[idx + 1..]);
	let offset = id_positions
		.get(target_id)
		.or_else(|| nav.attribute("id").and_then(|id| id_positions.get(id)))
		.copied()
		.unwrap_or(0);
	let mut item = crate::document::TocItem::new(label, target_id.to_string(), offset);
	for child in nav.children() {
		if child.node_type() == NodeType::Element && child.tag_name().name() == "navPoint" {
			if let Some(child_item) = convert_daisy_navpoint(child, id_positions) {
				item.children.push(child_item);
			}
		}
	}
	Some(item)
}
