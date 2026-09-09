use std::{
	cell::{Cell, RefCell},
	rc::Rc,
};
#[cfg(not(target_os = "windows"))]
use std::{collections::HashMap, ffi::c_void};

use paperback_core::{document::MarkerType, session::DocumentSession};
use patois::t;
use wxdragon::prelude::*;

/// The view choice indices for [`show_elements_dialog`]. Headings is a tree; every other
/// view (Links, Pages, Tables, Lists, ...) is a flat list shown in the same list pane.
const VIEW_HEADINGS: u32 = 0;
/// How long a flat view's (empty) list is left shown before it is filled. Mirror of the Find All
/// results-list fix: NVDA intermittently spent ~10 s enumerating a list when it serviced the
/// "list just appeared" event after the rows had been added; showing an empty list first and
/// filling it later removes that stall.
const FLAT_POPULATE_DELAY_MS: i32 = 150;
const VIEW_LINKS: u32 = 1;
const VIEW_PAGES: u32 = 2;
const VIEW_TABLES: u32 = 3;
const VIEW_LISTS: u32 = 4;

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

/// One entry per page, labelled "Page N: <first content line>" (the same line that page
/// navigation (p / Shift+P) announces), or just "Page N" when the page has no readable line.
fn page_entries(session: &DocumentSession) -> Vec<PageEntry> {
	session
		.page_offsets()
		.iter()
		.enumerate()
		.map(|(index, &offset)| {
			let page = i32::try_from(index).unwrap_or(0) + 1;
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
/// jumps to, and which row is nearest the current position. The Links, Pages, Tables and Lists
/// views each have one; the shared list pane is repopulated from whichever is selected.
struct FlatView {
	entries: Vec<(String, i64)>,
	closest: Option<u32>,
}

/// The rows of a document-marker element view (Links, Tables, Lists): each shows the same text
/// reading navigation announces for that element, and the row nearest the current position is
/// preselected. A marker with its own text (a table's caption, a link's label) uses it; a
/// text-less marker (a list) reads the line it sits on.
fn marker_flat_view(session: &DocumentSession, mtype: MarkerType, position: i64) -> FlatView {
	let data = session.element_list(mtype, position);
	let entries =
		data.items.iter().map(|item| (item.text.clone(), i64::try_from(item.offset).unwrap_or(i64::MAX))).collect();
	let closest = if data.closest_index >= 0 { u32::try_from(data.closest_index).ok() } else { None };
	FlatView { entries, closest }
}

fn link_flat_view(session: &DocumentSession, position: i64) -> FlatView {
	marker_flat_view(session, MarkerType::Link, position)
}

fn table_flat_view(session: &DocumentSession, position: i64) -> FlatView {
	marker_flat_view(session, MarkerType::Table, position)
}

fn list_flat_view(session: &DocumentSession, position: i64) -> FlatView {
	marker_flat_view(session, MarkerType::List, position)
}

fn page_flat_view(session: &DocumentSession, position: i64) -> FlatView {
	let entries = page_entries(session).iter().map(|page| (page.label.clone(), page.offset)).collect();
	let closest = {
		let page = session.current_page(position);
		if page >= 1 { u32::try_from(page - 1).ok() } else { None }
	};
	FlatView { entries, closest }
}

/// The flat (non-headings) views, keyed by their `VIEW_*` selection. Holding them together lets
/// the toggle, double-click and OK handlers look up the active view without threading each view
/// through as its own argument; a future flat view adds one field here and one arm of
/// [`FlatViews::view`].
struct FlatViews {
	links: FlatView,
	pages: FlatView,
	tables: FlatView,
	lists: FlatView,
}

impl FlatViews {
	fn for_session(session: &DocumentSession, position: i64) -> Self {
		Self {
			links: link_flat_view(session, position),
			pages: page_flat_view(session, position),
			tables: table_flat_view(session, position),
			lists: list_flat_view(session, position),
		}
	}

	const fn view(&self, selection: u32) -> Option<&FlatView> {
		match selection {
			VIEW_LINKS => Some(&self.links),
			VIEW_PAGES => Some(&self.pages),
			VIEW_TABLES => Some(&self.tables),
			VIEW_LISTS => Some(&self.lists),
			_ => None,
		}
	}
}

// The flat (non-headings) pane. Its rows can number in the tens of thousands (the Pages view of
// a long document), so, as with the Find All results list, it is a virtual wxListCtrl on
// Windows/Linux and a store-backed wxDataViewListCtrl on macOS (the split the All Documents
// dialog uses): the control keeps no rows and asks for text only for the rows it shows, so
// switching to a flat view is O(1) instead of appending every row. This mirrors ui/find.rs; if a
// third control needs the same glue, factor it into a shared module.
#[cfg(target_os = "macos")]
type FlatList = DataViewListCtrl;
#[cfg(not(target_os = "macos"))]
type FlatList = ListCtrl;

fn flat_list_size(dialog: Dialog) -> Size {
	dialog.from_dip(Size::new(400, 500))
}

#[cfg(not(target_os = "macos"))]
fn build_flat_list(dialog: Dialog, flat_labels: Rc<RefCell<Vec<String>>>) -> FlatList {
	let size = flat_list_size(dialog);
	let list = ListCtrl::builder(&dialog)
		.with_style(ListCtrlStyle::Report | ListCtrlStyle::Virtual | ListCtrlStyle::SingleSel)
		.with_size(size)
		.build();
	list.insert_column(0, "", ListColumnFormat::Left, dialog.from_dip_int(400));
	list.set_min_size(size);
	list.set_max_size(size);
	// The control asks for text on demand; hand back a prebuilt label (a cheap clone).
	list.set_virtual_text_callback(move |index, _column| {
		let labels = flat_labels.borrow();
		usize::try_from(index).ok().and_then(|index| labels.get(index)).cloned().unwrap_or_default()
	});
	list.set_item_count(0);
	list
}

#[cfg(target_os = "macos")]
fn build_flat_list(dialog: Dialog, _flat_labels: Rc<RefCell<Vec<String>>>) -> FlatList {
	let size = flat_list_size(dialog);
	let list = DataViewListCtrl::builder(&dialog).with_style(DataViewStyle::RowLines).with_size(size).build();
	list.append_text_column("", 0, DataViewAlign::Left, dialog.from_dip_int(400), DataViewColumnFlags::Resizable);
	list.set_min_size(size);
	list.set_max_size(size);
	list
}

#[cfg(not(target_os = "macos"))]
fn populate_flat_list(list: FlatList, flat_labels: &Rc<RefCell<Vec<String>>>, view: &FlatView) {
	// Reset to zero rows first so a previous run's selection never lingers when the new view has
	// no row nearest the caret.
	list.set_item_count(0);
	// Build the labels, dropping the borrow before the count changes: setting the count can make
	// the native control ask the virtual callback for text, which must not find the cache still
	// mutably borrowed (that would panic across the FFI boundary).
	let len = {
		let mut labels = flat_labels.borrow_mut();
		labels.clear();
		labels.reserve(view.entries.len());
		for (label, _) in &view.entries {
			labels.push(label.clone());
		}
		labels.len()
	};
	list.set_item_count(i64::try_from(len).unwrap_or(0));
	if let Some(index) = view.closest {
		select_flat_row(list, i32::try_from(index).unwrap_or(0));
	}
}

#[cfg(target_os = "macos")]
fn populate_flat_list(list: FlatList, _flat_labels: &Rc<RefCell<Vec<String>>>, view: &FlatView) {
	list.delete_all_items();
	for (label, _) in &view.entries {
		list.append_item(&[Variant::from(label.clone())]);
	}
	if let Some(index) = view.closest {
		select_flat_row(list, i32::try_from(index).unwrap_or(0));
	}
}

#[cfg(not(target_os = "macos"))]
fn select_flat_row(list: FlatList, index: i32) {
	if index < 0 {
		return;
	}
	list.set_item_state(
		i64::from(index),
		ListItemState::Selected | ListItemState::Focused,
		ListItemState::Selected | ListItemState::Focused,
	);
	list.ensure_visible(i64::from(index));
}

#[cfg(target_os = "macos")]
fn select_flat_row(list: FlatList, index: i32) {
	let Ok(index) = usize::try_from(index) else { return };
	list.select_row(index);
	if let Some(item) = list.row_to_item(index) {
		list.set_current_item(&item);
		list.ensure_visible(&item);
	}
}

#[cfg(not(target_os = "macos"))]
fn flat_list_selected_index(list: FlatList) -> Option<usize> {
	let index = list.get_first_selected_item();
	if index >= 0 { usize::try_from(index).ok() } else { None }
}

#[cfg(target_os = "macos")]
fn flat_list_selected_index(list: FlatList) -> Option<usize> {
	list.get_selected_row()
}

/// Shows `view`'s rows in the shared flat pane, naming the list after the view the user chose.
fn present_flat_view(list: FlatList, flat_labels: &Rc<RefCell<Vec<String>>>, view: &FlatView, name: &str) {
	list.set_accessibility_label(name);
	populate_flat_list(list, flat_labels, view);
}

#[cfg(not(target_os = "macos"))]
fn clear_flat_list(list: FlatList) {
	list.set_item_count(0);
}

#[cfg(target_os = "macos")]
fn clear_flat_list(list: FlatList) {
	list.delete_all_items();
}

/// Switches the flat pane to `view` (the list entry whose `VIEW_*` index is `selection`) a short
/// moment after it was shown empty, so the screen reader settles on the newly-shown pane before it
/// fills with rows. If the user has since chosen a different view, the pending fill is skipped.
fn schedule_flat_fill(
	frame: Frame,
	list: FlatList,
	flat_labels: Rc<RefCell<Vec<String>>>,
	views: Rc<FlatViews>,
	view_choice: Choice,
	selection: u32,
) {
	let timer_holder: Rc<RefCell<Option<Timer<Frame>>>> = Rc::new(RefCell::new(None));
	let holder = Rc::clone(&timer_holder);
	let labels_for_tick = Rc::clone(&flat_labels);
	let views_for_tick = Rc::clone(&views);
	let timer = Timer::new(&frame);
	timer.on_tick(move |_event| {
		if view_choice.get_selection() == Some(selection) {
			fill_flat_view(list, &labels_for_tick, &views_for_tick, view_choice, selection);
		}
		*holder.borrow_mut() = None;
	});
	if !timer.start(FLAT_POPULATE_DELAY_MS, true) {
		fill_flat_view(list, &flat_labels, &views, view_choice, selection);
		return;
	}
	*timer_holder.borrow_mut() = Some(timer);
}

/// Fills the list with `views.view(selection)`'s rows while it is hidden, then shows it.
fn fill_flat_view(
	list: FlatList,
	flat_labels: &Rc<RefCell<Vec<String>>>,
	views: &Rc<FlatViews>,
	view_choice: Choice,
	selection: u32,
) {
	if let Some(view) = views.view(selection) {
		let name = view_choice.get_string_selection().unwrap_or_default();
		list.show(false);
		present_flat_view(list, flat_labels, view, &name);
		list.show(true);
	}
}

/// The offset of the row selected in the flat list pane for `view`, if any.
fn flat_selected_offset(view: u32, list: FlatList, views: &FlatViews) -> Option<i64> {
	let entries = &views.view(view)?.entries;
	flat_list_selected_index(list).and_then(|index| entries.get(index)).map(|(_, offset)| *offset)
}

// ── DataViewTreeCtrl implementation (Linux + macOS) ───────────────────────────

#[cfg(not(target_os = "windows"))]
struct ElementsDialogUiDv {
	content_sizer: BoxSizer,
	view_choice: Choice,
	headings_tree: DataViewTreeCtrl,
	// The flat-list pane, shared by every non-headings view (Links, Pages, ...): its
	// contents are swapped in when the view changes.
	content_list: FlatList,
}

#[cfg(not(target_os = "windows"))]
fn show_elements_dialog_dv(parent: &Frame, session: &DocumentSession, current_pos: i64) -> Option<(i64, ElementsKind)> {
	// TRANSLATORS: Title of the Elements dialog
	let dialog = Dialog::builder(parent, &t("Elements")).build();
	let flat_labels = Rc::new(RefCell::new(Vec::new()));
	let ElementsDialogUiDv { content_sizer, view_choice, headings_tree, content_list } =
		build_elements_dialog_ui_dv(dialog, Rc::clone(&flat_labels));
	let (selected_offset, item_offsets) = populate_elements_dialog_dv(session, current_pos, headings_tree);
	let item_offsets = Rc::new(item_offsets);
	let views = Rc::new(FlatViews::for_session(session, current_pos));
	bind_elements_view_toggle_dv(*parent, view_choice, headings_tree, content_list, dialog, &views, &flat_labels);
	bind_elements_activation_dv(
		dialog,
		view_choice,
		headings_tree,
		content_list,
		&item_offsets,
		&views,
		&selected_offset,
	);
	let (ok_button, cancel_button) = build_elements_buttons(dialog);
	bind_elements_ok_action_dv(
		dialog,
		view_choice,
		headings_tree,
		content_list,
		&item_offsets,
		&views,
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
fn build_elements_dialog_ui_dv(dialog: Dialog, flat_labels: Rc<RefCell<Vec<String>>>) -> ElementsDialogUiDv {
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
	// TRANSLATORS: Choice option in the view dropdown to show the list of tables
	view_choice.append(&t("Tables"));
	// TRANSLATORS: Choice option in the view dropdown to show the list of lists
	view_choice.append(&t("Lists"));
	view_choice.set_selection(VIEW_HEADINGS);
	#[cfg(target_os = "macos")]
	view_choice.set_accessibility_label(choice_label_text.replace('&', "").trim_end_matches(':').trim());
	choice_sizer.add(&choice_label, 0, SizerFlag::AlignCenterVertical | SizerFlag::Right, super::DIALOG_PADDING);
	choice_sizer.add(&view_choice, 1, SizerFlag::Expand, 0);
	content_sizer.add_sizer(&choice_sizer, 0, SizerFlag::Expand | SizerFlag::All, super::DIALOG_PADDING);
	let headings_tree = DataViewTreeCtrl::builder(&dialog).with_size(dialog.from_dip(Size::new(400, 500))).build();
	content_sizer.add(
		&headings_tree,
		1,
		SizerFlag::Expand | SizerFlag::Left | SizerFlag::Right | SizerFlag::Bottom,
		super::DIALOG_PADDING,
	);
	let content_list = build_flat_list(dialog, flat_labels);
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
	// Precompute which items have children (single pass over items) so we can use append_container
	// vs append_item.
	let mut has_children_vec = vec![false; tree_data.items.len()];
	for item in &tree_data.items {
		if item.parent_index >= 0
			&& let Ok(parent_idx) = usize::try_from(item.parent_index)
			&& let Some(flag) = has_children_vec.get_mut(parent_idx)
		{
			*flag = true;
		}
	}
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
	if let Some(idx) = select_idx
		&& let Some(item) = item_ids.get(idx)
	{
		headings_tree.select(item);
		headings_tree.ensure_visible(item);
	}
	(selected_offset, item_offsets)
}

#[cfg(not(target_os = "windows"))]
fn bind_elements_view_toggle_dv(
	frame: Frame,
	view_choice: Choice,
	headings_tree: DataViewTreeCtrl,
	content_list: FlatList,
	dialog: Dialog,
	views: &Rc<FlatViews>,
	flat_labels: &Rc<RefCell<Vec<String>>>,
) {
	let headings_tree_for_choice = headings_tree;
	let content_list_for_choice = content_list;
	let dialog_for_layout = dialog;
	let views_for_choice = Rc::clone(views);
	let flat_labels_for_choice = Rc::clone(flat_labels);
	view_choice.on_selection_changed(move |_| {
		let selection = view_choice.get_selection().unwrap_or(VIEW_HEADINGS);
		// Switch which pane is shown. Focus is deliberately left in the view choice (the change
		// comes from the user arrowing through it, and throwing them into the newly shown pane
		// with every arrow makes the dropdown impossible to browse).
		if selection == VIEW_HEADINGS {
			headings_tree_for_choice.show(true);
			content_list_for_choice.show(false);
		} else {
			headings_tree_for_choice.show(false);
			// Show the pane empty first, then fill it a moment later so the screen reader settles
			// on the newly-shown list before it grows to its full size (see the Find All fix).
			clear_flat_list(content_list_for_choice);
			let name = view_choice.get_string_selection().unwrap_or_default();
			content_list_for_choice.set_accessibility_label(&name);
			content_list_for_choice.show(true);
			schedule_flat_fill(
				frame,
				content_list_for_choice,
				Rc::clone(&flat_labels_for_choice),
				Rc::clone(&views_for_choice),
				view_choice,
				selection,
			);
		}
		dialog_for_layout.layout();
	});
}

#[cfg(not(target_os = "windows"))]
fn bind_elements_activation_dv(
	dialog: Dialog,
	view_choice: Choice,
	headings_tree: DataViewTreeCtrl,
	content_list: FlatList,
	item_offsets: &Rc<HashMap<usize, i64>>,
	views: &Rc<FlatViews>,
	selected_offset: &Rc<Cell<i64>>,
) {
	let offsets_for_tree = Rc::clone(item_offsets);
	let selected_for_tree = Rc::clone(selected_offset);
	let dialog_for_tree = dialog;
	headings_tree.on_item_activated(move |event| {
		if let Some(item) = event.get_item()
			&& let Some(id_ptr) = item.get_id::<c_void>()
			&& let Some(&offset) = offsets_for_tree.get(&(id_ptr as usize))
		{
			selected_for_tree.set(offset);
			dialog_for_tree.end_modal(wxdragon::id::ID_OK);
		}
	});
	let view_for_list = view_choice;
	let list_for_click = content_list;
	let views_for_click = Rc::clone(views);
	let selected_for_list = Rc::clone(selected_offset);
	let dialog_for_list = dialog;
	content_list.on_item_activated(move |_| {
		if let Some(offset) = flat_selected_offset(
			view_for_list.get_selection().unwrap_or(VIEW_HEADINGS),
			list_for_click,
			&views_for_click,
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
	content_list: FlatList,
	item_offsets: &Rc<HashMap<usize, i64>>,
	views: &Rc<FlatViews>,
	selected_offset: &Rc<Cell<i64>>,
	ok_button: Button,
) {
	let offsets_for_ok = Rc::clone(item_offsets);
	let selected_for_ok = Rc::clone(selected_offset);
	let dialog_for_ok = dialog;
	let view_for_ok = view_choice;
	let list_for_ok = content_list;
	let views_for_ok = Rc::clone(views);
	ok_button.on_click(move |_| {
		let selection = view_for_ok.get_selection().unwrap_or(VIEW_HEADINGS);
		if selection == VIEW_HEADINGS {
			if let Some(item) = headings_tree.get_selection()
				&& let Some(id_ptr) = item.get_id::<c_void>()
				&& let Some(&offset) = offsets_for_ok.get(&(id_ptr as usize))
			{
				selected_for_ok.set(offset);
				dialog_for_ok.end_modal(wxdragon::id::ID_OK);
			}
		} else if let Some(offset) = flat_selected_offset(selection, list_for_ok, &views_for_ok) {
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
	content_list: FlatList,
}

#[cfg(target_os = "windows")]
fn show_elements_dialog_wx(parent: &Frame, session: &DocumentSession, current_pos: i64) -> Option<(i64, ElementsKind)> {
	// TRANSLATORS: Title of the Elements dialog
	let dialog = Dialog::builder(parent, &t("Elements")).build();
	let flat_labels = Rc::new(RefCell::new(Vec::new()));
	let ElementsDialogUi { content_sizer, view_choice, headings_tree, content_list } =
		build_elements_dialog_ui(dialog, Rc::clone(&flat_labels));
	let selected_offset = populate_elements_dialog(session, current_pos, headings_tree);
	let views = Rc::new(FlatViews::for_session(session, current_pos));
	bind_elements_view_toggle(*parent, view_choice, headings_tree, content_list, dialog, &views, &flat_labels);
	bind_elements_activation(dialog, view_choice, headings_tree, content_list, &selected_offset, &views);
	let (ok_button, cancel_button) = build_elements_buttons(dialog);
	bind_elements_ok_action(dialog, view_choice, headings_tree, content_list, &selected_offset, &views, ok_button);
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
fn build_elements_dialog_ui(dialog: Dialog, flat_labels: Rc<RefCell<Vec<String>>>) -> ElementsDialogUi {
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
	// TRANSLATORS: Choice option in the view dropdown to show the list of tables
	view_choice.append(&t("Tables"));
	// TRANSLATORS: Choice option in the view dropdown to show the list of lists
	view_choice.append(&t("Lists"));
	view_choice.set_selection(VIEW_HEADINGS);
	#[cfg(target_os = "macos")]
	view_choice.set_accessibility_label(choice_label_text.replace('&', "").trim_end_matches(':').trim());
	choice_sizer.add(&choice_label, 0, SizerFlag::AlignCenterVertical | SizerFlag::Right, super::DIALOG_PADDING);
	choice_sizer.add(&view_choice, 1, SizerFlag::Expand, 0);
	content_sizer.add_sizer(&choice_sizer, 0, SizerFlag::Expand | SizerFlag::All, super::DIALOG_PADDING);
	let headings_sizer = BoxSizer::builder(Orientation::Vertical).build();
	let headings_tree = TreeCtrl::builder(&dialog)
		.with_style(TreeCtrlStyle::Default | TreeCtrlStyle::HideRoot)
		.with_size(dialog.from_dip(Size::new(400, 500)))
		.build();
	headings_sizer.add(&headings_tree, 1, SizerFlag::Expand, 0);
	content_sizer.add_sizer(
		&headings_sizer,
		1,
		SizerFlag::Expand | SizerFlag::Left | SizerFlag::Right | SizerFlag::Bottom,
		super::DIALOG_PADDING,
	);
	let list_sizer = BoxSizer::builder(Orientation::Vertical).build();
	let content_list = build_flat_list(dialog, flat_labels);
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
	frame: Frame,
	view_choice: Choice,
	headings_tree: TreeCtrl,
	content_list: FlatList,
	dialog: Dialog,
	views: &Rc<FlatViews>,
	flat_labels: &Rc<RefCell<Vec<String>>>,
) {
	let headings_tree_for_choice = headings_tree;
	let content_list_for_choice = content_list;
	let dialog_for_layout = dialog;
	let views_for_choice = Rc::clone(views);
	let flat_labels_for_choice = Rc::clone(flat_labels);
	view_choice.on_selection_changed(move |_| {
		let selection = view_choice.get_selection().unwrap_or(VIEW_HEADINGS);
		// Switch which pane is shown. Focus is deliberately left in the view choice (the change
		// comes from the user arrowing through it, and throwing them into the newly shown pane
		// with every arrow makes the dropdown impossible to browse).
		if selection == VIEW_HEADINGS {
			headings_tree_for_choice.show(true);
			content_list_for_choice.show(false);
		} else {
			headings_tree_for_choice.show(false);
			// Show the pane empty first, then fill it a moment later so the screen reader settles
			// on the newly-shown list before it grows to its full size (see the Find All fix).
			clear_flat_list(content_list_for_choice);
			let name = view_choice.get_string_selection().unwrap_or_default();
			content_list_for_choice.set_accessibility_label(&name);
			content_list_for_choice.show(true);
			schedule_flat_fill(
				frame,
				content_list_for_choice,
				Rc::clone(&flat_labels_for_choice),
				Rc::clone(&views_for_choice),
				view_choice,
				selection,
			);
		}
		dialog_for_layout.layout();
	});
}

#[cfg(target_os = "windows")]
fn bind_elements_activation(
	dialog: Dialog,
	view_choice: Choice,
	headings_tree: TreeCtrl,
	content_list: FlatList,
	selected_offset: &Rc<Cell<i64>>,
	views: &Rc<FlatViews>,
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
	let views_for_click = Rc::clone(views);
	let selected_for_list = Rc::clone(selected_offset);
	let dialog_for_list = dialog;
	content_list.on_item_activated(move |_| {
		if let Some(offset) = flat_selected_offset(
			view_for_list.get_selection().unwrap_or(VIEW_HEADINGS),
			list_for_click,
			&views_for_click,
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
	content_list: FlatList,
	selected_offset: &Rc<Cell<i64>>,
	views: &Rc<FlatViews>,
	ok_button: Button,
) {
	let selected_offset_for_ok = Rc::clone(selected_offset);
	let dialog_for_ok = dialog;
	let view_for_ok = view_choice;
	let list_for_ok = content_list;
	let views_for_ok = Rc::clone(views);
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
		} else if let Some(offset) = flat_selected_offset(selection, list_for_ok, &views_for_ok) {
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
