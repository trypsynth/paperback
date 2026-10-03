use paperback_core::config::{ActionId, ConfigManager, ShortcutCategory};
use patois::t;
use wxdragon::prelude::*;

use crate::ui::commands::{self, Enable};

mod audio_menu;
mod bookmarks_menu;
mod builder;
#[cfg(target_os = "macos")]
mod edit_menu;
mod file_menu;
mod go_menu;
mod help_menu;
mod state;
mod tools_menu;

use builder::build_menu;
pub use builder::{MenuEntry, format_menu_label, item_with_help};
pub use file_menu::recent_documents_for_menu;
pub use state::{update_menu_item_states, update_reopen_state};

/// One menu of the menu bar, holding the commands of one shortcut category.
pub struct TopMenu {
	pub category: ShortcutCategory,
	pub entries: Vec<MenuEntry>,
}

/// The menu bar's menus, in order.
pub fn menus(config: &ConfigManager, compact_go: bool) -> Vec<TopMenu> {
	vec![
		TopMenu { category: ShortcutCategory::File, entries: file_menu::entries(config) },
		TopMenu { category: ShortcutCategory::Go, entries: go_menu::entries(config, compact_go) },
		TopMenu { category: ShortcutCategory::Bookmarks, entries: bookmarks_menu::entries(config) },
		TopMenu { category: ShortcutCategory::Audio, entries: audio_menu::entries(config) },
		TopMenu { category: ShortcutCategory::Tools, entries: tools_menu::entries(config) },
		TopMenu { category: ShortcutCategory::Help, entries: help_menu::entries(config) },
	]
}

/// The menu bar label of the menu holding `category`'s commands.
fn title(category: ShortcutCategory) -> String {
	match category {
		// TRANSLATORS: Top-level "File" menu label in the menu bar
		ShortcutCategory::File => t("&File"),
		// TRANSLATORS: Top-level "Go" menu label in the menu bar
		ShortcutCategory::Go => t("&Go"),
		// TRANSLATORS: Top-level "Bookmarks" menu label in the menu bar
		ShortcutCategory::Bookmarks => t("&Bookmarks"),
		// TRANSLATORS: Top-level "Audio" menu label in the menu bar
		ShortcutCategory::Audio => t("&Audio"),
		// TRANSLATORS: Top-level "Tools" menu label in the menu bar
		ShortcutCategory::Tools => t("&Tools"),
		// TRANSLATORS: Top-level "Help" menu label in the menu bar
		ShortcutCategory::Help => t("&Help"),
	}
}

/// Commands reachable only through their keyboard shortcut.
pub const fn is_keyboard_only(action: ActionId) -> bool {
	matches!(action, ActionId::AnnouncePercent | ActionId::SetTemporaryBookmark | ActionId::JumpToTemporaryBookmark)
}

pub fn create_menu_bar(config: &ConfigManager) -> MenuBar {
	let compact_go = config.get_app_bool("compact_go_menu", true);
	let mut menus = menus(config, compact_go).into_iter();
	let mut builder = MenuBar::builder();
	if let Some(file) = menus.next() {
		builder = builder.append(build_menu(&file.entries), &title(file.category));
	}
	#[cfg(target_os = "macos")]
	{
		// TRANSLATORS: Top-level "Edit" menu label in the menu bar (macOS only)
		let edit_label = t("&Edit");
		builder = builder.append(edit_menu::create_edit_menu(config), &edit_label);
	}
	for menu in menus {
		builder = builder.append(build_menu(&menu.entries), &title(menu.category));
	}
	let menu_bar = builder.build();
	commands::apply_enable_to(&menu_bar, Enable::HasRecentDocuments, config.has_recent_documents());
	menu_bar
}

#[cfg(test)]
mod tests {
	use rstest::rstest;

	use super::*;
	use crate::ui::menu_ids;

	fn ids(entries: &[MenuEntry], out: &mut Vec<i32>) {
		for entry in entries {
			match entry {
				MenuEntry::Item(spec) | MenuEntry::Check(spec, _) => out.push(spec.id),
				MenuEntry::Submenu { entries, .. } => ids(entries, out),
				MenuEntry::Separator | MenuEntry::Disabled(_) => {}
			}
		}
	}

	fn action_for(id: i32) -> Option<ActionId> {
		ActionId::all().iter().copied().find(|&action| menu_ids::action_to_menu_id(action) == id)
	}

	/// macOS puts Exit and Settings in the application menu, and Linux has no OCR engine.
	const fn omitted_on_this_platform(action: ActionId) -> bool {
		if cfg!(target_os = "macos") {
			matches!(action, ActionId::Exit | ActionId::Options)
		} else if cfg!(target_os = "linux") {
			matches!(action, ActionId::BatchOcr)
		} else {
			false
		}
	}

	#[test]
	fn menus_follow_the_shortcut_categories_in_order() {
		let categories: Vec<ShortcutCategory> =
			menus(&ConfigManager::new(), true).iter().map(|menu| menu.category).collect();
		assert_eq!(categories, ShortcutCategory::all());
	}

	#[rstest]
	#[case::compact_go(true)]
	#[case::flat_go(false)]
	fn every_item_sits_in_the_menu_of_its_category(#[case] compact_go: bool) {
		for menu in menus(&ConfigManager::new(), compact_go) {
			let mut found = Vec::new();
			ids(&menu.entries, &mut found);
			for action in found.into_iter().filter_map(action_for) {
				assert_eq!(action.category(), menu.category, "{action:?} is in the {:?} menu", menu.category);
			}
		}
	}

	#[rstest]
	#[case::compact_go(true)]
	#[case::flat_go(false)]
	fn every_action_has_one_menu_item_unless_keyboard_only(#[case] compact_go: bool) {
		let mut found = Vec::new();
		for menu in menus(&ConfigManager::new(), compact_go) {
			ids(&menu.entries, &mut found);
		}
		for &action in ActionId::all() {
			let id = menu_ids::action_to_menu_id(action);
			let count = found.iter().filter(|&&found_id| found_id == id).count();
			let expected = usize::from(!(is_keyboard_only(action) || omitted_on_this_platform(action)));
			assert_eq!(count, expected, "{action:?} has {count} menu items");
		}
	}
}
