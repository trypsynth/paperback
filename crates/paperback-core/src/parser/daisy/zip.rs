use std::{collections::HashMap, fs::File, io::BufReader, path::Path};

use anyhow::{Context, Result};
use zip::ZipArchive;

use super::{
	daisy2::{build_daisy2_document, build_daisy2_text_only_document},
	ncx::{parse_daisy_ncx, parse_daisy2_ncc_metadata},
	opf::{find_single_dtbook_href, parse_opf_package},
	plain_audio::build_plain_audio_zip_document,
	timeline::build_daisy_document,
};
use crate::{
	audio::AudioLocation,
	document::{Document, DocumentBuffer, Marker, MarkerType, ParserContext},
	parser::{
		PASSWORD_REQUIRED_ERROR_PREFIX, add_converter_markers,
		convert::xml_to_text::XmlToText,
		util::{path::extract_title_from_path, toc::build_toc_from_headings},
	},
	t,
	util::zip::read_zip_entry_by_name_with_password,
};

/// Parses a DAISY book packaged as a zip archive: DAISY 3 (OPF manifest plus DTBook XML, with
/// or without a SMIL audio layer), DAISY 2.02 (`ncc.html`), or, failing both, a bare bundle of
/// audio files with no markup at all.
pub(super) fn parse(context: &ParserContext, path: &Path) -> Result<Document> {
	let mut title = extract_title_from_path(&context.file_path);
	let mut author = String::new();
	let buffer;
	let file = File::open(path).context("Failed to open zip file")?;
	let mut archive = ZipArchive::new(BufReader::new(file)).context("Failed to read zip archive")?;
	// zip 9 hands back a Result per name, since decoding one can fail. A name that will not
	// decode cannot match what this scan is looking for, so drop those rather than fail the file.
	let opf_path = archive
		.file_names()
		.flatten()
		.find(|n| Path::new(&**n).extension().is_some_and(|ext| ext.eq_ignore_ascii_case("opf")))
		.map(String::from);
	let opf_found = opf_path.is_some();
	if let Some(opf_name) = opf_path {
		let opf_dir = opf_name.rsplit_once('/').map_or_else(String::new, |(dir, _)| dir.to_string());
		let opf_content = read_zip_entry_by_name_with_password(&mut archive, &opf_name, context.password.as_deref())
			.map_err(|e| {
				if e.to_string().starts_with(PASSWORD_REQUIRED_ERROR_PREFIX) {
					e
				} else {
					e.context("Failed to read OPF file")
				}
			})?;
		let package = parse_opf_package(&opf_content, &opf_dir)?;
		if let Some(t) = package.title.clone() {
			title = t;
		}
		if let Some(a) = package.author.clone() {
			author = a;
		}
		let password = context.password.clone();
		let archive_path = context.file_path.clone();
		{
			let mut read_text = |href: &str| -> Result<String> {
				read_zip_entry_by_name_with_password(&mut archive, href, password.as_deref())
			};
			let resolve_audio = |href: &str| AudioLocation::ZipEntry {
				archive: archive_path.clone(),
				entry: href.to_string(),
				password: password.clone(),
			};
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
		if let Some(dtbook_path) = find_single_dtbook_href(&package) {
			let xml_content =
				read_zip_entry_by_name_with_password(&mut archive, &dtbook_path, context.password.as_deref()).map_err(
					|e| {
						if e.to_string().starts_with(PASSWORD_REQUIRED_ERROR_PREFIX) {
							e
						} else {
							e.context("Failed to read XML file from zip")
						}
					},
				)?;
			let mut converter = XmlToText::with_render_tables_inline(context.render_tables_inline);
			if converter.convert(&xml_content) {
				let mut converted_buffer = DocumentBuffer::with_content(converter.get_text());
				add_converter_markers(&mut converted_buffer, &converter, 0);
				for pb in converter.get_page_breaks() {
					converted_buffer
						.add_marker(Marker::new(MarkerType::PageBreak, pb.offset).with_text(pb.text.clone()));
				}
				buffer = converted_buffer;
			} else {
				// TRANSLATORS: Error shown when a DAISY book's DTBook XML fails to convert to plain text
				anyhow::bail!(t("Failed to convert DTBook XML to text"));
			}
			let mut toc_items = None;
			let ncx_path = archive
				.file_names()
				.flatten()
				.find(|n| Path::new(&**n).extension().is_some_and(|ext| ext.eq_ignore_ascii_case("ncx")))
				.map(String::from);
			if let Some(ncx_name) = ncx_path {
				match read_zip_entry_by_name_with_password(&mut archive, &ncx_name, context.password.as_deref()) {
					Ok(ncx_content) if !ncx_content.is_empty() => {
						if let Some(ncx_toc) = parse_daisy_ncx(
							&ncx_content,
							&super::opf::dir_of(&ncx_name),
							converter.get_id_positions(),
							&HashMap::new(),
						) && !ncx_toc.is_empty()
						{
							toc_items = Some(ncx_toc);
						}
					}
					Ok(_) => {
						tracing::debug!(ncx_name = %ncx_name, "ncx file is empty, using heading-derived toc");
					}
					Err(e) => {
						tracing::warn!(ncx_name = %ncx_name, error = %e, "ncx file present but failed to read, using heading-derived toc");
					}
				}
			} else {
				tracing::debug!("no ncx file found in zip archive, using heading-derived toc");
			}
			let toc_items = toc_items.unwrap_or_else(|| build_toc_from_headings(converter.get_headings()));
			tracing::debug!(path = %path.display(), "parsed daisy book as daisy 3 (opf and dtbook xml) from zip archive");
			return Ok(Document {
				title,
				author,
				buffer,
				toc_items,
				id_positions: converter.get_id_positions().clone(),
				..Document::default()
			});
		}
		tracing::warn!(opf_name = %opf_name, "opf found but no dtbook manifest item, trying daisy 2.02");
	}
	let ncc_path =
		archive.file_names().flatten().find(|n| n.ends_with("ncc.html") || n.ends_with("NCC.html")).map(String::from);
	let ncc_found = ncc_path.is_some();
	if let Some(ncc_name) = ncc_path {
		let ncc_content = read_zip_entry_by_name_with_password(&mut archive, &ncc_name, context.password.as_deref())
			.map_err(|e| {
				if e.to_string().starts_with(PASSWORD_REQUIRED_ERROR_PREFIX) {
					e
				} else {
					e.context("Failed to read ncc.html")
				}
			})?;
		let (ncc_title, ncc_author) = parse_daisy2_ncc_metadata(&ncc_content);
		if let Some(t) = ncc_title {
			title = t;
		}
		if let Some(a) = ncc_author {
			author = a;
		}
		let password = context.password.clone();
		let archive_path = context.file_path.clone();
		{
			let mut read_text = |href: &str| -> Result<String> {
				read_zip_entry_by_name_with_password(&mut archive, href, password.as_deref())
			};
			let resolve_audio = |href: &str| AudioLocation::ZipEntry {
				archive: archive_path.clone(),
				entry: href.to_string(),
				password: password.clone(),
			};
			if let Some(document) = build_daisy2_document(
				&ncc_content,
				&ncc_name,
				title.clone(),
				author.clone(),
				context.render_tables_inline,
				&mut read_text,
				&resolve_audio,
			) {
				tracing::debug!(path = %path.display(), "parsed daisy book as daisy 2.02 (ncc.html + smil audio) from zip archive");
				return Ok(document);
			}
		}
		let mut read_text = |href: &str| -> Result<String> {
			read_zip_entry_by_name_with_password(&mut archive, href, password.as_deref())
		};
		if let Some(document) = build_daisy2_text_only_document(
			&ncc_content,
			&ncc_name,
			title.clone(),
			author.clone(),
			context.render_tables_inline,
			&mut read_text,
		) {
			tracing::debug!(path = %path.display(), "parsed daisy book as daisy 2.02 (ncc.html, text-only) from zip archive");
			return Ok(document);
		}
	}
	tracing::warn!(opf_found, ncc_found, "exhausted daisy 3 and daisy 2.02 detection attempts in zip archive");
	// Not a recognizable DAISY book, but plenty of "audiobook" zips out there (e.g. from
	// AudioVault) are just a folder of narration files with no markup at all. Rather than
	// refuse those, present each audio file as its own playable, textless section.
	if let Some(document) =
		build_plain_audio_zip_document(&archive, &context.file_path, title, author, context.password.as_deref())
	{
		tracing::debug!(path = %path.display(), "parsed zip archive as a plain audio-only bundle");
		return Ok(document);
	}
	// TRANSLATORS: Error shown when a ZIP file is not a recognizable DAISY 3 or DAISY 2.02 book
	anyhow::bail!(t("ZIP archive does not appear to be a valid DAISY 3 or DAISY 2.02 book"));
}
