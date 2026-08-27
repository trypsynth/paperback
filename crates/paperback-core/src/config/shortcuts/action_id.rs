//! [`ActionId`] names every user-triggerable action and supplies its category, display name,
//! and default key chord.

use super::{KeyChord, ShortcutCategory};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
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
	PlayPauseAudio,
	SeekAudioForward,
	SeekAudioBackward,
	IncreaseAudioSeekAmount,
	DecreaseAudioSeekAmount,
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
			Self::PlayPauseAudio,
			Self::SeekAudioForward,
			Self::SeekAudioBackward,
			Self::IncreaseAudioSeekAmount,
			Self::DecreaseAudioSeekAmount,
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
			| Self::PlayPauseAudio
			| Self::SeekAudioForward
			| Self::SeekAudioBackward
			| Self::IncreaseAudioSeekAmount
			| Self::DecreaseAudioSeekAmount
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
			Self::PlayPauseAudio => "Play/Pause Audio",
			Self::SeekAudioForward => "Seek Audio Forward",
			Self::SeekAudioBackward => "Seek Audio Backward",
			Self::IncreaseAudioSeekAmount => "Increase Audio Seek Amount",
			Self::DecreaseAudioSeekAmount => "Decrease Audio Seek Amount",
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
			Self::PlayPauseAudio => Some(KeyChord::new(true, false, false, "Space")),
			Self::SeekAudioForward => Some(KeyChord::new(false, false, false, "'")),
			Self::SeekAudioBackward => Some(KeyChord::new(false, false, false, ";")),
			Self::IncreaseAudioSeekAmount => Some(KeyChord::new(false, false, true, "'")),
			Self::DecreaseAudioSeekAmount => Some(KeyChord::new(false, false, true, ";")),
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
			Self::PlayPauseAudio => Some(KeyChord::new(true, false, false, "Space")),
			Self::SeekAudioForward => Some(KeyChord::new(false, false, false, "'")),
			Self::SeekAudioBackward => Some(KeyChord::new(false, false, false, ";")),
			Self::IncreaseAudioSeekAmount => Some(KeyChord::new(false, false, true, "'")),
			Self::DecreaseAudioSeekAmount => Some(KeyChord::new(false, false, true, ";")),
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
