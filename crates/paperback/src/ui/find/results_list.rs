use std::{cell::RefCell, rc::Rc};

use paperback_core::session::FindAllLine;
use patois::t;
use wxdragon::prelude::*;

use super::super::navigation;

// VoiceOver does not see a wxListCtrl as a table, so macOS gets a wxDataViewListCtrl; elsewhere a virtual list, since appending a native row per match froze the dialog on large documents.
#[cfg(target_os = "macos")]
pub(super) type ResultsList = DataViewListCtrl;
#[cfg(not(target_os = "macos"))]
pub(super) type ResultsList = ListCtrl;

fn result_row_label(row: &FindAllLine) -> String {
	if row.page > 0 { navigation::page_announcement(row.page, &row.text) } else { row.text.clone() }
}

/// Used as both minimum and maximum so `fit()` never sizes the dialog to a 40,000-row list's item count.
fn results_list_size(dialog: Dialog) -> Size {
	dialog.from_dip(Size::new(600, 500))
}

#[cfg(not(target_os = "macos"))]
pub(super) fn build_results_list(
	dialog: Dialog,
	_result_rows: Rc<RefCell<Vec<FindAllLine>>>,
	result_labels: Rc<RefCell<Vec<String>>>,
) -> ResultsList {
	let size = results_list_size(dialog);
	let results_list = ListCtrl::builder(&dialog)
		.with_style(ListCtrlStyle::Report | ListCtrlStyle::Virtual | ListCtrlStyle::SingleSel)
		.with_size(size)
		.build();
	results_list.insert_column(0, "", ListColumnFormat::Left, dialog.from_dip_int(600));
	results_list.set_min_size(size);
	results_list.set_max_size(size);
	// TRANSLATORS: Accessible name of the list of Find All results
	results_list.set_accessibility_label(&t("Results"));
	results_list.set_virtual_text_callback(move |index, _column| {
		let labels = result_labels.borrow();
		usize::try_from(index).ok().and_then(|index| labels.get(index)).cloned().unwrap_or_default()
	});
	results_list.set_item_count(0);
	results_list
}

#[cfg(target_os = "macos")]
pub(super) fn build_results_list(
	dialog: Dialog,
	_result_rows: Rc<RefCell<Vec<FindAllLine>>>,
	_result_labels: Rc<RefCell<Vec<String>>>,
) -> ResultsList {
	let size = results_list_size(dialog);
	let results_list = DataViewListCtrl::builder(&dialog).with_style(DataViewStyle::RowLines).with_size(size).build();
	results_list.append_text_column(
		"",
		0,
		DataViewAlign::Left,
		dialog.from_dip_int(600),
		DataViewColumnFlags::Resizable,
	);
	results_list.set_min_size(size);
	results_list.set_max_size(size);
	// TRANSLATORS: Accessible name of the list of Find All results
	results_list.set_accessibility_label(&t("Results"));
	results_list
}

#[cfg(not(target_os = "macos"))]
pub(super) fn populate_results_list(
	list: ResultsList,
	rows: &Rc<RefCell<Vec<FindAllLine>>>,
	labels: &Rc<RefCell<Vec<String>>>,
) {
	let mut labels = labels.borrow_mut();
	labels.clear();
	{
		let rows = rows.borrow();
		labels.reserve(rows.len());
		for row in rows.iter() {
			labels.push(result_row_label(row));
		}
	}
	list.set_item_count(i64::try_from(labels.len()).unwrap_or(0));
}

#[cfg(target_os = "macos")]
pub(super) fn populate_results_list(
	list: ResultsList,
	rows: &Rc<RefCell<Vec<FindAllLine>>>,
	_labels: &Rc<RefCell<Vec<String>>>,
) {
	list.delete_all_items();
	for row in rows.borrow().iter() {
		list.append_item(&[Variant::from(result_row_label(row))]);
	}
}

/// Leaving the previous run's rows in place made NVDA enumerate all of them when the list was shown and focused again.
#[cfg(not(target_os = "macos"))]
pub(super) fn clear_results_list(list: ResultsList) {
	list.set_item_count(0);
}

#[cfg(target_os = "macos")]
pub(super) fn clear_results_list(list: ResultsList) {
	list.delete_all_items();
}

#[cfg(not(target_os = "macos"))]
pub(super) fn select_results_row(list: ResultsList, index: i32) {
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
pub(super) fn select_results_row(list: ResultsList, index: i32) {
	let Ok(index) = usize::try_from(index) else { return };
	list.select_row(index);
	if let Some(item) = list.row_to_item(index) {
		list.set_current_item(&item);
		list.ensure_visible(&item);
	}
}

#[cfg(not(target_os = "macos"))]
pub(super) fn results_selected_index(list: ResultsList) -> Option<usize> {
	let index = list.get_first_selected_item();
	if index >= 0 { usize::try_from(index).ok() } else { None }
}

#[cfg(target_os = "macos")]
pub(super) fn results_selected_index(list: ResultsList) -> Option<usize> {
	list.get_selected_row()
}
