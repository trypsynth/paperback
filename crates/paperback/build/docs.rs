//! Rendering the readmes into the HTML embedded in the binary for the Help menu, and generating the lookup table that maps a language code to it.

use std::{env, fmt::Write as _, fs, path::PathBuf};

use shipfitter::docs::{Page, bcp47, convert, readmes};

use crate::paths;

pub fn build() {
	let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap_or_default());
	let doc_dir = paths::workspace_dir().join("doc");
	println!("cargo:rerun-if-changed={}", doc_dir.display());
	// The readmes are embedded in the binary and nothing reads them from the install directory, so a build without them would ship an app whose Help menu does nothing. Fail instead.
	let readmes =
		readmes(&doc_dir).unwrap_or_else(|e| panic!("couldn't list the readmes in {}: {e}", doc_dir.display()));
	let mut code = String::from("pub fn readme_for_lang(lang: &str) -> Option<&'static [u8]> {\n    match lang {\n");
	for readme in &readmes {
		// The page's `lang`, so a screen reader reads a translated manual in that language's voice.
		let lang = bcp47(&readme.code);
		let page = Page { lang: &lang, title: "Paperback Documentation", ..Page::default() };
		let html = readme.html_name();
		convert(&readme.path, &out_dir.join(&html), &page).unwrap_or_else(|e| panic!("{e}"));
		let _ = writeln!(
			code,
			"        {:?} => Some(include_bytes!(concat!(env!(\"OUT_DIR\"), {:?}))),",
			readme.code,
			format!("/{html}")
		);
	}
	code.push_str("        _ => None,\n    }\n}\n");
	let _ = fs::write(out_dir.join("lang_readmes.rs"), code);
}
