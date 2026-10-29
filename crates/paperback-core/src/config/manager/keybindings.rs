use super::ConfigManager;
use crate::config::shortcuts::{ActionId, KeyChord, ShortcutsConfig, default_window_hotkey};

impl ConfigManager {
	/// `None` when the reader cleared it.
	pub fn get_hotkey(&self) -> Option<KeyChord> {
		if !self.initialized {
			return Some(default_window_hotkey());
		}
		stored_hotkey(&self.data.borrow().app.hotkey)
	}

	pub fn set_hotkey(&self, hotkey: Option<&KeyChord>) {
		if !self.initialized {
			return;
		}
		self.data.borrow_mut().app.hotkey = hotkey.cloned().unwrap_or_else(|| KeyChord::new(false, false, false, ""));
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

/// Older configs kept the key as one char, so Space was " " and a cleared hotkey was NUL.
fn stored_hotkey(stored: &KeyChord) -> Option<KeyChord> {
	let key = match stored.key.as_str() {
		"" | "\0" => return None,
		" " => "Space".to_string(),
		key => KeyChord::normalize_key_name(key),
	};
	Some(KeyChord { key, ..stored.clone() })
}

#[cfg(test)]
mod tests {
	use super::*;

	fn config() -> ConfigManager {
		let mut config = ConfigManager::new();
		config.initialized = true;
		config
	}

	#[test]
	fn a_hotkey_saved_before_the_shortcuts_dialog_held_it_reads_back() {
		let read = |key: &str| {
			let app: crate::config::AppSettings = toml::from_str(&format!(
				"[hotkey]\nctrl = true\nalt = true\nshift = false\nwin = false\nkey = \"{key}\"\n"
			))
			.unwrap();
			stored_hotkey(&app.hotkey)
		};
		assert_eq!(read("P"), Some(default_window_hotkey()));
		assert_eq!(read(" ").map(|chord| chord.key), Some("Space".to_string()));
		assert_eq!(read("\\u0000"), None);
	}

	#[test]
	fn a_cleared_hotkey_stays_cleared_and_a_missing_one_is_the_default() {
		let config = config();
		config.set_hotkey(None);
		assert_eq!(config.get_hotkey(), None);
		config.set_hotkey(Some(&KeyChord::new(true, false, true, "F5").with_win(true)));
		assert_eq!(
			config.get_hotkey().map(|chord| chord.to_shortcut_string()),
			Some(KeyChord::new(true, false, true, "F5").with_win(true).to_shortcut_string())
		);
		let app: crate::config::AppSettings = toml::from_str("").unwrap();
		assert_eq!(stored_hotkey(&app.hotkey), Some(default_window_hotkey()));
	}
}
