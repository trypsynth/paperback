use std::time::{self, SystemTime};

use wxdragon::translations::translate as t;

use crate::session::StatusInfo;

pub fn format_status_text(info: &StatusInfo) -> String {
	let line_label = t("Line");
	let char_label = t("Character");
	let reading_label = t("Reading");
	format!(
		"{} {}, {} {}, {} {}%",
		line_label, info.line_number, char_label, info.character_number, reading_label, info.percentage
	)
}

pub fn calculate_sleep_timer_remaining(start_ms: i64, duration_minutes: i32) -> i32 {
	let now = SystemTime::now()
		.duration_since(time::UNIX_EPOCH)
		.ok()
		.and_then(|d| i64::try_from(d.as_millis()).ok())
		.unwrap_or(0);
	let elapsed_ms = now - start_ms;
	let duration_ms = i64::from(duration_minutes) * 60 * 1000;
	let remaining_ms = duration_ms - elapsed_ms;
	if remaining_ms < 0 { 0 } else { i32::try_from(remaining_ms / 1000).unwrap_or(i32::MAX) }
}

pub fn format_sleep_timer_status(base_status: &str, remaining_seconds: i32) -> String {
	let minutes = remaining_seconds / 60;
	let seconds = remaining_seconds % 60;
	let sleep_label = t("Sleep timer");
	format!("{base_status} | {sleep_label}: {minutes:02}:{seconds:02}")
}
