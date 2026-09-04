use patois::{nt, t};
use wxdragon::prelude::*;

use super::duration_format::format_duration_seconds;

fn format_reading_time(word_count: usize, wpm: i32) -> String {
	if wpm <= 0 {
		return String::new();
	}
	let total_seconds = (word_count as f64 / f64::from(wpm) * 60.0).round() as u64;
	let time_str = format_duration_seconds(total_seconds);
	// TRANSLATORS: Prompt showing estimated reading time. The {} placeholder is replaced with a formatted duration like "1 hour, 5 minutes".
	let template = t("Estimated reading time: {}");
	template.replace("{}", &time_str)
}

/// Total file count and playback duration (in milliseconds) for an audio-only document, so
/// [`show_word_count_dialog`] can report duration instead of a meaningless word count.
pub struct AudioOnlySummary {
	pub file_count: usize,
	pub total_duration_ms: u64,
}

pub fn show_word_count_dialog(
	parent: &Frame,
	word_count: usize,
	reading_speed_wpm: i32,
	is_selection: bool,
	audio_only: Option<AudioOnlySummary>,
) {
	// TRANSLATORS: Title of the Word Count dialog
	let title = t("Word Count");
	if let Some(summary) = audio_only {
		// TRANSLATORS: Message template for an audio-only document's file count. The %d placeholder is replaced with the number of audio files.
		let files_template = nt(
			"This document contains %d audio file.",
			"This document contains %d audio files.",
			summary.file_count as u64,
		);
		let mut msg = files_template.replacen("%d", &summary.file_count.to_string(), 1);
		msg.push('\n');
		// TRANSLATORS: Prompt showing an audio-only document's total playback duration across all its files. The {} placeholder is replaced with a formatted duration like "1 hour, 5 minutes".
		let duration_template = t("Total duration: {}");
		msg.push_str(&duration_template.replace("{}", &format_duration_seconds(summary.total_duration_ms / 1000)));
		let dialog = MessageDialog::builder(parent, &msg, &title).with_style(MessageDialogStyle::OK).build();
		dialog.show_modal();
		return;
	}
	let words_template = if is_selection {
		// TRANSLATORS: Message template for selection word count. The %d placeholder is replaced with the number of words.
		nt("The selection contains %d word.", "The selection contains %d words.", word_count as u64)
	} else {
		// TRANSLATORS: Message template for document word count. The %d placeholder is replaced with the number of words.
		nt("This document contains %d word.", "This document contains %d words.", word_count as u64)
	};
	let mut msg = words_template.replacen("%d", &word_count.to_string(), 1);
	let reading_time = format_reading_time(word_count, reading_speed_wpm);
	if !reading_time.is_empty() {
		msg.push('\n');
		msg.push_str(&reading_time);
	}
	let dialog = MessageDialog::builder(parent, &msg, &title).with_style(MessageDialogStyle::OK).build();
	dialog.show_modal();
}
