use wx_utils::seq_ids;
use wxdragon::id::{ID_ABOUT, ID_EXIT, ID_HIGHEST};

pub const EXIT: i32 = ID_EXIT;
pub const ABOUT: i32 = ID_ABOUT;
#[allow(clippy::cast_possible_truncation)]
pub const PREFERENCES: i32 = wxdragon::ffi::WXD_ID_PREFERENCES as i32;

// Real stock ids so wxWidgets binds each to its native macOS selector with a nil target, routing it through the responder chain to the focused control (and letting AppKit add Emoji & Symbols and Dictation).
#[cfg(target_os = "macos")]
pub use edit_ids::*;
#[cfg(target_os = "macos")]
mod edit_ids {
	#![allow(clippy::cast_possible_truncation)]
	use wxdragon::ffi;
	pub const UNDO: i32 = ffi::WXD_ID_UNDO as i32;
	pub const REDO: i32 = ffi::WXD_ID_REDO as i32;
	pub const CUT: i32 = ffi::WXD_ID_CUT as i32;
	pub const PASTE: i32 = ffi::WXD_ID_PASTE as i32;
	pub const DELETE: i32 = ffi::WXD_ID_CLEAR as i32;
}

#[allow(clippy::cast_possible_truncation)]
pub const COPY: i32 = wxdragon::ffi::WXD_ID_COPY as i32;
#[allow(clippy::cast_possible_truncation)]
pub const SELECT_ALL: i32 = wxdragon::ffi::WXD_ID_SELECTALL as i32;

// Above wxWidgets' stock ids, which run from 5000 to 5999. One of ours landing on a stock id takes on its meaning: on macOS wxID_CUT, wxID_COPY, wxID_PASTE and wxID_CLEAR are bound to the native cut:, copy:, paste: and delete: actions instead of to our handler.
const BASE: i32 = ID_HIGHEST + 1;

seq_ids!(BASE => OPEN, CLOSE, CLOSE_ALL, SHOW_ALL_DOCUMENTS, REOPEN_LAST_CLOSED, CLEAR_RECENT_DOCUMENTS, RELOAD, OPEN_FROM_URL);

pub const RECENT_DOCUMENT_BASE: i32 = BASE + 100;
pub const RECENT_DOCUMENT_MAX: i32 = BASE + 199;

seq_ids!(BASE + 200 => FIND, FIND_NEXT, FIND_PREVIOUS, ANNOUNCE_PERCENT, SET_TEMPORARY_BOOKMARK, JUMP_TO_TEMPORARY_BOOKMARK);

seq_ids!(BASE + 210 => GO_TO_LINE, GO_TO_PERCENT, GO_TO_PAGE);

seq_ids!(BASE + 220 => GO_BACK, GO_FORWARD);

seq_ids!(BASE + 230 => PREVIOUS_SECTION, NEXT_SECTION);

seq_ids!(BASE + 240 => PREVIOUS_HEADING, NEXT_HEADING);
seq_ids!(BASE + 250 =>
	PREVIOUS_HEADING_1, NEXT_HEADING_1,
	PREVIOUS_HEADING_2, NEXT_HEADING_2,
	PREVIOUS_HEADING_3, NEXT_HEADING_3,
	PREVIOUS_HEADING_4, NEXT_HEADING_4,
	PREVIOUS_HEADING_5, NEXT_HEADING_5,
	PREVIOUS_HEADING_6, NEXT_HEADING_6,
);

seq_ids!(BASE + 270 => PREVIOUS_PAGE, NEXT_PAGE);

seq_ids!(BASE + 280 =>
	PREVIOUS_BOOKMARK, NEXT_BOOKMARK,
	PREVIOUS_NOTE, NEXT_NOTE,
	JUMP_TO_ALL_BOOKMARKS, JUMP_TO_BOOKMARKS_ONLY, JUMP_TO_NOTES_ONLY,
	VIEW_NOTE_TEXT,
);

