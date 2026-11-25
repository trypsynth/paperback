use paperback_formats::FormatMeta;
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
		let flags = if format.installer.default_checked { "" } else { "; Flags: unchecked" };
		lines.push(format!(
			"\tName: \"{}\"; Description: \"Make Paperback the default app for {} ({ext_list})\"{flags}",
			default_task(format),
			format.name
		));
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
