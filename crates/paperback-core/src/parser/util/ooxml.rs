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

use crate::{parser::PASSWORD_REQUIRED_ERROR_PREFIX, t, util::zip::read_zip_entry_by_name};

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
							&& !id.is_empty() && !target.is_empty()
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

/// If `path` looks like an encrypted OLE compound file (has an `EncryptionInfo` stream),
/// attempts to decrypt it with `password` and returns the decrypted bytes. Shared by `word`'s
/// `.docx` parsing and `powerpoint`'s `.pptx` parsing, since both formats wrap the same
/// OLE-container encryption when password-protected.
///
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

	/// A missing rels part is normal (a document with no hyperlinks has none), so it must read
	/// as "no relationships" rather than failing the parse.
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
