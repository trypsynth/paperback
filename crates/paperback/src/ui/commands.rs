//! One table drives the menu bar, the dispatcher and the enable pass, so a command is described once; anything absent from [`COMMANDS`] falls through to `bind_menu_events` when [`dispatch`] returns false.

use std::{rc::Rc, sync::Mutex};

use paperback_core::config::{ActionId, ConfigManager};
use patois::t;
use wxdragon::prelude::*;

use super::{
	document_manager::DocumentManager,
	menu::{MenuEntry, format_menu_label, item_with_help},
	menu_ids,
	navigation::{MarkerNavTarget, handle_marker_navigation},
};

pub mod audio;
pub mod bookmarks;
pub mod file;
pub mod navigation;
pub mod selection;

/// Declared per command, so none can stay enabled with no document open by being left out of a list kept elsewhere.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Enable {
	Always,
	HasDocument,
	HasRecentlyClosed,
	HasRecentDocuments,
}

/// One borrow of the shared state for every handler, rather than a clone per closure.
pub struct Ctx<'a> {
	pub frame: &'a Frame,
	pub dm: &'a Rc<Mutex<DocumentManager>>,
	pub config: &'a Rc<Mutex<ConfigManager>>,
	pub live_region_label: StaticText,
	/// A shortcut and a menu click raise the same event, so `menu_events::bind_key_source` tags shortcuts; `false` only ever costs latency, never the message.
	pub from_keyboard: bool,
}

/// Marker navigation is data, so a new navigable element is one row rather than a handler of its own.
pub enum Behavior {
	Run(fn(&Ctx)),
	Navigate { target: MarkerNavTarget, next: bool },
}

pub struct Command {
	pub action: ActionId,
	/// A function so the text is translated at menu-build time, letting the menu bar follow a runtime language change.
	pub label: fn() -> String,
	pub help: Option<fn() -> String>,
	pub enable: Enable,
	pub behavior: Behavior,
}

impl Command {
	pub const fn id(&self) -> i32 {
		menu_ids::action_to_menu_id(self.action)
	}

	pub fn menu_label(&self, config: &ConfigManager) -> String {
		format_menu_label(&(self.label)(), self.action, config)
	}

	pub fn help_text(&self) -> String {
		self.help.map_or_else(String::new, |help| help())
	}

	fn run(&self, ctx: &Ctx) {
		match self.behavior {
			Behavior::Run(handler) => handler(ctx),
			Behavior::Navigate { target, next } => {
				handle_marker_navigation(ctx.dm, ctx.config, ctx.live_region_label, target, next, ctx.from_keyboard);
			}
		}
	}
}

