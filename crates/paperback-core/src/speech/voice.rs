//! Which voice reads a book.

use super::Voice;

/// The first of `preferred` that's installed, such as the book's own voice and then the app's, else one for `language`, the book's language. `None` leaves the choice to the engine's default.
///
/// A voice for exactly `language`, such as `pt-BR`, beats one that only shares its language, such as `pt-PT`.
#[must_use]
pub fn choose_voice<'a>(voices: &'a [Voice], preferred: &[Option<&str>], language: Option<&str>) -> Option<&'a Voice> {
	preferred.iter().flatten().find_map(|id| voices.iter().find(|voice| voice.id == *id)).or_else(|| {
		let language = normalize(language?);
		let base = |tag: &str| tag.split('-').next().unwrap_or_default().to_owned();
		voices
			.iter()
			.find(|voice| normalize(&voice.language) == language)
			.or_else(|| voices.iter().find(|voice| base(&normalize(&voice.language)) == base(&language)))
	})
}

/// Tags are compared without regard to case or `_`, as platforms write them both ways.
fn normalize(tag: &str) -> String {
	tag.replace('_', "-").to_lowercase()
}

#[cfg(test)]
mod tests {
	use super::*;

	fn voices() -> Vec<Voice> {
		[("david", "en-US"), ("maria", "pt_PT"), ("helena", "pt-BR"), ("petteri", "fi-FI")]
			.map(|(id, language)| Voice { id: id.into(), name: id.into(), language: language.into() })
			.to_vec()
	}

	fn chosen(preferred: &[Option<&str>], language: Option<&str>) -> Option<String> {
		choose_voice(&voices(), preferred, language).map(|voice| voice.id.clone())
	}

	#[test]
	fn the_books_voice_comes_first_then_the_apps() {
		assert_eq!(chosen(&[Some("petteri"), Some("david")], Some("en")), Some("petteri".into()));
		assert_eq!(chosen(&[None, Some("david")], Some("fi")), Some("david".into()));
	}

	#[test]
	fn an_uninstalled_voice_is_skipped() {
		assert_eq!(chosen(&[Some("gone"), Some("david")], None), Some("david".into()));
	}

	#[test]
	fn otherwise_a_voice_for_the_books_language_is_found() {
		assert_eq!(chosen(&[None, None], Some("pt-BR")), Some("helena".into()));
		assert_eq!(chosen(&[], Some("PT_pt")), Some("maria".into()));
		assert_eq!(chosen(&[], Some("fi")), Some("petteri".into()));
		assert_eq!(chosen(&[], Some("de")), None);
		assert_eq!(chosen(&[], None), None);
	}
}