seq_ids!(BASE + 290 => PREVIOUS_LINK, NEXT_LINK);

seq_ids!(BASE + 295 => PREVIOUS_IMAGE, NEXT_IMAGE);

seq_ids!(BASE + 300 => PREVIOUS_FIGURE, NEXT_FIGURE);

seq_ids!(BASE + 305 => PREVIOUS_TABLE, NEXT_TABLE, PREVIOUS_SEPARATOR, NEXT_SEPARATOR);
seq_ids!(BASE + 310 => PREVIOUS_LIST, NEXT_LIST, PREVIOUS_LIST_ITEM, NEXT_LIST_ITEM);
seq_ids!(BASE + 314 => CONTAINER_START, CONTAINER_END);
seq_ids!(BASE + 316 => PREVIOUS_FORMULA, NEXT_FORMULA);

seq_ids!(BASE + 400 =>
	WORD_COUNT, DOCUMENT_INFO, TABLE_OF_CONTENTS, ELEMENTS_LIST,
	REVEAL_FILE_IN_FOLDER, OPEN_IN_WEB_VIEW, VIEW_SOURCE,
);

seq_ids!(BASE + 410 => IMPORT_DOCUMENT_DATA, EXPORT_DOCUMENT_DATA, EXPORT_TO_PLAIN_TEXT, EXPORT_TO_HTML, EXPORT_TO_MARKDOWN, EXPORT_NOTES_AND_BOOKMARKS);

seq_ids!(BASE + 420 => TOGGLE_BOOKMARK, BOOKMARK_WITH_NOTE);

seq_ids!(BASE + 430 => OPTIONS, SLEEP_TIMER, CUSTOMIZE_SHORTCUTS);

seq_ids!(BASE + 440 => TOGGLE_WORD_WRAP, TOGGLE_FULL_SCREEN);

seq_ids!(BASE + 450 =>
	PLAY_PAUSE_AUDIO, SEEK_AUDIO_FORWARD, SEEK_AUDIO_BACKWARD,
	INCREASE_AUDIO_SEEK_AMOUNT, DECREASE_AUDIO_SEEK_AMOUNT,
	INCREASE_AUDIO_SPEED, DECREASE_AUDIO_SPEED, INCREASE_AUDIO_VOLUME, DECREASE_AUDIO_VOLUME,
);

seq_ids!(BASE + 460 => BATCH_OCR, RECOGNIZE_CURRENT_PAGE);

seq_ids!(BASE + 470 => SET_SELECTION_START, COPY_FROM_SELECTION_START, JUMP_TO_SELECTION_START);

seq_ids!(BASE + 480 => SAVE_IMAGE, READ_IMAGE_TEXT);

seq_ids!(BASE + 500 => VIEW_HELP_BROWSER, VIEW_HELP_PAPERBACK, CHECK_FOR_UPDATES, DONATE);

seq_ids!(BASE + 900 => RESTORE);

