use std::time::{self, SystemTime};

use wxdragon::{prelude::*, translations::translate as t};

use super::document_manager::DocumentManager;
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

pub fn update_status_bar_with_sleep_timer(
	frame: &Frame,
	dm: &DocumentManager,
	sleep_timer_start_ms: i64,
	sleep_timer_duration_minutes: i32,
) {
	if dm.tab_count() == 0 {
		if sleep_timer_start_ms > 0 {
			let remaining = calculate_sleep_timer_remaining(sleep_timer_start_ms, sleep_timer_duration_minutes);
			if remaining > 0 {
				let status_text = format_sleep_timer_status(&t("Ready"), remaining);
				frame.set_status_text(&status_text, 0);
				return;
			}
		}
		frame.set_status_text(&t("Ready"), 0);
		return;
	}
	if let Some(tab) = dm.active_tab() {
		let position = tab.text_ctrl.get_insertion_point();
		let status_info = tab.session.get_status_info(position);
		let mut status_text = format_status_text(&status_info);
		if sleep_timer_start_ms > 0 {
			let remaining = calculate_sleep_timer_remaining(sleep_timer_start_ms, sleep_timer_duration_minutes);
			if remaining > 0 {
				status_text = format_sleep_timer_status(&status_text, remaining);
			}
		}
		frame.set_status_text(&status_text, 0);
	}
}
