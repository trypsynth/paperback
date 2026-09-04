use std::{cell::Cell, rc::Rc};
#[cfg(not(target_os = "windows"))]
use std::{collections::HashMap, ffi::c_void};

use paperback_core::session::DocumentSession;
use patois::t;
use wx_utils::dpi;
use wxdragon::prelude::*;

/// The view choice indices for [`show_elements_dialog`]. Headings is a tree; every other
/// view (Links, Pages, ...) is a flat list shown in the same list pane.
const VIEW_HEADINGS: u32 = 0;
const VIEW_LINKS: u32 = 1;
const VIEW_PAGES: u32 = 2;

/// Which view of the dialog a jump came from, so the caller can announce it appropriately: a
/// page row announces like page navigation, while a heading or link reads the line it lands on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElementsKind {
	Heading,
	Link,
	Page,
}

impl ElementsKind {
	/// Maps a `VIEW_*` choice index to the kind of element it lists.
	const fn from_view(selection: u32) -> Self {
		match selection {
			VIEW_HEADINGS => Self::Heading,
			VIEW_PAGES => Self::Page,
			// Any other flat content view reads as a link until it needs its own announcement.
			_ => Self::Link,
		}
	}
}

pub fn show_elements_dialog(
	parent: &Frame,
	session: &DocumentSession,
	current_pos: i64,
) -> Option<(i64, ElementsKind)> {
	#[cfg(not(target_os = "windows"))]
	return show_elements_dialog_dv(parent, session, current_pos);
	#[cfg(target_os = "windows")]
	return show_elements_dialog_wx(parent, session, current_pos);
}

// ── Shared helpers (both platform implementations use these) ───────────────────

/// A page entry for the Pages view: the page's marker offset (where a jump lands) and the
/// label to show, mirroring what page navigation announces for the page.
struct PageEntry {
	offset: i64,
	label: String,
}

/// One entry per page, labelled "Page N: <first content line>" — the same line that page
/// navigation (p / Shift+P) announces — or just "Page N" when the page has no readable line.
fn page_entries(session: &DocumentSession) -> Vec<PageEntry> {
	(0..session.page_count())
		.map(|index| {
			let page = i32::try_from(index).unwrap_or(0) + 1;
			let offset = session.page_offset(page);
			let content = session.first_content_line_after(offset);
			PageEntry { offset, label: page_label(page, &content) }
		})
		.collect()
}

fn page_label(page: i32, content: &str) -> String {
	let content = content.trim();
	let page_text = page.to_string();
	if content.is_empty() {
		// TRANSLATORS: A page in the Elements list with no readable first line; %d is the page number
		t("Page %d").replacen("%d", &page_text, 1)
	} else {
		// TRANSLATORS: A page in the Elements list; %d is the page number, %s is the page's first line of text
		t("Page %d: %s").replacen("%d", &page_text, 1).replacen("%s", content, 1)
	}
}

/// The rows for one flat (non-headings) view: what each row shows, the document offset it
/// jumps to, and which row is nearest the current position. The Links and Pages views each
/// have one; the shared list pane is repopulated from whichever is selected.
struct FlatView {
	entries: Vec<(String, i64)>,
	closest: Option<u32>,
}

fn link_flat_view(session: &DocumentSession, position: i64) -> FlatView {
	let data = session.link_list(position);
	let entries =
		data.items.iter().map(|item| (item.text.clone(), i64::try_from(item.offset).unwrap_or(i64::MAX))).collect();
	let closest = if data.closest_index >= 0 { u32::try_from(data.closest_index).ok() } else { None };
	FlatView { entries, closest }
}

fn page_flat_view(session: &DocumentSession, position: i64) -> FlatView {
	let entries = page_entries(session).iter().map(|page| (page.label.clone(), page.offset)).collect();
	let closest = {
		let page = session.current_page(position);
		if page >= 1 { u32::try_from(page - 1).ok() } else { None }
	};
	FlatView { entries, closest }
}

/// Replaces `list`'s rows with `view`'s, selecting the row nearest the current position.
fn fill_flat_list(list: ListBox, view: &FlatView) {
	list.clear();
	for (label, _) in &view.entries {
		list.append(label);
	}
	if let Some(index) = view.closest {
		list.set_selection(index, true);
	}
}

