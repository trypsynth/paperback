#[cfg(target_os = "windows")]
use std::cell::RefCell;
use std::{cell::Cell, rc::Rc};
#[cfg(not(target_os = "windows"))]
use std::{collections::HashMap, ffi::c_void};

use paperback_core::document::TocItem;
use patois::t;
use wxdragon::prelude::*;

#[cfg(target_os = "windows")]
use super::elements::flat::{build_flat_list, flat_list_selected_index, select_flat_row};

#[cfg(target_os = "windows")]
const KEY_SPACE: i32 = 32;

pub fn show_toc_dialog(parent: &Frame, toc_items: &[TocItem], current_offset: i32) -> Option<i32> {
	#[cfg(not(target_os = "windows"))]
	return show_toc_dialog_dv(parent, toc_items, current_offset);
	#[cfg(target_os = "windows")]
	return show_toc_dialog_wx(parent, toc_items, current_offset);
}

// ── DataViewTreeCtrl implementation (Linux + macOS) ───────────────────────────

#[cfg(not(target_os = "windows"))]
fn show_toc_dialog_dv(parent: &Frame, toc_items: &[TocItem], current_offset: i32) -> Option<i32> {
	// TRANSLATORS: Title of the Table of Contents dialog
	let dialog_title = t("Table of Contents");
	let dialog = Dialog::builder(parent, &dialog_title).build();
	let selected_offset = Rc::new(Cell::new(-1i32));
	let tree = DataViewTreeCtrl::builder(&dialog).with_size(dialog.from_dip(Size::new(400, 500))).build();
	let mut item_offsets: HashMap<usize, i32> = HashMap::new();
	populate_toc_tree_dv(tree, &DataViewItem::default(), toc_items, &mut item_offsets);
	if current_offset != -1 {
		find_and_select_dv(tree, &DataViewItem::default(), current_offset, &item_offsets);
	}
	let item_offsets = Rc::new(item_offsets);
	bind_toc_selection_dv(tree, Rc::clone(&item_offsets), Rc::clone(&selected_offset));
	bind_toc_activation_dv(dialog, tree, Rc::clone(&item_offsets), Rc::clone(&selected_offset));
	let (ok_button, cancel_button) = build_toc_buttons(dialog);
	// OK confirms the tree's current selection at click time, as Enter does, rather than
	// trusting a cached offset: the entry nearest the reader is preselected while the tree
	// is being built, before the selection-changed listener is attached, so that listener
	// never records it and OK would otherwise claim nothing is selected.
	let resolve_selected: Rc<dyn Fn() -> Option<i32>> = {
		let tree_for_ok = tree;
		let offsets_for_ok = Rc::clone(&item_offsets);
		Rc::new(move || {
			let item = tree_for_ok.get_selection()?;
			let id_ptr = item.get_id::<c_void>()?;
			offsets_for_ok.get(&(id_ptr as usize)).copied()
		})
	};
	bind_toc_ok(dialog, ok_button, Rc::clone(&selected_offset), resolve_selected);
	bind_toc_layout_dv(dialog, tree, ok_button, cancel_button);
	tree.set_focus();
	if dialog.show_modal() == wxdragon::id::ID_OK {
		let offset = selected_offset.get();
		if offset >= 0 { Some(offset) } else { None }
	} else {
		None
	}
}

#[cfg(not(target_os = "windows"))]
fn populate_toc_tree_dv(
	tree: DataViewTreeCtrl,
	parent: &DataViewItem,
	items: &[TocItem],
	item_offsets: &mut HashMap<usize, i32>,
) {
	for item in items {
		// TRANSLATORS: Placeholder text shown in the table of contents tree when an entry has no title
		let display_text = if item.name.is_empty() { t("Untitled") } else { item.name.clone() };
		let offset = i32::try_from(item.offset).unwrap_or(i32::MAX);
		let node = if item.children.is_empty() {
			tree.append_item(parent, &display_text, -1)
		} else {
			tree.append_container(parent, &display_text, -1, -1)
		};
		if let Some(id_ptr) = node.get_id::<c_void>() {
			item_offsets.insert(id_ptr as usize, offset);
		}
		if !item.children.is_empty() {
			populate_toc_tree_dv(tree, &node, &item.children, item_offsets);
		}
	}
}

#[cfg(not(target_os = "windows"))]
fn bind_toc_selection_dv(
	tree: DataViewTreeCtrl,
	item_offsets: Rc<HashMap<usize, i32>>,
	selected_offset: Rc<Cell<i32>>,
) {
	tree.on_selection_changed(move |event| {
		if let Some(item) = event.get_item()
			&& let Some(id_ptr) = item.get_id::<c_void>()
			&& let Some(&offset) = item_offsets.get(&(id_ptr as usize))
		{
			selected_offset.set(offset);
		}
	});
}

