use paperback_core::config::{ActionId, ConfigManager};
use wxdragon::prelude::*;

pub struct MenuItemSpec {
	pub id: i32,
	pub label: String,
	pub help: Option<String>,
}

pub enum MenuEntry {
	Item(MenuItemSpec),
	/// A checkable item, and whether it starts checked.
	Check(MenuItemSpec, bool),
	/// An item that is shown greyed out and has no id of its own.
	Disabled(String),
	Separator,
	Submenu {
		label: String,
		help: String,
		entries: Vec<Self>,
	},
}

pub const fn item(id: i32, label: String) -> MenuEntry {
	MenuEntry::Item(MenuItemSpec { id, label, help: None })
}

pub const fn item_with_help(id: i32, label: String, help: String) -> MenuEntry {
	MenuEntry::Item(MenuItemSpec { id, label, help: Some(help) })
}

pub const fn check(id: i32, label: String, help: String, checked: bool) -> MenuEntry {
	MenuEntry::Check(MenuItemSpec { id, label, help: Some(help) }, checked)
}

pub const fn submenu(label: String, help: String, entries: Vec<MenuEntry>) -> MenuEntry {
	MenuEntry::Submenu { label, help, entries }
}

pub fn build_menu(entries: &[MenuEntry]) -> Menu {
	let menu = Menu::builder().build();
	append_menu_entries(&menu, entries);
	menu
}

pub fn append_menu_entries(menu: &Menu, entries: &[MenuEntry]) {
	for entry in entries {
		match entry {
			MenuEntry::Item(spec) => {
				let _ = menu.append(spec.id, &spec.label, spec.help.as_deref().unwrap_or(""), ItemKind::Normal);
			}
			MenuEntry::Check(spec, checked) => {
				let _ = menu.append(spec.id, &spec.label, spec.help.as_deref().unwrap_or(""), ItemKind::Check);
				menu.check_item(spec.id, *checked);
			}
			MenuEntry::Disabled(label) => {
				if let Some(item) = menu.append(ID_ANY.try_into().unwrap(), label, "", ItemKind::Normal) {
					item.enable(false);
				}
			}
			MenuEntry::Separator => menu.append_separator(),
			MenuEntry::Submenu { label, help, entries } => {
				let _ = menu.append_submenu(build_menu(entries), label, help);
			}
		}
	}
}

pub fn format_menu_label(base: &str, action: ActionId, config: &ConfigManager) -> String {
	wx_utils::menu_label(base, &config.get_shortcut_menu_str(action))
}
