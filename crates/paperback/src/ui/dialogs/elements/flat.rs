use std::{cell::RefCell, rc::Rc};

use paperback_core::{
	document::{DocPos, MarkerType},
	session::DocumentSession,
};
use patois::t;
use wxdragon::prelude::*;

use super::{FLAT_POPULATE_DELAY_MS, VIEW_LINKS, VIEW_LISTS, VIEW_PAGES, VIEW_TABLES};

struct PageEntry {
	offset: DocPos,
	label: String,
}

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

struct FlatView {
	entries: Vec<(String, DocPos)>,
	closest: Option<u32>,
}

fn marker_flat_view(session: &DocumentSession, mtype: MarkerType, position: DocPos) -> FlatView {
	let data = session.element_list(mtype, position);
	let entries = data.items.iter().map(|item| (item.text.clone(), item.offset)).collect();
	let closest = if data.closest_index >= 0 { u32::try_from(data.closest_index).ok() } else { None };
	FlatView { entries, closest }
}

fn link_flat_view(session: &DocumentSession, position: DocPos) -> FlatView {
	marker_flat_view(session, MarkerType::Link, position)
}

fn table_flat_view(session: &DocumentSession, position: DocPos) -> FlatView {
	marker_flat_view(session, MarkerType::Table, position)
}

fn list_flat_view(session: &DocumentSession, position: DocPos) -> FlatView {
	marker_flat_view(session, MarkerType::List, position)
}

fn page_flat_view(session: &DocumentSession, position: DocPos) -> FlatView {
	let entries = page_entries(session).iter().map(|page| (page.label.clone(), page.offset)).collect();
	let closest = {
		let page = session.current_page(position);
		if page >= 1 { u32::try_from(page - 1).ok() } else { None }
	};
	FlatView { entries, closest }
}

pub(super) struct FlatViews {
	links: FlatView,
	pages: FlatView,
	tables: FlatView,
	lists: FlatView,
}

impl FlatViews {
	pub(super) fn for_session(session: &DocumentSession, position: DocPos) -> Self {
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

// Rows can number in the tens of thousands (Pages in a long book), so off macOS it is a virtual list that asks for text only for the rows it shows; mirrors ui/find.rs.
#[cfg(target_os = "macos")]
pub(super) type FlatList = DataViewListCtrl;
#[cfg(not(target_os = "macos"))]
pub(super) type FlatList = ListCtrl;

fn flat_list_size(dialog: Dialog) -> Size {
	dialog.from_dip(Size::new(400, 500))
}

#[cfg(not(target_os = "macos"))]
pub(super) fn build_flat_list(dialog: Dialog, flat_labels: Rc<RefCell<Vec<String>>>) -> FlatList {
	let size = flat_list_size(dialog);
	let list = ListCtrl::builder(&dialog)
		.with_style(ListCtrlStyle::Report | ListCtrlStyle::Virtual | ListCtrlStyle::SingleSel)
		.with_size(size)
		.build();
	list.insert_column(0, "", ListColumnFormat::Left, dialog.from_dip_int(400));
	list.set_min_size(size);
	list.set_max_size(size);
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
	// So a previous view's selection never lingers when the new one has no row nearest the caret.
	list.set_item_count(0);
	// Setting the count can call the virtual text callback, so the borrow must end first or it panics across FFI.
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

pub(super) fn schedule_flat_fill(
	frame: Frame,
	list: FlatList,
	flat_labels: &Rc<RefCell<Vec<String>>>,
	views: &Rc<FlatViews>,
	view_choice: Choice,
	selection: u32,
) {
	let timer_holder: Rc<RefCell<Option<Timer<Frame>>>> = Rc::new(RefCell::new(None));
	let holder = Rc::clone(&timer_holder);
	let labels_for_tick = Rc::clone(flat_labels);
	let views_for_tick = Rc::clone(views);
	let timer = Timer::new(&frame);
	timer.on_tick(move |_event| {
		if view_choice.get_selection() == Some(selection) {
			fill_flat_view(list, &labels_for_tick, &views_for_tick, view_choice, selection);
		}
		*holder.borrow_mut() = None;
	});
	if !timer.start(FLAT_POPULATE_DELAY_MS, true) {
		fill_flat_view(list, flat_labels, views, view_choice, selection);
		return;
	}
	*timer_holder.borrow_mut() = Some(timer);
}

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

pub(super) fn flat_selected_offset(view: u32, list: FlatList, views: &FlatViews) -> Option<DocPos> {
	let entries = &views.view(view)?.entries;
	flat_list_selected_index(list).and_then(|index| entries.get(index)).map(|(_, offset)| *offset)
}
