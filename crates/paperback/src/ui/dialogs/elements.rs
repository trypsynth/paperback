use std::{cell::Cell, rc::Rc};
#[cfg(not(target_os = "windows"))]
use std::{collections::HashMap, ffi::c_void};

use paperback_core::session::DocumentSession;
use patois::t;
use wxdragon::prelude::*;

pub fn show_elements_dialog(parent: &Frame, session: &DocumentSession, current_pos: i64) -> Option<i64> {
	#[cfg(not(target_os = "windows"))]
	return show_elements_dialog_dv(parent, session, current_pos);
	#[cfg(target_os = "windows")]
	return show_elements_dialog_wx(parent, session, current_pos);
}

// ── DataViewTreeCtrl implementation (Linux + macOS) ───────────────────────────

#[cfg(not(target_os = "windows"))]
struct ElementsDialogUiDv {
	content_sizer: BoxSizer,
	view_choice: Choice,
	headings_tree: DataViewTreeCtrl,
	links_list: ListBox,
}

#[cfg(not(target_os = "windows"))]
fn show_elements_dialog_dv(parent: &Frame, session: &DocumentSession, current_pos: i64) -> Option<i64> {
	// TRANSLATORS: Title of the Elements dialog
	let dialog = Dialog::builder(parent, &t("Elements")).build();
	let ElementsDialogUiDv { content_sizer, view_choice, headings_tree, links_list } =
		build_elements_dialog_ui_dv(dialog);
	let (selected_offset, item_offsets, link_offsets) =
		populate_elements_dialog_dv(session, current_pos, headings_tree, links_list);
	let item_offsets = Rc::new(item_offsets);
	bind_elements_view_toggle_dv(view_choice, headings_tree, links_list, dialog);
	bind_elements_activation_dv(dialog, headings_tree, links_list, &item_offsets, &link_offsets, &selected_offset);
	let (ok_button, cancel_button) = build_elements_buttons(dialog);
	bind_elements_ok_action_dv(
		dialog,
		view_choice,
		headings_tree,
		links_list,
		&item_offsets,
		&link_offsets,
		&selected_offset,
		ok_button,
	);
	finalize_elements_layout(dialog, content_sizer, ok_button, cancel_button);
	if view_choice.get_selection().unwrap_or(0) == 0 {
		headings_tree.set_focus();
	} else {
		links_list.set_focus();
	}
	if dialog.show_modal() == wxdragon::id::ID_OK {
		let offset = selected_offset.get();
		if offset >= 0 { Some(offset) } else { None }
	} else {
		None
	}
}

#[cfg(not(target_os = "windows"))]
fn build_elements_dialog_ui_dv(dialog: Dialog) -> ElementsDialogUiDv {
	let content_sizer = BoxSizer::builder(Orientation::Vertical).build();
	let choice_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	// TRANSLATORS: Label for the view selection dropdown in the Elements dialog
	let choice_label_text = t("&View:");
	let choice_label = StaticText::builder(&dialog).with_label(&choice_label_text).build();
	let view_choice = Choice::builder(&dialog).build();
	// TRANSLATORS: Choice option in the view dropdown to show headings list
	view_choice.append(&t("Headings"));
	// TRANSLATORS: Choice option in the view dropdown to show links list
	view_choice.append(&t("Links"));
	view_choice.set_selection(0);
	#[cfg(target_os = "macos")]
	view_choice.set_accessibility_label(choice_label_text.replace('&', "").trim_end_matches(':').trim());
	choice_sizer.add(&choice_label, 0, SizerFlag::AlignCenterVertical | SizerFlag::Right, super::DIALOG_PADDING);
	choice_sizer.add(&view_choice, 1, SizerFlag::Expand, 0);
	content_sizer.add_sizer(&choice_sizer, 0, SizerFlag::Expand | SizerFlag::All, super::DIALOG_PADDING);
	let headings_tree = DataViewTreeCtrl::builder(&dialog).with_size(Size::new(400, 500)).build();
	content_sizer.add(
		&headings_tree,
		1,
		SizerFlag::Expand | SizerFlag::Left | SizerFlag::Right | SizerFlag::Bottom,
		super::DIALOG_PADDING,
	);
	let links_list = ListBox::builder(&dialog).build();
	content_sizer.add(
		&links_list,
		1,
		SizerFlag::Expand | SizerFlag::Left | SizerFlag::Right | SizerFlag::Bottom,
		super::DIALOG_PADDING,
	);
	links_list.show(false);
	ElementsDialogUiDv { content_sizer, view_choice, headings_tree, links_list }
}

