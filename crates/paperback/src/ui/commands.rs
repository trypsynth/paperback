//! One record per user-triggerable command: its identity, its menu label and help text, when
//! it is available, and what it does.
//!
//! A [`Command`] holds all of it, so the menu bar, the dispatcher and the enable/disable pass
//! can be derived from one list and a command described once. [`ActionId`] is the identity,
//! shared with the shortcuts dialog and the keyboard config, which already carry its display
//! name and default key chord. Handlers live in submodules by feature.
//!
//! Not every command is here. Anything absent from [`COMMANDS`] is handled by
//! `bind_menu_events`, which falls through to its own match when [`dispatch`] returns false.

use std::{rc::Rc, sync::Mutex};

use paperback_core::config::{ActionId, ConfigManager};
use patois::t;
use wxdragon::prelude::*;

use super::{
	document_manager::DocumentManager,
	menu::{MenuEntry, format_menu_label, item_with_help},
	menu_ids,
	navigation::{self, MarkerNavTarget},
};

pub mod file;

/// What has to be true for a command to be usable.
///
/// A command declares its own condition, so it cannot end up enabled with no document open
/// by being left out of a list kept somewhere else.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Enable {
	/// Always available, document or not.
	Always,
	/// Needs a document open.
	HasDocument,
	/// Needs something in the recently-closed stack.
	HasRecentlyClosed,
}

/// What a handler is allowed to touch.
///
/// One borrow of the shared state for every handler, rather than a clone per closure.
pub struct Ctx<'a> {
	pub frame: &'a Frame,
	pub dm: &'a Rc<Mutex<DocumentManager>>,
	pub config: &'a Rc<Mutex<ConfigManager>>,
	pub live_region_label: StaticText,
}

/// What running a command does.
///
/// Marker navigation is data rather than a function: every one of those commands is the same
/// `handle_marker_navigation` call with a different target and direction, so holding the
/// difference as values makes a new navigable element one row rather than a handler of its own.
pub enum Behavior {
	/// Run this handler.
	Run(fn(&Ctx)),
	/// Move to the previous or next `target`.
	Navigate { target: MarkerNavTarget, next: bool },
}

/// A single command: identity, presentation, availability, behaviour.
pub struct Command {
	/// Identity, shared with the shortcuts dialog and the keyboard config.
	pub action: ActionId,
	/// Menu label, without the accelerator; [`Command::menu_label`] appends that.
	///
	/// A function rather than a string so the text is translated at menu-build time, which is
	/// what makes the menu bar follow a language change at runtime.
	pub label: fn() -> String,
	/// Status-bar help text, for the items that have one. Most of the Go menu's do not.
	pub help: Option<fn() -> String>,
	pub enable: Enable,
	pub behavior: Behavior,
}

impl Command {
	/// The wx id this command's menu item carries.
	pub const fn id(&self) -> i32 {
		menu_ids::action_to_menu_id(self.action)
	}

	/// Label with the user's current shortcut appended, ready for a menu item.
	pub fn menu_label(&self, config: &ConfigManager) -> String {
		format_menu_label(&(self.label)(), self.action, config)
	}

	/// Status-bar help text, empty when the item has none.
	pub fn help_text(&self) -> String {
		self.help.map_or_else(String::new, |help| help())
	}

	fn run(&self, ctx: &Ctx) {
		match self.behavior {
			Behavior::Run(handler) => handler(ctx),
			Behavior::Navigate { target, next } => {
				navigation::handle_marker_navigation(ctx.dm, ctx.config, ctx.live_region_label, target, next);
			}
		}
	}
}

/// Every command that has been ported to the table.
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
];

/// Looks up a ported command by its action.
pub fn for_action(action: ActionId) -> Option<&'static Command> {
	COMMANDS.iter().find(|command| command.action == action)
}

/// One menu entry for `action`, for the group builders in `menu/go_menu.rs`.
///
/// Panics for the same reason [`append_item`] does, and is covered by the same kind of test.
pub fn menu_entry(action: ActionId, config: &ConfigManager) -> MenuEntry {
	let command =
		for_action(action).unwrap_or_else(|| panic!("{action:?} is in a menu's item list but not in COMMANDS"));
	item_with_help(command.id(), command.menu_label(config), command.help_text())
}

/// Menu entries for `actions`, in the order given.
pub fn menu_entries(actions: &[ActionId], config: &ConfigManager) -> Vec<MenuEntry> {
	actions.iter().map(|&action| menu_entry(action, config)).collect()
}

/// Appends `action`'s menu item, with its label, shortcut and help text, to `menu`.
///
/// Panics if `action` has not been ported to [`COMMANDS`], rather than quietly building a menu
/// with an item missing. Every caller's list is covered by a test, so this fires in CI rather
/// than on a user's launch.
pub fn append_item(menu: &Menu, action: ActionId, config: &ConfigManager) {
	let command =
		for_action(action).unwrap_or_else(|| panic!("{action:?} is in a menu's item list but not in COMMANDS"));
	let _ = menu.append(command.id(), &command.menu_label(config), &command.help_text(), ItemKind::Normal);
}

/// Looks up a ported command by wx id.
pub fn find(id: i32) -> Option<&'static Command> {
	COMMANDS.iter().find(|command| command.id() == id)
}

/// Runs the command bound to `id`, reporting whether one was found.
///
/// `false` means the id belongs to a command that has not been ported yet, and the caller
/// should fall through to its own dispatch.
pub fn dispatch(id: i32, ctx: &Ctx) -> bool {
	find(id).is_some_and(|command| {
		command.run(ctx);
		true
	})
}

/// Enables or disables every ported command whose availability is governed by `enable`.
///
/// Split by condition rather than taking all of them at once because the call sites are:
/// opening or closing a document knows whether one is open, and the reopen stack changes
/// on its own schedule.
pub fn apply_enable(frame: &Frame, enable: Enable, available: bool) {
	let Some(menu_bar) = frame.get_menu_bar() else {
		return;
	};
	for command in COMMANDS.iter().filter(|command| command.enable == enable) {
		menu_bar.enable_item(command.id(), available);
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	/// Two commands sharing a wx id would make [`find`] pick whichever came first, silently
	/// running the wrong one.
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

	/// A command whose id came back as the fallback would collide with anything else that
	/// failed to map, and would enable and disable the wrong menu item.
	#[test]
	fn no_command_maps_to_a_placeholder_id() {
		for command in COMMANDS {
			assert_ne!(command.id(), 0, "{:?} has no menu id", command.action);
			assert_ne!(command.id(), -1, "{:?} maps to ID_ANY", command.action);
		}
	}
}
