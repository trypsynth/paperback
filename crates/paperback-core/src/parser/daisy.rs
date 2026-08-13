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
	audio::{AudioLocation, AudioTimelineBuilder},
	document::{Document, DocumentBuffer, Marker, MarkerType, ParserContext, TocItem},
	parser::{
		PASSWORD_REQUIRED_ERROR_PREFIX, Parser, add_converter_markers,
		html_to_text::{HtmlSourceMode, HtmlToText},
		smil::parse_smil_pars,
		util::{
			path::{extract_title_from_path, resolve_relative_path},
			toc::{build_toc_from_buffer, build_toc_from_headings},
		},
		xml_to_text::XmlToText,
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
				let opf_dir = opf_name.rsplit_once('/').map_or_else(String::new, |(dir, _)| dir.to_string());
				let opf_content =
					read_zip_entry_by_name_with_password(&mut archive, &opf_name, context.password.as_deref())
						.map_err(|e| {
							if e.to_string().starts_with(PASSWORD_REQUIRED_ERROR_PREFIX) {
								e
							} else {
								e.context("Failed to read OPF file")
							}
						})?;
				let package = parse_opf_package(&opf_content, &opf_dir)?;
				if let Some(t) = package.title.clone() {
					title = t;
				}
				if let Some(a) = package.author.clone() {
					author = a;
				}
				let password = context.password.clone();
				let archive_path = context.file_path.clone();
				{
					let mut read_text = |href: &str| -> Result<String> {
						read_zip_entry_by_name_with_password(&mut archive, href, password.as_deref())
					};
					let resolve_audio =
						|href: &str| AudioLocation::ZipEntry { archive: archive_path.clone(), entry: href.to_string() };
					if let Some(document) = build_daisy_document(
						&package,
						title.clone(),
						author.clone(),
						context.render_tables_inline,
						&mut read_text,
						&resolve_audio,
					) {
						return Ok(document);
					}
				}
				if let Some(dtbook_path) = find_single_dtbook_href(&package) {
					let xml_content =
						read_zip_entry_by_name_with_password(&mut archive, &dtbook_path, context.password.as_deref())
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
					if let Some(ncx_name) = ncx_path
						&& let Ok(ncx_content) =
							read_zip_entry_by_name_with_password(&mut archive, &ncx_name, context.password.as_deref())
						&& !ncx_content.is_empty()
						&& let Some(ncx_toc) = parse_daisy_ncx(&ncx_content, converter.get_id_positions())
						&& !ncx_toc.is_empty()
					{
						toc_items = Some(ncx_toc);
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
					if let Ok(c) =
						read_zip_entry_by_name_with_password(&mut archive, &link_path, context.password.as_deref())
					{
						combined_html.push_str(&c);
						combined_html.push_str("\n\n");
					}
				}
				let mut converter = HtmlToText::with_render_tables_inline(context.render_tables_inline);
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
			// TRANSLATORS: Error shown when a ZIP file is not a recognizable DAISY 3 or DAISY 2.02 book
			anyhow::bail!(t("ZIP archive does not appear to be a valid DAISY 3 or DAISY 2.02 book"));
		}
		let opf_content = convert_to_utf8(&fs::read(path)?);
		let package = parse_opf_package(&opf_content, "")?;
		if let Some(t) = package.title.clone() {
			title = t;
		}
		if let Some(a) = package.author.clone() {
			author = a;
		}
		let base_dir = path.parent().unwrap_or_else(|| Path::new(""));
		{
			let mut read_text = |href: &str| -> Result<String> {
				let full_path = base_dir.join(href);
				Ok(convert_to_utf8(
					&fs::read(&full_path).with_context(|| format!("Failed to read file at {}", full_path.display()))?,
				))
			};
			let resolve_audio = |href: &str| AudioLocation::File(base_dir.join(href).to_string_lossy().to_string());
			if let Some(document) = build_daisy_document(
				&package,
				title.clone(),
				author.clone(),
				context.render_tables_inline,
				&mut read_text,
				&resolve_audio,
			) {
				return Ok(document);
			}
		}
		if let Some(dtbook_path) = find_single_dtbook_href(&package) {
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
				if let Ok(entries) = fs::read_dir(base_dir) {
					for entry in entries.flatten() {
						let path = entry.path();
						if path.is_file()
							&& path.extension().is_some_and(|e| e.eq_ignore_ascii_case("ncx"))
							&& let Some(ncx_content) = fs::read(&path).ok().map(|b| convert_to_utf8(&b))
							&& let Some(ncx_toc) = parse_daisy_ncx(&ncx_content, converter.get_id_positions())
							&& !ncx_toc.is_empty()
						{
							toc_items = Some(ncx_toc);
							break;
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
		// TRANSLATORS: Error shown when a DAISY .opf file is invalid or its DTBook XML can't be located
		anyhow::bail!(t("Invalid DAISY .opf file or could not find DTBook XML in manifest"));
	}
}

struct ManifestItem {
	href: String,
	media_type: String,
}

/// A parsed OPF package: manifest items and spine in document order, plus the metadata the
/// parser cares about. Hrefs are resolved against the archive/directory root at parse time.
struct OpfPackage {
	items: Vec<(String, ManifestItem)>,
	spine: Vec<String>,
	title: Option<String>,
	author: Option<String>,
}

impl OpfPackage {
	fn item(&self, id: &str) -> Option<&ManifestItem> {
		self.items.iter().find(|(item_id, _)| item_id == id).map(|(_, item)| item)
	}
}

fn parse_opf_package(opf_content: &str, opf_dir: &str) -> Result<OpfPackage> {
	let doc =
		XmlDocument::parse_with_options(opf_content, ParsingOptions { allow_dtd: true, ..ParsingOptions::default() })
			.context("Failed to parse OPF XML")?;
	let mut items = Vec::new();
	let mut spine = Vec::new();
	let mut title = None;
	let mut author = None;
	if let Some(package) =
		doc.descendants().find(|n| n.node_type() == NodeType::Element && n.tag_name().name() == "package")
	{
		for child in package.children() {
			if !child.is_element() {
				continue;
			}
			match child.tag_name().name() {
				"metadata" => {
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
				}
				"manifest" => {
					for item in child.children().filter(|n| n.is_element() && n.tag_name().name() == "item") {
						let Some(href) = item.attribute("href") else { continue };
						let id = item.attribute("id").unwrap_or("").to_string();
						let media_type = item.attribute("media-type").unwrap_or("").to_string();
						items.push((id, ManifestItem { href: resolve_relative_path(opf_dir, href), media_type }));
					}
				}
				"spine" => {
					for itemref in child.children().filter(|n| n.is_element() && n.tag_name().name() == "itemref") {
						if let Some(idref) = itemref.attribute("idref") {
							spine.push(idref.to_string());
						}
					}
				}
				_ => {}
			}
		}
	}
	Ok(OpfPackage { items, spine, title, author })
}

fn is_dtbook_item(item: &ManifestItem) -> bool {
	item.media_type == "application/x-dtbook+xml"
}

/// Finds the single `DTBook` XML file for a legacy single-file DAISY book: the manifest's
/// declared `DTBook` item, or failing that, the first plain `.xml` item.
fn find_single_dtbook_href(package: &OpfPackage) -> Option<String> {
	let mut fallback = None;
	for (_, item) in &package.items {
		if is_dtbook_item(item) {
			return Some(item.href.clone());
		}
		if item.media_type == "text/xml"
			&& fallback.is_none()
			&& Path::new(&item.href).extension().is_some_and(|ext| ext.eq_ignore_ascii_case("xml"))
		{
			fallback = Some(item.href.clone());
		}
	}
	fallback
}

fn dir_of(resolved_path: &str) -> String {
	resolved_path.rsplit_once('/').map_or_else(String::new, |(dir, _)| dir.to_string())
}

/// Converts one `DTBook` XML file into the shared buffer, offsetting its markers and id
/// positions by the buffer's current length. A no-op if `href` was already converted.
fn convert_dtbook_file(
	href: &str,
	read_text: &mut dyn FnMut(&str) -> Result<String>,
	render_tables_inline: bool,
	buffer: &mut DocumentBuffer,
	id_positions: &mut HashMap<String, usize>,
	file_offsets: &mut HashMap<String, usize>,
	file_ids: &mut HashMap<String, HashMap<String, usize>>,
) -> Result<()> {
	if file_offsets.contains_key(href) {
		return Ok(());
	}
	let xml_content = read_text(href)?;
	let mut converter = XmlToText::with_render_tables_inline(render_tables_inline);
	if !converter.convert(&xml_content) {
		// TRANSLATORS: Error shown when a DAISY book's DTBook XML fails to convert to plain text
		anyhow::bail!(t("Failed to convert DTBook XML to text"));
	}
	let offset = buffer.current_position();
	buffer.append(&converter.get_text());
	add_converter_markers(buffer, &converter, offset);
	for pb in converter.get_page_breaks() {
		buffer.add_marker(Marker::new(MarkerType::PageBreak, offset + pb.offset).with_text(pb.text.clone()));
	}
	for (id, pos) in converter.get_id_positions() {
		// Bare ids are only unique within their own file, so keep the first file's position
		// and expose a path-qualified key alongside it.
		let absolute = offset + pos;
		id_positions.entry(id.clone()).or_insert(absolute);
		id_positions.insert(format!("{href}#{id}"), absolute);
	}
	file_ids.insert(href.to_string(), converter.get_id_positions().clone());
	file_offsets.insert(href.to_string(), offset);
	Ok(())
}

/// Builds a multi-file DAISY 3 document by walking the OPF spine, converting each `DTBook`
/// XML section (directly, or via a SMIL file's `<text>` references) into the shared buffer
/// and turning every SMIL `<par>` into an `AudioClip` anchored at that text's position.
/// Chapters that fail to convert are skipped. Returns `None` when the spine is empty or
/// names nothing convertible, so the caller can fall back to single-file handling.
fn build_daisy_document(
	package: &OpfPackage,
	title: String,
	author: String,
	render_tables_inline: bool,
	read_text: &mut dyn FnMut(&str) -> Result<String>,
	resolve_audio: &dyn Fn(&str) -> AudioLocation,
) -> Option<Document> {
	if package.spine.is_empty() {
		return None;
	}
	let mut buffer = DocumentBuffer::new();
	let mut id_positions: HashMap<String, usize> = HashMap::new();
	let mut file_offsets: HashMap<String, usize> = HashMap::new();
	let mut file_ids: HashMap<String, HashMap<String, usize>> = HashMap::new();
	let mut source_indices: HashMap<String, usize> = HashMap::new();
	let mut audio_builder = AudioTimelineBuilder::new();
	let mut pending_clips: Vec<(usize, u64, u64, usize)> = Vec::new();
	let mut converted_any = false;

	for idref in &package.spine {
		let Some(item) = package.item(idref) else { continue };
		if is_dtbook_item(item) {
			// One malformed chapter shouldn't cost the reader the whole book: skip it and keep
			// assembling the rest instead of falling back to single-file handling.
			if convert_dtbook_file(
				&item.href,
				read_text,
				render_tables_inline,
				&mut buffer,
				&mut id_positions,
				&mut file_offsets,
				&mut file_ids,
			)
			.is_err()
			{
				continue;
			}
			converted_any = true;
			continue;
		}
		if !item.href.to_ascii_lowercase().ends_with(".smil") {
			continue;
		}
		let Ok(smil_content) = read_text(&item.href) else { continue };
		let smil_dir = dir_of(&item.href);
		// A `<text src>` naming only a fragment resolves against the file the most recent
		// explicit reference in this same SMIL named.
		let mut current_text_href: Option<String> = None;
		for par in parse_smil_pars(&smil_content) {
			let text_href = match &par.text_file {
				Some(text_file) => {
					let href = resolve_relative_path(&smil_dir, text_file);
					current_text_href = Some(href.clone());
					href
				}
				None => {
					let Some(href) = current_text_href.clone() else { continue };
					href
				}
			};
			if !file_offsets.contains_key(&text_href) {
				if convert_dtbook_file(
					&text_href,
					read_text,
					render_tables_inline,
					&mut buffer,
					&mut id_positions,
					&mut file_offsets,
					&mut file_ids,
				)
				.is_err()
				{
					continue;
				}
				converted_any = true;
			}
			let (Some(&base), Some(&local_pos)) =
				(file_offsets.get(&text_href), file_ids.get(&text_href).and_then(|ids| ids.get(&par.text_id)))
			else {
				continue;
			};
			let audio_href = resolve_relative_path(&smil_dir, &par.audio_src);
			let source = *source_indices
				.entry(audio_href.clone())
				.or_insert_with(|| audio_builder.add_source(resolve_audio(&audio_href), None));
			pending_clips.push((source, par.clip_begin_ms, par.clip_end_ms, base + local_pos));
		}
	}

	if !converted_any {
		return None;
	}

	let mut toc_items = None;
	if let Some((_, ncx_item)) = package.items.iter().find(|(_, item)| {
		item.media_type == "application/x-dtbncx+xml" || item.href.to_ascii_lowercase().ends_with(".ncx")
	}) && let Ok(ncx_content) = read_text(&ncx_item.href)
		&& !ncx_content.is_empty()
		&& let Some(ncx_toc) = parse_daisy_ncx(&ncx_content, &id_positions)
		&& !ncx_toc.is_empty()
	{
		toc_items = Some(ncx_toc);
	}
	let toc_items = toc_items.unwrap_or_else(|| build_toc_from_buffer(&buffer));

	pending_clips.sort_by_key(|clip| clip.3);
	let doc_end = buffer.current_position();
	for index in 0..pending_clips.len() {
		let (source, begin_ms, end_ms, start) = pending_clips[index];
		let end = pending_clips.get(index + 1).map_or(doc_end, |next| next.3);
		audio_builder.add_clip(source, begin_ms, end_ms, start, end);
	}
	let audio = audio_builder.build();

	Some(Document {
		title,
		author,
		buffer,
		toc_items,
		id_positions,
		audio: (!audio.is_empty()).then_some(audio),
		..Document::default()
	})
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
		assert!(document.audio.is_none());
	}

	fn write_zip(entries: &[(&str, &[u8])]) -> Vec<u8> {
		use std::io::Write;

		use zip::{ZipWriter, write::FileOptions};

		let mut buf = Vec::new();
		{
			let cursor = std::io::Cursor::new(&mut buf);
			let mut writer = ZipWriter::new(cursor);
			for (name, data) in entries {
				writer.start_file(*name, FileOptions::<()>::default()).unwrap();
				writer.write_all(data).unwrap();
			}
			writer.finish().unwrap();
		}
		buf
	}

	/// A minimal two-section DAISY 3 "full text, full audio" book, exercising the multi-file
	/// offset math and the SMIL-to-text-position audio linkage end to end.
	#[test]
	fn parses_multi_file_daisy_book_with_audio() {
		let opf = br#"<?xml version="1.0" encoding="UTF-8"?>
<package xmlns="http://openebook.org/namespaces/oeb-package/1.0/" unique-identifier="bookid">
  <metadata>
    <dc-metadata xmlns:dc="http://purl.org/dc/elements/1.1/">
      <dc:Title>Two Section Book</dc:Title>
      <dc:Creator>A. Author</dc:Creator>
    </dc-metadata>
  </metadata>
  <manifest>
    <item id="xml1" href="book1.xml" media-type="application/x-dtbook+xml" />
    <item id="xml2" href="book2.xml" media-type="application/x-dtbook+xml" />
    <item id="ncx" href="book.ncx" media-type="application/x-dtbncx+xml" />
    <item id="audio1" href="book1.mp3" media-type="audio/mpeg" />
    <item id="audio2" href="book2.mp3" media-type="audio/mpeg" />
    <item id="smil1" href="section1.smil" media-type="application/smil" />
    <item id="smil2" href="section2.smil" media-type="application/smil" />
  </manifest>
  <spine>
    <itemref idref="smil1" />
    <itemref idref="smil2" />
  </spine>
</package>"#;
		let book1 = br#"<?xml version="1.0" encoding="UTF-8"?>
<dtbook><book><bodymatter>
<h1 id="h1">Chapter One</h1>
<p id="p1">First paragraph.</p>
</bodymatter></book></dtbook>"#;
		let book2 = br#"<?xml version="1.0" encoding="UTF-8"?>
<dtbook><book><bodymatter>
<h1 id="h2">Chapter Two</h1>
<p id="p2">Second paragraph.</p>
</bodymatter></book></dtbook>"#;
		let smil1 = br#"<smil xmlns="http://www.w3.org/2001/SMIL20/"><body><seq id="s">
<par id="par_h1"><text src="book1.xml#h1" /><audio src="book1.mp3" clipBegin="0s" clipEnd="2s" /></par>
<par id="par_p1"><text src="book1.xml#p1" /><audio src="book1.mp3" clipBegin="2s" clipEnd="5s" /></par>
</seq></body></smil>"#;
		let smil2 = br#"<smil xmlns="http://www.w3.org/2001/SMIL20/"><body><seq id="s">
<par id="par_h2"><text src="book2.xml#h2" /><audio src="book2.mp3" clipBegin="0s" clipEnd="1.5s" /></par>
<par id="par_p2"><text src="book2.xml#p2" /><audio src="book2.mp3" clipBegin="1.5s" clipEnd="4s" /></par>
</seq></body></smil>"#;
		let ncx = br#"<?xml version="1.0" encoding="UTF-8"?>
<ncx xmlns="http://www.daisy.org/z3986/2005/ncx/"><navMap>
<navPoint id="np1"><navLabel><text>Chapter One</text></navLabel><content src="book1.xml#h1" /></navPoint>
<navPoint id="np2"><navLabel><text>Chapter Two</text></navLabel><content src="book2.xml#h2" /></navPoint>
</navMap></ncx>"#;

		let zip_bytes = write_zip(&[
			("book.opf", opf.as_slice()),
			("book1.xml", book1.as_slice()),
			("book2.xml", book2.as_slice()),
			("book.ncx", ncx.as_slice()),
			("section1.smil", smil1.as_slice()),
			("section2.smil", smil2.as_slice()),
			("book1.mp3", b"fake-mp3-1"),
			("book2.mp3", b"fake-mp3-2"),
		]);

		let dir = TempDir::new("daisy_multi");
		let zip_path = dir.path().join("book.zip");
		fs::write(&zip_path, &zip_bytes).expect("write zip");

		let context = ParserContext::new(zip_path.to_string_lossy().to_string());
		let document = DaisyParser.parse(&context).expect("multi-file DAISY parse should succeed");

		assert_eq!(document.title, "Two Section Book");
		assert_eq!(document.author, "A. Author");
		assert!(document.buffer.content.contains("Chapter One"));
		assert!(document.buffer.content.contains("Chapter Two"));
		assert!(
			document.buffer.content.find("Chapter One").unwrap() < document.buffer.content.find("Chapter Two").unwrap()
		);
		assert_eq!(document.toc_items.len(), 2);
		assert_eq!(document.toc_items[1].name, "Chapter Two");

		let audio = document.audio.expect("audio timeline should be populated");
		assert_eq!(audio.clips().len(), 4);
		assert_eq!(audio.sources().len(), 2);
		// The second file's clips resume the elapsed clock where the first file's left off.
		assert_eq!(audio.total_duration_ms(), 2000 + 3000 + 1500 + 2500);
		let h2_pos = document.buffer.content.find("Chapter Two").unwrap();
		let clip_index = audio.clip_index_at_position(h2_pos).expect("chapter two should be narrated");
		assert_eq!(audio.clip_start_ms(clip_index), Some(5000));
	}

	/// Two chapters can legally reuse the same bare id; the merged map keeps the first file's
	/// position while still exposing both via their path-qualified keys.
	#[test]
	fn multi_file_daisy_book_keeps_first_occurrence_of_a_duplicate_bare_id() {
		let opf = br#"<?xml version="1.0" encoding="UTF-8"?>
<package xmlns="http://openebook.org/namespaces/oeb-package/1.0/" unique-identifier="bookid">
  <metadata>
    <dc-metadata xmlns:dc="http://purl.org/dc/elements/1.1/">
      <dc:Title>Duplicate Id Book</dc:Title>
    </dc-metadata>
  </metadata>
  <manifest>
    <item id="xml1" href="book1.xml" media-type="application/x-dtbook+xml" />
    <item id="xml2" href="book2.xml" media-type="application/x-dtbook+xml" />
  </manifest>
  <spine>
    <itemref idref="xml1" />
    <itemref idref="xml2" />
  </spine>
</package>"#;
		let book1 = br#"<?xml version="1.0" encoding="UTF-8"?>
<dtbook><book><bodymatter><p id="dup">First occurrence.</p></bodymatter></book></dtbook>"#;
		let book2 = br#"<?xml version="1.0" encoding="UTF-8"?>
<dtbook><book><bodymatter><p id="dup">Second occurrence.</p></bodymatter></book></dtbook>"#;

		let zip_bytes = write_zip(&[
			("book.opf", opf.as_slice()),
			("book1.xml", book1.as_slice()),
			("book2.xml", book2.as_slice()),
		]);

		let dir = TempDir::new("daisy_dup_id");
		let zip_path = dir.path().join("book.zip");
		fs::write(&zip_path, &zip_bytes).expect("write zip");

		let context = ParserContext::new(zip_path.to_string_lossy().to_string());
		let document = DaisyParser.parse(&context).expect("DAISY parse should succeed");

		let first_pos = document.buffer.content.find("First occurrence.").unwrap();
		let second_pos = document.buffer.content.find("Second occurrence.").unwrap();
		assert_eq!(
			document.id_positions.get("dup").copied(),
			Some(first_pos),
			"bare id should keep the first file's position"
		);
		assert_eq!(document.id_positions.get("book1.xml#dup").copied(), Some(first_pos));
		assert_eq!(document.id_positions.get("book2.xml#dup").copied(), Some(second_pos));
	}

	/// A `<text src="#id">` with no file part resolves against the most recent explicit
	/// `<text src="file#id">` in the same SMIL, rather than losing that clip.
	#[test]
	fn smil_bare_fragment_resolves_against_the_last_explicit_text_file() {
		let opf = br#"<?xml version="1.0" encoding="UTF-8"?>
<package xmlns="http://openebook.org/namespaces/oeb-package/1.0/" unique-identifier="bookid">
  <metadata>
    <dc-metadata xmlns:dc="http://purl.org/dc/elements/1.1/">
      <dc:Title>Bare Fragment Book</dc:Title>
    </dc-metadata>
  </metadata>
  <manifest>
    <item id="xml1" href="book1.xml" media-type="application/x-dtbook+xml" />
    <item id="audio1" href="book1.mp3" media-type="audio/mpeg" />
    <item id="smil1" href="section1.smil" media-type="application/smil" />
  </manifest>
  <spine>
    <itemref idref="smil1" />
  </spine>
</package>"#;
		let book1 = br#"<?xml version="1.0" encoding="UTF-8"?>
<dtbook><book><bodymatter>
<p id="p1">First sentence.</p>
<p id="p2">Second sentence.</p>
</bodymatter></book></dtbook>"#;
		let smil1 = br##"<smil xmlns="http://www.w3.org/2001/SMIL20/"><body><seq id="s">
<par id="par_p1"><text src="book1.xml#p1" /><audio src="book1.mp3" clipBegin="0s" clipEnd="2s" /></par>
<par id="par_p2"><text src="#p2" /><audio src="book1.mp3" clipBegin="2s" clipEnd="4s" /></par>
</seq></body></smil>"##;

		let zip_bytes = write_zip(&[
			("book.opf", opf.as_slice()),
			("book1.xml", book1.as_slice()),
			("section1.smil", smil1.as_slice()),
			("book1.mp3", b"fake-mp3"),
		]);

		let dir = TempDir::new("daisy_bare_fragment");
		let zip_path = dir.path().join("book.zip");
		fs::write(&zip_path, &zip_bytes).expect("write zip");

		let context = ParserContext::new(zip_path.to_string_lossy().to_string());
		let document = DaisyParser.parse(&context).expect("DAISY parse should succeed");

		let audio = document.audio.expect("audio timeline should be populated");
		assert_eq!(audio.clips().len(), 2, "the bare-fragment par's clip must not be dropped");
		let p2_pos = document.buffer.content.find("Second sentence.").unwrap();
		let clip_index = audio.clip_index_at_position(p2_pos).expect("second sentence should be narrated");
		assert_eq!(audio.clip_start_ms(clip_index), Some(2000));
	}

	/// One unparseable chapter shouldn't cost the reader the rest of the book.
	#[test]
	fn corrupt_chapter_does_not_abort_the_whole_book() {
		let opf = br#"<?xml version="1.0" encoding="UTF-8"?>
<package xmlns="http://openebook.org/namespaces/oeb-package/1.0/" unique-identifier="bookid">
  <metadata>
    <dc-metadata xmlns:dc="http://purl.org/dc/elements/1.1/">
      <dc:Title>Partly Corrupt Book</dc:Title>
    </dc-metadata>
  </metadata>
  <manifest>
    <item id="xml1" href="book1.xml" media-type="application/x-dtbook+xml" />
    <item id="xml2" href="book2.xml" media-type="application/x-dtbook+xml" />
  </manifest>
  <spine>
    <itemref idref="xml1" />
    <itemref idref="xml2" />
  </spine>
</package>"#;
		let book1 = br#"<?xml version="1.0" encoding="UTF-8"?>
<dtbook><book><bodymatter><p id="p1">Valid chapter text.</p></bodymatter></book></dtbook>"#;
		// Deliberately malformed: unclosed tags fail XML parsing outright.
		let book2 = b"<dtbook><book><bodymatter><p id=\"p2\">Broken chapter";

		let zip_bytes = write_zip(&[
			("book.opf", opf.as_slice()),
			("book1.xml", book1.as_slice()),
			("book2.xml", book2.as_slice()),
		]);

		let dir = TempDir::new("daisy_corrupt_chapter");
		let zip_path = dir.path().join("book.zip");
		fs::write(&zip_path, &zip_bytes).expect("write zip");

		let context = ParserContext::new(zip_path.to_string_lossy().to_string());
		let document = DaisyParser.parse(&context).expect("DAISY parse should succeed despite one corrupt chapter");

		assert!(document.buffer.content.contains("Valid chapter text."));
	}
}
