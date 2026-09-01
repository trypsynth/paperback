use std::{
	cmp::Ordering,
	io::{Cursor, Read, Seek},
	iter::Peekable,
	path::Path,
	str::Chars,
};

use lofty::prelude::*;
use zip::ZipArchive;

use crate::{
	audio::{AudioLocation, AudioTimelineBuilder},
	document::{Document, DocumentBuffer, Marker, MarkerType, TocItem},
	util::zip::read_zip_entry_bytes_with_password,
};

const PLAIN_AUDIO_EXTENSIONS: &[&str] =
	&["mp3", "m4a", "m4b", "aac", "ogg", "oga", "opus", "wav", "flac", "wma", "aif", "aiff"];

fn is_plain_audio_entry(name: &str) -> bool {
	Path::new(name)
		.extension()
		.and_then(|ext| ext.to_str())
		.is_some_and(|ext| PLAIN_AUDIO_EXTENSIONS.iter().any(|candidate| candidate.eq_ignore_ascii_case(ext)))
}

/// Orders file names the way a person would: numeric runs compare by value, so "track2" sorts
/// before "track10" instead of after it, which plain byte-wise sorting would get wrong for the
/// common case of unpadded track numbers.
fn natural_cmp(a: &str, b: &str) -> Ordering {
	let mut a = a.chars().peekable();
	let mut b = b.chars().peekable();
	loop {
		return match (a.peek().copied(), b.peek().copied()) {
			(None, None) => Ordering::Equal,
			(None, Some(_)) => Ordering::Less,
			(Some(_), None) => Ordering::Greater,
			(Some(ca), Some(cb)) if ca.is_ascii_digit() && cb.is_ascii_digit() => {
				match take_number(&mut a).cmp(&take_number(&mut b)) {
					Ordering::Equal => continue,
					other => other,
				}
			}
			(Some(ca), Some(cb)) => match ca.to_ascii_lowercase().cmp(&cb.to_ascii_lowercase()) {
				Ordering::Equal => {
					a.next();
					b.next();
					continue;
				}
				other => other,
			},
		};
	}
}

fn take_number(chars: &mut Peekable<Chars>) -> u64 {
	let mut value: u64 = 0;
	while let Some(c) = chars.peek().copied().filter(char::is_ascii_digit) {
		value = value.saturating_mul(10).saturating_add(u64::from(c as u8 - b'0'));
		chars.next();
	}
	value
}

/// Reads back the real duration of a zip-embedded audio entry via a lightweight container/tag
/// probe (no full decode). `None` when the format isn't one `lofty` recognizes (e.g. WMA) or the
/// entry doesn't parse as audio at all; callers fall back to a placeholder in that case rather
/// than fail the whole document over one bad or unsupported entry.
fn probe_duration_ms<R: Read + Seek>(archive: &mut ZipArchive<R>, entry: &str, password: Option<&str>) -> Option<u64> {
	let bytes = read_zip_entry_bytes_with_password(archive, entry, password).ok()?;
	let tagged_file = lofty::probe::Probe::new(Cursor::new(bytes)).guess_file_type().ok()?.read().ok()?;
	u64::try_from(tagged_file.properties().duration().as_millis()).ok()
}

/// Builds a document out of a zip that is nothing but a bundle of audio files: no OPF, no NCC,
/// no markup relating them to any text. Each audio file becomes its own textless section, named
/// after the file and ordered naturally by file name. Playback has no per-sentence granularity to
/// offer, so each section is a single clip spanning its whole source; seeking within a section and
/// crossing between sections (see `AudioTimeline::next_source_after` and
/// `previous_source_before`) are the only ways to move around, which is why the document is
/// marked `audio_only`: there is no text spine for a reading unit to step through, so read-aloud
/// UIs navigate it by elapsed time instead.
///
/// Each clip's end time is the file's real duration, probed via `probe_duration_ms`, falling back
/// to a generous placeholder when probing fails. `AudioTimeline`'s bookkeeping only strictly needs
/// a source's clip to end after every other clip against the same source begins (there is only
/// one), so a placeholder still keeps players working; the cost of falling back is that elapsed
/// time can't say where that one file ends and the next begins, so a seek running off either end
/// of it resolves against the file's own real length, which only a prepared decoder knows.
pub(super) fn build_plain_audio_zip_document<R: Read + Seek>(
	archive: &mut ZipArchive<R>,
	archive_path: &str,
	title: String,
	author: String,
	password: Option<&str>,
) -> Option<Document> {
	const PLACEHOLDER_CLIP_DURATION_MS: u64 = 24 * 60 * 60 * 1000;
	// zip 9 hands back a Result per name, since decoding one can fail. A name that will not
	// decode cannot match what this scan is looking for, so drop those rather than fail the file.
	let mut entries: Vec<String> =
		archive.file_names().flatten().filter(|name| is_plain_audio_entry(name)).map(String::from).collect();
	if entries.is_empty() {
		return None;
	}
	entries.sort_by(|a, b| natural_cmp(a, b));
	let mut buffer = DocumentBuffer::new();
	let mut toc_items = Vec::with_capacity(entries.len());
	let mut audio_builder = AudioTimelineBuilder::new();
	for entry in &entries {
		let name =
			Path::new(entry).file_stem().map_or_else(|| entry.clone(), |stem| stem.to_string_lossy().to_string());
		let position = buffer.current_position();
		// A bare newline, not a space: this is what the reading area shows for an audio-only
		// document, and a genuine blank line reads and navigates like any other blank line
		// elsewhere in the app, unlike a run of space characters (which screen readers announce
		// character by character, once per file).
		buffer.append("\n");
		// The marker carries the file name so section navigation has something to announce.
		// Without it the name lives only in the TOC, and `fill_marker_text_if_empty` falls back
		// to the line enclosing the marker, which here is always empty.
		buffer.add_marker(Marker::new(MarkerType::SectionBreak, position).with_text(name.clone()));
		let probed_ms = probe_duration_ms(archive, entry, password);
		let clip_duration_ms = probed_ms.unwrap_or(PLACEHOLDER_CLIP_DURATION_MS);
		let source = audio_builder.add_source(
			AudioLocation::ZipEntry {
				archive: archive_path.to_string(),
				entry: entry.clone(),
				password: password.map(str::to_string),
			},
			probed_ms,
		);
		audio_builder.add_clip(source, 0, clip_duration_ms, position, position + 1);
		toc_items.push(TocItem::new(name, entry.clone(), position));
	}
	let audio = audio_builder.build();
	Some(Document { title, author, buffer, toc_items, audio: Some(audio), audio_only: true, ..Document::default() })
}
