use std::{fs, path::Path};

use anyhow::{Context, Result};

use crate::{
	document::{Document, DocumentBuffer, ParserContext, ParserFlags},
	encoding::convert_to_utf8,
	parser::{
		Parser, add_converter_markers, daisy::DaisyParser, path::extract_title_from_path, toc::build_toc_from_headings,
	},
	xml_to_text::XmlToText,
};

pub struct XmlParser;

impl Parser for XmlParser {
	fn name(&self) -> &'static str {
		"XML Files"
	}

	fn extensions(&self) -> &[&str] {
		&["xml"]
	}

	fn supported_flags(&self) -> ParserFlags {
		ParserFlags::SUPPORTS_TOC | ParserFlags::SUPPORTS_LISTS
	}

	fn parse(&self, context: &ParserContext) -> Result<Document> {
		if let Some(base_dir) = Path::new(&context.file_path).parent() {
			if let Ok(entries) = fs::read_dir(base_dir) {
				for entry in entries.flatten() {
					let path = entry.path();
					if path.is_file() && path.extension().is_some_and(|e| e.eq_ignore_ascii_case("opf")) {
						let mut daisy_context = context.clone();
						daisy_context.file_path = path.to_string_lossy().to_string();
						let daisy_parser = DaisyParser;
						if let Ok(doc) = daisy_parser.parse(&daisy_context) {
							return Ok(doc);
						}
					}
				}
			}
		}

		let bytes =
			fs::read(&context.file_path).with_context(|| format!("Failed to open XML file '{}'", context.file_path))?;
		if bytes.is_empty() {
			anyhow::bail!("XML file is empty: {}", context.file_path);
		}
		let xml_content = convert_to_utf8(&bytes);
		let mut converter = XmlToText::new();
		if !converter.convert(&xml_content) {
			anyhow::bail!("Failed to convert XML to text: {}", context.file_path);
		}
		let text = converter.get_text();
		let mut buffer = DocumentBuffer::with_content(text);
		let id_positions = converter.get_id_positions().clone();
		add_converter_markers(&mut buffer, &converter, 0);
		let toc_items = build_toc_from_headings(converter.get_headings());
		let title = extract_title_from_path(&context.file_path);
		let mut doc = Document::new().with_title(title);
		doc.set_buffer(buffer);
		doc.toc_items = toc_items;
		doc.id_positions = id_positions;
		Ok(doc)
	}
}
