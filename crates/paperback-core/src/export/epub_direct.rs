use std::{
	collections::HashMap,
	fs::File,
	io::{BufReader, Read, Seek},
	path::Path,
};

use anyhow::{Context, Result};
use roxmltree::{Document as XmlDoc, NodeType, ParsingOptions};
use zip::ZipArchive;

use crate::util::{text::url_decode, zip::read_zip_entry_by_name};

/// Render an EPUB directly to HTML by stitching spine-item bodies.
///
/// Bypasses the text-buffer pipeline entirely — no `HtmlToText`, no marker
/// reconstruction — making it dramatically faster on large EPUBs.
pub fn render(file_path: &str) -> Result<String> {
	let file = File::open(file_path).with_context(|| format!("failed to open '{file_path}'"))?;
	let mut archive = ZipArchive::new(BufReader::new(file)).context("failed to read EPUB as zip")?;

	let opf_path = find_opf_path(&mut archive)?;
	let opf_dir =
		Path::new(&opf_path).parent().map(|d| d.to_string_lossy().into_owned().replace('\\', "/")).unwrap_or_default();
	let opf_content = read_zip_entry_by_name(&mut archive, &opf_path)?;
	let (manifest, spine, title) = parse_opf(&opf_content, &opf_dir)?;

	// spine file path → <section> id
	let spine_path_to_id: HashMap<String, String> =
		spine.iter().filter_map(|id| manifest.get(id).map(|(path, _)| (path.clone(), path_to_id(path)))).collect();

	let mut out = format!(
		"<!DOCTYPE html>\n<html>\n<head>\n<meta charset=\"utf-8\">\n<title>{}</title>\n</head>\n<body>\n",
		escape_html(&title)
	);

	for idref in &spine {
		let Some((path, _)) = manifest.get(idref) else { continue };
		let Ok(content) = read_zip_entry_by_name(&mut archive, path) else { continue };

		let section_id = path_to_id(path);
		out.push_str(&format!("<section id=\"{}\">\n", escape_attr(&section_id)));

		let body = extract_body(&content);
		let file_dir = Path::new(path.as_str())
			.parent()
			.map(|d| d.to_string_lossy().into_owned().replace('\\', "/"))
			.unwrap_or_default();
		rewrite_hrefs_into(body, &file_dir, &spine_path_to_id, &mut out);
		out.push_str("\n</section>\n");
	}

	out.push_str("</body>\n</html>\n");
	Ok(out)
}

fn find_opf_path<R: Read + Seek>(archive: &mut ZipArchive<R>) -> Result<String> {
	let container =
		read_zip_entry_by_name(archive, "META-INF/container.xml").context("failed to read META-INF/container.xml")?;
	let doc = XmlDoc::parse_with_options(&container, ParsingOptions { allow_dtd: true, ..Default::default() })
		.context("failed to parse container.xml")?;
	doc.descendants()
		.find(|n| n.node_type() == NodeType::Element && n.tag_name().name() == "rootfile")
		.and_then(|n| n.attribute("full-path"))
		.map(str::to_string)
		.ok_or_else(|| anyhow::anyhow!("rootfile not found in container.xml"))
}

/// Returns `(manifest: id → (path, media_type), spine order, document title)`.
fn parse_opf(content: &str, opf_dir: &str) -> Result<(HashMap<String, (String, String)>, Vec<String>, String)> {
	let doc = XmlDoc::parse_with_options(content, ParsingOptions { allow_dtd: true, ..Default::default() })
		.context("failed to parse OPF")?;

	let mut manifest: HashMap<String, (String, String)> = HashMap::new();
	let mut spine: Vec<String> = Vec::new();
	let mut title = String::new();

	for node in doc.descendants() {
		if node.node_type() != NodeType::Element {
			continue;
		}
		match node.tag_name().name() {
			"title" if title.is_empty() => {
				if let Some(t) = node.text() {
					title = t.to_string();
				}
			}
			"item" => {
				let (Some(id), Some(href)) = (node.attribute("id"), node.attribute("href")) else { continue };
				let media_type = node.attribute("media-type").unwrap_or("").to_string();
				let decoded = url_decode(href);
				let path = if opf_dir.is_empty() {
					normalize_epub_path(&decoded)
				} else {
					normalize_epub_path(&format!("{opf_dir}/{decoded}"))
				};
				manifest.insert(id.to_string(), (path, media_type));
			}
			"itemref" => {
				if let Some(idref) = node.attribute("idref") {
					spine.push(idref.to_string());
				}
			}
			_ => {}
		}
	}

	if title.is_empty() {
		title = "Document".to_string();
	}
	Ok((manifest, spine, title))
}

