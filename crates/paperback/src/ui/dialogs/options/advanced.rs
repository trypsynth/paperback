//! The Advanced tab: options most people never need.

use logkeeper::Level;
use paperback_core::config::ConfigManager;
use patois::t;
use wxdragon::prelude::*;

use super::{OPTION_PADDING, labeled_row};
use crate::config_ext::{
	DEFAULT_LOG_LEVEL, UpdateChannel, get_log_level, get_update_channel, set_log_level, set_update_channel,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AdvancedSettings {
	pub update_channel: UpdateChannel,
	pub log_level: Level,
	pub plain_text: bool,
	pub edge_beep: bool,
}

impl AdvancedSettings {
	pub fn load(config: &ConfigManager) -> Self {
		Self {
			update_channel: get_update_channel(config),
			log_level: get_log_level(config),
			plain_text: config.get_app_bool("plain_text", false),
			edge_beep: config.get_app_bool("edge_beep", false),
		}
	}

	pub fn save(self, config: &ConfigManager) {
		set_update_channel(config, self.update_channel);
		set_log_level(config, self.log_level);
		config.set_app_bool("plain_text", self.plain_text);
		config.set_app_bool("edge_beep", self.edge_beep);
	}
}

pub(super) struct AdvancedTab {
	update_channel: Choice,
	log_level: Choice,
	plain_text: CheckBox,
	edge_beep: CheckBox,
}

impl AdvancedTab {
	pub(super) fn new(panel: Panel, settings: AdvancedSettings, padding: i32) -> Self {
		// TRANSLATORS: Label for the update channel selection dropdown
		let channel_label_text = t("Update Channel:");
		let channel_label = StaticText::builder(&panel).with_label(&channel_label_text).build();
		let update_channel = Choice::builder(&panel).build();
		// TRANSLATORS: Stable update channel option
		update_channel.append(&t("Stable"));
		// TRANSLATORS: Developer/development update channel option
		update_channel.append(&t("Dev"));
		#[cfg(target_os = "macos")]
		update_channel.set_accessibility_label(channel_label_text.replace('&', "").trim_end_matches(':').trim());
		// TRANSLATORS: Label for the dropdown choosing how much Paperback writes to its log file
		let log_level_label_text = t("Log Level:");
		let log_level_label = StaticText::builder(&panel).with_label(&log_level_label_text).build();
		let log_level = Choice::builder(&panel).build();
		for level in Level::ALL {
			log_level.append(&log_level_name(level));
		}
		#[cfg(target_os = "macos")]
		log_level.set_accessibility_label(log_level_label_text.replace('&', "").trim_end_matches(':').trim());
		let plain_text = CheckBox::builder(&panel)
			// TRANSLATORS: Option to load the reading view without bold, italic and underline styling, which can make a long document load faster. The reading font, colours and spacing still apply.
			.with_label(&t("Load text &without formatting"))
			.build();
		let edge_beep = CheckBox::builder(&panel)
			// TRANSLATORS: Option to play the system default beep when a movement key cannot move the caret at the very start or end of the document
			.with_label(&t("&Beep at the start and end of the document"))
			.build();
		let sizer = BoxSizer::builder(Orientation::Vertical).build();
		sizer.add_sizer(&labeled_row(channel_label, &update_channel, padding), 0, SizerFlag::All, OPTION_PADDING);
		sizer.add_sizer(&labeled_row(log_level_label, &log_level, padding), 0, SizerFlag::All, OPTION_PADDING);
		sizer.add(&plain_text, 0, SizerFlag::All, OPTION_PADDING);
		#[cfg(target_os = "windows")]
		sizer.add(&edge_beep, 0, SizerFlag::All, OPTION_PADDING);
		#[cfg(not(target_os = "windows"))]
		edge_beep.show(false);
		panel.set_sizer(sizer, true);
		update_channel.set_selection(match settings.update_channel {
			UpdateChannel::Stable => 0,
			UpdateChannel::Dev => 1,
		});
		let level_index = Level::ALL.iter().position(|level| *level == settings.log_level).unwrap_or(0);
		log_level.set_selection(u32::try_from(level_index).unwrap_or(0));
		plain_text.set_value(settings.plain_text);
		edge_beep.set_value(settings.edge_beep);
		Self { update_channel, log_level, plain_text, edge_beep }
	}

	pub(super) fn values(&self) -> AdvancedSettings {
		let update_channel = match self.update_channel.get_selection() {
			Some(1) => UpdateChannel::Dev,
			_ => UpdateChannel::Stable,
		};
		let chosen_level = self.log_level.get_selection().and_then(|index| Level::ALL.get(index as usize).copied());
		AdvancedSettings {
			update_channel,
			log_level: chosen_level.unwrap_or(DEFAULT_LOG_LEVEL),
			plain_text: self.plain_text.is_checked(),
			edge_beep: self.edge_beep.is_checked(),
		}
	}
}

fn log_level_name(level: Level) -> String {
	match level {
		// TRANSLATORS: Log level choice that turns logging off, so Paperback writes no log file at all
		Level::Off => t("Off"),
		// TRANSLATORS: Log level choice that records only errors
		Level::Error => t("Errors"),
		// TRANSLATORS: Log level choice that records warnings and errors
		Level::Warn => t("Warnings"),
		// TRANSLATORS: Log level choice that records general activity, warnings and errors
		Level::Info => t("Information"),
		// TRANSLATORS: Log level choice with the detail useful for reporting a bug; the default
		Level::Debug => t("Debug"),
		// TRANSLATORS: Log level choice that records everything Paperback can, which makes a very large log
		Level::Trace => t("Trace"),
	}
}
