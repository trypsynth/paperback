use std::{collections::HashMap, fs, path::Path};

use anyhow::{Context, Result};

use super::{
	daisy2::{build_daisy2_document, build_daisy2_text_only_document},
	ncx::{parse_daisy_ncx, parse_daisy2_ncc_metadata},
	opf::{find_single_dtbook_href, parse_opf_package},
	timeline::build_daisy_document,
};
use crate::{
	audio::AudioLocation,
	document::{Document, DocumentBuffer, Marker, MarkerType, ParserContext},
	parser::{
		add_converter_markers,
		convert::xml_to_text::XmlToText,
		util::{path::extract_title_from_path, toc::build_toc_from_headings},
	},
	t,
	util::encoding::convert_to_utf8,
};

/// Parses a DAISY book laid out as loose files on disk: either DAISY 3, where
/// `context.file_path` names the OPF with the DTBook XML (and, for narrated books, the SMIL and
/// audio files) sitting alongside it, or DAISY 2.02, where it names `ncc.html` with the SMIL,
/// content HTML, and audio files sitting alongside that instead.
pub(super) fn parse(context: &ParserContext, path: &Path) -> Result<Document> {
	let mut title = extract_title_from_path(&context.file_path);
	let mut author = String::new();
	if path.file_name().is_some_and(|n| n.eq_ignore_ascii_case("ncc.html")) {
		return parse_daisy2(context, path, title);
	}
	let opf_content = convert_to_utf8(&fs::read(path)?);
	let package = parse_opf_package(&opf_content, "")?;
	if let Some(t) = package.title.clone() {
		title = t;
	}
	if let Some(a) = package.author.clone() {
		author = a;
	}
	let base_dir = path.parent().unwrap_or_else(|| Path::new(""));
	{
		let mut read_text = |href: &str| -> Result<String> {
			let full_path = base_dir.join(href);
			Ok(convert_to_utf8(
				&fs::read(&full_path).with_context(|| format!("Failed to read file at {}", full_path.display()))?,
			))
		};
		let resolve_audio = |href: &str| AudioLocation::File(base_dir.join(href).to_string_lossy().to_string());
		if let Some(document) = build_daisy_document(
			&package,
			title.clone(),
			author.clone(),
			context.render_tables_inline,
			&mut read_text,
			&resolve_audio,
		) {
			return Ok(document);
		}
	}
	let dtbook_href = find_single_dtbook_href(&package);
	let dtbook_found = dtbook_href.is_some();
	if let Some(dtbook_path) = dtbook_href {
		let xml_full_path = base_dir.join(&dtbook_path);
		let xml_content = convert_to_utf8(
			&fs::read(&xml_full_path)
				.with_context(|| format!("Failed to read DTBook XML file at {}", xml_full_path.display()))?,
		);
		let mut converter = XmlToText::with_render_tables_inline(context.render_tables_inline);
		if converter.convert(&xml_content) {
			let mut buffer = DocumentBuffer::with_content(converter.get_text());
			add_converter_markers(&mut buffer, &converter, 0);
			for pb in converter.get_page_breaks() {
				buffer.add_marker(Marker::new(MarkerType::PageBreak, pb.offset).with_text(pb.text.clone()));
			}
			let mut toc_items = None;
			let mut ncx_found = false;
			if let Ok(entries) = fs::read_dir(base_dir) {
				for entry in entries.flatten() {
					let entry_path = entry.path();
					if entry_path.is_file() && entry_path.extension().is_some_and(|e| e.eq_ignore_ascii_case("ncx")) {
						ncx_found = true;
						match fs::read(&entry_path) {
							Ok(bytes) => {
								let ncx_content = convert_to_utf8(&bytes);
								if let Some(ncx_toc) =
									parse_daisy_ncx(&ncx_content, "", converter.get_id_positions(), &HashMap::new())
									&& !ncx_toc.is_empty()
								{
									toc_items = Some(ncx_toc);
									break;
								}
							}
							Err(e) => {
								tracing::warn!(path = %entry_path.display(), error = %e, "ncx file present but failed to read, using heading-derived toc");
							}
						}
					}
				}
			}
			if toc_items.is_none() {
				if ncx_found {
					tracing::debug!("ncx file found but did not yield toc items, using heading-derived toc");
				} else {
					tracing::debug!("no ncx file found in directory, using heading-derived toc");
				}
			}
			let toc_items = toc_items.unwrap_or_else(|| build_toc_from_headings(converter.get_headings()));
			tracing::debug!(path = %path.display(), "parsed daisy book as daisy 3 (opf and dtbook xml) from loose files");
			return Ok(Document {
				title,
				author,
				buffer,
				toc_items,
				id_positions: converter.get_id_positions().clone(),
				..Document::default()
			});
		}
	}
	tracing::warn!(dtbook_found, "could not parse daisy opf file or locate dtbook xml in manifest");
	// TRANSLATORS: Error shown when a DAISY .opf file is invalid or its DTBook XML can't be located
	anyhow::bail!(t("Invalid DAISY .opf file or could not find DTBook XML in manifest"));
}

/// Parses a DAISY 2.02 book laid out as loose files on disk: `path` names `ncc.html`, with the
/// SMIL (for narrated books), content HTML, and audio files sitting alongside it.
fn parse_daisy2(context: &ParserContext, path: &Path, mut title: String) -> Result<Document> {
	let ncc_content = convert_to_utf8(&fs::read(path)?);
	let mut author = String::new();
	let (ncc_title, ncc_author) = parse_daisy2_ncc_metadata(&ncc_content);
	if let Some(t) = ncc_title {
		title = t;
	}
	if let Some(a) = ncc_author {
		author = a;
	}
	let base_dir = path.parent().unwrap_or_else(|| Path::new(""));
	let ncc_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("ncc.html");
	let mut read_text = |href: &str| -> Result<String> {
		let full_path = base_dir.join(href);
		Ok(convert_to_utf8(
			&fs::read(&full_path).with_context(|| format!("Failed to read file at {}", full_path.display()))?,
		))
	};
	let resolve_audio = |href: &str| AudioLocation::File(base_dir.join(href).to_string_lossy().to_string());
	if let Some(document) = build_daisy2_document(
		&ncc_content,
		ncc_name,
		title.clone(),
		author.clone(),
		context.render_tables_inline,
		&mut read_text,
		&resolve_audio,
	) {
		tracing::debug!(path = %path.display(), "parsed daisy book as daisy 2.02 (ncc.html + smil audio) from loose files");
		return Ok(document);
	}
	if let Some(document) = build_daisy2_text_only_document(
		&ncc_content,
		ncc_name,
		title,
		author,
		context.render_tables_inline,
		&mut read_text,
	) {
		tracing::debug!(path = %path.display(), "parsed daisy book as daisy 2.02 (ncc.html, text-only) from loose files");
		return Ok(document);
	}
	// TRANSLATORS: Error shown when a DAISY 2.02 ncc.html file names no readable content pages
	anyhow::bail!(t("Invalid DAISY 2.02 ncc.html file or could not find its content pages"));
}
