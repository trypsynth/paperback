//! Builds a DAISY 2.02 document: `ncc.html` names the SMIL files (one per section, usually),
//! each SMIL aligns a `<par>`'s `<text>` fragment (into one or more shared content HTML files)
//! with an `<audio>` clip, exactly like a DAISY 3 NCX/SMIL pair except the "master" file is
//! `ncc.html` itself rather than an OPF spine plus a separate NCX.

use std::collections::HashMap;

use anyhow::Result;

use super::{
	ncx::{extract_daisy2_links, parse_daisy2_ncc_headings},
	opf::dir_of,
	smil::parse_smil_pars,
	timeline::{PendingClip, bound_open_ended_clips},
};
use crate::{
	audio::{AudioLocation, AudioTimelineBuilder},
	document::{Document, DocumentBuffer},
	parser::{
		add_converter_markers,
		convert::html_to_text::{HtmlSourceMode, HtmlToText},
		util::{path::resolve_relative_path, toc::build_toc_from_headings},
	},
};

/// Converts one content HTML file into the shared buffer, offsetting its markers and id
/// positions by the buffer's current length. A no-op if `href` was already converted.
fn convert_html_file(
	href: &str,
	read_text: &mut dyn FnMut(&str) -> Result<String>,
	render_tables_inline: bool,
	buffer: &mut DocumentBuffer,
	id_positions: &mut HashMap<String, usize>,
	file_offsets: &mut HashMap<String, usize>,
	file_ids: &mut HashMap<String, HashMap<String, usize>>,
) -> Result<()> {
	if file_offsets.contains_key(href) {
		return Ok(());
	}
	let html_content = read_text(href)?;
	let mut converter = HtmlToText::with_render_tables_inline(render_tables_inline);
	converter.convert(&html_content, HtmlSourceMode::NativeHtml);
	let offset = buffer.current_position();
	buffer.append(&converter.get_text());
	add_converter_markers(buffer, &converter, offset);
	for (id, pos) in converter.get_id_positions() {
		// Bare ids are only unique within their own file, so keep the first file's position
		// and expose a path-qualified key alongside it.
		let absolute = offset + pos;
		id_positions.entry(id.clone()).or_insert(absolute);
		id_positions.insert(format!("{href}#{id}"), absolute);
	}
	file_ids.insert(href.to_string(), converter.get_id_positions().clone());
	file_offsets.insert(href.to_string(), offset);
	Ok(())
}

