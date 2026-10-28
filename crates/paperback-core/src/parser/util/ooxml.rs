use std::{
	collections::HashMap,
	fs::File,
	io::{Read, Seek},
};

use anyhow::{Context, Result};
use cfb::CompoundFile;
use office_crypto::decrypt_from_file;
use roxmltree::{Node, NodeType};
use zip::ZipArchive;

use crate::{
	parser::{PASSWORD_REQUIRED_ERROR_PREFIX, util::path::resolve_relative_path},
	t,
	util::zip::read_zip_entry_by_name,
};

#[derive(Debug, Default)]
pub struct Relationships {
	pub links: HashMap<String, String>,
	/// Resolved against the part's own folder: `media/image1.png` beside `word/document.xml` is `word/media/image1.png`.
	pub images: HashMap<String, String>,
}

pub fn read_relationships<R: Read + Seek>(archive: &mut ZipArchive<R>, rels_path: &str) -> Relationships {
	let links = read_ooxml_relationships(archive, rels_path);
	let mut images = HashMap::new();
	let part_folder = rels_path.rsplit_once("/_rels/").map_or("", |(folder, _)| folder);
	if let Ok(content) = read_zip_entry_by_name(archive, rels_path)
		&& let Ok(document) = roxmltree::Document::parse(&content)
	{
		for relationship in document.descendants().filter(|node| node.tag_name().name() == "Relationship") {
			let is_image = relationship.attribute("Type").is_some_and(|kind| kind.ends_with("/image"));
			if !is_image || relationship.attribute("TargetMode") == Some("External") {
				continue;
			}
			if let (Some(id), Some(target)) = (relationship.attribute("Id"), relationship.attribute("Target")) {
				let entry =
					target.strip_prefix('/').map_or_else(|| resolve_relative_path(part_folder, target), str::to_string);
				images.insert(id.to_string(), entry);
			}
		}
	}
	Relationships { links, images }
}

pub fn read_core_properties<R: Read + Seek>(archive: &mut ZipArchive<R>) -> (Option<String>, Option<String>) {
	let Ok(content) = read_zip_entry_by_name(archive, "docProps/core.xml") else { return (None, None) };
	let Ok(document) = roxmltree::Document::parse(&content) else { return (None, None) };
	let field = |name: &str| {
		document
			.descendants()
			.find(|node| node.tag_name().name() == name)
			.and_then(|node| node.text())
			.map(str::trim)
			.filter(|text| !text.is_empty())
			.map(str::to_string)
	};
	(field("title"), field("creator"))
}

pub fn read_ooxml_relationships<R: Read + Seek>(
	archive: &mut ZipArchive<R>,
	rels_path: &str,
) -> HashMap<String, String> {
	let mut rels = HashMap::new();
	match read_zip_entry_by_name(archive, rels_path) {
		Ok(rels_content) => match roxmltree::Document::parse(&rels_content) {
			Ok(rels_doc) => {
				for node in rels_doc.descendants() {
					if node.node_type() == NodeType::Element && node.tag_name().name() == "Relationship" {
						let id = node.attribute("Id").unwrap_or("").to_string();
						let target = node.attribute("Target").unwrap_or("").to_string();
						let rel_type = node.attribute("Type").unwrap_or("");
						if rel_type == "http://schemas.openxmlformats.org/officeDocument/2006/relationships/hyperlink"
							&& !id.is_empty()
							&& !target.is_empty()
						{
							rels.insert(id, target);
						}
					}
				}
			}
			Err(error) => {
				tracing::warn!(rels_path = %rels_path, error = %error, "ooxml relationships part present but failed to parse, hyperlinks in this part will be lost");
			}
		},
		Err(_) => {
			tracing::debug!(rels_path = %rels_path, "ooxml relationships part absent, no hyperlinks for this part");
		}
	}
	rels
}

#[must_use]
pub fn collect_ooxml_run_text(run_element: Node) -> String {
	let mut text = String::new();
	for child in run_element.children() {
		if child.node_type() != NodeType::Element {
			continue;
		}
		match child.tag_name().name() {
			"t" => {
				if let Some(t) = child.text() {
					text.push_str(t);
				}
			}
			"tab" => text.push('\t'),
			"br" => text.push('\n'),
			_ => {}
		}
	}
	text
}

