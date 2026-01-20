use std::{
	env,
	path::PathBuf,
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
		if let Some(langs_dir) = langs_directory() {
			add_catalog_lookup_path_prefix(langs_dir.to_string_lossy().as_ref());
		}
		let system_lang = system_language();
		translations.set_language_str(&system_lang);
		let _loaded = translations.add_catalog("paperback");
		translations.add_std_catalog();
		Translations::set_global(translations);
		self.available_languages = Translations::get()
			.map(|translations| translations.get_available_translations("paperback"))
			.unwrap_or_default()
			.into_iter()
			.filter(|code| !code.is_empty())
			.map(|code| LanguageInfo { code: code.clone(), name: code.clone(), native_name: code })
			.collect();
		if self.available_languages.iter().all(|lang| lang.code != "en") {
			self.available_languages.push(LanguageInfo {
				code: "en".to_string(),
				name: "English".to_string(),
				native_name: "English".to_string(),
			});
		}
		if self.is_language_available(&system_lang) {
			self.current_language = system_lang;
		}
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
		if let Some(translations) = Translations::get() {
			translations.set_language_str(language_code);
		}
		self.current_language = language_code.to_string();
		true
	}

	pub fn current_language(&self) -> String {
		self.current_language.clone()
	}

	pub fn available_languages(&self) -> Vec<LanguageInfo> {
		self.available_languages.clone()
	}

	pub fn language_display_name(&self, language_code: &str) -> String {
		self.available_languages
			.iter()
			.find(|lang| lang.code == language_code)
			.map(|lang| lang.native_name.clone())
			.unwrap_or_else(|| language_code.to_string())
	}

	pub fn is_language_available(&self, language_code: &str) -> bool {
		self.available_languages.iter().any(|lang| lang.code == language_code)
	}

	fn new() -> Self {
		Self {
			current_language: "en".to_string(),
			available_languages: vec![LanguageInfo {
				code: "en".to_string(),
				name: "English".to_string(),
				native_name: "English".to_string(),
			}],
			initialized: false,
		}
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
