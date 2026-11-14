//! NLS BARD's braille books: a zip, or a folder, of BRF volumes and a braille package (`.bopf`), an Open eBook 1.0 package that names the book, its braille code and its volumes in order.
//!
//! The code is in `brl:brailleCode` ("EBAE") in older packages and `brl:code` ("ueb") in newer ones, with the grade in `brl:contractionLevel`, and `dc:Format` says it again ("BRF/UEB contracted"). Reading a book with the table its code names matters most for the many older books in EBAE, which UEB, the reader's table by default, garbles.

use std::{
	fs::{self, File},
	io::BufReader,
	path::Path,
};

use anyhow::{Context, Result};
use roxmltree::{Document as XmlDocument, ParsingOptions};
use zip::ZipArchive;

use super::{Page, back_translate, likeliest_pages, tables, volume_toc};
use crate::{
	document::{Document, ParserContext},
	parser::{NotThisFormat, Parser, braille::BrailleCode, util::path::extract_title_from_path},
	util::zip::{list_zip_entries, read_zip_entry_by_name, read_zip_entry_bytes},
};

pub struct BopfParser;

impl Parser for BopfParser {
	fn parse(&self, context: &ParserContext) -> Result<Document> {
		let path = Path::new(&context.file_path);
		let (package, volumes) = if path.extension().is_some_and(|extension| extension.eq_ignore_ascii_case("zip")) {
			read_zip(path)?
		} else {
			read_folder(path)?
		};
		let table = package.table().unwrap_or(context.braille_table);
		tracing::debug!(path = %context.file_path, table, volumes = volumes.len(), "parsing BARD braille book");
		let (translator, braille_ascii) = tables::back_translator(table)?;
		let context = ParserContext { braille_table: table, ..context.clone() };
		let volumes: Vec<Vec<Page>> =
			volumes.iter().map(|bytes| likeliest_pages(bytes, &braille_ascii, &translator, &context)).collect();
		let pages: Vec<Page> = volumes.iter().flatten().cloned().collect();
		let buffer = back_translate(&pages, &translator, &context);
		let toc = if volumes.len() > 1 { volume_toc(&volumes, &buffer) } else { Vec::new() };
		let title = Some(package.title.clone())
			.filter(|title| !title.is_empty())
			.unwrap_or_else(|| extract_title_from_path(&context.file_path));
		let mut document = Document::new().with_title(title).with_author(package.author.clone());
		document.set_buffer(buffer);
		document.toc_items = toc;
		Ok(document)
	}
}

/// The table the braille package beside a lone volume names, when one in its folder lists it. A folder can hold several books, so only the package that lists this file counts.
pub(super) fn table_beside(volume: &Path) -> Option<&'static str> {
	let name = volume.file_name()?.to_str()?;
	fs::read_dir(volume.parent()?)
		.ok()?
		.flatten()
		.map(|entry| entry.path())
		.filter(|path| path.extension().is_some_and(|extension| extension.eq_ignore_ascii_case("bopf")))
		.filter_map(|path| Package::parse(&fs::read_to_string(path).ok()?).ok())
		.find(|package| package.lists(name))?
		.table()
}

fn read_zip(path: &Path) -> Result<(Package, Vec<Vec<u8>>)> {
	let file = File::open(path).with_context(|| format!("Failed to open '{}'", path.display()))?;
	let mut archive = ZipArchive::new(BufReader::new(file))
		.with_context(|| format!("Failed to read '{}' as a zip", path.display()))?;
	let Some(name) = list_zip_entries(&archive)
		.into_iter()
		.map(|entry| entry.name)
		.find(|name| name.to_ascii_lowercase().ends_with(".bopf"))
	else {
		return Err(NotThisFormat("A zip without a braille package").into());
	};
	let package = Package::parse(&read_zip_entry_by_name(&mut archive, &name)?)?;
	let folder = name.rsplit_once('/').map_or("", |(folder, _)| folder);
	let volumes = package
		.volumes
		.iter()
		.map(|href| {
			let entry = if folder.is_empty() { href.clone() } else { format!("{folder}/{href}") };
			read_zip_entry_bytes(&mut archive, &entry)
				.with_context(|| format!("The braille package lists '{href}', which the zip doesn't have"))
		})
		.collect::<Result<_>>()?;
	Ok((package, volumes))
}