/// The offset of the row selected in the flat list pane for `view`, if any.
fn flat_selected_offset(view: u32, list: ListBox, links: &FlatView, pages: &FlatView) -> Option<i64> {
	let entries = match view {
		VIEW_LINKS => &links.entries,
		VIEW_PAGES => &pages.entries,
		_ => return None,
	};
	list.get_selection()
		.and_then(|index| usize::try_from(index).ok())
		.and_then(|index| entries.get(index))
		.map(|(_, offset)| *offset)
}

// ── DataViewTreeCtrl implementation (Linux + macOS) ───────────────────────────

#[cfg(not(target_os = "windows"))]
struct ElementsDialogUiDv {
	content_sizer: BoxSizer,
	view_choice: Choice,
	headings_tree: DataViewTreeCtrl,
	// The flat-list pane, shared by every non-headings view (Links, Pages, ...): its
	// contents are swapped in when the view changes.
	content_list: ListBox,
}

#[cfg(not(target_os = "windows"))]
fn show_elements_dialog_dv(parent: &Frame, session: &DocumentSession, current_pos: i64) -> Option<(i64, ElementsKind)> {
	// TRANSLATORS: Title of the Elements dialog
	let dialog = Dialog::builder(parent, &t("Elements")).build();
	let ElementsDialogUiDv { content_sizer, view_choice, headings_tree, content_list } =
		build_elements_dialog_ui_dv(dialog);
	let (selected_offset, item_offsets) = populate_elements_dialog_dv(session, current_pos, headings_tree);
	let item_offsets = Rc::new(item_offsets);
	let links = Rc::new(link_flat_view(session, current_pos));
	let pages = Rc::new(page_flat_view(session, current_pos));
	bind_elements_view_toggle_dv(view_choice, headings_tree, content_list, dialog, &links, &pages);
	bind_elements_activation_dv(
		dialog,
		view_choice,
		headings_tree,
		content_list,
		&item_offsets,
		&links,
		&pages,
		&selected_offset,
	);
	let (ok_button, cancel_button) = build_elements_buttons(dialog);
	bind_elements_ok_action_dv(
		dialog,
		view_choice,
		headings_tree,
		content_list,
		&item_offsets,
		&links,
		&pages,
		&selected_offset,
		ok_button,
	);
	finalize_elements_layout(dialog, content_sizer, ok_button, cancel_button);
	// The dialog opens on Headings (the choice is set to index 0 when it is built), so the
	// tree is the visible pane.
	headings_tree.set_focus();
	if dialog.show_modal() == wxdragon::id::ID_OK {
		let offset = selected_offset.get();
		let kind = ElementsKind::from_view(view_choice.get_selection().unwrap_or(VIEW_HEADINGS));
		if offset >= 0 { Some((offset, kind)) } else { None }
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
	// TRANSLATORS: Choice option in the view dropdown to show the list of pages
	view_choice.append(&t("Pages"));
	view_choice.set_selection(VIEW_HEADINGS);
	#[cfg(target_os = "macos")]
	view_choice.set_accessibility_label(choice_label_text.replace('&', "").trim_end_matches(':').trim());
	choice_sizer.add(&choice_label, 0, SizerFlag::AlignCenterVertical | SizerFlag::Right, super::DIALOG_PADDING);
	choice_sizer.add(&view_choice, 1, SizerFlag::Expand, 0);
	content_sizer.add_sizer(&choice_sizer, 0, SizerFlag::Expand | SizerFlag::All, super::DIALOG_PADDING);
	let headings_tree =
		DataViewTreeCtrl::builder(&dialog).with_size(dpi::scale_size(&dialog, Size::new(400, 500))).build();
	content_sizer.add(
		&headings_tree,
		1,
		SizerFlag::Expand | SizerFlag::Left | SizerFlag::Right | SizerFlag::Bottom,
		super::DIALOG_PADDING,
	);
	let content_list = ListBox::builder(&dialog).build();
	content_sizer.add(
		&content_list,
		1,
		SizerFlag::Expand | SizerFlag::Left | SizerFlag::Right | SizerFlag::Bottom,
		super::DIALOG_PADDING,
	);
	content_list.show(false);
	ElementsDialogUiDv { content_sizer, view_choice, headings_tree, content_list }
}

#[cfg(not(target_os = "windows"))]
fn populate_elements_dialog_dv(
	session: &DocumentSession,
	current_pos: i64,
	headings_tree: DataViewTreeCtrl,
) -> (Rc<Cell<i64>>, HashMap<usize, i64>) {
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
	(selected_offset, item_offsets)
}

#[cfg(not(target_os = "windows"))]
fn bind_elements_view_toggle_dv(
	view_choice: Choice,
	headings_tree: DataViewTreeCtrl,
	content_list: ListBox,
	dialog: Dialog,
	links: &Rc<FlatView>,
	pages: &Rc<FlatView>,
) {
	let headings_tree_for_choice = headings_tree;
	let content_list_for_choice = content_list;
	let dialog_for_layout = dialog;
	let links_for_choice = Rc::clone(links);
	let pages_for_choice = Rc::clone(pages);
	view_choice.on_selection_changed(move |_| {
		let selection = view_choice.get_selection().unwrap_or(VIEW_HEADINGS);
		// Switch which pane is shown, repopulating the shared flat list for a non-headings
		// view. Focus is deliberately left where it is: the change comes from the user
		// arrowing through the view choice, and throwing them into the newly shown pane
		// with every arrow makes the dropdown impossible to browse.
		if selection == VIEW_HEADINGS {
			headings_tree_for_choice.show(true);
			content_list_for_choice.show(false);
		} else {
			headings_tree_for_choice.show(false);
			content_list_for_choice.show(true);
			let view = if selection == VIEW_LINKS { &links_for_choice } else { &pages_for_choice };
			fill_flat_list(content_list_for_choice, view);
		}
		dialog_for_layout.layout();
	});
}

#[cfg(not(target_os = "windows"))]
fn bind_elements_activation_dv(
	dialog: Dialog,
	view_choice: Choice,
	headings_tree: DataViewTreeCtrl,
	content_list: ListBox,
	item_offsets: &Rc<HashMap<usize, i64>>,
	links: &Rc<FlatView>,
	pages: &Rc<FlatView>,
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
	let view_for_list = view_choice;
	let list_for_click = content_list;
	let links_for_click = Rc::clone(links);
	let pages_for_click = Rc::clone(pages);
	let selected_for_list = Rc::clone(selected_offset);
	let dialog_for_list = dialog;
	content_list.on_item_double_clicked(move |_| {
		if let Some(offset) = flat_selected_offset(
			view_for_list.get_selection().unwrap_or(VIEW_HEADINGS),
			list_for_click,
			&links_for_click,
			&pages_for_click,
		) {
			selected_for_list.set(offset);
			dialog_for_list.end_modal(wxdragon::id::ID_OK);
		}
	});
}

#[cfg(not(target_os = "windows"))]
fn bind_elements_ok_action_dv(
	dialog: Dialog,
	view_choice: Choice,
	headings_tree: DataViewTreeCtrl,
	content_list: ListBox,
	item_offsets: &Rc<HashMap<usize, i64>>,
	links: &Rc<FlatView>,
	pages: &Rc<FlatView>,
	selected_offset: &Rc<Cell<i64>>,
	ok_button: Button,
) {
	let offsets_for_ok = Rc::clone(item_offsets);
	let selected_for_ok = Rc::clone(selected_offset);
	let dialog_for_ok = dialog;
	let view_for_ok = view_choice;
	let list_for_ok = content_list;
	let links_for_ok = Rc::clone(links);
	let pages_for_ok = Rc::clone(pages);
	ok_button.on_click(move |_| {
		let selection = view_for_ok.get_selection().unwrap_or(VIEW_HEADINGS);
		if selection == VIEW_HEADINGS {
			if let Some(item) = headings_tree.get_selection() {
				if let Some(id_ptr) = item.get_id::<c_void>() {
					if let Some(&offset) = offsets_for_ok.get(&(id_ptr as usize)) {
						selected_for_ok.set(offset);
						dialog_for_ok.end_modal(wxdragon::id::ID_OK);
					}
				}
			}
		} else if let Some(offset) = flat_selected_offset(selection, list_for_ok, &links_for_ok, &pages_for_ok) {
			selected_for_ok.set(offset);
			dialog_for_ok.end_modal(wxdragon::id::ID_OK);
		}
	});
}

// ── Windows implementation (TreeCtrl) ─────────────────────────────────────────

#[cfg(target_os = "windows")]
struct ElementsDialogUi {
	content_sizer: BoxSizer,
	view_choice: Choice,
	headings_tree: TreeCtrl,
	// The flat-list pane, shared by every non-headings view (Links, Pages, ...): its
	// contents are swapped in when the view changes.
	content_list: ListBox,
}

#[cfg(target_os = "windows")]
fn show_elements_dialog_wx(parent: &Frame, session: &DocumentSession, current_pos: i64) -> Option<(i64, ElementsKind)> {
	// TRANSLATORS: Title of the Elements dialog
	let dialog = Dialog::builder(parent, &t("Elements")).build();
	let ElementsDialogUi { content_sizer, view_choice, headings_tree, content_list } = build_elements_dialog_ui(dialog);
	let selected_offset = populate_elements_dialog(session, current_pos, headings_tree);
	let links = Rc::new(link_flat_view(session, current_pos));
	let pages = Rc::new(page_flat_view(session, current_pos));
	bind_elements_view_toggle(view_choice, headings_tree, content_list, dialog, &links, &pages);
	bind_elements_activation(dialog, view_choice, headings_tree, content_list, &selected_offset, &links, &pages);
	let (ok_button, cancel_button) = build_elements_buttons(dialog);
	bind_elements_ok_action(
		dialog,
		view_choice,
		headings_tree,
		content_list,
		&selected_offset,
		&links,
		&pages,
		ok_button,
	);
	finalize_elements_layout(dialog, content_sizer, ok_button, cancel_button);
	// The dialog opens on Headings (the choice is set to index 0 when it is built), so the
	// tree is the visible pane.
	headings_tree.set_focus();
	if dialog.show_modal() == ID_OK {
		let offset = selected_offset.get();
		let kind = ElementsKind::from_view(view_choice.get_selection().unwrap_or(VIEW_HEADINGS));
		if offset >= 0 { Some((offset, kind)) } else { None }
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
	// TRANSLATORS: Choice option in the view dropdown to show the list of pages
	view_choice.append(&t("Pages"));
	view_choice.set_selection(VIEW_HEADINGS);
	#[cfg(target_os = "macos")]
	view_choice.set_accessibility_label(choice_label_text.replace('&', "").trim_end_matches(':').trim());
	choice_sizer.add(&choice_label, 0, SizerFlag::AlignCenterVertical | SizerFlag::Right, super::DIALOG_PADDING);
	choice_sizer.add(&view_choice, 1, SizerFlag::Expand, 0);
	content_sizer.add_sizer(&choice_sizer, 0, SizerFlag::Expand | SizerFlag::All, super::DIALOG_PADDING);
	let headings_sizer = BoxSizer::builder(Orientation::Vertical).build();
	let headings_tree = TreeCtrl::builder(&dialog)
		.with_style(TreeCtrlStyle::Default | TreeCtrlStyle::HideRoot)
		.with_size(dpi::scale_size(&dialog, Size::new(400, 500)))
		.build();
	headings_sizer.add(&headings_tree, 1, SizerFlag::Expand, 0);
	content_sizer.add_sizer(
		&headings_sizer,
		1,
		SizerFlag::Expand | SizerFlag::Left | SizerFlag::Right | SizerFlag::Bottom,
		super::DIALOG_PADDING,
	);
	let list_sizer = BoxSizer::builder(Orientation::Vertical).build();
	let content_list = ListBox::builder(&dialog).build();
	list_sizer.add(&content_list, 1, SizerFlag::Expand, 0);
	content_sizer.add_sizer(
		&list_sizer,
		1,
		SizerFlag::Expand | SizerFlag::Left | SizerFlag::Right | SizerFlag::Bottom,
		super::DIALOG_PADDING,
	);
	content_list.show(false);
	ElementsDialogUi { content_sizer, view_choice, headings_tree, content_list }
}

#[cfg(target_os = "windows")]
fn populate_elements_dialog(session: &DocumentSession, current_pos: i64, headings_tree: TreeCtrl) -> Rc<Cell<i64>> {
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
		// TRANSLATORS: Placeholder text shown in the elements list when a document element has no text content
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
	selected_offset
}

#[cfg(target_os = "windows")]
fn bind_elements_view_toggle(
	view_choice: Choice,
	headings_tree: TreeCtrl,
	content_list: ListBox,
	dialog: Dialog,
	links: &Rc<FlatView>,
	pages: &Rc<FlatView>,
) {
	let headings_tree_for_choice = headings_tree;
	let content_list_for_choice = content_list;
	let dialog_for_layout = dialog;
	let links_for_choice = Rc::clone(links);
	let pages_for_choice = Rc::clone(pages);
	view_choice.on_selection_changed(move |_| {
		let selection = view_choice.get_selection().unwrap_or(VIEW_HEADINGS);
		// Switch which pane is shown, repopulating the shared flat list for a non-headings
		// view. Focus is deliberately left where it is: the change comes from the user
		// arrowing through the view choice, and throwing them into the newly shown pane
		// with every arrow makes the dropdown impossible to browse.
		if selection == VIEW_HEADINGS {
			headings_tree_for_choice.show(true);
			content_list_for_choice.show(false);
		} else {
			headings_tree_for_choice.show(false);
			content_list_for_choice.show(true);
			let view = if selection == VIEW_LINKS { &links_for_choice } else { &pages_for_choice };
			fill_flat_list(content_list_for_choice, view);
		}
		dialog_for_layout.layout();
	});
}

#[cfg(target_os = "windows")]
fn bind_elements_activation(
	dialog: Dialog,
	view_choice: Choice,
	headings_tree: TreeCtrl,
	content_list: ListBox,
	selected_offset: &Rc<Cell<i64>>,
	links: &Rc<FlatView>,
	pages: &Rc<FlatView>,
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
	let view_for_list = view_choice;
	let list_for_click = content_list;
	let links_for_click = Rc::clone(links);
	let pages_for_click = Rc::clone(pages);
	let selected_for_list = Rc::clone(selected_offset);
	let dialog_for_list = dialog;
	content_list.on_item_double_clicked(move |_| {
		if let Some(offset) = flat_selected_offset(
			view_for_list.get_selection().unwrap_or(VIEW_HEADINGS),
			list_for_click,
			&links_for_click,
			&pages_for_click,
		) {
			selected_for_list.set(offset);
			dialog_for_list.end_modal(ID_OK);
		}
	});
}

#[cfg(target_os = "windows")]
fn bind_elements_ok_action(
	dialog: Dialog,
	view_choice: Choice,
	headings_tree: TreeCtrl,
	content_list: ListBox,
	selected_offset: &Rc<Cell<i64>>,
	links: &Rc<FlatView>,
	pages: &Rc<FlatView>,
	ok_button: Button,
) {
	let selected_offset_for_ok = Rc::clone(selected_offset);
	let dialog_for_ok = dialog;
	let view_for_ok = view_choice;
	let list_for_ok = content_list;
	let links_for_ok = Rc::clone(links);
	let pages_for_ok = Rc::clone(pages);
	ok_button.on_click(move |_| {
		let selection = view_for_ok.get_selection().unwrap_or(VIEW_HEADINGS);
		if selection == VIEW_HEADINGS {
			if let Some(item) = headings_tree.get_selection()
				&& let Some(data) = headings_tree.get_custom_data(&item)
				&& let Some(offset) = data.downcast_ref::<i64>()
			{
				selected_offset_for_ok.set(*offset);
				dialog_for_ok.end_modal(ID_OK);
			}
		} else if let Some(offset) = flat_selected_offset(selection, list_for_ok, &links_for_ok, &pages_for_ok) {
			selected_offset_for_ok.set(offset);
			dialog_for_ok.end_modal(ID_OK);
		}
	});
}

// ── Layout helpers ─────────────────────────────────────────────────────────────

fn build_elements_buttons(dialog: Dialog) -> (Button, Button) {
	// TRANSLATORS: Label for the confirmation button
	super::build_ok_cancel_buttons(&dialog, &t("OK"))
}

fn finalize_elements_layout(dialog: Dialog, content_sizer: BoxSizer, ok_button: Button, cancel_button: Button) {
	super::add_ok_cancel_footer(content_sizer, ok_button, cancel_button);
	dialog.set_sizer_and_fit(content_sizer, true);
	dialog.centre();
}
