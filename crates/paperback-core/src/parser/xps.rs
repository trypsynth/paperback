use std::{
	collections::HashMap,
	fs::File,
	io::{BufReader, Read, Seek},
};

use anyhow::{Context, Result};
use zip::ZipArchive;

use self::{
	package::{CORE_PROPERTIES, Package, PagePart, STORY_FRAGMENTS},
	page::read_page,
	structure::{Fragment, read_fragments},
	writer::PageWriter,
};
use crate::{
	document::{Document, DocumentBuffer, Marker, MarkerType, ParserContext},
	parser::{
		Parser,
		util::{ooxml::parse_core_properties, path::extract_title_from_path},
	},
	t,
};

mod package;
mod page;
mod structure;
mod writer;

pub struct XpsParser;

impl Parser for XpsParser {
	fn parse(&self, context: &ParserContext) -> Result<Document> {
		tracing::debug!(path = %context.file_path, "parsing xps file");
		let file = File::open(&context.file_path)
			.with_context(|| format!("Failed to open XPS file '{}'", context.file_path))?;
		let archive = ZipArchive::new(BufReader::new(file))
			.with_context(|| format!("Failed to read XPS as zip '{}'", context.file_path))?;
		read_document(Package::new(archive), context)
	}
}

fn page_fragments<R: Read + Seek>(package: &mut Package<R>, page: &str) -> Vec<Fragment> {
	let mut fragments = Vec::new();
	for part in package.related(page, STORY_FRAGMENTS) {
		match package.read_text(&part).and_then(|xml| read_fragments(&xml)) {
			Ok(mut read) => fragments.append(&mut read),
			Err(error) => tracing::warn!(%part, %error, "failed to read xps story fragments"),
		}
	}
	fragments
}

fn core_properties<R: Read + Seek>(package: &mut Package<R>) -> (Option<String>, Option<String>) {
	let Some(part) = package.related("", CORE_PROPERTIES).into_iter().next() else { return (None, None) };
	package.read_text(&part).map_or((None, None), |xml| parse_core_properties(&xml))
}

fn read_document<R: Read + Seek>(mut package: Package<R>, context: &ParserContext) -> Result<Document> {
	let pages: Vec<PagePart> = package.documents().iter().flat_map(|document| package.pages(document)).collect();
	if pages.is_empty() {
		tracing::warn!(path = %context.file_path, "xps file has no pages");
		// TRANSLATORS: Error shown when an XPS document has no pages
		anyhow::bail!(t("XPS file contains no pages"));
	}
	let mut buffer = DocumentBuffer::new();
	let mut id_positions = HashMap::new();
	for (index, part) in pages.iter().enumerate() {
		let page_start = buffer.end();
		id_positions.insert(format!("page_{index}"), page_start);
		buffer.add_marker(Marker::new(MarkerType::PageBreak, page_start).with_text(format!("Page {}", index + 1)));
		// A page that can't be read keeps its place, so the pages after it keep their numbers.
		let page = match package.read_text(&part.name).and_then(|xml| read_page(&xml)) {
			Ok(page) => page,
			Err(error) => {
				tracing::warn!(part = %part.name, %error, "failed to read xps page, skipping its text");
				continue;
			}
		};
		let fragments = page_fragments(&mut package, &part.name);
		if fragments.is_empty() {
			for run in page.runs {
				buffer.append(&run.text);
				buffer.append("\n");
			}
		} else {
			PageWriter::new(&mut buffer, &page, context.render_tables_inline)
				.write(&fragments, context.strip_running_text);
		}
	}
	let (title, author) = core_properties(&mut package);
	let title = title.unwrap_or_else(|| extract_title_from_path(&context.file_path));
	let mut document = Document::new().with_title(title).with_author(author.unwrap_or_default());
	document.set_buffer(buffer);
	document.id_positions = id_positions;
	tracing::debug!(path = %context.file_path, page_count = pages.len(), "parsed xps file successfully");
	Ok(document)
}

#[cfg(test)]
mod tests;
