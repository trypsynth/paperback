use shipfitter::build::configure_file;

use crate::paths::{self, target_profile_dir};

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

fn format_tasks_block() -> String {
	let mut lines = Vec::new();
	for format in paperback_formats::ALL {
		let ext_list = format.extensions.iter().map(|ext| format!("*.{ext}")).collect::<Vec<_>>().join(", ");
		let flags = if format.installer.default_checked { String::new() } else { "; Flags: unchecked".to_string() };
		lines.push(format!(
			"\tName: \"assoc_{}\"; Description: \"Associate with {} ({})\"{}",
			slugify(format.name),
			format.name,
			ext_list,
			flags
		));
	}
	lines.join("\n")
}

fn format_registry_block() -> String {
	let mut lines = Vec::new();
	for format in paperback_formats::ALL {
		let task = format!("assoc_{}", slugify(format.name));
		lines.push(format!("\t; {}.", format.name));
		for ext in format.extensions {
			// DAISY and Word-in-zip both register zip, so it has its own standalone task rather than one format's checkbox.
			if *ext == "zip" {
				continue;
			}
			if format.installer.default_handler {
				lines.push(format!(
					"\tRoot: HKCR; Subkey: \".{ext}\"; ValueType: string; ValueName: \"\"; ValueData: \"Paperback.Document\"; Flags: uninsdeletevalue; Tasks: {task}"
				));
			} else {
				lines.push(format!(
					"\tRoot: HKCR; Subkey: \".{ext}\\OpenWithProgids\"; ValueType: string; ValueName: \"Paperback.Document\"; ValueData: \"\"; Flags: uninsdeletevalue; Tasks: {task}"
				));
			}
		}
	}
	lines.join("\n")
}

pub fn configure() {
	let Some(target_dir) = target_profile_dir() else {
		return;
	};
	let input_path = paths::workspace_dir().join("paperback.iss.in");
	if !input_path.exists() {
		return;
	}
	let replacements = [("FORMAT_TASKS", format_tasks_block()), ("FORMAT_REGISTRY", format_registry_block())];
	let replacements: Vec<(&str, &str)> = replacements.iter().map(|(name, value)| (*name, value.as_str())).collect();
	if let Err(e) = configure_file(&input_path, &target_dir.join("paperback.iss"), &replacements) {
		println!("cargo:warning=Failed to write installer script: {e}");
	}
}
