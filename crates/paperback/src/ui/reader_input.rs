//! The reading control: building it, and everything it does in response to a key or a click.
//!
//! A closure installed on a control cannot borrow the tab that owns it, so the handlers go
//! through the document manager to reach the active tab, which is the tab whose control has
//! focus.

use std::{rc::Rc, sync::Mutex};

use paperback_core::config::ActionId;
use patois::t;
use wxdragon::prelude::*;

use super::{
	document_manager::{DocumentManager, DocumentTab},
	menu_ids,
	text_render::reload_window_around,
};

/// Builds the reading control and binds everything it responds to.
pub(super) fn build_text_ctrl(
	panel: Panel,
	word_wrap: bool,
	self_rc: &Rc<Mutex<DocumentManager>>,
	frame: Frame,
) -> TextCtrl {
	let style = TextCtrlStyle::MultiLine
		| TextCtrlStyle::ReadOnly
		| TextCtrlStyle::Rich2
		| if word_wrap { TextCtrlStyle::WordWrap } else { TextCtrlStyle::DontWrap };
	let text_ctrl = TextCtrl::builder(&panel).with_style(style).build();
	let dm_for_enter = Rc::clone(self_rc);
	text_ctrl.on_char(move |event| {
		if let WindowEventData::Keyboard(kbd) = event {
			if kbd.get_key_code() == Some(13) || kbd.get_key_code() == Some(32) {
				let table_html = {
					let dm = dm_for_enter.lock().unwrap();
					dm.activate_current_table()
				};
				if let Some(html) = table_html {
					let frame = dm_for_enter.lock().unwrap().frame;
					// TRANSLATORS: Title of the dialog showing the HTML rendering of a table activated in the document
					super::dialogs::show_web_view_dialog(&frame, &t("Table View"), &html, false, None);
				} else {
					let mut dm = dm_for_enter.lock().unwrap();
					dm.activate_current_link();
				}
			} else {
				kbd.event.skip(true);
			}
		}
	});
	// Ctrl+C is intercepted here rather than through the Edit menu because that menu only
	// exists on macOS; elsewhere the control handles the key itself and no menu event fires.
	#[cfg(not(target_os = "macos"))]
	{
		let dm_for_copy = Rc::clone(self_rc);
		text_ctrl.bind_internal(EventType::KEY_DOWN, move |event| {
			let handled = event.get_key_code() == Some(i32::from(b'C'))
				&& event.control_down()
				&& dm_for_copy.try_lock().is_ok_and(|dm| dm.copy_whole_document_if_all_selected());
			if !handled {
				event.skip(true);
			}
		});
	}
	let dm_for_key_up = Rc::clone(self_rc);
	text_ctrl.bind_internal(EventType::KEY_UP, move |event| {
		event.skip(true);
		if let Ok(mut dm) = dm_for_key_up.try_lock() {
			// Before the status bar reads the position, so it reports the compacted window.
			dm.compact_window_after_user_move();
			dm.update_status_bar();
			dm.save_position_throttled();
			dm.check_bookmark_sounds();
		}
	});
	let dm_for_mouse = Rc::clone(self_rc);
	text_ctrl.bind_internal(wxdragon::event::EventType::LEFT_UP, move |event| {
		event.skip(true);
		if let Ok(mut dm) = dm_for_mouse.try_lock() {
			dm.set_preferred_column(None);
			dm.compact_window_after_user_move();
			dm.update_status_bar();
			dm.save_position_throttled();
			dm.check_bookmark_sounds();
		}
	});
	let text_ctrl_for_menu = text_ctrl;
	let dm_for_keys = Rc::clone(self_rc);
	let frame_for_keys = frame;
	text_ctrl.on_key_down(move |event| {
		if let WindowEventData::Keyboard(kbd) = &event
			&& let Some(key) = kbd.get_key_code()
		{
			if (key == WXK_F10 && kbd.shift_down()) || key == WXK_WINDOWS_MENU {
				kbd.event.skip(false);
				show_reader_context_menu(text_ctrl_for_menu);
				return;
			}
			if let Some(to_end) = document_edge_for_key(key, kbd.control_down(), kbd.shift_down(), kbd.alt_down()) {
				kbd.event.skip(false);
				if let Ok(mut dm) = dm_for_keys.try_lock() {
					dm.set_preferred_column(None);
					dm.jump_to_document_edge(to_end);
				}
				return;
			}
			#[cfg(target_os = "windows")]
			if (key == WXK_DOWN || key == WXK_UP) && !kbd.shift_down() && !kbd.control_down() && !kbd.alt_down() {
				let going_down = key == WXK_DOWN;
				let nav_result = dm_for_keys.try_lock().ok().and_then(|mut dm| {
					let start_of_line = dm.config.lock().unwrap().get_app_bool("line_start_navigation", false);
					let pref_col = dm.preferred_column();
					dm.active_tab_mut()
						.and_then(|tab| navigate_line_by_column(tab, going_down, pref_col, start_of_line))
				});
				if let Some((new_pos, new_col)) = nav_result {
					kbd.event.skip(false);
					text_ctrl_for_menu.set_insertion_point(new_pos);
					text_ctrl_for_menu.show_position(new_pos);
					if let Ok(dm) = dm_for_keys.try_lock() {
						dm.set_preferred_column(Some(new_col));
						dm.update_status_bar();
					}
				} else {
					kbd.event.skip(true);
				}
				return;
			}
			#[cfg(target_os = "windows")]
			if let Ok(dm) = dm_for_keys.try_lock() {
				dm.set_preferred_column(None);
			}
			let action = {
				if let Ok(dm) = dm_for_keys.try_lock() {
					let config = dm.config.lock().unwrap();
					config.get_shortcuts().find_action(key, kbd.control_down(), kbd.alt_down(), kbd.shift_down())
				} else {
					None
				}
			};
			if let Some(act) = action {
				match act {
					ActionId::AnnouncePercent => {
						kbd.event.skip(false);
						if let Ok(dm) = dm_for_keys.try_lock() {
							dm.announce_current_percent();
						}
						return;
					}
					ActionId::SetTemporaryBookmark => {
						kbd.event.skip(false);
						if let Ok(dm) = dm_for_keys.try_lock() {
							dm.set_temporary_bookmark();
						}
						return;
					}
					ActionId::JumpToTemporaryBookmark => {
						kbd.event.skip(false);
						if let Ok(mut dm) = dm_for_keys.try_lock() {
							dm.jump_to_temporary_bookmark();
						}
						return;
					}
					_ => {
						if !kbd.control_down() && !kbd.alt_down() || cfg!(target_os = "linux") {
							let menu_id = menu_ids::action_to_menu_id(act);
							kbd.event.skip(false);
							frame_for_keys.process_menu_command(menu_id);
							return;
						}
					}
				}
			}
		}
		event.skip(true);
	});
	let text_ctrl_for_right_click = text_ctrl;
	text_ctrl.bind_internal(EventType::RIGHT_UP, move |event| {
		event.skip(false);
		show_reader_context_menu(text_ctrl_for_right_click);
	});
	text_ctrl
}

