use wx_utils::seq_ids;
use wxdragon::id::{ID_ABOUT, ID_EXIT};

// Re-export standard IDs
pub const EXIT: i32 = ID_EXIT;
pub const ABOUT: i32 = ID_ABOUT;
#[allow(clippy::cast_possible_truncation)]
pub const PREFERENCES: i32 = wxdragon::ffi::WXD_ID_PREFERENCES as i32;

// Standard wxWidgets IDs for the macOS Edit menu. They must be the real wxWidgets
// IDs (not custom ones) so wxWidgets binds each item to its native macOS selector
// — cut:, copy:, paste:, delete:, selectAll: — with a nil target. AppKit then
// routes the command through the responder chain to the focused control and, because
// a `copy:` item is present, appends its own "Emoji & Symbols" and "Start Dictation"
// items to the menu. Undo/Redo have no native selector mapping but are handled by
// wxTextCtrl when it has focus.
#[cfg(target_os = "macos")]
pub use edit_ids::*;
#[cfg(target_os = "macos")]
mod edit_ids {
	#![allow(clippy::cast_possible_truncation)]
	use wxdragon::ffi;
	pub const UNDO: i32 = ffi::WXD_ID_UNDO as i32;
	pub const REDO: i32 = ffi::WXD_ID_REDO as i32;
	pub const CUT: i32 = ffi::WXD_ID_CUT as i32;
	pub const COPY: i32 = ffi::WXD_ID_COPY as i32;
	pub const PASTE: i32 = ffi::WXD_ID_PASTE as i32;
	pub const DELETE: i32 = ffi::WXD_ID_CLEAR as i32;
	pub const SELECT_ALL: i32 = ffi::WXD_ID_SELECTALL as i32;
}

// Base for custom IDs
const BASE: i32 = 5000;

// File menu (BASE + 0..99)
seq_ids!(BASE => OPEN, CLOSE, CLOSE_ALL, SHOW_ALL_DOCUMENTS, REOPEN_LAST_CLOSED);

// Recent documents - reserved range (BASE + 100..199)
pub const RECENT_DOCUMENT_BASE: i32 = BASE + 100;
pub const RECENT_DOCUMENT_MAX: i32 = BASE + 199;

// Go menu: Find (BASE + 200..209)
seq_ids!(BASE + 200 => FIND, FIND_NEXT, FIND_PREVIOUS, ANNOUNCE_PERCENT, SET_TEMPORARY_BOOKMARK, JUMP_TO_TEMPORARY_BOOKMARK);

// Go menu: Go to (BASE + 210..219)
seq_ids!(BASE + 210 => GO_TO_LINE, GO_TO_PERCENT, GO_TO_PAGE);

// Go menu: History (BASE + 220..229)
seq_ids!(BASE + 220 => GO_BACK, GO_FORWARD);

// Go menu: Section navigation (BASE + 230..239)
seq_ids!(BASE + 230 => PREVIOUS_SECTION, NEXT_SECTION);

// Go menu: Heading navigation (BASE + 240..269)
seq_ids!(BASE + 240 => PREVIOUS_HEADING, NEXT_HEADING);
seq_ids!(BASE + 250 =>
	PREVIOUS_HEADING_1, NEXT_HEADING_1,
	PREVIOUS_HEADING_2, NEXT_HEADING_2,
	PREVIOUS_HEADING_3, NEXT_HEADING_3,
	PREVIOUS_HEADING_4, NEXT_HEADING_4,
	PREVIOUS_HEADING_5, NEXT_HEADING_5,
	PREVIOUS_HEADING_6, NEXT_HEADING_6,
);

// Go menu: Page navigation (BASE + 270..279)
seq_ids!(BASE + 270 => PREVIOUS_PAGE, NEXT_PAGE);

// Go menu: Bookmarks and notes (BASE + 280..289)
seq_ids!(BASE + 280 =>
	PREVIOUS_BOOKMARK, NEXT_BOOKMARK,
	PREVIOUS_NOTE, NEXT_NOTE,
	JUMP_TO_ALL_BOOKMARKS, JUMP_TO_BOOKMARKS_ONLY, JUMP_TO_NOTES_ONLY,
	VIEW_NOTE_TEXT,
);

// Go menu: Link navigation (BASE + 290..299)
seq_ids!(BASE + 290 => PREVIOUS_LINK, NEXT_LINK);

