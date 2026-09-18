//! Converting an EPUB's spine items to text: each spine item's XHTML is read from the zip
//! and run through the XML/HTML converters in parallel, then the resulting sections are
//! stitched into one [`DocumentBuffer`] with section-break/link markers and id positions
//! placed at their absolute offsets.

use std::{collections::HashMap, fs::File, io::BufReader, mem};

use anyhow::Result;
use rayon::prelude::*;
use zip::ZipArchive;

use super::{href::resolve_href, package::ManifestItem};
use crate::{
	document::{DocumentBuffer, Marker, MarkerType},
	parser::{
		ConverterOutput, add_converter_markers_excluding_links,
		convert::{
			html_to_text::{HtmlSourceMode, HtmlToText},
			xml_to_text::XmlToText,
		},
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
	id_positions: HashMap<String, usize>,
	/// Whether the section's markup holds an image element at all, described or not. The converters
	/// only record an image that carries a description, so this is the only sign that a page whose
	/// whole body is one undescribed picture is a picture and not simply empty.
	has_image_markup: bool,
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
	pub(super) start: usize,
	pub(super) end: usize,
}

pub(super) struct SpineConversionResult {
	pub(super) buffer: DocumentBuffer,
	pub(super) id_positions: HashMap<String, usize>,
	pub(super) sections: Vec<SectionMeta>,
	pub(super) conversion_errors: Vec<String>,
}

/// Reads and converts every spine item to text. Each rayon worker opens its own independent
/// `ZipArchive` (via `map_init`, so this happens once per task rather than once per item) so the
/// zip-read I/O and the HTML-to-text conversion both run across cores instead of the read being
/// serialized through one shared archive handle before conversion can start.
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
				let section =
					convert_section(&data, render_tables_inline).map_err(|err| format!("{} ({err})", item.path))?;
				Ok((item, section))
			},
		)
		.collect();
	// Keep each spine item's original index (for the "Section N" label, which reflects spine
	// position even across skipped items) alongside its manifest item and converted section.
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
	// `DocumentBuffer::from_parts` builds the buffer's content and per-char indices for every
	// section in parallel, in one pass, instead of appending them one at a time; it hands back
	// each section's `[start, end)` span so markers and id positions (below) can still be placed
	// relative to where each section landed.
	// A spine item that is a picture, or a wrapper page whose whole body is one image, converts to
	// no text and would otherwise leave the reader a blank stretch of book with no sign a page is
	// there. Each such section is given a one-line image placeholder so the book is navigable and
	// the picture can be reached; see [`section_is_image`].
	let image_sections: Vec<bool> =
		ok_entries.iter().map(|(_, item, section)| section_is_image(item, section)).collect();
	let image_placeholder = format!("[{}]", t("Image"));
	// A section is replaced by the placeholder when it is a picture with no real text of its own.
	// Its converter markers and id positions point into the text that was dropped, so they are not
	// applied below.
	let mut replaced = vec![false; ok_entries.len()];
	let texts: Vec<String> = ok_entries
		.iter_mut()
		.zip(&image_sections)
		.enumerate()
		.map(|(index, ((_, item, section), is_image))| {
			let text = mem::take(&mut section.text);
			// A raster image read as a spine item comes through as the decode garbage of its own
			// bytes, which is never real text, so it is always replaced. An SVG or a wrapper page
			// keeps whatever real text it has and takes the placeholder only when it has none.
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
		// An image page gets an Image marker so it is announced and reachable with the image
		// navigation key, the same as a picture inside a text page. The section's path rides along
		// as the reference, as it does for a comic archive's pages.
		if *is_image {
			buffer.add_marker(Marker::new(MarkerType::Image, section_start).with_reference(item.path.clone()));
		}
		// A section whose text was replaced by the placeholder has no content its markers or id
		// positions still line up with, so only its section break and image marker are kept.
		if !was_replaced {
			for (id, relative) in &section.id_positions {
				let absolute = section_start + relative;
				// Keep the first occurrence for bare ids to avoid later sections overwriting earlier ones.
				id_positions.entry(id.clone()).or_insert(absolute);
				id_positions.insert(format!("{}#{id}", item.path), absolute);
			}
			add_converter_markers_excluding_links(&mut buffer, section, section_start);
			for link in &section.links {
				let resolved = resolve_href(&item.path, &link.reference);
				buffer.add_marker(
					Marker::new(MarkerType::Link, section_start + link.offset)
						.with_text(link.text.clone())
						.with_reference(resolved),
				);
			}
		}
		sections.push(SectionMeta { path: item.path.clone(), start: section_start, end: span.end });
	}
	SpineConversionResult { buffer, id_positions, sections, conversion_errors }
}