/// Which end of the document a key press names as a "jump to the very start/end" gesture, if
/// any: `Some(true)` for the end, `Some(false)` for the start. See
/// `DocumentManager::jump_to_document_edge` for why these are intercepted rather than left to
/// the text control.
///
/// Ctrl+Home/Ctrl+End everywhere - wxWidgets reports macOS's Command key as `control_down`, so
/// that covers Cmd+Home/Cmd+End there - plus Cmd+Up/Cmd+Down on macOS, which is what Mac text
/// views actually bind document start/end to, and the only one of the two most Apple keyboards
/// can even type (they have no Home/End keys). Bare Home/End are deliberately not included on
/// macOS: there they scroll without moving the caret, which is a different gesture.
const fn document_edge_for_key(key: i32, control: bool, shift: bool, alt: bool) -> Option<bool> {
	if !control || shift || alt {
		return None;
	}
	match key {
		WXK_HOME => Some(false),
		WXK_END => Some(true),
		#[cfg(target_os = "macos")]
		WXK_UP => Some(false),
		#[cfg(target_os = "macos")]
		WXK_DOWN => Some(true),
		_ => None,
	}
}

/// One line-vertical-navigation attempt within whatever's currently loaded in `tab.text_ctrl`.
/// Returns `None` (outer) if the current position has no known line/column (shouldn't happen in
/// practice), `Some(None)` if the target line falls outside what's currently loaded - the caller
/// checks whether there's more document in that direction and, if so, reloads and retries - or
/// `Some(Some(..))` on success.
#[cfg(target_os = "windows")]
fn try_navigate_line_by_column(
	tab: &DocumentTab,
	going_down: bool,
	pref_col: Option<i64>,
	start_of_line: bool,
) -> Option<Option<(i64, i64)>> {
	let text_ctrl = tab.text_ctrl;
	let current_pos = text_ctrl.get_insertion_point().max(0);
	let (current_col, current_line) = text_ctrl.position_to_xy(current_pos)?;
	let col = pref_col.unwrap_or(current_col);
	let target_line = if going_down { current_line + 1 } else { current_line - 1 };
	if target_line < 0 {
		return Some(None);
	}
	let target_line_start = text_ctrl.xy_to_position(0, target_line);
	if target_line_start < 0 {
		return Some(None);
	}
	if start_of_line {
		return Some(Some((target_line_start, 0)));
	}
	let target_line_len = i64::from(text_ctrl.get_line_length(target_line));
	let new_pos = target_line_start + col.min(target_line_len);
	Some(Some((new_pos, col)))
}

