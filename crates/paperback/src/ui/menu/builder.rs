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

/// Gives every item at each level of `entries` an access key of its own, where its label has a
/// free letter to give it.
///
/// Each label is translated on its own, so two items of one menu can arrive with the same letter,
/// and pressing it then only cycles between them. The first keeps its letter and a later one moves
/// to the first free letter of its label. A Latin letter only ever moves to another, so the one in a
/// Japanese label's "(&F)" is never moved onto a kana.
pub fn deduplicate_access_keys(entries: &mut [MenuEntry]) {
	let mut used = Vec::new();
	for entry in entries {
		match entry {
			MenuEntry::Item(spec) | MenuEntry::Check(spec, _) => claim_access_key(&mut spec.label, &mut used),
			MenuEntry::Submenu { label, entries, .. } => {
				claim_access_key(label, &mut used);
				deduplicate_access_keys(entries);
			}
			MenuEntry::Separator | MenuEntry::Disabled(_) => {}
		}
	}
}

/// Moves `label`'s access key off a letter in `used` when it can, then adds the key it ends up
/// with to `used`.
pub fn claim_access_key(label: &mut String, used: &mut Vec<char>) {
	// The shortcut after a tab is shown in its own column and is no part of the access key.
	let text_end = label.find('\t').unwrap_or(label.len());
	let Some((at, key)) = access_key(&label[..text_end]) else { return };
	if !used.contains(&fold(key)) {
		used.push(fold(key));
		return;
	}
	let mut plain = label.clone();
	plain.remove(at);
	let text = &plain[..text_end - 1];
	let free = |same_kind: bool| {
		text.char_indices().find(|&(_, c)| {
			c.is_alphanumeric() && (!same_kind || c.is_ascii() == key.is_ascii()) && !used.contains(&fold(c))
		})
	};
	// A letter like the one it had first. A Latin key stays Latin, but an accented or Cyrillic one
	// may fall back to any letter, as "Á" in Portuguese has nowhere else to go.
	let free = free(true).or_else(|| if key.is_ascii() { None } else { free(false) });
	if let Some((i, c)) = free {
		used.push(fold(c));
		plain.insert(i, '&');
		*label = plain;
	}
}

/// Where the `&` that marks `text`'s access key is, and the key after it. `&&` is a literal
/// ampersand, not a marker.
fn access_key(text: &str) -> Option<(usize, char)> {
	let mut chars = text.char_indices();
	while let Some((at, c)) = chars.next() {
		if c == '&' {
			match chars.next() {
				Some((_, '&')) => {}
				Some((_, key)) => return Some((at, key)),
				None => return None,
			}
		}
	}
	None
}

fn fold(c: char) -> char {
	c.to_lowercase().next().unwrap_or(c)
}

pub fn format_menu_label(base: &str, action: ActionId, config: &ConfigManager) -> String {
	wx_utils::menu_label(base, &config.get_shortcut_menu_str(action))
}

#[cfg(test)]
mod tests {
	use super::*;

	fn labels(entries: &[MenuEntry]) -> Vec<&str> {
		entries
			.iter()
			.filter_map(|entry| match entry {
				MenuEntry::Item(spec) | MenuEntry::Check(spec, _) => Some(spec.label.as_str()),
				MenuEntry::Submenu { label, .. } => Some(label.as_str()),
				MenuEntry::Separator | MenuEntry::Disabled(_) => None,
			})
			.collect()
	}

	/// The German Go menu as the machine translation left it, where Alt+Z and Alt+V each reached two items.
	#[test]
	fn a_later_item_moves_to_a_free_letter_of_its_own_label() {
		let mut entries = vec![
			item(1, "Gehe &zurück\tAlt+Left".to_string()),
			item(2, "Gehe &vorwärts\tAlt+Right".to_string()),
			MenuEntry::Separator,
			item(3, "&Zur Seite gehen".to_string()),
			submenu("&Verknüpfungen".to_string(), String::new(), vec![item(4, "&Vorherige".to_string())]),
		];
		deduplicate_access_keys(&mut entries);
		assert_eq!(
			labels(&entries),
			vec!["Gehe &zurück\tAlt+Left", "Gehe &vorwärts\tAlt+Right", "Z&ur Seite gehen", "V&erknüpfungen"]
		);
	}

	#[test]
	fn each_submenu_has_letters_of_its_own() {
		let mut entries = vec![
			item(1, "&Previous".to_string()),
			submenu("&Links".to_string(), String::new(), vec![item(2, "&Previous Link".to_string())]),
		];
		deduplicate_access_keys(&mut entries);
		let MenuEntry::Submenu { entries: inner, .. } = &entries[1] else { panic!("not a submenu") };
		assert_eq!(labels(inner), vec!["&Previous Link"]);
	}

	#[test]
	fn the_french_menu_bar_stops_sharing_its_a() {
		let mut used = Vec::new();
		let titles: Vec<String> = ["&Fichier", "&Affichage", "&Aller", "&Audio", "&Aide"]
			.into_iter()
			.map(|title| {
				let mut title = title.to_string();
				claim_access_key(&mut title, &mut used);
				title
			})
			.collect();
		assert_eq!(titles, vec!["&Fichier", "&Affichage", "A&ller", "A&udio", "A&ide"]);
	}

	#[test]
	fn a_latin_key_is_never_moved_onto_a_kana() {
		let mut entries = vec![item(1, "ファイル(&F)".to_string()), item(2, "フォント(&F)".to_string())];
		deduplicate_access_keys(&mut entries);
		assert_eq!(labels(&entries), vec!["ファイル(&F)", "フォント(&F)"]);
	}

	#[test]
	fn an_accented_key_can_move_to_a_plain_letter() {
		let mut entries = vec![item(1, "Avançar &Áudio".to_string()), item(2, "Retroceder &Áudio".to_string())];
		deduplicate_access_keys(&mut entries);
		assert_eq!(labels(&entries), vec!["Avançar &Áudio", "&Retroceder Áudio"]);
	}

	#[test]
	fn a_literal_ampersand_is_not_an_access_key() {
		let mut entries = vec![
			item(1, "&Save".to_string()),
			item(2, "Search && &Replace".to_string()),
			item(3, "&Rock && Roll".to_string()),
		];
		deduplicate_access_keys(&mut entries);
		assert_eq!(labels(&entries), vec!["&Save", "Search && &Replace", "R&ock && Roll"]);
	}
}
