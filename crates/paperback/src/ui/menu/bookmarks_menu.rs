use paperback_core::config::{ActionId, ConfigManager};

use super::builder::MenuEntry;
use crate::ui::commands;

pub fn entries(config: &ConfigManager) -> Vec<MenuEntry> {
	let mut entries = commands::menu_entries(&[ActionId::ToggleBookmark, ActionId::BookmarkWithNote], config);
	entries.push(MenuEntry::Separator);
	entries.extend(commands::menu_entries(
		&[
			ActionId::PreviousBookmark,
			ActionId::NextBookmark,
			ActionId::PreviousNote,
			ActionId::NextNote,
			ActionId::ViewNoteText,
		],
		config,
	));
	entries.push(MenuEntry::Separator);
	entries.extend(commands::menu_entries(
		&[ActionId::JumpToAllBookmarks, ActionId::JumpToBookmarksOnly, ActionId::JumpToNotesOnly],
		config,
	));
	entries
}
