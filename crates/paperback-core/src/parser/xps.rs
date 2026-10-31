use std::{
	collections::HashMap,
	fs::File,
	io::{BufReader, Read, Seek},
};

use anyhow::{Context, Result};
use zip::ZipArchive;

use self::{
	package::{CORE_PROPERTIES, DOCUMENT_STRUCTURE, Package, PagePart, STORY_FRAGMENTS},
	page::read_page,
	structure::{Fragment, OutlineEntry, read_fragments, read_outline},
	writer::{PageWriter, WrittenParagraph},
};
use crate::{
	document::{DocPos, Document, DocumentBuffer, Marker, MarkerType, ParserContext, TocItem},
	parser::{
		Parser,
		pdf::{Line, build_toc_tree, resolve_headings, split_lines},
		util::{ooxml::parse_core_properties, path::extract_title_from_path},
	},
	t,
	util::text::{collapse_whitespace, trim_string},
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

fn outline_entries<R: Read + Seek>(package: &mut Package<R>, documents: &[String]) -> Vec<OutlineEntry> {
	let mut entries = Vec::new();
	for part in documents.iter().flat_map(|document| package.related(document, DOCUMENT_STRUCTURE)).collect::<Vec<_>>()
	{
		match package.read_text(&part).and_then(|xml| read_outline(&xml)) {
			Ok(mut read) => entries.append(&mut read),
			Err(error) => tracing::warn!(%part, %error, "failed to read xps document structure"),
		}
	}
	entries
}

struct WrittenPage {
	start: DocPos,
	paragraphs: Vec<WrittenParagraph>,
}

fn outline_items(entries: Vec<OutlineEntry>, pages: &[PagePart], written: &[WrittenPage]) -> Vec<(u32, TocItem)> {
	let targets =
		pages.iter().enumerate().flat_map(|(index, page)| page.link_targets.iter().map(move |name| (name, index)));
	let target_pages: HashMap<&String, usize> = targets.collect();
	let mut next_paragraph = vec![0; written.len()];
	let mut items = Vec::new();
	for entry in entries {
		let Some(&page) = target_pages.get(&entry.target) else {
			tracing::warn!(target = %entry.target, "xps outline entry leads nowhere");
			continue;
		};
		let title = trim_string(&collapse_whitespace(&entry.title));
		let paragraphs = &written[page].paragraphs[next_paragraph[page]..];
		let found = paragraphs.iter().position(|paragraph| paragraph.text == title);
		let offset = found.map_or(written[page].start, |found| paragraphs[found].position);
		next_paragraph[page] += found.map_or(0, |found| found + 1);
		items.push((entry.level, TocItem::new(title, String::new(), offset)));
	}
	items
}

/// Counted in characters because a paragraph is one line here, so on a short page the headings would otherwise outnumber the text they head.
fn body_size(written: &[WrittenPage]) -> f64 {
	let mut sizes: Vec<(f64, usize)> = Vec::new();
	for paragraph in written.iter().flat_map(|page| &page.paragraphs) {
		let characters = paragraph.text.chars().count();
		match sizes.iter_mut().find(|(size, _)| (*size - paragraph.size).abs() < f64::EPSILON) {
			Some((_, total)) => *total += characters,
			None => sizes.push((paragraph.size, characters)),
		}
	}
	sizes.into_iter().max_by_key(|(_, characters)| *characters).map_or(0.0, |(size, _)| size)
}

fn detected_headings(written: &[WrittenPage]) -> Vec<(DocPos, String)> {
	let body = body_size(written);
	let mut headings = Vec::new();
	for page in written {
		let lines: Vec<Line> = page
			.paragraphs
			.iter()
			.map(|paragraph| Line {
				text: paragraph.text.clone(),
				size: paragraph.size,
				top: 0.0,
				bottom: 0.0,
				monospaced: false,
				bold: false,
			})
			.collect();
		let marked = split_lines(&lines, body).into_iter().filter(|(_, is_heading, _)| *is_heading);
		headings.extend(marked.map(|(text, _, index)| (page.paragraphs[index].position, text)));
	}
	headings
}

fn read_document<R: Read + Seek>(mut package: Package<R>, context: &ParserContext) -> Result<Document> {
	let documents = package.documents();
	let pages: Vec<PagePart> = documents.iter().flat_map(|document| package.pages(document)).collect();
	if pages.is_empty() {
		tracing::warn!(path = %context.file_path, "xps file has no pages");
		// TRANSLATORS: Error shown when an XPS document has no pages
		anyhow::bail!(t("XPS file contains no pages"));
	}
	let mut buffer = DocumentBuffer::new();
	let mut id_positions = HashMap::new();
	let mut written: Vec<WrittenPage> = Vec::new();
	for (index, part) in pages.iter().enumerate() {
		let page_start = buffer.end();
		id_positions.insert(format!("page_{index}"), page_start);
		buffer.add_marker(Marker::new(MarkerType::PageBreak, page_start).with_text(format!("Page {}", index + 1)));
		written.push(WrittenPage { start: page_start, paragraphs: Vec::new() });
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
			let writer = PageWriter::new(&mut buffer, &page, context.render_tables_inline);
			written[index].paragraphs = writer.write(&fragments, context.strip_running_text);
		}
	}
	let entries = outline_entries(&mut package, &documents);
	let mut toc_items = build_toc_tree(outline_items(entries, &pages, &written));
	resolve_headings(&mut buffer, &mut toc_items, Vec::new(), detected_headings(&written), false);
	let (title, author) = core_properties(&mut package);
	let title = title.unwrap_or_else(|| extract_title_from_path(&context.file_path));
	let mut document = Document::new().with_title(title).with_author(author.unwrap_or_default());
	document.set_buffer(buffer);
	document.id_positions = id_positions;
	document.toc_items = toc_items;
	tracing::debug!(path = %context.file_path, page_count = pages.len(), "parsed xps file successfully");
	Ok(document)
}

#[cfg(test)]
mod tests;