fn extract_body(html: &str) -> &str {
	let tag_start = html.find("<body").or_else(|| html.find("<BODY"));
	let Some(tag_start) = tag_start else { return html };
	let Some(gt_offset) = html[tag_start..].find('>') else { return html };
	let content_start = tag_start + gt_offset + 1;
	let body_close = html.rfind("</body>").or_else(|| html.rfind("</BODY>")).unwrap_or(html.len());
	if content_start <= body_close { &html[content_start..body_close] } else { html }
}

/// Scan `html` for `href="…"` / `href='…'`, rewrite cross-file links, and append to `out`.
fn rewrite_hrefs_into(html: &str, current_dir: &str, spine_path_to_id: &HashMap<String, String>, out: &mut String) {
	let mut pos = 0;
	while pos < html.len() {
		match html[pos..].find("href=") {
			None => {
				out.push_str(&html[pos..]);
				return;
			}
			Some(rel) => {
				let href_pos = pos + rel;
				out.push_str(&html[pos..href_pos + 5]); // up to and including "href="
				let rest = &html[href_pos + 5..];
				let quote = match rest.as_bytes().first() {
					Some(&b'"') => '"',
					Some(&b'\'') => '\'',
					_ => {
						pos = href_pos + 5;
						continue;
					}
				};
				out.push(quote);
				let after_q = &rest[1..];
				match after_q.find(quote) {
					None => {
						out.push_str(after_q);
						return;
					}
					Some(val_end) => {
						let value = &after_q[..val_end];
						out.push_str(&rewrite_single_href(value, current_dir, spine_path_to_id));
						out.push(quote);
						pos = href_pos + 5 + 1 + val_end + 1;
					}
				}
			}
		}
	}
}

fn rewrite_single_href(href: &str, current_dir: &str, spine_path_to_id: &HashMap<String, String>) -> String {
	if href.is_empty() || href.starts_with('#') {
		return href.to_string();
	}
	if href.contains("://") || href.starts_with("mailto:") || href.starts_with("data:") {
		return href.to_string();
	}
	let (path_part, fragment) = match href.find('#') {
		Some(i) => (&href[..i], Some(&href[i + 1..])),
		None => (href, None),
	};
	let decoded = url_decode(path_part);
	let resolved = if current_dir.is_empty() {
		normalize_epub_path(&decoded)
	} else {
		normalize_epub_path(&format!("{current_dir}/{decoded}"))
	};
	match spine_path_to_id.get(&resolved) {
		Some(section_id) => match fragment {
			None | Some("") => format!("#{section_id}"),
			Some(frag) => format!("#{frag}"),
		},
		None => href.to_string(),
	}
}

fn normalize_epub_path(path: &str) -> String {
	let mut parts: Vec<&str> = Vec::new();
	for seg in path.split('/') {
		match seg {
			".." => {
				parts.pop();
			}
			"." | "" => {}
			s => parts.push(s),
		}
	}
	parts.join("/")
}

fn path_to_id(path: &str) -> String {
	path.chars().map(|c| if c.is_ascii_alphanumeric() || c == '-' { c } else { '-' }).collect()
}

fn escape_html(s: &str) -> String {
	let mut out = String::with_capacity(s.len());
	for ch in s.chars() {
		match ch {
			'&' => out.push_str("&amp;"),
			'<' => out.push_str("&lt;"),
			'>' => out.push_str("&gt;"),
			c => out.push(c),
		}
	}
	out
}

fn escape_attr(s: &str) -> String {
	s.replace('&', "&amp;").replace('"', "&quot;")
}