pub static COMMANDS: &[Command] = &[
	Command {
		action: ActionId::Open,
		// TRANSLATORS: Menu item in the File menu to open a document.
		label: || t("&Open..."),
		// TRANSLATORS: Status-bar help text for the File > Open menu item.
		help: Some(|| t("Open a document")),
		enable: Enable::Always,
		behavior: Behavior::Run(file::open),
	},
	Command {
		action: ActionId::OpenFromUrl,
		// TRANSLATORS: Menu item in the File menu to open documents from web links.
		label: || t("Open from &URL..."),
		// TRANSLATORS: Status-bar help text for the File > Open from URL menu item.
		help: Some(|| t("Open documents from web links")),
		enable: Enable::Always,
		behavior: Behavior::Run(file::open_from_url),
	},
	Command {
		action: ActionId::Close,
		// TRANSLATORS: Menu item in the File menu to close the current document.
		label: || t("&Close"),
		// TRANSLATORS: Status-bar help text for the File > Close menu item.
		help: Some(|| t("Close the current document")),
		enable: Enable::HasDocument,
		behavior: Behavior::Run(file::close),
	},
	Command {
		action: ActionId::CloseAll,
		// TRANSLATORS: Menu item in the File menu to close all open documents.
		label: || t("Close &All"),
		// TRANSLATORS: Status-bar help text for the File > Close All menu item.
		help: Some(|| t("Close all documents")),
		enable: Enable::HasDocument,
		behavior: Behavior::Run(file::close_all),
	},
	Command {
		action: ActionId::ReopenLastClosed,
		// TRANSLATORS: Menu item in the File menu to reopen the most recently closed document.
		label: || t("Reopen &Last Closed"),
		// TRANSLATORS: Status-bar help text for the File > Reopen Last Closed menu item.
		help: Some(|| t("Reopen the last closed document")),
		enable: Enable::HasRecentlyClosed,
		behavior: Behavior::Run(file::reopen_last_closed),
	},
	Command {
		action: ActionId::Reload,
		// TRANSLATORS: Menu item in the View menu to re-read the current document from disk.
		label: || t("&Reload"),
		// TRANSLATORS: Status-bar help text for the View > Reload menu item.
		help: Some(|| t("Re-read the current document from disk")),
		enable: Enable::HasDocument,
		behavior: Behavior::Run(file::reload),
	},
	Command {
		action: ActionId::ClearRecentDocuments,
		// TRANSLATORS: Menu item in the File > Recent Documents submenu to empty the recent documents list.
		label: || t("Clea&r Recent Documents"),
		// TRANSLATORS: Status-bar help text for the File > Recent Documents > Clear Recent Documents menu item.
		help: Some(|| t("Remove all documents from the Recent Documents list")),
		enable: Enable::HasRecentDocuments,
		behavior: Behavior::Run(file::clear_recent_documents),
	},
	Command {
		action: ActionId::Exit,
		// TRANSLATORS: Menu item in the File menu to exit the application, shown only on Windows and Linux since macOS provides its own Quit menu item.
		label: || t("E&xit"),
		// TRANSLATORS: Status-bar help text for the File > Exit menu item.
		help: Some(|| t("Exit the application")),
		enable: Enable::Always,
		behavior: Behavior::Run(file::exit),
	},
	Command {
		action: ActionId::PreviousSection,
		// TRANSLATORS: Menu item in the Go menu to move to the previous section of the document.
		label: || t("Previous Section"),
		// TRANSLATORS: Status-bar help text for the Go > Previous Section menu item.
		help: Some(|| t("Go to previous section")),
		enable: Enable::HasDocument,
		behavior: Behavior::Navigate { target: MarkerNavTarget::Section, next: false },
	},
	Command {
		action: ActionId::NextSection,
		// TRANSLATORS: Menu item in the Go menu to move to the next section of the document.
		label: || t("Next Section"),
		// TRANSLATORS: Status-bar help text for the Go > Next Section menu item.
		help: Some(|| t("Go to next section")),
		enable: Enable::HasDocument,
		behavior: Behavior::Navigate { target: MarkerNavTarget::Section, next: true },
	},
	Command {
		action: ActionId::PreviousHeading,
		// TRANSLATORS: Menu item in the Go menu to move to the previous heading of any level in the document.
		label: || t("&Previous Heading"),
		// TRANSLATORS: Status-bar help text for the Go > Previous Heading menu item.
		help: Some(|| t("Go to previous heading")),
		enable: Enable::HasDocument,
		behavior: Behavior::Navigate { target: MarkerNavTarget::Heading(0), next: false },
	},
	Command {
		action: ActionId::NextHeading,
		// TRANSLATORS: Menu item in the Go menu to move to the next heading of any level in the document.
		label: || t("&Next Heading"),
		// TRANSLATORS: Status-bar help text for the Go > Next Heading menu item.
		help: Some(|| t("Go to next heading")),
		enable: Enable::HasDocument,
		behavior: Behavior::Navigate { target: MarkerNavTarget::Heading(0), next: true },
	},
	Command {
		action: ActionId::PreviousHeading1,
		// TRANSLATORS: Menu item in the Go menu to move to the previous level-1 heading in the document.
		label: || t("Previous Heading Level &1"),
		help: None,
		enable: Enable::HasDocument,
		behavior: Behavior::Navigate { target: MarkerNavTarget::Heading(1), next: false },
	},
	Command {
		action: ActionId::NextHeading1,
		// TRANSLATORS: Menu item in the Go menu to move to the next level-1 heading in the document.
		label: || t("Next Heading Level 1"),
		help: None,
		enable: Enable::HasDocument,
		behavior: Behavior::Navigate { target: MarkerNavTarget::Heading(1), next: true },
	},
	Command {
		action: ActionId::PreviousHeading2,
		// TRANSLATORS: Menu item in the Go menu to move to the previous level-2 heading in the document.
		label: || t("Previous Heading Level &2"),
		help: None,
		enable: Enable::HasDocument,
		behavior: Behavior::Navigate { target: MarkerNavTarget::Heading(2), next: false },
	},
	Command {
		action: ActionId::NextHeading2,
		// TRANSLATORS: Menu item in the Go menu to move to the next level-2 heading in the document.
		label: || t("Next Heading Level 2"),
		help: None,
		enable: Enable::HasDocument,
		behavior: Behavior::Navigate { target: MarkerNavTarget::Heading(2), next: true },
	},
	Command {
		action: ActionId::PreviousHeading3,
		// TRANSLATORS: Menu item in the Go menu to move to the previous level-3 heading in the document.
		label: || t("Previous Heading Level &3"),
		help: None,
		enable: Enable::HasDocument,
		behavior: Behavior::Navigate { target: MarkerNavTarget::Heading(3), next: false },
	},
	Command {
		action: ActionId::NextHeading3,
		// TRANSLATORS: Menu item in the Go menu to move to the next level-3 heading in the document.
		label: || t("Next Heading Level 3"),
		help: None,
		enable: Enable::HasDocument,
		behavior: Behavior::Navigate { target: MarkerNavTarget::Heading(3), next: true },
	},
	Command {
		action: ActionId::PreviousHeading4,
		// TRANSLATORS: Menu item in the Go menu to move to the previous level-4 heading in the document.
		label: || t("Previous Heading Level &4"),
		help: None,
		enable: Enable::HasDocument,
		behavior: Behavior::Navigate { target: MarkerNavTarget::Heading(4), next: false },
	},
	Command {
		action: ActionId::NextHeading4,
		// TRANSLATORS: Menu item in the Go menu to move to the next level-4 heading in the document.
		label: || t("Next Heading Level 4"),
		help: None,
		enable: Enable::HasDocument,
		behavior: Behavior::Navigate { target: MarkerNavTarget::Heading(4), next: true },
	},
	Command {
		action: ActionId::PreviousHeading5,
		// TRANSLATORS: Menu item in the Go menu to move to the previous level-5 heading in the document.
		label: || t("Previous Heading Level &5"),
		help: None,
		enable: Enable::HasDocument,
		behavior: Behavior::Navigate { target: MarkerNavTarget::Heading(5), next: false },
	},
	Command {
		action: ActionId::NextHeading5,
		// TRANSLATORS: Menu item in the Go menu to move to the next level-5 heading in the document.
		label: || t("Next Heading Level 5"),
		help: None,
		enable: Enable::HasDocument,
		behavior: Behavior::Navigate { target: MarkerNavTarget::Heading(5), next: true },
	},
	Command {
		action: ActionId::PreviousHeading6,
		// TRANSLATORS: Menu item in the Go menu to move to the previous level-6 heading in the document.
		label: || t("Previous Heading Level &6"),
		help: None,
		enable: Enable::HasDocument,
		behavior: Behavior::Navigate { target: MarkerNavTarget::Heading(6), next: false },
	},
	Command {
		action: ActionId::NextHeading6,
		// TRANSLATORS: Menu item in the Go menu to move to the next level-6 heading in the document.
		label: || t("Next Heading Level 6"),
		help: None,
		enable: Enable::HasDocument,
		behavior: Behavior::Navigate { target: MarkerNavTarget::Heading(6), next: true },
	},
	Command {
		action: ActionId::PreviousPage,
		// TRANSLATORS: Menu item in the Go menu to move to the previous page.
		label: || t("Previous Pa&ge"),
		help: None,
		enable: Enable::HasDocument,
		behavior: Behavior::Navigate { target: MarkerNavTarget::Page, next: false },
	},
	Command {
		action: ActionId::NextPage,
		// TRANSLATORS: Menu item in the Go menu to move to the next page.
		label: || t("Next Pag&e"),
		help: None,
		enable: Enable::HasDocument,
		behavior: Behavior::Navigate { target: MarkerNavTarget::Page, next: true },
	},
	Command {
		action: ActionId::PreviousLink,
		// TRANSLATORS: Menu item in the Go menu to move to the previous link in the document.
		label: || t("Previous Lin&k"),
		help: None,
		enable: Enable::HasDocument,
		behavior: Behavior::Navigate { target: MarkerNavTarget::Link, next: false },
	},
	Command {
		action: ActionId::NextLink,
		// TRANSLATORS: Menu item in the Go menu to move to the next link in the document.
		label: || t("Next Lin&k"),
		help: None,
		enable: Enable::HasDocument,
		behavior: Behavior::Navigate { target: MarkerNavTarget::Link, next: true },
	},
	Command {
		action: ActionId::PreviousImage,
		// TRANSLATORS: Menu item in the Go menu to move to the previous image in the document.
		label: || t("Previous Ima&ge"),
		help: None,
		enable: Enable::HasDocument,
		behavior: Behavior::Navigate { target: MarkerNavTarget::Image, next: false },
	},
	Command {
		action: ActionId::NextImage,
		// TRANSLATORS: Menu item in the Go menu to move to the next image in the document.
		label: || t("Next Ima&ge"),
		help: None,
		enable: Enable::HasDocument,
		behavior: Behavior::Navigate { target: MarkerNavTarget::Image, next: true },
	},
	Command {
		action: ActionId::PreviousFigure,
		// TRANSLATORS: Menu item in the Go menu to move to the previous figure in the document.
		label: || t("Previous Figu&re"),
		help: None,
		enable: Enable::HasDocument,
		behavior: Behavior::Navigate { target: MarkerNavTarget::Figure, next: false },
	},
	Command {
		action: ActionId::NextFigure,
		// TRANSLATORS: Menu item in the Go menu to move to the next figure in the document.
		label: || t("Next Figu&re"),
		help: None,
		enable: Enable::HasDocument,
		behavior: Behavior::Navigate { target: MarkerNavTarget::Figure, next: true },
	},
	Command {
		action: ActionId::PreviousTable,
		// TRANSLATORS: Menu item in the Go menu to move to the previous table in the document.
		label: || t("Previous &Table"),
		help: None,
		enable: Enable::HasDocument,
		behavior: Behavior::Navigate { target: MarkerNavTarget::Table, next: false },
	},
	Command {
		action: ActionId::NextTable,
		// TRANSLATORS: Menu item in the Go menu to move to the next table in the document.
		label: || t("Next &Table"),
		help: None,
		enable: Enable::HasDocument,
		behavior: Behavior::Navigate { target: MarkerNavTarget::Table, next: true },
	},
	Command {
		action: ActionId::PreviousFormula,
		// TRANSLATORS: Menu item in the Go menu to move to the previous formula.
		label: || t("Previous For&mula"),
		help: None,
		enable: Enable::HasDocument,
		behavior: Behavior::Navigate { target: MarkerNavTarget::Formula, next: false },
	},
	Command {
		action: ActionId::NextFormula,
		// TRANSLATORS: Menu item in the Go menu to move to the next formula.
		label: || t("Next For&mula"),
		help: None,
		enable: Enable::HasDocument,
		behavior: Behavior::Navigate { target: MarkerNavTarget::Formula, next: true },
	},
	Command {
		action: ActionId::PreviousSeparator,
		// TRANSLATORS: Menu item in the Go menu to move to the previous separator (e.g. a horizontal rule) in the document.
		label: || t("Previous Se&parator"),
		help: None,
		enable: Enable::HasDocument,
		behavior: Behavior::Navigate { target: MarkerNavTarget::Separator, next: false },
	},
	Command {
		action: ActionId::NextSeparator,
		// TRANSLATORS: Menu item in the Go menu to move to the next separator (e.g. a horizontal rule) in the document.
		label: || t("Next Se&parator"),
		help: None,
		enable: Enable::HasDocument,
		behavior: Behavior::Navigate { target: MarkerNavTarget::Separator, next: true },
	},
	Command {
		action: ActionId::PreviousList,
		// TRANSLATORS: Menu item in the Go menu to move to the previous list in the document.
		label: || t("Previous L&ist"),
		help: None,
		enable: Enable::HasDocument,
		behavior: Behavior::Navigate { target: MarkerNavTarget::List, next: false },
	},
	Command {
		action: ActionId::NextList,
		// TRANSLATORS: Menu item in the Go menu to move to the next list in the document.
		label: || t("Next L&ist"),
		help: None,
		enable: Enable::HasDocument,
		behavior: Behavior::Navigate { target: MarkerNavTarget::List, next: true },
	},
	Command {
		action: ActionId::PreviousListItem,
		// TRANSLATORS: Menu item in the Go menu to move to the previous item within the current list.
		label: || t("Previous List &Item"),
		help: None,
		enable: Enable::HasDocument,
		behavior: Behavior::Navigate { target: MarkerNavTarget::ListItem, next: false },
	},
	Command {
		action: ActionId::NextListItem,
		// TRANSLATORS: Menu item in the Go menu to move to the next item within the current list.
		label: || t("Next List I&tem"),
		help: None,
		enable: Enable::HasDocument,
		behavior: Behavior::Navigate { target: MarkerNavTarget::ListItem, next: true },
	},
	Command {
		action: ActionId::GoBack,
		// TRANSLATORS: Menu item in the Go menu to move back to the previous position in navigation history.
		label: || t("Go &Back"),
		help: None,
		enable: Enable::HasDocument,
		behavior: Behavior::Run(navigation::go_back),
	},
	Command {
		action: ActionId::GoForward,
		// TRANSLATORS: Menu item in the Go menu to move forward to the next position in navigation history.
		label: || t("Go &Forward"),
		help: None,
		enable: Enable::HasDocument,
		behavior: Behavior::Run(navigation::go_forward),
	},
	Command {
		action: ActionId::ContainerStart,
		// TRANSLATORS: Menu item in the Go menu to move to the start of the current list or table.
		label: || t("Container &Start"),
		// TRANSLATORS: Status-bar help text for the Go > Container Start menu item.
		help: Some(|| t("Go to the start of the current list or table")),
		enable: Enable::HasDocument,
		behavior: Behavior::Run(navigation::container_start),
	},
	Command {
		action: ActionId::ContainerEnd,
		// TRANSLATORS: Menu item in the Go menu to move past the end of the current list or table.
		label: || t("Past Container &End"),
		// TRANSLATORS: Status-bar help text for the Go > Past Container End menu item.
		help: Some(|| t("Go past the end of the current list or table")),
		enable: Enable::HasDocument,
		behavior: Behavior::Run(navigation::container_end),
	},
	Command {
		action: ActionId::PreviousBookmark,
		// TRANSLATORS: Menu item in the Bookmarks menu to move to the previous bookmark in the document.
		label: || t("&Previous Bookmark"),
		// TRANSLATORS: Status-bar help text for the Bookmarks > Previous Bookmark menu item.
		help: Some(|| t("Go to previous bookmark")),
		enable: Enable::HasDocument,
		behavior: Behavior::Run(bookmarks::previous_bookmark),
	},
	Command {
		action: ActionId::NextBookmark,
		// TRANSLATORS: Menu item in the Bookmarks menu to move to the next bookmark in the document.
		label: || t("&Next Bookmark"),
		// TRANSLATORS: Status-bar help text for the Bookmarks > Next Bookmark menu item.
		help: Some(|| t("Go to next bookmark")),
		enable: Enable::HasDocument,
		behavior: Behavior::Run(bookmarks::next_bookmark),
	},
	Command {
		action: ActionId::PreviousNote,
		// TRANSLATORS: Menu item in the Bookmarks menu to move to the previous note in the document.
		label: || t("Previous Not&e"),
		// TRANSLATORS: Status-bar help text for the Bookmarks > Previous Note menu item.
		help: Some(|| t("Go to previous note")),
		enable: Enable::HasDocument,
		behavior: Behavior::Run(bookmarks::previous_note),
	},
	Command {
		action: ActionId::NextNote,
		// TRANSLATORS: Menu item in the Bookmarks menu to move to the next note in the document.
		label: || t("Next N&ote"),
		// TRANSLATORS: Status-bar help text for the Bookmarks > Next Note menu item.
		help: Some(|| t("Go to next note")),
		enable: Enable::HasDocument,
		behavior: Behavior::Run(bookmarks::next_note),
	},
	Command {
		action: ActionId::JumpToAllBookmarks,
		// TRANSLATORS: Menu item in the Bookmarks menu to open a dialog listing all bookmarks and notes.
		label: || t("Jump to &All..."),
		// TRANSLATORS: Status-bar help text for the Bookmarks > Jump to All menu item.
		help: Some(|| t("Show all bookmarks and notes")),
		enable: Enable::HasDocument,
		behavior: Behavior::Run(bookmarks::jump_to_all),
	},
	Command {
		action: ActionId::JumpToBookmarksOnly,
		// TRANSLATORS: Menu item in the Bookmarks menu to open a dialog listing only bookmarks.
		label: || t("Jump to &Bookmarks Only..."),
		// TRANSLATORS: Status-bar help text for the Bookmarks > Jump to Bookmarks Only menu item.
		help: Some(|| t("Show bookmarks only")),
		enable: Enable::HasDocument,
		behavior: Behavior::Run(bookmarks::jump_to_bookmarks_only),
	},
	Command {
		action: ActionId::JumpToNotesOnly,
		// TRANSLATORS: Menu item in the Bookmarks menu to open a dialog listing only notes.
		label: || t("Jump to Notes Onl&y..."),
		// TRANSLATORS: Status-bar help text for the Bookmarks > Jump to Notes Only menu item.
		help: Some(|| t("Show notes only")),
		enable: Enable::HasDocument,
		behavior: Behavior::Run(bookmarks::jump_to_notes_only),
	},
	Command {
		action: ActionId::ViewNoteText,
		// TRANSLATORS: Menu item in the Bookmarks menu to view the text of the note at the current reading position.
		label: || t("&View Note Text..."),
		// TRANSLATORS: Status-bar help text for the Bookmarks > View Note Text menu item.
		help: Some(|| t("View the note at current position")),
		enable: Enable::HasDocument,
		behavior: Behavior::Run(bookmarks::view_note_text),
	},
	Command {
		action: ActionId::ToggleBookmark,
		// TRANSLATORS: Menu item in the Bookmarks menu to add or remove a bookmark at the current reading position.
		label: || t("&Toggle Bookmark"),
		help: None,
		enable: Enable::HasDocument,
		behavior: Behavior::Run(bookmarks::toggle),
	},
	Command {
		action: ActionId::BookmarkWithNote,
		// TRANSLATORS: Menu item in the Bookmarks menu to add a bookmark with an attached note at the current reading position.
		label: || t("Bookmark &with Note..."),
		help: None,
		enable: Enable::HasDocument,
		behavior: Behavior::Run(bookmarks::with_note),
	},
	Command {
		action: ActionId::SetTemporaryBookmark,
		// TRANSLATORS: Menu item in the Bookmarks menu to remember the current reading position as the temporary bookmark.
		label: || t("&Set Temporary Bookmark"),
		help: None,
		enable: Enable::HasDocument,
		behavior: Behavior::Run(bookmarks::set_temporary),
	},
	Command {
		action: ActionId::JumpToTemporaryBookmark,
		// TRANSLATORS: Menu item in the Bookmarks menu to go back to the position remembered as the temporary bookmark.
		label: || t("&Jump to Temporary Bookmark"),
		help: None,
		enable: Enable::HasDocument,
		behavior: Behavior::Run(bookmarks::jump_to_temporary),
	},
	Command {
		action: ActionId::SetSelectionStart,
		// TRANSLATORS: Menu item in the Edit menu to mark the current position as the beginning of a selection to copy from later.
		label: || t("Set &Selection Start"),
		// TRANSLATORS: Status-bar help text for the Edit > Set Selection Start menu item.
		help: Some(|| t("Mark the beginning of a selection to copy from")),
		enable: Enable::HasDocument,
		behavior: Behavior::Run(selection::set_start),
	},
	Command {
		action: ActionId::CopyFromSelectionStart,
		// TRANSLATORS: Menu item in the Edit menu to copy everything from the marked beginning of a selection to the current position.
		label: || t("Copy fro&m Selection Start"),
		// TRANSLATORS: Status-bar help text for the Edit > Copy from Selection Start menu item.
		help: Some(|| t("Copy from the beginning of the selection to here")),
		// Not gated on a mark, so pressing it with nothing marked can say so instead of being a silent dead item.
		enable: Enable::HasDocument,
		behavior: Behavior::Run(selection::copy_from_start),
	},
	Command {
		action: ActionId::JumpToSelectionStart,
		// TRANSLATORS: Menu item in the Edit menu to go back to the marked beginning of the selection.
		label: || t("&Jump to Selection Start"),
		// TRANSLATORS: Status-bar help text for the Edit > Jump to Selection Start menu item.
		help: Some(|| t("Go back to the beginning of the selection")),
		// Not gated on a mark, for the same reason as the copy above.
		enable: Enable::HasDocument,
		behavior: Behavior::Run(selection::jump_to_start),
	},
	Command {
		action: ActionId::PlayPauseAudio,
		// TRANSLATORS: Menu item in the Audio menu to play or pause the document's audio narration.
		label: || t("&Play/Pause Audio"),
		// TRANSLATORS: Status-bar help text for the Play/Pause Audio menu item.
		help: Some(|| t("Play or pause this document's audio narration")),
		enable: Enable::HasDocument,
		behavior: Behavior::Run(audio::toggle_play_pause),
	},
	Command {
		action: ActionId::SeekAudioForward,
		// TRANSLATORS: Menu item in the Audio menu to skip the audio narration forward.
		label: || t("Seek Audio &Forward"),
		// TRANSLATORS: Status-bar help text for the Seek Audio Forward menu item.
		help: Some(|| t("Skip the audio narration forward")),
		enable: Enable::HasDocument,
		behavior: Behavior::Run(audio::seek_forward),
	},
	Command {
		action: ActionId::SeekAudioBackward,
		// TRANSLATORS: Menu item in the Audio menu to skip the audio narration backward.
		label: || t("Seek Audio &Backward"),
		// TRANSLATORS: Status-bar help text for the Seek Audio Backward menu item.
		help: Some(|| t("Skip the audio narration backward")),
		enable: Enable::HasDocument,
		behavior: Behavior::Run(audio::seek_backward),
	},
	Command {
		action: ActionId::IncreaseAudioSeekAmount,
		// TRANSLATORS: Menu item in the Audio menu to increase the amount of time each audio seek skips.
		label: || t("Increase Audio Seek &Amount"),
		// TRANSLATORS: Status-bar help text for the Increase Audio Seek Amount menu item.
		help: Some(|| t("Increase how far seeking the audio narration moves")),
		enable: Enable::Always,
		behavior: Behavior::Run(audio::increase_seek_amount),
	},
	Command {
		action: ActionId::DecreaseAudioSeekAmount,
		// TRANSLATORS: Menu item in the Audio menu to decrease the amount of time each audio seek skips.
		label: || t("Decrease Audio Seek Am&ount"),
		// TRANSLATORS: Status-bar help text for the Decrease Audio Seek Amount menu item.
		help: Some(|| t("Decrease how far seeking the audio narration moves")),
		enable: Enable::Always,
		behavior: Behavior::Run(audio::decrease_seek_amount),
	},
	Command {
		action: ActionId::IncreaseAudioSpeed,
		// TRANSLATORS: Menu item in the Audio menu to increase the audio narration's playback speed.
		label: || t("&Increase Audio Speed"),
		// TRANSLATORS: Status-bar help text for the Increase Audio Speed menu item.
		help: Some(|| t("Increase how fast the audio narration plays")),
		enable: Enable::HasDocument,
		behavior: Behavior::Run(audio::increase_speed),
	},
	Command {
		action: ActionId::DecreaseAudioSpeed,
		// TRANSLATORS: Menu item in the Audio menu to decrease the audio narration's playback speed.
		label: || t("&Decrease Audio Speed"),
		// TRANSLATORS: Status-bar help text for the Decrease Audio Speed menu item.
		help: Some(|| t("Decrease how fast the audio narration plays")),
		enable: Enable::HasDocument,
		behavior: Behavior::Run(audio::decrease_speed),
	},
	Command {
		action: ActionId::IncreaseAudioVolume,
		// TRANSLATORS: Menu item in the Audio menu to make the audio narration louder.
		label: || t("Increase Audio &Volume"),
		// TRANSLATORS: Status-bar help text for the Increase Audio Volume menu item.
		help: Some(|| t("Make the audio narration louder")),
		enable: Enable::HasDocument,
		behavior: Behavior::Run(audio::increase_volume),
	},
	Command {
		action: ActionId::DecreaseAudioVolume,
		// TRANSLATORS: Menu item in the Audio menu to make the audio narration quieter.
		label: || t("Decrease Audio Vo&lume"),
		// TRANSLATORS: Status-bar help text for the Decrease Audio Volume menu item.
		help: Some(|| t("Make the audio narration quieter")),
		enable: Enable::HasDocument,
		behavior: Behavior::Run(audio::decrease_volume),
	},
];