#[cfg(not(target_os = "windows"))]
fn populate_elements_dialog_dv(
	session: &DocumentSession,
	current_pos: i64,
	headings_tree: DataViewTreeCtrl,
	links_list: ListBox,
) -> (Rc<Cell<i64>>, HashMap<usize, i64>, Rc<Vec<i64>>) {
	let selected_offset = Rc::new(Cell::new(-1i64));
	let mut item_offsets: HashMap<usize, i64> = HashMap::new();
	let tree_data = session.heading_tree(current_pos);
	// Precompute which items have children so we can use append_container vs append_item.
	let has_children_vec: Vec<bool> = (0..tree_data.items.len())
		.map(|i| {
			tree_data.items.iter().any(|it| it.parent_index >= 0 && usize::try_from(it.parent_index).ok() == Some(i))
		})
		.collect();
	let root = DataViewItem::default();
	let mut item_ids: Vec<DataViewItem> = Vec::new();
	for (current_idx, item) in tree_data.items.iter().enumerate() {
		let parent: &DataViewItem = if item.parent_index >= 0 {
			usize::try_from(item.parent_index).ok().and_then(|idx| item_ids.get(idx)).unwrap_or(&root)
		} else {
			&root
		};
		// TRANSLATORS: Placeholder text shown in the elements list when a document element has no text content
		let display_text = if item.text.is_empty() { t("Untitled") } else { item.text.clone() };
		let offset = i64::try_from(item.offset).unwrap_or(i64::MAX);
		let node = if has_children_vec[current_idx] {
			headings_tree.append_container(parent, &display_text, -1, -1)
		} else {
			headings_tree.append_item(parent, &display_text, -1)
		};
		if let Some(id_ptr) = node.get_id::<c_void>() {
			item_offsets.insert(id_ptr as usize, offset);
		}
		item_ids.push(node);
	}
	let select_idx = if tree_data.closest_index >= 0 {
		usize::try_from(tree_data.closest_index).ok()
	} else if !item_ids.is_empty() {
		Some(0)
	} else {
		None
	};
	if let Some(idx) = select_idx {
		if let Some(item) = item_ids.get(idx) {
			headings_tree.select(item);
			headings_tree.ensure_visible(item);
		}
	}
	let link_data = session.link_list(current_pos);
	let mut link_offsets = Vec::new();
	for item in link_data.items {
		links_list.append(&item.text);
		link_offsets.push(i64::try_from(item.offset).unwrap_or(i64::MAX));
	}
	if !link_offsets.is_empty() {
		let idx = if link_data.closest_index >= 0 { link_data.closest_index } else { 0 };
		if let Ok(idx_u32) = u32::try_from(idx) {
			links_list.set_selection(idx_u32, true);
		}
	}
	(selected_offset, item_offsets, Rc::new(link_offsets))
}

#[cfg(not(target_os = "windows"))]
fn bind_elements_view_toggle_dv(
	view_choice: Choice,
	headings_tree: DataViewTreeCtrl,
	links_list: ListBox,
	dialog: Dialog,
) {
	view_choice.on_selection_changed(move |_| {
		let selection = view_choice.get_selection().unwrap_or(0);
		if selection == 0 {
			headings_tree.show(true);
			links_list.show(false);
			headings_tree.set_focus();
		} else {
			headings_tree.show(false);
			links_list.show(true);
			links_list.set_focus();
		}
		dialog.layout();
	});
}

#[cfg(not(target_os = "windows"))]
fn bind_elements_activation_dv(
	dialog: Dialog,
	headings_tree: DataViewTreeCtrl,
	links_list: ListBox,
	item_offsets: &Rc<HashMap<usize, i64>>,
	link_offsets: &Rc<Vec<i64>>,
	selected_offset: &Rc<Cell<i64>>,
) {
	let offsets_for_tree = Rc::clone(item_offsets);
	let selected_for_tree = Rc::clone(selected_offset);
	let dialog_for_tree = dialog;
	headings_tree.on_item_activated(move |event| {
		if let Some(item) = event.get_item() {
			if let Some(id_ptr) = item.get_id::<c_void>() {
				if let Some(&offset) = offsets_for_tree.get(&(id_ptr as usize)) {
					selected_for_tree.set(offset);
					dialog_for_tree.end_modal(wxdragon::id::ID_OK);
				}
			}
		}
	});
	let offsets_for_list = Rc::clone(link_offsets);
	let selected_for_list = Rc::clone(selected_offset);
	let dialog_for_list = dialog;
	links_list.on_item_double_clicked(move |event| {
		let selection = event.get_selection().unwrap_or(-1);
		if selection >= 0 {
			if let Ok(index) = usize::try_from(selection) {
				if let Some(offset) = offsets_for_list.get(index) {
					selected_for_list.set(*offset);
					dialog_for_list.end_modal(wxdragon::id::ID_OK);
				}
			}
		}
	});
}

