//! Embeds wxWidgets' own standard message catalogs (`wxstd`) into the binary and
//! serves them to wxWidgets at runtime via a [`TranslationsLoader`].
//!
//! wxWidgets ships translations for its built-in UI strings ("OK", "Cancel",
//! progress-dialog labels, etc.) as gettext `.mo` files. When Paperback is
//! installed there is no wxWidgets `share/locale` tree next to the executable,
//! so those strings would never be translated. To fix that we bundle the
//! catalogs at compile time (mirroring how [`patois::embed_domain!`] bundles
//! Paperback's own translations) and hand the bytes to wxWidgets on demand.
//!
//! The generated `wx_translations.rs` (produced by `build.rs`) provides:
//! - `wx_catalog(lang) -> Option<&'static [u8]>`
//! - `wx_available_languages() -> &'static [&'static str]`

use std::borrow::Cow;

use wxdragon::translations::TranslationsLoader;

include!(concat!(env!("OUT_DIR"), "/wx_translations.rs"));

/// wxWidgets requests the versioned domain (`wxstd-3.3`) first and falls back to
/// the unversioned `wxstd`. Match both so we stay correct across wx versions.
fn is_wxstd_domain(domain: &str) -> bool {
	domain == "wxstd" || domain.starts_with("wxstd-")
}

/// A [`TranslationsLoader`] that serves the embedded `wxstd` catalogs.
///
/// It is a zero-sized type, so a fresh instance can be handed to every
/// `Translations` object without cost.
pub struct WxStdCatalogLoader;

impl TranslationsLoader for WxStdCatalogLoader {
	fn load_catalog(&self, domain: &str, lang: &str) -> Option<Cow<'_, [u8]>> {
		if !is_wxstd_domain(domain) {
			return None;
		}
		wx_catalog(lang).map(Cow::Borrowed)
	}

	fn available_translations(&self, domain: &str) -> Vec<String> {
		if !is_wxstd_domain(domain) {
			return Vec::new();
		}
		wx_available_languages().iter().map(|&code| code.to_string()).collect()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	/// The `.mo` magic number, little- and big-endian (see GNU gettext format).
	const MO_MAGIC_LE: [u8; 4] = [0xde, 0x12, 0x04, 0x95];
	const MO_MAGIC_BE: [u8; 4] = [0x95, 0x04, 0x12, 0xde];

	fn is_mo(bytes: &[u8]) -> bool {
		bytes.len() >= 4 && (bytes[..4] == MO_MAGIC_LE || bytes[..4] == MO_MAGIC_BE)
	}

	/// wxWidgets only produces its catalog tree when gettext is available at build
	/// time. When it isn't (e.g. CI without gettext), `build.rs` emits an empty set
	/// and the crate still builds. The content tests below can't assert on catalogs
	/// that were never generated, so they no-op in that case; on a full dev build
	/// (with wx catalogs present) they run their real assertions.
	fn catalogs_available() -> bool {
		!wx_available_languages().is_empty()
	}

	#[test]
	fn dutch_catalog_is_embedded_and_valid() {
		if !catalogs_available() {
			return;
		}
		let langs = wx_available_languages();
		assert!(langs.contains(&"nl"), "Dutch catalog missing from embedded set: {langs:?}");
		let bytes = wx_catalog("nl").expect("wx_catalog(\"nl\") returned None");
		assert!(is_mo(bytes), "embedded nl catalog is not a valid .mo file");
	}

	#[test]
	fn available_languages_matches_catalog_lookup() {
		for &lang in wx_available_languages() {
			assert!(wx_catalog(lang).is_some(), "language {lang} listed but wx_catalog returned None");
		}
	}

	#[test]
	fn only_paperback_languages_are_embedded() {
		if !catalogs_available() {
			return;
		}
		let langs = wx_available_languages();
		// wx ships ~46 catalogs; we restrict to Paperback's own set, so languages
		// wx has but Paperback does not must be absent.
		assert!(!langs.contains(&"af"), "af is not a Paperback language but was embedded");
		assert!(!langs.contains(&"tr"), "tr is not a Paperback language but was embedded");
		// Paperback's pt_br maps to wx's pt_BR (case-insensitive match).
		assert!(langs.contains(&"pt_BR"), "pt_BR should be embedded: {langs:?}");
	}

	#[test]
	fn loader_scopes_to_wxstd_domain() {
		let loader = WxStdCatalogLoader;
		// Regardless of whether catalogs were embedded, a non-wxstd domain resolves
		// to nothing.
		assert!(loader.available_translations("paperback").is_empty());
		assert!(loader.load_catalog("paperback", "nl").is_none());
		if !catalogs_available() {
			return;
		}
		// With catalogs present, both the versioned and unversioned wxstd domains resolve.
		assert!(!loader.available_translations("wxstd").is_empty());
		assert!(!loader.available_translations("wxstd-3.3").is_empty());
	}

	#[test]
	fn loader_load_catalog_returns_mo_bytes() {
		if !catalogs_available() {
			return;
		}
		let loader = WxStdCatalogLoader;
		let bytes = loader.load_catalog("wxstd-3.3", "nl").expect("loader should serve nl for wxstd");
		assert!(is_mo(&bytes), "loader returned non-.mo bytes");
	}

	/// End-to-end proof that wxWidgets actually loads and translates through our
	/// loader, mirroring wxdragon's own `rust_loader_serves_embedded_catalog`.
	/// Runs headless (no wxApp), like that upstream test.
	#[test]
	fn wxwidgets_translates_via_embedded_catalog() {
		if !catalogs_available() {
			return;
		}
		use wxdragon::translations::Translations;

		let translations = Translations::new();
		translations.set_loader(WxStdCatalogLoader);
		translations.set_language_str("nl");
		assert!(translations.add_std_catalog(), "add_std_catalog should load the embedded wxstd catalog");
		// wxWidgets registers the catalog under the versioned domain it requests first,
		// so query that domain. "Cancel" differs in Dutch ("Annuleer"), unlike "OK".
		assert_eq!(
			translations.get_string("Cancel", "wxstd-3.3").as_deref(),
			Some("Annuleer"),
			"wx string should be translated to Dutch via the embedded catalog"
		);
	}
}