fn read_folder(path: &Path) -> Result<(Package, Vec<Vec<u8>>)> {
	let package =
		Package::parse(&fs::read_to_string(path).with_context(|| format!("Failed to open '{}'", path.display()))?)?;
	let folder = path.parent().unwrap_or_else(|| Path::new("."));
	let volumes = package
		.volumes
		.iter()
		.map(|href| {
			fs::read(folder.join(href))
				.with_context(|| format!("The braille package lists '{href}', which isn't beside it"))
		})
		.collect::<Result<_>>()?;
	Ok((package, volumes))
}

/// What a braille package says about its book.
#[derive(Debug, Default, PartialEq, Eq)]
struct Package {
	title: String,
	author: String,
	language: String,
	/// The braille code and grade as a system `table_for_code` reads, such as "EBAE grade2".
	system: String,
	/// The volumes' paths in the order they're read.
	volumes: Vec<String>,
}

impl Package {
	fn parse(xml: &str) -> Result<Self> {
		let document =
			XmlDocument::parse_with_options(xml, ParsingOptions { allow_dtd: true, ..ParsingOptions::default() })
				.context("The braille package isn't XML")?;
		let elements = || document.descendants().filter(roxmltree::Node::is_element);
		let named = |name: &str| {
			elements()
				.find(|node| node.tag_name().name().eq_ignore_ascii_case(name))
				.and_then(|node| node.text())
				.map(|text| text.trim().to_string())
		};
		let meta = |name: &str| {
			elements()
				.find(|node| node.tag_name().name() == "meta" && node.attribute("name") == Some(name))
				.and_then(|node| node.attribute("content"))
				.map(|content| content.trim().to_string())
				.filter(|content| !content.is_empty())
		};
		let format = named("Format").unwrap_or_default();
		// "BRF/UEB contracted": the code after the slash, and whether it's contracted.
		let format_code =
			format.split_once('/').and_then(|(_, rest)| rest.split_whitespace().next()).map(str::to_string);
		let code = meta("brl:brailleCode").or_else(|| meta("brl:code")).or(format_code).unwrap_or_default();
		let format_words = format.to_ascii_lowercase();
		let grade = meta("brl:contractionLevel").or_else(|| {
			if format_words.contains("uncontracted") {
				Some("1".to_string())
			} else {
				format_words.contains("contracted").then(|| "2".to_string())
			}
		});
		let system = match grade {
			Some(grade) if !code.is_empty() => format!("{code} grade{grade}"),
			_ => code,
		};
		let manifest: Vec<(String, String)> = elements()
			.filter(|node| node.tag_name().name() == "item")
			.filter_map(|node| Some((node.attribute("id")?.to_string(), node.attribute("href")?.to_string())))
			.collect();
		let mut volumes: Vec<String> = elements()
			.filter(|node| node.tag_name().name() == "itemref")
			.filter_map(|node| {
				manifest
					.iter()
					.find(|(id, _)| Some(id.as_str()) == node.attribute("idref"))
					.map(|(_, href)| href.clone())
			})
			.filter(|href| !href.to_ascii_lowercase().ends_with(".bopf"))
			.collect();
		if volumes.is_empty() {
			volumes = manifest
				.into_iter()
				.map(|(_, href)| href)
				.filter(|href| href.to_ascii_lowercase().ends_with(".brf"))
				.collect();
		}
		// Catalog titles end with the punctuation before a subtitle the record leaves out, such as "Cooking without looking :".
		let title = named("Title").unwrap_or_default().trim_end_matches([' ', ':', ';', '/', ',']).to_string();
		Ok(Self {
			title,
			author: named("Creator").unwrap_or_default(),
			language: named("Language").unwrap_or_default(),
			system,
			volumes,
		})
	}

