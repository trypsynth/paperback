use std::{
	collections::HashMap,
	fs::File,
	io::{BufReader, Read, Seek},
	path::Path,
};

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
	document::{DocPos, Document, DocumentBuffer, Marker, MarkerType, ParserContext},
	parser::{
		PASSWORD_REQUIRED_ERROR_PREFIX, add_converter_markers,
		convert::xml_to_text::XmlToText,
		is_data_uri, is_external_url,
		util::{
			path::{extract_title_from_path, resolve_relative_path},
			toc::build_toc_from_headings,
		},
	},
	t,
	util::{
		text::url_decode,
		zip::{read_zip_entry_by_name_with_password, zip_entry_name_resolver},
	},
};

pub(super) fn parse(context: &ParserContext, path: &Path) -> Result<Document> {
	let mut document = parse_archive(context, path)?;
	resolve_picture_entries(&mut document, path);
	Ok(document)
}

/// The markup names a picture relative to its text file, which nothing that later saves the picture or reads it with OCR can know, so the entry is settled here while the archive's layout is at hand.
fn resolve_picture_entries(document: &mut Document, path: &Path) {
	let Ok(archive) =
		File::open(path).map(BufReader::new).and_then(|file| ZipArchive::new(file).map_err(std::io::Error::other))
	else {
		return;
	};
	let names: Vec<String> = archive.file_names().flatten().map(String::from).collect();
	let mut folders: Vec<String> = names
		.iter()
		.filter(|name| {
			Path::new(name.as_str()).extension().is_some_and(|ext| {
				["opf", "xml", "html", "htm", "xhtml", "ncx", "smil"]
					.iter()
					.any(|known| ext.eq_ignore_ascii_case(known))
			})
		})
		.map(|name| name.rsplit_once('/').map_or_else(String::new, |(folder, _)| folder.to_string()))
		.collect();
	folders.sort();
	folders.dedup();
	for marker in &mut document.buffer.markers {
		if !matches!(marker.mtype, MarkerType::Image | MarkerType::Figure)
			|| marker.reference.is_empty()
			|| is_data_uri(&marker.reference)
			|| is_external_url(&marker.reference)
		{
			continue;
		}
		if let Some(entry) = picture_entry(&names, &folders, &marker.reference) {
			marker.reference = entry;
		}
	}
}

/// Matched without regard to case, as the rest of the book's files are.
fn picture_entry(names: &[String], folders: &[String], source: &str) -> Option<String> {
	let source = url_decode(source.split(['#', '?']).next().unwrap_or_default());
	let find = |wanted: &str| names.iter().find(|name| name.eq_ignore_ascii_case(wanted)).cloned();
	std::iter::once(source.trim_start_matches('/').to_string())
		.chain(folders.iter().map(|folder| resolve_relative_path(folder, &source)))
		.find_map(|candidate| find(&candidate))
		.or_else(|| {
			let ending = format!("/{}", source.trim_start_matches(['.', '/']).to_lowercase());
			let mut matches = names.iter().filter(|name| name.to_lowercase().ends_with(&ending));
			match (matches.next(), matches.next()) {
				(Some(only), None) => Some(only.clone()),
				_ => None,
			}
		})
}