pub fn for_action(action: ActionId) -> Option<&'static Command> {
	COMMANDS.iter().find(|command| command.action == action)
}

/// Panics rather than quietly building a menu with an item missing; the menu tests build every menu, so CI catches it.
pub fn menu_entry(action: ActionId, config: &ConfigManager) -> MenuEntry {
	let command =
		for_action(action).unwrap_or_else(|| panic!("{action:?} is in a menu's item list but not in COMMANDS"));
	item_with_help(command.id(), command.menu_label(config), command.help_text())
}

pub fn menu_entries(actions: &[ActionId], config: &ConfigManager) -> Vec<MenuEntry> {
	actions.iter().map(|&action| menu_entry(action, config)).collect()
}

pub fn find(id: i32) -> Option<&'static Command> {
	COMMANDS.iter().find(|command| command.id() == id)
}

/// `false` means the command isn't in the table yet and the caller falls through to its own dispatch.
pub fn dispatch(id: i32, ctx: &Ctx) -> bool {
	find(id).is_some_and(|command| {
		command.run(ctx);
		true
	})
}

/// One condition at a time because each call site knows only one: whether a document is open, or the reopen stack, which changes on its own schedule.
pub fn apply_enable(frame: &Frame, enable: Enable, available: bool) {
	let Some(menu_bar) = frame.get_menu_bar() else {
		return;
	};
	apply_enable_to(&menu_bar, enable, available);
}

