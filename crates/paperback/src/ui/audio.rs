//! Playback commands: play/pause, seeking, and the seek amount.

use std::{rc::Rc, sync::Mutex};

use paperback_core::{audio::AudioTimeline, config::ConfigManager};
use patois::t;
use wxdragon::prelude::*;

use super::{dialogs, document_manager::DocumentManager, navigation::set_caret_to_doc_offset};
use crate::audio_player::AudioPlayer;

pub fn handle_toggle_play_pause_audio(doc_manager: &Rc<Mutex<DocumentManager>>, live_region_label: StaticText) {
	let mut dm = doc_manager.lock().unwrap();
	let has_audio = {
		let Some(tab) = dm.active_tab_mut() else { return };
		if let Some(player) = tab.audio_player.as_mut() {
			player.toggle();
			true
		} else {
			false
		}
	};
	drop(dm);
	if !has_audio {
		// TRANSLATORS: Announced when trying to play/pause audio on a document that has none
		live_region::announce(live_region_label, &t("This document has no audio."));
	}
}

/// Where seeking `amount_ms` forward from wherever `player` is now would land, if that runs
/// past the real end of the currently loaded file and "continue into the next file" is
/// enabled: the elapsed time in the document that continuing from the very start of whatever
/// comes next resolves to. `None` if the seek doesn't run off the file's real end, there's no
/// next source, or the next source's own narration hasn't started by that raw position yet
/// (an extremely long seek amount over a very short next file) - the caller falls back to the
/// ordinary clamped-at-`total_duration_ms` target in every one of those cases.
fn spilled_seek_target_ms(player: &AudioPlayer, amount_ms: u64) -> Option<u64> {
	let (source, raw_ms, length_ms) = player.current_file_position_and_length_ms()?;
	spill_overflow_into_next_source(&player.timeline(), source, raw_ms, length_ms, amount_ms)
}

/// The arithmetic behind `spilled_seek_target_ms`, split out so it's testable without a real
/// native media control backing `AudioPlayer`.
///
/// This has to go through the *real* decoder-reported file length (`length_ms`) rather than the
/// document's own declared clip duration, since a plain-audio-zip bundle's placeholder clip
/// duration (see `build_plain_audio_zip_document`) is hours longer than the real file, so the
/// ordinary elapsed-time-based target would just resolve back into the same file, past its real
/// end, where the native seek call clamps it to the file's own last frame instead of advancing.
fn spill_overflow_into_next_source(
	timeline: &AudioTimeline,
	source: usize,
	raw_ms: u64,
	length_ms: u64,
	amount_ms: u64,
) -> Option<u64> {
	let naive_in_file_ms = raw_ms.saturating_add(amount_ms);
	if naive_in_file_ms <= length_ms {
		return None;
	}
	let overflow_ms = naive_in_file_ms - length_ms;
	let next_source = timeline.next_source_after(source)?;
	timeline.elapsed_for_source_position(next_source, overflow_ms)
}

/// Skips the active document's audio narration backward or forward by the configured seek
/// amount (`audio_seek_amount_seconds`, default 10). When "sync caret to audio" is on, the
/// caret follows the new audio position, mirroring what `pump_audio` does during playback;
/// this is the one-shot equivalent for an explicit seek rather than passive following.
pub fn handle_seek_audio(
	doc_manager: &Rc<Mutex<DocumentManager>>,
	config: &Rc<Mutex<ConfigManager>>,
	live_region_label: StaticText,
	forward: bool,
) {
	let (sync_enabled, amount_seconds, spill_into_next_file) = {
		let cfg = config.lock().unwrap();
		(
			cfg.get_app_bool("sync_caret_to_audio", true),
			cfg.get_app_int("audio_seek_amount_seconds", 10),
			cfg.get_app_bool("audio_seek_continues_into_next_file", false),
		)
	};
	let amount_ms = u64::try_from(amount_seconds.max(1)).unwrap_or(10) * 1000;
	let mut dm = doc_manager.lock().unwrap();
	let Some(tab) = dm.active_tab_mut() else { return };
	let Some(player) = tab.audio_player.as_mut() else {
		drop(dm);
		// TRANSLATORS: Announced when trying to seek audio on a document that has none
		live_region::announce(live_region_label, &t("This document has no audio."));
		return;
	};
	let Some(current_ms) = player.resume_point_ms() else {
		drop(dm);
		// TRANSLATORS: Announced when trying to seek audio before playback has established a position
		live_region::announce(live_region_label, &t("Audio hasn't started playing yet."));
		return;
	};
	let total_ms = player.timeline().total_duration_ms();
	let target_ms = if forward {
		let clamped_target_ms = current_ms.saturating_add(amount_ms).min(total_ms);
		if spill_into_next_file {
			spilled_seek_target_ms(player, amount_ms).unwrap_or(clamped_target_ms)
		} else {
			clamped_target_ms
		}
	} else {
		current_ms.saturating_sub(amount_ms)
	};
	player.seek_to_ms(target_ms);
	let sync_position = sync_enabled
		.then(|| player.timeline().cursor_at_elapsed(target_ms))
		.flatten()
		.and_then(|cursor| player.timeline().clip(cursor.clip).map(|clip| i64::try_from(clip.start).unwrap_or(0)));
	if let Some(position) = sync_position {
		set_caret_to_doc_offset(tab, position);
	}
}

