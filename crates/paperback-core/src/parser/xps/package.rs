use std::{
	collections::{BTreeMap, HashMap},
	io::{Read, Seek},
};

use anyhow::{Result, bail};
use roxmltree::{Document as XmlDocument, Node};
use zip::ZipArchive;

use crate::{
	parser::util::path::resolve_relative_path,
	util::{encoding::decode_html, zip::read_zip_entry_bytes},
};

const FIXED_REPRESENTATION: &str = "/fixedrepresentation";
pub(super) const CORE_PROPERTIES: &str = "/core-properties";
pub(super) const DOCUMENT_STRUCTURE: &str = "/documentstructure";
pub(super) const STORY_FRAGMENTS: &str = "/storyfragments";

#[derive(Debug, PartialEq, Eq)]
pub(super) struct PagePart {
	pub name: String,
	pub link_targets: Vec<String>,
}

pub(super) struct Package<R> {
	archive: ZipArchive<R>,
	/// Keyed in lower case, since part names ignore case and a reference need not match how the entry is stored.
	parts: HashMap<String, Vec<String>>,
}

/// A printer driver streams a part as pieces while it is still being written, each an entry named for its place in the sequence.
fn piece_number(segment: &str) -> Option<u32> {
	let (number, suffix) = segment.strip_prefix('[')?.split_once(']')?;
	(suffix.eq_ignore_ascii_case(".piece") || suffix.eq_ignore_ascii_case(".last.piece"))
		.then(|| number.parse().ok())?
}

fn part_entries(names: impl Iterator<Item = String>) -> HashMap<String, Vec<String>> {
	let mut whole = HashMap::new();
	let mut pieces: HashMap<String, BTreeMap<u32, String>> = HashMap::new();
	for entry in names {
		match entry.rsplit_once('/').and_then(|(part, last)| Some((part, piece_number(last)?))) {
			Some((part, number)) => {
				pieces.entry(part.to_lowercase()).or_default().insert(number, entry);
			}
			None => {
				whole.insert(entry.to_lowercase(), vec![entry]);
			}
		}
	}
	whole.extend(pieces.into_iter().map(|(part, entries)| (part, entries.into_values().collect())));
	whole
}

pub(super) fn resolve_part(source: &str, target: &str) -> String {
	resolve_relative_path(source.rsplit_once('/').map_or("", |(folder, _)| folder), target)
}

fn relationships_part(source: &str) -> String {
	match source.rsplit_once('/') {
		Some((folder, file)) => format!("{folder}/_rels/{file}.rels"),
		None => format!("_rels/{source}.rels"),
	}
}

pub(super) fn is_element(node: Node, name: &str) -> bool {
	node.is_element() && node.tag_name().name() == name
}

impl<R: Read + Seek> Package<R> {
	pub fn new(archive: ZipArchive<R>) -> Self {
		let parts = part_entries(archive.file_names().flatten().map(String::from));
		Self { archive, parts }
	}

	pub fn read_bytes(&mut self, part: &str) -> Result<Vec<u8>> {
		let Some(entries) = self.parts.get(&part.to_lowercase()) else { bail!("no part named '{part}'") };
		let mut bytes = Vec::new();
		for entry in entries {
			bytes.append(&mut read_zip_entry_bytes(&mut self.archive, entry)?);
		}
		Ok(bytes)
	}

	pub fn read_text(&mut self, part: &str) -> Result<String> {
		Ok(decode_html(&self.read_bytes(part)?, None))
	}

	/// The empty source stands for the package itself.
	pub fn related(&mut self, source: &str, kind: &str) -> Vec<String> {
		let Ok(xml) = self.read_text(&relationships_part(source)) else { return Vec::new() };
		let Ok(document) = XmlDocument::parse(&xml) else {
			tracing::warn!(source, "xps relationships part failed to parse");
			return Vec::new();
		};
		document
			.descendants()
			.filter(|node| is_element(*node, "Relationship"))
			.filter(|node| node.attribute("Type").is_some_and(|found| found.ends_with(kind)))
			.filter_map(|node| node.attribute("Target"))
			.map(|target| resolve_part(source, target))
			.collect()
	}

