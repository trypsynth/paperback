use std::{
	env,
	fs::File,
	io::BufReader,
	path::{Path, PathBuf},
	rc::Rc,
	sync::Mutex,
};

use paperback_core::{
	config::ConfigManager,
	parser::{PASSWORD_REQUIRED_ERROR_PREFIX, parser_supports_extension, parser_supports_path},
	util::{
		rar,
		zip::{extract_zip_entry_to_cache, zip_books_to_browse},
	},
};
use patois::t;
use wxdragon::prelude::*;

use super::dialogs;
use crate::ui::{dialogs::ZipChoice, document_manager::prompt_for_password};

pub fn choose_format(frame: &Frame, path: &Path, config: &Rc<Mutex<ConfigManager>>) -> bool {
	ensure_parser_for_unknown_file(frame, path, config)
}

pub(crate) fn ensure_parser_ready_for_path(frame: &Frame, path: &Path, config: &Rc<Mutex<ConfigManager>>) -> bool {
	let extension = parser_extension_for_path(path);
	if extension.is_empty() || parser_supports_path(path) {
		return true;
	}
	ensure_parser_for_unknown_file(frame, path, config)
}

/// Runs before the document manager is locked, since the picker is modal.
pub(crate) fn resolve_zip_path(
	frame: &Frame,
	path: &Path,
	config: &Rc<Mutex<ConfigManager>>,
	quit_if_closed: bool,
) -> Option<PathBuf> {
	let is_rar = path.extension().is_some_and(|ext| ext.eq_ignore_ascii_case("rar"));
	// A format the user forced for this zip means they want it opened as one document, not browsed.
	if !(is_rar || path.extension().is_some_and(|ext| ext.eq_ignore_ascii_case("zip")))
		|| !config.lock().unwrap().get_document_format(&path.to_string_lossy()).is_empty()
	{
		return Some(path.to_path_buf());
	}
	// A RAR is opened as its zip copy, so it gets the picker and everything else a zip does.
	let path = if is_rar { rar_copy(frame, path, config)? } else { path.to_path_buf() };
	browse_zip(frame, &path, quit_if_closed)
}

/// Asks for the password when the archive needs one, and keeps it as a document's password is kept, in case the copy has to be made again.
fn rar_copy(frame: &Frame, path: &Path, config: &Rc<Mutex<ConfigManager>>) -> Option<PathBuf> {
	let path_str = path.to_string_lossy();
	let saved = config.lock().unwrap().get_document_password(&path_str);
	let mut password = (!saved.is_empty()).then_some(saved);
	loop {
		match rar::zip_copy(path, password.as_deref(), &rar::cache_root()) {
			Ok(copy) => {
				if let Some(password) = &password {
					config.lock().unwrap().set_document_password(&path_str, password);
				}
				return Some(copy);
			}
			Err(err) if err.to_string().starts_with(PASSWORD_REQUIRED_ERROR_PREFIX) => {
				password = Some(prompt_for_password(frame)?);
			}
			Err(err) => {
				show_extract_error(frame, path, &err);
				return None;
			}
		}
	}
}

/// Skips the saved-format check, for a downloaded zip whose settings are kept under its link.
pub fn browse_zip(frame: &Frame, path: &Path, quit_if_closed: bool) -> Option<PathBuf> {
	if !path.extension().is_some_and(|ext| ext.eq_ignore_ascii_case("zip")) {
		return Some(path.to_path_buf());
	}
	let Some(names) = File::open(path)
		.ok()
		.and_then(|file| zip::ZipArchive::new(BufReader::new(file)).ok())
		.and_then(|archive| zip_books_to_browse(&archive))
	else {
		return Some(path.to_path_buf());
	};
	let chosen = match dialogs::show_zip_entries_dialog(frame, &names) {
		ZipChoice::Entry(chosen) => chosen,
		ZipChoice::Cancelled => return None,
		ZipChoice::Closed => {
			if quit_if_closed {
				// Queued so the frame closes from the event loop, not from inside this open call.
				wxdragon::call_after(Box::new(|| {
					if let Some(window) = crate::ui::app::main_window_from_ptr() {
						window.frame().close(false);
					}
				}));
			}
			return None;
		}
	};
	// Temp, not the config dir: extracted books are disposable and the OS cleans it.
	let cache = env::temp_dir().join("paperback-zip-cache");
	match extract_zip_entry_to_cache(path, &names[chosen], &cache) {
		Ok(extracted) => Some(extracted),
		Err(err) => {
			show_extract_error(frame, path, &err);
			None
		}
	}
}

fn show_extract_error(frame: &Frame, path: &Path, err: &anyhow::Error) {
	tracing::error!(path = %path.display(), error = %err, "failed to extract archive entry");
	// TRANSLATORS: Error shown when a document inside a zip archive can't be extracted; {} is the underlying error
	let message = t("Couldn't extract the document from the archive: {}").replace("{}", &err.to_string());
	// TRANSLATORS: Generic error dialog title
	let title = t("Error");
	MessageDialog::builder(frame, &message, &title)
		.with_style(MessageDialogStyle::OK | MessageDialogStyle::IconError | MessageDialogStyle::Centre)
		.build()
		.show_modal();
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

/// The config is locked only around reading and saving the format, not while the dialog is up: a handler in the dialog's event loop that locks it would otherwise wait on this frame forever, which is what opening an unknown file from the command line did.
fn ensure_parser_for_unknown_file(parent: &Frame, path: &Path, config: &Rc<Mutex<ConfigManager>>) -> bool {
	let path_str = path.to_string_lossy();
	let saved_format = config.lock().unwrap().get_document_format(&path_str);
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
	config.lock().unwrap().set_document_format(&path_str, &format);
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
