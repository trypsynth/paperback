//! Qt Compressed Help, the `.qch` files `qhelpgenerator` builds from a `.qhp` project and Qt Assistant and Qt Creator read: an SQLite database holding the project's files zlib-compressed and its table of contents as a `QDataStream`.
//!
//! The reading follows `QHelpDBReader` and `QHelpEngineCore` in qttools, which define the format; it has no other specification.

use std::{
	collections::{HashMap, HashSet},
	mem,
	path::Path,
};

use anyhow::Result;
use rayon::prelude::*;

use crate::{
	document::{DocPos, Document, DocumentBuffer, Marker, MarkerType, ParserContext, TocItem, is_heading_marker},
	parser::{
		Parser, add_converter_markers_excluding_links, add_heading_markers_where, align_toc_to_title_lines,
		chm::convert::{SectionContent, convert_section},
		util::path::extract_title_from_path,
	},
};

mod compress;
mod db;
mod href;
mod stream;

use db::{FileEntry, QchDatabase};
use href::{BookLocation, Target, document_path};
use stream::{ContentsNode, build_tree, read_contents};

/// Where an entry whose page the book doesn't have points: past the end, as no position in the book is.
const UNRESOLVED: DocPos = DocPos::from_raw(usize::MAX);

pub struct QchParser;

/// The extensions `qhelpgenerator` reads a title from, and so the files it treats as pages; any case, since a project made on Windows may have `.HTM`.
fn is_html_name(name: &str) -> bool {
	Path::new(name)
		.extension()
		.is_some_and(|extension| extension.eq_ignore_ascii_case("html") || extension.eq_ignore_ascii_case("htm"))
}

impl Parser for QchParser {
	fn parse(&self, context: &ParserContext) -> Result<Document> {
		tracing::debug!(path = %context.file_path, "parsing qch file");
		let db = QchDatabase::open(&context.file_path)?;
		let namespace = db.namespace()?;
		let (folder_id, folder) = db.virtual_folder()?;
		let book = BookLocation { namespace, folder };
		let files = db.files(folder_id)?;
		let contents: Vec<ContentsNode> =
			db.contents()?.iter().flat_map(|data| build_tree(read_contents(data))).collect();
		let author = db.metadata("author")?.filter(|author| !author.trim().is_empty());
		tracing::debug!(namespace = %book.namespace, folder = %book.folder, files = files.len(), "qch structure detected");
		// Qt Assistant names a book by its first contents entry, as the file stores no title of its own.
		let title = contents
			.first()
			.map(|node| node.title.trim().to_string())
			.filter(|title| !title.is_empty())
			.unwrap_or_else(|| extract_title_from_path(&context.file_path));
		let mut toc_items = contents.into_iter().map(|node| toc_item(&book, node)).collect::<Vec<_>>();
		let pages = order_pages(files.into_iter().filter(|file| is_html_name(&file.name)).collect(), &toc_items);
		let converted = convert_pages(&context.file_path, &pages, context.render_tables_inline);
		let mut sections: Vec<(usize, &FileEntry, SectionContent)> = Vec::with_capacity(converted.len());
		for (index, slot) in converted.into_iter().enumerate() {
			match slot {
				Ok(section) => sections.push((index, &pages[index], section)),
				Err(err) => tracing::warn!(error = %err, "skipping qch page that could not be read"),
			}
		}
		let texts: Vec<String> = sections
			.iter_mut()
			.enumerate()
			.map(|(position, (_, _, section))| {
				let text = mem::take(&mut section.text);
				if text.ends_with('\n') || (text.is_empty() && position > 0) { text } else { text + "\n" }
			})
			.collect();
		let (mut buffer, spans) = DocumentBuffer::from_parts(texts);
		let mut id_positions = HashMap::new();
		let mut page_positions = HashMap::new();
		for ((index, page, section), span) in sections.iter().zip(&spans) {
			let start = span.start;
			let path = document_path(&page.name);
			page_positions.insert(path.clone(), start);
			id_positions.insert(path.clone(), start);
			for (id, relative) in &section.id_positions {
				id_positions.insert(format!("{path}#{id}"), start + relative.from_start());
			}
			buffer.add_marker(
				Marker::new(MarkerType::SectionBreak, start)
					.with_text(format!("Section {}", index + 1))
					.with_reference(path),
			);
			// Pictures are named as `FileNameTable` names them, which is what reading one back looks up.
			add_converter_markers_excluding_links(&mut buffer, section, start, &|source| match book
				.resolve(&page.name, source)
			{
				Target::Internal { name, .. } => name,
				Target::External(url) => url,
			});
			for link in &section.links {
				buffer.add_marker(
					Marker::new(MarkerType::Link, start + link.offset.from_start())
						.with_text(link.text.clone())
						.with_reference(book.resolve(&page.name, &link.reference).into_reference()),
				);
			}
		}
		resolve_toc_offsets(&mut toc_items, &page_positions, &id_positions);
		align_toc_to_title_lines(&buffer, &mut toc_items);
		// Qt's generated pages have headings, but a hand-made project's may not, and only those get the contents' names.
		let mut starts: Vec<DocPos> = page_positions.values().copied().collect();
		starts.sort_unstable();
		let headed = pages_with_headings(&buffer, &starts);
		add_heading_markers_where(&mut buffer, &toc_items, 1, &|offset| !headed.contains(&page_start(&starts, offset)));
		let mut document = Document::new().with_title(title);
		if let Some(author) = author {
			document = document.with_author(author);
		}
		document.set_buffer(buffer);
		document.id_positions = id_positions;
		document.toc_items = toc_items;
		tracing::debug!(path = %context.file_path, "parsed qch file successfully");
		Ok(document)
	}
}

