use wxdragon::prelude::*;

use crate::ui::{
	commands::{self, Enable},
	menu_ids,
};

/// Commands that need a document open and have not moved to the command table yet. The ones
/// that have declare `Enable::HasDocument` instead, and are handled below through
/// `commands::apply_enable`.
const DOCUMENT_DEPENDENT_IDS: &[i32] = &[
	menu_ids::FIND,
	menu_ids::FIND_NEXT,
	menu_ids::FIND_PREVIOUS,
	menu_ids::GO_TO_LINE,
	menu_ids::GO_TO_PERCENT,
	menu_ids::GO_TO_PAGE,
	menu_ids::WORD_COUNT,
	menu_ids::DOCUMENT_INFO,
	menu_ids::TABLE_OF_CONTENTS,
	menu_ids::ELEMENTS_LIST,
	menu_ids::REVEAL_FILE_IN_FOLDER,
	menu_ids::OPEN_IN_WEB_VIEW,
	menu_ids::VIEW_SOURCE,
	menu_ids::IMPORT_DOCUMENT_DATA,
	menu_ids::EXPORT_DOCUMENT_DATA,
	menu_ids::EXPORT_TO_PLAIN_TEXT,
];

pub fn update_menu_item_states(frame: &Frame, has_document: bool) {
	let Some(menu_bar) = frame.get_menu_bar() else {
		return;
	};
	for &id in DOCUMENT_DEPENDENT_IDS {
		menu_bar.enable_item(id, has_document);
	}
	commands::apply_enable(frame, Enable::HasDocument, has_document);
}

pub fn update_reopen_state(frame: &Frame, has_recently_closed: bool) {
	commands::apply_enable(frame, Enable::HasRecentlyClosed, has_recently_closed);
}
