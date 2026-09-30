use std::fs;

use super::*;
use crate::{
	document::{Marker, MarkerType, ParserContext},
	parser::{Parser, daisy::DaisyParser},
	util::test_support::TempDir,
};

/// A zip with nothing but audio files and no DAISY markup at all (e.g. an AudioVault-style
/// bundle) should still open: one textless section per audio file, in natural file-name
/// order, each playable end to end.
#[test]
fn plain_audio_zip_becomes_one_textless_section_per_file() {
	let zip_bytes = write_zip(&[
		("Track 2.mp3", b"fake-mp3-2"),
		("Track 10.mp3", b"fake-mp3-10"),
		("Track 1.mp3", b"fake-mp3-1"),
		("cover.jpg", b"not-audio"),
	]);
	let dir = TempDir::new("daisy_plain_audio_zip");
	let zip_path = dir.path().join("Some Audiobook.zip");
	fs::write(&zip_path, &zip_bytes).expect("write zip");
	let context = ParserContext::new(zip_path.to_string_lossy().to_string());
	let document = DaisyParser.parse(&context).expect("plain audio zip should parse");
	assert_eq!(document.toc_items.len(), 3, "the non-audio entry must not become a section");
	assert_eq!(
		document.toc_items.iter().map(|item| item.name.as_str()).collect::<Vec<_>>(),
		vec!["Track 1", "Track 2", "Track 10"]
	);
	assert_eq!(
		document.buffer.content, "\n\n\n",
		"the reading area must show one blank line per file, not literal space characters \
		 (a screen reader announces those character by character instead of treating them as blank)"
	);
	assert!(document.audio_only, "read-aloud UIs navigate this by elapsed time, not by text unit");
	// Each section must carry a SectionBreak marker, or Previous/Next Section navigation
	// (bound to [ and ]) finds nothing to jump to.
	let section_markers: Vec<&Marker> =
		document.buffer.markers.iter().filter(|m| m.mtype == MarkerType::SectionBreak).collect();
	assert_eq!(
		section_markers.iter().map(|m| m.position).collect::<Vec<_>>(),
		document.toc_items.iter().map(|item| item.offset).collect::<Vec<_>>()
	);
	// Each marker names its file, so stepping by section announces where the jump landed.
	// The buffer is nothing but blank lines, so a marker with no text of its own would leave
	// every section announcing nothing.
	assert_eq!(
		section_markers.iter().map(|m| m.text.as_str()).collect::<Vec<_>>(),
		vec!["Track 1", "Track 2", "Track 10"]
	);
	let audio = document.audio.expect("audio timeline should be populated");
	assert_eq!(audio.sources().len(), 3);
	assert_eq!(audio.clips().len(), 3);
	// Each section is independently seekable and switching sections switches files.
	let second_section_offset = document.toc_items[1].offset;
	let clip_index = audio.clip_index_at_position(second_section_offset).expect("section should have a clip");
	assert_eq!(audio.clip(clip_index).unwrap().source, 1);
	assert_eq!(audio.next_source_after(0), Some(1));
	assert_eq!(audio.next_source_after(1), Some(2));
	assert_eq!(audio.next_source_after(2), None);
}

/// A recognizable audio format's real duration is probed and used as the clip's length,
/// rather than the generous placeholder that stands in when probing isn't possible.
#[test]
fn plain_audio_zip_probes_real_duration_for_a_recognizable_audio_file() {
	let wav_bytes = make_wav(8000, 16_000); // 16,000 samples at 8kHz = 2.000s
	let zip_bytes = write_zip(&[("Track 1.wav", &wav_bytes)]);
	let dir = TempDir::new("daisy_plain_audio_zip_duration");
	let zip_path = dir.path().join("Some Audiobook.zip");
	fs::write(&zip_path, &zip_bytes).expect("write zip");
	let context = ParserContext::new(zip_path.to_string_lossy().to_string());
	let document = DaisyParser.parse(&context).expect("plain audio zip should parse");
	let audio = document.audio.expect("audio timeline should be populated");
	assert_eq!(audio.total_duration_ms(), 2000, "duration should come from the real WAV data, not a placeholder");
}

/// A file the probe can't parse as audio falls back to the placeholder duration instead of
/// failing the whole document over one bad entry.
#[test]
fn plain_audio_zip_falls_back_to_placeholder_duration_when_probing_fails() {
	let zip_bytes = write_zip(&[("Track 1.mp3", b"not-really-an-mp3")]);
	let dir = TempDir::new("daisy_plain_audio_zip_no_duration");
	let zip_path = dir.path().join("Some Audiobook.zip");
	fs::write(&zip_path, &zip_bytes).expect("write zip");
	let context = ParserContext::new(zip_path.to_string_lossy().to_string());
	let document = DaisyParser.parse(&context).expect("plain audio zip should parse");
	let audio = document.audio.expect("audio timeline should be populated");
	assert_eq!(
		audio.total_duration_ms(),
		24 * 60 * 60 * 1000,
		"unparseable audio should fall back to the 24h placeholder duration"
	);
}
