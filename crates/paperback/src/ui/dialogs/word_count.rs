use patois::{nt, t};
use wxdragon::prelude::*;

fn format_reading_time(word_count: usize, wpm: i32) -> String {
	if wpm <= 0 {
		return String::new();
	}
	let total_seconds = (word_count as f64 / f64::from(wpm) * 60.0).round() as u64;
	let hours = total_seconds / 3600;
	let minutes = (total_seconds % 3600) / 60;
	let seconds = total_seconds % 60;
	let mut parts: Vec<String> = Vec::new();
	if hours >= 1 {
		// TRANSLATORS: Duration segment for hours in the estimated reading time (e.g. "1 hour" / "5 hours"). The %d placeholder is replaced with the count.
		parts.push(nt("%d hour", "%d hours", hours).replacen("%d", &hours.to_string(), 1));
	}
	if minutes >= 1 {
		// TRANSLATORS: Duration segment for minutes in the estimated reading time (e.g. "1 minute" / "5 minutes"). The %d placeholder is replaced with the count.
		parts.push(nt("%d minute", "%d minutes", minutes).replacen("%d", &minutes.to_string(), 1));
	}
	if seconds >= 1 || total_seconds == 0 {
		// TRANSLATORS: Duration segment for seconds in the estimated reading time (e.g. "1 second" / "5 seconds"). The %d placeholder is replaced with the count.
		parts.push(nt("%d second", "%d seconds", seconds).replacen("%d", &seconds.to_string(), 1));
	}
	let time_str = parts.join(", ");
	// TRANSLATORS: Prompt showing estimated reading time. The {} placeholder is replaced with a formatted duration like "1 hour, 5 minutes".
	let template = t("Estimated reading time: {}");
	template.replace("{}", &time_str)
}

pub fn show_word_count_dialog(parent: &Frame, word_count: usize, reading_speed_wpm: i32, is_selection: bool) {
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
	// TRANSLATORS: Title of the Word Count dialog
	let title = t("Word Count");
	let dialog = MessageDialog::builder(parent, &msg, &title).with_style(MessageDialogStyle::OK).build();
	dialog.show_modal();
}
