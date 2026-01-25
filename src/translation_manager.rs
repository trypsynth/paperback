use std::{
	env, fs,
	path::{Path, PathBuf},
	sync::{Mutex, OnceLock},
};

use wxdragon::translations::{Translations, add_catalog_lookup_path_prefix};

#[derive(Clone, Debug)]
pub struct LanguageInfo {
	pub code: String,
	pub name: String,
	pub native_name: String,
}

pub struct TranslationManager {
	current_language: String,
	available_languages: Vec<LanguageInfo>,
	initialized: bool,
}

impl TranslationManager {
	pub fn instance() -> &'static Mutex<TranslationManager> {
		static INSTANCE: OnceLock<Mutex<TranslationManager>> = OnceLock::new();
		INSTANCE.get_or_init(|| Mutex::new(TranslationManager::new()))
	}

	pub fn initialize(&mut self) -> bool {
		if self.initialized {
			return true;
		}
		let translations = Translations::new();
		Translations::set_global(translations);
		if let Some(langs_dir) = langs_directory() {
			add_catalog_lookup_path_prefix(langs_dir.to_string_lossy().as_ref());
			self.scan_available_languages(&langs_dir);
		} else {
			self.ensure_english_available();
		}
		let system_lang = system_language();
		if self.is_language_available(&system_lang) {
			self.current_language = system_lang;
		} else {
			self.current_language = "en".to_string();
		}
		self.apply_language_settings(&self.current_language.clone());
		self.initialized = true;
		true
	}

	pub fn set_language(&mut self, language_code: &str) -> bool {
		if !self.initialized {
			return false;
		}
		if !self.is_language_available(language_code) {
			return false;
		}
		self.current_language = language_code.to_string();
		self.apply_language_settings(language_code)
	}

	fn apply_language_settings(&self, language_code: &str) -> bool {
		let translations = Translations::new();
		Translations::set_global(translations);
		if let Some(langs_dir) = langs_directory() {
			add_catalog_lookup_path_prefix(langs_dir.to_string_lossy().as_ref());
		}
		if let Some(t) = Translations::get() {
			t.set_language_str(language_code);
			t.add_std_catalog();
			if language_code != "en" {
				t.add_catalog("paperback");
			}
			true
		} else {
			false
		}
	}

	fn scan_available_languages(&mut self, langs_dir: &Path) {
		self.available_languages.clear();
		self.ensure_english_available();
		if let Ok(entries) = fs::read_dir(langs_dir) {
			for entry in entries.flatten() {
				if let Ok(file_type) = entry.file_type() {
					if file_type.is_dir() {
						let path = entry.path();
						let dir_name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
						let mo_path = path.join("LC_MESSAGES").join("paperback.mo");
						if mo_path.exists() {
							let native_name = get_native_name(&dir_name);
							let name = native_name.clone();
							self.available_languages.push(LanguageInfo { code: dir_name, name, native_name });
						}
					}
				}
			}
		}
	}

	fn ensure_english_available(&mut self) {
		if !self.available_languages.iter().any(|l| l.code == "en") {
			self.available_languages.push(LanguageInfo {
				code: "en".to_string(),
				name: "English".to_string(),
				native_name: "English".to_string(),
			});
		}
	}

	pub fn current_language(&self) -> String {
		self.current_language.clone()
	}

	pub fn available_languages(&self) -> Vec<LanguageInfo> {
		self.available_languages.clone()
	}

	pub fn is_language_available(&self, language_code: &str) -> bool {
		self.available_languages.iter().any(|lang| lang.code == language_code)
	}

	fn new() -> Self {
		Self { current_language: "en".to_string(), available_languages: Vec::new(), initialized: false }
	}
}

fn get_native_name(code: &str) -> String {
	match code {
		"bs" => "Bosanski".to_string(),
		"de" => "Deutsch".to_string(),
		"es" => "Español".to_string(),
		"fr" => "Français".to_string(),
		"ru" => "Русский".to_string(),
		"sr" => "Српски".to_string(),
		"vi" => "Tiếng Việt".to_string(),
		"en" => "English".to_string(),
		_ => code.to_string(),
	}
}

fn langs_directory() -> Option<PathBuf> {
	let exe_path = env::current_exe().ok()?;
	let exe_dir = exe_path.parent()?;
	Some(exe_dir.join("langs"))
}

fn system_language() -> String {
	for key in ["LC_ALL", "LANG", "LANGUAGE"] {
		if let Ok(value) = env::var(key) {
			let trimmed = value.trim();
			if trimmed.is_empty() {
				continue;
			}
			let lang = trimmed.split('.').next().unwrap_or(trimmed);
			let lang = lang.split('@').next().unwrap_or(lang);
			let lang = lang.split(['_', '-']).next().unwrap_or(lang);
			if !lang.is_empty() {
				return lang.to_string();
			}
		}
	}
	"en".to_string()
}
