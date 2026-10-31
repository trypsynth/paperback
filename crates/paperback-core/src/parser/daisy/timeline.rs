use std::collections::HashMap;

use anyhow::Result;

use super::{
	ncx::{ncx_entries, parse_daisy_ncx},
	opf::{ManifestItem, OpfPackage, dir_of, is_dtbook_like_item},
	smil::{parse_smil_audio_pars, parse_smil_pars},
};
use crate::{
	audio::{AudioLocation, AudioTimelineBuilder},
	document::{DocLen, DocPos, Document, DocumentBuffer, Marker, MarkerType},
	parser::{
		add_converter_markers,
		convert::xml_to_text::XmlToText,
		util::{
			path::resolve_relative_path,
			toc::{build_toc_from_buffer, heading_level_to_marker_type},
		},
	},
	t,
};

fn convert_dtbook_file(
	href: &str,
	read_text: &mut dyn FnMut(&str) -> Result<String>,
	render_tables_inline: bool,
	buffer: &mut DocumentBuffer,
	id_positions: &mut HashMap<String, DocPos>,
	file_offsets: &mut HashMap<String, DocPos>,
	file_ids: &mut HashMap<String, HashMap<String, DocPos>>,
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
	let offset = buffer.end();
	buffer.append(converter.get_text());
	add_converter_markers(buffer, &converter, offset);
	for pb in converter.get_page_breaks() {
		buffer
			.add_marker(Marker::new(MarkerType::PageBreak, offset + pb.offset.from_start()).with_text(pb.text.clone()));
	}
	for (id, pos) in converter.get_id_positions() {
		// Bare ids are only unique within their own file, so keep the first file's position and expose a path-qualified key alongside it.
		let absolute = offset + pos.from_start();
		id_positions.entry(id.clone()).or_insert(absolute);
		id_positions.insert(format!("{href}#{id}"), absolute);
	}
	file_ids.insert(href.to_string(), converter.get_id_positions().clone());
	file_offsets.insert(href.to_string(), offset);
	Ok(())
}

pub(super) struct PendingClip {
	pub(super) source: usize,
	pub(super) begin_ms: u64,
	pub(super) end_ms: Option<u64>,
	pub(super) start: DocPos,
}

pub(super) struct ResolvedClip {
	pub(super) source: usize,
	pub(super) begin_ms: u64,
	pub(super) end_ms: u64,
	pub(super) start: DocPos,
}