	fn table(&self) -> Option<&'static str> {
		tables::table_for_code(BrailleCode { language: &self.language, system: &self.system, cells: Some(6) })
	}

	fn lists(&self, file_name: &str) -> bool {
		self.volumes.iter().any(|href| href.rsplit('/').next().is_some_and(|name| name.eq_ignore_ascii_case(file_name)))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const EBAE: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<package xmlns="http://openebook.org/namespaces/oeb-package/1.0/" xmlns:dc="http://purl.org/dc/elements/1.0/">
<metadata><dc-metadata><dc:Title>Cooking without looking :</dc:Title><dc:Creator role="aut">Tipps, Esther Knudson</dc:Creator><dc:Language>en</dc:Language><dc:Format>BRF/EBAE contracted</dc:Format></dc-metadata>
<x-metadata><meta name="brl:brailleCode" content="EBAE"/><meta name="brl:contractionLevel" content="2"/></x-metadata></metadata>
<manifest><item id="vol1" href="13478v01.brf" media-type="text/brf"/><item id="vol2" href="13478v02.brf" media-type="text/brf"/><item id="bopf" href="13478.bopf" media-type="text/xml"/></manifest>
<spine><itemref idref="vol1"/><itemref idref="vol2"/></spine></package>"#;

	const UEB: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE package PUBLIC "+//ISBN 0-9673008-1-9//DTD OEB 1.0.1 Package//EN" "oebpkg101.dtd">
<package unique-identifier="uid" xmlns="http://openebook.org/namespaces/oeb-package/1.0/">
<metadata><dc-metadata xmlns:dc="http://purl.org/dc/elements/1.0/"><dc:Creator>Kwan, Kevin</dc:Creator><dc:Title>Crazy rich Asians</dc:Title><dc:Language>en</dc:Language><dc:Format>BRF/UEB contracted</dc:Format></dc-metadata>
<x-metadata><meta name="brl:code" content="ueb"></meta><meta name="brl:contractionLevel" content="2"></meta></x-metadata></metadata>
<manifest><item media-type="text/xml" id="bopf" href="26528.bopf"></item><item media-type="text/plain" id="brf01" href="26528v01.brf"></item><item media-type="text/plain" id="brf02" href="26528v02.brf"></item></manifest>
<spine><itemref idref="brf01"></itemref><itemref idref="brf02"></itemref></spine></package>"#;

	#[test]
	fn reads_an_older_package_naming_ebae() {
		let package = Package::parse(EBAE).expect("a package");
		assert_eq!(package.title, "Cooking without looking");
		assert_eq!(package.author, "Tipps, Esther Knudson");
		assert_eq!(package.volumes, ["13478v01.brf", "13478v02.brf"]);
		assert_eq!(package.table(), Some("en-us-g2.ctb"));
	}

	#[test]
	fn reads_a_newer_package_naming_ueb_with_a_doctype() {
		let package = Package::parse(UEB).expect("a package");
		assert_eq!(package.title, "Crazy rich Asians");
		assert_eq!(package.volumes, ["26528v01.brf", "26528v02.brf"]);
		assert_eq!(package.table(), Some("en-ueb-g2.ctb"));
	}

	#[test]
	fn takes_the_code_from_the_format_when_no_meta_names_it() {
		let package = Package::parse(
			&UEB.replace(
				r#"<meta name="brl:code" content="ueb"></meta><meta name="brl:contractionLevel" content="2"></meta>"#,
				"",
			)
			.replace("UEB contracted", "UEB uncontracted"),
		)
		.expect("a package");
		assert_eq!(package.table(), Some("en-ueb-g1.ctb"));
	}

	#[test]
	fn a_lone_volume_takes_the_table_of_the_package_that_lists_it() {
		let dir = crate::util::test_support::TempDir::new("bopf-beside");
		dir.write_str("13478.bopf", EBAE);
		dir.write_str("26528.bopf", UEB);
		let volume = dir.write_str("26528v02.brf", ",HELLO\r\n");
		assert_eq!(table_beside(Path::new(&volume)), Some("en-ueb-g2.ctb"));
		let other = dir.write_str("13478v01.brf", ",HELLO\r\n");
		assert_eq!(table_beside(Path::new(&other)), Some("en-us-g2.ctb"));
		let unlisted = dir.write_str("other.brf", ",HELLO\r\n");
		assert_eq!(table_beside(Path::new(&unlisted)), None);
	}
}
