//! The Find dialog's persisted checkbox state and search-text history.

use super::ConfigManager;
use crate::config::settings::FindSettings;

impl ConfigManager {
	pub fn get_find_settings(&self) -> FindSettings {
		FindSettings {
			match_case: self.get_app_bool("find_match_case", false),
			whole_word: self.get_app_bool("find_whole_word", false),
			use_regex: self.get_app_bool("find_use_regex", false),
		}
	}

	pub fn set_find_settings(&self, settings: FindSettings) {
		self.set_app_bool("find_match_case", settings.match_case);
		self.set_app_bool("find_whole_word", settings.whole_word);
		self.set_app_bool("find_use_regex", settings.use_regex);
	}

	pub fn get_find_history(&self) -> Vec<String> {
		if !self.initialized {
			return Vec::new();
		}
		self.data.borrow().find_history.clone()
	}

	pub fn add_find_history(&self, text: &str, max_len: usize) {
		if !self.initialized {
			return;
		}
		let trimmed = text.trim().to_string();
		if trimmed.is_empty() {
			return;
		}
		{
			let mut data = self.data.borrow_mut();
			if let Some(idx) = data.find_history.iter().position(|e| e == &trimmed) {
				data.find_history.remove(idx);
			}
			data.find_history.insert(0, trimmed);
			while data.find_history.len() > max_len {
				data.find_history.pop();
			}
		}
		self.dirty.set(true);
	}
}
