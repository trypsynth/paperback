use std::{
	collections::HashMap,
	fs::{self, File},
	io::{BufReader, Read},
	path::Path,
};

use anyhow::{Context, Result};
use roxmltree::{Document as XmlDocument, Node, NodeType, ParsingOptions};
use zip::ZipArchive;

use crate::{
	document::{Document, DocumentBuffer, Marker, MarkerType, ParserContext, TocItem},
	parser::{
		PASSWORD_REQUIRED_ERROR_PREFIX, Parser, add_converter_markers,
		convert::{
			html_to_text::{HtmlSourceMode, HtmlToText},
			xml_to_text::XmlToText,
		},
		util::{path::extract_title_from_path, toc::build_toc_from_headings},
	},
	t,
	util::{encoding::convert_to_utf8, zip::read_zip_entry_by_name_with_password},
};

pub struct DaisyParser;

impl Parser for DaisyParser {
	fn parse(&self, context: &ParserContext) -> Result<Document> {
		let path = Path::new(&context.file_path);
		let mut title = extract_title_from_path(&context.file_path);
		let mut author = String::new();
		let mut buffer;
		let ext_is_zip = path.extension().is_some_and(|ext| ext.eq_ignore_ascii_case("zip"));
		let is_zip = ext_is_zip || {
			let magic_result = File::open(path).and_then(|f| {
				let mut header = [0; 4];
				let mut reader = BufReader::new(f);
				reader.read_exact(&mut header)?;
				Ok(header == [0x50, 0x4b, 0x03, 0x04])
			});
			if let Err(ref e) = magic_result {
				tracing::warn!(path = %path.display(), error = %e, "failed to read file header while checking for zip magic bytes");
			}
			magic_result.unwrap_or(false)
		};
		if ext_is_zip {
			tracing::debug!(path = %path.display(), "detected zip via file extension");
		} else if is_zip {
			tracing::debug!(path = %path.display(), "detected zip via magic bytes");
		}
		tracing::debug!(path = %path.display(), is_zip, "starting daisy parse");
		if is_zip {
			tracing::debug!("taking zip archive parse path");
			let file = File::open(path).context("Failed to open zip file")?;
			let mut archive = ZipArchive::new(BufReader::new(file)).context("Failed to read zip archive")?;
			let opf_path = archive
				.file_names()
				.find(|n| Path::new(n).extension().is_some_and(|ext| ext.eq_ignore_ascii_case("opf")))
				.map(String::from);
			let opf_found = opf_path.is_some();
			if let Some(opf_name) = opf_path {
				let (manifest_xml, metadata) = {
					let opf_content =
						read_zip_entry_by_name_with_password(&mut archive, &opf_name, context.password.as_deref())
							.map_err(|e| {
								if e.to_string().starts_with(PASSWORD_REQUIRED_ERROR_PREFIX) {
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
								if e.to_string().starts_with(PASSWORD_REQUIRED_ERROR_PREFIX) {
									e
								} else {
									e.context("Failed to read XML file from zip")
								}
							})?;
					let mut converter = XmlToText::with_render_tables_inline(context.render_tables_inline);
					if converter.convert(&xml_content) {
						buffer = DocumentBuffer::with_content(converter.get_text());
						add_converter_markers(&mut buffer, &converter, 0);
						for pb in converter.get_page_breaks() {
							buffer.add_marker(Marker::new(MarkerType::PageBreak, pb.offset).with_text(pb.text.clone()));
						}
					} else {
						// TRANSLATORS: Error shown when a DAISY book's DTBook XML fails to convert to plain text
						anyhow::bail!(t("Failed to convert DTBook XML to text"));
					}
					let mut toc_items = None;
					let ncx_path = archive
						.file_names()
						.find(|n| Path::new(n).extension().is_some_and(|ext| ext.eq_ignore_ascii_case("ncx")))
						.map(String::from);
					if let Some(ncx_name) = ncx_path {
						match read_zip_entry_by_name_with_password(&mut archive, &ncx_name, context.password.as_deref())
						{
							Ok(ncx_content) if !ncx_content.is_empty() => {
								if let Some(ncx_toc) = parse_daisy_ncx(&ncx_content, converter.get_id_positions())
									&& !ncx_toc.is_empty()
								{
									toc_items = Some(ncx_toc);
								}
							}
							Ok(_) => {
								tracing::debug!(ncx_name = %ncx_name, "ncx file is empty, using heading-derived toc");
							}
							Err(e) => {
								tracing::warn!(ncx_name = %ncx_name, error = %e, "ncx file present but failed to read, using heading-derived toc");
							}
						}
					} else {
						tracing::debug!("no ncx file found in zip archive, using heading-derived toc");
					}
					let toc_items = toc_items.unwrap_or_else(|| build_toc_from_headings(converter.get_headings()));
					tracing::debug!(path = %path.display(), "parsed daisy book as daisy 3 (opf and dtbook xml) from zip archive");
					return Ok(Document {
						title,
						author,
						buffer,
						toc_items,
						id_positions: converter.get_id_positions().clone(),
						..Document::default()
					});
				}
				tracing::warn!(opf_name = %opf_name, "opf found but no dtbook manifest item, trying daisy 2.02");
			}
			let ncc_path =
				archive.file_names().find(|n| n.ends_with("ncc.html") || n.ends_with("NCC.html")).map(String::from);
			let ncc_found = ncc_path.is_some();
			if let Some(ncc_name) = ncc_path {
				let ncc_content =
					read_zip_entry_by_name_with_password(&mut archive, &ncc_name, context.password.as_deref())
						.map_err(|e| {
							if e.to_string().starts_with(PASSWORD_REQUIRED_ERROR_PREFIX) {
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
					match read_zip_entry_by_name_with_password(&mut archive, &link_path, context.password.as_deref()) {
						Ok(c) => {
							combined_html.push_str(&c);
							combined_html.push_str("\n\n");
						}
						Err(e) => {
							tracing::warn!(link = %link_path, error = %e, "failed to read linked content page, skipping");
						}
					}
				}
				let mut converter = HtmlToText::with_render_tables_inline(context.render_tables_inline);
				if converter.convert(&combined_html, HtmlSourceMode::NativeHtml) {
					buffer = DocumentBuffer::with_content(converter.get_text());
					add_converter_markers(&mut buffer, &converter, 0);
					let toc_items = build_toc_from_headings(converter.get_headings());
					tracing::debug!(path = %path.display(), "parsed daisy book as daisy 2.02 (ncc.html) from zip archive");
					return Ok(Document {
						title,
						author,
						buffer,
						toc_items,
						id_positions: converter.get_id_positions().clone(),
						..Document::default()
					});
				}
				// currently unreachable since HtmlToText::convert always returns true today
				tracing::warn!("html to text conversion reported failure for daisy 2.02 book");
			}
			tracing::warn!(opf_found, ncc_found, "exhausted daisy 3 and daisy 2.02 detection attempts in zip archive");
			// TRANSLATORS: Error shown when a ZIP file is not a recognizable DAISY 3 or DAISY 2.02 book
			anyhow::bail!(t("ZIP archive does not appear to be a valid DAISY 3 or DAISY 2.02 book"));
		}
		tracing::debug!(path = %path.display(), "taking loose files parse path");
		let file_content = convert_to_utf8(&fs::read(path)?);
		let (manifest_xml, metadata) = parse_opf_metadata_and_manifest(&file_content)?;
		if let Some(t) = metadata.0 {
			title = t;
		}
		if let Some(a) = metadata.1 {
			author = a;
		}
		let dtbook_found = manifest_xml.is_some();
		if let Some(dtbook_path) = manifest_xml {
			let base_dir = path.parent().unwrap_or_else(|| Path::new(""));
			let xml_full_path = base_dir.join(&dtbook_path);
			let xml_content = convert_to_utf8(
				&fs::read(&xml_full_path)
					.with_context(|| format!("Failed to read DTBook XML file at {}", xml_full_path.display()))?,
			);
			let mut converter = XmlToText::with_render_tables_inline(context.render_tables_inline);
			if converter.convert(&xml_content) {
				buffer = DocumentBuffer::with_content(converter.get_text());
				add_converter_markers(&mut buffer, &converter, 0);
				for pb in converter.get_page_breaks() {
					buffer.add_marker(Marker::new(MarkerType::PageBreak, pb.offset).with_text(pb.text.clone()));
				}
				let mut toc_items = None;
				let mut ncx_found = false;
				if let Ok(entries) = fs::read_dir(base_dir) {
					for entry in entries.flatten() {
						let path = entry.path();
						if path.is_file() && path.extension().is_some_and(|e| e.eq_ignore_ascii_case("ncx")) {
							ncx_found = true;
							match fs::read(&path) {
								Ok(bytes) => {
									let ncx_content = convert_to_utf8(&bytes);
									if let Some(ncx_toc) = parse_daisy_ncx(&ncx_content, converter.get_id_positions())
										&& !ncx_toc.is_empty()
									{
										toc_items = Some(ncx_toc);
										break;
									}
								}
								Err(e) => {
									tracing::warn!(path = %path.display(), error = %e, "ncx file present but failed to read, using heading-derived toc");
								}
							}
						}
					}
				}
				if toc_items.is_none() {
					if ncx_found {
						tracing::debug!("ncx file found but did not yield toc items, using heading-derived toc");
					} else {
						tracing::debug!("no ncx file found in directory, using heading-derived toc");
					}
				}
				let toc_items = toc_items.unwrap_or_else(|| build_toc_from_headings(converter.get_headings()));
				tracing::debug!(path = %path.display(), "parsed daisy book as daisy 3 (opf and dtbook xml) from loose files");
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
		tracing::warn!(dtbook_found, "could not parse daisy opf file or locate dtbook xml in manifest");
		// TRANSLATORS: Error shown when a DAISY .opf file is invalid or its DTBook XML can't be located
		anyhow::bail!(t("Invalid DAISY .opf file or could not find DTBook XML in manifest"));
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
								let href = item.attribute("href").map(ToString::to_string);
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

fn parse_daisy_ncx(ncx_content: &str, id_positions: &HashMap<String, usize>) -> Option<Vec<TocItem>> {
	let ncx_doc =
		XmlDocument::parse_with_options(ncx_content, ParsingOptions { allow_dtd: true, ..ParsingOptions::default() })
			.ok()?;
	let nav_map =
		ncx_doc.descendants().find(|n| n.node_type() == NodeType::Element && n.tag_name().name() == "navMap")?;
	let mut items = Vec::new();
	for navpoint in nav_map.children() {
		if navpoint.node_type() == NodeType::Element
			&& navpoint.tag_name().name() == "navPoint"
			&& let Some(item) = convert_daisy_navpoint(navpoint, id_positions)
		{
			items.push(item);
		}
	}
	if items.is_empty() { None } else { Some(items) }
}

fn convert_daisy_navpoint(nav: Node, id_positions: &HashMap<String, usize>) -> Option<TocItem> {
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
	let mut item = TocItem::new(label, target_id.to_string(), offset);
	for child in nav.children() {
		if child.node_type() == NodeType::Element
			&& child.tag_name().name() == "navPoint"
			&& let Some(child_item) = convert_daisy_navpoint(child, id_positions)
		{
			item.children.push(child_item);
		}
	}
	Some(item)
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::util::test_support::TempDir;

	// Regression test for https://github.com/trypsynth/paperback/issues/606: a real-world DAISY
	// book declared its DTBook XML as ISO-8859-1 but was actually encoded as Windows-1252 (a very
	// common mislabeling), which made `fs::read_to_string` fail outright since the bytes were not
	// valid UTF-8.
	#[test]
	fn parses_dtbook_xml_declared_as_iso_8859_1_but_encoded_as_windows_1252() {
		let dir = TempDir::new("daisy");
		let opf_path = dir.path().join("book.opf");
		let xml_path = dir.path().join("book.xml");
		fs::write(
			&opf_path,
			br#"<?xml version="1.0" encoding="ISO-8859-1"?>
<package unique-identifier="uid">
  <metadata>
    <dc-metadata xmlns:dc="http://purl.org/dc/elements/1.0/">
      <dc:Title>Test Book</dc:Title>
      <dc:Creator>Test Author</dc:Creator>
    </dc-metadata>
  </metadata>
  <manifest>
    <item href="book.xml" media-type="application/x-dtbook+xml"/>
  </manifest>
</package>
"#,
		)
		.expect("write opf");
		// Windows-1252 bytes for curly quotes (0x93/0x94) and 0xE7 for the c-cedilla in
		// "Fran\xE7ois" -- both invalid as standalone UTF-8.
		let mut xml_bytes = Vec::new();
		xml_bytes.extend_from_slice(b"<?xml version=\"1.0\" encoding=\"ISO-8859-1\"?>\n");
		xml_bytes.extend_from_slice(b"<dtbook><book><frontmatter><p id=\"p1\">Fran\xE7ois said, \x93hello\x94.</p>");
		xml_bytes.extend_from_slice(b"</frontmatter></book></dtbook>");
		fs::write(&xml_path, &xml_bytes).expect("write dtbook xml");

		let context = ParserContext::new(opf_path.to_string_lossy().to_string());
		let document = DaisyParser.parse(&context).expect("DAISY parse should succeed on mislabeled encoding");

		assert_eq!(document.title, "Test Book");
		assert_eq!(document.author, "Test Author");
		assert!(!document.buffer.content.contains('\u{FFFD}'), "no replacement characters expected");
		assert!(document.buffer.content.contains("François"), "c-cedilla should decode correctly");
		assert!(document.buffer.content.contains('\u{201C}'), "left curly quote should decode correctly");
		assert!(document.buffer.content.contains('\u{201D}'), "right curly quote should decode correctly");
	}
}
