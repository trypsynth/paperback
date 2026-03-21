use std::{
	env, fs,
	path::PathBuf,
	sync::{Mutex, OnceLock},
};

use wxdragon::translations::{Locale, Translations, add_catalog_lookup_path_prefix};

#[derive(Clone, Debug)]
pub struct LanguageInfo {
	pub code: String,
	pub native_name: String,
}

pub struct TranslationManager {
	current_language: String,
	available_languages: Vec<LanguageInfo>,
	initialized: bool,
}

impl TranslationManager {
	pub fn instance() -> &'static Mutex<Self> {
		static INSTANCE: OnceLock<Mutex<TranslationManager>> = OnceLock::new();
		INSTANCE.get_or_init(|| Mutex::new(Self::new()))
	}

	pub fn initialize(&mut self) -> bool {
		if self.initialized {
			return true;
		}
		let translations = Translations::new();
		if let Some(langs_dir) = langs_directory() {
			add_catalog_lookup_path_prefix(langs_dir.to_string_lossy().as_ref());
		}
		self.scan_available_languages();
		let system_lang_id = Locale::get_system_language();
		let raw_sys_lang =
			Locale::get_language_info(system_lang_id).map_or_else(|| "en".to_string(), |info| info.canonical_name());
		let sys_lang = raw_sys_lang
			.split('_')
			.next()
			.unwrap_or(&raw_sys_lang)
			.split('-')
			.next()
			.unwrap_or(&raw_sys_lang)
			.to_string();
		if self.is_language_available(&sys_lang) {
			self.current_language = sys_lang;
		} else {
			self.current_language = "en".to_string();
		}
		if self.current_language != "en" {
			translations.set_language_str(&self.current_language);
		}
		translations.add_std_catalog();
		if self.current_language != "en" {
			translations.add_catalog("paperback");
		}
		Translations::set_global(translations);
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
		let translations = Translations::new();
		if let Some(langs_dir) = langs_directory() {
			add_catalog_lookup_path_prefix(langs_dir.to_string_lossy().as_ref());
		}

		translations.set_language_str(language_code);
		translations.add_std_catalog();
		if language_code != "en" {
			translations.add_catalog("paperback");
		}
		Translations::set_global(translations);

		true
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

	fn scan_available_languages(&mut self) {
		if let Some(langs_dir) = langs_directory() {
			if let Ok(entries) = fs::read_dir(langs_dir) {
				for entry in entries.flatten() {
					let path = entry.path();
					if path.is_dir() {
						let dirname = path.file_name().and_then(|n| n.to_str()).unwrap_or_default().to_string();
						let catalog_path = path.join("LC_MESSAGES").join("paperback.mo");
						if catalog_path.exists() {
							let mut native_name = dirname.clone();
							if let Some(info) = Locale::find_language_info(&dirname) {
								let desc = info.native_description();
								if !desc.is_empty() {
									native_name = desc;
								}
							}
							self.available_languages.push(LanguageInfo { code: dirname, native_name });
						}
					}
				}
			}
		}
	}

	fn new() -> Self {
		Self {
			current_language: "en".to_string(),
			available_languages: vec![LanguageInfo { code: "en".to_string(), native_name: "English".to_string() }],
			initialized: false,
		}
	}
}

fn langs_directory() -> Option<PathBuf> {
	let exe_path = env::current_exe().ok()?;
	let exe_dir = exe_path.parent()?;
	Some(exe_dir.join("langs"))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn new_manager_has_english_available_by_default() {
		let manager = TranslationManager::new();
		assert_eq!(manager.current_language(), "en");
		assert!(manager.is_language_available("en"));
		assert!(!manager.is_language_available("zz"));
	}

	#[test]
	fn set_language_fails_when_not_initialized() {
		let mut manager = TranslationManager::new();
		assert!(!manager.set_language("en"));
		assert_eq!(manager.current_language(), "en");
	}

	#[test]
	fn available_languages_returns_clone() {
		let manager = TranslationManager::new();
		let mut langs = manager.available_languages();
		langs.push(LanguageInfo { code: "xx".to_string(), native_name: "Fake".to_string() });
		assert!(!manager.is_language_available("xx"));
	}

	#[test]
	fn langs_directory_points_inside_exe_dir() {
		let langs = langs_directory().expect("langs dir");
		assert_eq!(langs.file_name().and_then(|n| n.to_str()), Some("langs"));
	}
}