/// Whether a spine item is a page the reader meets as a picture rather than as text.
///
/// Two shapes count. A spine item that is itself an image (`image/svg+xml`, `image/jpeg`, and the
/// like) is a page-as-picture, which is how the fixed-layout books in the EPUB sample set are
/// built. A wrapper page whose media type is XHTML but whose whole body came through as an image
/// with no text of its own is the other, which is how a picture book puts one plate on each page.
/// A page that has any text is not one of these, whatever else it also holds.
fn section_is_image(item: &ManifestItem, section: &SectionContent) -> bool {
	if item.media_type.starts_with("image/") {
		return true;
	}
	section.has_image_markup && section.text.trim().is_empty()
}

/// Whether a section's markup contains an image element (`<img>`, SVG `<image>`, or a `<figure>`),
/// described or not. A plain substring test is enough: it only decides whether a text-less page is
/// a picture to announce, and a false positive there costs one image line on an already empty page.
fn content_has_image_markup(content: &str) -> bool {
	let lowered = content.to_ascii_lowercase();
	lowered.contains("<img") || lowered.contains("<image") || lowered.contains("<figure")
}

/// Whether a spine item is a raster image, whose bytes are not text and decode to garbage when
/// read as any. SVG is left out: it is real XML and may hold text worth keeping.
fn is_binary_image(media_type: &str) -> bool {
	media_type.starts_with("image/") && media_type != "image/svg+xml"
}

fn convert_section(content: &str, render_tables_inline: bool) -> Result<SectionContent> {
	let mut xml_converter = XmlToText::with_render_tables_inline(render_tables_inline);
	if xml_converter.convert(content) {
		return Ok(SectionContent {
			text: xml_converter.get_text(),
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
		});
	}
	tracing::warn!("epub section xml conversion failed, falling back to html converter");
	let mut html_converter = HtmlToText::with_render_tables_inline(render_tables_inline);
	if html_converter.convert(content, HtmlSourceMode::NativeHtml) {
		return Ok(SectionContent {
			text: html_converter.get_text(),
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
		});
	}
	// currently unreachable, HtmlToText::convert always returns true today
	tracing::warn!("epub section content unsupported by both xml and html converters");
	// TRANSLATORS: Error shown when an EPUB spine item's content type cannot be converted
	anyhow::bail!(t("unsupported content"))
}

#[cfg(test)]
mod tests {
	use super::{content_has_image_markup, is_binary_image};

	#[test]
	fn image_markup_is_detected_however_it_is_written() {
		assert!(content_has_image_markup("<body><img src='x.jpg'/></body>"));
		assert!(content_has_image_markup("<svg><image href='x'/></svg>"));
		assert!(content_has_image_markup("<figure><figcaption>x</figcaption></figure>"));
		// Case does not matter.
		assert!(content_has_image_markup("<BODY><IMG SRC='x'/></BODY>"));
		// A page of plain prose is not an image page.
		assert!(!content_has_image_markup("<body><p>Just words here.</p></body>"));
	}

	#[test]
	fn a_raster_image_is_binary_but_an_svg_is_not() {
		assert!(is_binary_image("image/jpeg"));
		assert!(is_binary_image("image/png"));
		assert!(is_binary_image("image/gif"));
		// SVG is real XML and may carry text, so it is not treated as binary.
		assert!(!is_binary_image("image/svg+xml"));
		assert!(!is_binary_image("application/xhtml+xml"));
		assert!(!is_binary_image("text/css"));
	}
}
