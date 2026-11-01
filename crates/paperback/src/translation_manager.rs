use std::sync::{Mutex, OnceLock};

pub use patois::LanguageInfo;
use patois::ui::WxTranslationManager;

use crate::WxStdCatalogLoader;

pub struct TranslationManager {
	inner: WxTranslationManager,
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
		let raw_sys_lang = patois::LanguageManager::system_language();
		self.inner.initialize(WxStdCatalogLoader);
		self.initialized = true;
		// A system language with a region can be served by another region's catalogue, so ask patois rather than guess from the code.
		if patois::ui::best_available_language(&raw_sys_lang, &self.available_languages()).is_none() {
			tracing::warn!(system_lang = %raw_sys_lang, "system language not available, falling back to English");
		}
		tracing::info!(system_lang = %raw_sys_lang, selected = %self.inner.current_language(), "translations initialized");
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
		self.inner.set_language(language_code, WxStdCatalogLoader)
	}

	pub fn current_language(&self) -> String {
		self.inner.current_language()
	}

	pub fn available_languages(&self) -> Vec<LanguageInfo> {
		self.inner.available_languages()
	}

	pub fn is_language_available(&self, language_code: &str) -> bool {
		self.inner.is_language_available(language_code)
	}

	fn new() -> Self {
		Self { inner: WxTranslationManager::new("paperback"), initialized: false }
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
		manager.available_languages().push(LanguageInfo { code: "xx".to_string(), name: "Fake".to_string() });
		assert!(!manager.is_language_available("xx"));
	}

	/// Broke twice, both showing English buttons in a translated updater dialog (#285): the app and `ship-shape` linked separate patois copies, and pot generation skipped dependency strings.
	#[test]
	fn dependency_owned_strings_translate_through_the_app_catalog() {
		patois::set_default_domain("paperback");
		let _locale = crate::test_locale::pinned_to("fr");
		// "Downloading update..." exists only in ship-shape, so it catches dependency strings missing from the pot.
		for msgid in ["&Yes", "&No", "Downloading update..."] {
			assert_ne!(patois::t(msgid), msgid, "{msgid} is missing from the French catalog");
		}
	}

	#[test]
	fn only_shipped_languages_have_wxstd_catalogs_embedded() {
		use wxdragon::translations::TranslationsLoader as _;
		let loader = WxStdCatalogLoader;
		let langs = loader.available_translations("wxstd-3.3");
		// No catalogs when the build had no gettext, as on CI.
		if langs.is_empty() {
			return;
		}
		assert!(langs.iter().any(|l| l.eq_ignore_ascii_case("de")), "expected German catalog, got {langs:?}");
		assert!(!langs.iter().any(|l| l.eq_ignore_ascii_case("af")), "af is not a paperback language: {langs:?}");
	}

	#[test]
	fn wxwidgets_translates_via_embedded_german_catalog() {
		use wxdragon::translations::Translations;
		let loader = WxStdCatalogLoader;
		{
			use wxdragon::translations::TranslationsLoader as _;
			if loader.available_translations("wxstd-3.3").is_empty() {
				return;
			}
		}
		let translations = Translations::new();
		translations.set_loader(WxStdCatalogLoader);
		translations.set_language_str("de");
		assert!(translations.add_std_catalog(), "add_std_catalog should load the embedded wxstd catalog");
		assert_eq!(translations.get_string("Cancel", "wxstd-3.3").as_deref(), Some("Abbrechen"));
	}
}