/// A contents link is relative to the folder, as Qt Assistant reads it.
fn toc_item(book: &BookLocation, node: ContentsNode) -> TocItem {
	let mut item = TocItem::new(node.title, book.resolve("", &node.link).into_reference(), UNRESOLVED);
	item.children = node.children.into_iter().map(|child| toc_item(book, child)).collect();
	item
}

/// Pages in contents order, then the rest in the order the project listed them, so following the contents reads the book front to back and every page a link can reach is still there.
fn order_pages(pages: Vec<FileEntry>, toc_items: &[TocItem]) -> Vec<FileEntry> {
	let mut by_path: HashMap<String, FileEntry> =
		pages.iter().map(|page| (document_path(&page.name), page.clone())).collect();
	let mut ordered = Vec::with_capacity(pages.len());
	let mut references = Vec::new();
	collect_references(toc_items, &mut references);
	for reference in references {
		let path = reference.split_once('#').map_or(reference, |(path, _)| path);
		if let Some(page) = by_path.remove(path) {
			ordered.push(page);
		}
	}
	ordered.extend(pages.into_iter().filter(|page| by_path.contains_key(&document_path(&page.name))));
	ordered
}

fn collect_references<'a>(items: &'a [TocItem], references: &mut Vec<&'a str>) {
	for item in items {
		references.push(&item.reference);
		collect_references(&item.children, references);
	}
}

/// Inflating and converting pages is nearly all of the time a big book takes, so each rayon worker opens its own connection and the reads run in parallel too.
fn convert_pages(
	source_path: &str,
	pages: &[FileEntry],
	render_tables_inline: bool,
) -> Vec<Result<SectionContent, String>> {
	pages
		.par_iter()
		.map_init(
			|| QchDatabase::open(source_path).map_err(|err| err.to_string()),
			|db, page| {
				let db = db.as_ref().map_err(|err| format!("{} ({err})", page.name))?;
				let bytes = db.file_data(page.file_id).map_err(|err| format!("{} ({err:#})", page.name))?;
				Ok(convert_section(&bytes, render_tables_inline, None))
			},
		)
		.collect()
}

fn resolve_toc_offsets(
	items: &mut [TocItem],
	page_positions: &HashMap<String, DocPos>,
	id_positions: &HashMap<String, DocPos>,
) {
	for item in items {
		let (path, fragment) = item
			.reference
			.split_once('#')
			.map_or((item.reference.as_str(), None), |(path, fragment)| (path, Some(fragment)));
		item.offset = fragment
			.and_then(|fragment| id_positions.get(&format!("{path}#{fragment}")))
			.or_else(|| page_positions.get(path))
			.copied()
			.unwrap_or(UNRESOLVED);
		resolve_toc_offsets(&mut item.children, page_positions, id_positions);
	}
}

fn page_start(starts: &[DocPos], offset: DocPos) -> DocPos {
	starts[..starts.partition_point(|start| *start <= offset)].last().copied().unwrap_or_default()
}

fn pages_with_headings(buffer: &DocumentBuffer, starts: &[DocPos]) -> HashSet<DocPos> {
	buffer
		.markers
		.iter()
		.filter(|marker| is_heading_marker(marker.mtype))
		.map(|marker| page_start(starts, marker.position))
		.collect()
}

/// For a picture the parser named; `name` is its name in the book's folder.
pub fn file_bytes(path: &str, name: &str) -> Result<Vec<u8>> {
	QchDatabase::open(path)?.file_data_by_name(name)
}

#[cfg(test)]
mod tests;
