use std::{collections::HashMap, fs, path::Path};

use paperback_formats::FormatMeta;
use patois_build::po::PoDocument;
use shipfitter::build::configure_file;

use crate::{
	installer_strings::MESSAGES,
	paths::{self, target_profile_dir},
};

/// Each catalog, the Inno Setup messages file for it, and whether that file is a copy in `installer/languages`, for the four Inno Setup ships only as unofficial translations.
const LANGUAGES: [(&str, &str, bool); 17] = [
	("bs", "Bosnian", true),
	("cs", "Czech", false),
	("de", "German", false),
	("es", "Spanish", false),
	("fi", "Finnish", false),
	("fr", "French", false),
	("hu", "Hungarian", false),
	("ja", "Japanese", false),
	("nl", "Dutch", false),
	("pl", "Polish", false),
	("pt_br", "BrazilianPortuguese", false),
	("ru", "Russian", false),
	("sk", "Slovak", false),
	("sr", "SerbianLatin", true),
	("uk", "Ukrainian", false),
	("vi", "Vietnamese", true),
	("zh_CN", "ChineseSimplified", true),
];

fn slugify(name: &str) -> String {
	let mut slug = String::new();
	let mut last_was_sep = true;
	for ch in name.chars() {
		if ch.is_ascii_alphanumeric() {
			slug.push(ch.to_ascii_lowercase());
			last_was_sep = false;
		} else if !last_was_sep {
			slug.push('_');
			last_was_sep = true;
		}
	}
	if slug.ends_with('_') {
		slug.pop();
	}
	slug
}

/// Named apart from the old per-format `assoc_` tasks, so a box someone ticked for those years ago isn't carried over.
fn default_task(format: &FormatMeta) -> String {
	format!("default_{}", slugify(format.name))
}

fn format_tasks_block() -> String {
	let mut lines = Vec::new();
	for format in paperback_formats::ALL.iter().filter(|format| format.installer.offered_as_default) {
		let ext_list = paperback_formats::associated_extensions(format)
			.iter()
			.map(|ext| format!("*.{ext}"))
			.collect::<Vec<_>>()
			.join(", ");
		let task = default_task(format);
		assert!(
			MESSAGES.iter().any(|(name, _)| *name == task),
			"{} is offered as a default app, so installer_strings.rs needs a {task} message for its checkbox, naming {ext_list}",
			format.name
		);
		let flags = if format.installer.default_checked { "" } else { "; Flags: unchecked" };
		lines.push(format!("\tName: \"{task}\"; Description: \"{{cm:{task}}}\"{flags}"));
	}
	lines.join("\n")
}

/// Every extension gets Paperback in its "Open with" list, which takes nothing over; a format whose box is ticked also gets Paperback as its default. `.zip`, which several formats open, is listed once.
fn format_registry_block() -> String {
	let mut lines = Vec::new();
	let mut listed: Vec<&str> = Vec::new();
	for format in paperback_formats::ALL {
		lines.push(format!("\t; {}.", format.name));
		for ext in paperback_formats::associated_extensions(format) {
			if format.installer.offered_as_default && ext != "zip" {
				lines.push(format!(
					"\tRoot: HKCR; Subkey: \".{ext}\"; ValueType: string; ValueName: \"\"; ValueData: \"Paperback.Document\"; Flags: uninsdeletevalue; Tasks: {}",
					default_task(format)
				));
			}
			if listed.contains(&ext) {
				continue;
			}
			listed.push(ext);
			lines.push(format!(
				"\tRoot: HKCR; Subkey: \".{ext}\\OpenWithProgids\"; ValueType: string; ValueName: \"Paperback.Document\"; ValueData: \"\"; Flags: uninsdeletevalue"
			));
			lines.push(format!(
				"\tRoot: HKCR; Subkey: \"Applications\\paperback.exe\\SupportedTypes\"; ValueType: string; ValueName: \".{ext}\"; ValueData: \"\""
			));
		}
	}
	lines.join("\n")
}

fn languages_block() -> String {
	let vendored = paths::workspace_dir().join("installer").join("languages");
	let mut lines = vec!["\tName: \"en\"; MessagesFile: \"compiler:Default.isl\"".to_string()];
	for (code, inno, copied) in LANGUAGES {
		let file = if copied {
			vendored.join(format!("{inno}.isl")).display().to_string()
		} else {
			format!("compiler:Languages\\{inno}.isl")
		};
		lines.push(format!("\tName: \"{code}\"; MessagesFile: \"{file}\""));
	}
	lines.join("\n")
}

/// Machine translations are fuzzy and used, as the app uses them; a string a catalog lacks falls back to the English line, which names no language.
fn custom_messages_block() -> String {
	let po_dir = paths::workspace_dir().join("po");
	let mut lines: Vec<String> = MESSAGES.iter().map(|(name, english)| format!("{name}={english}")).collect();
	for (code, _, _) in LANGUAGES {
		let translations = catalog(&po_dir.join(format!("{code}.po")));
		for (name, english) in MESSAGES {
			if let Some(translated) = translations.get(*english) {
				lines.push(format!("{code}.{name}={}", translated.replace(['\r', '\n'], " ")));
			}
		}
	}
	lines.join("\n")
}

fn catalog(path: &Path) -> HashMap<String, String> {
	let Ok(content) = fs::read_to_string(path) else {
		println!("cargo:warning=No catalog at {} for the installer", path.display());
		return HashMap::new();
	};
	PoDocument::parse(&content)
		.entries
		.into_iter()
		.filter(|entry| !entry.msgid.is_empty() && !entry.msgstr.is_empty())
		.map(|entry| (entry.msgid, entry.msgstr))
		.collect()
}

pub fn configure() {
	let Some(target_dir) = target_profile_dir() else {
		return;
	};
	let input_path = paths::workspace_dir().join("paperback.iss.in");
	if !input_path.exists() {
		return;
	}
	let replacements = [
		("LANGUAGES", languages_block()),
		("CUSTOM_MESSAGES", custom_messages_block()),
		("FORMAT_TASKS", format_tasks_block()),
		("FORMAT_REGISTRY", format_registry_block()),
	];
	let replacements: Vec<(&str, &str)> = replacements.iter().map(|(name, value)| (*name, value.as_str())).collect();
	let output = target_dir.join("paperback.iss");
	if let Err(e) = configure_file(&input_path, &output, &replacements) {
		println!("cargo:warning=Failed to write installer script: {e}");
		return;
	}
	// Inno Setup reads a script without a byte order mark in the system's code page, which would garble every translation.
	if let Ok(script) = fs::read_to_string(&output)
		&& !script.starts_with('\u{FEFF}')
		&& let Err(e) = fs::write(&output, format!("\u{FEFF}{script}"))
	{
		println!("cargo:warning=Failed to write installer script: {e}");
	}
}
