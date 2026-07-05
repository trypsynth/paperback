use std::sync::{Mutex, OnceLock};

pub use patois::LanguageInfo;
use wxdragon::translations::Translations;

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
		let mgr = patois::LanguageManager::new("paperback");
		self.available_languages = mgr.available();
		let raw_sys_lang = patois::LanguageManager::system_language();
		let sys_lang = raw_sys_lang.split('_').next().unwrap_or(&raw_sys_lang).to_string();
		if self.is_language_available(&sys_lang) {
			self.current_language = sys_lang;
		} else {
			if sys_lang != "en" {
				tracing::warn!(system_lang = %raw_sys_lang, "system language not available, falling back to English");
			}
			self.current_language = "en".to_string();
		}
		if self.current_language != "en" {
			translations.set_language_str(&self.current_language);
		}
		translations.add_std_catalog();
		Translations::set_global(translations);
		patois::set_default_domain("paperback");
		patois::set_locale(&self.current_language);
		self.initialized = true;
		tracing::info!(system_lang = %raw_sys_lang, selected = %self.current_language, "translations initialized");
		true
	}

	pub fn set_language(&mut self, language_code: &str) -> bool {
		if !self.initialized {
			tracing::warn!(language = %language_code, "set_language called before initialize");
			return false;
		}
		if !self.is_language_available(language_code) {
			tracing::warn!(language = %language_code, "requested language not available");
			return false;
		}
		tracing::info!(language = %language_code, "switching language");
		self.current_language = language_code.to_string();
		let translations = Translations::new();
		translations.set_language_str(language_code);
		translations.add_std_catalog();
		Translations::set_global(translations);
		patois::set_locale(language_code);
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

	fn new() -> Self {
		Self {
			current_language: "en".to_string(),
			available_languages: vec![LanguageInfo { code: "en".to_string(), name: "English".to_string() }],
			initialized: false,
		}
	}
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
		langs.push(LanguageInfo { code: "xx".to_string(), name: "Fake".to_string() });
		assert!(!manager.is_language_available("xx"));
	}
}
