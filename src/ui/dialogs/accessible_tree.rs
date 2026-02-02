use std::{cell::RefCell, ffi::c_void, rc::Rc};

use gtk::{Dialog, Label, ListBox, ListBoxRow, ResponseType, Widget, Window, gdk::EventKey, glib::{Propagation, translate}, prelude::*};
use wxdragon::{prelude::{Frame, WxWidget}, translations::translate as t};

const GDK_KEY_LEFT: u32 = 0xff51;
const GDK_KEY_RIGHT: u32 = 0xff53;
const GDK_KEY_TAB: u32 = 0xff09;
const GDK_KEY_ISO_LEFT_TAB: u32 = 0xfe20;
const ATK_POLITENESS_POLITE: i32 = 1;

unsafe extern "C" {
	safe fn gtk_widget_get_accessible(widget: *mut c_void) -> *mut c_void;
	unsafe fn atk_object_set_name(obj: *mut c_void, name: *const std::ffi::c_char);
	unsafe fn g_signal_emit_by_name(instance: *mut c_void, signal_name: *const std::ffi::c_char, ...);
}

struct RowInfo {
	depth: i32,
	has_children: bool,
	expanded: bool,
}

/// A GtkListBox that simulates accessible tree expand/collapse behavior.
pub struct AccessibleTree {
	pub list_box: ListBox,
	rows: Rc<RefCell<Vec<RowInfo>>>,
	offsets: Vec<i64>,
}

impl AccessibleTree {
	pub fn new() -> Self {
		let list_box = ListBox::new();
		list_box.set_selection_mode(gtk::SelectionMode::Browse);
		list_box.set_activate_on_single_click(false);
		Self {
			list_box,
			rows: Rc::new(RefCell::new(Vec::new())),
			offsets: Vec::new(),
		}
	}

	/// Add an item to the tree. Items must be added in depth-first order.
	pub fn add_item(&mut self, name: &str, offset: i64, depth: i32, has_children: bool) {
		let indent = "  ".repeat(depth as usize);
		let display_text = format!("{indent}{name}");

		let label = Label::new(Some(&display_text));
		label.set_xalign(0.0);
		let row = ListBoxRow::new();
		row.add(&label);
		self.list_box.add(&row);

		self.rows.borrow_mut().push(RowInfo { depth, has_children, expanded: false });
		self.offsets.push(offset);

		let idx = self.offsets.len() - 1;
		set_row_accessible_name(&self.list_box, &self.rows.borrow(), idx);
	}

	/// Expand ancestors of the item at the given offset and return its row index.
	pub fn expand_to_offset(&self, offset: i64) -> Option<i32> {
		let target_idx = self.offsets.iter().position(|&o| o == offset)?;
		let mut info = self.rows.borrow_mut();
		let target_depth = info[target_idx].depth;
		let mut required_depth = target_depth - 1;
		if required_depth >= 0 {
			for j in (0..target_idx).rev() {
				if info[j].depth == required_depth {
					info[j].expanded = true;
					set_row_accessible_name(&self.list_box, &info, j);
					if required_depth == 0 {
						break;
					}
					required_depth -= 1;
				}
			}
		}
		update_visibility(&self.list_box, &info);
		Some(target_idx as i32)
	}

	/// Show the tree with initial visibility applied, and focus the given row index.
	pub fn show_and_focus(&self, focus_idx: i32) {
		update_visibility(&self.list_box, &self.rows.borrow());
		if let Some(row) = self.list_box.row_at_index(focus_idx) {
			self.list_box.select_row(Some(&row));
			row.grab_focus();
		}
	}

	/// Connect selection tracking, activation, and key handling.
	/// `shift_tab_target`: widget to focus on Shift-Tab.
	pub fn connect_events(&self, dialog: &Dialog, on_select: Rc<dyn Fn(i64)>, shift_tab_target: impl IsA<Widget> + 'static) {
		let offsets_for_sel = self.offsets.clone();
		self.list_box.connect_row_selected(move |_, row| {
			if let Some(row) = row {
				let idx = row.index() as usize;
				if let Some(&offset) = offsets_for_sel.get(idx) {
					on_select(offset);
				}
			}
		});

		let dialog_clone = dialog.clone();
		self.list_box.connect_row_activated(move |_, _| {
			dialog_clone.response(ResponseType::Ok);
		});

		let rows_for_key = Rc::clone(&self.rows);
		let dialog_for_key = dialog.clone();
		let shift_target: Widget = shift_tab_target.upcast();
		self.list_box.connect_key_press_event(move |lb, event| {
			handle_key(lb, event, &rows_for_key, &dialog_for_key, &shift_target)
		});
	}
}

