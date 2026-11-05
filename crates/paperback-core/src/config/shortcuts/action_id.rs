use super::{KeyChord, ShortcutCategory};
use crate::t;

// Each action is one row, so a new one can't miss a match. The serialized names are what config files store, and `all()` keeps table order, which decides which action wins a shared chord.
macro_rules! actions {
	($($name:ident, $category:ident, $label:expr, $chord:expr $(, mac: $mac:expr)?;)*) => {
		#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
		#[serde(rename_all = "snake_case")]
		pub enum ActionId {
			$($name,)*
		}

		impl ActionId {
			pub const fn all() -> &'static [Self] {
				&[$(Self::$name,)*]
			}

			pub const fn category(self) -> ShortcutCategory {
				match self {
					$(Self::$name => ShortcutCategory::$category,)*
				}
			}

			pub fn display_name(self) -> String {
				match self {
					$(Self::$name => $label,)*
				}
			}

			pub(super) fn default_chord_on(self, mac: bool) -> Option<KeyChord> {
				match self {
					$(Self::$name => actions!(@chord mac, $chord $(, $mac)?),)*
				}
			}
		}
	};
	(@chord $on_mac:ident, $chord:expr) => {
		Option::<KeyChord>::from($chord)
	};
	(@chord $on_mac:ident, $chord:expr, $mac:expr) => {
		if $on_mac { Option::<KeyChord>::from($mac) } else { Option::<KeyChord>::from($chord) }
	};
}

impl ActionId {
	pub fn default_chord(self) -> Option<KeyChord> {
		self.default_chord_on(cfg!(target_os = "macos"))
	}
}

// `ctrl` is the command modifier, which KeyChord makes Cmd on macOS; `physical_ctrl` stays Control there.
fn ctrl(key: &str) -> KeyChord {
	KeyChord::new(true, false, false, key)
}

fn ctrl_shift(key: &str) -> KeyChord {
	KeyChord::new(true, false, true, key)
}

fn ctrl_alt(key: &str) -> KeyChord {
	KeyChord::new(true, true, false, key)
}

fn alt(key: &str) -> KeyChord {
	KeyChord::new(false, true, false, key)
}

fn alt_shift(key: &str) -> KeyChord {
	KeyChord::new(false, true, true, key)
}

fn shift(key: &str) -> KeyChord {
	KeyChord::new(false, false, true, key)
}

fn key(key: &str) -> KeyChord {
	KeyChord::new(false, false, false, key)
}

fn physical_ctrl(key: &str) -> KeyChord {
	KeyChord::new_raw_ctrl(true, false, false, key)
}

fn physical_ctrl_shift(key: &str) -> KeyChord {
	KeyChord::new_raw_ctrl(true, false, true, key)
}

fn ctrl_physical_ctrl(key: &str) -> KeyChord {
	KeyChord { raw_ctrl: true, ..KeyChord::new(true, false, false, key) }
}