// Go menu: Image navigation (BASE + 295..299)
seq_ids!(BASE + 295 => PREVIOUS_IMAGE, NEXT_IMAGE);

// Go menu: Figure navigation (BASE + 300..304)
seq_ids!(BASE + 300 => PREVIOUS_FIGURE, NEXT_FIGURE);

// Go menu: Element navigation (BASE + 305..319)
seq_ids!(BASE + 305 => PREVIOUS_TABLE, NEXT_TABLE, PREVIOUS_SEPARATOR, NEXT_SEPARATOR);
seq_ids!(BASE + 310 => PREVIOUS_LIST, NEXT_LIST, PREVIOUS_LIST_ITEM, NEXT_LIST_ITEM);
seq_ids!(BASE + 314 => CONTAINER_START, CONTAINER_END);

// Tools menu: Document info (BASE + 400..409)
seq_ids!(BASE + 400 =>
	WORD_COUNT, DOCUMENT_INFO, TABLE_OF_CONTENTS, ELEMENTS_LIST,
	REVEAL_FILE_IN_FOLDER, OPEN_IN_WEB_VIEW, VIEW_SOURCE,
);

// Tools menu: Import/Export (BASE + 410..419)
seq_ids!(BASE + 410 => IMPORT_DOCUMENT_DATA, EXPORT_DOCUMENT_DATA, EXPORT_TO_PLAIN_TEXT, EXPORT_TO_HTML, EXPORT_TO_MARKDOWN);

// Tools menu: Bookmarks (BASE + 420..429)
seq_ids!(BASE + 420 => TOGGLE_BOOKMARK, BOOKMARK_WITH_NOTE);

// Tools menu: Settings (BASE + 430..439)
seq_ids!(BASE + 430 => OPTIONS, SLEEP_TIMER, CUSTOMIZE_SHORTCUTS);

// Tools menu: View toggles (BASE + 440..449)
seq_ids!(BASE + 440 => TOGGLE_WORD_WRAP, TOGGLE_FULL_SCREEN);

// Help menu (BASE + 500..599)
seq_ids!(BASE + 500 => VIEW_HELP_BROWSER, VIEW_HELP_PAPERBACK, CHECK_FOR_UPDATES, DONATE);

// System tray (BASE + 900..999)
seq_ids!(BASE + 900 => RESTORE);

