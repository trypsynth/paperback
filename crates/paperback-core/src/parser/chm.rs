use std::{
	collections::{HashMap, HashSet},
	mem,
};

use anyhow::{Context, Result};
use libchm::{ChmFile, Entry, EntryCategory, EntrySel};

use crate::{
	document::{DocPos, Document, DocumentBuffer, Marker, MarkerType, ParserContext, is_heading_marker},
	parser::{
		Parser, add_converter_markers_excluding_links, add_heading_markers_where, util::path::extract_title_from_path,
	},
	util::text::url_decode,
};

mod convert;
mod encoding;
mod href;
mod toc;

use convert::{SectionContent, convert_sections};
use href::{normalize_path, resolve_chm_href};
use toc::{build_ordered_file_list, calculate_toc_offsets, parse_hhc_file, parse_system_file};

pub struct ChmParser;

impl Parser for ChmParser {
	fn parse(&self, context: &ParserContext) -> Result<Document> {
		tracing::debug!(path = %context.file_path, "parsing chm file");
		let mut chm = ChmFile::open(&context.file_path)
			.with_context(|| format!("Failed to open CHM file: {}", context.file_path))?;
		let mut html_files = Vec::new();
		// Enumeration already decoded each entry, so keeping it saves a directory B-tree descent per file below.
		let mut entries_by_path: HashMap<String, Entry> = HashMap::new();
		let mut hhc_file = String::new();
		for entry in chm.entries(EntrySel::ALL)? {
			let lower_path = entry.path.to_lowercase();
			if lower_path.contains(".hhc") && (hhc_file.is_empty() || lower_path.contains("index.hhc")) {
				hhc_file.clone_from(&entry.path);
			}
			if (lower_path.contains(".htm") || lower_path.contains(".html")) && entry.category != EntryCategory::Special
			{
				let path = entry.path.clone();
				entries_by_path.insert(path.clone(), entry);
				html_files.push(path);
			}
		}
		html_files.sort();
		tracing::debug!(html_file_count = html_files.len(), hhc_found = !hhc_file.is_empty(), "chm structure detected");
		let system = parse_system_file(&mut chm);
		let title = system.title.unwrap_or_else(|| {
			tracing::debug!(path = %context.file_path, "no title found in #SYSTEM record, falling back to filename");
			extract_title_from_path(&context.file_path)
		});
		if hhc_file.is_empty() {
			tracing::debug!(path = %context.file_path, "chm has no hhc file, table of contents will be empty");
		}
		let mut toc_items =
			if hhc_file.is_empty() { Vec::new() } else { parse_hhc_file(&mut chm, &hhc_file, system.encoding)? };
		let ordered_files = build_ordered_file_list(&html_files, &toc_items);
		let converted = convert_sections(
			&context.file_path,
			&ordered_files,
			&entries_by_path,
			context.render_tables_inline,
			system.encoding,
		);
		// The original index is kept so the "Section N" label reflects position even when earlier files were skipped.
		let mut sections: Vec<(usize, &String, SectionContent)> = Vec::with_capacity(converted.len());
		for (idx, slot) in converted.into_iter().enumerate() {
			match slot {
				Ok(section) => sections.push((idx, &ordered_files[idx], section)),
				Err(err) => {
					tracing::warn!(error = %err, "skipping chm html entry that could not be read or converted");
				}
			}
		}
		// An empty section after the first adds nothing, because the text so far already ends with a newline.
		let texts: Vec<String> = sections
			.iter_mut()
			.enumerate()
			.map(|(pos, (_, _, section))| {
				let text = mem::take(&mut section.text);
				if text.ends_with('\n') || (text.is_empty() && pos > 0) { text } else { text + "\n" }
			})
			.collect();
		let (mut buffer, spans) = DocumentBuffer::from_parts(texts);
		let mut id_positions = HashMap::new();
		let mut file_positions = HashMap::new();
		for ((idx, file_path, section), span) in sections.iter().zip(&spans) {
			let section_start = span.start;
			let normalized_path = normalize_path(file_path);
			file_positions.insert(normalized_path.clone(), section_start);
			// Store file-level position so fragment-less internal links can be resolved.
			id_positions.insert(normalized_path.clone(), section_start);
			for (id, relative_pos) in &section.id_positions {
				id_positions.insert(format!("{normalized_path}#{id}"), section_start + relative_pos.from_start());
			}
			buffer.add_marker(
				Marker::new(MarkerType::SectionBreak, section_start)
					.with_text(format!("Section {}", idx + 1))
					.with_reference((*file_path).clone()),
			);
			add_converter_markers_excluding_links(&mut buffer, section, section_start, &|source| {
				url_decode(resolve_chm_href(file_path, source).split('#').next().unwrap_or_default())
			});
			for link in &section.links {
				let resolved_href = resolve_chm_href(file_path, &link.reference);
				buffer.add_marker(
					Marker::new(MarkerType::Link, section_start + link.offset.from_start())
						.with_text(link.text.clone())
						.with_reference(resolved_href),
				);
			}
		}
		calculate_toc_offsets(&mut toc_items, &file_positions, &id_positions);
		// Asked per topic, not per book: a reference that marks up two of its six hundred pages needs the other 598, without those two announced twice.
		let headed = sections_with_headings(&buffer, &file_positions);
		add_heading_markers_where(&mut buffer, &toc_items, 1, &|offset| {
			!headed.contains(&section_start(&file_positions, offset))
		});
		let mut document = Document::new().with_title(title);
		document.set_buffer(buffer);
		document.id_positions = id_positions;
		document.toc_items = toc_items;
		tracing::debug!(path = %context.file_path, "parsed chm file successfully");
		Ok(document)
	}
}

fn section_start(file_positions: &HashMap<String, DocPos>, offset: DocPos) -> DocPos {
	file_positions.values().copied().filter(|start| *start <= offset).max().unwrap_or_default()
}

fn sections_with_headings(buffer: &DocumentBuffer, file_positions: &HashMap<String, DocPos>) -> HashSet<DocPos> {
	buffer
		.markers
		.iter()
		.filter(|marker| is_heading_marker(marker.mtype))
		.map(|marker| section_start(file_positions, marker.position))
		.collect()
}
