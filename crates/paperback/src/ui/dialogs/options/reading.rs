//! The Reading tab: moving through and searching a document.

use paperback_core::config::ConfigManager;
use patois::t;
use wxdragon::prelude::*;

use super::{OPTION_PADDING, labeled_row};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReadingSettings {
	pub compact_go_menu: bool,
	pub navigation_wrap: bool,
	pub line_start_navigation: bool,
	pub highlight_found_text: bool,
	pub reading_speed_wpm: i32,
}

impl ReadingSettings {
	pub fn load(config: &ConfigManager) -> Self {
		Self {
			compact_go_menu: config.get_app_bool("compact_go_menu", true),
			navigation_wrap: config.get_app_bool("navigation_wrap", false),
			line_start_navigation: config.get_app_bool("line_start_navigation", false),
			highlight_found_text: config.get_app_bool("highlight_found_text", true),
			reading_speed_wpm: config.get_app_int("reading_speed_wpm", 150).clamp(1, 2000),
		}
	}

	pub fn save(&self, config: &ConfigManager) {
		config.set_app_bool("compact_go_menu", self.compact_go_menu);
		config.set_app_bool("navigation_wrap", self.navigation_wrap);
		config.set_app_bool("line_start_navigation", self.line_start_navigation);
		// Read at the next Find, so a match already selected isn't cleared out from under the reader as the dialog closes.
		config.set_app_bool("highlight_found_text", self.highlight_found_text);
		config.set_app_int("reading_speed_wpm", self.reading_speed_wpm);
	}
}

pub(super) struct ReadingTab {
	compact_go_menu: CheckBox,
	navigation_wrap: CheckBox,
	line_start_navigation: CheckBox,
	highlight_found_text: CheckBox,
	reading_speed: SpinCtrl,
}

impl ReadingTab {
	pub(super) fn new(panel: Panel, settings: &ReadingSettings, padding: i32) -> Self {
		// TRANSLATORS: Option to show a compact Go navigation menu in the menu bar
		let compact_go_menu = CheckBox::builder(&panel).with_label(&t("Show compact &go menu")).build();
		// TRANSLATORS: Option to wrap navigation around to the beginning/end when navigating elements
		let navigation_wrap = CheckBox::builder(&panel).with_label(&t("&Wrap navigation")).build();
		let line_start_navigation =
			// TRANSLATORS: Option to move the caret to the start of the line when navigating up or down
			CheckBox::builder(&panel).with_label(&t("Move to &start of line")).build();
		let highlight_found_text = CheckBox::builder(&panel)
			// TRANSLATORS: Option to select the text Find matches; turned off, Find only moves the caret to the match
			.with_label(&t("&Highlight found text"))
			.build();
		let reading_speed_label =
			// TRANSLATORS: Label for the reading speed input field (Words Per Minute)
			StaticText::builder(&panel).with_label(&t("&Reading speed (words per minute):")).build();
		let reading_speed = SpinCtrl::builder(&panel).with_range(1, 2000).build();
		let sizer = BoxSizer::builder(Orientation::Vertical).build();
		sizer.add(&compact_go_menu, 0, SizerFlag::All, OPTION_PADDING);
		sizer.add(&navigation_wrap, 0, SizerFlag::All, OPTION_PADDING);
		#[cfg(target_os = "windows")]
		sizer.add(&line_start_navigation, 0, SizerFlag::All, OPTION_PADDING);
		#[cfg(not(target_os = "windows"))]
		line_start_navigation.show(false);
		sizer.add(&highlight_found_text, 0, SizerFlag::All, OPTION_PADDING);
		sizer.add_sizer(&labeled_row(reading_speed_label, &reading_speed, padding), 0, SizerFlag::All, OPTION_PADDING);
		panel.set_sizer(sizer, true);
		compact_go_menu.set_value(settings.compact_go_menu);
		navigation_wrap.set_value(settings.navigation_wrap);
		line_start_navigation.set_value(settings.line_start_navigation);
		highlight_found_text.set_value(settings.highlight_found_text);
		reading_speed.set_value(settings.reading_speed_wpm);
		Self { compact_go_menu, navigation_wrap, line_start_navigation, highlight_found_text, reading_speed }
	}

	pub(super) fn values(&self) -> ReadingSettings {
		ReadingSettings {
			compact_go_menu: self.compact_go_menu.is_checked(),
			navigation_wrap: self.navigation_wrap.is_checked(),
			line_start_navigation: self.line_start_navigation.is_checked(),
			highlight_found_text: self.highlight_found_text.is_checked(),
			reading_speed_wpm: self.reading_speed.value(),
		}
	}
}