pub const fn action_to_menu_id(action: paperback_core::config::ActionId) -> Option<i32> {
	use paperback_core::config::ActionId;
	match action {
		ActionId::Open => Some(OPEN),
		ActionId::Close => Some(CLOSE),
		ActionId::CloseAll => Some(CLOSE_ALL),
		ActionId::ReopenLastClosed => Some(REOPEN_LAST_CLOSED),
		ActionId::ShowAllRecentDocuments => Some(SHOW_ALL_DOCUMENTS),
		ActionId::Exit => Some(EXIT),
		ActionId::Find => Some(FIND),
		ActionId::FindNext => Some(FIND_NEXT),
		ActionId::FindPrevious => Some(FIND_PREVIOUS),
		ActionId::GoToLine => Some(GO_TO_LINE),
		ActionId::GoToPercent => Some(GO_TO_PERCENT),
		ActionId::GoToPage => Some(GO_TO_PAGE),
		ActionId::GoBack => Some(GO_BACK),
		ActionId::GoForward => Some(GO_FORWARD),
		ActionId::AnnouncePercent => Some(ANNOUNCE_PERCENT),
		ActionId::SetTemporaryBookmark => Some(SET_TEMPORARY_BOOKMARK),
		ActionId::JumpToTemporaryBookmark => Some(JUMP_TO_TEMPORARY_BOOKMARK),
		ActionId::PreviousSection => Some(PREVIOUS_SECTION),
		ActionId::NextSection => Some(NEXT_SECTION),
		ActionId::PreviousHeading => Some(PREVIOUS_HEADING),
		ActionId::NextHeading => Some(NEXT_HEADING),
		ActionId::PreviousHeading1 => Some(PREVIOUS_HEADING_1),
		ActionId::NextHeading1 => Some(NEXT_HEADING_1),
		ActionId::PreviousHeading2 => Some(PREVIOUS_HEADING_2),
		ActionId::NextHeading2 => Some(NEXT_HEADING_2),
		ActionId::PreviousHeading3 => Some(PREVIOUS_HEADING_3),
		ActionId::NextHeading3 => Some(NEXT_HEADING_3),
		ActionId::PreviousHeading4 => Some(PREVIOUS_HEADING_4),
		ActionId::NextHeading4 => Some(NEXT_HEADING_4),
		ActionId::PreviousHeading5 => Some(PREVIOUS_HEADING_5),
		ActionId::NextHeading5 => Some(NEXT_HEADING_5),
		ActionId::PreviousHeading6 => Some(PREVIOUS_HEADING_6),
		ActionId::NextHeading6 => Some(NEXT_HEADING_6),
		ActionId::PreviousPage => Some(PREVIOUS_PAGE),
		ActionId::NextPage => Some(NEXT_PAGE),
		ActionId::PreviousBookmark => Some(PREVIOUS_BOOKMARK),
		ActionId::NextBookmark => Some(NEXT_BOOKMARK),
		ActionId::PreviousNote => Some(PREVIOUS_NOTE),
		ActionId::NextNote => Some(NEXT_NOTE),
		ActionId::JumpToAllBookmarks => Some(JUMP_TO_ALL_BOOKMARKS),
		ActionId::JumpToBookmarksOnly => Some(JUMP_TO_BOOKMARKS_ONLY),
		ActionId::JumpToNotesOnly => Some(JUMP_TO_NOTES_ONLY),
		ActionId::ViewNoteText => Some(VIEW_NOTE_TEXT),
		ActionId::PreviousLink => Some(PREVIOUS_LINK),
		ActionId::NextLink => Some(NEXT_LINK),
		ActionId::PreviousImage => Some(PREVIOUS_IMAGE),
		ActionId::NextImage => Some(NEXT_IMAGE),
		ActionId::PreviousFigure => Some(PREVIOUS_FIGURE),
		ActionId::NextFigure => Some(NEXT_FIGURE),
		ActionId::PreviousTable => Some(PREVIOUS_TABLE),
		ActionId::NextTable => Some(NEXT_TABLE),
		ActionId::PreviousSeparator => Some(PREVIOUS_SEPARATOR),
		ActionId::NextSeparator => Some(NEXT_SEPARATOR),
		ActionId::PreviousList => Some(PREVIOUS_LIST),
		ActionId::NextList => Some(NEXT_LIST),
		ActionId::PreviousListItem => Some(PREVIOUS_LIST_ITEM),
		ActionId::NextListItem => Some(NEXT_LIST_ITEM),
		ActionId::ContainerStart => Some(CONTAINER_START),
		ActionId::ContainerEnd => Some(CONTAINER_END),
		ActionId::WordCount => Some(WORD_COUNT),
		ActionId::DocumentInfo => Some(DOCUMENT_INFO),
		ActionId::TableOfContents => Some(TABLE_OF_CONTENTS),
		ActionId::ElementsList => Some(ELEMENTS_LIST),
		ActionId::RevealFileInFolder => Some(REVEAL_FILE_IN_FOLDER),
		ActionId::OpenInWebView => Some(OPEN_IN_WEB_VIEW),
		ActionId::ViewSource => Some(VIEW_SOURCE),
		ActionId::ToggleBookmark => Some(TOGGLE_BOOKMARK),
		ActionId::BookmarkWithNote => Some(BOOKMARK_WITH_NOTE),
		ActionId::ToggleWordWrap => Some(TOGGLE_WORD_WRAP),
		ActionId::ToggleFullScreen => Some(TOGGLE_FULL_SCREEN),
		ActionId::Options => Some(OPTIONS),
		ActionId::SleepTimer => Some(SLEEP_TIMER),
		ActionId::CustomizeShortcuts => Some(CUSTOMIZE_SHORTCUTS),
		ActionId::ImportDocumentData => Some(IMPORT_DOCUMENT_DATA),
		ActionId::ExportDocumentData => Some(EXPORT_DOCUMENT_DATA),
		ActionId::ExportToPlainText => Some(EXPORT_TO_PLAIN_TEXT),
		ActionId::ExportToHtml => Some(EXPORT_TO_HTML),
		ActionId::ExportToMarkdown => Some(EXPORT_TO_MARKDOWN),
		ActionId::About => Some(ABOUT),
		ActionId::ViewHelpBrowser => Some(VIEW_HELP_BROWSER),
		ActionId::ViewHelpPaperback => Some(VIEW_HELP_PAPERBACK),
		ActionId::CheckForUpdates => Some(CHECK_FOR_UPDATES),
		ActionId::Donate => Some(DONATE),
	}
}
