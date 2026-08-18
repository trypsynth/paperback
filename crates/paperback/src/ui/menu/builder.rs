use paperback_core::config::{ActionId, ConfigManager};
use wxdragon::prelude::*;

pub struct MenuItemSpec {
	pub id: i32,
	pub label: String,
	pub help: Option<String>,
}

pub enum MenuEntry {
	Item(MenuItemSpec),
	Separator,
}

pub const fn item(id: i32, label: String) -> MenuEntry {
	MenuEntry::Item(MenuItemSpec { id, label, help: None })
}

pub const fn item_with_help(id: i32, label: String, help: String) -> MenuEntry {
	MenuEntry::Item(MenuItemSpec { id, label, help: Some(help) })
}

pub fn build_menu(entries: &[MenuEntry]) -> Menu {
	let mut builder = Menu::builder();
	for entry in entries {
		builder = match entry {
			MenuEntry::Item(spec) => builder.append_item(spec.id, &spec.label, spec.help.as_deref().unwrap_or("")),
			MenuEntry::Separator => builder.append_separator(),
		};
	}
	builder.build()
}

pub fn append_menu_entries(menu: &Menu, entries: &[MenuEntry]) {
	for entry in entries {
		match entry {
			MenuEntry::Item(spec) => {
				let _ = menu.append(spec.id, &spec.label, "", ItemKind::Normal);
			}
			MenuEntry::Separator => menu.append_separator(),
		}
	}
}

pub fn format_menu_label(base: &str, action: ActionId, config: &ConfigManager) -> String {
	let shortcut = config.get_shortcut_menu_str(action);
	if shortcut.is_empty() { base.to_string() } else { format!("{base}\t{shortcut}") }
}
