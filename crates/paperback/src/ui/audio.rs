use std::{rc::Rc, sync::Mutex};

use paperback_core::{
	audio::AudioTimeline,
	config::{ConfigManager, DocumentSpeech},
};
use patois::t;
use wxdragon::prelude::*;

use super::{dialogs, document_manager::DocumentManager, navigation::set_caret_to_doc_offset};
use crate::{
	audio_player::{self, AudioPlayer},
	ui::navigation::announce_for_command,
};

pub fn handle_toggle_play_pause_audio(
	doc_manager: &Rc<Mutex<DocumentManager>>,
	live_region_label: StaticText,
	from_keyboard: bool,
) {
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
	if has_audio {
		return;
	}
	// A document without recorded audio is read aloud instead.
	let result = dm.toggle_read_aloud();
	drop(dm);
	if let Err(message) = result {
		announce_for_command(live_region_label, from_keyboard, message);
	}
}

fn spilled_seek_target_ms(player: &AudioPlayer, amount_ms: u64) -> Option<u64> {
	let (source, raw_ms, length_ms) = player.current_file_position_and_length_ms()?;
	spill_overflow_into_next_source(player.timeline(), source, raw_ms, length_ms, amount_ms)
}

// Uses the decoder's real file length, not the clip duration: a plain-audio-zip bundle's placeholder clips run hours past the real file, so an elapsed-time target would land back in the same file.
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

pub fn handle_seek_audio(
	doc_manager: &Rc<Mutex<DocumentManager>>,
	config: &Rc<Mutex<ConfigManager>>,
	live_region_label: StaticText,
	forward: bool,
	from_keyboard: bool,
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
		// Without recorded audio, the seek keys move read aloud by a paragraph.
		let skipped = dm.skip_read_aloud(forward);
		drop(dm);
		let message = match skipped {
			Some(true) => return,
			// TRANSLATORS: Announced when skipping forward while read aloud is on the last paragraph
			Some(false) if forward => t("No next paragraph."),
			// TRANSLATORS: Announced when skipping back while read aloud is on the first paragraph
			Some(false) => t("No previous paragraph."),
			// TRANSLATORS: Announced when trying to seek audio on a document that has none
			None => t("This document has no audio."),
		};
		announce_for_command(live_region_label, from_keyboard, message);
		return;
	};
	let Some(current_ms) = player.resume_point_ms() else {
		drop(dm);
		// TRANSLATORS: Announced when trying to seek audio before playback has established a position
		announce_for_command(live_region_label, from_keyboard, t("Audio hasn't started playing yet."));
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
		.and_then(|cursor| player.timeline().clip(cursor.clip).map(|clip| clip.start));
	if let Some(position) = sync_position {
		set_caret_to_doc_offset(tab, position);
	}
}

pub fn handle_change_audio_speed(
	doc_manager: &Rc<Mutex<DocumentManager>>,
	config: &Rc<Mutex<ConfigManager>>,
	live_region_label: StaticText,
	increase: bool,
	from_keyboard: bool,
) {
	let mut dm = doc_manager.lock().unwrap();
	let result = dm.active_tab_mut().and_then(|tab| tab.audio_player.as_mut()).map(|player| {
		let before = player.speed();
		let after = if increase { player.increase_speed() } else { player.decrease_speed() };
		(after, (after - before).abs() < f32::EPSILON)
	});
	// One speed for every book, so the other open recordings change with it.
	if let Some((speed, _)) = result {
		dm.set_audio_speed(speed);
		config.lock().unwrap().set_app_string(audio_player::SPEED_KEY, &speed.to_string());
	}
	let Some((speed, at_limit)) = result else {
		// Without recorded audio, the speed keys change the speech rate.
		let changed = dm.change_read_aloud_rate(increase);
		drop(dm);
		let message = match changed {
			Ok((rate, at_limit)) => {
				// TRANSLATORS: Announced when the read aloud rate changes; {} is the rate from 0 to 100, where 50 is the voice's normal speed
				limit_message(&t("Rate {}").replace("{}", &rate.to_string()), at_limit, increase)
			}
			Err(message) if message.is_empty() => return,
			Err(message) => message,
		};
		announce_for_command(live_region_label, from_keyboard, message);
		return;
	};
	drop(dm);
	let label = speed_label(speed);
	let message = if at_limit && increase {
		// TRANSLATORS: Announced when the audio speed is already at its fastest; {} is the current speed, e.g. "2x"
		t("{} (maximum)").replace("{}", &label)
	} else if at_limit {
		// TRANSLATORS: Announced when the audio speed is already at its slowest; {} is the current speed, e.g. "0.5x"
		t("{} (minimum)").replace("{}", &label)
	} else {
		label
	};
	announce_for_command(live_region_label, from_keyboard, message);
}