fn handle_key(lb: &ListBox, event: &EventKey, rows: &Rc<RefCell<Vec<RowInfo>>>, dialog: &Dialog, shift_tab_target: &Widget) -> Propagation {
	let keyval = *event.keyval();

	if keyval == GDK_KEY_TAB || keyval == GDK_KEY_ISO_LEFT_TAB {
		if keyval == GDK_KEY_TAB {
			if let Some(button) = dialog.widget_for_response(ResponseType::Ok) {
				button.grab_focus();
			}
		} else {
			shift_tab_target.grab_focus();
		}
		return Propagation::Stop;
	}

	if keyval != GDK_KEY_LEFT && keyval != GDK_KEY_RIGHT {
		return Propagation::Proceed;
	}

	let Some(selected) = lb.selected_row() else { return Propagation::Proceed };
	let idx = selected.index() as usize;
	let mut info = rows.borrow_mut();
	if idx >= info.len() {
		return Propagation::Proceed;
	}

	if keyval == GDK_KEY_RIGHT {
		if info[idx].has_children && !info[idx].expanded {
			info[idx].expanded = true;
			update_visibility(lb, &info);
			notify_expand_change(lb, &info, idx);
			return Propagation::Stop;
		}
	} else if info[idx].has_children && info[idx].expanded {
		collapse_recursive(&mut info, idx);
		update_visibility(lb, &info);
		notify_expand_change(lb, &info, idx);
		return Propagation::Stop;
	} else if info[idx].depth > 0 {
		let parent_depth = info[idx].depth - 1;
		for j in (0..idx).rev() {
			if info[j].depth == parent_depth {
				drop(info);
				if let Some(row) = lb.row_at_index(j as i32) {
					lb.select_row(Some(&row));
					row.grab_focus();
				}
				return Propagation::Stop;
			}
		}
	}
	Propagation::Proceed
}

fn collapse_recursive(info: &mut [RowInfo], idx: usize) {
	info[idx].expanded = false;
	let depth = info[idx].depth;
	for j in (idx + 1)..info.len() {
		if info[j].depth <= depth {
			break;
		}
		if info[j].has_children {
			info[j].expanded = false;
		}
	}
}

fn update_visibility(list_box: &ListBox, info: &[RowInfo]) {
	for i in 0..info.len() {
		if let Some(row) = list_box.row_at_index(i as i32) {
			row.set_visible(is_visible(info, i));
		}
	}
}

fn is_visible(info: &[RowInfo], idx: usize) -> bool {
	if info[idx].depth == 0 {
		return true;
	}
	let mut required_depth = info[idx].depth - 1;
	for j in (0..idx).rev() {
		if info[j].depth == required_depth {
			if !info[j].expanded {
				return false;
			}
			if required_depth == 0 {
				return true;
			}
			required_depth -= 1;
		}
	}
	false
}

fn set_row_accessible_name(list_box: &ListBox, info: &[RowInfo], idx: usize) {
	let Some(row) = list_box.row_at_index(idx as i32) else { return };
	let label_text = row.child()
		.and_then(|w| w.downcast::<Label>().ok())
		.map(|l| l.text().to_string())
		.unwrap_or_default();
	let accessible_name = if info[idx].has_children {
		let state = if info[idx].expanded { t("expanded") } else { t("collapsed") };
		format!("{label_text}, {state}")
	} else {
		label_text
	};
	let atk_obj = gtk_widget_get_accessible(row.as_ptr() as *mut c_void);
	if !atk_obj.is_null() {
		if let Ok(c_name) = std::ffi::CString::new(accessible_name) {
			unsafe { atk_object_set_name(atk_obj, c_name.as_ptr()); }
		}
	}
}

fn notify_expand_change(list_box: &ListBox, info: &[RowInfo], idx: usize) {
	set_row_accessible_name(list_box, info, idx);
	let Some(row) = list_box.row_at_index(idx as i32) else { return };
	let state_text = if info[idx].expanded { t("expanded") } else { t("collapsed") };
	let atk_obj = gtk_widget_get_accessible(row.as_ptr() as *mut c_void);
	if !atk_obj.is_null() {
		if let Ok(c_msg) = std::ffi::CString::new(state_text) {
			unsafe {
				g_signal_emit_by_name(atk_obj, c"notification".as_ptr(), c_msg.as_ptr(), ATK_POLITENESS_POLITE);
				g_signal_emit_by_name(atk_obj, c"announcement".as_ptr(), c_msg.as_ptr());
			}
		}
	}
}

pub fn set_transient_parent(dialog: &Dialog, parent: &Frame) {
	let handle = parent.get_handle();
	if handle.is_null() {
		return;
	}
	unsafe {
		let gtk_widget: Widget = translate::from_glib_none(handle as *mut gtk::ffi::GtkWidget);
		if let Some(toplevel) = gtk_widget.toplevel() {
			if toplevel.is_toplevel() {
				if let Ok(window) = toplevel.downcast::<Window>() {
					dialog.set_transient_for(Some(&window));
				}
			}
		}
	}
}