/// A human-readable label for one of `dialogs::AUDIO_SEEK_AMOUNTS_SECONDS`, matching the text
/// shown for it in the Options dialog's seek-amount dropdown, for the live-region announcement
/// made when the amount changes via keyboard shortcut.
fn seek_amount_label(seconds: i32) -> String {
	match seconds {
		// TRANSLATORS: Audio seek amount, announced after changing it via keyboard shortcut
		5 => t("5 seconds"),
		// TRANSLATORS: Audio seek amount, announced after changing it via keyboard shortcut
		10 => t("10 seconds"),
		// TRANSLATORS: Audio seek amount, announced after changing it via keyboard shortcut
		30 => t("30 seconds"),
		// TRANSLATORS: Audio seek amount, announced after changing it via keyboard shortcut
		60 => t("1 minute"),
		// TRANSLATORS: Audio seek amount, announced after changing it via keyboard shortcut
		120 => t("2 minutes"),
		// TRANSLATORS: Audio seek amount, announced after changing it via keyboard shortcut
		300 => t("5 minutes"),
		// TRANSLATORS: Audio seek amount, announced after changing it via keyboard shortcut
		600 => t("10 minutes"),
		// TRANSLATORS: Audio seek amount, announced after changing it via keyboard shortcut
		1800 => t("30 minutes"),
		// TRANSLATORS: Audio seek amount, announced after changing it via keyboard shortcut
		3600 => t("1 hour"),
		other => format!("{other}s"),
	}
}

/// Nudges the configured audio seek amount (used by `handle_seek_audio`) one step up or down
/// through the same preset list shown in the Options dialog's dropdown, and announces the new
/// value. A global setting rather than a per-document action, so unlike `handle_seek_audio` this
/// doesn't need an active document or audio player.
pub fn handle_change_seek_amount(config: &Rc<Mutex<ConfigManager>>, live_region_label: StaticText, increase: bool) {
	let presets = dialogs::AUDIO_SEEK_AMOUNTS_SECONDS;
	let cfg = config.lock().unwrap();
	let current = cfg.get_app_int("audio_seek_amount_seconds", 10);
	let index = presets.iter().position(|&secs| secs == current).unwrap_or_else(|| {
		presets
			.iter()
			.enumerate()
			.min_by_key(|&(_, &secs)| (secs - current).abs())
			.map_or(0, |(nearest_index, _)| nearest_index)
	});
	let new_index = if increase { (index + 1).min(presets.len() - 1) } else { index.saturating_sub(1) };
	let new_value = presets[new_index];
	let at_limit = new_index == index;
	cfg.set_app_int("audio_seek_amount_seconds", new_value);
	cfg.flush();
	drop(cfg);
	let label = seek_amount_label(new_value);
	let message = if at_limit && increase {
		// TRANSLATORS: Announced when the audio seek amount is already at its largest preset; {} is the current amount, e.g. "1 hour"
		t("{} (maximum)").replace("{}", &label)
	} else if at_limit {
		// TRANSLATORS: Announced when the audio seek amount is already at its smallest preset; {} is the current amount, e.g. "5 seconds"
		t("{} (minimum)").replace("{}", &label)
	} else {
		label
	};
	live_region::announce(live_region_label, &message);
}

#[cfg(test)]
mod tests {
	use paperback_core::audio::{AudioLocation, AudioTimelineBuilder};

	use super::*;

	/// Two "files", each one placeholder-duration clip covering its whole 24h declared length
	/// (like `build_plain_audio_zip_document`'s clips), so a real file length far shorter than
	/// that placeholder is what has to trigger the spill, not the document's own clip bounds.
	fn plain_audio_zip_timeline() -> AudioTimeline {
		const PLACEHOLDER_MS: u64 = 24 * 60 * 60 * 1000;
		let mut builder = AudioTimelineBuilder::new();
		let file1 = builder.add_source(AudioLocation::File("chapter1.mp3".to_string()), None);
		let file2 = builder.add_source(AudioLocation::File("chapter2.mp3".to_string()), None);
		builder.add_clip(file1, 0, PLACEHOLDER_MS, 0, 1);
		builder.add_clip(file2, 0, PLACEHOLDER_MS, 1, 2);
		builder.build()
	}

	#[test]
	fn spill_overflow_into_next_source_is_none_when_the_seek_stays_within_the_real_file() {
		let timeline = plain_audio_zip_timeline();
		// 30s into a 45s file, seeking 10s: lands at 40s, still short of the real 45s end.
		assert_eq!(spill_overflow_into_next_source(&timeline, 0, 30_000, 45_000, 10_000), None);
	}

	#[test]
	fn spill_overflow_into_next_source_is_none_exactly_at_the_real_end() {
		let timeline = plain_audio_zip_timeline();
		assert_eq!(spill_overflow_into_next_source(&timeline, 0, 35_000, 45_000, 10_000), None);
	}

	#[test]
	fn spill_overflow_into_next_source_lands_the_overflow_into_the_next_files_start() {
		let timeline = plain_audio_zip_timeline();
		// 40s into a 45s file, seeking 10s: 5s of that seek belongs to whatever plays next.
		let target = spill_overflow_into_next_source(&timeline, 0, 40_000, 45_000, 10_000);
		assert_eq!(target, timeline.elapsed_for_source_position(1, 5_000));
	}

	#[test]
	fn spill_overflow_into_next_source_is_none_past_the_last_file() {
		let timeline = plain_audio_zip_timeline();
		assert_eq!(spill_overflow_into_next_source(&timeline, 1, 40_000, 45_000, 10_000), None);
	}
}
