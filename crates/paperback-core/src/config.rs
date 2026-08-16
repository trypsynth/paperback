use std::{
	cell::{Cell, RefCell},
	cmp::Ordering,
	collections::HashMap,
	fs,
	path::{Path, PathBuf},
};

use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};

use crate::types::{DocumentListItem, DocumentListStatus};

const CONFIG_VERSION: u32 = 4;
const DEFAULT_RECENT_DOCUMENTS_TO_SHOW: i64 = 25;
const MAX_RECENT_DOCUMENTS_TO_SHOW: usize = 100;

#[derive(Clone, Debug, Default)]
pub struct Bookmark {
	pub start: i64,
	pub end: i64,
	pub note: String,
}

#[derive(Clone, Debug, Default)]
pub struct NavigationHistory {
	pub positions: Vec<i64>,
	pub index: usize,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FindSettings {
	pub match_case: bool,
	pub whole_word: bool,
	pub use_regex: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReadabilityFont {
	pub face_name: String,
	pub point_size: i32,
	pub style: i32,
	pub weight: i32,
	pub underlined: bool,
	pub strikethrough: bool,
	/// RGB color packed as 0xRRGGBB, or -1 for default (no override)
	pub color: i32,
	/// Platform-specific font encoding identifier; 0 means default. Ignored on
	/// platforms that manage encoding themselves (e.g. Android/iOS Unicode rendering).
	pub encoding: i32,
}

impl Default for ReadabilityFont {
	fn default() -> Self {
		Self {
			face_name: String::new(),
			point_size: 0,
			style: 0,
			weight: 0,
			underlined: false,
			strikethrough: false,
			color: -1,
			encoding: 0,
		}
	}
}

impl ReadabilityFont {
	#[must_use]
	pub const fn is_default(&self) -> bool {
		self.face_name.is_empty() && self.point_size == 0
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct StoredBookmark {
	pub start: i64,
	pub end: i64,
	#[serde(default)]
	pub note: String,
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HotkeyConfig {
	pub ctrl: bool,
	pub alt: bool,
	pub shift: bool,
	pub win: bool,
	pub key: char,
}

impl Default for HotkeyConfig {
	fn default() -> Self {
		Self { ctrl: true, alt: true, shift: false, win: false, key: 'P' }
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ShortcutCategory {
	File,
	Go,
	Tools,
	Help,
}

impl ShortcutCategory {
	pub const fn all() -> &'static [Self] {
		&[Self::File, Self::Go, Self::Tools, Self::Help]
	}

	pub const fn display_name(self) -> &'static str {
		match self {
			Self::File => "File",
			Self::Go => "Go",
			Self::Tools => "Tools",
			Self::Help => "Help",
		}
	}

	pub fn actions(self) -> Vec<ActionId> {
		ActionId::all().iter().copied().filter(|a| a.category() == self).collect()
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionId {
	Open,
	Close,
	CloseAll,
	ReopenLastClosed,
	ShowAllRecentDocuments,
	Exit,
	Find,
	FindNext,
	FindPrevious,
	GoToLine,
	GoToPercent,
	GoToPage,
	GoBack,
	GoForward,
	AnnouncePercent,
	SetTemporaryBookmark,
	JumpToTemporaryBookmark,
	PreviousSection,
	NextSection,
	PreviousHeading,
	NextHeading,
	PreviousHeading1,
	NextHeading1,
	PreviousHeading2,
	NextHeading2,
	PreviousHeading3,
	NextHeading3,
	PreviousHeading4,
	NextHeading4,
	PreviousHeading5,
	NextHeading5,
	PreviousHeading6,
	NextHeading6,
	PreviousPage,
	NextPage,
	PreviousBookmark,
	NextBookmark,
	PreviousNote,
	NextNote,
	JumpToAllBookmarks,
	JumpToBookmarksOnly,
	JumpToNotesOnly,
	ViewNoteText,
	PreviousLink,
	NextLink,
	PreviousImage,
	NextImage,
	PreviousFigure,
	NextFigure,
	PreviousTable,
	NextTable,
	PreviousSeparator,
	NextSeparator,
	PreviousList,
	NextList,
	PreviousListItem,
	NextListItem,
	ContainerStart,
	ContainerEnd,
	WordCount,
	DocumentInfo,
	TableOfContents,
	ElementsList,
	RevealFileInFolder,
	OpenInWebView,
	ViewSource,
	ToggleBookmark,
	BookmarkWithNote,
	ToggleWordWrap,
	ToggleFullScreen,
	Options,
	SleepTimer,
	CustomizeShortcuts,
	ImportDocumentData,
	ExportDocumentData,
	ExportToPlainText,
	ExportToHtml,
	ExportToMarkdown,
	About,
	ViewHelpBrowser,
	ViewHelpPaperback,
	CheckForUpdates,
	Donate,
}

impl ActionId {
	pub const fn all() -> &'static [Self] {
		&[
			Self::Open,
			Self::Close,
			Self::CloseAll,
			Self::ReopenLastClosed,
			Self::ShowAllRecentDocuments,
			Self::Exit,
			Self::Find,
			Self::FindNext,
			Self::FindPrevious,
			Self::GoToLine,
			Self::GoToPercent,
			Self::GoToPage,
			Self::GoBack,
			Self::GoForward,
			Self::AnnouncePercent,
			Self::SetTemporaryBookmark,
			Self::JumpToTemporaryBookmark,
			Self::PreviousSection,
			Self::NextSection,
			Self::PreviousHeading,
			Self::NextHeading,
			Self::PreviousHeading1,
			Self::NextHeading1,
			Self::PreviousHeading2,
			Self::NextHeading2,
			Self::PreviousHeading3,
			Self::NextHeading3,
			Self::PreviousHeading4,
			Self::NextHeading4,
			Self::PreviousHeading5,
			Self::NextHeading5,
			Self::PreviousHeading6,
			Self::NextHeading6,
			Self::PreviousPage,
			Self::NextPage,
			Self::PreviousBookmark,
			Self::NextBookmark,
			Self::PreviousNote,
			Self::NextNote,
			Self::JumpToAllBookmarks,
			Self::JumpToBookmarksOnly,
			Self::JumpToNotesOnly,
			Self::ViewNoteText,
			Self::PreviousLink,
			Self::NextLink,
			Self::PreviousImage,
			Self::NextImage,
			Self::PreviousFigure,
			Self::NextFigure,
			Self::PreviousTable,
			Self::NextTable,
			Self::PreviousSeparator,
			Self::NextSeparator,
			Self::PreviousList,
			Self::NextList,
			Self::PreviousListItem,
			Self::NextListItem,
			Self::ContainerStart,
			Self::ContainerEnd,
			Self::WordCount,
			Self::DocumentInfo,
			Self::TableOfContents,
			Self::ElementsList,
			Self::RevealFileInFolder,
			Self::OpenInWebView,
			Self::ViewSource,
			Self::ToggleBookmark,
			Self::BookmarkWithNote,
			Self::ToggleWordWrap,
			Self::ToggleFullScreen,
			Self::Options,
			Self::SleepTimer,
			Self::CustomizeShortcuts,
			Self::ImportDocumentData,
			Self::ExportDocumentData,
			Self::ExportToPlainText,
			Self::ExportToHtml,
			Self::ExportToMarkdown,
			Self::About,
			Self::ViewHelpBrowser,
			Self::ViewHelpPaperback,
			Self::CheckForUpdates,
			Self::Donate,
		]
	}

	pub const fn category(self) -> ShortcutCategory {
		match self {
			Self::Open
			| Self::Close
			| Self::CloseAll
			| Self::ReopenLastClosed
			| Self::ShowAllRecentDocuments
			| Self::Exit => ShortcutCategory::File,
			Self::Find
			| Self::FindNext
			| Self::FindPrevious
			| Self::GoToLine
			| Self::GoToPercent
			| Self::GoToPage
			| Self::GoBack
			| Self::GoForward
			| Self::AnnouncePercent
			| Self::SetTemporaryBookmark
			| Self::JumpToTemporaryBookmark
			| Self::PreviousSection
			| Self::NextSection
			| Self::PreviousHeading
			| Self::NextHeading
			| Self::PreviousHeading1
			| Self::NextHeading1
			| Self::PreviousHeading2
			| Self::NextHeading2
			| Self::PreviousHeading3
			| Self::NextHeading3
			| Self::PreviousHeading4
			| Self::NextHeading4
			| Self::PreviousHeading5
			| Self::NextHeading5
			| Self::PreviousHeading6
			| Self::NextHeading6
			| Self::PreviousPage
			| Self::NextPage
			| Self::PreviousBookmark
			| Self::NextBookmark
			| Self::PreviousNote
			| Self::NextNote
			| Self::JumpToAllBookmarks
			| Self::JumpToBookmarksOnly
			| Self::JumpToNotesOnly
			| Self::ViewNoteText
			| Self::PreviousLink
			| Self::NextLink
			| Self::PreviousImage
			| Self::NextImage
			| Self::PreviousFigure
			| Self::NextFigure
			| Self::PreviousTable
			| Self::NextTable
			| Self::PreviousSeparator
			| Self::NextSeparator
			| Self::PreviousList
			| Self::NextList
			| Self::PreviousListItem
			| Self::NextListItem
			| Self::ContainerStart
			| Self::ContainerEnd => ShortcutCategory::Go,
			Self::WordCount
			| Self::DocumentInfo
			| Self::TableOfContents
			| Self::ElementsList
			| Self::RevealFileInFolder
			| Self::OpenInWebView
			| Self::ViewSource
			| Self::ToggleBookmark
			| Self::BookmarkWithNote
			| Self::ToggleWordWrap
			| Self::ToggleFullScreen
			| Self::Options
			| Self::SleepTimer
			| Self::CustomizeShortcuts
			| Self::ImportDocumentData
			| Self::ExportDocumentData
			| Self::ExportToPlainText
			| Self::ExportToHtml
			| Self::ExportToMarkdown => ShortcutCategory::Tools,
			Self::About | Self::ViewHelpBrowser | Self::ViewHelpPaperback | Self::CheckForUpdates | Self::Donate => {
				ShortcutCategory::Help
			}
		}
	}

	pub const fn display_name(self) -> &'static str {
		match self {
			Self::Open => "Open...",
			Self::Close => "Close",
			Self::CloseAll => "Close All",
			Self::ReopenLastClosed => "Reopen Last Closed",
			Self::ShowAllRecentDocuments => "Show All Recent Documents...",
			Self::Exit => "Exit",
			Self::Find => "Find...",
			Self::FindNext => "Find Next",
			Self::FindPrevious => "Find Previous",
			Self::GoToLine => "Go to Line...",
			Self::GoToPercent => "Go to Percent...",
			Self::GoToPage => "Go to Page...",
			Self::GoBack => "Go Back",
			Self::GoForward => "Go Forward",
			Self::AnnouncePercent => "Announce Percentage",
			Self::SetTemporaryBookmark => "Set Temporary Bookmark",
			Self::JumpToTemporaryBookmark => "Jump to Temporary Bookmark",
			Self::PreviousSection => "Previous Section",
			Self::NextSection => "Next Section",
			Self::PreviousHeading => "Previous Heading",
			Self::NextHeading => "Next Heading",
			Self::PreviousHeading1 => "Previous Heading Level 1",
			Self::NextHeading1 => "Next Heading Level 1",
			Self::PreviousHeading2 => "Previous Heading Level 2",
			Self::NextHeading2 => "Next Heading Level 2",
			Self::PreviousHeading3 => "Previous Heading Level 3",
			Self::NextHeading3 => "Next Heading Level 3",
			Self::PreviousHeading4 => "Previous Heading Level 4",
			Self::NextHeading4 => "Next Heading Level 4",
			Self::PreviousHeading5 => "Previous Heading Level 5",
			Self::NextHeading5 => "Next Heading Level 5",
			Self::PreviousHeading6 => "Previous Heading Level 6",
			Self::NextHeading6 => "Next Heading Level 6",
			Self::PreviousPage => "Previous Page",
			Self::NextPage => "Next Page",
			Self::PreviousBookmark => "Previous Bookmark",
			Self::NextBookmark => "Next Bookmark",
			Self::PreviousNote => "Previous Note",
			Self::NextNote => "Next Note",
			Self::JumpToAllBookmarks => "Jump to All Bookmarks...",
			Self::JumpToBookmarksOnly => "Jump to Bookmarks Only...",
			Self::JumpToNotesOnly => "Jump to Notes Only...",
			Self::ViewNoteText => "View Note Text",
			Self::PreviousLink => "Previous Link",
			Self::NextLink => "Next Link",
			Self::PreviousImage => "Previous Image",
			Self::NextImage => "Next Image",
			Self::PreviousFigure => "Previous Figure",
			Self::NextFigure => "Next Figure",
			Self::PreviousTable => "Previous Table",
			Self::NextTable => "Next Table",
			Self::PreviousSeparator => "Previous Separator",
			Self::NextSeparator => "Next Separator",
			Self::PreviousList => "Previous List",
			Self::NextList => "Next List",
			Self::PreviousListItem => "Previous List Item",
			Self::NextListItem => "Next List Item",
			Self::ContainerStart => "Container Start",
			Self::ContainerEnd => "Past Container End",
			Self::WordCount => "Word Count",
			Self::DocumentInfo => "Document Info",
			Self::TableOfContents => "Table of Contents",
			Self::ElementsList => "Elements List...",
			Self::RevealFileInFolder => "Reveal File in Folder",
			Self::OpenInWebView => "Open in Web View",
			Self::ViewSource => "View Source",
			Self::ToggleBookmark => "Toggle Bookmark",
			Self::BookmarkWithNote => "Bookmark with Note",
			Self::ToggleWordWrap => "Toggle Word Wrap",
			Self::ToggleFullScreen => "Full Screen",
			Self::Options => "Options...",
			Self::SleepTimer => "Sleep Timer...",
			Self::CustomizeShortcuts => "Customize Keyboard Shortcuts...",
			Self::ImportDocumentData => "Import Document Data...",
			Self::ExportDocumentData => "Export Document Data...",
			Self::ExportToPlainText => "Export to Plain Text...",
			Self::ExportToHtml => "Export to HTML...",
			Self::ExportToMarkdown => "Export to Markdown...",
			Self::About => "About Paperback",
			Self::ViewHelpBrowser => "View Help in Browser",
			Self::ViewHelpPaperback => "View Help in Paperback",
			Self::CheckForUpdates => "Check for Updates",
			Self::Donate => "Donate",
		}
	}

	pub fn default_chord(self) -> Option<KeyChord> {
		#[cfg(target_os = "macos")]
		match self {
			Self::Open => Some(KeyChord::new(true, false, false, "O")),
			Self::Close => Some(KeyChord::new(true, false, false, "W")),
			Self::CloseAll => Some(KeyChord::new(true, false, true, "W")),
			Self::ReopenLastClosed => Some(KeyChord::new(true, false, true, "T")),
			Self::ShowAllRecentDocuments => Some(KeyChord::new(true, false, false, "R")),
			Self::Exit => None,
			Self::Find => Some(KeyChord::new(true, false, false, "F")),
			Self::FindNext => Some(KeyChord::new(true, false, false, "G")),
			Self::FindPrevious => Some(KeyChord::new(true, false, true, "G")),
			Self::GoToLine => Some(KeyChord::new(true, false, false, "L")),
			Self::GoToPercent => Some(KeyChord::new(true, false, true, "L")),
			Self::GoToPage => Some(KeyChord::new(true, false, false, "P")),
			Self::GoBack => Some(KeyChord::new(true, false, false, "[")),
			Self::GoForward => Some(KeyChord::new(true, false, false, "]")),
			Self::AnnouncePercent => Some(KeyChord::new(false, false, false, "=")),
			Self::SetTemporaryBookmark => Some(KeyChord::new(false, false, false, "/")),
			Self::JumpToTemporaryBookmark => Some(KeyChord::new(false, false, false, "\\")),
			Self::PreviousSection => Some(KeyChord::new(false, false, false, "[")),
			Self::NextSection => Some(KeyChord::new(false, false, false, "]")),
			Self::PreviousHeading => Some(KeyChord::new(false, false, true, "H")),
			Self::NextHeading => Some(KeyChord::new(false, false, false, "H")),
			Self::PreviousHeading1 => Some(KeyChord::new(false, false, true, "1")),
			Self::NextHeading1 => Some(KeyChord::new(false, false, false, "1")),
			Self::PreviousHeading2 => Some(KeyChord::new(false, false, true, "2")),
			Self::NextHeading2 => Some(KeyChord::new(false, false, false, "2")),
			Self::PreviousHeading3 => Some(KeyChord::new(false, false, true, "3")),
			Self::NextHeading3 => Some(KeyChord::new(false, false, false, "3")),
			Self::PreviousHeading4 => Some(KeyChord::new(false, false, true, "4")),
			Self::NextHeading4 => Some(KeyChord::new(false, false, false, "4")),
			Self::PreviousHeading5 => Some(KeyChord::new(false, false, true, "5")),
			Self::NextHeading5 => Some(KeyChord::new(false, false, false, "5")),
			Self::PreviousHeading6 => Some(KeyChord::new(false, false, true, "6")),
			Self::NextHeading6 => Some(KeyChord::new(false, false, false, "6")),
			Self::PreviousPage => Some(KeyChord::new(false, false, true, "P")),
			Self::NextPage => Some(KeyChord::new(false, false, false, "P")),
			Self::PreviousBookmark => Some(KeyChord::new(false, false, true, "B")),
			Self::NextBookmark => Some(KeyChord::new(false, false, false, "B")),
			Self::PreviousNote => Some(KeyChord::new(false, false, true, "N")),
			Self::NextNote => Some(KeyChord::new(false, false, false, "N")),
			Self::JumpToAllBookmarks => Some(KeyChord::new(true, false, false, "B")),
			Self::JumpToBookmarksOnly => Some(KeyChord::new(true, true, false, "B")),
			Self::JumpToNotesOnly => Some(KeyChord::new(true, true, false, "M")),
			Self::ViewNoteText => Some(KeyChord::new_raw_ctrl(true, false, true, "W")),
			Self::PreviousLink => Some(KeyChord::new(false, false, true, "K")),
			Self::NextLink => Some(KeyChord::new(false, false, false, "K")),
			Self::PreviousImage => Some(KeyChord::new(false, false, true, "G")),
			Self::NextImage => Some(KeyChord::new(false, false, false, "G")),
			Self::PreviousFigure => Some(KeyChord::new(false, false, true, "F")),
			Self::NextFigure => Some(KeyChord::new(false, false, false, "F")),
			Self::PreviousTable => Some(KeyChord::new(false, false, true, "T")),
			Self::NextTable => Some(KeyChord::new(false, false, false, "T")),
			Self::PreviousSeparator => Some(KeyChord::new(false, false, true, "S")),
			Self::NextSeparator => Some(KeyChord::new(false, false, false, "S")),
			Self::PreviousList => Some(KeyChord::new(false, false, true, "L")),
			Self::NextList => Some(KeyChord::new(false, false, false, "L")),
			Self::PreviousListItem => Some(KeyChord::new(false, false, true, "I")),
			Self::NextListItem => Some(KeyChord::new(false, false, false, "I")),
			Self::ContainerStart => Some(KeyChord::new(false, false, true, ",")),
			Self::ContainerEnd => Some(KeyChord::new(false, false, false, ",")),
			Self::WordCount => Some(KeyChord::new_raw_ctrl(true, false, false, "W")),
			Self::DocumentInfo => Some(KeyChord::new(true, false, false, "I")),
			Self::TableOfContents => Some(KeyChord::new(true, false, false, "T")),
			Self::ElementsList => Some(KeyChord::new(false, false, false, "F7")),
			Self::RevealFileInFolder => Some(KeyChord::new(true, false, true, "C")),
			Self::OpenInWebView => Some(KeyChord::new(true, false, true, "V")),
			Self::ViewSource => Some(KeyChord::new(true, false, false, "U")),
			Self::ToggleBookmark => Some(KeyChord::new(true, false, true, "B")),
			Self::BookmarkWithNote => Some(KeyChord::new(true, false, true, "N")),
			Self::ToggleWordWrap => Some(KeyChord::new(true, true, false, "W")),
			// The conventional shortcut is Control+Command+F; RawCtrl forces the
			// physical Control key while plain Ctrl auto-translates to Command on mac.
			Self::ToggleFullScreen => {
				Some(KeyChord { ctrl: true, raw_ctrl: true, alt: false, shift: false, key: "F".to_string() })
			}
			Self::Options => Some(KeyChord::new(true, false, false, ",")),
			Self::SleepTimer => Some(KeyChord::new(true, false, true, "S")),
			Self::CustomizeShortcuts => None,
			Self::ImportDocumentData => Some(KeyChord::new(true, false, true, "I")),
			Self::ExportDocumentData => Some(KeyChord::new(true, false, true, "E")),
			Self::ExportToPlainText => Some(KeyChord::new(true, false, false, "E")),
			Self::ExportToHtml => None,
			Self::ExportToMarkdown => None,
			Self::About => Some(KeyChord::new(true, false, false, "F1")),
			Self::ViewHelpBrowser => Some(KeyChord::new(false, false, false, "F1")),
			Self::ViewHelpPaperback => Some(KeyChord::new(false, false, true, "F1")),
			Self::CheckForUpdates => Some(KeyChord::new(true, false, true, "U")),
			Self::Donate => Some(KeyChord::new(true, false, false, "D")),
		}

		#[cfg(not(target_os = "macos"))]
		match self {
			Self::Open => Some(KeyChord::new(true, false, false, "O")),
			Self::Close => Some(KeyChord::new(true, false, false, "F4")),
			Self::CloseAll => Some(KeyChord::new(true, false, true, "F4")),
			Self::ReopenLastClosed => Some(KeyChord::new(true, false, true, "T")),
			Self::ShowAllRecentDocuments => Some(KeyChord::new(true, false, false, "R")),
			Self::Exit => Some(KeyChord::new(true, false, false, "Q")),
			Self::Find => Some(KeyChord::new(true, false, false, "F")),
			Self::FindNext => Some(KeyChord::new(false, false, false, "F3")),
			Self::FindPrevious => Some(KeyChord::new(false, false, true, "F3")),
			Self::GoToLine => Some(KeyChord::new(true, false, false, "G")),
			Self::GoToPercent => Some(KeyChord::new(true, false, true, "G")),
			Self::GoToPage => Some(KeyChord::new(true, false, false, "P")),
			Self::GoBack => Some(KeyChord::new(false, true, false, "Left")),
			Self::GoForward => Some(KeyChord::new(false, true, false, "Right")),
			Self::AnnouncePercent => Some(KeyChord::new(false, false, false, "=")),
			Self::SetTemporaryBookmark => Some(KeyChord::new(false, false, false, "/")),
			Self::JumpToTemporaryBookmark => Some(KeyChord::new(false, false, false, "\\")),
			Self::PreviousSection => Some(KeyChord::new(false, false, false, "[")),
			Self::NextSection => Some(KeyChord::new(false, false, false, "]")),
			Self::PreviousHeading => Some(KeyChord::new(false, false, true, "H")),
			Self::NextHeading => Some(KeyChord::new(false, false, false, "H")),
			Self::PreviousHeading1 => Some(KeyChord::new(false, false, true, "1")),
			Self::NextHeading1 => Some(KeyChord::new(false, false, false, "1")),
			Self::PreviousHeading2 => Some(KeyChord::new(false, false, true, "2")),
			Self::NextHeading2 => Some(KeyChord::new(false, false, false, "2")),
			Self::PreviousHeading3 => Some(KeyChord::new(false, false, true, "3")),
			Self::NextHeading3 => Some(KeyChord::new(false, false, false, "3")),
			Self::PreviousHeading4 => Some(KeyChord::new(false, false, true, "4")),
			Self::NextHeading4 => Some(KeyChord::new(false, false, false, "4")),
			Self::PreviousHeading5 => Some(KeyChord::new(false, false, true, "5")),
			Self::NextHeading5 => Some(KeyChord::new(false, false, false, "5")),
			Self::PreviousHeading6 => Some(KeyChord::new(false, false, true, "6")),
			Self::NextHeading6 => Some(KeyChord::new(false, false, false, "6")),
			Self::PreviousPage => Some(KeyChord::new(false, false, true, "P")),
			Self::NextPage => Some(KeyChord::new(false, false, false, "P")),
			Self::PreviousBookmark => Some(KeyChord::new(false, false, true, "B")),
			Self::NextBookmark => Some(KeyChord::new(false, false, false, "B")),
			Self::PreviousNote => Some(KeyChord::new(false, false, true, "N")),
			Self::NextNote => Some(KeyChord::new(false, false, false, "N")),
			Self::JumpToAllBookmarks => Some(KeyChord::new(true, false, false, "B")),
			Self::JumpToBookmarksOnly => Some(KeyChord::new(true, true, false, "B")),
			Self::JumpToNotesOnly => Some(KeyChord::new(true, true, false, "M")),
			Self::ViewNoteText => Some(KeyChord::new(true, false, true, "W")),
			Self::PreviousLink => Some(KeyChord::new(false, false, true, "K")),
			Self::NextLink => Some(KeyChord::new(false, false, false, "K")),
			Self::PreviousImage => Some(KeyChord::new(false, false, true, "G")),
			Self::NextImage => Some(KeyChord::new(false, false, false, "G")),
			Self::PreviousFigure => Some(KeyChord::new(false, false, true, "F")),
			Self::NextFigure => Some(KeyChord::new(false, false, false, "F")),
			Self::PreviousTable => Some(KeyChord::new(false, false, true, "T")),
			Self::NextTable => Some(KeyChord::new(false, false, false, "T")),
			Self::PreviousSeparator => Some(KeyChord::new(false, false, true, "S")),
			Self::NextSeparator => Some(KeyChord::new(false, false, false, "S")),
			Self::PreviousList => Some(KeyChord::new(false, false, true, "L")),
			Self::NextList => Some(KeyChord::new(false, false, false, "L")),
			Self::PreviousListItem => Some(KeyChord::new(false, false, true, "I")),
			Self::NextListItem => Some(KeyChord::new(false, false, false, "I")),
			Self::ContainerStart => Some(KeyChord::new(false, false, true, ",")),
			Self::ContainerEnd => Some(KeyChord::new(false, false, false, ",")),
			Self::WordCount => Some(KeyChord::new(true, false, false, "W")),
			Self::DocumentInfo => Some(KeyChord::new(true, false, false, "I")),
			Self::TableOfContents => Some(KeyChord::new(true, false, false, "T")),
			Self::ElementsList => Some(KeyChord::new(false, false, false, "F7")),
			Self::RevealFileInFolder => Some(KeyChord::new(true, false, true, "C")),
			Self::OpenInWebView => Some(KeyChord::new(true, false, true, "V")),
			Self::ViewSource => Some(KeyChord::new(true, false, false, "U")),
			Self::ToggleBookmark => Some(KeyChord::new(true, false, true, "B")),
			Self::BookmarkWithNote => Some(KeyChord::new(true, false, true, "N")),
			Self::ToggleWordWrap => Some(KeyChord::new(true, true, false, "W")),
			Self::ToggleFullScreen => Some(KeyChord::new(false, false, false, "F11")),
			Self::Options => Some(KeyChord::new(true, false, false, ",")),
			Self::SleepTimer => Some(KeyChord::new(true, false, true, "S")),
			Self::CustomizeShortcuts => None,
			Self::ImportDocumentData => Some(KeyChord::new(true, false, true, "I")),
			Self::ExportDocumentData => Some(KeyChord::new(true, false, true, "E")),
			Self::ExportToPlainText => Some(KeyChord::new(true, false, false, "E")),
			Self::ExportToHtml => None,
			Self::ExportToMarkdown => None,
			Self::About => Some(KeyChord::new(true, false, false, "F1")),
			Self::ViewHelpBrowser => Some(KeyChord::new(false, false, false, "F1")),
			Self::ViewHelpPaperback => Some(KeyChord::new(false, false, true, "F1")),
			Self::CheckForUpdates => Some(KeyChord::new(true, false, true, "U")),
			Self::Donate => Some(KeyChord::new(true, false, false, "D")),
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct KeyChord {
	pub ctrl: bool,
	#[serde(default)]
	pub raw_ctrl: bool,
	pub alt: bool,
	pub shift: bool,
	pub key: String,
}

impl KeyChord {
	pub fn new(ctrl: bool, alt: bool, shift: bool, key: impl Into<String>) -> Self {
		let key_str = key.into();
		let normalized = Self::normalize_key_name(&key_str);
		Self { ctrl, raw_ctrl: false, alt, shift, key: normalized }
	}

	pub fn new_raw_ctrl(raw_ctrl: bool, alt: bool, shift: bool, key: impl Into<String>) -> Self {
		let key_str = key.into();
		let normalized = Self::normalize_key_name(&key_str);
		Self { ctrl: false, raw_ctrl, alt, shift, key: normalized }
	}

	pub fn normalize_key_name(key: &str) -> String {
		let trimmed = key.trim();
		if trimmed.eq_ignore_ascii_case("return") || trimmed.eq_ignore_ascii_case("enter") {
			"Enter".to_string()
		} else if trimmed.eq_ignore_ascii_case("space") {
			"Space".to_string()
		} else if trimmed.eq_ignore_ascii_case("tab") {
			"Tab".to_string()
		} else if trimmed.eq_ignore_ascii_case("backspace") || trimmed.eq_ignore_ascii_case("back") {
			"Backspace".to_string()
		} else if trimmed.eq_ignore_ascii_case("delete") || trimmed.eq_ignore_ascii_case("del") {
			"Delete".to_string()
		} else if trimmed.eq_ignore_ascii_case("escape") || trimmed.eq_ignore_ascii_case("esc") {
			"Escape".to_string()
		} else if trimmed.eq_ignore_ascii_case("home") {
			"Home".to_string()
		} else if trimmed.eq_ignore_ascii_case("end") {
			"End".to_string()
		} else if trimmed.eq_ignore_ascii_case("pageup")
			|| trimmed.eq_ignore_ascii_case("page up")
			|| trimmed.eq_ignore_ascii_case("pgup")
		{
			"PageUp".to_string()
		} else if trimmed.eq_ignore_ascii_case("pagedown")
			|| trimmed.eq_ignore_ascii_case("page down")
			|| trimmed.eq_ignore_ascii_case("pgdn")
		{
			"PageDown".to_string()
		} else if trimmed.eq_ignore_ascii_case("left") || trimmed.eq_ignore_ascii_case("left arrow") {
			"Left".to_string()
		} else if trimmed.eq_ignore_ascii_case("right") || trimmed.eq_ignore_ascii_case("right arrow") {
			"Right".to_string()
		} else if trimmed.eq_ignore_ascii_case("up") || trimmed.eq_ignore_ascii_case("up arrow") {
			"Up".to_string()
		} else if trimmed.eq_ignore_ascii_case("down") || trimmed.eq_ignore_ascii_case("down arrow") {
			"Down".to_string()
		} else if trimmed.len() >= 2
			&& trimmed.starts_with(['F', 'f'])
			&& trimmed[1..].chars().all(|c| c.is_ascii_digit())
		{
			format!("F{}", &trimmed[1..])
		} else if trimmed.len() == 1 {
			let ch = trimmed.chars().next().unwrap();
			if ch.is_ascii_alphabetic() { ch.to_ascii_uppercase().to_string() } else { trimmed.to_string() }
		} else {
			trimmed.to_string()
		}
	}

	pub fn to_shortcut_string(&self) -> String {
		let mut parts = Vec::new();
		if self.raw_ctrl {
			parts.push("RawCtrl");
		}
		if self.ctrl {
			parts.push("Ctrl");
		}
		if self.alt {
			parts.push("Alt");
		}
		if self.shift {
			parts.push("Shift");
		}
		parts.push(&self.key);
		parts.join("+")
	}

	pub fn parse(input: &str) -> Option<Self> {
		let trimmed = input.trim();
		if trimmed.is_empty() || trimmed.eq_ignore_ascii_case("none") {
			return None;
		}
		let mut ctrl = false;
		let mut raw_ctrl = false;
		let mut alt = false;
		let mut shift = false;
		let mut remaining = trimmed;

		while let Some(plus_idx) = remaining.find('+') {
			let prefix = &remaining[..plus_idx];
			if prefix.eq_ignore_ascii_case("rawctrl") {
				raw_ctrl = true;
				remaining = &remaining[plus_idx + 1..];
			} else if prefix.eq_ignore_ascii_case("ctrl") || prefix.eq_ignore_ascii_case("control") {
				ctrl = true;
				remaining = &remaining[plus_idx + 1..];
			} else if prefix.eq_ignore_ascii_case("alt") {
				alt = true;
				remaining = &remaining[plus_idx + 1..];
			} else if prefix.eq_ignore_ascii_case("shift") {
				shift = true;
				remaining = &remaining[plus_idx + 1..];
			} else {
				break;
			}
		}
		let key = remaining.to_string();
		if key.is_empty() {
			return None;
		}
		let normalized = Self::normalize_key_name(&key);
		Some(Self { ctrl, raw_ctrl, alt, shift, key: normalized })
	}

	pub fn from_key_code(key_code: i32, ctrl: bool, alt: bool, shift: bool) -> Option<Self> {
		let key_name = match key_code {
			13 | 370 => "Enter".to_string(),
			9 => "Tab".to_string(),
			32 => "Space".to_string(),
			8 => "Backspace".to_string(),
			127 | 308 | 386 => "Delete".to_string(),
			27 => "Escape".to_string(),
			313 | 377 => "Home".to_string(),
			312 | 379 => "End".to_string(),
			366 | 376 => "PageUp".to_string(),
			367 | 381 => "PageDown".to_string(),
			314 | 378 => "Left".to_string(),
			316 | 380 => "Right".to_string(),
			315 | 382 => "Up".to_string(),
			317 | 383 => "Down".to_string(),
			340..=363 => format!("F{}", key_code - 340 + 1),
			65..=90 => (char::from_u32(key_code as u32)?).to_string(),
			97..=122 => (char::from_u32((key_code - 32) as u32)?).to_string(),
			48..=57 => (char::from_u32(key_code as u32)?).to_string(),
			324..=333 => (char::from_u32((key_code - 324 + 48) as u32)?).to_string(),
			44 | 188 => ",".to_string(),
			46 | 190 | 387 => ".".to_string(),
			47 | 191 | 388 => "/".to_string(),
			91 | 219 => "[".to_string(),
			93 | 221 => "]".to_string(),
			92 | 220 => "\\".to_string(),
			45 | 189 | 390 => "-".to_string(),
			61 | 187 => "=".to_string(),
			59 | 186 => ";".to_string(),
			39 | 222 => "'".to_string(),
			96 | 192 => "`".to_string(),
			_ => return None,
		};
		Some(Self { ctrl, raw_ctrl: false, alt, shift, key: key_name })
	}

	pub fn matches(&self, key_code: i32, ctrl: bool, alt: bool, shift: bool) -> bool {
		let self_ctrl = self.ctrl || self.raw_ctrl;
		if self_ctrl != ctrl || self.alt != alt || self.shift != shift {
			return false;
		}
		let key_str = self.key.as_str();
		if key_str.eq_ignore_ascii_case("Enter") {
			key_code == 13 || key_code == 370
		} else if key_str.eq_ignore_ascii_case("Tab") {
			key_code == 9
		} else if key_str.eq_ignore_ascii_case("Space") {
			key_code == 32
		} else if key_str.eq_ignore_ascii_case("Backspace") {
			key_code == 8
		} else if key_str.eq_ignore_ascii_case("Delete") {
			key_code == 127 || key_code == 308 || key_code == 386
		} else if key_str.eq_ignore_ascii_case("Escape") {
			key_code == 27
		} else if key_str.eq_ignore_ascii_case("Home") {
			key_code == 313 || key_code == 377
		} else if key_str.eq_ignore_ascii_case("End") {
			key_code == 312 || key_code == 379
		} else if key_str.eq_ignore_ascii_case("PageUp") {
			key_code == 366 || key_code == 376
		} else if key_str.eq_ignore_ascii_case("PageDown") {
			key_code == 367 || key_code == 381
		} else if key_str.eq_ignore_ascii_case("Left") {
			key_code == 314 || key_code == 378
		} else if key_str.eq_ignore_ascii_case("Right") {
			key_code == 316 || key_code == 380
		} else if key_str.eq_ignore_ascii_case("Up") {
			key_code == 315 || key_code == 382
		} else if key_str.eq_ignore_ascii_case("Down") {
			key_code == 317 || key_code == 383
		} else if key_str.starts_with(['F', 'f'])
			&& let Ok(num) = key_str[1..].parse::<i32>()
			&& (1..=24).contains(&num)
		{
			key_code == 340 + num - 1
		} else if key_str.len() == 1 {
			let ch = key_str.chars().next().unwrap();
			if ch.is_ascii_alphabetic() {
				let upper = ch.to_ascii_uppercase() as i32;
				key_code == upper || key_code == upper + 32
			} else if ch.is_ascii_digit() {
				key_code == ch as i32 || key_code == (ch as i32 - 48 + 324)
			} else {
				match ch {
					',' => key_code == 44 || key_code == 188,
					'.' => key_code == 46 || key_code == 190 || key_code == 387,
					'/' => key_code == 47 || key_code == 191 || key_code == 388,
					'\\' => key_code == 92 || key_code == 220,
					'[' => key_code == 91 || key_code == 219,
					']' => key_code == 93 || key_code == 221,
					'-' => key_code == 45 || key_code == 189 || key_code == 390,
					'=' => key_code == 61 || key_code == 187,
					';' => key_code == 59 || key_code == 186,
					'\'' => key_code == 39 || key_code == 222,
					'`' => key_code == 96 || key_code == 192,
					_ => key_code == ch as i32,
				}
			}
		} else {
			false
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ShortcutsConfig {
	#[serde(default)]
	pub bindings: HashMap<ActionId, Option<String>>,
}

impl ShortcutsConfig {
	pub fn get_chord(&self, action: ActionId) -> Option<KeyChord> {
		if let Some(entry) = self.bindings.get(&action) {
			match entry {
				Some(s) => KeyChord::parse(s),
				None => None,
			}
		} else {
			action.default_chord()
		}
	}

	pub fn get_display_str(&self, action: ActionId) -> String {
		self.get_chord(action).map_or_else(|| "None".to_string(), |c| c.to_shortcut_string())
	}

	pub fn get_menu_str(&self, action: ActionId) -> String {
		self.get_chord(action).map_or_else(String::new, |c| c.to_shortcut_string())
	}

	pub fn set_chord(&mut self, action: ActionId, chord: Option<KeyChord>) {
		self.bindings.insert(action, chord.map(|c| c.to_shortcut_string()));
	}

	pub fn reset_action(&mut self, action: ActionId) {
		self.bindings.remove(&action);
	}

	pub fn reset_category(&mut self, category: ShortcutCategory) {
		for action in category.actions() {
			self.bindings.remove(&action);
		}
	}

	pub fn reset_all(&mut self) {
		self.bindings.clear();
	}

	pub fn find_action(&self, key_code: i32, ctrl: bool, alt: bool, shift: bool) -> Option<ActionId> {
		for &action in ActionId::all() {
			if let Some(chord) = self.get_chord(action)
				&& chord.matches(key_code, ctrl, alt, shift)
			{
				return Some(action);
			}
		}
		None
	}
}

const fn default_true() -> bool {
	true
}
const fn default_recent_documents_to_show() -> i64 {
	DEFAULT_RECENT_DOCUMENTS_TO_SHOW
}
const fn default_sleep_timer() -> i64 {
	30
}
const fn default_reading_speed_wpm() -> i64 {
	150
}
const fn default_font_color() -> i64 {
	-1
}
const fn default_bg_color() -> i64 {
	-1
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppSettings {
	#[serde(default = "default_true")]
	pub restore_previous_documents: bool,
	#[serde(default)]
	pub word_wrap: bool,
	#[serde(default = "default_true")]
	pub render_tables_inline: bool,
	#[serde(default)]
	pub navigation_wrap: bool,
	#[serde(default)]
	pub find_match_case: bool,
	#[serde(default)]
	pub find_whole_word: bool,
	#[serde(default)]
	pub find_use_regex: bool,
	#[serde(default = "default_recent_documents_to_show")]
	pub recent_documents_to_show: i64,
	#[serde(default = "default_sleep_timer")]
	pub sleep_timer_duration: i64,
	#[serde(default = "default_reading_speed_wpm")]
	pub reading_speed_wpm: i64,
	#[serde(default)]
	pub font_face_name: String,
	#[serde(default)]
	pub font_point_size: i64,
	#[serde(default)]
	pub font_style: i64,
	#[serde(default)]
	pub font_weight: i64,
	#[serde(default)]
	pub font_underlined: bool,
	#[serde(default)]
	pub font_strikethrough: bool,
	#[serde(default = "default_font_color")]
	pub font_color: i64,
	#[serde(default = "default_bg_color")]
	pub bg_color: i64,
	#[serde(default)]
	pub text_alignment: i64,
	#[serde(default)]
	pub letter_spacing: i64,
	#[serde(default)]
	pub paragraph_spacing: i64,
	#[serde(default)]
	pub line_spacing: i64,
	#[serde(default)]
	pub hotkey: HotkeyConfig,
	#[serde(default)]
	pub shortcuts: ShortcutsConfig,
	/// Pass-through storage for host-specific settings (e.g. desktop UI preferences).
	/// Keys written here are preserved on read/write so host consumers can store their
	/// own fields alongside the generic ones without conflict.
	#[serde(flatten, default)]
	pub extra: HashMap<String, toml::Value>,
}

impl Default for AppSettings {
	fn default() -> Self {
		Self {
			restore_previous_documents: true,
			word_wrap: false,
			render_tables_inline: true,
			navigation_wrap: false,
			find_match_case: false,
			find_whole_word: false,
			find_use_regex: false,
			recent_documents_to_show: DEFAULT_RECENT_DOCUMENTS_TO_SHOW,
			sleep_timer_duration: 30,
			reading_speed_wpm: 150,
			font_face_name: String::new(),
			font_point_size: 0,
			font_style: 0,
			font_weight: 0,
			font_underlined: false,
			font_strikethrough: false,
			font_color: -1,
			bg_color: -1,
			text_alignment: 0,
			letter_spacing: 0,
			paragraph_spacing: 0,
			line_spacing: 0,
			hotkey: HotkeyConfig::default(),
			shortcuts: ShortcutsConfig::default(),
			extra: HashMap::new(),
		}
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct DocumentConfig {
	pub path: String,
	#[serde(default)]
	pub last_position: i64,
	#[serde(default)]
	pub navigation_history: Vec<i64>,
	#[serde(default)]
	pub navigation_history_index: usize,
	#[serde(default)]
	pub bookmarks: Vec<StoredBookmark>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub temporary_bookmark: Option<i64>,
	#[serde(default, skip_serializing_if = "String::is_empty")]
	pub format: String,
	#[serde(default, skip_serializing_if = "String::is_empty")]
	pub password: String,
	#[serde(default)]
	pub opened: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
struct SidecarData {
	#[serde(default, skip_serializing_if = "Option::is_none")]
	last_position: Option<i64>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	format: Option<String>,
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	bookmarks: Vec<StoredBookmark>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	temporary_bookmark: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ConfigData {
	pub version: u32,
	#[serde(default)]
	pub app: AppSettings,
	#[serde(default)]
	pub recent_documents: Vec<String>,
	#[serde(default)]
	pub opened_documents: Vec<String>,
	#[serde(default)]
	pub find_history: Vec<String>,
	#[serde(default)]
	pub documents: HashMap<String, DocumentConfig>,
	#[serde(default)]
	pub path_hashes: HashMap<String, String>,
}

impl Default for ConfigData {
	fn default() -> Self {
		Self {
			version: CONFIG_VERSION,
			app: AppSettings::default(),
			recent_documents: Vec::new(),
			opened_documents: Vec::new(),
			find_history: Vec::new(),
			documents: HashMap::new(),
			path_hashes: HashMap::new(),
		}
	}
}

pub struct ConfigManager {
	data: RefCell<ConfigData>,
	config_path: PathBuf,
	dirty: Cell<bool>,
	initialized: bool,
}

impl Default for ConfigManager {
	fn default() -> Self {
		Self::new()
	}
}

impl ConfigManager {
	#[must_use]
	pub fn new() -> Self {
		Self {
			data: RefCell::new(ConfigData::default()),
			config_path: PathBuf::new(),
			dirty: Cell::new(false),
			initialized: false,
		}
	}

	pub fn initialize(&mut self, config_path: PathBuf) -> bool {
		let (data, needs_save) = if config_path.exists() {
			fs::read_to_string(&config_path)
				.ok()
				.and_then(|s| toml::from_str::<ConfigData>(&s).ok())
				.map_or_else(|| (ConfigData::default(), true), |d| (d, false))
		} else {
			(ConfigData::default(), true)
		};

		self.config_path = config_path;
		self.initialized = true;
		*self.data.borrow_mut() = data;

		if needs_save {
			self.dirty.set(true);
			self.flush();
		}

		true
	}

	pub fn refresh_document_hash(&self, path: &str) {
		if !self.initialized {
			return;
		}
		{
			let data = self.data.borrow();
			if data.path_hashes.contains_key(path) {
				return;
			}
		}
		let digest = compute_document_hash(path);
		let encoded = URL_SAFE_NO_PAD.encode(digest);
		let new_key = format!("doc_{encoded}");

		let mut data = self.data.borrow_mut();
		if let Some(old_key) = data.path_hashes.get(path).cloned() {
			if old_key != new_key {
				if let Some(mut doc) = data.documents.remove(&old_key) {
					doc.path = path.to_string();
					data.documents.insert(new_key.clone(), doc);
				}
				data.path_hashes.insert(path.to_string(), new_key);
				self.dirty.set(true);
			}
		} else {
			if !data.documents.contains_key(&new_key) {
				let mut old_hasher = Sha1::new();
				old_hasher.update(path.as_bytes());
				let old_encoded = URL_SAFE_NO_PAD.encode(old_hasher.finalize());
				let old_key = format!("doc_{old_encoded}");

				if let Some(mut doc) = data.documents.remove(&old_key) {
					doc.path = path.to_string();
					data.documents.insert(new_key.clone(), doc);
				}
			}
			data.path_hashes.insert(path.to_string(), new_key);
			self.dirty.set(true);
		}
	}

	pub fn associate_uri_with_local_file(&self, uri: &str, local_path: &str) {
		let digest = compute_document_hash(local_path);
		let encoded = URL_SAFE_NO_PAD.encode(digest);
		let new_key = format!("doc_{encoded}");

		let mut data = self.data.borrow_mut();
		data.path_hashes.insert(uri.to_string(), new_key);
		self.dirty.set(true);
	}

	pub fn get_doc_key(&self, path: &str) -> String {
		{
			let data = self.data.borrow();
			if let Some(hash) = data.path_hashes.get(path) {
				return hash.clone();
			}
		}

		let digest = compute_document_hash(path);
		let encoded = URL_SAFE_NO_PAD.encode(digest);
		let new_key = format!("doc_{encoded}");

		let mut data = self.data.borrow_mut();
		if !data.documents.contains_key(&new_key) {
			let mut old_hasher = Sha1::new();
			old_hasher.update(path.as_bytes());
			let old_encoded = URL_SAFE_NO_PAD.encode(old_hasher.finalize());
			let old_key = format!("doc_{old_encoded}");

			if let Some(doc) = data.documents.remove(&old_key) {
				data.documents.insert(new_key.clone(), doc);
			}
		}

		data.path_hashes.insert(path.to_string(), new_key.clone());
		self.dirty.set(true);
		new_key
	}

	pub fn flush(&self) {
		if !self.initialized || !self.dirty.get() {
			return;
		}
		let data = self.data.borrow();
		if let Ok(s) = toml::to_string_pretty(&*data) {
			let _ = fs::write(&self.config_path, s);
			self.dirty.set(false);
		}
	}

	pub fn get_app_string(&self, key: &str, default_value: &str) -> String {
		if !self.initialized {
			return default_value.to_string();
		}
		let data = self.data.borrow();
		data.app.extra.get(key).and_then(|v| v.as_str()).map_or_else(|| default_value.to_string(), str::to_string)
	}

	pub fn get_app_bool(&self, key: &str, default_value: bool) -> bool {
		if !self.initialized {
			return default_value;
		}
		let data = self.data.borrow();
		match key {
			"restore_previous_documents" => data.app.restore_previous_documents,
			"word_wrap" => data.app.word_wrap,
			"render_tables_inline" => data.app.render_tables_inline,
			"navigation_wrap" => data.app.navigation_wrap,
			"find_match_case" => data.app.find_match_case,
			"find_whole_word" => data.app.find_whole_word,
			"find_use_regex" => data.app.find_use_regex,
			_ => data.app.extra.get(key).and_then(toml::Value::as_bool).unwrap_or(default_value),
		}
	}

	pub fn get_app_int(&self, key: &str, default_value: i32) -> i32 {
		if !self.initialized {
			return default_value;
		}
		let data = self.data.borrow();
		let v: i64 = match key {
			"recent_documents_to_show" => data.app.recent_documents_to_show,
			"sleep_timer_duration" => data.app.sleep_timer_duration,
			"reading_speed_wpm" => data.app.reading_speed_wpm,
			_ => {
				return data
					.app
					.extra
					.get(key)
					.and_then(toml::Value::as_integer)
					.and_then(|i| i32::try_from(i).ok())
					.unwrap_or(default_value);
			}
		};
		v.try_into().unwrap_or(default_value)
	}

	pub fn set_app_string(&self, key: &str, value: &str) {
		if !self.initialized {
			return;
		}
		self.data.borrow_mut().app.extra.insert(key.to_string(), toml::Value::String(value.to_string()));
		self.dirty.set(true);
	}

	pub fn set_app_bool(&self, key: &str, value: bool) {
		if !self.initialized {
			return;
		}
		{
			let mut data = self.data.borrow_mut();
			match key {
				"restore_previous_documents" => data.app.restore_previous_documents = value,
				"word_wrap" => data.app.word_wrap = value,
				"render_tables_inline" => data.app.render_tables_inline = value,
				"navigation_wrap" => data.app.navigation_wrap = value,
				"find_match_case" => data.app.find_match_case = value,
				"find_whole_word" => data.app.find_whole_word = value,
				"find_use_regex" => data.app.find_use_regex = value,
				_ => {
					data.app.extra.insert(key.to_string(), toml::Value::Boolean(value));
				}
			}
		}
		self.dirty.set(true);
	}

	pub fn set_app_int(&self, key: &str, value: i32) {
		if !self.initialized {
			return;
		}
		{
			let mut data = self.data.borrow_mut();
			match key {
				"recent_documents_to_show" => data.app.recent_documents_to_show = i64::from(value),
				"sleep_timer_duration" => data.app.sleep_timer_duration = i64::from(value),
				"reading_speed_wpm" => data.app.reading_speed_wpm = i64::from(value),
				_ => {
					data.app.extra.insert(key.to_string(), toml::Value::Integer(i64::from(value)));
				}
			}
		}
		self.dirty.set(true);
	}

	pub fn get_readability_font(&self) -> ReadabilityFont {
		if !self.initialized {
			return ReadabilityFont::default();
		}
		let data = self.data.borrow();
		ReadabilityFont {
			face_name: data.app.font_face_name.clone(),
			point_size: data.app.font_point_size.try_into().unwrap_or(0),
			style: data.app.font_style.try_into().unwrap_or(0),
			weight: data.app.font_weight.try_into().unwrap_or(0),
			underlined: data.app.font_underlined,
			strikethrough: data.app.font_strikethrough,
			color: data.app.font_color.try_into().unwrap_or(-1),
			encoding: data
				.app
				.extra
				.get("font_encoding")
				.and_then(toml::Value::as_integer)
				.and_then(|i| i32::try_from(i).ok())
				.unwrap_or(0),
		}
	}

	pub fn set_readability_font(&self, font: &ReadabilityFont) {
		if !self.initialized {
			return;
		}
		{
			let mut data = self.data.borrow_mut();
			data.app.font_face_name = font.face_name.clone();
			data.app.font_point_size = i64::from(font.point_size);
			data.app.font_style = i64::from(font.style);
			data.app.font_weight = i64::from(font.weight);
			data.app.font_underlined = font.underlined;
			data.app.font_strikethrough = font.strikethrough;
			data.app.font_color = i64::from(font.color);
			data.app.extra.insert("font_encoding".to_string(), toml::Value::Integer(i64::from(font.encoding)));
		}
		self.dirty.set(true);
	}

	pub fn get_line_spacing(&self) -> i32 {
		if !self.initialized {
			return 0;
		}
		self.data.borrow().app.line_spacing.try_into().unwrap_or(0)
	}

	pub fn set_line_spacing(&self, value: i32) {
		if !self.initialized {
			return;
		}
		self.data.borrow_mut().app.line_spacing = i64::from(value);
		self.dirty.set(true);
	}

	pub fn get_bg_color(&self) -> i32 {
		if !self.initialized {
			return -1;
		}
		self.data.borrow().app.bg_color.try_into().unwrap_or(-1)
	}

	pub fn set_bg_color(&self, color: i32) {
		if !self.initialized {
			return;
		}
		self.data.borrow_mut().app.bg_color = i64::from(color);
		self.dirty.set(true);
	}

	pub fn get_text_alignment(&self) -> i32 {
		if !self.initialized {
			return 0;
		}
		self.data.borrow().app.text_alignment.try_into().unwrap_or(0)
	}

	pub fn set_text_alignment(&self, value: i32) {
		if !self.initialized {
			return;
		}
		self.data.borrow_mut().app.text_alignment = i64::from(value);
		self.dirty.set(true);
	}

	pub fn get_letter_spacing(&self) -> i32 {
		if !self.initialized {
			return 0;
		}
		self.data.borrow().app.letter_spacing.clamp(0, 2).try_into().unwrap_or(0)
	}

	pub fn set_letter_spacing(&self, value: i32) {
		if !self.initialized {
			return;
		}
		self.data.borrow_mut().app.letter_spacing = i64::from(value);
		self.dirty.set(true);
	}

	pub fn get_paragraph_spacing(&self) -> i32 {
		if !self.initialized {
			return 0;
		}
		self.data.borrow().app.paragraph_spacing.clamp(0, 2).try_into().unwrap_or(0)
	}

	pub fn set_paragraph_spacing(&self, value: i32) {
		if !self.initialized {
			return;
		}
		self.data.borrow_mut().app.paragraph_spacing = i64::from(value);
		self.dirty.set(true);
	}

	pub fn get_hotkey(&self) -> HotkeyConfig {
		if !self.initialized {
			return HotkeyConfig::default();
		}
		self.data.borrow().app.hotkey.clone()
	}

	pub fn set_hotkey(&self, hotkey: &HotkeyConfig) {
		if !self.initialized {
			return;
		}
		self.data.borrow_mut().app.hotkey = hotkey.clone();
		self.dirty.set(true);
	}

	pub fn get_shortcuts(&self) -> ShortcutsConfig {
		if !self.initialized {
			return ShortcutsConfig::default();
		}
		self.data.borrow().app.shortcuts.clone()
	}

	pub fn set_shortcuts(&self, shortcuts: &ShortcutsConfig) {
		if !self.initialized {
			return;
		}
		self.data.borrow_mut().app.shortcuts = shortcuts.clone();
		self.dirty.set(true);
	}

	pub fn get_shortcut_chord(&self, action: ActionId) -> Option<KeyChord> {
		self.get_shortcuts().get_chord(action)
	}

	pub fn get_shortcut_menu_str(&self, action: ActionId) -> String {
		self.get_shortcuts().get_menu_str(action)
	}

	pub fn add_recent_document(&self, path: &str) {
		if !self.initialized {
			return;
		}
		{
			let key = self.get_doc_key(path);
			let mut data = self.data.borrow_mut();
			Self::doc_entry_mut(&mut data, key, path);
			if let Some(idx) = data.recent_documents.iter().position(|p| p == path) {
				data.recent_documents.remove(idx);
			}
			data.recent_documents.insert(0, path.to_string());
			while data.recent_documents.len() > MAX_RECENT_DOCUMENTS_TO_SHOW {
				data.recent_documents.pop();
			}
		}
		self.dirty.set(true);
	}

	pub fn get_recent_documents(&self) -> Vec<String> {
		if !self.initialized {
			return Vec::new();
		}
		self.data.borrow().recent_documents.clone()
	}

	pub fn add_opened_document(&self, path: &str) {
		if !self.initialized {
			return;
		}
		{
			let mut data = self.data.borrow_mut();
			if !data.opened_documents.iter().any(|p| p == path) {
				data.opened_documents.push(path.to_string());
			}
		}
		self.dirty.set(true);
	}

	pub fn remove_opened_document(&self, path: &str) {
		if !self.initialized {
			return;
		}
		{
			let mut data = self.data.borrow_mut();
			if let Some(idx) = data.opened_documents.iter().position(|p| p == path) {
				data.opened_documents.remove(idx);
			}
		}
		self.dirty.set(true);
	}

	pub fn get_opened_documents(&self) -> Vec<String> {
		if !self.initialized {
			return Vec::new();
		}
		self.data.borrow().opened_documents.clone()
	}

	pub fn get_opened_documents_existing(&self) -> Vec<String> {
		self.get_opened_documents().into_iter().filter(|path| Path::new(path).exists()).collect()
	}

	pub fn get_find_settings(&self) -> FindSettings {
		FindSettings {
			match_case: self.get_app_bool("find_match_case", false),
			whole_word: self.get_app_bool("find_whole_word", false),
			use_regex: self.get_app_bool("find_use_regex", false),
		}
	}

	pub fn set_find_settings(&self, settings: FindSettings) {
		self.set_app_bool("find_match_case", settings.match_case);
		self.set_app_bool("find_whole_word", settings.whole_word);
		self.set_app_bool("find_use_regex", settings.use_regex);
	}

	pub fn get_find_history(&self) -> Vec<String> {
		if !self.initialized {
			return Vec::new();
		}
		self.data.borrow().find_history.clone()
	}

	pub fn add_find_history(&self, text: &str, max_len: usize) {
		if !self.initialized {
			return;
		}
		let trimmed = text.trim().to_string();
		if trimmed.is_empty() {
			return;
		}
		{
			let mut data = self.data.borrow_mut();
			if let Some(idx) = data.find_history.iter().position(|e| e == &trimmed) {
				data.find_history.remove(idx);
			}
			data.find_history.insert(0, trimmed);
			while data.find_history.len() > max_len {
				data.find_history.pop();
			}
		}
		self.dirty.set(true);
	}

	pub fn set_document_position(&self, path: &str, position: i64) {
		if !self.initialized {
			return;
		}
		{
			let key = self.get_doc_key(path);
			let mut data = self.data.borrow_mut();
			Self::doc_entry_mut(&mut data, key, path).last_position = position;
		}
		self.dirty.set(true);
	}

	#[must_use]
	pub fn get_document_position(&self, path: &str) -> i64 {
		if !self.initialized {
			return 0;
		}
		let key = self.get_doc_key(path);
		self.data.borrow().documents.get(&key).map_or(0, |d| d.last_position)
	}

	#[must_use]
	pub fn get_validated_document_position(&self, path: &str, max_position: i64) -> i64 {
		let saved = self.get_document_position(path);
		if saved > 0 && saved <= max_position { saved } else { -1 }
	}

	/// Sets the single per-document temporary bookmark position (`None` clears it).
	pub fn set_temporary_bookmark(&self, path: &str, position: Option<i64>) {
		if !self.initialized {
			return;
		}
		{
			let key = self.get_doc_key(path);
			let mut data = self.data.borrow_mut();
			Self::doc_entry_mut(&mut data, key, path).temporary_bookmark = position;
		}
		self.dirty.set(true);
	}

	#[must_use]
	pub fn get_temporary_bookmark(&self, path: &str) -> Option<i64> {
		if !self.initialized {
			return None;
		}
		let key = self.get_doc_key(path);
		self.data.borrow().documents.get(&key).and_then(|d| d.temporary_bookmark)
	}

	pub fn set_navigation_history(&self, path: &str, history: &[i64], history_index: usize) {
		if !self.initialized {
			return;
		}
		{
			let key = self.get_doc_key(path);
			let mut data = self.data.borrow_mut();
			let doc = Self::doc_entry_mut(&mut data, key, path);
			doc.navigation_history = history.to_vec();
			doc.navigation_history_index = history_index;
		}
		self.dirty.set(true);
	}

	pub fn get_navigation_history(&self, path: &str) -> NavigationHistory {
		let mut nav = NavigationHistory::default();
		if !self.initialized {
			return nav;
		}
		let key = self.get_doc_key(path);
		if let Some(doc) = self.data.borrow().documents.get(&key) {
			nav.positions = doc.navigation_history.clone();
			nav.index = doc.navigation_history_index;
		}
		nav
	}

	/// Sets the per-document opened flag. Prefer `add_opened_document` /`remove_opened_document` for maintaining the opened-documents list.
	pub fn set_document_opened(&self, path: &str, opened: bool) {
		if !self.initialized {
			return;
		}
		{
			let key = self.get_doc_key(path);
			let mut data = self.data.borrow_mut();
			Self::doc_entry_mut(&mut data, key, path).opened = opened;
		}
		self.dirty.set(true);
	}

	pub fn remove_document_history(&self, path: &str) {
		if !self.initialized {
			return;
		}
		{
			let key = self.get_doc_key(path);
			let mut data = self.data.borrow_mut();
			if let Some(idx) = data.recent_documents.iter().position(|p| p == path) {
				data.recent_documents.remove(idx);
			}
			data.documents.remove(&key);
		}
		self.dirty.set(true);
	}

	pub fn rename_document_path(&self, old_path: &str, new_path: &str) {
		if !self.initialized {
			return;
		}
		let mut data = self.data.borrow_mut();
		for p in &mut data.recent_documents {
			if p == old_path {
				*p = new_path.to_string();
			}
		}
		for p in &mut data.opened_documents {
			if p == old_path {
				*p = new_path.to_string();
			}
		}
		if let Some(doc_key) = data.path_hashes.remove(old_path) {
			data.path_hashes.insert(new_path.to_string(), doc_key.clone());
			if let Some(doc) = data.documents.get_mut(&doc_key) {
				doc.path = new_path.to_string();
			}
		}
		self.dirty.set(true);
	}

	pub fn get_all_documents(&self) -> Vec<String> {
		if !self.initialized {
			return Vec::new();
		}
		self.data.borrow().documents.values().map(|d| d.path.clone()).filter(|p| !p.is_empty()).collect()
	}

	pub fn add_bookmark(&self, path: &str, start: i64, end: i64, note: &str) {
		if !self.initialized {
			return;
		}
		{
			let key = self.get_doc_key(path);
			let mut data = self.data.borrow_mut();
			let doc = Self::doc_entry_mut(&mut data, key, path);
			if doc.bookmarks.iter().any(|bm| bm.start == start && bm.end == end) {
				return;
			}
			doc.bookmarks.push(StoredBookmark { start, end, note: note.to_string() });
			doc.bookmarks.sort_by_key(|a| a.start);
		}
		self.dirty.set(true);
	}

	pub fn remove_bookmark(&self, path: &str, start: i64, end: i64) {
		if !self.initialized {
			return;
		}
		{
			let key = self.get_doc_key(path);
			let mut data = self.data.borrow_mut();
			let doc = Self::doc_entry_mut(&mut data, key, path);
			if let Some(idx) = doc.bookmarks.iter().position(|bm| bm.start == start && bm.end == end) {
				doc.bookmarks.remove(idx);
			}
		}
		self.dirty.set(true);
	}

	pub fn toggle_bookmark(&self, path: &str, start: i64, end: i64, note: &str) {
		if self.get_bookmarks(path).iter().any(|bm| bm.start == start && bm.end == end) {
			self.remove_bookmark(path, start, end);
		} else {
			self.add_bookmark(path, start, end, note);
		}
	}

	pub fn update_bookmark_note(&self, path: &str, start: i64, end: i64, note: &str) {
		if !self.initialized {
			return;
		}
		{
			let key = self.get_doc_key(path);
			let mut data = self.data.borrow_mut();
			let doc = Self::doc_entry_mut(&mut data, key, path);
			if let Some(bm) = doc.bookmarks.iter_mut().find(|bm| bm.start == start && bm.end == end) {
				bm.note = note.to_string();
			}
		}
		self.dirty.set(true);
	}

	pub fn get_bookmarks(&self, path: &str) -> Vec<Bookmark> {
		if !self.initialized {
			return Vec::new();
		}
		self.data
			.borrow()
			.documents
			.get(&self.get_doc_key(path))
			.map(|d| {
				d.bookmarks.iter().map(|bm| Bookmark { start: bm.start, end: bm.end, note: bm.note.clone() }).collect()
			})
			.unwrap_or_default()
	}

	pub fn set_document_format(&self, path: &str, format: &str) {
		if !self.initialized {
			return;
		}
		{
			let key = self.get_doc_key(path);
			let mut data = self.data.borrow_mut();
			Self::doc_entry_mut(&mut data, key, path).format = format.to_string();
		}
		self.dirty.set(true);
	}

	pub fn get_document_format(&self, path: &str) -> String {
		if !self.initialized {
			return String::new();
		}
		let key = self.get_doc_key(path);
		self.data.borrow().documents.get(&key).map(|d| d.format.clone()).unwrap_or_default()
	}

	pub fn set_document_password(&self, path: &str, password: &str) {
		if !self.initialized {
			return;
		}
		{
			let key = self.get_doc_key(path);
			let mut data = self.data.borrow_mut();
			Self::doc_entry_mut(&mut data, key, path).password = password.to_string();
		}
		self.dirty.set(true);
	}

	pub fn get_document_password(&self, path: &str) -> String {
		if !self.initialized {
			return String::new();
		}
		let key = self.get_doc_key(path);
		self.data.borrow().documents.get(&key).map(|d| d.password.clone()).unwrap_or_default()
	}

	/// Import document settings from a `.paperback` sidecar file if it exists.
	pub fn import_document_settings(&self, path: &str) {
		let import_path = Path::new(path).with_extension("paperback");
		if let Some(import_path_str) = import_path.to_str()
			&& import_path.exists()
		{
			self.import_settings_from_file(path, import_path_str);
		}
	}

	/// Import document settings from a specified TOML sidecar file.
	pub fn import_settings_from_file(&self, doc_path: &str, import_path: &str) {
		if !self.initialized || !Path::new(import_path).exists() {
			return;
		}
		let Ok(content) = fs::read_to_string(import_path) else { return };
		let Ok(sidecar) = toml::from_str::<SidecarData>(&content) else { return };
		if let Some(pos) = sidecar.last_position {
			self.set_document_position(doc_path, pos);
		}
		if let Some(format) = sidecar.format {
			self.set_document_format(doc_path, &format);
		}
		if let Some(position) = sidecar.temporary_bookmark {
			self.set_temporary_bookmark(doc_path, Some(position));
		}
		if !sidecar.bookmarks.is_empty() {
			let key = self.get_doc_key(doc_path);
			let mut data = self.data.borrow_mut();
			Self::doc_entry_mut(&mut data, key, doc_path).bookmarks = sidecar.bookmarks;
			self.dirty.set(true);
		}
	}

	/// Export document settings to a `.paperback` sidecar TOML file.
	pub fn export_document_settings(&self, doc_path: &str, export_path: &str) {
		if !self.initialized {
			return;
		}
		let key = self.get_doc_key(doc_path);
		let data = self.data.borrow();
		let doc = data.documents.get(&key);
		let sidecar = SidecarData {
			last_position: doc.map(|d| d.last_position).filter(|&p| p > 0),
			format: doc.and_then(|d| if d.format.is_empty() { None } else { Some(d.format.clone()) }),
			bookmarks: doc.map(|d| d.bookmarks.clone()).unwrap_or_default(),
			temporary_bookmark: doc.and_then(|d| d.temporary_bookmark),
		};
		if let Ok(s) = toml::to_string_pretty(&sidecar) {
			let _ = fs::write(export_path, s);
		}
	}

	fn doc_entry_mut<'a>(data: &'a mut ConfigData, key: String, path: &str) -> &'a mut DocumentConfig {
		let entry = data.documents.entry(key).or_default();
		if entry.path.is_empty() {
			entry.path = path.to_string();
		}
		entry
	}
}

impl Drop for ConfigManager {
	fn drop(&mut self) {
		if !self.initialized {
			return;
		}
		self.flush();
	}
}

pub fn get_sorted_document_list(
	config: &ConfigManager,
	open_paths: &[String],
	filter: &str,
	status_filter: Option<DocumentListStatus>,
) -> Vec<DocumentListItem> {
	let recent_docs = config.get_recent_documents();
	let all_docs = config.get_all_documents();
	let mut doc_paths: Vec<String> = Vec::new();
	for path in &recent_docs {
		if !doc_paths.contains(path) {
			doc_paths.push(path.clone());
		}
	}
	let mut rest: Vec<String> = all_docs.iter().filter(|path| !doc_paths.contains(path)).cloned().collect();
	rest.sort_by(|a, b| {
		let a_path = Path::new(a);
		let b_path = Path::new(b);
		let a_name = a_path.file_name().and_then(|n| n.to_str()).unwrap_or(a);
		let b_name = b_path.file_name().and_then(|n| n.to_str()).unwrap_or(b);
		let name_cmp = a_name.to_lowercase().cmp(&b_name.to_lowercase());
		if name_cmp != Ordering::Equal {
			return name_cmp;
		}
		a.to_lowercase().cmp(&b.to_lowercase())
	});
	doc_paths.extend(rest);
	let filter_lower = filter.to_lowercase();
	doc_paths
		.into_iter()
		.filter_map(|path| {
			let path_obj = Path::new(&path);
			let filename = path_obj.file_name().and_then(|n| n.to_str()).unwrap_or("").to_string();
			if !filter.is_empty() && !filename.to_lowercase().contains(&filter_lower) {
				return None;
			}
			let status = if !path_obj.exists() {
				DocumentListStatus::Missing
			} else if open_paths.contains(&path) {
				DocumentListStatus::Open
			} else {
				DocumentListStatus::Closed
			};
			if status_filter.is_some_and(|wanted| wanted != status) {
				return None;
			}
			Some(DocumentListItem { path, filename, status })
		})
		.collect()
}

#[must_use]
pub fn compute_document_hash(path: &str) -> [u8; 20] {
	let mut hasher = Sha1::new();
	if let Ok(mut file) = fs::File::open(path) {
		use std::io::{Read, Seek, SeekFrom};
		let mut buffer = [0; 65536];
		let mut total_read = 0;
		let max_read = 1024 * 1024;
		while total_read < max_read {
			let to_read = std::cmp::min(buffer.len(), max_read - total_read);
			if let Ok(n) = file.read(&mut buffer[..to_read]) {
				if n == 0 {
					break;
				}
				hasher.update(&buffer[..n]);
				total_read += n;
			} else {
				break;
			}
		}
		if let Ok(metadata) = file.metadata() {
			let file_size = metadata.len();
			hasher.update(file_size.to_le_bytes());
			if file_size > max_read as u64 {
				let seek_pos = std::cmp::max(file_size.saturating_sub(max_read as u64), max_read as u64);
				if file.seek(SeekFrom::Start(seek_pos)).is_ok() {
					let mut end_read = 0;
					let end_max = (file_size - seek_pos) as usize;
					while end_read < end_max {
						let to_read = std::cmp::min(buffer.len(), end_max - end_read);
						if let Ok(n) = file.read(&mut buffer[..to_read]) {
							if n == 0 {
								break;
							}
							hasher.update(&buffer[..n]);
							end_read += n;
						} else {
							break;
						}
					}
				}
			}
		}
	} else {
		hasher.update(path.as_bytes());
	}
	hasher.finalize().into()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn doc_key_is_stable_and_prefixed() {
		let mut config = ConfigManager::new();
		config.initialized = true;
		let a = config.get_doc_key("C:\\books\\a.epub");
		let b = config.get_doc_key("C:\\books\\a.epub");
		assert_eq!(a, b);
		assert!(a.starts_with("doc_"));
		assert!(!a.contains('/'));
	}

	#[test]
	fn doc_key_differs_for_different_inputs() {
		let mut config = ConfigManager::new();
		config.initialized = true;
		let a = config.get_doc_key("book-a.epub");
		let b = config.get_doc_key("book-b.epub");
		assert_ne!(a, b);
	}

	#[test]
	fn render_tables_inline_round_trips() {
		let mut config = ConfigManager::new();
		config.initialized = true;
		assert!(config.get_app_bool("render_tables_inline", true));
		config.set_app_bool("render_tables_inline", false);
		assert!(!config.get_app_bool("render_tables_inline", true));
		config.set_app_bool("render_tables_inline", true);
		assert!(config.get_app_bool("render_tables_inline", true));
	}

	#[test]
	fn temporary_bookmark_set_get_overwrite_clear() {
		let mut config = ConfigManager::new();
		config.initialized = true;
		let path = "book.epub";
		assert_eq!(config.get_temporary_bookmark(path), None);
		config.set_temporary_bookmark(path, Some(42_000));
		assert_eq!(config.get_temporary_bookmark(path), Some(42_000));
		config.set_temporary_bookmark(path, Some(43_000));
		assert_eq!(config.get_temporary_bookmark(path), Some(43_000));
		config.set_temporary_bookmark(path, None);
		assert_eq!(config.get_temporary_bookmark(path), None);
	}

	#[test]
	fn temporary_bookmark_serializes_and_loads() {
		let doc =
			DocumentConfig { path: "book.epub".into(), temporary_bookmark: Some(12_345), ..DocumentConfig::default() };
		let serialized = toml::to_string(&doc).unwrap();
		let parsed: DocumentConfig = toml::from_str(&serialized).unwrap();
		assert_eq!(parsed.temporary_bookmark, Some(12_345));
	}

	#[test]
	fn temporary_bookmark_defaults_to_none_when_missing() {
		// Old config files without the field must load as None.
		let parsed: DocumentConfig = toml::from_str("path = \"book.epub\"\n").unwrap();
		assert_eq!(parsed.temporary_bookmark, None);
	}

	#[test]
	fn key_chord_parse_and_to_string() {
		let chord = KeyChord::parse("Ctrl+Shift+O").unwrap();
		assert!(chord.ctrl);
		assert!(chord.shift);
		assert!(!chord.alt);
		assert_eq!(chord.key, "O");
		assert_eq!(chord.to_shortcut_string(), "Ctrl+Shift+O");

		let single = KeyChord::parse("H").unwrap();
		assert!(!single.ctrl);
		assert!(!single.shift);
		assert!(!single.alt);
		assert_eq!(single.key, "H");
		assert_eq!(single.to_shortcut_string(), "H");

		assert_eq!(KeyChord::parse("none"), None);
		assert_eq!(KeyChord::parse(""), None);
	}

	#[test]
	fn shortcuts_config_set_reset_and_find() {
		let mut sc = ShortcutsConfig::default();
		let default_open = sc.get_chord(ActionId::Open);
		assert!(default_open.is_some());

		let new_chord = KeyChord::new(true, true, false, "K");
		sc.set_chord(ActionId::Open, Some(new_chord.clone()));
		assert_eq!(sc.get_chord(ActionId::Open), Some(new_chord));

		let matched = sc.find_action(75, true, true, false);
		assert_eq!(matched, Some(ActionId::Open));

		sc.reset_action(ActionId::Open);
		assert_eq!(sc.get_chord(ActionId::Open), default_open);
	}

	#[test]
	fn shortcut_category_actions_coverage() {
		let mut total_actions = 0;
		for cat in ShortcutCategory::all() {
			let actions = cat.actions();
			assert!(!actions.is_empty());
			for action in &actions {
				assert_eq!(action.category(), *cat);
			}
			total_actions += actions.len();
		}
		assert_eq!(total_actions, ActionId::all().len());
	}

	#[test]
	fn get_sorted_document_list_filters_by_status() {
		use crate::util::test_support::TempDir;

		let dir = TempDir::new("document-list-status-filter");
		let open_path = dir.write_str("open.txt", "content");
		let closed_path = dir.write_str("closed.txt", "content");
		let missing_path = dir.join_str("missing.txt");
		let mut config = ConfigManager::new();
		config.initialized = true;
		for path in [&open_path, &closed_path, &missing_path] {
			config.add_recent_document(path);
		}
		let open_paths = vec![open_path.clone()];

		let all = get_sorted_document_list(&config, &open_paths, "", None);
		assert_eq!(all.len(), 3);

		let open_only = get_sorted_document_list(&config, &open_paths, "", Some(DocumentListStatus::Open));
		assert_eq!(open_only.iter().map(|item| &item.path).collect::<Vec<_>>(), vec![&open_path]);

		let closed_only = get_sorted_document_list(&config, &open_paths, "", Some(DocumentListStatus::Closed));
		assert_eq!(closed_only.iter().map(|item| &item.path).collect::<Vec<_>>(), vec![&closed_path]);

		let missing_only = get_sorted_document_list(&config, &open_paths, "", Some(DocumentListStatus::Missing));
		assert_eq!(missing_only.iter().map(|item| &item.path).collect::<Vec<_>>(), vec![&missing_path]);
	}
}
