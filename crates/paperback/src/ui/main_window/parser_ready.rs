//! Confirming a parser exists for a file before opening it: known extensions pass straight
//! through, while an unrecognized one prompts the user via the "Open As" dialog (remembering
//! their choice per-path) unless a previous choice for that path is already on record.

use std::{path::Path, rc::Rc, sync::Mutex};

use paperback_core::{config::ConfigManager, parser::parser_supports_extension};
use patois::t;
use wxdragon::prelude::*;

use super::dialogs;

pub(super) fn ensure_parser_ready_for_path(frame: &Frame, path: &Path, config: &Rc<Mutex<ConfigManager>>) -> bool {
	let extension = parser_extension_for_path(path);
	if extension.is_empty() || parser_supports_extension(&extension) {
		return true;
	}
	let cfg = config.lock().unwrap();
	ensure_parser_for_unknown_file(frame, path, &cfg)
}

fn parser_extension_for_path(path: &Path) -> String {
	let from_path = path.extension().and_then(|ext| ext.to_str()).map(clean_extension_token).unwrap_or_default();
	if !from_path.is_empty() {
		return from_path;
	}
	// Fallback for odd IPC/CLI strings that may contain trailing quotes or whitespace.
	let raw = path.to_string_lossy();
	let cleaned = raw.trim().trim_matches(['"', '\'', '\0']);
	let candidate = cleaned
		.rsplit_once(['/', '\\'])
		.map_or(cleaned, |(_, file_name)| file_name)
		.rsplit_once('.')
		.map_or("", |(_, ext)| ext)
		.trim();
	clean_extension_token(candidate)
}

fn clean_extension_token(raw: &str) -> String {
	let trimmed = raw.trim().trim_matches(['"', '\'', '\0']);
	trimmed.chars().take_while(char::is_ascii_alphanumeric).collect()
}

fn ensure_parser_for_unknown_file(parent: &Frame, path: &Path, config: &ConfigManager) -> bool {
	let path_str = path.to_string_lossy();
	let saved_format = config.get_document_format(&path_str);
	if !saved_format.is_empty() && parser_supports_extension(&saved_format) {
		return true;
	}
	let Some(format) = dialogs::show_open_as_dialog(parent, path) else {
		return false;
	};
	if !parser_supports_extension(&format) {
		// TRANSLATORS: Error shown when the user picks a file format from the "Open As" dialog that this parser build doesn't support
		let message = t("Unsupported format selected.");
		// TRANSLATORS: Generic error dialog title
		let title = t("Error");
		let dialog = MessageDialog::builder(parent, &message, &title)
			.with_style(MessageDialogStyle::OK | MessageDialogStyle::IconError | MessageDialogStyle::Centre)
			.build();
		dialog.show_modal();
		return false;
	}
	config.set_document_format(&path_str, &format);
	true
}

#[cfg(test)]
mod tests {
	use std::path::Path;

	use super::parser_extension_for_path;

	#[test]
	fn parser_extension_for_path_handles_normal_paths() {
		assert_eq!(parser_extension_for_path(Path::new("book.epub")), "epub");
		assert_eq!(parser_extension_for_path(Path::new("C:\\docs\\book.PDF")), "PDF");
	}

	#[test]
	fn parser_extension_for_path_strips_quotes_and_whitespace() {
		assert_eq!(parser_extension_for_path(Path::new("  \"book.epub\"  ")), "epub");
		assert_eq!(parser_extension_for_path(Path::new("'book.txt'")), "txt");
	}

	#[test]
	fn parser_extension_for_path_returns_empty_for_no_extension() {
		assert_eq!(parser_extension_for_path(Path::new("README")), "");
	}

	#[test]
	fn parser_extension_for_path_handles_ipc_artifacts() {
		assert_eq!(parser_extension_for_path(Path::new("book.epub\u{0}")), "epub");
		assert_eq!(parser_extension_for_path(Path::new(" \"book.epub\u{0}\" ")), "epub");
	}
}