/// Volume is per `AudioPlayer` rather than global because recordings differ in how loud they are.
pub fn handle_change_audio_volume(
	doc_manager: &Rc<Mutex<DocumentManager>>,
	live_region_label: StaticText,
	increase: bool,
	from_keyboard: bool,
) {
	let mut dm = doc_manager.lock().unwrap();
	let result = dm.active_tab_mut().and_then(|tab| tab.audio_player.as_mut()).map(|player| {
		let before = player.volume();
		let after = if increase { player.increase_volume() } else { player.decrease_volume() };
		(after, (after - before).abs() < f32::EPSILON)
	});
	// Without recorded audio, the volume keys change the volume of read aloud.
	let result = result.or_else(|| dm.change_read_aloud_volume(increase));
	drop(dm);
	let Some((volume, at_limit)) = result else {
		// TRANSLATORS: Announced when trying to change audio volume on a document that has none
		announce_for_command(live_region_label, from_keyboard, t("This document has no audio."));
		return;
	};
	let label = volume_label(volume);
	let message = if at_limit && increase {
		// TRANSLATORS: Announced when the audio volume is already at its loudest; {} is the current volume, e.g. "200%"
		t("{} (maximum)").replace("{}", &label)
	} else if at_limit {
		// TRANSLATORS: Announced when the audio volume is already at its quietest; {} is the current volume, e.g. "10%"
		t("{} (minimum)").replace("{}", &label)
	} else {
		label
	};
	announce_for_command(live_region_label, from_keyboard, message);
}

pub fn handle_speech_settings(
	frame: &Frame,
	doc_manager: &Rc<Mutex<DocumentManager>>,
	config: &Rc<Mutex<ConfigManager>>,
	live_region_label: StaticText,
	from_keyboard: bool,
) {
	let (engine, path) = {
		let mut dm = doc_manager.lock().unwrap();
		let path = dm.active_tab().map(|tab| tab.file_path.to_string_lossy().to_string());
		(dm.speech_engine(), path)
	};
	let engine = match engine {
		Ok(engine) => engine,
		Err(message) => {
			announce_for_command(live_region_label, from_keyboard, message);
			return;
		}
	};
	let keys =
		|engine: &str| (format!("tts_voice_{engine}"), format!("tts_rate_{engine}"), format!("tts_pitch_{engine}"));
	let initial = {
		let cfg = config.lock().unwrap();
		let (voice_key, rate_key, pitch_key) = keys(&engine);
		let book = path.as_deref().map(|path| cfg.get_document_speech(path)).unwrap_or_default();
		let app_voice = cfg.get_app_string(&voice_key, "");
		dialogs::SpeechValues {
			engine: engine.clone(),
			this_book_only: book.voice.is_some() || book.rate_percent.is_some() || book.pitch_percent.is_some(),
			voice: book.voice.clone().or_else(|| Some(app_voice).filter(|voice| !voice.is_empty())),
			rate: book.rate_percent.unwrap_or_else(|| cfg.get_app_int(&rate_key, 50)),
			pitch: book.pitch_percent.unwrap_or_else(|| cfg.get_app_int(&pitch_key, 50)),
			paragraph_pause_ms: cfg.get_app_int("tts_paragraph_pause_ms", 0),
		}
	};
	// Paused so the sample isn't spoken over the book.
	let was_playing = doc_manager.lock().unwrap().pause_read_aloud();
	let sample_dm = Rc::clone(doc_manager);
	let play_sample = move |engine: &str, voice: Option<String>, rate: i32, pitch: i32| {
		sample_dm.lock().unwrap().play_speech_sample(engine, voice, rate, pitch);
	};
	let voices_dm = Rc::clone(doc_manager);
	let voices_for = move |engine: &str| voices_dm.lock().unwrap().speech_voices(engine);
	let chosen = dialogs::show_speech_settings_dialog(frame, &initial, path.is_some(), voices_for, play_sample);
	doc_manager.lock().unwrap().stop_speech_sample();
	let Some(chosen) = chosen else {
		if was_playing {
			doc_manager.lock().unwrap().resume_read_aloud();
		}
		return;
	};
	{
		let cfg = config.lock().unwrap();
		let (voice_key, rate_key, pitch_key) = keys(&chosen.engine);
		cfg.set_app_string("tts_engine", &chosen.engine);
		cfg.set_app_int("tts_paragraph_pause_ms", chosen.paragraph_pause_ms);
		if let Some(path) = path.as_deref()
			&& chosen.this_book_only
		{
			let speech = DocumentSpeech {
				voice: chosen.voice,
				rate_percent: Some(chosen.rate),
				pitch_percent: Some(chosen.pitch),
			};
			cfg.set_document_speech(path, &speech);
		} else {
			cfg.set_app_string(&voice_key, chosen.voice.as_deref().unwrap_or_default());
			cfg.set_app_int(&rate_key, chosen.rate);
			cfg.set_app_int(&pitch_key, chosen.pitch);
			// Unchecking the box hands the book back to the app's settings.
			if let Some(path) = path.as_deref() {
				cfg.set_document_speech(path, &DocumentSpeech::default());
			}
		}
		cfg.flush();
	}
	let mut dm = doc_manager.lock().unwrap();
	// A different engine stops reading, so there's nothing left to carry on.
	if chosen.engine == engine {
		dm.restart_read_aloud();
		if was_playing {
			dm.resume_read_aloud();
		}
	} else {
		dm.set_speech_engine(&chosen.engine);
	}
}

