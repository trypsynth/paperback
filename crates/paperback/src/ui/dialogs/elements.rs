use std::{cell::Cell, rc::Rc};

#[cfg(target_os = "linux")]
use gtk::{Dialog, ListBox as GtkListBox, PolicyType, ResponseType, Widget, Window, glib::Propagation, prelude::*};
use paperback_core::session::DocumentSession;
use patois::t;
#[cfg(target_os = "linux")]
use wxdragon::prelude::Frame;
#[cfg(not(target_os = "linux"))]
use wxdragon::prelude::*;

#[cfg(target_os = "linux")]
use super::accessible_tree::{self, AccessibleTree};

#[cfg(not(target_os = "linux"))]
const DIALOG_PADDING: i32 = 10;

pub fn show_elements_dialog(parent: &Frame, session: &DocumentSession, current_pos: i64) -> Option<i64> {
	#[cfg(target_os = "linux")]
	return show_elements_dialog_gtk(parent, session, current_pos);
	#[cfg(not(target_os = "linux"))]
	return show_elements_dialog_wx(parent, session, current_pos);
}

#[cfg(target_os = "linux")]
fn show_elements_dialog_gtk(parent: &Frame, session: &DocumentSession, current_pos: i64) -> Option<i64> {
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
	let view_combo = gtk::ComboBoxText::new();
	view_combo.append_text(&t("Headings"));
	view_combo.append_text(&t("Links"));
	view_combo.set_active(Some(0));
	let mut headings_tree = AccessibleTree::new();
	let tree_data = session.heading_tree(current_pos);
	populate_headings(&mut headings_tree, &tree_data.items);
	let headings_select_idx = if tree_data.closest_index >= 0 {
		tree_data
			.items
			.get(tree_data.closest_index as usize)
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
	let content = dialog.content_area();
	content.pack_start(&view_combo, false, false, 5);
	content.pack_start(&headings_scrolled, true, true, 0);
	content.pack_start(&links_scrolled, true, true, 0);
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
	unsafe {
		dialog.destroy();
	}
	if response == ResponseType::Ok {
		let offset = selected_offset.get();
		if offset >= 0 { Some(offset) } else { None }
	} else {
		None
	}
}

#[cfg(target_os = "linux")]
const GDK_KEY_TAB: u32 = 0xff09;
#[cfg(target_os = "linux")]
const GDK_KEY_ISO_LEFT_TAB: u32 = 0xfe20;

#[cfg(target_os = "linux")]
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

#[cfg(target_os = "linux")]
fn populate_headings(tree: &mut AccessibleTree, items: &[paperback_core::types::HeadingTreeItem]) {
	let mut depths = Vec::with_capacity(items.len());
	let mut child_counts = vec![0usize; items.len()];
	for item in items {
		let depth =
			if item.parent_index >= 0 { depths.get(item.parent_index as usize).map_or(0, |&d: &i32| d + 1) } else { 0 };
		depths.push(depth);
		if item.parent_index >= 0 {
			if let Ok(pi) = usize::try_from(item.parent_index) {
				child_counts[pi] += 1;
			}
		}
	}
	for (i, item) in items.iter().enumerate() {
		let name = if item.text.is_empty() { t("Untitled") } else { item.text.clone() };
		let offset = i64::try_from(item.offset).unwrap_or(i64::MAX);
		tree.add_item(&name, offset, depths[i], child_counts[i] > 0);
	}
}

#[cfg(not(target_os = "linux"))]
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
	if dialog.show_modal() == wxdragon::id::ID_OK {
		let offset = selected_offset.get();
		if offset >= 0 { Some(offset) } else { None }
	} else {
		None
	}
}

#[cfg(not(target_os = "linux"))]
struct ElementsDialogUi {
	content_sizer: BoxSizer,
	view_choice: Choice,
	headings_tree: TreeCtrl,
	links_list: ListBox,
}