/// An open-ended clip runs to whatever plays next from the same file anywhere in the book, not just its own SMIL; a trailing one would need the file's real duration, which the parser doesn't decode, so it is dropped.
pub(super) fn bound_open_ended_clips(pending: &[PendingClip]) -> Vec<ResolvedClip> {
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

/// `None` when the spine names nothing convertible, so the caller can fall back to single-file handling.
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
	let mut id_positions: HashMap<String, DocPos> = HashMap::new();
	let mut file_offsets: HashMap<String, DocPos> = HashMap::new();
	let mut file_ids: HashMap<String, HashMap<String, DocPos>> = HashMap::new();
	let mut source_indices: HashMap<String, usize> = HashMap::new();
	let mut audio_builder = AudioTimelineBuilder::new();
	let mut pending_clips: Vec<PendingClip> = Vec::new();
	// A DAISY 3 NCX points at ids inside the SMIL files, not the text; keyed bare and path-qualified, first occurrence winning.
	let mut smil_anchors: HashMap<String, DocPos> = HashMap::new();
	let mut converted_any = false;
	for idref in &package.spine {
		let Some(item) = package.item(idref) else { continue };
		if is_dtbook_like_item(item) {
			// One malformed chapter shouldn't cost the reader the whole book, so skip it rather than fall back to single-file handling.
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
		// A `<text src>` naming only a fragment resolves against the file the most recent explicit reference in this same SMIL named.
		let mut current_text_href: Option<String> = None;
		for par in parse_smil_pars(&smil_content) {
			let text_href = if let Some(text_file) = &par.text_file {
				let href = resolve_relative_path(&smil_dir, text_file);
				current_text_href = Some(href.clone());
				href
			} else {
				let Some(href) = current_text_href.clone() else { continue };
				href
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
			let position = base + local_pos.from_start();
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
		return build_audio_only_document(package, title, author, read_text, resolve_audio);
	}
	let toc_items = if let Some((_, ncx_item)) = package.items.iter().find(|(_, item)| is_ncx_item(item))
		&& let Ok(ncx_content) = read_text(&ncx_item.href)
		&& !ncx_content.is_empty()
		&& let Some(ncx_toc) = parse_daisy_ncx(&ncx_content, &dir_of(&ncx_item.href), &id_positions, &smil_anchors)
		&& !ncx_toc.is_empty()
	{
		ncx_toc
	} else {
		build_toc_from_buffer(&buffer)
	};
	let resolved_clips = bound_open_ended_clips(&pending_clips);
	let doc_end = buffer.end();
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

/// An audio-only DAISY 3 book has no text to play against, so each NCX label stands in for its section, as an audio-only DAISY 2.02 book's `ncc.html` headings do.
fn build_audio_only_document(
	package: &OpfPackage,
	title: String,
	author: String,
	read_text: &mut dyn FnMut(&str) -> Result<String>,
	resolve_audio: &dyn Fn(&str) -> AudioLocation,
) -> Option<Document> {
	let ncx_item = package.items.iter().map(|(_, item)| item).find(|item| is_ncx_item(item))?;
	let ncx_content = read_text(&ncx_item.href).ok()?;
	let ncx_dir = dir_of(&ncx_item.href);
	let mut buffer = DocumentBuffer::new();
	let mut smil_anchors: HashMap<String, DocPos> = HashMap::new();
	for (level, label, src) in ncx_entries(&ncx_content) {
		let start = buffer.end();
		buffer.append(&label);
		buffer.append("\n");
		buffer.add_marker(
			Marker::new(heading_level_to_marker_type(level), start)
				.with_text(label.clone())
				.with_level(level)
				.with_length(DocLen::of(&label)),
		);
		if let Some((file, fragment)) = src.split_once('#') {
			smil_anchors.entry(format!("{}#{fragment}", resolve_relative_path(&ncx_dir, file))).or_insert(start);
			smil_anchors.entry(fragment.to_string()).or_insert(start);
		}
	}
	let mut audio_builder = AudioTimelineBuilder::new();
	let mut source_indices: HashMap<String, usize> = HashMap::new();
	let mut pending_clips: Vec<PendingClip> = Vec::new();
	let mut position = DocPos::ZERO;
	for idref in &package.spine {
		let Some(item) = package.item(idref) else { continue };
		if !item.href.to_ascii_lowercase().ends_with(".smil") {
			continue;
		}
		let Ok(smil_content) = read_text(&item.href) else { continue };
		let smil_dir = dir_of(&item.href);
		for par in parse_smil_audio_pars(&smil_content) {
			let heading = par.anchor_ids.iter().find_map(|anchor| {
				smil_anchors.get(&format!("{}#{anchor}", item.href)).or_else(|| smil_anchors.get(anchor)).copied()
			});
			let audio_href = resolve_relative_path(&smil_dir, &par.audio_src);
			let source = *source_indices
				.entry(audio_href.clone())
				.or_insert_with(|| audio_builder.add_source(resolve_audio(&audio_href), None));
			// A section's phrases all start at its one heading line, so a run of them from one file plays as one clip.
			if let Some(last) = pending_clips.last_mut()
				&& heading.is_none_or(|heading| heading == position)
				&& last.start == position
				&& last.source == source
				&& last.end_ms.is_some_and(|end| end <= par.clip_begin_ms)
			{
				last.end_ms = par.clip_end_ms;
				continue;
			}
			position = heading.unwrap_or(position);
			pending_clips.push(PendingClip {
				source,
				begin_ms: par.clip_begin_ms,
				end_ms: par.clip_end_ms,
				start: position,
			});
		}
	}
	if pending_clips.is_empty() {
		return None;
	}
	let resolved_clips = bound_open_ended_clips(&pending_clips);
	let doc_end = buffer.end();
	for index in 0..resolved_clips.len() {
		let clip = &resolved_clips[index];
		let end = resolved_clips.get(index + 1).map_or(doc_end, |next| next.start);
		audio_builder.add_clip(clip.source, clip.begin_ms, clip.end_ms, clip.start, end);
	}
	let toc_items = parse_daisy_ncx(&ncx_content, &ncx_dir, &HashMap::new(), &smil_anchors)
		.unwrap_or_else(|| build_toc_from_buffer(&buffer));
	let audio = audio_builder.build();
	Some(Document {
		title,
		author,
		buffer,
		toc_items,
		audio: (!audio.is_empty()).then_some(audio),
		..Document::default()
	})
}

fn is_ncx_item(item: &ManifestItem) -> bool {
	item.media_type == "application/x-dtbncx+xml" || item.href.to_ascii_lowercase().ends_with(".ncx")
}
