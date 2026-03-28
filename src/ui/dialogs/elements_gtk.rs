use std::{cell::Cell, rc::Rc};

use gtk::{Dialog, ListBox as GtkListBox, PolicyType, ResponseType, Widget, Window, gdk::EventKey, glib::Propagation, prelude::*};
use wxdragon::{prelude::Frame, translations::translate as t};

use super::accessible_tree::{self, AccessibleTree};
use crate::session::DocumentSession;

pub fn show_elements_dialog(parent: &Frame, session: &DocumentSession, current_pos: i64) -> Option<i64> {
	gtk::init().ok()?;

	let dialog = Dialog::with_buttons(
		Some(&t("Elements")),
		Window::NONE,
		gtk::DialogFlags::MODAL | gtk::DialogFlags::DESTROY_WITH_PARENT,
		&[(&t("OK"), ResponseType::Ok), (&t("Cancel"), ResponseType::Cancel)],
	);
	accessible_tree::set_transient_parent(&dialog, parent);
	dialog.set_default_size(400, 500);
	dialog.set_default_response(ResponseType::Ok);

	let selected_offset = Rc::new(Cell::new(-1i64));

	// View switcher
	let view_combo = gtk::ComboBoxText::new();
	view_combo.append_text(&t("Headings"));
	view_combo.append_text(&t("Links"));
	view_combo.set_active(Some(0));

	// Headings tree
	let mut headings_tree = AccessibleTree::new();
	let tree_data = session.heading_tree(current_pos);
	populate_headings(&mut headings_tree, &tree_data.items);
	let headings_select_idx = if tree_data.closest_index >= 0 {
		// The heading tree uses flat indices; find the matching row
		tree_data.items.get(tree_data.closest_index as usize)
			.and_then(|item| headings_tree.expand_to_offset(i64::try_from(item.offset).unwrap_or(i64::MAX)))
	} else {
		None
	};

	let sel_headings = Rc::clone(&selected_offset);
	headings_tree.connect_events(&dialog, Rc::new(move |offset| sel_headings.set(offset)), view_combo.clone());

	let headings_scrolled = gtk::ScrolledWindow::builder()
		.hscrollbar_policy(PolicyType::Automatic)
		.vscrollbar_policy(PolicyType::Automatic)
		.build();
	headings_scrolled.add(&headings_tree.list_box);

	// Links list
	let links_list = GtkListBox::new();
	links_list.set_selection_mode(gtk::SelectionMode::Browse);
	let link_data = session.link_list(current_pos);
	let mut link_offsets = Vec::new();
	for item in &link_data.items {
		let label = gtk::Label::new(Some(&item.text));
		label.set_xalign(0.0);
		let row = gtk::ListBoxRow::new();
		row.add(&label);
		links_list.add(&row);
		link_offsets.push(i64::try_from(item.offset).unwrap_or(i64::MAX));
	}
	let link_offsets = Rc::new(link_offsets);

	let sel_links = Rc::clone(&selected_offset);
	let offsets_for_sel = Rc::clone(&link_offsets);
	links_list.connect_row_selected(move |_, row| {
		if let Some(row) = row {
			let idx = row.index() as usize;
			if let Some(&offset) = offsets_for_sel.get(idx) {
				sel_links.set(offset);
			}
		}
	});

	let dialog_for_link_activate = dialog.clone();
	links_list.connect_row_activated(move |_, _| {
		dialog_for_link_activate.response(ResponseType::Ok);
	});

	connect_list_tab_handling(&links_list, &dialog, view_combo.clone());

	let links_scrolled = gtk::ScrolledWindow::builder()
		.hscrollbar_policy(PolicyType::Automatic)
		.vscrollbar_policy(PolicyType::Automatic)
		.build();
	links_scrolled.add(&links_list);

	// Layout
	let content = dialog.content_area();
	content.pack_start(&view_combo, false, false, 5);
	content.pack_start(&headings_scrolled, true, true, 0);
	content.pack_start(&links_scrolled, true, true, 0);

	// View toggle
	let headings_scrolled_for_toggle = headings_scrolled.clone();
	let links_scrolled_for_toggle = links_scrolled.clone();
	view_combo.connect_changed(move |combo| {
		let is_headings = combo.active() == Some(0);
		headings_scrolled_for_toggle.set_visible(is_headings);
		links_scrolled_for_toggle.set_visible(!is_headings);
	});

	dialog.show_all();
	links_scrolled.set_visible(false);

	headings_tree.show_and_focus(headings_select_idx.unwrap_or(0));
	if !link_data.items.is_empty() {
		let link_idx = if link_data.closest_index >= 0 { link_data.closest_index } else { 0 };
		if let Some(row) = links_list.row_at_index(link_idx) {
			links_list.select_row(Some(&row));
		}
	}

	let response = dialog.run();
	unsafe { dialog.destroy(); }

	if response == ResponseType::Ok {
		let offset = selected_offset.get();
		if offset >= 0 { Some(offset) } else { None }
	} else {
		None
	}
}

const GDK_KEY_TAB: u32 = 0xff09;
const GDK_KEY_ISO_LEFT_TAB: u32 = 0xfe20;

fn connect_list_tab_handling(list_box: &GtkListBox, dialog: &Dialog, shift_tab_target: impl IsA<Widget> + 'static) {
	let dialog_clone = dialog.clone();
	let shift_target: Widget = shift_tab_target.upcast();
	list_box.connect_key_press_event(move |_, event| {
		let keyval = *event.keyval();
		if keyval == GDK_KEY_TAB || keyval == GDK_KEY_ISO_LEFT_TAB {
			if keyval == GDK_KEY_TAB {
				if let Some(button) = dialog_clone.widget_for_response(ResponseType::Ok) {
					button.grab_focus();
				}
			} else {
				shift_target.grab_focus();
			}
			return Propagation::Stop;
		}
		Propagation::Proceed
	});
}

fn populate_headings(tree: &mut AccessibleTree, items: &[crate::types::HeadingTreeItem]) {
	// HeadingTreeItem uses flat parent_index references. We need to convert
	// to depth-first order with depth tracking.
	let mut depths = Vec::with_capacity(items.len());
	let mut child_counts = vec![0usize; items.len()];

	// First pass: compute depths and mark which items have children
	for item in items {
		let depth = if item.parent_index >= 0 {
			depths.get(item.parent_index as usize).map_or(0, |&d: &i32| d + 1)
		} else {
			0
		};
		depths.push(depth);
		if item.parent_index >= 0 {
			if let Ok(pi) = usize::try_from(item.parent_index) {
				child_counts[pi] += 1;
			}
		}
	}

	// Second pass: add items (already in depth-first order from session)
	for (i, item) in items.iter().enumerate() {
		let name = if item.text.is_empty() { t("Untitled") } else { item.text.clone() };
		let offset = i64::try_from(item.offset).unwrap_or(i64::MAX);
		tree.add_item(&name, offset, depths[i], child_counts[i] > 0);
	}
}