#[cfg(not(target_os = "linux"))]
fn build_elements_dialog_ui(dialog: Dialog) -> ElementsDialogUi {
	let content_sizer = BoxSizer::builder(Orientation::Vertical).build();
	let choice_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	let choice_label = StaticText::builder(&dialog).with_label(&t("&View:")).build();
	let view_choice = Choice::builder(&dialog).build();
	view_choice.append(&t("Headings"));
	view_choice.append(&t("Links"));
	view_choice.set_selection(0);
	choice_sizer.add(&choice_label, 0, SizerFlag::AlignCenterVertical | SizerFlag::Right, DIALOG_PADDING);
	choice_sizer.add(&view_choice, 1, SizerFlag::Expand, 0);
	content_sizer.add_sizer(&choice_sizer, 0, SizerFlag::Expand | SizerFlag::All, DIALOG_PADDING);
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
		DIALOG_PADDING,
	);
	let links_sizer = BoxSizer::builder(Orientation::Vertical).build();
	let links_list = ListBox::builder(&dialog).build();
	links_sizer.add(&links_list, 1, SizerFlag::Expand, 0);
	content_sizer.add_sizer(
		&links_sizer,
		1,
		SizerFlag::Expand | SizerFlag::Left | SizerFlag::Right | SizerFlag::Bottom,
		DIALOG_PADDING,
	);
	links_list.show(false);
	ElementsDialogUi { content_sizer, view_choice, headings_tree, links_list }
}

#[cfg(not(target_os = "linux"))]
fn populate_elements_dialog(
	session: &DocumentSession,
	current_pos: i64,
	headings_tree: TreeCtrl,
	links_list: ListBox,
) -> (Rc<Cell<i64>>, Rc<Vec<i64>>) {
	let selected_offset = Rc::new(Cell::new(-1i64));
	let root = headings_tree.add_root(&t("Root"), None, None).unwrap();
	let tree_data = session.heading_tree(current_pos);
	let mut item_ids: Vec<wxdragon::widgets::treectrl::TreeItemId> = Vec::new();
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
		if let Ok(index) = usize::try_from(tree_data.closest_index) {
			if let Some(item) = item_ids.get(index) {
				headings_tree.select_item(item);
				headings_tree.ensure_visible(item);
			}
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

#[cfg(not(target_os = "linux"))]
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

#[cfg(not(target_os = "linux"))]
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
		if let Some(item) = event.get_item() {
			if let Some(data) = tree_for_activate.get_custom_data(&item) {
				if let Some(offset) = data.downcast_ref::<i64>() {
					selected_offset_for_tree.set(*offset);
					dialog_for_tree.end_modal(wxdragon::id::ID_OK);
				}
			}
		}
	});
	let selected_offset_for_list = Rc::clone(selected_offset);
	let offsets_for_list = Rc::clone(link_offsets);
	let dialog_for_list = dialog;
	links_list.on_item_double_clicked(move |event| {
		let selection = event.get_selection().unwrap_or(-1);
		if selection >= 0 {
			if let Ok(index) = usize::try_from(selection) {
				if let Some(offset) = offsets_for_list.get(index) {
					selected_offset_for_list.set(*offset);
					dialog_for_list.end_modal(wxdragon::id::ID_OK);
				}
			}
		}
	});
}

#[cfg(not(target_os = "linux"))]
fn build_elements_buttons(dialog: Dialog) -> (Button, Button) {
	let ok_button = Button::builder(&dialog).with_id(wxdragon::id::ID_OK).with_label(&t("OK")).build();
	let cancel_button = Button::builder(&dialog).with_id(wxdragon::id::ID_CANCEL).with_label(&t("Cancel")).build();
	dialog.set_escape_id(wxdragon::id::ID_CANCEL);
	ok_button.set_default();
	(ok_button, cancel_button)
}

#[cfg(not(target_os = "linux"))]
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
			if let Some(item) = headings_tree.get_selection() {
				if let Some(data) = headings_tree.get_custom_data(&item) {
					if let Some(offset) = data.downcast_ref::<i64>() {
						selected_offset_for_ok.set(*offset);
						dialog_for_ok.end_modal(wxdragon::id::ID_OK);
					}
				}
			}
		} else if let Some(idx) = links_list.get_selection() {
			if let Ok(index) = usize::try_from(idx) {
				if let Some(offset) = offsets_for_ok.get(index) {
					selected_offset_for_ok.set(*offset);
					dialog_for_ok.end_modal(wxdragon::id::ID_OK);
				}
			}
		}
	});
}

#[cfg(not(target_os = "linux"))]
fn finalize_elements_layout(dialog: Dialog, content_sizer: BoxSizer, ok_button: Button, cancel_button: Button) {
	let button_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	button_sizer.add_stretch_spacer(1);
	button_sizer.add(&ok_button, 0, SizerFlag::All, DIALOG_PADDING);
	button_sizer.add(&cancel_button, 0, SizerFlag::All, DIALOG_PADDING);
	content_sizer.add_sizer(&button_sizer, 0, SizerFlag::Expand, 0);
	dialog.set_sizer_and_fit(content_sizer, true);
	dialog.centre();
}