#[cfg(not(target_os = "windows"))]
fn bind_toc_activation_dv(
	dialog: Dialog,
	tree: DataViewTreeCtrl,
	item_offsets: Rc<HashMap<usize, i32>>,
	selected_offset: Rc<Cell<i32>>,
) {
	let dialog_for_activate = dialog;
	tree.on_item_activated(move |event| {
		if let Some(item) = event.get_item()
			&& let Some(id_ptr) = item.get_id::<c_void>()
			&& let Some(&offset) = item_offsets.get(&(id_ptr as usize))
		{
			selected_offset.set(offset);
			dialog_for_activate.end_modal(wxdragon::id::ID_OK);
		}
	});
}

#[cfg(not(target_os = "windows"))]
fn bind_toc_layout_dv(dialog: Dialog, tree: DataViewTreeCtrl, ok_button: Button, cancel_button: Button) {
	let content_sizer = BoxSizer::builder(Orientation::Vertical).build();
	content_sizer.add(&tree, 1, SizerFlag::Expand | SizerFlag::All, super::DIALOG_PADDING);
	let button_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	button_sizer.add_stretch_spacer(1);
	// macOS HIG puts the default/affirmative action rightmost (Cancel, then OK); this
	// implementation also covers Linux, which keeps the OK-then-Cancel order.
	#[cfg(target_os = "macos")]
	{
		button_sizer.add(&cancel_button, 0, SizerFlag::Right, super::DIALOG_PADDING);
		button_sizer.add(&ok_button, 0, SizerFlag::Right, super::DIALOG_PADDING);
	}
	#[cfg(not(target_os = "macos"))]
	{
		button_sizer.add(&ok_button, 0, SizerFlag::Right, super::DIALOG_PADDING);
		button_sizer.add(&cancel_button, 0, SizerFlag::Right, super::DIALOG_PADDING);
	}
	content_sizer.add_sizer(
		&button_sizer,
		0,
		SizerFlag::Expand | SizerFlag::Bottom | SizerFlag::Right,
		super::DIALOG_PADDING,
	);
	dialog.set_sizer_and_fit(content_sizer, true);
	dialog.centre();
}

#[cfg(not(target_os = "windows"))]
fn find_and_select_dv(
	tree: DataViewTreeCtrl,
	parent: &DataViewItem,
	offset: i32,
	item_offsets: &HashMap<usize, i32>,
) -> bool {
	let count = tree.get_child_count(parent);
	for i in 0..count {
		let child = tree.get_nth_child(parent, i);
		if let Some(id_ptr) = child.get_id::<c_void>()
			&& item_offsets.get(&(id_ptr as usize)) == Some(&offset)
		{
			tree.select(&child);
			tree.ensure_visible(&child);
			return true;
		}
		if find_and_select_dv(tree, &child, offset, item_offsets) {
			return true;
		}
	}
	false
}

// ── Windows implementation (TreeCtrl) ─────────────────────────────────────────

#[cfg(target_os = "windows")]
fn show_toc_dialog_wx(parent: &Frame, toc_items: &[TocItem], current_offset: i32) -> Option<i32> {
	// TRANSLATORS: Title of the Table of Contents dialog
	let dialog_title = t("Table of Contents");
	let dialog = Dialog::builder(parent, &dialog_title).build();
	let selected_offset = Rc::new(Cell::new(-1));
	if largest_sibling_group(toc_items) > super::MAX_TREE_SIBLINGS {
		let mut entries = Vec::new();
		flatten_toc(toc_items, &mut entries);
		let list = build_toc_list(dialog, &entries, current_offset, &selected_offset);
		list.set_accessibility_label(&dialog_title);
		let resolve_selected: Rc<dyn Fn() -> Option<i32>> = {
			let offsets: Vec<i32> = entries.iter().map(|item| toc_offset(item)).collect();
			Rc::new(move || flat_list_selected_index(list).and_then(|index| offsets.get(index)).copied())
		};
		return run_toc_dialog(dialog, &list, &selected_offset, resolve_selected);
	}
	let (tree, _root) = build_toc_tree(dialog, toc_items, current_offset);
	bind_toc_selection(tree, Rc::clone(&selected_offset));
	bind_toc_activation(dialog, tree, Rc::clone(&selected_offset));
	bind_toc_search(tree);
	// OK confirms the tree's current selection at click time, as Enter does, rather than
	// trusting a cached offset: the entry nearest the reader is preselected while the tree
	// is being built, before the selection-changed listener is attached, so that listener
	// never records it and OK would otherwise claim nothing is selected.
	let resolve_selected: Rc<dyn Fn() -> Option<i32>> = {
		let tree_for_ok = tree;
		Rc::new(move || {
			let item = tree_for_ok.get_selection()?;
			let data = tree_for_ok.get_custom_data(&item)?;
			data.downcast_ref::<i32>().copied()
		})
	};
	run_toc_dialog(dialog, &tree, &selected_offset, resolve_selected)
}

