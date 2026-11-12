//! eBraille, the DAISY Consortium's EPUB 3 for braille, whose text is Unicode braille and whose package names its braille code.
//!
//! A book that carries its print as a second rendition is read in print, as written. Otherwise its braille is back-translated with the table its code names, such as `en-ueb-g2.ctb` for `UEB grade2` in `en-Brai-US`, or with the reader's own table when none matches.

use std::{
	fs::{self, File},
	io::BufReader,
	path::Path,
};

use anyhow::{Context, Result};
use roxmltree::{Document as XmlDocument, ParsingOptions};
use zip::ZipArchive;

use crate::{
	document::{Document, ParserContext},
	parser::{
		NotThisFormat, Parser,
		braille::{self, BrailleCode, back_translate_document, table_for_code},
		epub::{parse_epub, rootfiles},
	},
	util::{package, zip::read_zip_entry_by_name},
};

pub struct EbrailleParser;

impl Parser for EbrailleParser {
	fn parse(&self, context: &ParserContext) -> Result<Document> {
		let path = Path::new(&context.file_path);
		if package::is_package_document(path) {
			// An unpacked eBraille book, its folder of files opened by its package document, is read from a zip of it.
			let opf = String::from_utf8_lossy(&fs::read(path)?).into_owned();
			if !package::is_braille_package(&opf) {
				return Err(NotThisFormat("A package without braille").into());
			}
			let copy = package::zip_copy(path, "ebrl", &package::cache_root())?;
			let mut document =
				self.parse(&ParserContext { file_path: copy.zip.to_string_lossy().into_owned(), ..context.clone() })?;
			package::point_images_at(&mut document, &copy.root);
			return Ok(document);
		}
		tracing::debug!(path = %context.file_path, "parsing ebraille");
		let file = File::open(&context.file_path)
			.with_context(|| format!("Failed to open eBraille file '{}'", context.file_path))?;
		let mut archive = ZipArchive::new(BufReader::new(file))
			.with_context(|| format!("Failed to read eBraille as zip '{}'", context.file_path))?;
		let renditions = rootfiles(&mut archive)?;
		// Beside a braille rendition, any other is the print: SBS's books mark it `textual`, but DAISY Pipeline's leave it unmarked.
		let print =
			renditions.iter().find(|rendition| rendition.access_mode.as_deref() == Some("textual")).or_else(|| {
				(renditions.len() > 1)
					.then(|| renditions.iter().find(|rendition| rendition.access_mode.as_deref() != Some("tactile")))
					.flatten()
			});
		if let Some(print) = print {
			tracing::debug!(path = %context.file_path, rendition = %print.path, "ebraille has a print rendition, reading it");
			return parse_epub(context, Some(&print.path));
		}
		let braille_rendition = renditions
			.iter()
			.find(|rendition| rendition.access_mode.as_deref() == Some("tactile"))
			.or_else(|| renditions.first())
			.context("eBraille container names no package document")?;
		let package = read_zip_entry_by_name(&mut archive, &braille_rendition.path)?;
		let (language, system, cells) = braille_code(&package);
		let table = table_for_code(BrailleCode { language: &language, system: &system, cells })
			.unwrap_or(context.braille_table);
		tracing::debug!(path = %context.file_path, language, system, table, "back-translating ebraille");
		let mut document = parse_epub(context, Some(&braille_rendition.path))?;
		let (translator, _) = braille::back_translator(table)?;
		back_translate_document(&mut document, &translator);
		Ok(document)
	}
}

