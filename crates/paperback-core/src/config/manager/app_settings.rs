//! The generic `get_app_*`/`set_app_*` accessors: a handful of named boolean/integer settings
//! get dedicated [`super::super::settings::AppSettings`] fields (for the ones every reader
//! needs), with everything else falling through to `AppSettings::extra`, a free-form TOML map -
//! host UIs can introduce a new setting without a `ConfigData` schema change.

use super::ConfigManager;

impl ConfigManager {
	pub fn get_app_string(&self, key: &str, default_value: &str) -> String {
		if !self.initialized {
			return default_value.to_string();
		}
		let data = self.data.borrow();
		data.app.extra.get(key).and_then(|v| v.as_str()).map_or_else(|| default_value.to_string(), str::to_string)
	}

	pub fn get_app_bool(&self, key: &str, default_value: bool) -> bool {
		if !self.initialized {
			return default_value;
		}
		let data = self.data.borrow();
		match key {
			"restore_previous_documents" => data.app.restore_previous_documents,
			"word_wrap" => data.app.word_wrap,
			"render_tables_inline" => data.app.render_tables_inline,
			"navigation_wrap" => data.app.navigation_wrap,
			"find_match_case" => data.app.find_match_case,
			"find_whole_word" => data.app.find_whole_word,
			"find_use_regex" => data.app.find_use_regex,
			_ => data.app.extra.get(key).and_then(toml::Value::as_bool).unwrap_or(default_value),
		}
	}

	pub fn get_app_int(&self, key: &str, default_value: i32) -> i32 {
		if !self.initialized {
			return default_value;
		}
		let data = self.data.borrow();
		let v: i64 = match key {
			"recent_documents_to_show" => data.app.recent_documents_to_show,
			"sleep_timer_duration" => data.app.sleep_timer_duration,
			"reading_speed_wpm" => data.app.reading_speed_wpm,
			// Readability settings have typed fields on AppConfig. Without these arms the
			// generic API would read a stale copy from `extra` while writing a second one
			// beside the real field, giving the [app] table two entries with the same name.
			"line_spacing" => data.app.line_spacing,
			"paragraph_spacing" => data.app.paragraph_spacing,
			"letter_spacing" => data.app.letter_spacing,
			"text_alignment" => data.app.text_alignment,
			_ => {
				return data
					.app
					.extra
					.get(key)
					.and_then(toml::Value::as_integer)
					.and_then(|i| i32::try_from(i).ok())
					.unwrap_or(default_value);
			}
		};
		v.try_into().unwrap_or(default_value)
	}

	pub fn set_app_string(&self, key: &str, value: &str) {
		if !self.initialized {
			return;
		}
		self.data.borrow_mut().app.extra.insert(key.to_string(), toml::Value::String(value.to_string()));
		self.dirty.set(true);
	}

	pub fn set_app_bool(&self, key: &str, value: bool) {
		if !self.initialized {
			return;
		}
		{
			let mut data = self.data.borrow_mut();
			match key {
				"restore_previous_documents" => data.app.restore_previous_documents = value,
				"word_wrap" => data.app.word_wrap = value,
				"render_tables_inline" => data.app.render_tables_inline = value,
				"navigation_wrap" => data.app.navigation_wrap = value,
				"find_match_case" => data.app.find_match_case = value,
				"find_whole_word" => data.app.find_whole_word = value,
				"find_use_regex" => data.app.find_use_regex = value,
				_ => {
					data.app.extra.insert(key.to_string(), toml::Value::Boolean(value));
				}
			}
		}
		self.dirty.set(true);
	}

	pub fn set_app_int(&self, key: &str, value: i32) {
		if !self.initialized {
			return;
		}
		{
			let mut data = self.data.borrow_mut();
			match key {
				"recent_documents_to_show" => data.app.recent_documents_to_show = i64::from(value),
				"sleep_timer_duration" => data.app.sleep_timer_duration = i64::from(value),
				"reading_speed_wpm" => data.app.reading_speed_wpm = i64::from(value),
				"line_spacing" => data.app.line_spacing = i64::from(value),
				"paragraph_spacing" => data.app.paragraph_spacing = i64::from(value),
				"letter_spacing" => data.app.letter_spacing = i64::from(value),
				"text_alignment" => data.app.text_alignment = i64::from(value),
				_ => {
					data.app.extra.insert(key.to_string(), toml::Value::Integer(i64::from(value)));
				}
			}
		}
		self.dirty.set(true);
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn render_tables_inline_round_trips() {
		let mut config = ConfigManager::new();
		config.initialized = true;
		assert!(config.get_app_bool("render_tables_inline", true));
		config.set_app_bool("render_tables_inline", false);
		assert!(!config.get_app_bool("render_tables_inline", true));
		config.set_app_bool("render_tables_inline", true);
		assert!(config.get_app_bool("render_tables_inline", true));
	}

	// These have typed fields on AppConfig, so the generic int API has to reach the field
	// rather than the `extra` map: two entries under one name make the [app] table invalid.
	#[test]
	fn readability_ints_use_their_typed_fields() {
		let mut config = ConfigManager::new();
		config.initialized = true;
		for key in ["line_spacing", "paragraph_spacing", "letter_spacing", "text_alignment"] {
			config.set_app_int(key, 2);
			assert_eq!(config.get_app_int(key, 0), 2, "{key} did not round trip");
			assert!(!config.data.borrow().app.extra.contains_key(key), "{key} leaked into extra");
		}
	}
}