#[cfg(target_os = "windows")]
fn run_toc_dialog<W: WxWidget>(
	dialog: Dialog,
	control: &W,
	selected_offset: &Rc<Cell<i32>>,
	resolve_selected: Rc<dyn Fn() -> Option<i32>>,
) -> Option<i32> {
	let (ok_button, cancel_button) = build_toc_buttons(dialog);
	bind_toc_ok(dialog, ok_button, Rc::clone(selected_offset), resolve_selected);
	bind_toc_layout(dialog, control, ok_button, cancel_button);
	control.set_focus();
	if dialog.show_modal() == ID_OK {
		let offset = selected_offset.get();
		if offset >= 0 { Some(offset) } else { None }
	} else {
		None
	}
}

#[cfg(target_os = "windows")]
fn build_toc_tree(dialog: Dialog, toc_items: &[TocItem], current_offset: i32) -> (TreeCtrl, TreeItemId) {
	let tree = TreeCtrl::builder(&dialog)
		.with_style(TreeCtrlStyle::Default | TreeCtrlStyle::HideRoot)
		.with_size(dialog.from_dip(Size::new(400, 500)))
		.build();
	let root = tree.add_root("Root", None, None).unwrap();
	populate_toc_tree(tree, &root, toc_items);
	if current_offset != -1 {
		find_and_select_item(tree, &root, current_offset);
	}
	(tree, root)
}

#[cfg(target_os = "windows")]
fn largest_sibling_group(items: &[TocItem]) -> usize {
	items.iter().map(|item| largest_sibling_group(&item.children)).fold(items.len(), usize::max)
}

/// Every entry of the table of contents in reading order, children after their parent.
#[cfg(target_os = "windows")]
fn flatten_toc<'a>(items: &'a [TocItem], entries: &mut Vec<&'a TocItem>) {
	for item in items {
		entries.push(item);
		flatten_toc(&item.children, entries);
	}
}

/// The table of contents as a virtual list, with the reader's current entry selected.
#[cfg(target_os = "windows")]
fn build_toc_list(
	dialog: Dialog,
	entries: &[&TocItem],
	current_offset: i32,
	selected_offset: &Rc<Cell<i32>>,
) -> ListCtrl {
	let labels: Vec<String> = entries.iter().map(|item| toc_label(item)).collect();
	let len = labels.len();
	let list = build_flat_list(dialog, Rc::new(RefCell::new(labels)));
	list.set_item_count(i64::try_from(len).unwrap_or(0));
	let offsets: Vec<i32> = entries.iter().map(|item| toc_offset(item)).collect();
	let current = offsets.iter().position(|&offset| offset == current_offset).unwrap_or(0);
	select_flat_row(list, i32::try_from(current).unwrap_or(0));
	let dialog_for_activate = dialog;
	let selected_for_activate = Rc::clone(selected_offset);
	list.on_item_activated(move |_| {
		if let Some(&offset) = flat_list_selected_index(list).and_then(|index| offsets.get(index)) {
			selected_for_activate.set(offset);
			dialog_for_activate.end_modal(ID_OK);
		}
	});
	list
}

#[cfg(target_os = "windows")]
fn bind_toc_selection(tree: TreeCtrl, selected_offset: Rc<Cell<i32>>) {
	let tree_for_sel = tree;
	tree.on_selection_changed(move |event| {
		if let Some(item) = event.get_item()
			&& let Some(data) = tree_for_sel.get_custom_data(&item)
			&& let Some(offset) = data.downcast_ref::<i32>()
		{
			selected_offset.set(*offset);
		}
	});
}

#[cfg(target_os = "windows")]
fn bind_toc_activation(dialog: Dialog, tree: TreeCtrl, selected_offset: Rc<Cell<i32>>) {
	let dialog_for_activate = dialog;
	let tree_for_activate = tree;
	tree.on_item_activated(move |event| {
		if let Some(item) = event.get_item()
			&& let Some(data) = tree_for_activate.get_custom_data(&item)
			&& let Some(offset) = data.downcast_ref::<i32>()
		{
			selected_offset.set(*offset);
			dialog_for_activate.end_modal(ID_OK);
		}
	});
}

