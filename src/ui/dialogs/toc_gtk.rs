use std::{cell::Cell, rc::Rc};

use gtk::{Dialog, PolicyType, ResponseType, Window, prelude::*};
use wxdragon::{prelude::Frame, translations::translate as t};

use super::accessible_tree::{self, AccessibleTree};
use crate::document::TocItem;

pub fn show_toc_dialog(parent: &Frame, toc_items: &[TocItem], current_offset: i32) -> Option<i32> {
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

	let select_idx = if current_offset != -1 {
		tree.expand_to_offset(i64::from(current_offset))
	} else {
		None
	};

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
	unsafe { dialog.destroy(); }

	if response == ResponseType::Ok {
		let offset = selected_offset.get();
		if offset >= 0 { Some(offset as i32) } else { None }
	} else {
		None
	}
}

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
