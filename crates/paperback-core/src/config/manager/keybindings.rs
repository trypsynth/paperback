//! Hotkey and keyboard-shortcut passthroughs. `ConfigManager` just stores/returns these two
//! blobs ([`HotkeyConfig`]/[`ShortcutsConfig`]) - all the actual chord parsing/formatting logic
//! lives in [`crate::config::shortcuts`].

use super::ConfigManager;
use crate::config::shortcuts::{ActionId, HotkeyConfig, KeyChord, ShortcutsConfig};

impl ConfigManager {
	pub fn get_hotkey(&self) -> HotkeyConfig {
		if !self.initialized {
			return HotkeyConfig::default();
		}
		self.data.borrow().app.hotkey.clone()
	}

	pub fn set_hotkey(&self, hotkey: &HotkeyConfig) {
		if !self.initialized {
			return;
		}
		self.data.borrow_mut().app.hotkey = hotkey.clone();
		self.dirty.set(true);
	}

	pub fn get_shortcuts(&self) -> ShortcutsConfig {
		if !self.initialized {
			return ShortcutsConfig::default();
		}
		self.data.borrow().app.shortcuts.clone()
	}

	pub fn set_shortcuts(&self, shortcuts: &ShortcutsConfig) {
		if !self.initialized {
			return;
		}
		self.data.borrow_mut().app.shortcuts = shortcuts.clone();
		self.dirty.set(true);
	}

	pub fn get_shortcut_chord(&self, action: ActionId) -> Option<KeyChord> {
		self.get_shortcuts().get_chord(action)
	}

	pub fn get_shortcut_menu_str(&self, action: ActionId) -> String {
		self.get_shortcuts().get_menu_str(action)
	}
}
