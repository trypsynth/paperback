//! The flat (non-headings) element views shared by both platform implementations: the view model
//! (Links, Pages, Tables, Lists) and the virtual list widget that shows whichever view is chosen.

#[cfg(target_os = "windows")]
use std::collections::HashMap;
use std::{cell::RefCell, rc::Rc};

#[cfg(target_os = "windows")]
use paperback_core::types::HeadingTree;
use paperback_core::{document::MarkerType, session::DocumentSession};
use patois::t;
use wxdragon::prelude::*;

use super::{FLAT_POPULATE_DELAY_MS, VIEW_HEADINGS, VIEW_LINKS, VIEW_LISTS, VIEW_PAGES, VIEW_TABLES};

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
pub(super) struct FlatView {
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

/// The Headings view as a list in reading order, for a heading tree with more than
/// [`super::super::MAX_TREE_SIBLINGS`] headings under one parent (see that constant for why).
#[cfg(target_os = "windows")]
pub(super) fn headings_as_list(tree: &HeadingTree) -> Option<FlatView> {
	let mut siblings: HashMap<i32, usize> = HashMap::new();
	for item in &tree.items {
		*siblings.entry(item.parent_index).or_default() += 1;
	}
	if siblings.values().all(|&count| count <= super::super::MAX_TREE_SIBLINGS) {
		return None;
	}
	let entries = tree
		.items
		.iter()
		.map(|item| {
			// TRANSLATORS: Placeholder text shown in the elements list when a document element has no text content
			let label = if item.text.is_empty() { t("Untitled") } else { item.text.clone() };
			(label, i64::try_from(item.offset).unwrap_or(i64::MAX))
		})
		.collect();
	let closest = if tree.closest_index >= 0 { u32::try_from(tree.closest_index).ok() } else { Some(0) };
	Some(FlatView { entries, closest })
}

/// The flat (non-headings) views, keyed by their `VIEW_*` selection. Holding them together lets
/// the toggle, double-click and OK handlers look up the active view without threading each view
/// through as its own argument; a future flat view adds one field here and one arm of
/// [`FlatViews::view`].
pub(super) struct FlatViews {
	/// Set only when the headings are shown as a list instead of a tree.
	headings: Option<FlatView>,
	links: FlatView,
	pages: FlatView,
	tables: FlatView,
	lists: FlatView,
}

impl FlatViews {
	pub(super) fn for_session(session: &DocumentSession, position: i64, headings: Option<FlatView>) -> Self {
		Self {
			headings,
			links: link_flat_view(session, position),
			pages: page_flat_view(session, position),
			tables: table_flat_view(session, position),
			lists: list_flat_view(session, position),
		}
	}

	#[cfg(target_os = "windows")]
	pub(super) const fn headings_are_flat(&self) -> bool {
		self.headings.is_some()
	}

	const fn view(&self, selection: u32) -> Option<&FlatView> {
		match selection {
			VIEW_HEADINGS => self.headings.as_ref(),
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
pub(super) type FlatList = DataViewListCtrl;
#[cfg(not(target_os = "macos"))]
pub(super) type FlatList = ListCtrl;

fn flat_list_size(dialog: Dialog) -> Size {
	dialog.from_dip(Size::new(400, 500))
}

#[cfg(not(target_os = "macos"))]
pub(in crate::ui::dialogs) fn build_flat_list(dialog: Dialog, flat_labels: Rc<RefCell<Vec<String>>>) -> FlatList {
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
pub(super) fn build_flat_list(dialog: Dialog, _flat_labels: Rc<RefCell<Vec<String>>>) -> FlatList {
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
pub(in crate::ui::dialogs) fn select_flat_row(list: FlatList, index: i32) {
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
pub(in crate::ui::dialogs) fn flat_list_selected_index(list: FlatList) -> Option<usize> {
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
pub(super) fn clear_flat_list(list: FlatList) {
	list.set_item_count(0);
}

#[cfg(target_os = "macos")]
pub(super) fn clear_flat_list(list: FlatList) {
	list.delete_all_items();
}

/// Switches the flat pane to `view` (the list entry whose `VIEW_*` index is `selection`) a short
/// moment after it was shown empty, so the screen reader settles on the newly-shown pane before it
/// fills with rows. If the user has since chosen a different view, the pending fill is skipped.
pub(super) fn schedule_flat_fill(
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
pub(super) fn fill_flat_view(
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
pub(super) fn flat_selected_offset(view: u32, list: FlatList, views: &FlatViews) -> Option<i64> {
	let entries = &views.view(view)?.entries;
	flat_list_selected_index(list).and_then(|index| entries.get(index)).map(|(_, offset)| *offset)
}