/// `label`, marked as the limit when a step couldn't go any further.
fn limit_message(label: &str, at_limit: bool, increase: bool) -> String {
	match (at_limit, increase) {
		// TRANSLATORS: Announced when a setting is already at its highest; {} is its value
		(true, true) => t("{} (maximum)").replace("{}", label),
		// TRANSLATORS: Announced when a setting is already at its lowest; {} is its value
		(true, false) => t("{} (minimum)").replace("{}", label),
		_ => label.to_owned(),
	}
}

fn volume_label(volume: f32) -> String {
	format!("{}%", (volume * 100.0).round())
}

fn speed_label(speed: f32) -> String {
	let mut label = format!("{speed:.2}");
	while label.ends_with('0') {
		label.pop();
	}
	if label.ends_with('.') {
		label.pop();
	}
	format!("{label}x")
}

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

pub fn handle_change_seek_amount(
	config: &Rc<Mutex<ConfigManager>>,
	live_region_label: StaticText,
	increase: bool,
	from_keyboard: bool,
) {
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
	announce_for_command(live_region_label, from_keyboard, message);
}

#[cfg(test)]
mod tests {
	use paperback_core::{
		audio::{AudioLocation, AudioTimelineBuilder},
		document::DocPos,
	};

	use super::*;

	#[test]
	fn volume_reads_as_a_whole_percentage() {
		assert_eq!(volume_label(1.0), "100%");
		assert_eq!(volume_label(0.1), "10%");
		assert_eq!(volume_label(1.5), "150%");
	}

	/// 24h placeholder clips like `build_plain_audio_zip_document`'s, so only the real file length can trigger the spill.
	fn plain_audio_zip_timeline() -> AudioTimeline {
		const PLACEHOLDER_MS: u64 = 24 * 60 * 60 * 1000;
		let mut builder = AudioTimelineBuilder::new();
		let file1 = builder.add_source(AudioLocation::File("chapter1.mp3".to_string()), None);
		let file2 = builder.add_source(AudioLocation::File("chapter2.mp3".to_string()), None);
		builder.add_clip(file1, 0, PLACEHOLDER_MS, DocPos::from_raw(0), DocPos::from_raw(1));
		builder.add_clip(file2, 0, PLACEHOLDER_MS, DocPos::from_raw(1), DocPos::from_raw(2));
		builder.build()
	}

	#[test]
	fn spill_overflow_into_next_source_is_none_when_the_seek_stays_within_the_real_file() {
		let timeline = plain_audio_zip_timeline();
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
		let target = spill_overflow_into_next_source(&timeline, 0, 40_000, 45_000, 10_000);
		assert_eq!(target, timeline.elapsed_for_source_position(1, 5_000));
	}

	#[test]
	fn spill_overflow_into_next_source_is_none_past_the_last_file() {
		let timeline = plain_audio_zip_timeline();
		assert_eq!(spill_overflow_into_next_source(&timeline, 1, 40_000, 45_000, 10_000), None);
	}
}
