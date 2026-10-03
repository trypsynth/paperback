//! The keyboard shortcut system: [`ActionId`] names every user-triggerable action,
//! [`KeyChord`] represents a physical key combination, and [`ShortcutsConfig`] maps the
//! two together with per-action overrides layered on [`ActionId::default_chord`]. Also
//! holds [`HotkeyConfig`], the single global show/hide hotkey, which is unrelated to the
//! per-action bindings but is just as much a "keyboard shortcut" setting.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

mod action_id;

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
	Audio,
	Tools,
	Help,
}

impl ShortcutCategory {
	pub const fn all() -> &'static [Self] {
		&[Self::File, Self::Go, Self::Audio, Self::Tools, Self::Help]
	}

	pub fn display_name(self) -> String {
		match self {
			// TRANSLATORS: Name of the "File" category of keyboard shortcuts, shown as a tab label in the Customize Keyboard Shortcuts dialog
			Self::File => crate::t("File"),
			// TRANSLATORS: Name of the "Go" (navigation) category of keyboard shortcuts, shown as a tab label in the Customize Keyboard Shortcuts dialog
			Self::Go => crate::t("Go"),
			// TRANSLATORS: Name of the "Audio" category of keyboard shortcuts, shown as a tab label in the Customize Keyboard Shortcuts dialog
			Self::Audio => crate::t("Audio"),
			// TRANSLATORS: Name of the "Tools" category of keyboard shortcuts, shown as a tab label in the Customize Keyboard Shortcuts dialog
			Self::Tools => crate::t("Tools"),
			// TRANSLATORS: Name of the "Help" category of keyboard shortcuts, shown as a tab label in the Customize Keyboard Shortcuts dialog
			Self::Help => crate::t("Help"),
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
		// TRANSLATORS: Shown in the Customize Keyboard Shortcuts list in place of a key combination when an action has no shortcut assigned
		self.get_chord(action).map_or_else(|| crate::t("None"), |c| c.to_shortcut_string())
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

	/// On a Japanese keyboard `=` and `'` are typed with Shift, so the reader finds their shortcuts by the character typed instead of the key pressed (#982). That lookup is by character code with no modifiers, and has to land on the defaults.
	#[test]
	fn punctuation_defaults_are_found_by_the_character_they_type() {
		let sc = ShortcutsConfig::default();
		assert_eq!(sc.find_action(i32::from(b'='), false, false, false), Some(ActionId::AnnouncePercent));
		assert_eq!(sc.find_action(i32::from(b'\''), false, false, false), Some(ActionId::SeekAudioForward));
	}

	/// Found by the key code the text control reports, 344 (`WXK_F5`), through the lookup the key handler uses.
	#[test]
	fn f5_reloads_the_document() {
		let chord = ActionId::Reload.default_chord().expect("reload ships a default chord");
		assert_eq!(chord, KeyChord::new(false, false, false, "F5"));
		assert!(chord.matches(344, false, false, false), "F5 on its own must match");
		assert_eq!(ShortcutsConfig::default().find_action(344, false, false, false), Some(ActionId::Reload));
		assert!(!chord.matches(344, true, false, false), "must not answer for Ctrl+F5");
		assert!(!chord.matches(344, false, true, false), "must not answer for Alt+F5");
		assert!(!chord.matches(344, false, false, true), "must not answer for Shift+F5");
	}

	/// `find_action` returns the first match, so of two actions sharing a default chord the later one could never be reached.
	#[test]
	fn no_two_actions_share_a_default_chord() {
		let all = ActionId::all();
		for (index, &first) in all.iter().enumerate() {
			let Some(first_chord) = first.default_chord() else {
				continue;
			};
			for &second in &all[index + 1..] {
				let Some(second_chord) = second.default_chord() else {
					continue;
				};
				assert_ne!(
					(
						&first_chord.key,
						first_chord.ctrl,
						first_chord.raw_ctrl,
						first_chord.alt,
						first_chord.shift,
						first_chord.win
					),
					(
						&second_chord.key,
						second_chord.ctrl,
						second_chord.raw_ctrl,
						second_chord.alt,
						second_chord.shift,
						second_chord.win
					),
					"{first:?} and {second:?} ship the same default chord"
				);
			}
		}
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

	#[cfg(target_os = "macos")]
	#[test]
	fn play_pause_audio_uses_physical_control_on_macos() {
		let chord = ActionId::PlayPauseAudio.default_chord().unwrap();
		assert!(!chord.ctrl);
		assert!(chord.raw_ctrl);
		assert_eq!(chord.key, "Space");
	}

	/// Pinned to the key codes the text control actually reports, so a typo in the key string
	/// fails here rather than shipping a shortcut that silently never fires. 348 and 349 are
	/// `WXK_F9` and `WXK_F10` as wxdragon defines them.
	#[cfg(not(target_os = "macos"))]
	#[test]
	fn selection_shortcuts_are_alt_f9_f10_and_alt_shift_f9() {
		for (action, key_code, key_name, own_shift) in [
			(ActionId::SetSelectionStart, 348, "F9", false),
			(ActionId::CopyFromSelectionStart, 349, "F10", false),
			(ActionId::JumpToSelectionStart, 348, "F9", true),
		] {
			let chord = action.default_chord().expect("every selection command ships a default");
			assert_eq!(chord, KeyChord::new(false, true, own_shift, key_name), "{action:?} default chord");
			assert!(chord.matches(key_code, false, true, own_shift), "{action:?} must match its own chord");
			// The other Alt form on the same key belongs to a sibling command, not to this one.
			assert!(
				!chord.matches(key_code, false, true, !own_shift),
				"{action:?} must not answer for the other Alt form"
			);
			// Alt is doing the work: the bare key must not claim it, or F10 alone would collide
			// with the menu bar and Shift+F10 with the reader's context menu.
			assert!(!chord.matches(key_code, false, false, false), "{action:?} must not match the bare key");
			assert!(!chord.matches(key_code, false, false, true), "{action:?} must not match Shift+{key_name}");
		}
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

	#[test]
	fn audio_category_holds_the_playback_commands() {
		assert_eq!(
			ShortcutCategory::Audio.actions(),
			[
				ActionId::PlayPauseAudio,
				ActionId::SeekAudioForward,
				ActionId::SeekAudioBackward,
				ActionId::IncreaseAudioSeekAmount,
				ActionId::DecreaseAudioSeekAmount,
				ActionId::IncreaseAudioSpeed,
				ActionId::DecreaseAudioSpeed,
			]
		);
	}

	#[test]
	fn formula_shortcuts_are_discoverable_and_customizable() {
		let mut config = ShortcutsConfig::default();
		assert_eq!(config.find_action(i32::from(b'M'), false, false, false), Some(ActionId::NextFormula));
		assert_eq!(config.find_action(i32::from(b'M'), false, false, true), Some(ActionId::PreviousFormula));
		assert_eq!(ActionId::NextFormula.category(), ShortcutCategory::Go);
		config.set_chord(ActionId::NextFormula, Some(KeyChord::new(true, true, true, "M")));
		assert_eq!(config.find_action(i32::from(b'M'), false, false, false), None);
		assert_eq!(config.find_action(i32::from(b'M'), true, true, true), Some(ActionId::NextFormula));
		let serialized = toml::to_string(&config).unwrap();
		assert!(serialized.contains("next_formula"));
		let restored: ShortcutsConfig = toml::from_str(&serialized).unwrap();
		assert_eq!(restored.get_chord(ActionId::NextFormula), config.get_chord(ActionId::NextFormula));
	}
}
