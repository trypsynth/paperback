use std::{cell::Cell, rc::Rc};

#[cfg(target_os = "linux")]
use gtk::{Dialog, PolicyType, ResponseType, Window, prelude::*};
use paperback_core::document::TocItem;
#[cfg(target_os = "linux")]
use wxdragon::prelude::Frame;
#[cfg(target_os = "linux")]
use wxdragon::translations::translate as t;
#[cfg(not(target_os = "linux"))]
use wxdragon::{prelude::*, translations::translate as t};

#[cfg(target_os = "linux")]
use super::accessible_tree::{self, AccessibleTree};

#[cfg(not(target_os = "linux"))]
const DIALOG_PADDING: i32 = 10;
#[cfg(not(target_os = "linux"))]
const KEY_SPACE: i32 = 32;

pub fn show_toc_dialog(parent: &Frame, toc_items: &[TocItem], current_offset: i32) -> Option<i32> {
	#[cfg(target_os = "linux")]
	return show_toc_dialog_gtk(parent, toc_items, current_offset);
	#[cfg(not(target_os = "linux"))]
	return show_toc_dialog_wx(parent, toc_items, current_offset);
}

#[cfg(target_os = "linux")]
fn show_toc_dialog_gtk(parent: &Frame, toc_items: &[TocItem], current_offset: i32) -> Option<i32> {
	gtk::init().ok()?;
	let dialog = Dialog::with_buttons(
		Some(&t("Table of Contents")),
		Window::NONE,
		gtk::DialogFlags::MODAL | gtk::DialogFlags::DESTROY_WITH_PARENT,
		&[(&t("OK"), ResponseType::Ok), (&t("Cancel"), ResponseType::Cancel)],
	);
	accessible_tree::set_transient_parent(&dialog, parent);
	dialog.set_default_size(400, 500);
	dialog.set_default_response(ResponseType::Ok);
	let mut tree = AccessibleTree::new();
	populate_toc(&mut tree, toc_items, 0);
	let select_idx = if current_offset != -1 { tree.expand_to_offset(i64::from(current_offset)) } else { None };
	let selected_offset = Rc::new(Cell::new(-1i64));
	let sel = Rc::clone(&selected_offset);
	let cancel = dialog.widget_for_response(ResponseType::Cancel).unwrap();
	tree.connect_events(&dialog, Rc::new(move |offset| sel.set(offset)), cancel);
	let scrolled = gtk::ScrolledWindow::builder()
		.hscrollbar_policy(PolicyType::Automatic)
		.vscrollbar_policy(PolicyType::Automatic)
		.build();
	scrolled.add(&tree.list_box);
	dialog.content_area().pack_start(&scrolled, true, true, 0);
	dialog.show_all();
	tree.show_and_focus(select_idx.unwrap_or(0));
	let response = dialog.run();
	unsafe {
		dialog.destroy();
	}
	if response == ResponseType::Ok {
		let offset = selected_offset.get();
		if offset >= 0 { Some(offset as i32) } else { None }
	} else {
		None
	}
}

#[cfg(target_os = "linux")]
fn populate_toc(tree: &mut AccessibleTree, items: &[TocItem], depth: i32) {
	for item in items {
		let name = if item.name.is_empty() { t("Untitled") } else { item.name.clone() };
		let offset = i64::try_from(item.offset).unwrap_or(i64::MAX);
		tree.add_item(&name, offset, depth, !item.children.is_empty());
		if !item.children.is_empty() {
			populate_toc(tree, &item.children, depth + 1);
		}
	}
}

#[cfg(not(target_os = "linux"))]
fn show_toc_dialog_wx(parent: &Frame, toc_items: &[TocItem], current_offset: i32) -> Option<i32> {
	let dialog_title = t("Table of Contents");
	let dialog = Dialog::builder(parent, &dialog_title).build();
	let selected_offset = Rc::new(Cell::new(-1));
	let (tree, _root) = build_toc_tree(dialog, toc_items, current_offset);
	bind_toc_selection(tree, Rc::clone(&selected_offset));
	bind_toc_activation(dialog, tree, Rc::clone(&selected_offset));
	bind_toc_search(tree);
	let (ok_button, cancel_button) = build_toc_buttons(dialog);
	bind_toc_ok(dialog, ok_button, Rc::clone(&selected_offset));
	bind_toc_layout(dialog, tree, ok_button, cancel_button);
	tree.set_focus();
	if dialog.show_modal() == wxdragon::id::ID_OK {
		let offset = selected_offset.get();
		if offset >= 0 { Some(offset) } else { None }
	} else {
		None
	}
}

#[cfg(not(target_os = "linux"))]
fn build_toc_tree(dialog: Dialog, toc_items: &[TocItem], current_offset: i32) -> (TreeCtrl, TreeItemId) {
	let tree = TreeCtrl::builder(&dialog)
		.with_style(TreeCtrlStyle::Default | TreeCtrlStyle::HideRoot)
		.with_size(Size::new(400, 500))
		.build();
	let root = tree.add_root(&t("Root"), None, None).unwrap();
	populate_toc_tree(tree, &root, toc_items);
	if current_offset != -1 {
		find_and_select_item(tree, &root, current_offset);
	}
	(tree, root)
}