/// Password-protected `.docx` and `.pptx` both wrap the zip in the same OLE-container encryption.
pub fn try_decrypt_office_file(path: &str, password: Option<&str>) -> Result<Option<Vec<u8>>> {
	let file = File::open(path).with_context(|| format!("Failed to open '{path}'"))?;
	// An unencrypted OOXML file is a plain zip, not a compound file.
	let Ok(compound) = CompoundFile::open(file) else { return Ok(None) };
	// Encrypted OOXML files always contain an EncryptionInfo stream.
	if compound.entry("/EncryptionInfo").is_err() {
		return Ok(None);
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
	use std::io::{Cursor, Write};

	use zip::{ZipWriter, write::FileOptions};

	use super::*;

	const HYPERLINK_TYPE: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/hyperlink";
	const IMAGE_TYPE: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image";

	fn archive_containing(entry: &str, contents: &str) -> ZipArchive<Cursor<Vec<u8>>> {
		let mut cursor = Cursor::new(Vec::new());
		{
			let mut writer = ZipWriter::new(&mut cursor);
			writer.start_file(entry, FileOptions::<()>::default()).expect("start file");
			writer.write_all(contents.as_bytes()).expect("write entry");
			writer.finish().expect("finish zip");
		}
		cursor.set_position(0);
		ZipArchive::new(cursor).expect("open zip")
	}

	#[test]
	fn the_core_properties_give_the_title_and_author() {
		let core = r#"<cp:coreProperties xmlns:cp="http://schemas.openxmlformats.org/package/2006/metadata/core-properties" xmlns:dc="http://purl.org/dc/elements/1.1/"><dc:title>Annual Report</dc:title><dc:creator>Ann Smith</dc:creator></cp:coreProperties>"#;
		let mut archive = archive_containing("docProps/core.xml", core);
		assert_eq!(read_core_properties(&mut archive), (Some("Annual Report".into()), Some("Ann Smith".into())));
		let mut blank = archive_containing("docProps/core.xml", "<coreProperties><title> </title></coreProperties>");
		assert_eq!(read_core_properties(&mut blank), (None, None));
	}

	fn rels_xml(body: &str) -> String {
		format!(
			r#"<?xml version="1.0" encoding="UTF-8"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">{body}</Relationships>"#
		)
	}

	fn run_element_text(xml: &str) -> String {
		let doc = roxmltree::Document::parse(xml).expect("parse run xml");
		let run = doc.descendants().find(|node| node.tag_name().name() == "r").expect("run element");
		collect_ooxml_run_text(run)
	}

	#[test]
	fn reads_hyperlink_relationships_by_id() {
		let xml = rels_xml(&format!(
			r#"<Relationship Id="rId1" Type="{HYPERLINK_TYPE}" Target="https://example.com"/>
			<Relationship Id="rId2" Type="{HYPERLINK_TYPE}" Target="https://example.org"/>"#
		));
		let mut archive = archive_containing("word/_rels/document.xml.rels", &xml);
		let rels = read_ooxml_relationships(&mut archive, "word/_rels/document.xml.rels");
		assert_eq!(rels.len(), 2);
		assert_eq!(rels.get("rId1").map(String::as_str), Some("https://example.com"));
		assert_eq!(rels.get("rId2").map(String::as_str), Some("https://example.org"));
	}

	#[test]
	fn ignores_relationships_that_are_not_hyperlinks() {
		let xml = rels_xml(&format!(r#"<Relationship Id="rId1" Type="{IMAGE_TYPE}" Target="media/image1.png"/>"#));
		let mut archive = archive_containing("word/_rels/document.xml.rels", &xml);
		assert!(read_ooxml_relationships(&mut archive, "word/_rels/document.xml.rels").is_empty());
	}

	#[test]
	fn skips_hyperlinks_missing_an_id_or_target() {
		let xml = rels_xml(&format!(
			r#"<Relationship Type="{HYPERLINK_TYPE}" Target="https://example.com"/>
			<Relationship Id="rId2" Type="{HYPERLINK_TYPE}" Target=""/>
			<Relationship Id="rId3" Type="{HYPERLINK_TYPE}" Target="https://example.net"/>"#
		));
		let mut archive = archive_containing("word/_rels/document.xml.rels", &xml);
		let rels = read_ooxml_relationships(&mut archive, "word/_rels/document.xml.rels");
		assert_eq!(rels.keys().collect::<Vec<_>>(), vec!["rId3"]);
	}

	/// A document with no hyperlinks has no rels part, so absence must not fail the parse.
	#[test]
	fn returns_empty_when_the_rels_entry_is_absent() {
		let mut archive = archive_containing("word/document.xml", "<document/>");
		assert!(read_ooxml_relationships(&mut archive, "word/_rels/document.xml.rels").is_empty());
	}

	#[test]
	fn returns_empty_for_malformed_rels_xml() {
		let mut archive = archive_containing("word/_rels/document.xml.rels", "<Relationships><oops>");
		assert!(read_ooxml_relationships(&mut archive, "word/_rels/document.xml.rels").is_empty());
	}

	#[test]
	fn run_text_concatenates_text_tabs_and_breaks() {
		let text = run_element_text("<r><t>one</t><tab/><t>two</t><br/><t>three</t></r>");
		assert_eq!(text, "one\ttwo\nthree");
	}

	#[test]
	fn run_text_ignores_formatting_children_and_empty_text() {
		let text = run_element_text("<r><rPr><b/></rPr><t></t><t>visible</t><noProof/></r>");
		assert_eq!(text, "visible");
	}

	#[test]
	fn run_text_is_empty_for_a_run_with_no_content() {
		assert_eq!(run_element_text("<r/>"), "");
	}
}
