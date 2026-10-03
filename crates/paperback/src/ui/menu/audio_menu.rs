use paperback_core::config::{ActionId, ConfigManager};

use super::builder::MenuEntry;
use crate::ui::commands;

pub fn entries(config: &ConfigManager) -> Vec<MenuEntry> {
	let mut entries = commands::menu_entries(&[ActionId::PlayPauseAudio], config);
	entries.push(MenuEntry::Separator);
	entries.extend(commands::menu_entries(
		&[
			ActionId::SeekAudioForward,
			ActionId::SeekAudioBackward,
			ActionId::IncreaseAudioSeekAmount,
			ActionId::DecreaseAudioSeekAmount,
		],
		config,
	));
	entries.push(MenuEntry::Separator);
	entries.extend(commands::menu_entries(&[ActionId::IncreaseAudioSpeed, ActionId::DecreaseAudioSpeed], config));
	entries
}