/// Builds a DAISY 2.02 document by walking `ncc.html`'s links to SMIL files, converting each
/// content HTML file a `<text>` element references into the shared buffer, and turning every
/// SMIL `<par>` into an `AudioClip` anchored at that text's position. Sections that fail to
/// convert are skipped. Returns `None` when `ncc.html` names no SMIL files at all (a text-only
/// DAISY 2.02 book links straight to content HTML instead), so the caller can fall back to
/// treating the linked files as plain HTML.
pub(super) fn build_daisy2_document(
	ncc_content: &str,
	ncc_name: &str,
	title: String,
	author: String,
	render_tables_inline: bool,
	read_text: &mut dyn FnMut(&str) -> Result<String>,
	resolve_audio: &dyn Fn(&str) -> AudioLocation,
) -> Option<Document> {
	let ncc_dir = dir_of(ncc_name);
	let smil_links: Vec<String> =
		extract_daisy2_links(ncc_content).into_iter().filter(|l| l.to_ascii_lowercase().ends_with(".smil")).collect();
	if smil_links.is_empty() {
		return None;
	}

	let mut buffer = DocumentBuffer::new();
	let mut id_positions: HashMap<String, usize> = HashMap::new();
	let mut file_offsets: HashMap<String, usize> = HashMap::new();
	let mut file_ids: HashMap<String, HashMap<String, usize>> = HashMap::new();
	let mut source_indices: HashMap<String, usize> = HashMap::new();
	let mut audio_builder = AudioTimelineBuilder::new();
	let mut pending_clips: Vec<PendingClip> = Vec::new();
	// Where each id *inside the SMIL files* lands in the text, which is what an `ncc.html`
	// heading link points at. Keyed both bare and path-qualified (against the link text as
	// written in `ncc.html`), first occurrence winning.
	let mut smil_anchors: HashMap<String, usize> = HashMap::new();
	let mut converted_any = false;

	for smil_link in &smil_links {
		let smil_href = resolve_relative_path(&ncc_dir, smil_link);
		let Ok(smil_content) = read_text(&smil_href) else { continue };
		let smil_dir = dir_of(&smil_href);
		// A `<text src>` naming only a fragment resolves against the file the most recent
		// explicit reference in this same SMIL named.
		let mut current_text_href: Option<String> = None;
		for par in parse_smil_pars(&smil_content) {
			let text_href = match &par.text_file {
				Some(text_file) => {
					let href = resolve_relative_path(&smil_dir, text_file);
					current_text_href = Some(href.clone());
					href
				}
				None => {
					let Some(href) = current_text_href.clone() else { continue };
					href
				}
			};
			if !file_offsets.contains_key(&text_href)
				&& convert_html_file(
					&text_href,
					read_text,
					render_tables_inline,
					&mut buffer,
					&mut id_positions,
					&mut file_offsets,
					&mut file_ids,
				)
				.is_err()
			{
				continue;
			}
			converted_any = true;
			let (Some(&base), Some(&local_pos)) =
				(file_offsets.get(&text_href), file_ids.get(&text_href).and_then(|ids| ids.get(&par.text_id)))
			else {
				continue;
			};
			let position = base + local_pos;
			for anchor in &par.anchor_ids {
				smil_anchors.entry(anchor.clone()).or_insert(position);
				smil_anchors.entry(format!("{smil_link}#{anchor}")).or_insert(position);
			}
			let audio_href = resolve_relative_path(&smil_dir, &par.audio_src);
			let source = *source_indices
				.entry(audio_href.clone())
				.or_insert_with(|| audio_builder.add_source(resolve_audio(&audio_href), None));
			pending_clips.push(PendingClip {
				source,
				begin_ms: par.clip_begin_ms,
				end_ms: par.clip_end_ms,
				start: position,
			});
		}
	}

	if !converted_any {
		return None;
	}

	let headings = parse_daisy2_ncc_headings(ncc_content, &id_positions, &smil_anchors);
	let toc_items = build_toc_from_headings(&headings);

	let resolved_clips = bound_open_ended_clips(&pending_clips);
	let doc_end = buffer.current_position();
	for index in 0..resolved_clips.len() {
		let clip = &resolved_clips[index];
		let end = resolved_clips.get(index + 1).map_or(doc_end, |next| next.start);
		audio_builder.add_clip(clip.source, clip.begin_ms, clip.end_ms, clip.start, end);
	}
	let audio = audio_builder.build();

	Some(Document {
		title,
		author,
		buffer,
		toc_items,
		id_positions,
		audio: (!audio.is_empty()).then_some(audio),
		..Document::default()
	})
}

/// Builds a text-only DAISY 2.02 document: `ncc.html` links straight at content HTML pages
/// with no SMIL audio layer. Combines every linked page into one buffer and derives the TOC
/// from its headings. Returns `None` when none of the linked pages could be read.
pub(super) fn build_daisy2_text_only_document(
	ncc_content: &str,
	ncc_name: &str,
	title: String,
	author: String,
	render_tables_inline: bool,
	read_text: &mut dyn FnMut(&str) -> Result<String>,
) -> Option<Document> {
	let ncc_dir = dir_of(ncc_name);
	let mut combined_html = String::new();
	for link in extract_daisy2_links(ncc_content) {
		let href = resolve_relative_path(&ncc_dir, &link);
		match read_text(&href) {
			Ok(content) => {
				combined_html.push_str(&content);
				combined_html.push_str("\n\n");
			}
			Err(e) => {
				tracing::warn!(link = %href, error = %e, "failed to read linked content page, skipping");
			}
		}
	}
	if combined_html.is_empty() {
		return None;
	}
	let mut converter = HtmlToText::with_render_tables_inline(render_tables_inline);
	converter.convert(&combined_html, HtmlSourceMode::NativeHtml);
	let mut buffer = DocumentBuffer::with_content(converter.get_text());
	add_converter_markers(&mut buffer, &converter, 0);
	let toc_items = build_toc_from_headings(converter.get_headings());
	Some(Document {
		title,
		author,
		buffer,
		toc_items,
		id_positions: converter.get_id_positions().clone(),
		..Document::default()
	})
}
