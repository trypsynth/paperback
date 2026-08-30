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
	// Single-key shortcuts (t = next table, l = next list, …) are deliberately NOT registered as
	// OS menu accelerators: a menu accelerator fires even while typing in an editable control
	// inside the window, which would swallow those letters in the in-window find strip. Instead
	// they're shown inline (e.g. "Next Table (T)") and handled by the text control's own key
	// handler, so they respond while reading the book but never while typing. Modifier combos
	// (Ctrl/Alt) stay as real accelerators — they can't be typed as plain text.
	let Some(chord) = config.get_shortcuts().get_chord(action) else {
		return base.to_string();
	};
	let shortcut = chord.to_shortcut_string();
	if !chord.ctrl && !chord.raw_ctrl && !chord.alt {
		format!("{base} ({shortcut})")
	} else {
		format!("{base}\t{shortcut}")
	}
}