/// The braille language tag (`en-Brai-US`), `a11y:brailleSystem` and `a11y:brailleCellType` of a package document, each empty or `None` when it is missing.
fn braille_code(package: &str) -> (String, String, Option<u8>) {
	let Ok(document) =
		XmlDocument::parse_with_options(package, ParsingOptions { allow_dtd: true, ..ParsingOptions::default() })
	else {
		return (String::new(), String::new(), None);
	};
	let text = |node: roxmltree::Node| node.text().unwrap_or_default().trim().to_owned();
	let elements = || document.descendants().filter(roxmltree::Node::is_element);
	let language = elements()
		.filter(|node| node.tag_name().name() == "language")
		.map(text)
		.find(|language| language.split('-').any(|subtag| subtag.eq_ignore_ascii_case("Brai")))
		.unwrap_or_default();
	let meta = |property: &str| {
		elements()
			.find(|node| node.tag_name().name() == "meta" && node.attribute("property") == Some(property))
			.map(text)
	};
	let system = meta("a11y:brailleSystem").unwrap_or_default();
	let cells = meta("a11y:brailleCellType").and_then(|cells| cells.parse().ok());
	(language, system, cells)
}

#[cfg(test)]
mod tests {
	use super::braille_code;

	#[test]
	fn reads_the_braille_code_a_package_names() {
		let package = r#"<package xmlns="http://www.idpf.org/2007/opf"><metadata xmlns:dc="http://purl.org/dc/elements/1.1/"><dc:language>en-US</dc:language><dc:language>en-Brai-US</dc:language><meta property="a11y:brailleCellType">6</meta><meta property="a11y:brailleSystem">UEB grade2</meta></metadata></package>"#;
		assert_eq!(braille_code(package), ("en-Brai-US".to_owned(), "UEB grade2".to_owned(), Some(6)));
		assert_eq!(braille_code("<package/>"), (String::new(), String::new(), None));
	}
}

#[cfg(test)]
mod book_tests {
	use std::io::Write;

	use zip::{ZipWriter, write::SimpleFileOptions};

	use super::*;
	use crate::{document::MarkerType, util::test_support::TempDir};

	const PACKAGE: &str = r#"<?xml version="1.0" encoding="utf-8"?><package xmlns="http://www.idpf.org/2007/opf" version="3.0" unique-identifier="id"><metadata xmlns:dc="http://purl.org/dc/elements/1.1/"><dc:title>Max at the Zoo</dc:title><dc:identifier id="id">zoo</dc:identifier><dc:language>en-US</dc:language><dc:language>en-Brai-US</dc:language><meta property="a11y:brailleSystem">UEB grade2</meta><meta property="a11y:brailleCellType">6</meta></metadata><manifest><item id="nav" href="index.html" media-type="application/xhtml+xml" properties="nav"/><item id="one" href="ebraille/one.html" media-type="application/xhtml+xml"/></manifest><spine><itemref idref="one"/></spine></package>"#;
	const NAV: &str = r#"<?xml version="1.0" encoding="utf-8"?><html xmlns="http://www.w3.org/1999/xhtml" xmlns:epub="http://www.idpf.org/2007/ops"><head><title>toc</title></head><body><nav epub:type="toc"><ol><li><a href="ebraille/one.html#zoo">⠠⠍⠁⠭ ⠁⠞ ⠮ ⠠⠵⠕⠕</a></li></ol></nav><nav epub:type="page-list"><ol><li><a href="ebraille/one.html#page1">⠼⠁</a></li></ol></nav></body></html>"#;
	const CHAPTER: &str = r#"<?xml version="1.0" encoding="utf-8"?><html xmlns="http://www.w3.org/1999/xhtml" xmlns:epub="http://www.idpf.org/2007/ops"><head><title>one</title></head><body><span role="doc-pagebreak" epub:type="pagebreak" id="page1" aria-label="⠼⠁"></span><h1 id="zoo">⠠⠍⠁⠭ ⠁⠞ ⠮ ⠠⠵⠕⠕</h1><p>⠠⠍⠁⠭ ⠴ ⠁ ⠇⠇ ⠍⠕⠝⠅⠑⠽⠲</p><table><tr><td>⠠⠍⠁⠭</td><td>⠈⠯</td></tr></table></body></html>"#;