actions! {
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	Open, File, t("Open..."), ctrl("O");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	OpenFromUrl, File, t("Open from URL..."), ctrl("L"), mac: ctrl_alt("O");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	Close, File, t("Close"), ctrl("F4"), mac: ctrl("W");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	CloseAll, File, t("Close All"), ctrl_shift("F4"), mac: ctrl_shift("W");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	ReopenLastClosed, File, t("Reopen Last Closed"), ctrl_shift("T");
	// Not Cmd+R, the browser's reload on macOS, which is Show All Recent Documents here.
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	Reload, View, t("Reload"), key("F5");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	ShowAllRecentDocuments, File, t("Show All Recent Documents..."), ctrl("R");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	ClearRecentDocuments, File, t("Clear Recent Documents"), None;
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	Exit, File, t("Exit"), ctrl("Q"), mac: None;
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	Find, Edit, t("Find..."), ctrl("F");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	FindNext, Edit, t("Find Next"), key("F3"), mac: ctrl("G");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	FindPrevious, Edit, t("Find Previous"), shift("F3"), mac: ctrl_shift("G");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	GoToLine, Go, t("Go to Line..."), ctrl("G"), mac: ctrl("L");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	GoToPercent, Go, t("Go to Percent..."), ctrl_shift("G"), mac: ctrl_shift("L");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	GoToPage, Go, t("Go to Page..."), ctrl("P");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	GoBack, Go, t("Go Back"), alt("Left"), mac: ctrl("[");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	GoForward, Go, t("Go Forward"), alt("Right"), mac: ctrl("]");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	AnnouncePercent, Go, t("Announce Percentage"), key("=");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	SetTemporaryBookmark, Bookmarks, t("Set Temporary Bookmark"), key("/");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	JumpToTemporaryBookmark, Bookmarks, t("Jump to Temporary Bookmark"), key("\\");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	PreviousSection, Go, t("Previous Section"), key("[");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	NextSection, Go, t("Next Section"), key("]");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	PreviousHeading, Go, t("Previous Heading"), shift("H");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	NextHeading, Go, t("Next Heading"), key("H");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	PreviousHeading1, Go, t("Previous Heading Level 1"), shift("1");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	NextHeading1, Go, t("Next Heading Level 1"), key("1");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	PreviousHeading2, Go, t("Previous Heading Level 2"), shift("2");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	NextHeading2, Go, t("Next Heading Level 2"), key("2");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	PreviousHeading3, Go, t("Previous Heading Level 3"), shift("3");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	NextHeading3, Go, t("Next Heading Level 3"), key("3");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	PreviousHeading4, Go, t("Previous Heading Level 4"), shift("4");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	NextHeading4, Go, t("Next Heading Level 4"), key("4");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	PreviousHeading5, Go, t("Previous Heading Level 5"), shift("5");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	NextHeading5, Go, t("Next Heading Level 5"), key("5");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	PreviousHeading6, Go, t("Previous Heading Level 6"), shift("6");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	NextHeading6, Go, t("Next Heading Level 6"), key("6");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	PreviousPage, Go, t("Previous Page"), shift("P");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	NextPage, Go, t("Next Page"), key("P");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	PreviousBookmark, Bookmarks, t("Previous Bookmark"), shift("B");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	NextBookmark, Bookmarks, t("Next Bookmark"), key("B");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	PreviousNote, Bookmarks, t("Previous Note"), shift("N");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	NextNote, Bookmarks, t("Next Note"), key("N");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	JumpToAllBookmarks, Bookmarks, t("Jump to All Bookmarks..."), ctrl("B");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	JumpToBookmarksOnly, Bookmarks, t("Jump to Bookmarks Only..."), ctrl_alt("B");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	JumpToNotesOnly, Bookmarks, t("Jump to Notes Only..."), ctrl_alt("M");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	ViewNoteText, Bookmarks, t("View Note Text"), ctrl_shift("W"), mac: physical_ctrl_shift("W");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	PreviousLink, Go, t("Previous Link"), shift("K");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	NextLink, Go, t("Next Link"), key("K");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	PreviousImage, Go, t("Previous Image"), shift("G");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	NextImage, Go, t("Next Image"), key("G");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	PreviousFigure, Go, t("Previous Figure"), shift("F");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	NextFigure, Go, t("Next Figure"), key("F");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	PreviousTable, Go, t("Previous Table"), shift("T");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	NextTable, Go, t("Next Table"), key("T");
	// TRANSLATORS: Shortcut action to move to the previous formula
	PreviousFormula, Go, t("Previous Formula"), shift("M");
	// TRANSLATORS: Shortcut action to move to the next formula
	NextFormula, Go, t("Next Formula"), key("M");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	PreviousSeparator, Go, t("Previous Separator"), shift("S");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	NextSeparator, Go, t("Next Separator"), key("S");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	PreviousList, Go, t("Previous List"), shift("L");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	NextList, Go, t("Next List"), key("L");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	PreviousListItem, Go, t("Previous List Item"), shift("I");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	NextListItem, Go, t("Next List Item"), key("I");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	ContainerStart, Go, t("Container Start"), shift(",");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	ContainerEnd, Go, t("Past Container End"), key(",");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	WordCount, Tools, t("Word Count"), ctrl("W"), mac: physical_ctrl("W");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	DocumentInfo, Tools, t("Document Info..."), ctrl("I");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	TableOfContents, Go, t("Table of Contents..."), ctrl("T");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	ElementsList, Go, t("Elements List..."), key("F7");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	RevealFileInFolder, File, t("Reveal File in Folder"), ctrl_shift("C");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	OpenInWebView, View, t("Open in Web View"), ctrl_shift("V");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	ViewSource, View, t("View Source"), ctrl("U");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	ToggleBookmark, Bookmarks, t("Toggle Bookmark"), ctrl_shift("B");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	BookmarkWithNote, Bookmarks, t("Bookmark with Note"), ctrl_shift("N");
	// Shift+F10 is the context menu, so this pair is on Alt. Option+F9/F10 is a media key on Apple keyboards and the system eats it before wx sees it, so macOS has them on Cmd.
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	SetSelectionStart, Edit, t("Set Selection Start"), alt("F9"), mac: ctrl("F9");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	CopyFromSelectionStart, Edit, t("Copy from Selection Start"), alt("F10"), mac: ctrl("F10");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	JumpToSelectionStart, Edit, t("Jump to Selection Start"), alt_shift("F9"), mac: ctrl_shift("F9");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	ToggleWordWrap, View, t("Toggle Word Wrap"), ctrl_alt("W");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	PlayPauseAudio, Audio, t("Play/Pause Audio"), ctrl("Space"), mac: physical_ctrl("Space");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	SeekAudioForward, Audio, t("Seek Audio Forward"), key("'");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	SeekAudioBackward, Audio, t("Seek Audio Backward"), key(";");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	IncreaseAudioSeekAmount, Audio, t("Increase Audio Seek Amount"), shift("'");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	DecreaseAudioSeekAmount, Audio, t("Decrease Audio Seek Amount"), shift(";");
	// Shift+, and Shift+. are already Container Start/End, so speed adds Ctrl.
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	IncreaseAudioSpeed, Audio, t("Increase Audio Speed"), ctrl_shift(".");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	DecreaseAudioSpeed, Audio, t("Decrease Audio Speed"), ctrl_shift(",");
	// Next to seek on ' and ;, which Shift turns into the seek amount.
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	IncreaseAudioVolume, Audio, t("Increase Audio Volume"), ctrl("'");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	DecreaseAudioVolume, Audio, t("Decrease Audio Volume"), ctrl(";");
	// The conventional Control+Command+F on macOS.
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	ToggleFullScreen, View, t("Full Screen"), key("F11"), mac: ctrl_physical_ctrl("F");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	Options, Tools, t("Settings..."), ctrl(",");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	SleepTimer, Tools, t("Sleep Timer..."), ctrl_shift("S");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	BatchOcr, Tools, t("Batch OCR..."), ctrl_shift("O");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	RecognizeCurrentPage, Tools, t("Recognize Current Page"), ctrl_shift("R");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	CustomizeShortcuts, Tools, t("Customize Keyboard Shortcuts..."), None;
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	ImportDocumentData, File, t("Import Document Data..."), ctrl_shift("I");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	ExportDocumentData, File, t("Export Document Data..."), ctrl_shift("E");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	ExportNotesAndBookmarks, File, t("Export Notes and Bookmarks as Text or HTML..."), ctrl_alt("E");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	ExportToPlainText, File, t("Export to Plain Text..."), ctrl("E");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	ExportToHtml, File, t("Export to HTML..."), None;
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	ExportToMarkdown, File, t("Export to Markdown..."), None;
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	About, Help, t("About Paperback"), ctrl("F1");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	ViewHelpBrowser, Help, t("View Help in Browser"), key("F1");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	ViewHelpPaperback, Help, t("View Help in Paperback"), shift("F1");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	CheckForUpdates, Help, t("Check for Updates"), ctrl_shift("U");
	// TRANSLATORS: Name of this keyboard-shortcut action, shown in the Customize Keyboard Shortcuts dialog (its list of assignable actions, and the "Set Shortcut for {}" / conflict-reassignment prompts).
	Donate, Help, t("Donate"), ctrl("D");
}
