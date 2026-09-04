use std::collections::HashMap;

use libchm::{ChmFile, Entry};
use rayon::prelude::*;

use crate::{
	parser::{
		ConverterOutput,
		convert::html_to_text::{HtmlSourceMode, HtmlToText},
	},
	types::{FormatInfo, HeadingInfo, ImageInfo, LinkInfo, ListInfo, ListItemInfo, SeparatorInfo, TableInfo},
	util::encoding::convert_to_utf8,
};

/// One converted HTML file: its text plus everything the converter recorded about it.
///
/// This is what crosses back from a rayon worker to the sequential assembly step, so it owns
/// its data rather than borrowing the converter that produced it.
pub(super) struct SectionContent {
	pub text: String,
	headings: Vec<HeadingInfo>,
	pub links: Vec<LinkInfo>,
	images: Vec<ImageInfo>,
	figures: Vec<ImageInfo>,
	tables: Vec<TableInfo>,
	separators: Vec<SeparatorInfo>,
	lists: Vec<ListInfo>,
	list_items: Vec<ListItemInfo>,
	bolds: Vec<FormatInfo>,
	italics: Vec<FormatInfo>,
	underlines: Vec<FormatInfo>,
	pub id_positions: HashMap<String, usize>,
}

impl ConverterOutput for SectionContent {
	fn get_headings(&self) -> &[HeadingInfo] {
		&self.headings
	}
	fn get_links(&self) -> &[LinkInfo] {
		&self.links
	}
	fn get_images(&self) -> &[ImageInfo] {
		&self.images
	}
	fn get_figures(&self) -> &[ImageInfo] {
		&self.figures
	}
	fn get_tables(&self) -> &[TableInfo] {
		&self.tables
	}
	fn get_separators(&self) -> &[SeparatorInfo] {
		&self.separators
	}
	fn get_lists(&self) -> &[ListInfo] {
		&self.lists
	}
	fn get_list_items(&self) -> &[ListItemInfo] {
		&self.list_items
	}
	fn get_bolds(&self) -> &[FormatInfo] {
		&self.bolds
	}
	fn get_italics(&self) -> &[FormatInfo] {
		&self.italics
	}
	fn get_underlines(&self) -> &[FormatInfo] {
		&self.underlines
	}
}

/// Reads and converts every HTML file to text, returning one result per file in order.
///
/// Conversion dominates the cost of parsing a CHM (around 90% of the time on a large one) and
/// each file is independent, so this runs across cores. Each rayon worker opens its own
/// [`ChmFile`] via `map_init` (once per task rather than once per file), so the reads and the
/// LZX decompression parallelise too instead of being serialised through one shared handle.
/// Failures are returned rather than dropped, so the caller can log each one and carry on.
pub(super) fn convert_sections(
	source_path: &str,
	ordered_files: &[String],
	entries_by_path: &HashMap<String, Entry>,
	render_tables_inline: bool,
) -> Vec<Result<SectionContent, String>> {
	ordered_files
		.par_iter()
		.map_init(
			|| ChmFile::open(source_path).map_err(|err| err.to_string()),
			|chm_result, file_path| {
				let chm = chm_result.as_mut().map_err(|err| format!("{file_path} ({err})"))?;
				// Enumeration already handed us the entry; look it up again only if this path
				// somehow was not one of the files we enumerated.
				let content_bytes = match entries_by_path.get(file_path) {
					Some(entry) => chm.read(entry),
					None => chm.find(file_path).and_then(|entry| chm.read(&entry)),
				}
				.map_err(|err| format!("{file_path} ({err})"))?;
				if content_bytes.is_empty() {
					return Err(format!("{file_path} (entry is empty)"));
				}
				convert_section(&content_bytes, render_tables_inline)
					.ok_or_else(|| format!("{file_path} (html conversion failed)"))
			},
		)
		.collect()
}

/// Convert one file's raw bytes to text, transcoding to UTF-8 first.
fn convert_section(content_bytes: &[u8], render_tables_inline: bool) -> Option<SectionContent> {
	let utf8_content = convert_to_utf8(content_bytes);
	let mut converter = HtmlToText::with_render_tables_inline(render_tables_inline);
	// currently always true, HtmlToText::convert has no failure path today
	if !converter.convert(&utf8_content, HtmlSourceMode::NativeHtml) {
		return None;
	}
	Some(SectionContent {
		text: converter.get_text(),
		headings: converter.get_headings().to_vec(),
		links: converter.get_links().to_vec(),
		images: ConverterOutput::get_images(&converter).to_vec(),
		figures: ConverterOutput::get_figures(&converter).to_vec(),
		tables: converter.get_tables().to_vec(),
		separators: converter.get_separators().to_vec(),
		lists: converter.get_lists().to_vec(),
		list_items: converter.get_list_items().to_vec(),
		bolds: converter.get_bolds().to_vec(),
		italics: converter.get_italics().to_vec(),
		underlines: converter.get_underlines().to_vec(),
		id_positions: converter.get_id_positions().clone(),
	})
}