	/// `print` is the print rendition's package and chapter, and whether `container.xml` marks it `textual`.
	fn write_book(dir: &TempDir, name: &str, print: Option<(&str, &str, bool)>) -> String {
		let path = dir.join_str(name);
		let mut zip = ZipWriter::new(File::create(&path).expect("create the book"));
		let options = SimpleFileOptions::default();
		let rendition = match print {
			Some((_, _, true)) => {
				r#"<rootfile full-path="original/package.opf" media-type="application/oebps-package+xml" rend:accessMode="textual"/>"#
			}
			Some((_, _, false)) => {
				r#"<rootfile full-path="original/package.opf" media-type="application/oebps-package+xml"/>"#
			}
			None => "",
		};
		let container = format!(
			r#"<?xml version="1.0"?><container xmlns="urn:oasis:names:tc:opendocument:xmlns:container" xmlns:rend="http://www.idpf.org/2013/rendition" version="1.0"><rootfiles><rootfile full-path="package.opf" media-type="application/oebps-package+xml" rend:accessMode="tactile"/>{rendition}</rootfiles></container>"#
		);
		let mut files = vec![
			("mimetype", "application/epub+zip".to_owned()),
			("META-INF/container.xml", container),
			("package.opf", PACKAGE.to_owned()),
			("index.html", NAV.to_owned()),
			("ebraille/one.html", CHAPTER.to_owned()),
		];
		if let Some((package, chapter, _)) = print {
			files.push(("original/package.opf", package.to_owned()));
			files.push(("original/one.html", chapter.to_owned()));
		}
		for (name, contents) in files {
			zip.start_file(name, options).expect("start a file");
			zip.write_all(contents.as_bytes()).expect("write a file");
		}
		zip.finish().expect("finish the book");
		path
	}

	#[test]
	fn back_translates_the_braille_and_everything_pointing_into_it() {
		let dir = TempDir::new("ebraille-braille");
		let document =
			EbrailleParser.parse(&ParserContext::new(write_book(&dir, "zoo.ebrl", None))).expect("read the book");
		assert!(document.buffer.content.contains("Max at the Zoo"), "{}", document.buffer.content);
		assert!(document.buffer.content.contains("Max was a little monkey."), "{}", document.buffer.content);
		assert_eq!(document.toc_items[0].name, "Max at the Zoo");
		let heading =
			document.buffer.markers.iter().find(|marker| marker.mtype == MarkerType::Heading1).expect("a heading");
		assert_eq!(heading.text, "Max at the Zoo");
		let start = document.buffer.content.chars().skip(heading.position.get()).take(14).collect::<String>();
		assert_eq!(start, "Max at the Zoo");
		let page = document.buffer.markers.iter().find(|marker| marker.mtype == MarkerType::PageBreak).expect("a page");
		assert_eq!(page.text, "1");
	}

	#[test]
	fn back_translates_a_tables_html() {
		let dir = TempDir::new("ebraille-table");
		let document =
			EbrailleParser.parse(&ParserContext::new(write_book(&dir, "zoo.ebrl", None))).expect("read the book");
		let table = document.buffer.markers.iter().find(|marker| marker.mtype == MarkerType::Table).expect("a table");
		assert!(table.reference.contains(">Max<"), "{}", table.reference);
		assert!(table.reference.contains(">&amp;<"), "{}", table.reference);
	}

	#[test]
	fn reads_the_print_rendition_when_there_is_one() {
		let dir = TempDir::new("ebraille-print");
		let package = PACKAGE
			.replace("<dc:language>en-Brai-US</dc:language>", "")
			.replace("ebraille/one.html", "one.html")
			.replace(r#"<item id="nav" href="index.html" media-type="application/xhtml+xml" properties="nav"/>"#, "");
		let chapter = r#"<?xml version="1.0" encoding="utf-8"?><html xmlns="http://www.w3.org/1999/xhtml"><head><title>one</title></head><body><h1>Max at the Zoo</h1><p>Max was a little monkey, in print.</p></body></html>"#;
		// SBS marks the print rendition `textual`; DAISY Pipeline leaves it unmarked.
		for marked in [true, false] {
			let book = write_book(&dir, &format!("zoo-{marked}.ebrl"), Some((&package, chapter, marked)));
			let document = EbrailleParser.parse(&ParserContext::new(book)).expect("read the book");
			assert!(
				document.buffer.content.contains("Max was a little monkey, in print."),
				"{}",
				document.buffer.content
			);
		}
	}
}