pub fn apply_enable_to(menu_bar: &MenuBar, enable: Enable, available: bool) {
	for command in COMMANDS.iter().filter(|command| command.enable == enable) {
		menu_bar.enable_item(command.id(), available);
	}
}

#[cfg(test)]
mod tests {
	use rstest::rstest;

	use super::*;

	/// Two commands sharing a wx id would make [`find`] silently run whichever came first.
	#[test]
	fn every_command_has_a_distinct_id() {
		let mut seen: Vec<i32> = COMMANDS.iter().map(Command::id).collect();
		let before = seen.len();
		seen.sort_unstable();
		seen.dedup();
		assert_eq!(seen.len(), before, "two commands share a menu id");
	}

	/// The table is the dispatch path, so an id it does not answer for is a dead menu item.
	#[test]
	fn dispatch_finds_every_command() {
		for command in COMMANDS {
			assert!(find(command.id()).is_some(), "{:?} is not reachable by its own id", command.action);
		}
	}

	#[test]
	fn clear_recent_documents_needs_recent_documents() {
		let command = for_action(ActionId::ClearRecentDocuments).unwrap();
		assert_eq!(command.enable, Enable::HasRecentDocuments);
	}

	/// On for a help file or a source view too, where the handler does nothing: no other command's state changes as the reader switches tabs.
	#[test]
	fn reload_needs_a_document() {
		let command = for_action(ActionId::Reload).unwrap();
		assert_eq!(command.enable, Enable::HasDocument);
	}

	#[rstest]
	#[case::play_pause(ActionId::PlayPauseAudio)]
	#[case::seek_forward(ActionId::SeekAudioForward)]
	#[case::seek_backward(ActionId::SeekAudioBackward)]
	#[case::increase_speed(ActionId::IncreaseAudioSpeed)]
	#[case::decrease_speed(ActionId::DecreaseAudioSpeed)]
	#[case::increase_volume(ActionId::IncreaseAudioVolume)]
	#[case::decrease_volume(ActionId::DecreaseAudioVolume)]
	fn audio_playback_needs_a_document(#[case] action: ActionId) {
		let command = for_action(action).unwrap();
		assert_eq!(command.enable, Enable::HasDocument);
	}

	/// A fallback id would collide with anything else that failed to map and toggle the wrong menu item.
	#[test]
	fn no_command_maps_to_a_placeholder_id() {
		for command in COMMANDS {
			assert_ne!(command.id(), 0, "{:?} has no menu id", command.action);
			assert_ne!(command.id(), -1, "{:?} maps to ID_ANY", command.action);
		}
	}
}