	pub fn documents(&mut self) -> Vec<String> {
		let mut documents = Vec::new();
		for root in self.related("", FIXED_REPRESENTATION) {
			let Ok(xml) = self.read_text(&root) else { continue };
			let Ok(parsed) = XmlDocument::parse(&xml) else {
				tracing::warn!(part = %root, "xps document sequence failed to parse");
				continue;
			};
			if is_element(parsed.root_element(), "FixedDocument") {
				documents.push(root);
				continue;
			}
			let references = parsed.descendants().filter(|node| is_element(*node, "DocumentReference"));
			documents.extend(
				references.filter_map(|node| node.attribute("Source")).map(|source| resolve_part(&root, source)),
			);
		}
		documents
	}

	pub fn pages(&mut self, document: &str) -> Vec<PagePart> {
		let Ok(xml) = self.read_text(document) else { return Vec::new() };
		let Ok(parsed) = XmlDocument::parse(&xml) else {
			tracing::warn!(part = document, "xps fixed document failed to parse");
			return Vec::new();
		};
		let page_contents = parsed.descendants().filter(|node| is_element(*node, "PageContent"));
		page_contents
			.filter_map(|page| {
				let targets = page.descendants().filter(|node| is_element(*node, "LinkTarget"));
				let link_targets = targets.filter_map(|target| target.attribute("Name")).map(String::from).collect();
				Some(PagePart { name: resolve_part(document, page.attribute("Source")?), link_targets })
			})
			.collect()
	}
}

#[cfg(test)]
impl Package<std::io::Cursor<Vec<u8>>> {
	pub fn from_parts(parts: &[(&str, &str)]) -> Self {
		let parts: Vec<(&str, &[u8])> = parts.iter().map(|(name, text)| (*name, text.as_bytes())).collect();
		Self::from_bytes(&parts)
	}

