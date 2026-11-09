//! The Audio tab: playing audio books and narration.

use paperback_core::config::ConfigManager;
use patois::t;
use wxdragon::prelude::*;

use super::{OPTION_PADDING, labeled_row};

/// Indexed by the seek amount choice's selection and also stepped through by `audio::handle_change_seek_amount`, so it must match the choices in the tab.
pub(crate) const AUDIO_SEEK_AMOUNTS_SECONDS: [i32; 9] = [5, 10, 30, 60, 120, 300, 600, 1800, 3600];

const DEFAULT_SEEK_AMOUNT_SECONDS: i32 = 10;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AudioSettings {
	pub bookmark_sounds: bool,
	pub sync_caret_to_audio: bool,
	pub audio_seek_amount_seconds: i32,
	pub audio_seek_continues_into_next_file: bool,
}

impl AudioSettings {
	pub fn load(config: &ConfigManager) -> Self {
		Self {
			bookmark_sounds: config.get_app_bool("bookmark_sounds", true),
			sync_caret_to_audio: config.get_app_bool("sync_caret_to_audio", true),
			audio_seek_amount_seconds: config.get_app_int("audio_seek_amount_seconds", DEFAULT_SEEK_AMOUNT_SECONDS),
			audio_seek_continues_into_next_file: config.get_app_bool("audio_seek_continues_into_next_file", false),
		}
	}

	pub fn save(&self, config: &ConfigManager) {
		config.set_app_bool("bookmark_sounds", self.bookmark_sounds);
		config.set_app_bool("sync_caret_to_audio", self.sync_caret_to_audio);
		config.set_app_int("audio_seek_amount_seconds", self.audio_seek_amount_seconds);
		config.set_app_bool("audio_seek_continues_into_next_file", self.audio_seek_continues_into_next_file);
	}
}

pub(super) struct AudioTab {
	bookmark_sounds: CheckBox,
	sync_caret_to_audio: CheckBox,
	seek_amount: Choice,
	seek_continues_into_next_file: CheckBox,
}

impl AudioTab {
	pub(super) fn new(panel: Panel, settings: &AudioSettings, padding: i32) -> Self {
		let bookmark_sounds =
			// TRANSLATORS: Option to play sound effects when bookmarks or notes are encountered
			CheckBox::builder(&panel).with_label(&t("Play &sounds on bookmarks and notes")).build();
		let sync_caret_to_audio =
			// TRANSLATORS: Option to move the reading caret to follow along with audio narration as it plays
			CheckBox::builder(&panel).with_label(&t("&Sync caret to audio playback")).build();
		// TRANSLATORS: Label for the audio seek amount dropdown, used when skipping audio narration backward/forward
		let seek_amount_label_text = t("&Audio seek amount:");
		let seek_amount_label = StaticText::builder(&panel).with_label(&seek_amount_label_text).build();
		let seek_amount = Choice::builder(&panel).build();
		// TRANSLATORS: Audio seek amount option, shown in the audio seek amount dropdown
		seek_amount.append(&t("5 seconds"));
		// TRANSLATORS: Audio seek amount option, shown in the audio seek amount dropdown
		seek_amount.append(&t("10 seconds"));
		// TRANSLATORS: Audio seek amount option, shown in the audio seek amount dropdown
		seek_amount.append(&t("30 seconds"));
		// TRANSLATORS: Audio seek amount option, shown in the audio seek amount dropdown
		seek_amount.append(&t("1 minute"));
		// TRANSLATORS: Audio seek amount option, shown in the audio seek amount dropdown
		seek_amount.append(&t("2 minutes"));
		// TRANSLATORS: Audio seek amount option, shown in the audio seek amount dropdown
		seek_amount.append(&t("5 minutes"));
		// TRANSLATORS: Audio seek amount option, shown in the audio seek amount dropdown
		seek_amount.append(&t("10 minutes"));
		// TRANSLATORS: Audio seek amount option, shown in the audio seek amount dropdown
		seek_amount.append(&t("30 minutes"));
		// TRANSLATORS: Audio seek amount option, shown in the audio seek amount dropdown
		seek_amount.append(&t("1 hour"));
		#[cfg(target_os = "macos")]
		seek_amount.set_accessibility_label(seek_amount_label_text.replace('&', "").trim_end_matches(':').trim());
		let seek_continues_into_next_file =
			// TRANSLATORS: Option controlling whether seeking audio narration past the end of a chapter's file continues into the next chapter, or stops at the end of the current one
			CheckBox::builder(&panel).with_label(&t("Seeking &past the end of a chapter continues into the next")).build();
		let sizer = BoxSizer::builder(Orientation::Vertical).build();
		sizer.add(&bookmark_sounds, 0, SizerFlag::All, OPTION_PADDING);
		sizer.add(&sync_caret_to_audio, 0, SizerFlag::All, OPTION_PADDING);
		sizer.add_sizer(&labeled_row(seek_amount_label, &seek_amount, padding), 0, SizerFlag::All, OPTION_PADDING);
		sizer.add(&seek_continues_into_next_file, 0, SizerFlag::All, OPTION_PADDING);
		panel.set_sizer(sizer, true);
		bookmark_sounds.set_value(settings.bookmark_sounds);
		sync_caret_to_audio.set_value(settings.sync_caret_to_audio);
		let seek_index = AUDIO_SEEK_AMOUNTS_SECONDS
			.iter()
			.position(|&seconds| seconds == settings.audio_seek_amount_seconds)
			.or_else(|| AUDIO_SEEK_AMOUNTS_SECONDS.iter().position(|&seconds| seconds == DEFAULT_SEEK_AMOUNT_SECONDS))
			.unwrap_or(1);
		seek_amount.set_selection(u32::try_from(seek_index).unwrap_or(1));
		seek_continues_into_next_file.set_value(settings.audio_seek_continues_into_next_file);
		Self { bookmark_sounds, sync_caret_to_audio, seek_amount, seek_continues_into_next_file }
	}

	pub(super) fn values(&self) -> AudioSettings {
		AudioSettings {
			bookmark_sounds: self.bookmark_sounds.is_checked(),
			sync_caret_to_audio: self.sync_caret_to_audio.is_checked(),
			audio_seek_amount_seconds: self
				.seek_amount
				.get_selection()
				.and_then(|index| AUDIO_SEEK_AMOUNTS_SECONDS.get(index as usize).copied())
				.unwrap_or(DEFAULT_SEEK_AMOUNT_SECONDS),
			audio_seek_continues_into_next_file: self.seek_continues_into_next_file.is_checked(),
		}
	}
}
