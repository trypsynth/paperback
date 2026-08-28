//! Filling in `paperback.iss.in`, including the per-format association tasks and the registry
//! entries that back them.

use std::{env, fs};

use crate::paths::{self, target_profile_dir};

/// Turns a format's display name into an Inno Setup task identifier, e.g.
/// "Flat `OpenDocument` Presentations" -> "`flat_opendocument_presentations`".
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

/// Builds the `[Tasks]` lines offering to associate each format from `paperback_formats::ALL`.
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

/// Builds the `[Registry]` lines wiring each format's extensions to the `assoc_*` tasks above.
/// The shared `zip` extension is skipped here; it's handled by its own standalone task since
/// multiple parsers (DAISY, Word-in-zip) register it and it shouldn't be tied to just one checkbox.
fn format_registry_block() -> String {
	let mut lines = Vec::new();
	for format in paperback_formats::ALL {
		let task = format!("assoc_{}", slugify(format.name));
		lines.push(format!("\t; {}.", format.name));
		for ext in format.extensions {
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
	println!("cargo:rerun-if-changed={}", input_path.display());
	if !input_path.exists() {
		return;
	}
	let content = match fs::read_to_string(&input_path) {
		Ok(c) => c,
		Err(e) => {
			println!("cargo:warning=Failed to read installer script: {e}");
			return;
		}
	};
	let version = env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "0.0.0".to_string());
	let (arch_allowed, arch_mode) = match env::var("CARGO_CFG_TARGET_ARCH").as_deref() {
		Ok("aarch64") => ("arm64", "arm64"),
		_ => ("x64compatible", "x64compatible"),
	};
	let new_content = content
		.replace("@PROJECT_VERSION@", &version)
		.replace("@ARCH_ALLOWED@", arch_allowed)
		.replace("@ARCH_MODE@", arch_mode)
		.replace("@FORMAT_TASKS@", &format_tasks_block())
		.replace("@FORMAT_REGISTRY@", &format_registry_block());
	let output_path = target_dir.join("paperback.iss");
	if let Err(e) = fs::write(&output_path, new_content) {
		println!("cargo:warning=Failed to write installer script: {e}");
	}
}
