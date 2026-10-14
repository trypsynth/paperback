//! What a scratch pad can and cannot do.
//!
//! A scratch pad is a tab the reader types into rather than a document parsed from a file, so most
//! of the app has nothing to act on: there are no pages to go to, no headings to step through, no
//! audio, and no file to reload, export or reveal. This module is the one place that policy is
//! written down.
//!
//! It is deliberately the *only* place for the two mechanisms that have no other way to agree: the
//! menu bar, which greys out what a scratch pad cannot use (see `menu::state`), and the key
//! dispatch of the reading control and of macOS's native shortcut monitor, both of which turn a
//! greyed-out item into a passed-through key rather than a command. Splitting the list between those
//! two would let them drift, and the symptom of drift is a letter eaten instead of typed.
//!
//! The scratch pad's own right-click menu is the one thing not derived from this: it is a shorter
//! menu rather than a greyed-out one, so it names the five commands it offers directly. Every one of
//! them is in the available set above.

use paperback_core::config::ActionId;

/// Whether `action` means anything on a scratch pad.
///
/// The available set is written out rather than the unavailable one, so an action added to the app
/// later is unavailable on a scratch pad until someone decides otherwise. That is the safer
/// default: the cost of being wrong is a command missing from a scratch pad, which the new action's
/// author will notice, where the cost of the opposite default is a command that reaches a tab with
/// no file behind it and misbehaves in whatever way it was least designed for.
#[must_use]
pub const fn available_in_scratch(action: ActionId) -> bool {
	matches!(
		action,
		// File. Everything but the import, export and reveal entries, which all want a file.
		ActionId::NewScratchPad
			| ActionId::Open
			| ActionId::OpenFromUrl
			| ActionId::Close
			| ActionId::CloseAll
			| ActionId::ReopenLastClosed
			| ActionId::ShowAllRecentDocuments
			| ActionId::ClearRecentDocuments
			| ActionId::Exit
			// Edit. Find works against the typed text like it does against a document's, and the
			// selection mark is a position in the text either way.
			| ActionId::Find
			| ActionId::FindNext
			| ActionId::FindPrevious
			| ActionId::SetSelectionStart
			| ActionId::CopyFromSelectionStart
			| ActionId::JumpToSelectionStart
			// View. Only the two that are about the window rather than the document.
			| ActionId::ToggleWordWrap
			| ActionId::ToggleFullScreen
			// Go. Both of these count lines and characters, which a scratch pad has.
			| ActionId::GoToLine
			| ActionId::GoToPercent
			// Tools, minus Batch OCR, which is about PDF pages.
			| ActionId::WordCount
			| ActionId::DocumentInfo
			| ActionId::SleepTimer
			| ActionId::CustomizeShortcuts
			| ActionId::Options
			// Help is about the app, not the document.
			| ActionId::About
			| ActionId::ViewHelpBrowser
			| ActionId::ViewHelpPaperback
			| ActionId::CheckForUpdates
			| ActionId::Donate
	)
}

#[cfg(test)]
mod tests {
	use super::*;

	/// Pinned so that widening or narrowing what a scratch pad can do is a deliberate edit here
	/// rather than a side effect of adding a command somewhere else.
	#[test]
	fn only_the_commands_a_scratch_pad_can_act_on_are_available() {
		const EXPECTED: &[ActionId] = &[
			ActionId::NewScratchPad,
			ActionId::Open,
			ActionId::OpenFromUrl,
			ActionId::Close,
			ActionId::CloseAll,
			ActionId::ReopenLastClosed,
			ActionId::ShowAllRecentDocuments,
			ActionId::ClearRecentDocuments,
			ActionId::Exit,
			ActionId::Find,
			ActionId::FindNext,
			ActionId::FindPrevious,
			ActionId::SetSelectionStart,
			ActionId::CopyFromSelectionStart,
			ActionId::JumpToSelectionStart,
			ActionId::ToggleWordWrap,
			ActionId::ToggleFullScreen,
			ActionId::GoToLine,
			ActionId::GoToPercent,
			ActionId::WordCount,
			ActionId::DocumentInfo,
			ActionId::SleepTimer,
			ActionId::CustomizeShortcuts,
			ActionId::Options,
			ActionId::About,
			ActionId::ViewHelpBrowser,
			ActionId::ViewHelpPaperback,
			ActionId::CheckForUpdates,
			ActionId::Donate,
		];
		let actual: Vec<ActionId> = ActionId::all().iter().copied().filter(|&a| available_in_scratch(a)).collect();
		assert_eq!(actual.len(), EXPECTED.len(), "available set is {actual:?}");
		// Compared as sets: `ActionId::all` runs in enum order, which is not the order above.
		for action in EXPECTED {
			assert!(actual.contains(action), "{action:?} should be available on a scratch pad");
		}
	}

	/// The menus a scratch pad has no use for at all, rather than merely some entries of.
	#[test]
	fn no_navigation_bookmark_or_audio_command_is_available() {
		let unavailable = [
			ActionId::Reload,
			ActionId::GoToPage,
			ActionId::GoBack,
			ActionId::GoForward,
			ActionId::AnnouncePercent,
			ActionId::TableOfContents,
			ActionId::ElementsList,
			ActionId::NextHeading,
			ActionId::NextTable,
			ActionId::NextLink,
			ActionId::NextPage,
			ActionId::ContainerStart,
			ActionId::ToggleBookmark,
			ActionId::BookmarkWithNote,
			ActionId::NextBookmark,
			ActionId::NextNote,
			ActionId::ViewNoteText,
			ActionId::SetTemporaryBookmark,
			ActionId::JumpToTemporaryBookmark,
			ActionId::PlayPauseAudio,
			ActionId::SeekAudioForward,
			ActionId::IncreaseAudioSeekAmount,
			ActionId::IncreaseAudioSpeed,
			ActionId::BatchOcr,
			ActionId::OpenInWebView,
			ActionId::ViewSource,
			ActionId::RevealFileInFolder,
			ActionId::ImportDocumentData,
			ActionId::ExportDocumentData,
			ActionId::ExportToPlainText,
			ActionId::ExportToHtml,
			ActionId::ExportToMarkdown,
		];
		for action in unavailable {
			assert!(!available_in_scratch(action), "{action:?} should not be available on a scratch pad");
		}
	}
}