#[cfg(not(target_os = "windows"))]
fn bind_elements_ok_action_dv(
	dialog: Dialog,
	view_choice: Choice,
	headings_tree: DataViewTreeCtrl,
	links_list: ListBox,
	item_offsets: &Rc<HashMap<usize, i64>>,
	link_offsets: &Rc<Vec<i64>>,
	selected_offset: &Rc<Cell<i64>>,
	ok_button: Button,
) {
	let offsets_for_ok = Rc::clone(item_offsets);
	let link_offsets_for_ok = Rc::clone(link_offsets);
	let selected_for_ok = Rc::clone(selected_offset);
	let dialog_for_ok = dialog;
	ok_button.on_click(move |_| {
		let selection = view_choice.get_selection().unwrap_or(0);
		if selection == 0 {
			if let Some(item) = headings_tree.get_selection() {
				if let Some(id_ptr) = item.get_id::<c_void>() {
					if let Some(&offset) = offsets_for_ok.get(&(id_ptr as usize)) {
						selected_for_ok.set(offset);
						dialog_for_ok.end_modal(wxdragon::id::ID_OK);
					}
				}
			}
		} else if let Some(idx) = links_list.get_selection() {
			if let Ok(index) = usize::try_from(idx) {
				if let Some(offset) = link_offsets_for_ok.get(index) {
					selected_for_ok.set(*offset);
					dialog_for_ok.end_modal(wxdragon::id::ID_OK);
				}
			}
		}
	});
}

// ── Windows implementation (TreeCtrl) ─────────────────────────────────────────

#[cfg(target_os = "windows")]
struct ElementsDialogUi {
	content_sizer: BoxSizer,
	view_choice: Choice,
	headings_tree: TreeCtrl,
	links_list: ListBox,
}

#[cfg(target_os = "windows")]
fn show_elements_dialog_wx(parent: &Frame, session: &DocumentSession, current_pos: i64) -> Option<i64> {
	let dialog = Dialog::builder(parent, &t("Elements")).build();
	let ElementsDialogUi { content_sizer, view_choice, headings_tree, links_list } = build_elements_dialog_ui(dialog);
	let (selected_offset, link_offsets) = populate_elements_dialog(session, current_pos, headings_tree, links_list);
	bind_elements_view_toggle(view_choice, headings_tree, links_list, dialog);
	bind_elements_activation(dialog, headings_tree, links_list, &selected_offset, &link_offsets);
	let (ok_button, cancel_button) = build_elements_buttons(dialog);
	bind_elements_ok_action(dialog, view_choice, headings_tree, links_list, &link_offsets, &selected_offset, ok_button);
	finalize_elements_layout(dialog, content_sizer, ok_button, cancel_button);
	if view_choice.get_selection().unwrap_or(0) == 0 {
		headings_tree.set_focus();
	} else {
		links_list.set_focus();
	}
	if dialog.show_modal() == ID_OK {
		let offset = selected_offset.get();
		if offset >= 0 { Some(offset) } else { None }
	} else {
		None
	}
}

