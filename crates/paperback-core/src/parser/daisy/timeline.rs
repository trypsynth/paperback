use std::collections::HashMap;

use anyhow::Result;

use super::{
	ncx::parse_daisy_ncx,
	opf::{OpfPackage, dir_of, is_dtbook_like_item},
	smil::parse_smil_pars,
};
use crate::{
	audio::{AudioLocation, AudioTimelineBuilder},
	document::{Document, DocumentBuffer, Marker, MarkerType},
	parser::{
		add_converter_markers,
		convert::xml_to_text::XmlToText,
		util::{path::resolve_relative_path, toc::build_toc_from_buffer},
	},
	t,
};

/// Converts one `DTBook` XML file into the shared buffer, offsetting its markers and id
/// positions by the buffer's current length. A no-op if `href` was already converted.
fn convert_dtbook_file(
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
	let xml_content = read_text(href)?;
	let mut converter = XmlToText::with_render_tables_inline(render_tables_inline);
	if !converter.convert(&xml_content) {
		// TRANSLATORS: Error shown when a DAISY book's DTBook XML fails to convert to plain text
		anyhow::bail!(t("Failed to convert DTBook XML to text"));
	}
	let offset = buffer.current_position();
	buffer.append(&converter.get_text());
	add_converter_markers(buffer, &converter, offset);
	for pb in converter.get_page_breaks() {
		buffer.add_marker(Marker::new(MarkerType::PageBreak, offset + pb.offset).with_text(pb.text.clone()));
	}
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

/// One SMIL `<par>`'s audio, before its text span is known. `end_ms` is `None` while the
/// par's `clipEnd` was absent, i.e. it plays to the end of its file.
struct PendingClip {
	source: usize,
	begin_ms: u64,
	end_ms: Option<u64>,
	start: usize,
}

/// A clip with a known text position and a known end time, ready for the timeline.
struct ResolvedClip {
	source: usize,
	begin_ms: u64,
	end_ms: u64,
	start: usize,
}

/// Gives every clip a concrete end time and sorts the result by text position. A par with no
/// `clipEnd` runs "to the end of the media" per SMIL 2.0, which in a DAISY book means up to
/// whatever plays next from the same audio file, so it is bounded by the next clip against
/// that source, searched across the whole book rather than just its own SMIL. A trailing
/// open-ended clip has nothing after it to measure against and would need the file's real
/// duration, which the parser doesn't decode, so it is dropped.
fn bound_open_ended_clips(pending: &[PendingClip]) -> Vec<ResolvedClip> {
	let mut resolved: Vec<ResolvedClip> = pending
		.iter()
		.filter_map(|clip| {
			let end_ms = clip.end_ms.or_else(|| {
				pending
					.iter()
					.filter(|other| other.source == clip.source && other.begin_ms > clip.begin_ms)
					.map(|other| other.begin_ms)
					.min()
			})?;
			Some(ResolvedClip { source: clip.source, begin_ms: clip.begin_ms, end_ms, start: clip.start })
		})
		.collect();
	resolved.sort_by_key(|clip| clip.start);
	resolved
}

/// Builds a multi-file DAISY 3 document by walking the OPF spine, converting each `DTBook`
/// XML section (directly, or via a SMIL file's `<text>` references) into the shared buffer
/// and turning every SMIL `<par>` into an `AudioClip` anchored at that text's position.
/// Chapters that fail to convert are skipped. Returns `None` when the spine is empty or
/// names nothing convertible, so the caller can fall back to single-file handling.
pub(super) fn build_daisy_document(
	package: &OpfPackage,
	title: String,
	author: String,
	render_tables_inline: bool,
	read_text: &mut dyn FnMut(&str) -> Result<String>,
	resolve_audio: &dyn Fn(&str) -> AudioLocation,
) -> Option<Document> {
	if package.spine.is_empty() {
		return None;
	}
	let mut buffer = DocumentBuffer::new();
	let mut id_positions: HashMap<String, usize> = HashMap::new();
	let mut file_offsets: HashMap<String, usize> = HashMap::new();
	let mut file_ids: HashMap<String, HashMap<String, usize>> = HashMap::new();
	let mut source_indices: HashMap<String, usize> = HashMap::new();
	let mut audio_builder = AudioTimelineBuilder::new();
	let mut pending_clips: Vec<PendingClip> = Vec::new();
	// Where each id *inside the SMIL files* lands in the text, which is what a DAISY 3 NCX
	// points at. Keyed both bare and path-qualified, first occurrence winning.
	let mut smil_anchors: HashMap<String, usize> = HashMap::new();
	let mut converted_any = false;

	for idref in &package.spine {
		let Some(item) = package.item(idref) else { continue };
		if is_dtbook_like_item(item) {
			// One malformed chapter shouldn't cost the reader the whole book: skip it and keep
			// assembling the rest instead of falling back to single-file handling.
			if convert_dtbook_file(
				&item.href,
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
			continue;
		}
		if !item.href.to_ascii_lowercase().ends_with(".smil") {
			continue;
		}
		let Ok(smil_content) = read_text(&item.href) else { continue };
		let smil_dir = dir_of(&item.href);
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
			if !file_offsets.contains_key(&text_href) {
				if convert_dtbook_file(
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
			}
			let (Some(&base), Some(&local_pos)) =
				(file_offsets.get(&text_href), file_ids.get(&text_href).and_then(|ids| ids.get(&par.text_id)))
			else {
				continue;
			};
			let position = base + local_pos;
			for anchor in &par.anchor_ids {
				smil_anchors.entry(anchor.clone()).or_insert(position);
				smil_anchors.entry(format!("{}#{anchor}", item.href)).or_insert(position);
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

	let mut toc_items = None;
	if let Some((_, ncx_item)) = package.items.iter().find(|(_, item)| {
		item.media_type == "application/x-dtbncx+xml" || item.href.to_ascii_lowercase().ends_with(".ncx")
	}) && let Ok(ncx_content) = read_text(&ncx_item.href)
		&& !ncx_content.is_empty()
		&& let Some(ncx_toc) = parse_daisy_ncx(&ncx_content, &dir_of(&ncx_item.href), &id_positions, &smil_anchors)
		&& !ncx_toc.is_empty()
	{
		toc_items = Some(ncx_toc);
	}
	let toc_items = toc_items.unwrap_or_else(|| build_toc_from_buffer(&buffer));

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
