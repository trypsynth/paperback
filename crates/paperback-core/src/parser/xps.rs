use std::{
	collections::HashMap,
	fs::File,
	io::{BufReader, Read, Seek},
};

use anyhow::{Context, Result};
use zip::ZipArchive;

use self::{
	fonts::Fonts,
	lines::{RunStyle, page_lines},
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
			Line, PendingLink, RunningText, build_toc_tree, page_paragraphs, place_links, resolve_headings,
			split_lines, survey_running_text,
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

fn read_related<R: Read + Seek, T>(
	package: &mut Package<R>,
	source: &str,
	kind: &str,
	read: fn(&str) -> Result<Vec<T>>,
) -> Vec<T> {
	let mut items = Vec::new();
	for part in package.related(source, kind) {
		match package.read_text(&part).and_then(|xml| read(&xml)) {
			Ok(mut found) => items.append(&mut found),
			Err(error) => tracing::warn!(%part, %error, kind, "failed to read xps part"),
		}
	}
	items
}

fn page_fragments<R: Read + Seek>(package: &mut Package<R>, page: &str) -> Vec<Fragment> {
	let mut fragments = read_related(package, page, STORY_FRAGMENTS, read_fragments);
	fragments.retain(|fragment| !fragment.blocks.is_empty());
	fragments
}

fn core_properties<R: Read + Seek>(package: &mut Package<R>) -> (Option<String>, Option<String>) {
	let Some(part) = package.related("", CORE_PROPERTIES).into_iter().next() else { return (None, None) };
	package.read_text(&part).map_or((None, None), |xml| parse_core_properties(&xml))
}

enum Content {
	Declared(Page, Vec<Fragment>),
	Drawn(Vec<Line>),
}

/// Every page is read before any is written, because running text is only known once every page's edges have been seen.
struct ReadPage {
	content: Content,
	links: Vec<PendingLink>,
}

impl ReadPage {
	/// A page that can't be read stays empty, so the pages after it keep their numbers.
	fn of<R: Read + Seek>(
		package: &mut Package<R>,
		part: &str,
		fonts: &mut Fonts,
		target_pages: &HashMap<&str, usize>,
	) -> Self {
		let page = match package.read_text(part).and_then(|xml| read_page(&xml)) {
			Ok(page) => page,
			Err(error) => {
				tracing::warn!(part, %error, "failed to read xps page, skipping its text");
				return Self { content: Content::Drawn(Vec::new()), links: Vec::new() };
			}
		};
		let fragments = page_fragments(package, part);
		let has_area = page.links.iter().any(|link| matches!(link.place, LinkPlace::Area(_)));
		// Only laying out lines and testing a run against a link rectangle need to know how wide it is.
		let styles: Vec<RunStyle> = if fragments.is_empty() || has_area {
			page.runs.iter().map(|run| fonts.style_of(package, part, run)).collect()
		} else {
			Vec::new()
		};
		let links = page_links(&page, &styles, target_pages);
		let content = if fragments.is_empty() {
			Content::Drawn(page_lines(&page, &styles))
		} else {
			Content::Declared(page, fragments)
		};
		Self { content, links }
	}

	fn drawn_lines(&self) -> Option<&[Line]> {
		match &self.content {
			Content::Drawn(lines) => Some(lines.as_slice()),
			Content::Declared(..) => None,
		}
	}
}

fn write_lines(
	buffer: &mut DocumentBuffer,
	lines: Vec<Line>,
	running: &RunningText,
	join: bool,
) -> Vec<(DocPos, String)> {
	let (_, paragraphs) = page_paragraphs(lines, running, join);
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
fn page_links(page: &Page, styles: &[RunStyle], target_pages: &HashMap<&str, usize>) -> Vec<PendingLink> {
	let mut links = Vec::new();
	for link in &page.links {
		let Some(reference) = link_reference(&link.uri, target_pages) else { continue };
		let runs: Vec<usize> = match &link.place {
			LinkPlace::Runs(runs) => runs.clone().collect(),
			LinkPlace::Area(area) => {
				let runs = page.runs.iter().zip(styles).enumerate();
				runs.filter(|(_, (run, style))| area.covers(run, style.width)).map(|(index, _)| index).collect()
			}
		};
		let text: String = runs.iter().map(|index| page.runs[*index].text.as_str()).collect();
		let text = trim_string(&collapse_whitespace(&text));
		if !text.is_empty()
			&& let Some(&first) = runs.first()
		{
			links.push((first, PendingLink { text, reference }));
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

fn detected_headings(written: Vec<WrittenPage>) -> Vec<(DocPos, String)> {
	let body = body_size(&written);
	let mut headings = Vec::new();
	for page in written {
		headings.extend(page.headings);
		let (positions, lines): (Vec<DocPos>, Vec<Line>) = page
			.paragraphs
			.into_iter()
			.map(|paragraph| {
				let line = Line {
					text: paragraph.text,
					size: paragraph.size,
					top: 0.0,
					bottom: 0.0,
					monospaced: false,
					bold: false,
				};
				(paragraph.position, line)
			})
			.unzip();
		let marked = split_lines(&lines, body).into_iter().filter(|(_, is_heading, _)| *is_heading);
		headings.extend(marked.map(|(text, _, index)| (positions[index], text)));
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
	let target_pages = link_target_pages(&pages);
	let mut fonts = Fonts::default();
	let read: Vec<ReadPage> =
		pages.iter().map(|part| ReadPage::of(&mut package, &part.name, &mut fonts, &target_pages)).collect();
	let running = survey_running_text(read.iter().filter_map(ReadPage::drawn_lines), context.strip_running_text);
	let mut buffer = DocumentBuffer::new();
	let mut id_positions = HashMap::new();
	let mut written: Vec<WrittenPage> = Vec::new();
	for (index, page) in read.into_iter().enumerate() {
		let start = buffer.end();
		id_positions.insert(format!("page_{index}"), start);
		buffer.add_marker(Marker::new(MarkerType::PageBreak, start).with_text(format!("Page {}", index + 1)));
		let (paragraphs, headings) = match page.content {
			Content::Drawn(lines) => {
				(Vec::new(), write_lines(&mut buffer, lines, &running, context.join_pdf_paragraphs))
			}
			Content::Declared(declared, fragments) => {
				let writer = PageWriter::new(&mut buffer, &declared, context.render_tables_inline);
				(writer.write(&fragments, context.strip_running_text), Vec::new())
			}
		};
		if buffer.end() == start {
			// TRANSLATORS: Shown in place of an XPS page that has no text, such as a scanned one
			buffer.append(&t("This page has no text."));
			buffer.append("\n");
		}
		let page_text = buffer.text(start..buffer.end()).to_string();
		place_links(&page.links, start, &page_text, &mut buffer);
		written.push(WrittenPage { start, paragraphs, headings });
	}
	let entries: Vec<OutlineEntry> = documents
		.iter()
		.flat_map(|document| read_related(&mut package, document, DOCUMENT_STRUCTURE, read_outline))
		.collect();
	let mut toc_items = build_toc_tree(outline_items(entries, &target_pages, &written));
	resolve_headings(&mut buffer, &mut toc_items, Vec::new(), detected_headings(written), false);
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