#[cfg(not(target_os = "linux"))]
fn bind_toc_selection(tree: TreeCtrl, selected_offset: Rc<Cell<i32>>) {
	let tree_for_sel = tree;
	tree.on_selection_changed(move |event| {
		if let Some(item) = event.get_item() {
			if let Some(data) = tree_for_sel.get_custom_data(&item) {
				if let Some(offset) = data.downcast_ref::<i32>() {
					selected_offset.set(*offset);
				}
			}
		}
	});
}

#[cfg(not(target_os = "linux"))]
fn bind_toc_activation(dialog: Dialog, tree: TreeCtrl, selected_offset: Rc<Cell<i32>>) {
	let dialog_for_activate = dialog;
	let tree_for_activate = tree;
	tree.on_item_activated(move |event| {
		if let Some(item) = event.get_item() {
			if let Some(data) = tree_for_activate.get_custom_data(&item) {
				if let Some(offset) = data.downcast_ref::<i32>() {
					selected_offset.set(*offset);
					dialog_for_activate.end_modal(wxdragon::id::ID_OK);
				}
			}
		}
	});
}

#[cfg(not(target_os = "linux"))]
fn bind_toc_search(tree: TreeCtrl) {
	// Prevent space from firing item_activated (which our handler maps to OK).
	tree.bind_internal(EventType::KEY_DOWN, move |event| {
		if let Some(key) = event.get_key_code() {
			if key == KEY_SPACE {
				event.skip(false);
				return;
			}
		}
		event.skip(true);
	});
}

#[cfg(not(target_os = "linux"))]
fn build_toc_buttons(dialog: Dialog) -> (Button, Button) {
	let ok_button = Button::builder(&dialog).with_label(&t("OK")).build();
	let cancel_button = Button::builder(&dialog).with_id(wxdragon::id::ID_CANCEL).with_label(&t("Cancel")).build();
	(ok_button, cancel_button)
}

#[cfg(not(target_os = "linux"))]
fn bind_toc_ok(dialog: Dialog, ok_button: Button, selected_offset: Rc<Cell<i32>>) {
	dialog.set_escape_id(wxdragon::id::ID_CANCEL);
	let dialog_for_ok = dialog;
	ok_button.on_click(move |_| {
		if selected_offset.get() >= 0 {
			dialog_for_ok.end_modal(wxdragon::id::ID_OK);
		} else {
			MessageDialog::builder(
				&dialog_for_ok,
				&t("Please select a section from the table of contents."),
				&t("No Selection"),
			)
			.with_style(MessageDialogStyle::OK | MessageDialogStyle::IconInformation | MessageDialogStyle::Centre)
			.build()
			.show_modal();
		}
	});
}

#[cfg(not(target_os = "linux"))]
fn bind_toc_layout(dialog: Dialog, tree: TreeCtrl, ok_button: Button, cancel_button: Button) {
	let content_sizer = BoxSizer::builder(Orientation::Vertical).build();
	content_sizer.add(&tree, 1, SizerFlag::Expand | SizerFlag::All, DIALOG_PADDING);
	let button_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	button_sizer.add_stretch_spacer(1);
	button_sizer.add(&ok_button, 0, SizerFlag::Right, DIALOG_PADDING);
	button_sizer.add(&cancel_button, 0, SizerFlag::Right, DIALOG_PADDING);
	content_sizer.add_sizer(&button_sizer, 0, SizerFlag::Expand | SizerFlag::Bottom | SizerFlag::Right, DIALOG_PADDING);
	dialog.set_sizer_and_fit(content_sizer, true);
	dialog.centre();
}

#[cfg(not(target_os = "linux"))]
fn populate_toc_tree(tree: TreeCtrl, parent: &TreeItemId, items: &[TocItem]) {
	for item in items {
		let display_text = if item.name.is_empty() { t("Untitled") } else { item.name.clone() };
		let offset = i32::try_from(item.offset).unwrap_or(i32::MAX);
		if let Some(id) = tree.append_item_with_data(parent, &display_text, offset, None, None) {
			if !item.children.is_empty() {
				populate_toc_tree(tree, &id, &item.children);
			}
		}
	}
}

#[cfg(not(target_os = "linux"))]
fn find_and_select_item(tree: TreeCtrl, parent: &TreeItemId, offset: i32) -> bool {
	if let Some((child, mut cookie)) = tree.get_first_child(parent) {
		let mut current_child = Some(child);
		while let Some(item) = current_child {
			if let Some(data) = tree.get_custom_data(&item) {
				if let Some(item_offset) = data.downcast_ref::<i32>() {
					if *item_offset == offset {
						tree.select_item(&item);
						tree.set_focused_item(&item);
						tree.ensure_visible(&item);
						return true;
					}
				}
			}
			if find_and_select_item(tree, &item, offset) {
				return true;
			}
			current_child = tree.get_next_child(parent, &mut cookie);
		}
	}
	false
}