pub const fn action_to_menu_id(action: paperback_core::config::ActionId) -> i32 {
	use paperback_core::config::ActionId;
	match action {
		ActionId::Open => OPEN,
		ActionId::OpenFromUrl => OPEN_FROM_URL,
		ActionId::Close => CLOSE,
		ActionId::CloseAll => CLOSE_ALL,
		ActionId::ReopenLastClosed => REOPEN_LAST_CLOSED,
		ActionId::Reload => RELOAD,
		ActionId::ShowAllRecentDocuments => SHOW_ALL_DOCUMENTS,
		ActionId::ClearRecentDocuments => CLEAR_RECENT_DOCUMENTS,
		ActionId::Exit => EXIT,
		ActionId::Find => FIND,
		ActionId::FindNext => FIND_NEXT,
		ActionId::FindPrevious => FIND_PREVIOUS,
		ActionId::GoToLine => GO_TO_LINE,
		ActionId::GoToPercent => GO_TO_PERCENT,
		ActionId::GoToPage => GO_TO_PAGE,
		ActionId::GoBack => GO_BACK,
		ActionId::GoForward => GO_FORWARD,
		ActionId::AnnouncePercent => ANNOUNCE_PERCENT,
		ActionId::SetTemporaryBookmark => SET_TEMPORARY_BOOKMARK,
		ActionId::JumpToTemporaryBookmark => JUMP_TO_TEMPORARY_BOOKMARK,
		ActionId::PreviousSection => PREVIOUS_SECTION,
		ActionId::NextSection => NEXT_SECTION,
		ActionId::PreviousHeading => PREVIOUS_HEADING,
		ActionId::NextHeading => NEXT_HEADING,
		ActionId::PreviousHeading1 => PREVIOUS_HEADING_1,
		ActionId::NextHeading1 => NEXT_HEADING_1,
		ActionId::PreviousHeading2 => PREVIOUS_HEADING_2,
		ActionId::NextHeading2 => NEXT_HEADING_2,
		ActionId::PreviousHeading3 => PREVIOUS_HEADING_3,
		ActionId::NextHeading3 => NEXT_HEADING_3,
		ActionId::PreviousHeading4 => PREVIOUS_HEADING_4,
		ActionId::NextHeading4 => NEXT_HEADING_4,
		ActionId::PreviousHeading5 => PREVIOUS_HEADING_5,
		ActionId::NextHeading5 => NEXT_HEADING_5,
		ActionId::PreviousHeading6 => PREVIOUS_HEADING_6,
		ActionId::NextHeading6 => NEXT_HEADING_6,
		ActionId::PreviousPage => PREVIOUS_PAGE,
		ActionId::NextPage => NEXT_PAGE,
		ActionId::PreviousBookmark => PREVIOUS_BOOKMARK,
		ActionId::NextBookmark => NEXT_BOOKMARK,
		ActionId::PreviousNote => PREVIOUS_NOTE,
		ActionId::NextNote => NEXT_NOTE,
		ActionId::JumpToAllBookmarks => JUMP_TO_ALL_BOOKMARKS,
		ActionId::JumpToBookmarksOnly => JUMP_TO_BOOKMARKS_ONLY,
		ActionId::JumpToNotesOnly => JUMP_TO_NOTES_ONLY,
		ActionId::ViewNoteText => VIEW_NOTE_TEXT,
		ActionId::PreviousLink => PREVIOUS_LINK,
		ActionId::NextLink => NEXT_LINK,
		ActionId::PreviousImage => PREVIOUS_IMAGE,
		ActionId::NextImage => NEXT_IMAGE,
		ActionId::PreviousFigure => PREVIOUS_FIGURE,
		ActionId::NextFigure => NEXT_FIGURE,
		ActionId::PreviousTable => PREVIOUS_TABLE,
		ActionId::NextTable => NEXT_TABLE,
		ActionId::PreviousFormula => PREVIOUS_FORMULA,
		ActionId::NextFormula => NEXT_FORMULA,
		ActionId::PreviousSeparator => PREVIOUS_SEPARATOR,
		ActionId::NextSeparator => NEXT_SEPARATOR,
		ActionId::PreviousList => PREVIOUS_LIST,
		ActionId::NextList => NEXT_LIST,
		ActionId::PreviousListItem => PREVIOUS_LIST_ITEM,
		ActionId::NextListItem => NEXT_LIST_ITEM,
		ActionId::ContainerStart => CONTAINER_START,
		ActionId::ContainerEnd => CONTAINER_END,
		ActionId::WordCount => WORD_COUNT,
		ActionId::DocumentInfo => DOCUMENT_INFO,
		ActionId::TableOfContents => TABLE_OF_CONTENTS,
		ActionId::ElementsList => ELEMENTS_LIST,
		ActionId::RevealFileInFolder => REVEAL_FILE_IN_FOLDER,
		ActionId::OpenInWebView => OPEN_IN_WEB_VIEW,
		ActionId::ViewSource => VIEW_SOURCE,
		ActionId::ToggleBookmark => TOGGLE_BOOKMARK,
		ActionId::BookmarkWithNote => BOOKMARK_WITH_NOTE,
		ActionId::SetSelectionStart => SET_SELECTION_START,
		ActionId::CopyFromSelectionStart => COPY_FROM_SELECTION_START,
		ActionId::JumpToSelectionStart => JUMP_TO_SELECTION_START,
		ActionId::ToggleWordWrap => TOGGLE_WORD_WRAP,
		ActionId::PlayPauseAudio => PLAY_PAUSE_AUDIO,
		ActionId::SeekAudioForward => SEEK_AUDIO_FORWARD,
		ActionId::SeekAudioBackward => SEEK_AUDIO_BACKWARD,
		ActionId::IncreaseAudioSeekAmount => INCREASE_AUDIO_SEEK_AMOUNT,
		ActionId::DecreaseAudioSeekAmount => DECREASE_AUDIO_SEEK_AMOUNT,
		ActionId::IncreaseAudioSpeed => INCREASE_AUDIO_SPEED,
		ActionId::DecreaseAudioSpeed => DECREASE_AUDIO_SPEED,
		ActionId::IncreaseAudioVolume => INCREASE_AUDIO_VOLUME,
		ActionId::DecreaseAudioVolume => DECREASE_AUDIO_VOLUME,
		ActionId::ToggleFullScreen => TOGGLE_FULL_SCREEN,
		ActionId::Options => OPTIONS,
		ActionId::SleepTimer => SLEEP_TIMER,
		ActionId::BatchOcr => BATCH_OCR,
		ActionId::RecognizeCurrentPage => RECOGNIZE_CURRENT_PAGE,
		ActionId::CustomizeShortcuts => CUSTOMIZE_SHORTCUTS,
		ActionId::ImportDocumentData => IMPORT_DOCUMENT_DATA,
		ActionId::ExportDocumentData => EXPORT_DOCUMENT_DATA,
		ActionId::ExportNotesAndBookmarks => EXPORT_NOTES_AND_BOOKMARKS,
		ActionId::ExportToPlainText => EXPORT_TO_PLAIN_TEXT,
		ActionId::ExportToHtml => EXPORT_TO_HTML,
		ActionId::ExportToMarkdown => EXPORT_TO_MARKDOWN,
		ActionId::About => ABOUT,
		ActionId::ViewHelpBrowser => VIEW_HELP_BROWSER,
		ActionId::ViewHelpPaperback => VIEW_HELP_PAPERBACK,
		ActionId::CheckForUpdates => CHECK_FOR_UPDATES,
		ActionId::Donate => DONATE,
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn clear_recent_documents_has_a_file_menu_id() {
		use paperback_core::config::ActionId;
		let id = action_to_menu_id(ActionId::ClearRecentDocuments);
		assert_eq!(id, CLEAR_RECENT_DOCUMENTS);
		assert!(id < RECENT_DOCUMENT_BASE);
	}

	/// The recent-document entries start at `BASE + 100`, so a File item past them would be enabled and disabled along with the recent list.
	#[test]
	fn reload_has_a_file_menu_id() {
		use paperback_core::config::ActionId;
		let id = action_to_menu_id(ActionId::Reload);
		assert_eq!(id, RELOAD);
		assert!(id < RECENT_DOCUMENT_BASE);
	}

	/// Only actions meant to be stock items may have a stock id: one that landed on `wxID_CUT` became the native cut on macOS.
	#[test]
	fn only_stock_actions_have_wxwidgets_ids() {
		use paperback_core::config::ActionId;
		for action in ActionId::all() {
			if matches!(action, ActionId::Exit | ActionId::About) {
				continue;
			}
			let id = action_to_menu_id(*action);
			assert!(!(5000..=ID_HIGHEST).contains(&id), "{action:?} has {id}, a wxWidgets stock id");
		}
	}
}
