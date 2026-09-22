//! Gathering the translator credits out of the committed `po/*.po` catalogs.
//!
//! Every catalog names whoever last worked on it in its `Last-Translator` header, which is the
//! only record of who did that work. Reading it here rather than keeping a list by hand means a
//! new translator is credited by the same commit that adds their language, and nobody has to
//! remember to add them a second time.

use std::{collections::BTreeSet, env, fs, path::PathBuf};

/// Catalogs whose header still carries the placeholder gettext writes for a catalog nobody has
/// claimed, which is not a person and must not be shown as one.
const PLACEHOLDERS: [&str; 2] = ["FULL NAME", "EMAIL@ADDRESS"];

pub fn build() {
	let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap_or_default());
	let mut names = BTreeSet::new();
	let Ok(entries) = fs::read_dir("../../po") else {
		let _ = fs::write(out_dir.join("translators.rs"), "pub static TRANSLATORS: &[&str] = &[];\n");
		return;
	};
	for entry in entries.flatten() {
		let path = entry.path();
		if path.extension().is_none_or(|ext| ext != "po") {
			continue;
		}
		println!("cargo:rerun-if-changed={}", path.display());
		let Ok(text) = fs::read_to_string(&path) else { continue };
		if let Some(name) = last_translator(&text) {
			names.insert(name);
		}
	}
	let mut code = String::from("pub static TRANSLATORS: &[&str] = &[\n");
	for name in &names {
		code.push_str(&format!("\t{name:?},\n"));
	}
	code.push_str("];\n");
	let _ = fs::write(out_dir.join("translators.rs"), code);
}

/// The person named in a catalog's `Last-Translator` header, without their email address.
///
/// The address is deliberately dropped: a credit in an About box is a thank-you, not a contact
/// list, and publishing addresses that people put in a file for maintainers is not the same as
/// publishing them in every shipped copy of the application.
fn last_translator(catalog: &str) -> Option<String> {
	let line = catalog.lines().find(|line| line.contains("Last-Translator:"))?;
	let after = line.split("Last-Translator:").nth(1)?;
	// The header is one quoted gettext string, so it carries a literal backslash-n and a closing
	// quote that are part of the file format rather than part of anybody's name.
	let name = after.split('<').next()?.replace("\\n", "").replace('"', "");
	let name = name.trim();
	// A name has letters in it. This rejects a header left empty, which would otherwise come
	// through as whatever punctuation the format left behind and be credited as a person.
	if !name.chars().any(char::is_alphabetic) || PLACEHOLDERS.iter().any(|placeholder| name.contains(placeholder)) {
		return None;
	}
	Some(name.to_string())
}

#[cfg(test)]
mod tests {
	use super::last_translator;

	#[test]
	fn the_name_is_taken_without_the_address() {
		let header = r#""Last-Translator: Michał Dziwisz <michal@dziwisz.net>\n""#;
		assert_eq!(Some("Michał Dziwisz".to_string()), last_translator(header));
	}

	/// An untouched catalog carries gettext's placeholder, which is not somebody's name.
	#[test]
	fn an_unclaimed_catalog_credits_nobody() {
		assert_eq!(None, last_translator(r#""Last-Translator: FULL NAME <EMAIL@ADDRESS>\n""#));
	}

	/// Some catalogs carry only an address, with no name in front of it.
	#[test]
	fn a_header_with_no_name_credits_nobody() {
		assert_eq!(None, last_translator(r#""Last-Translator: <someone@example.com>\n""#));
	}

	/// A catalog whose header was left empty, which is punctuation rather than a person. This is
	/// the one that got through: it reached the dialog as a credit reading "\n".
	#[test]
	fn an_empty_header_credits_nobody() {
		assert_eq!(None, last_translator(r#""Last-Translator: \n""#));
		assert_eq!(None, last_translator(r#""Last-Translator: ""#));
	}

	#[test]
	fn a_catalog_without_the_header_credits_nobody() {
		assert_eq!(None, last_translator("msgid \"\"\nmsgstr \"\"\n"));
	}
}