fn parse_archive(context: &ParserContext, path: &Path) -> Result<Document> {
	let mut title = extract_title_from_path(&context.file_path);
	let mut author = String::new();
	let file = File::open(path).context("Failed to open zip file")?;
	let mut archive = ZipArchive::new(BufReader::new(file)).context("Failed to read zip archive")?;
	let entry_name = zip_entry_name_resolver(&archive);
	// zip 9 hands back a Result per name; one that will not decode cannot match what this scan looks for, so drop it rather than fail the file.
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
				read_zip_entry_by_name_with_password(&mut archive, &entry_name(href), password.as_deref())
			};
			let resolve_audio = |href: &str| AudioLocation::ZipEntry {
				archive: archive_path.clone(),
				entry: entry_name(href),
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
			tracing::debug!(path = %path.display(), "parsing daisy book as daisy 3 (opf and dtbook xml) from zip archive");
			return dtbook_document(&mut archive, &dtbook_path, context, title, author);
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
				read_zip_entry_by_name_with_password(&mut archive, &entry_name(href), password.as_deref())
			};
			let resolve_audio = |href: &str| AudioLocation::ZipEntry {
				archive: archive_path.clone(),
				entry: entry_name(href),
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
			read_zip_entry_by_name_with_password(&mut archive, &entry_name(href), password.as_deref())
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
	// A DTBook on its own, without the package file a full DAISY 3 book has, is how the DAISY Consortium publishes some text samples.
	#[allow(clippy::needless_collect, reason = "the names borrow the archive the search then reads from")]
	let xml_names: Vec<String> = archive
		.file_names()
		.flatten()
		.filter(|name| name.to_ascii_lowercase().ends_with(".xml"))
		.map(String::from)
		.collect();
	if let Some(dtbook_name) = xml_names.into_iter().find(|name| {
		read_zip_entry_by_name_with_password(&mut archive, name, context.password.as_deref())
			.is_ok_and(|xml| xml.contains("<dtbook"))
	}) {
		tracing::debug!(path = %path.display(), dtbook_name = %dtbook_name, "parsing a lone dtbook from zip archive");
		return dtbook_document(&mut archive, &dtbook_name, context, title, author);
	}
	// Plenty of "audiobook" zips (e.g. from AudioVault) are just a folder of narration files with no markup, so each file becomes a textless section rather than the zip being refused.
	if let Some(document) =
		build_plain_audio_zip_document(&mut archive, &context.file_path, title, author, context.password.as_deref())
	{
		tracing::debug!(path = %path.display(), "parsed zip archive as a plain audio-only bundle");
		return Ok(document);
	}
	// TRANSLATORS: Error shown when a ZIP file is not a recognizable DAISY 3 or DAISY 2.02 book
	anyhow::bail!(t("ZIP archive does not appear to be a valid DAISY 3 or DAISY 2.02 book"));
}

/// The DTBook text, with the NCX's table of contents when the archive has one and its headings when it doesn't.
fn dtbook_document<R: Read + Seek>(
	archive: &mut ZipArchive<R>,
	dtbook_path: &str,
	context: &ParserContext,
	title: String,
	author: String,
) -> Result<Document> {
	let xml_content =
		read_zip_entry_by_name_with_password(archive, dtbook_path, context.password.as_deref()).map_err(|e| {
			if e.to_string().starts_with(PASSWORD_REQUIRED_ERROR_PREFIX) {
				e
			} else {
				e.context("Failed to read XML file from zip")
			}
		})?;
	let mut converter = XmlToText::with_render_tables_inline(context.render_tables_inline);
	let buffer = if converter.convert(&xml_content) {
		let mut converted_buffer = DocumentBuffer::with_content(converter.take_text());
		add_converter_markers(&mut converted_buffer, &converter, DocPos::ZERO);
		for pb in converter.get_page_breaks() {
			converted_buffer.add_marker(Marker::new(MarkerType::PageBreak, pb.offset).with_text(pb.text.clone()));
		}
		converted_buffer
	} else {
		// TRANSLATORS: Error shown when a DAISY book's DTBook XML fails to convert to plain text
		anyhow::bail!(t("Failed to convert DTBook XML to text"));
	};
	let mut toc_items = None;
	let ncx_path = archive
		.file_names()
		.flatten()
		.find(|n| Path::new(&**n).extension().is_some_and(|ext| ext.eq_ignore_ascii_case("ncx")))
		.map(String::from);
	if let Some(ncx_name) = ncx_path {
		match read_zip_entry_by_name_with_password(archive, &ncx_name, context.password.as_deref()) {
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
	Ok(Document {
		title,
		author,
		buffer,
		toc_items,
		id_positions: converter.get_id_positions().clone(),
		..Document::default()
	})
}
