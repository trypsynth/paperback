use std::{collections::HashMap, fs::File, io::BufReader, mem};

use anyhow::Result;
use rayon::prelude::*;
use scraper::Html;
use zip::ZipArchive;

use super::{
	href::{resolve_href, split_href},
	package::ManifestItem,
};
use crate::{
	document::{DocPos, DocumentBuffer, Marker, MarkerType},
	parser::{
		ConverterOutput, add_converter_markers_excluding_links,
		convert::{
			html_to_text::{HtmlSourceMode, HtmlToText},
			xml_to_text::XmlToText,
		},
		is_data_uri,
	},
	t,
	types::{
		FormatInfo, FormulaInfo, HeadingInfo, ImageInfo, LinkInfo, ListInfo, ListItemInfo, SeparatorInfo, TableInfo,
	},
	util::zip::read_zip_entry_by_name,
};

struct SectionContent {
	text: String,
	headings: Vec<HeadingInfo>,
	links: Vec<LinkInfo>,
	images: Vec<ImageInfo>,
	figures: Vec<ImageInfo>,
	tables: Vec<TableInfo>,
	formulas: Vec<FormulaInfo>,
	separators: Vec<SeparatorInfo>,
	lists: Vec<ListInfo>,
	list_items: Vec<ListItemInfo>,
	bolds: Vec<FormatInfo>,
	italics: Vec<FormatInfo>,
	underlines: Vec<FormatInfo>,
	id_positions: HashMap<String, DocPos>,
	/// The converters only record described images, so this is the only sign that a page whose whole body is one undescribed picture isn't simply empty.
	has_image_markup: bool,
	/// Picture-book plates are mostly undescribed and so unrecorded by the converters; this is how such a page's picture can still be saved.
	image_source: String,
	/// An EPUB whose XHTML isn't well-formed usually has every section like that, so the book gets one summary line rather than one per section.
	xml_error: Option<String>,
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
	fn get_formulas(&self) -> &[FormulaInfo] {
		&self.formulas
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

pub(super) struct SectionMeta {
	pub(super) path: String,
	pub(super) start: DocPos,
	pub(super) end: DocPos,
}

pub(super) struct SpineConversionResult {
	pub(super) buffer: DocumentBuffer,
	pub(super) id_positions: HashMap<String, DocPos>,
	pub(super) sections: Vec<SectionMeta>,
	pub(super) conversion_errors: Vec<String>,
}

/// Each rayon worker opens its own `ZipArchive` so zip reads run across cores instead of serializing through one shared handle.
pub(super) fn convert_spine_items(
	file_path: &str,
	manifest: &HashMap<String, ManifestItem>,
	spine: &[String],
	render_tables_inline: bool,
) -> SpineConversionResult {
	let converted: Vec<Result<(&ManifestItem, SectionContent), String>> = spine
		.par_iter()
		.map_init(
			|| {
				File::open(file_path)
					.map_err(|err| err.to_string())
					.and_then(|file| ZipArchive::new(BufReader::new(file)).map_err(|err| err.to_string()))
			},
			|archive_result, idref| {
				let item = manifest.get(idref).ok_or_else(|| format!("missing manifest item for {idref}"))?;
				let archive = archive_result.as_mut().map_err(|err| format!("{} ({err})", item.path))?;
				let data =
					read_zip_entry_by_name(archive, &item.path).map_err(|err| format!("{} ({err})", item.path))?;
				Ok((item, convert_section(&data, render_tables_inline)))
			},
		)
		.collect();
	// The original index is kept because the "Section N" label reflects spine position even across skipped items.
	let mut ok_entries: Vec<(usize, &ManifestItem, SectionContent)> = Vec::with_capacity(converted.len());
	let mut conversion_errors = Vec::new();
	for (idx, slot) in converted.into_iter().enumerate() {
		match slot {
			Ok((item, section)) => ok_entries.push((idx, item, section)),
			Err(err) => {
				tracing::warn!(error = %err, "skipping epub spine item that could not be read or converted");
				conversion_errors.push(err);
			}
		}
	}
	let read_as_html: Vec<(&str, &str)> = ok_entries
		.iter()
		.filter_map(|(_, item, section)| section.xml_error.as_deref().map(|error| (item.path.as_str(), error)))
		.collect();
	if let Some((first, error)) = read_as_html.first() {
		tracing::warn!(
			sections = read_as_html.len(),
			of = ok_entries.len(),
			first,
			error,
			"epub sections were not well-formed xml, so they were read as html"
		);
	}
	// A picture spine item or one-image wrapper page converts to no text, which would leave a blank stretch with no sign a page is there.
	let image_sections: Vec<bool> =
		ok_entries.iter().map(|(_, item, section)| section_is_image(item, section)).collect();
	let image_placeholder = format!("[{}]", t("Image"));
	// A replaced section's converter markers and id positions point into the dropped text, so they are not applied below.
	let mut replaced = vec![false; ok_entries.len()];
	let texts: Vec<String> = ok_entries
		.iter_mut()
		.zip(&image_sections)
		.enumerate()
		.map(|(index, ((_, item, section), is_image))| {
			let text = mem::take(&mut section.text);
			// A raster spine item decodes to garbage, never real text, so it is always replaced; an SVG or wrapper page keeps any real text it has.
			if is_binary_image(&item.media_type) || (*is_image && text.trim().is_empty()) {
				replaced[index] = true;
				image_placeholder.clone()
			} else {
				text
			}
		})
		.collect();
	let (mut buffer, spans) = DocumentBuffer::from_parts(texts);
	let mut id_positions = HashMap::new();
	let mut sections = Vec::new();
	for (((entry, span), is_image), was_replaced) in ok_entries.iter().zip(&spans).zip(&image_sections).zip(&replaced) {
		let (idx, item, section) = entry;
		let section_start = span.start;
		let section_label = format!("Section {}", idx + 1);
		buffer.add_marker(
			Marker::new(MarkerType::SectionBreak, section_start)
				.with_text(section_label)
				.with_reference(item.path.clone()),
		);
		// The picture's path rides along as the reference, as for a comic archive's pages, so it can be saved.
		if *is_image {
			let source = if item.media_type.starts_with("image/") {
				item.path.clone()
			} else if section.image_source.is_empty() || is_data_uri(&section.image_source) {
				section.image_source.clone()
			} else {
				split_href(&resolve_href(&item.path, &section.image_source)).0
			};
			buffer.add_marker(Marker::new(MarkerType::Image, section_start).with_reference(source));
		}
		if !was_replaced {
			for (id, relative) in &section.id_positions {
				let absolute = section_start + relative.from_start();
				// Keep the first occurrence for bare ids to avoid later sections overwriting earlier ones.
				id_positions.entry(id.clone()).or_insert(absolute);
				id_positions.insert(format!("{}#{id}", item.path), absolute);
			}
			add_converter_markers_excluding_links(&mut buffer, section, section_start, &|source| {
				split_href(&resolve_href(&item.path, source)).0
			});
			for link in &section.links {
				let resolved = resolve_href(&item.path, &link.reference);
				buffer.add_marker(
					Marker::new(MarkerType::Link, section_start + link.offset.from_start())
						.with_text(link.text.clone())
						.with_reference(resolved),
				);
			}
		}
		sections.push(SectionMeta { path: item.path.clone(), start: section_start, end: span.end });
	}
	SpineConversionResult { buffer, id_positions, sections, conversion_errors }
}

/// Fixed-layout books in the EPUB sample set make pictures spine items; picture books wrap one plate per text-less XHTML page.
fn section_is_image(item: &ManifestItem, section: &SectionContent) -> bool {
	if item.media_type.starts_with("image/") {
		return true;
	}
	section.has_image_markup && section.text.trim().is_empty()
}

/// A substring test is enough: a false positive costs one image line on an already empty page.
fn content_has_image_markup(content: &str) -> bool {
	let lowered = content.to_ascii_lowercase();
	lowered.contains("<img") || lowered.contains("<image") || lowered.contains("<figure")
}

/// SVG is left out: it is real XML and may hold text worth keeping.
fn is_binary_image(media_type: &str) -> bool {
	media_type.starts_with("image/") && media_type != "image/svg+xml"
}

/// Only asked of a page with no text, which is rare enough that parsing it a second time costs nothing.
fn first_image_source(content: &str, text: &str) -> String {
	if !text.trim().is_empty() || !content_has_image_markup(content) {
		return String::new();
	}
	HtmlToText::image_source(Html::parse_document(content).tree.root())
}

fn convert_section(content: &str, render_tables_inline: bool) -> SectionContent {
	let mut xml_converter = XmlToText::with_render_tables_inline(render_tables_inline);
	let xml_error = match xml_converter.try_convert(content) {
		Ok(()) => None,
		Err(error) => Some(error.to_string()),
	};
	if xml_error.is_none() {
		let text = xml_converter.take_text();
		return SectionContent {
			image_source: first_image_source(content, &text),
			text,
			headings: xml_converter.get_headings().to_vec(),
			links: xml_converter.get_links().to_vec(),
			images: xml_converter.get_images().to_vec(),
			figures: xml_converter.get_figures().to_vec(),
			tables: xml_converter.get_tables().to_vec(),
			formulas: xml_converter.get_formulas().to_vec(),
			separators: xml_converter.get_separators().to_vec(),
			lists: xml_converter.get_lists().to_vec(),
			list_items: xml_converter.get_list_items().to_vec(),
			bolds: xml_converter.get_bolds().to_vec(),
			italics: xml_converter.get_italics().to_vec(),
			underlines: xml_converter.get_underlines().to_vec(),
			id_positions: xml_converter.get_id_positions().clone(),
			has_image_markup: content_has_image_markup(content),
			xml_error: None,
		};
	}
	let mut html_converter = HtmlToText::with_render_tables_inline(render_tables_inline);
	html_converter.convert(content, HtmlSourceMode::NativeHtml);
	let text = html_converter.take_text();
	SectionContent {
		image_source: first_image_source(content, &text),
		text,
		headings: html_converter.get_headings().to_vec(),
		links: html_converter.get_links().to_vec(),
		images: html_converter.get_images().to_vec(),
		figures: html_converter.get_figures().to_vec(),
		tables: html_converter.get_tables().to_vec(),
		formulas: html_converter.get_formulas().to_vec(),
		separators: html_converter.get_separators().to_vec(),
		lists: html_converter.get_lists().to_vec(),
		list_items: html_converter.get_list_items().to_vec(),
		bolds: html_converter.get_bolds().to_vec(),
		italics: html_converter.get_italics().to_vec(),
		underlines: html_converter.get_underlines().to_vec(),
		id_positions: html_converter.get_id_positions().clone(),
		has_image_markup: content_has_image_markup(content),
		xml_error,
	}
}

#[cfg(test)]
mod tests {
	use super::{content_has_image_markup, convert_section, is_binary_image};

	#[test]
	fn a_section_that_is_not_well_formed_xml_is_read_as_html_and_says_why() {
		let section = convert_section("<html><body><p>Before <img src='a.jpg'> after</p></body></html>", false);
		assert!(section.text.contains("Before"), "{:?}", section.text);
		assert!(section.xml_error.is_some());
	}

	#[test]
	fn a_well_formed_section_records_no_xml_error() {
		let section = convert_section("<html><body><p>Fine</p></body></html>", false);
		assert_eq!(section.xml_error, None);
	}

	#[test]
	fn image_markup_is_detected_however_it_is_written() {
		assert!(content_has_image_markup("<body><img src='x.jpg'/></body>"));
		assert!(content_has_image_markup("<svg><image href='x'/></svg>"));
		assert!(content_has_image_markup("<figure><figcaption>x</figcaption></figure>"));
		assert!(content_has_image_markup("<BODY><IMG SRC='x'/></BODY>"));
		assert!(!content_has_image_markup("<body><p>Just words here.</p></body>"));
	}

	#[test]
	fn a_raster_image_is_binary_but_an_svg_is_not() {
		assert!(is_binary_image("image/jpeg"));
		assert!(is_binary_image("image/png"));
		assert!(is_binary_image("image/gif"));
		assert!(!is_binary_image("image/svg+xml"));
		assert!(!is_binary_image("application/xhtml+xml"));
		assert!(!is_binary_image("text/css"));
	}
}
