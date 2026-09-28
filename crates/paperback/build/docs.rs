//! Running the readmes through pandoc and generating the lookup table that maps a language code
//! to the HTML embedded in the binary for the Help menu.

use std::{
	env,
	fmt::Write as _,
	fs::{self, DirEntry},
	path::PathBuf,
};

use shipfitter::build::{pandoc, pandoc_available};

use crate::paths;

pub fn build() {
	let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap_or_default());
	let doc_dir = paths::workspace_dir().join("doc");
	let readme = doc_dir.join("readme.md");
	let config = doc_dir.join("pandoc.yaml");
	// The readmes are embedded in the binary and nothing reads them from the install directory,
	// so a build without them would ship an app whose Help menu does nothing. Fail instead.
	assert!(
		pandoc_available(),
		"pandoc is required to build Paperback: it converts doc/readme*.md into the help shown by \
		 the Help menu, which is embedded in the binary. Install it and build again."
	);
	let mut embedded_langs: Vec<String> = Vec::new();
	{
		if let Err(e) = pandoc(&readme, &config, &out_dir.join("readme.html"), Some("en")) {
			panic!("{e}");
		}
		embedded_langs.push("en".to_string());
		if let Ok(entries) = fs::read_dir(&doc_dir) {
			let mut doc_entries: Vec<_> = entries.flatten().collect();
			doc_entries.sort_by_key(DirEntry::file_name);
			for entry in doc_entries {
				let path = entry.path();
				if path.extension().and_then(|e| e.to_str()) != Some("md") {
					continue;
				}
				let stem = match path.file_stem().and_then(|s| s.to_str()) {
					Some(s) => s.to_string(),
					None => continue,
				};
				let lang_code = match stem.strip_prefix("readme-") {
					Some(code) if !code.is_empty() => code.to_string(),
					_ => continue,
				};
				let lang_output = out_dir.join(format!("readme-{lang_code}.html"));
				// Pandoc needs the language as BCP 47 to emit a usable `lang` attribute, but our
				// locale codes use gettext's underscore form (zh_CN). Without this metadata every
				// readme ships with `lang=""`, so a screen reader has no idea which language the
				// help is in and may read a translated page with an English voice.
				let bcp47 = lang_code.replace('_', "-");
				if let Err(e) = pandoc(&path, &config, &lang_output, Some(&bcp47)) {
					panic!("{e}");
				}
				embedded_langs.push(lang_code);
			}
		}
	}
	let code = {
		let mut code =
			String::from("pub fn readme_for_lang(lang: &str) -> Option<&'static [u8]> {\n    match lang {\n");
		for lang_code in &embedded_langs {
			let filename =
				if lang_code == "en" { "/readme.html".to_string() } else { format!("/readme-{lang_code}.html") };
			let _ = writeln!(
				code,
				"        {lang_code:?} => Some(include_bytes!(concat!(env!(\"OUT_DIR\"), {filename:?}))),",
			);
		}
		code.push_str("        _ => None,\n    }\n}\n");
		code
	};
	let _ = fs::write(out_dir.join("lang_readmes.rs"), code);
}
