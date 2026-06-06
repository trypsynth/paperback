use patois::t;
use wxdragon::prelude::*;

fn format_reading_time(word_count: usize, wpm: i32) -> String {
	if wpm <= 0 {
		return String::new();
	}
	let total_seconds = (word_count as f64 / wpm as f64 * 60.0).round() as u64;
	let hours = total_seconds / 3600;
	let minutes = (total_seconds % 3600) / 60;
	let seconds = total_seconds % 60;
	let mut parts: Vec<String> = Vec::new();
	if hours == 1 {
		parts.push(t("1 hour"));
	} else if hours > 1 {
		parts.push(format!("{} {}", hours, t("hours")));
	}
	if minutes == 1 {
		parts.push(t("1 minute"));
	} else if minutes > 1 {
		parts.push(format!("{} {}", minutes, t("minutes")));
	}
	if seconds == 1 {
		parts.push(t("1 second"));
	} else if seconds > 1 || total_seconds == 0 {
		parts.push(format!("{} {}", seconds, t("seconds")));
	}
	let time_str = parts.join(", ");
	let template = t("Estimated reading time: {}");
	template.replace("{}", &time_str)
}

pub fn show_word_count_dialog(parent: &Frame, word_count: usize, reading_speed_wpm: i32) {
	let words_template = t("This document contains {} words.");
	let mut msg = words_template.replace("{}", &word_count.to_string());
	let reading_time = format_reading_time(word_count, reading_speed_wpm);
	if !reading_time.is_empty() {
		msg.push('\n');
		msg.push_str(&reading_time);
	}
	let title = t("Word Count");
	let dialog = MessageDialog::builder(parent, &msg, &title).with_style(MessageDialogStyle::OK).build();
	dialog.show_modal();
}
