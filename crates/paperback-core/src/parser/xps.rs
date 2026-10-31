use std::{
	collections::HashMap,
	fs::File,
	io::{BufReader, Read, Seek},
};

use anyhow::{Context, Result};
use zip::ZipArchive;

use self::{
	fonts::Fonts,
	lines::page_lines,
	package::{CORE_PROPERTIES, DOCUMENT_STRUCTURE, Package, PagePart, STORY_FRAGMENTS},
	page::{LinkPlace, Page, read_page},
	structure::{Fragment, OutlineEntry, read_fragments, read_outline},
	writer::{PageWriter, WrittenParagraph},
};
use crate::{
	document::{DocPos, Document, DocumentBuffer, Marker, MarkerType, ParserContext, TocItem},
	parser::{
		Parser,
		pdf::{
			Line, PageEdges, PendingLink, RunningText, build_toc_tree, detect_running_text, join_paragraphs,
			median_line_font_size, place_links, resolve_headings, split_lines, strip_running_text,
		},
		util::{ooxml::parse_core_properties, path::extract_title_from_path},
	},
	t,
	util::text::{collapse_whitespace, trim_string},
};

mod fonts;
mod lines;
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
	fragments.retain(|fragment| !fragment.blocks.is_empty());
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

/// Every page is read before any is written, because running text is only known once every page's edges have been seen.
#[derive(Default)]
struct ReadPage {
	page: Page,
	fragments: Vec<Fragment>,
	lines: Vec<Line>,
}

impl ReadPage {
	/// A page that can't be read stays empty, so the pages after it keep their numbers.
	fn of<R: Read + Seek>(package: &mut Package<R>, part: &PagePart, fonts: &mut Fonts) -> Self {
		let mut page = match package.read_text(&part.name).and_then(|xml| read_page(&xml)) {
			Ok(page) => page,
			Err(error) => {
				tracing::warn!(part = %part.name, %error, "failed to read xps page, skipping its text");
				return Self::default();
			}
		};
		if page.links.iter().any(|link| matches!(link.place, LinkPlace::Area(_))) {
			for run in &mut page.runs {
				run.width = fonts.style_of(package, &part.name, run).width;
			}
		}
		let fragments = page_fragments(package, &part.name);
		let lines = if fragments.is_empty() {
			page_lines(&page, |run| fonts.style_of(package, &part.name, run))
		} else {
			Vec::new()
		};
		Self { page, fragments, lines }
	}
}

fn running_text(pages: &[ReadPage], strip: bool) -> RunningText {
	let drawn = || pages.iter().filter(|page| page.fragments.is_empty());
	let edges: Vec<PageEdges> = drawn().map(|page| PageEdges::of(&page.lines)).collect();
	let lines: Vec<Line> = drawn().flat_map(|page| page.lines.clone()).collect();
	let running = detect_running_text(&edges, median_line_font_size(&lines));
	if strip { running } else { running.recognizing_nothing() }
}

fn write_lines(
	buffer: &mut DocumentBuffer,
	lines: &[Line],
	running: &RunningText,
	join: bool,
) -> Vec<(DocPos, String)> {
	let mut lines = lines.to_vec();
	// Measured before the running text comes out, for the reason `untagged_page_lines` gives.
	let body_size = median_line_font_size(&lines);
	strip_running_text(&mut lines, running);
	let paragraphs = if join { join_paragraphs(&lines, body_size) } else { split_lines(&lines, body_size) };
	let mut headings = Vec::new();
	for (text, is_heading, _) in paragraphs {
		if is_heading {
			headings.push((buffer.end(), text.clone()));
		}
		buffer.append(&text);
		buffer.append("\n");
	}
	headings
}

struct WrittenPage {
	start: DocPos,
	paragraphs: Vec<WrittenParagraph>,
	headings: Vec<(DocPos, String)>,
}

fn link_target_pages(pages: &[PagePart]) -> HashMap<&str, usize> {
	let targets = pages.iter().enumerate();
	targets.flat_map(|(index, page)| page.link_targets.iter().map(move |name| (name.as_str(), index))).collect()
}

fn outline_items(
	entries: Vec<OutlineEntry>,
	target_pages: &HashMap<&str, usize>,
	written: &[WrittenPage],
) -> Vec<(u32, TocItem)> {
	let mut next_paragraph = vec![0; written.len()];
	let mut items = Vec::new();
	for entry in entries {
		let Some(&page) = target_pages.get(entry.target.as_str()) else {
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

/// A web address can carry a `#` too, but only a reference within the file has no scheme before it.
fn link_reference(uri: &str, target_pages: &HashMap<&str, usize>) -> Option<String> {
	match uri.rsplit_once('#') {
		Some((document, name)) if !document.contains(':') => Some(format!("#page_{}", target_pages.get(name)?)),
		_ => Some(uri.to_string()),
	}
}

/// In page order, which `place_links` searches in.
fn page_links(page: &Page, target_pages: &HashMap<&str, usize>) -> Vec<PendingLink> {
	let mut links = Vec::new();
	for link in &page.links {
		let Some(reference) = link_reference(&link.uri, target_pages) else { continue };
		let runs: Vec<usize> = match &link.place {
			LinkPlace::Runs(runs) => runs.clone().collect(),
			LinkPlace::Area(area) => (0..page.runs.len()).filter(|index| area.covers(&page.runs[*index])).collect(),
		};
		let text: String = runs.iter().map(|index| page.runs[*index].text.as_str()).collect();
		let text = trim_string(&collapse_whitespace(&text));
		if let (Some(first), false) = (runs.first(), text.is_empty()) {
			links.push((*first, PendingLink { text, reference }));
		}
	}
	links.sort_by_key(|(first, _)| *first);
	links.into_iter().map(|(_, link)| link).collect()
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
		headings.extend(page.headings.iter().cloned());
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
	let mut fonts = Fonts::default();
	let read: Vec<ReadPage> = pages.iter().map(|part| ReadPage::of(&mut package, part, &mut fonts)).collect();
	let running = running_text(&read, context.strip_running_text);
	let target_pages = link_target_pages(&pages);
	let mut written: Vec<WrittenPage> = Vec::new();
	for (index, page) in read.iter().enumerate() {
		let start = buffer.end();
		id_positions.insert(format!("page_{index}"), start);
		buffer.add_marker(Marker::new(MarkerType::PageBreak, start).with_text(format!("Page {}", index + 1)));
		let (paragraphs, headings) = if page.fragments.is_empty() {
			(Vec::new(), write_lines(&mut buffer, &page.lines, &running, context.join_pdf_paragraphs))
		} else {
			let writer = PageWriter::new(&mut buffer, &page.page, context.render_tables_inline);
			(writer.write(&page.fragments, context.strip_running_text), Vec::new())
		};
		if buffer.end() == start {
			// TRANSLATORS: Shown in place of an XPS page that has no text, such as a scanned one
			buffer.append(&t("This page has no text."));
			buffer.append("\n");
		}
		let page_text = buffer.text(start..buffer.end()).to_string();
		place_links(&page_links(&page.page, &target_pages), start, &page_text, &mut buffer);
		written.push(WrittenPage { start, paragraphs, headings });
	}
	let entries = outline_entries(&mut package, &documents);
	let mut toc_items = build_toc_tree(outline_items(entries, &target_pages, &written));
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