#[cfg(target_os = "windows")]
fn bind_toc_search(tree: TreeCtrl) {
	// Prevent space from triggering item_activated (which our handler maps to OK).
	tree.bind_internal(EventType::KEY_DOWN, move |event| {
		if let Some(key) = event.get_key_code()
			&& key == KEY_SPACE
		{
			event.skip(false);
			return;
		}
		event.skip(true);
	});
}

#[cfg(target_os = "windows")]
fn bind_toc_layout<W: WxWidget>(dialog: Dialog, control: &W, ok_button: Button, cancel_button: Button) {
	let content_sizer = BoxSizer::builder(Orientation::Vertical).build();
	content_sizer.add(control, 1, SizerFlag::Expand | SizerFlag::All, super::DIALOG_PADDING);
	let button_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	button_sizer.add_stretch_spacer(1);
	button_sizer.add(&ok_button, 0, SizerFlag::Right, super::DIALOG_PADDING);
	button_sizer.add(&cancel_button, 0, SizerFlag::Right, super::DIALOG_PADDING);
	content_sizer.add_sizer(
		&button_sizer,
		0,
		SizerFlag::Expand | SizerFlag::Bottom | SizerFlag::Right,
		super::DIALOG_PADDING,
	);
	dialog.set_sizer_and_fit(content_sizer, true);
	dialog.centre();
}

#[cfg(target_os = "windows")]
fn populate_toc_tree(tree: TreeCtrl, parent: &TreeItemId, items: &[TocItem]) {
	for item in items {
		if let Some(id) = tree.append_item_with_data(parent, &toc_label(item), toc_offset(item), None, None)
			&& !item.children.is_empty()
		{
			populate_toc_tree(tree, &id, &item.children);
		}
	}
}

#[cfg(target_os = "windows")]
fn find_and_select_item(tree: TreeCtrl, parent: &TreeItemId, offset: i32) -> bool {
	if let Some((child, mut cookie)) = tree.get_first_child(parent) {
		let mut current_child = Some(child);
		while let Some(item) = current_child {
			if let Some(data) = tree.get_custom_data(&item)
				&& let Some(item_offset) = data.downcast_ref::<i32>()
				&& *item_offset == offset
			{
				tree.select_item(&item);
				tree.set_focused_item(&item);
				tree.ensure_visible(&item);
				return true;
			}
			if find_and_select_item(tree, &item, offset) {
				return true;
			}
			current_child = tree.get_next_child(parent, &mut cookie);
		}
	}
	false
}

// ── Shared helpers ─────────────────────────────────────────────────────────────

#[cfg(target_os = "windows")]
fn toc_label(item: &TocItem) -> String {
	// TRANSLATORS: Placeholder text shown in the table of contents tree when an entry has no title
	if item.name.is_empty() { t("Untitled") } else { item.name.clone() }
}

#[cfg(target_os = "windows")]
fn toc_offset(item: &TocItem) -> i32 {
	i32::try_from(item.offset).unwrap_or(i32::MAX)
}

fn build_toc_buttons(dialog: Dialog) -> (Button, Button) {
	// TRANSLATORS: Label for the confirmation button
	let ok_button = Button::builder(&dialog).with_label(&t("OK")).build();
	// TRANSLATORS: Label for the cancellation button
	let cancel_button = Button::builder(&dialog).with_id(ID_CANCEL).with_label(&t("Cancel")).build();
	(ok_button, cancel_button)
}

fn bind_toc_ok(
	dialog: Dialog,
	ok_button: Button,
	selected_offset: Rc<Cell<i32>>,
	resolve_selected: Rc<dyn Fn() -> Option<i32>>,
) {
	dialog.set_escape_id(ID_CANCEL);
	let dialog_for_ok = dialog;
	ok_button.on_click(move |_| {
		if let Some(offset) = resolve_selected() {
			selected_offset.set(offset);
			dialog_for_ok.end_modal(ID_OK);
		} else {
			MessageDialog::builder(
				&dialog_for_ok,
				// TRANSLATORS: Error message shown when the user attempts to confirm the Table of Contents dialog without having selected any section
				&t("Please select a section from the table of contents."),
				// TRANSLATORS: Title of the error dialog shown when the user attempts to confirm the Table of Contents dialog without having selected any section
				&t("No Selection"),
			)
			.with_style(MessageDialogStyle::OK | MessageDialogStyle::IconInformation | MessageDialogStyle::Centre)
			.build()
			.show_modal();
		}
	});
}
