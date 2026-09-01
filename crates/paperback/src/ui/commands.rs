//! One record per user-triggerable command: its identity, its menu label and help text, when
//! it is available, and what it does.
//!
//! Commands used to be described in four places at once, split by which menu they happened to
//! sit under: `menu/<name>_menu.rs` built the item, `main_window/menu_<name>.rs` or an arm of
//! `bind_menu_events`' dispatch match ran it, `menu_ids` named its wx id, and `menu/state.rs`
//! listed it by hand if it needed a document open. Adding one meant four edits in three
//! directories, keyed to a position in the menu bar rather than to anything about the command.
//!
//! A [`Command`] holds all of it, [`ActionId`] is the identity (it already carries the display
//! name and default key chord for the shortcuts dialog), and the menu bar, the dispatcher and
//! the enable/disable pass are all derived from the same list. Handlers live in submodules by
//! feature, not by menu.
//!
//! Commands are ported over a few at a time. Anything not in [`COMMANDS`] yet is still handled
//! by `bind_menu_events`, which falls through to its own match when [`dispatch`] returns false.

use std::{rc::Rc, sync::Mutex};

use paperback_core::config::{ActionId, ConfigManager};
use patois::t;
use wxdragon::prelude::*;

use super::{document_manager::DocumentManager, menu::format_menu_label, menu_ids};

pub mod file;

/// What has to be true for a command to be usable.
///
/// Replaces the hand-maintained id list in `menu/state.rs`: a command declares its own
/// condition, so a new one cannot be forgotten there and then silently stay enabled with no
/// document open.
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
/// One borrow of the shared state rather than a clone per closure: the old dispatch match
/// opened with two dozen `Rc::clone`s and `_for_<handler>` rebindings, one set per arm that
/// needed them.
pub struct Ctx<'a> {
	pub frame: &'a Frame,
	pub dm: &'a Rc<Mutex<DocumentManager>>,
	pub config: &'a Rc<Mutex<ConfigManager>>,
	pub live_region_label: StaticText,
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
	/// Status-bar help text.
	pub help: fn() -> String,
	pub enable: Enable,
	pub run: fn(&Ctx),
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
}

/// Every command that has been ported to the table.
pub static COMMANDS: &[Command] = &[
	Command {
		action: ActionId::Open,
		// TRANSLATORS: Menu item in the File menu to open a document.
		label: || t("&Open..."),
		// TRANSLATORS: Status-bar help text for the File > Open menu item.
		help: || t("Open a document"),
		enable: Enable::Always,
		run: file::open,
	},
	Command {
		action: ActionId::Close,
		// TRANSLATORS: Menu item in the File menu to close the current document.
		label: || t("&Close"),
		// TRANSLATORS: Status-bar help text for the File > Close menu item.
		help: || t("Close the current document"),
		enable: Enable::HasDocument,
		run: file::close,
	},
	Command {
		action: ActionId::CloseAll,
		// TRANSLATORS: Menu item in the File menu to close all open documents.
		label: || t("Close &All"),
		// TRANSLATORS: Status-bar help text for the File > Close All menu item.
		help: || t("Close all documents"),
		enable: Enable::HasDocument,
		run: file::close_all,
	},
	Command {
		action: ActionId::ReopenLastClosed,
		// TRANSLATORS: Menu item in the File menu to reopen the most recently closed document.
		label: || t("Reopen &Last Closed"),
		// TRANSLATORS: Status-bar help text for the File > Reopen Last Closed menu item.
		help: || t("Reopen the last closed document"),
		enable: Enable::HasRecentlyClosed,
		run: file::reopen_last_closed,
	},
	Command {
		action: ActionId::Exit,
		// TRANSLATORS: Menu item in the File menu to exit the application, shown only on Windows and Linux since macOS provides its own Quit menu item.
		label: || t("E&xit"),
		// TRANSLATORS: Status-bar help text for the File > Exit menu item.
		help: || t("Exit the application"),
		enable: Enable::Always,
		run: file::exit,
	},
];

/// Looks up a ported command by its action.
pub fn for_action(action: ActionId) -> Option<&'static Command> {
	COMMANDS.iter().find(|command| command.action == action)
}

/// Appends `action`'s menu item, with its label, shortcut and help text, to `menu`.
///
/// Panics if `action` has not been ported to [`COMMANDS`], rather than quietly building a menu
/// with an item missing. Every caller's list is covered by a test, so this fires in CI rather
/// than on a user's launch.
pub fn append_item(menu: &Menu, action: ActionId, config: &ConfigManager) {
	let command =
		for_action(action).unwrap_or_else(|| panic!("{action:?} is in a menu's item list but not in COMMANDS"));
	let _ = menu.append(command.id(), &command.menu_label(config), &(command.help)(), ItemKind::Normal);
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
		(command.run)(ctx);
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
