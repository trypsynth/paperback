use paperback_core::config::ConfigManager;
use wxdragon::prelude::*;

use crate::ui::{
	commands::{self, Enable},
	document_manager::{DocumentManager, TabKind},
	menu_ids, scratch_pad,
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
	// These two export the document that is open like the others do, and were simply missed.
	menu_ids::EXPORT_TO_HTML,
	menu_ids::EXPORT_TO_MARKDOWN,
];

/// Edit > Copy and Select All, which macOS leaves to its native Edit menu.
#[cfg(not(target_os = "macos"))]
const DOCUMENT_DEPENDENT_EDIT_IDS: &[i32] = &[menu_ids::COPY, menu_ids::SELECT_ALL];

/// Commands that stay enabled with no document open but still do nothing on a scratch pad, where
/// there is no file and no page to apply them to. Every other item that a scratch pad cannot use
/// is already document-dependent, or reaches this file through the command table below.
const SCRATCH_DISABLED_IDS: &[i32] = &[menu_ids::BATCH_OCR];

/// Brings the menu bar in line with what is open: which items are enabled, and which of them still
/// carry a shortcut.
///
/// `active` is the kind of the tab in front, or `None` when there is none - the menu depends on
/// which tab is showing now, not only on whether any is, so this has to be called on a tab switch
/// as well as on an open or a close. A fresh bar comes back with everything enabled and every
/// shortcut in place, so this has to follow every one of those too.
pub fn update_menu_item_states(frame: &Frame, config: &ConfigManager, active: Option<TabKind>) {
	let has_document = active.is_some();
	let scratch = active == Some(TabKind::Scratch);
	let Some(menu_bar) = frame.get_menu_bar() else {
		return;
	};
	set_scratch_shortcuts(frame, config, scratch);
	for &id in DOCUMENT_DEPENDENT_IDS {
		menu_bar.enable_item(id, has_document && !(scratch && scratch_disallows(id)));
	}
	#[cfg(not(target_os = "macos"))]
	for &id in DOCUMENT_DEPENDENT_EDIT_IDS {
		// Copy and Select All work on a scratch pad: both act on text the control holds.
		menu_bar.enable_item(id, has_document);
	}
	if scratch {
		for &id in SCRATCH_DISABLED_IDS {
			menu_bar.enable_item(id, false);
		}
	}
	// The command table carries the `Enable::Always` items too, so they are set here rather than
	// left alone: a scratch pad has to be able to switch some of them off, and switching back to a
	// document has to switch them on again.
	for command in commands::COMMANDS {
		let allowed = !scratch || scratch_pad::available_in_scratch(command.action);
		let enabled = match command.enable {
			Enable::Always => allowed,
			Enable::HasDocument => has_document && allowed,
			// Applied from the config's state by `update_reopen_state` and `create_menu_bar`.
			Enable::HasRecentlyClosed | Enable::HasRecentDocuments => continue,
		};
		menu_bar.enable_item(command.id(), enabled);
	}
}

/// Takes the reader's shortcuts off the menu items a scratch pad cannot use, and puts them back.
///
/// Greying an item out is not enough on Windows, and this is the part that is easy to miss. A menu
/// item's shortcut is registered in the frame's accelerator table when the bar is built, and a
/// keystroke that table matches is translated before the focused control is ever offered it -
/// whether or not the item is enabled. So a greyed-out `Next Heading` still swallows `h`, which in a
/// scratch pad is a letter to type.
///
/// The shortcut comes from the label, after a tab, so taking it off the label is what takes it out
/// of that table: wx rebuilds the table when an item's label changes. Nothing is lost by doing it.
/// The command is greyed out on a scratch pad anyway, so a shortcut that would have done nothing
/// costs nothing, and both the hint and the shortcut come back with the item as soon as a document
/// is in front.
///
/// Only the shortcut column is touched, never the text before it, so an access key that the menu
/// builder moved to avoid a clash stays where it was put.
fn set_scratch_shortcuts(frame: &Frame, config: &ConfigManager, scratch: bool) {
	let Some(menu_bar) = frame.get_menu_bar() else {
		return;
	};
	for id in menu_item_ids() {
		let Some(action) = menu_ids::action_for(id) else {
			continue;
		};
		if scratch_pad::available_in_scratch(action) {
			continue;
		}
		let Some(item) = menu_bar.find_item(id) else {
			continue;
		};
		let label = item.get_label();
		if scratch {
			if let Some((text, _shortcut)) = label.split_once('\t') {
				item.set_label(text);
			}
		} else if !label.contains('\t') {
			let shortcut = config.get_shortcut_menu_str(action);
			if !shortcut.is_empty() {
				item.set_label(&wx_utils::menu_label(&label, &shortcut));
			}
		}
	}
}

