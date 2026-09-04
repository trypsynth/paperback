use std::{collections::HashMap, mem};

use anyhow::{Context, Result};
use libchm::{ChmFile, Entry, EntryCategory, EntrySel};

use crate::{
	document::{Document, DocumentBuffer, Marker, MarkerType, ParserContext},
	parser::{Parser, add_converter_markers_excluding_links, util::path::extract_title_from_path},
};

mod convert;
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
		// Keep the directory entry for each HTML file. Enumeration has already decoded it, so
		// holding on to it saves a full directory B-tree descent per file when we read them below.
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
		let title = parse_system_file(&mut chm).unwrap_or_else(|| {
			tracing::debug!(path = %context.file_path, "no title found in #SYSTEM record, falling back to filename");
			extract_title_from_path(&context.file_path)
		});
		if hhc_file.is_empty() {
			tracing::debug!(path = %context.file_path, "chm has no hhc file, table of contents will be empty");
		}
		let mut toc_items = if hhc_file.is_empty() { Vec::new() } else { parse_hhc_file(&mut chm, &hhc_file)? };
		let ordered_files = build_ordered_file_list(&html_files, &toc_items);
		let converted =
			convert_sections(&context.file_path, &ordered_files, &entries_by_path, context.render_tables_inline);
		// Keep each file's original index so the "Section N" label still reflects its position in
		// the document even when earlier files were skipped.
		let mut sections: Vec<(usize, &String, SectionContent)> = Vec::with_capacity(converted.len());
		for (idx, slot) in converted.into_iter().enumerate() {
			match slot {
				Ok(section) => sections.push((idx, &ordered_files[idx], section)),
				Err(err) => {
					tracing::warn!(error = %err, "skipping chm html entry that could not be read or converted");
				}
			}
		}
		// Sections are separated exactly as the one-at-a-time build separated them: a section that
		// does not already end with a newline gets one appended. An empty section after the first
		// adds nothing, because the text so far already ends with a newline.
		let texts: Vec<String> = sections
			.iter_mut()
			.enumerate()
			.map(|(pos, (_, _, section))| {
				let text = mem::take(&mut section.text);
				if text.ends_with('\n') || (text.is_empty() && pos > 0) { text } else { text + "\n" }
			})
			.collect();
		// `from_parts` builds the content and its per-character indices for every section in one
		// parallel pass, handing back where each section landed so markers can be placed after.
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
				id_positions.insert(format!("{normalized_path}#{id}"), section_start + relative_pos);
			}
			buffer.add_marker(
				Marker::new(MarkerType::SectionBreak, section_start)
					.with_text(format!("Section {}", idx + 1))
					.with_reference((*file_path).clone()),
			);
			add_converter_markers_excluding_links(&mut buffer, section, section_start);
			for link in &section.links {
				let resolved_href = resolve_chm_href(file_path, &link.reference);
				buffer.add_marker(
					Marker::new(MarkerType::Link, section_start + link.offset)
						.with_text(link.text.clone())
						.with_reference(resolved_href),
				);
			}
		}
		calculate_toc_offsets(&mut toc_items, &file_positions, &id_positions);
		let mut document = Document::new().with_title(title);
		document.set_buffer(buffer);
		document.id_positions = id_positions;
		document.toc_items = toc_items;
		tracing::debug!(path = %context.file_path, "parsed chm file successfully");
		Ok(document)
	}
}