	pub fn from_bytes(parts: &[(&str, &[u8])]) -> Self {
		use std::io::{Cursor, Write};

		use zip::{ZipWriter, write::FileOptions};
		let mut cursor = Cursor::new(Vec::new());
		let mut writer = ZipWriter::new(&mut cursor);
		for (name, bytes) in parts {
			writer.start_file(*name, FileOptions::<()>::default()).expect("start file");
			writer.write_all(bytes).expect("write file");
		}
		writer.finish().expect("finish zip");
		cursor.set_position(0);
		Self::new(ZipArchive::new(cursor).expect("open zip"))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const ROOT_RELS: &str = r#"<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
		<Relationship Id="R0" Type="http://schemas.microsoft.com/xps/2005/06/fixedrepresentation" Target="/Sequence.fdseq"/>
	</Relationships>"#;
	const SEQUENCE: &str = r#"<FixedDocumentSequence xmlns="http://schemas.microsoft.com/xps/2005/06">
		<DocumentReference Source="Documents/1/Doc.fdoc"/>
	</FixedDocumentSequence>"#;
	const DOCUMENT: &str = r#"<FixedDocument xmlns="http://schemas.microsoft.com/xps/2005/06">
		<PageContent Source="Pages/1.fpage"/>
		<PageContent Source="/Documents/1/Pages/2.fpage"/>
	</FixedDocument>"#;

	#[test]
	fn pages_are_found_through_the_fixed_representation_relationship() {
		let mut package = Package::from_parts(&[
			("_rels/.rels", ROOT_RELS),
			("Sequence.fdseq", SEQUENCE),
			("Documents/1/Doc.fdoc", DOCUMENT),
		]);
		let documents = package.documents();
		assert_eq!(documents, ["Documents/1/Doc.fdoc"]);
		let names: Vec<String> = package.pages(&documents[0]).into_iter().map(|page| page.name).collect();
		assert_eq!(names, ["Documents/1/Pages/1.fpage", "Documents/1/Pages/2.fpage"]);
	}

	#[test]
	fn the_openxps_relationship_prefix_is_followed_too() {
		let rels = r#"<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
			<Relationship Id="R1" Type="http://schemas.openxmlformats.org/package/2006/relationships/metadata/thumbnail" Target="thumb.jpg"/>
			<Relationship Id="R0" Type="http://schemas.openxps.org/oxps/v1.0/fixedrepresentation" Target="Sequence.fdseq"/>
		</Relationships>"#;
		let sequence = r#"<FixedDocumentSequence xmlns="http://schemas.openxps.org/oxps/v1.0">
			<DocumentReference Source="Doc.fdoc"/>
		</FixedDocumentSequence>"#;
		let mut package = Package::from_parts(&[("_rels/.rels", rels), ("Sequence.fdseq", sequence)]);
		assert_eq!(package.documents(), ["Doc.fdoc"]);
	}

	#[test]
	fn every_document_of_a_sequence_is_listed_in_order() {
		let sequence = r#"<FixedDocumentSequence>
			<DocumentReference Source="One.fdoc"/>
			<DocumentReference Source="Two.fdoc"/>
		</FixedDocumentSequence>"#;
		let mut package = Package::from_parts(&[("_rels/.rels", ROOT_RELS), ("Sequence.fdseq", sequence)]);
		assert_eq!(package.documents(), ["One.fdoc", "Two.fdoc"]);
	}

	#[test]
	fn a_relationship_straight_to_a_document_needs_no_sequence() {
		let rels = r#"<Relationships>
			<Relationship Id="R0" Type="http://schemas.microsoft.com/xps/2005/06/fixedrepresentation" Target="Doc.fdoc"/>
		</Relationships>"#;
		let mut package = Package::from_parts(&[("_rels/.rels", rels), ("Doc.fdoc", "<FixedDocument/>")]);
		assert_eq!(package.documents(), ["Doc.fdoc"]);
	}

	#[test]
	fn a_page_keeps_the_link_targets_that_lead_to_it() {
		let document = r#"<FixedDocument>
			<PageContent Source="1.fpage">
				<PageContent.LinkTargets>
					<LinkTarget Name="first"/>
					<LinkTarget Name="second"/>
				</PageContent.LinkTargets>
			</PageContent>
		</FixedDocument>"#;
		let mut package = Package::from_parts(&[("Doc.fdoc", document)]);
		assert_eq!(
			package.pages("Doc.fdoc"),
			[PagePart { name: "1.fpage".into(), link_targets: vec!["first".into(), "second".into()] }]
		);
	}

	#[test]
	fn parts_are_found_whatever_the_case_of_the_reference() {
		let mut package = Package::from_parts(&[("Documents/1/Pages/1.fpage", "text")]);
		assert_eq!(package.read_text("documents/1/PAGES/1.FPAGE").expect("read"), "text");
	}

	#[test]
	fn a_part_split_into_pieces_reads_as_one() {
		let mut package = Package::from_parts(&[
			("Doc.fdoc/[1].piece", "two "),
			("Doc.fdoc/[10].last.piece", "three"),
			("Doc.fdoc/[0].piece", "one "),
		]);
		assert_eq!(package.read_text("Doc.fdoc").expect("read"), "one two three");
	}

	#[test]
	fn a_part_saved_as_utf_16_reads_as_text() {
		let mut bytes = vec![0xFF, 0xFE];
		bytes.extend("<a/>".encode_utf16().flat_map(u16::to_le_bytes));
		let mut package = Package::from_bytes(&[("page.fpage", &bytes)]);
		assert_eq!(package.read_text("page.fpage").expect("read"), "<a/>");
	}

	#[test]
	fn a_package_without_a_fixed_representation_has_no_documents() {
		let mut package = Package::from_parts(&[("readme.txt", "not an xps file")]);
		assert!(package.documents().is_empty());
	}

	#[test]
	fn a_part_named_by_a_relationship_is_resolved_against_its_source() {
		let rels = r#"<Relationships>
			<Relationship Id="R0" Type="http://schemas.microsoft.com/xps/2005/06/documentstructure" Target="Structure/Doc.struct"/>
			<Relationship Id="R1" Type="http://schemas.microsoft.com/xps/2005/06/other" Target="Other.xml"/>
		</Relationships>"#;
		let mut package = Package::from_parts(&[("Documents/1/_rels/Doc.fdoc.rels", rels)]);
		assert_eq!(package.related("Documents/1/Doc.fdoc", DOCUMENT_STRUCTURE), ["Documents/1/Structure/Doc.struct"]);
	}
}
