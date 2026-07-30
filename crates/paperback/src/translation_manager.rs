use std::sync::{Mutex, OnceLock};

pub use patois::LanguageInfo;
use patois::ui::WxTranslationManager;

use crate::WxStdCatalogLoader;

/// Thin app-side wrapper around `patois::ui::WxTranslationManager`: owns the singleton
/// lifecycle (a concern of this app, not something patois should manage) and the
/// paperback-specific logging around it.
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
		let sys_lang = raw_sys_lang.split('_').next().unwrap_or(&raw_sys_lang).to_string();
		self.inner.initialize(WxStdCatalogLoader);
		self.initialized = true;
		if sys_lang != "en" && !self.is_language_available(&sys_lang) {
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
		let mut langs = manager.available_languages();
		langs.push(LanguageInfo { code: "xx".to_string(), name: "Fake".to_string() });
		assert!(!manager.is_language_available("xx"));
	}

	/// Confirms `patois::embed_wx_translations!()` (invoked in `main.rs`) actually embedded
	/// real wxstd catalogs restricted to paperback's own shipped languages, without needing
	/// a visible window — mirrors the `wxdragon`/`patois` upstream headless test pattern.
	/// Degrades gracefully (no-ops) if wxstd catalogs weren't available at build time, e.g.
	/// CI without gettext.
	#[test]
	fn only_shipped_languages_have_wxstd_catalogs_embedded() {
		use wxdragon::translations::TranslationsLoader as _;
		let loader = WxStdCatalogLoader;
		let langs = loader.available_translations("wxstd-3.3");
		if langs.is_empty() {
			return;
		}
		assert!(langs.iter().any(|l| l.eq_ignore_ascii_case("de")), "expected German catalog, got {langs:?}");
		assert!(!langs.iter().any(|l| l.eq_ignore_ascii_case("af")), "af is not a paperback language: {langs:?}");
	}

	/// End-to-end proof that wxWidgets actually loads and translates through the
	/// macro-generated loader: sets German, loads the embedded catalog, and checks a known
	/// wx string translates. Runs headless (no wxApp), like `WxStdCatalogLoader`'s own tests
	/// used to (before this logic moved to `patois::embed_wx_translations!()`).
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