/// Returns (`new_position`, `preferred_column`) for vertical navigation.
/// With `start_of_line` set, the caret lands at the start of the target visual line. Otherwise it
/// uses character-column-based navigation (`pref_col` or the current column), so the cursor lands on
/// the same character column (not pixel column) on the target visual line.
///
/// Reloads `tab`'s window and retries once if Up/Down would otherwise stop at a loaded-window
/// boundary that isn't the document's actual start/end. Without this, Up/Down (and Page Up/Down,
/// which RichEdit handles natively with no window awareness at all) can strand the caret mid-chapter
/// with no keyboard-only way past it except an explicit jump (heading/bookmark navigation etc.) -
/// found the hard way testing a huge book, not something worth leaving as a TODO.
#[cfg(target_os = "windows")]
fn navigate_line_by_column(
	tab: &mut DocumentTab,
	going_down: bool,
	pref_col: Option<i64>,
	start_of_line: bool,
) -> Option<(i64, i64)> {
	if let Some(result) = try_navigate_line_by_column(tab, going_down, pref_col, start_of_line)? {
		return Some(result);
	}
	let doc_len = tab.session.document_len();
	let has_more = if going_down { tab.window.end() < doc_len } else { tab.window.start() > 0 };
	if !has_more {
		return None;
	}
	let doc_pos = tab.window.to_doc(tab.text_ctrl.get_insertion_point().max(0));
	reload_window_around(tab, doc_pos, "line navigation");
	let local = tab.window.to_local(doc_pos);
	tab.text_ctrl.set_insertion_point(local);
	try_navigate_line_by_column(tab, going_down, pref_col, start_of_line)?
}

fn show_reader_context_menu(text_ctrl: TextCtrl) {
	text_ctrl.set_focus();
	let mut menu = Menu::builder()
		// TRANSLATORS: Right-click context menu item and status text to bookmark the current position
		.append_item(menu_ids::TOGGLE_BOOKMARK, &t("Create &bookmark"), &t("Create bookmark"))
		// TRANSLATORS: Right-click context menu item and status text to bookmark the current position with an attached note
		.append_item(menu_ids::BOOKMARK_WITH_NOTE, &t("Bookmark with &note"), &t("Create bookmark with note"))
		.append_separator()
		// TRANSLATORS: Right-click context menu item and status text to open the find dialog
		.append_item(menu_ids::FIND, &t("&Find"), &t("Find text"))
		// TRANSLATORS: Right-click context menu item and status text to repeat the last search forward
		.append_item(menu_ids::FIND_NEXT, &t("Find &next"), &t("Find next match"))
		// TRANSLATORS: Right-click context menu item and status text to repeat the last search backward
		.append_item(menu_ids::FIND_PREVIOUS, &t("Find &previous"), &t("Find previous match"))
		.append_separator()
		// TRANSLATORS: Right-click context menu item and status text to jump to a specific page
		.append_item(menu_ids::GO_TO_PAGE, &t("Go to &page"), &t("Go to page"))
		// TRANSLATORS: Right-click context menu item and status text to jump to a specific line
		.append_item(menu_ids::GO_TO_LINE, &t("Go to &line"), &t("Go to line"))
		// TRANSLATORS: Right-click context menu item and status text to jump to a percentage through the document
		.append_item(menu_ids::GO_TO_PERCENT, &t("Go to &percent"), &t("Go to percent"))
		.build();
	text_ctrl.popup_menu(&mut menu, None);
}
