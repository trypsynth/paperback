use patois::t;
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
	if hours == 1 {
		// TRANSLATORS: Singular label for 1 hour duration
		parts.push(t("1 hour"));
	} else if hours > 1 {
		// TRANSLATORS: Plural label for hours duration (e.g. "2 hours")
		parts.push(format!("{} {}", hours, t("hours")));
	}
	if minutes == 1 {
		// TRANSLATORS: Singular label for 1 minute duration
		parts.push(t("1 minute"));
	} else if minutes > 1 {
		// TRANSLATORS: Plural label for minutes duration (e.g. "2 minutes")
		parts.push(format!("{} {}", minutes, t("minutes")));
	}
	if seconds == 1 {
		// TRANSLATORS: Singular label for 1 second duration
		parts.push(t("1 second"));
	} else if seconds > 1 || total_seconds == 0 {
		// TRANSLATORS: Plural label for seconds duration (e.g. "2 seconds")
		parts.push(format!("{} {}", seconds, t("seconds")));
	}
	let time_str = parts.join(", ");
	// TRANSLATORS: Prompt showing estimated reading time. The {} placeholder is replaced with a formatted duration like "1 hour, 5 minutes".
	let template = t("Estimated reading time: {}");
	template.replace("{}", &time_str)
}

pub fn show_word_count_dialog(parent: &Frame, word_count: usize, reading_speed_wpm: i32, is_selection: bool) {
	let words_template = if is_selection {
		// TRANSLATORS: Message template for selection word count. The {} placeholder is replaced with the number of words.
		t("The selection contains {} words.")
	} else {
		// TRANSLATORS: Message template for document word count. The {} placeholder is replaced with the number of words.
		t("This document contains {} words.")
	};
	let mut msg = words_template.replace("{}", &word_count.to_string());
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