/// Every menu id that stands for a command, whichever of the two places its item is built in: the
/// command table, and the lists of items still assembled by hand in the menu files.
fn menu_item_ids() -> impl Iterator<Item = i32> {
	commands::COMMANDS
		.iter()
		.map(commands::Command::id)
		.chain(DOCUMENT_DEPENDENT_IDS.iter().copied())
		.chain(SCRATCH_DISABLED_IDS.iter().copied())
}

/// Whether an id in [`DOCUMENT_DEPENDENT_IDS`] is one a scratch pad cannot use. Ids belonging to
/// no action of their own keep the document-only rule they already had.
fn scratch_disallows(id: i32) -> bool {
	menu_ids::action_for(id).is_some_and(|action| !scratch_pad::available_in_scratch(action))
}

pub fn update_reopen_state(frame: &Frame, has_recently_closed: bool) {
	commands::apply_enable(frame, Enable::HasRecentlyClosed, has_recently_closed);
}

/// Everything the menu bar's enabled state depends on, read off the manager in one go.
///
/// A value rather than a borrow so that a caller can read it under the document manager's lock and
/// then drop that lock: the menu bar is never touched with the manager held. Kept together so a
/// caller cannot pass one half and forget the other, which is how the reopen item once came back
/// enabled and doing nothing after a menu bar was rebuilt.
#[derive(Clone, Copy)]
pub struct MenuState {
	pub active: Option<TabKind>,
	pub has_recently_closed: bool,
}

impl DocumentManager {
	pub fn menu_state(&self) -> MenuState {
		MenuState { active: self.active_tab_kind(), has_recently_closed: self.has_recently_closed() }
	}
}

/// Both of the updates above from one `MenuState`. `create_menu_bar` comes back with everything
/// enabled, so whichever of its callers just built a bar has to call this.
pub fn refresh_menu_states(frame: &Frame, config: &ConfigManager, state: MenuState) {
	update_menu_item_states(frame, config, state.active);
	update_reopen_state(frame, state.has_recently_closed);
}

#[cfg(test)]
mod tests {
	use super::*;

	/// Every id in the list has to belong to an action, because belonging to one is how a scratch
	/// pad knows whether to grey it out. An id added here without an action behind it would be
	/// silently left enabled on a scratch pad.
	#[test]
	fn every_document_dependent_id_belongs_to_an_action() {
		for &id in DOCUMENT_DEPENDENT_IDS {
			assert!(
				menu_ids::action_for(id).is_some(),
				"menu id {id} is in DOCUMENT_DEPENDENT_IDS but belongs to no action"
			);
		}
	}

	/// Everything a scratch pad greys out must also have its shortcut taken off, which means
	/// `menu_item_ids` has to reach every one of them. A menu item missing from those lists would
	/// be greyed and still swallow its key, which for a key that types is the bug this guards.
	#[test]
	fn every_menu_item_is_reachable_for_the_scratch_pad_shortcut_pass() {
		use paperback_core::config::ActionId;
		let reached: Vec<i32> = menu_item_ids().collect();
		for &action in ActionId::all() {
			if scratch_pad::available_in_scratch(action) || crate::ui::menu::is_keyboard_only(action) {
				continue;
			}
			let id = menu_ids::action_to_menu_id(action);
			assert!(
				reached.contains(&id),
				"{action:?} is neither available on a scratch pad nor reachable by the shortcut pass, so its key would still be swallowed"
			);
		}
	}

	/// The whole point of routing both through one predicate: an id that is document-dependent and
	/// whose action a scratch pad cannot use must come out greyed rather than merely enabled.
	#[test]
	fn a_scratch_pad_greys_out_the_document_dependent_ids_it_cannot_use() {
		let disallowed: Vec<i32> = DOCUMENT_DEPENDENT_IDS.iter().copied().filter(|&id| scratch_disallows(id)).collect();
		for id in [
			menu_ids::GO_TO_PAGE,
			menu_ids::TABLE_OF_CONTENTS,
			menu_ids::ELEMENTS_LIST,
			menu_ids::REVEAL_FILE_IN_FOLDER,
			menu_ids::OPEN_IN_WEB_VIEW,
			menu_ids::VIEW_SOURCE,
			menu_ids::IMPORT_DOCUMENT_DATA,
			menu_ids::EXPORT_TO_HTML,
			menu_ids::EXPORT_TO_MARKDOWN,
		] {
			assert!(disallowed.contains(&id), "menu id {id} should be greyed out on a scratch pad");
		}
		for id in [menu_ids::FIND, menu_ids::GO_TO_LINE, menu_ids::GO_TO_PERCENT, menu_ids::WORD_COUNT] {
			assert!(!disallowed.contains(&id), "menu id {id} should stay enabled on a scratch pad");
		}
	}
}
