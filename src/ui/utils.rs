use std::path::Path;

use bitflags::bitflags;
use wxdragon::{prelude::*, translations::translate as t};

use crate::{
	config::ConfigManager,
	parser::parser_supports_extension,
	reader_core,
	ui::dialogs,
};

#[derive(Clone, Debug, Default)]
pub struct SearchResult {
	pub found: bool,
	pub wrapped: bool,
	pub position: i64,
}

bitflags! {
	#[derive(Default)]
	pub struct FindOptions: u8 {
		const NONE = 0;
		const FORWARD = 1 << 0;
		const MATCH_CASE = 1 << 1;
		const MATCH_WHOLE_WORD = 1 << 2;
		const USE_REGEX = 1 << 3;
	}
}

pub fn find_text_with_wrap(haystack: &str, needle: &str, start: i64, options: FindOptions) -> SearchResult {
	if needle.is_empty() {
		return SearchResult::default();
	}
	let result = reader_core::reader_search_with_wrap(
		haystack,
		needle,
		start,
		options.contains(FindOptions::FORWARD),
		options.contains(FindOptions::MATCH_CASE),
		options.contains(FindOptions::MATCH_WHOLE_WORD),
		options.contains(FindOptions::USE_REGEX),
	);
	SearchResult { found: result.found, wrapped: result.wrapped, position: result.position }
}

pub fn ensure_parser_for_unknown_file(parent: &Frame, path: &Path, config: &mut ConfigManager) -> bool {
	let path_str = path.to_string_lossy();
	let saved_format = config.get_document_format(&path_str);
	if !saved_format.is_empty() && parser_supports_extension(&saved_format) {
		return true;
	}
	let Some(format) = dialogs::show_open_as_dialog(parent, path) else {
		return false;
	};
	if !parser_supports_extension(&format) {
		let message = t("Unsupported format selected.");
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