#[cfg(target_os = "windows")]
fn build_elements_dialog_ui(dialog: Dialog) -> ElementsDialogUi {
	let content_sizer = BoxSizer::builder(Orientation::Vertical).build();
	let choice_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	// TRANSLATORS: Label for the view selection dropdown in the Elements dialog
	let choice_label_text = t("&View:");
	let choice_label = StaticText::builder(&dialog).with_label(&choice_label_text).build();
	let view_choice = Choice::builder(&dialog).build();
	// TRANSLATORS: Choice option in the view dropdown to show headings list
	view_choice.append(&t("Headings"));
	// TRANSLATORS: Choice option in the view dropdown to show links list
	view_choice.append(&t("Links"));
	view_choice.set_selection(0);
	#[cfg(target_os = "macos")]
	view_choice.set_accessibility_label(choice_label_text.replace('&', "").trim_end_matches(':').trim());
	choice_sizer.add(&choice_label, 0, SizerFlag::AlignCenterVertical | SizerFlag::Right, super::DIALOG_PADDING);
	choice_sizer.add(&view_choice, 1, SizerFlag::Expand, 0);
	content_sizer.add_sizer(&choice_sizer, 0, SizerFlag::Expand | SizerFlag::All, super::DIALOG_PADDING);
	let headings_sizer = BoxSizer::builder(Orientation::Vertical).build();
	let headings_tree = TreeCtrl::builder(&dialog)
		.with_style(TreeCtrlStyle::Default | TreeCtrlStyle::HideRoot)
		.with_size(Size::new(400, 500))
		.build();
	headings_sizer.add(&headings_tree, 1, SizerFlag::Expand, 0);
	content_sizer.add_sizer(
		&headings_sizer,
		1,
		SizerFlag::Expand | SizerFlag::Left | SizerFlag::Right | SizerFlag::Bottom,
		super::DIALOG_PADDING,
	);
	let links_sizer = BoxSizer::builder(Orientation::Vertical).build();
	let links_list = ListBox::builder(&dialog).build();
	links_sizer.add(&links_list, 1, SizerFlag::Expand, 0);
	content_sizer.add_sizer(
		&links_sizer,
		1,
		SizerFlag::Expand | SizerFlag::Left | SizerFlag::Right | SizerFlag::Bottom,
		super::DIALOG_PADDING,
	);
	links_list.show(false);
	ElementsDialogUi { content_sizer, view_choice, headings_tree, links_list }
}

#[cfg(target_os = "windows")]
fn populate_elements_dialog(
	session: &DocumentSession,
	current_pos: i64,
	headings_tree: TreeCtrl,
	links_list: ListBox,
) -> (Rc<Cell<i64>>, Rc<Vec<i64>>) {
	let selected_offset = Rc::new(Cell::new(-1i64));
	let root = headings_tree.add_root("Root", None, None).unwrap();
	let tree_data = session.heading_tree(current_pos);
	let mut item_ids: Vec<TreeItemId> = Vec::new();
	if !tree_data.items.is_empty() {
		item_ids.reserve(tree_data.items.len());
	}
	for item in &tree_data.items {
		let parent_id = if item.parent_index >= 0 {
			usize::try_from(item.parent_index)
				.ok()
				.and_then(|idx| item_ids.get(idx).cloned())
				.unwrap_or_else(|| root.clone())
		} else {
			root.clone()
		};
		let display_text = if item.text.is_empty() { t("Untitled") } else { item.text.clone() };
		let offset = i64::try_from(item.offset).unwrap_or(i64::MAX);
		if let Some(id) = headings_tree.append_item_with_data(&parent_id, &display_text, offset, None, None) {
			item_ids.push(id);
		} else if let Some(root_child) = headings_tree.append_item_with_data(&root, &display_text, offset, None, None) {
			item_ids.push(root_child);
		}
	}
	headings_tree.expand_all();
	if tree_data.closest_index >= 0 {
		if let Ok(index) = usize::try_from(tree_data.closest_index)
			&& let Some(item) = item_ids.get(index)
		{
			headings_tree.select_item(item);
			headings_tree.ensure_visible(item);
		}
	} else if let Some((first_child, _)) = headings_tree.get_first_child(&root) {
		headings_tree.select_item(&first_child);
		headings_tree.ensure_visible(&first_child);
	}
	let link_data = session.link_list(current_pos);
	let mut link_offsets = Vec::new();
	for item in link_data.items {
		links_list.append(&item.text);
		link_offsets.push(i64::try_from(item.offset).unwrap_or(i64::MAX));
	}
	if !link_offsets.is_empty() {
		let idx = if link_data.closest_index >= 0 { link_data.closest_index } else { 0 };
		if let Ok(idx_u32) = u32::try_from(idx) {
			links_list.set_selection(idx_u32, true);
		}
	}
	(selected_offset, Rc::new(link_offsets))
}

