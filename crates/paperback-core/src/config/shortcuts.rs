//! The keyboard shortcut system: [`ActionId`] names every user-triggerable action,
//! [`KeyChord`] represents a physical key combination, and [`ShortcutsConfig`] maps the
//! two together with per-action overrides layered on [`ActionId::default_chord`]. Also
//! holds [`HotkeyConfig`], the single global show/hide hotkey, which is unrelated to the
//! per-action bindings but is just as much a "keyboard shortcut" setting.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

mod action_id;
mod key_chord;

pub use action_id::ActionId;
pub use key_chord::KeyChord;

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HotkeyConfig {
	pub ctrl: bool,
	pub alt: bool,
	pub shift: bool,
	pub win: bool,
	pub key: char,
}

impl Default for HotkeyConfig {
	fn default() -> Self {
		Self { ctrl: true, alt: true, shift: false, win: false, key: 'P' }
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ShortcutCategory {
	File,
	Go,
	Tools,
	Help,
}

impl ShortcutCategory {
	pub const fn all() -> &'static [Self] {
		&[Self::File, Self::Go, Self::Tools, Self::Help]
	}

	pub const fn display_name(self) -> &'static str {
		match self {
			Self::File => "File",
			Self::Go => "Go",
			Self::Tools => "Tools",
			Self::Help => "Help",
		}
	}

	pub fn actions(self) -> Vec<ActionId> {
		ActionId::all().iter().copied().filter(|a| a.category() == self).collect()
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ShortcutsConfig {
	#[serde(default)]
	pub bindings: HashMap<ActionId, Option<String>>,
}

impl ShortcutsConfig {
	pub fn get_chord(&self, action: ActionId) -> Option<KeyChord> {
		if let Some(entry) = self.bindings.get(&action) {
			match entry {
				Some(s) => KeyChord::parse(s),
				None => None,
			}
		} else {
			action.default_chord()
		}
	}

	pub fn get_display_str(&self, action: ActionId) -> String {
		self.get_chord(action).map_or_else(|| "None".to_string(), |c| c.to_shortcut_string())
	}

	pub fn get_menu_str(&self, action: ActionId) -> String {
		self.get_chord(action).map_or_else(String::new, |c| c.to_shortcut_string())
	}

	pub fn set_chord(&mut self, action: ActionId, chord: Option<KeyChord>) {
		self.bindings.insert(action, chord.map(|c| c.to_shortcut_string()));
	}

	pub fn reset_action(&mut self, action: ActionId) {
		self.bindings.remove(&action);
	}

	pub fn reset_category(&mut self, category: ShortcutCategory) {
		for action in category.actions() {
			self.bindings.remove(&action);
		}
	}

	pub fn reset_all(&mut self) {
		self.bindings.clear();
	}

	pub fn find_action(&self, key_code: i32, ctrl: bool, alt: bool, shift: bool) -> Option<ActionId> {
		for &action in ActionId::all() {
			if let Some(chord) = self.get_chord(action)
				&& chord.matches(key_code, ctrl, alt, shift)
			{
				return Some(action);
			}
		}
		None
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn key_chord_parse_and_to_string() {
		let chord = KeyChord::parse("Ctrl+Shift+O").unwrap();
		assert!(chord.ctrl);
		assert!(chord.shift);
		assert!(!chord.alt);
		assert_eq!(chord.key, "O");
		assert_eq!(chord.to_shortcut_string(), "Ctrl+Shift+O");
		let single = KeyChord::parse("H").unwrap();
		assert!(!single.ctrl);
		assert!(!single.shift);
		assert!(!single.alt);
		assert_eq!(single.key, "H");
		assert_eq!(single.to_shortcut_string(), "H");
		assert_eq!(KeyChord::parse("none"), None);
		assert_eq!(KeyChord::parse(""), None);
	}

	#[test]
	fn shortcuts_config_set_reset_and_find() {
		let mut sc = ShortcutsConfig::default();
		let default_open = sc.get_chord(ActionId::Open);
		assert!(default_open.is_some());
		let new_chord = KeyChord::new(true, true, false, "K");
		sc.set_chord(ActionId::Open, Some(new_chord.clone()));
		assert_eq!(sc.get_chord(ActionId::Open), Some(new_chord));
		let matched = sc.find_action(75, true, true, false);
		assert_eq!(matched, Some(ActionId::Open));
		sc.reset_action(ActionId::Open);
		assert_eq!(sc.get_chord(ActionId::Open), default_open);
	}

	#[test]
	fn shortcut_category_actions_coverage() {
		let mut total_actions = 0;
		for cat in ShortcutCategory::all() {
			let actions = cat.actions();
			assert!(!actions.is_empty());
			for action in &actions {
				assert_eq!(action.category(), *cat);
			}
			total_actions += actions.len();
		}
		assert_eq!(total_actions, ActionId::all().len());
	}
}