#[cfg(target_os = "windows")]
fn bind_elements_view_toggle(view_choice: Choice, headings_tree: TreeCtrl, links_list: ListBox, dialog: Dialog) {
	let headings_tree_for_choice = headings_tree;
	let links_list_for_choice = links_list;
	let dialog_for_layout = dialog;
	view_choice.on_selection_changed(move |_| {
		let selection = view_choice.get_selection().unwrap_or(0);
		if selection == 0 {
			headings_tree_for_choice.show(true);
			links_list_for_choice.show(false);
			headings_tree_for_choice.set_focus();
		} else {
			headings_tree_for_choice.show(false);
			links_list_for_choice.show(true);
			links_list_for_choice.set_focus();
		}
		dialog_for_layout.layout();
	});
}

#[cfg(target_os = "windows")]
fn bind_elements_activation(
	dialog: Dialog,
	headings_tree: TreeCtrl,
	links_list: ListBox,
	selected_offset: &Rc<Cell<i64>>,
	link_offsets: &Rc<Vec<i64>>,
) {
	let selected_offset_for_tree = Rc::clone(selected_offset);
	let tree_for_activate = headings_tree;
	let dialog_for_tree = dialog;
	headings_tree.on_item_activated(move |event| {
		if let Some(item) = event.get_item()
			&& let Some(data) = tree_for_activate.get_custom_data(&item)
			&& let Some(offset) = data.downcast_ref::<i64>()
		{
			selected_offset_for_tree.set(*offset);
			dialog_for_tree.end_modal(ID_OK);
		}
	});
	let selected_offset_for_list = Rc::clone(selected_offset);
	let offsets_for_list = Rc::clone(link_offsets);
	let dialog_for_list = dialog;
	links_list.on_item_double_clicked(move |event| {
		let selection = event.get_selection().unwrap_or(-1);
		if selection >= 0
			&& let Ok(index) = usize::try_from(selection)
			&& let Some(offset) = offsets_for_list.get(index)
		{
			selected_offset_for_list.set(*offset);
			dialog_for_list.end_modal(ID_OK);
		}
	});
}

#[cfg(target_os = "windows")]
fn bind_elements_ok_action(
	dialog: Dialog,
	view_choice: Choice,
	headings_tree: TreeCtrl,
	links_list: ListBox,
	link_offsets: &Rc<Vec<i64>>,
	selected_offset: &Rc<Cell<i64>>,
	ok_button: Button,
) {
	let offsets_for_ok = Rc::clone(link_offsets);
	let selected_offset_for_ok = Rc::clone(selected_offset);
	let dialog_for_ok = dialog;
	ok_button.on_click(move |_| {
		let selection = view_choice.get_selection().unwrap_or(0);
		if selection == 0 {
			if let Some(item) = headings_tree.get_selection()
				&& let Some(data) = headings_tree.get_custom_data(&item)
				&& let Some(offset) = data.downcast_ref::<i64>()
			{
				selected_offset_for_ok.set(*offset);
				dialog_for_ok.end_modal(ID_OK);
			}
		} else if let Some(idx) = links_list.get_selection()
			&& let Ok(index) = usize::try_from(idx)
			&& let Some(offset) = offsets_for_ok.get(index)
		{
			selected_offset_for_ok.set(*offset);
			dialog_for_ok.end_modal(ID_OK);
		}
	});
}

// ── Shared helpers ─────────────────────────────────────────────────────────────

fn build_elements_buttons(dialog: Dialog) -> (Button, Button) {
	// TRANSLATORS: Label for the confirmation button
	let ok_button = Button::builder(&dialog).with_id(ID_OK).with_label(&t("OK")).build();
	// TRANSLATORS: Label for the cancellation button
	let cancel_button = Button::builder(&dialog).with_id(ID_CANCEL).with_label(&t("Cancel")).build();
	dialog.set_escape_id(ID_CANCEL);
	ok_button.set_default();
	(ok_button, cancel_button)
}

fn finalize_elements_layout(dialog: Dialog, content_sizer: BoxSizer, ok_button: Button, cancel_button: Button) {
	let button_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	button_sizer.add_stretch_spacer(1);
	button_sizer.add(&ok_button, 0, SizerFlag::All, super::DIALOG_PADDING);
	button_sizer.add(&cancel_button, 0, SizerFlag::All, super::DIALOG_PADDING);
	content_sizer.add_sizer(&button_sizer, 0, SizerFlag::Expand, 0);
	dialog.set_sizer_and_fit(content_sizer, true);
	dialog.centre();
}
