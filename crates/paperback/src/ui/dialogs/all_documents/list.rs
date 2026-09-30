//! The platform-specific document list widget. wxListCtrl is not exposed to VoiceOver as an
//! accessible table on macOS, so a wxDataViewListCtrl is used there while the ListCtrl remains
//! appropriate on Windows and Linux; these accessors hide the difference from the dialog.

use wxdragon::prelude::*;

#[cfg(target_os = "macos")]
pub(super) type DocumentList = DataViewListCtrl;
#[cfg(not(target_os = "macos"))]
pub(super) type DocumentList = ListCtrl;

#[cfg(not(target_os = "macos"))]
pub(super) fn get_selected_index(list: DocumentList) -> i32 {
	let selected = list.get_first_selected_item();
	if selected >= 0 {
		return selected;
	}
	list.get_next_item(-1, ListNextItemFlag::All, ListItemState::Focused)
}

#[cfg(target_os = "macos")]
pub(super) fn get_selected_index(list: DocumentList) -> i32 {
	list.get_selected_row().and_then(|index| i32::try_from(index).ok()).unwrap_or(-1)
}

#[cfg(not(target_os = "macos"))]
pub(super) fn get_selected_indices(list: DocumentList) -> Vec<i32> {
	let mut indices = Vec::new();
	let mut next = list.get_first_selected_item();
	while next >= 0 {
		indices.push(next);
		next = list.get_next_item(i64::from(next), ListNextItemFlag::All, ListItemState::Selected);
	}
	indices
}

#[cfg(target_os = "macos")]
pub(super) fn get_selected_indices(list: DocumentList) -> Vec<i32> {
	(0..list.get_item_count())
		.filter(|&index| list.is_row_selected(index))
		.filter_map(|index| i32::try_from(index).ok())
		.collect()
}

pub(super) fn get_path_for_index(list: DocumentList, index: i32) -> Option<String> {
	if index < 0 {
		return None;
	}
	#[cfg(not(target_os = "macos"))]
	if let Ok(index_u64) = u64::try_from(index)
		&& let Some(data) = list.get_custom_data(index_u64)
		&& let Some(path) = data.as_ref().downcast_ref::<String>()
	{
		return Some(path.clone());
	}
	let path = get_document_list_text(list, index, 2);
	if path.is_empty() { None } else { Some(path) }
}

pub(super) fn get_selected_path(list: DocumentList) -> Option<String> {
	let index = get_selected_index(list);
	get_path_for_index(list, index)
}

#[cfg(not(target_os = "macos"))]
pub(super) fn document_list_item_count(list: DocumentList) -> i32 {
	list.get_item_count()
}

#[cfg(target_os = "macos")]
pub(super) fn document_list_item_count(list: DocumentList) -> i32 {
	i32::try_from(list.get_item_count()).unwrap_or(i32::MAX)
}

#[cfg(not(target_os = "macos"))]
pub(super) fn clear_document_list(list: DocumentList) {
	list.cleanup_all_custom_data();
	list.delete_all_items();
}

#[cfg(target_os = "macos")]
pub(super) fn clear_document_list(list: DocumentList) {
	list.delete_all_items();
}

#[cfg(not(target_os = "macos"))]
pub(super) fn append_document_list_item(list: DocumentList, filename: &str, status: &str, path: &str) {
	let index = i64::from(list.get_item_count());
	list.insert_item(index, filename, None);
	if let Ok(index_u64) = u64::try_from(index) {
		list.set_custom_data(index_u64, path.to_owned());
	}
	list.set_item_text_by_column(index, 1, status);
	list.set_item_text_by_column(index, 2, path);
}

#[cfg(target_os = "macos")]
pub(super) fn append_document_list_item(list: DocumentList, filename: &str, status: &str, path: &str) {
	list.append_item(&[Variant::from(filename), Variant::from(status), Variant::from(path)]);
}

#[cfg(not(target_os = "macos"))]
pub(super) fn select_document_list_item(list: DocumentList, index: i32) {
	list.set_item_state(
		i64::from(index),
		ListItemState::Selected | ListItemState::Focused,
		ListItemState::Selected | ListItemState::Focused,
	);
	list.ensure_visible(i64::from(index));
}

#[cfg(target_os = "macos")]
pub(super) fn select_document_list_item(list: DocumentList, index: i32) {
	let Ok(index) = usize::try_from(index) else { return };
	list.select_row(index);
	if let Some(item) = list.row_to_item(index) {
		list.set_current_item(&item);
		list.ensure_visible(&item);
	}
}

#[cfg(not(target_os = "macos"))]
pub(super) fn set_all_document_list_items_selected(list: DocumentList, selected: bool) {
	let state = if selected { ListItemState::Selected } else { ListItemState::default() };
	list.set_item_state(-1, state, ListItemState::Selected);
}

#[cfg(target_os = "macos")]
pub(super) fn set_all_document_list_items_selected(list: DocumentList, selected: bool) {
	if selected {
		list.select_all();
	} else {
		list.unselect_all();
	}
}

#[cfg(not(target_os = "macos"))]
pub(super) fn get_document_list_text(list: DocumentList, index: i32, column: usize) -> String {
	list.get_item_text(i64::from(index), column as i32)
}

#[cfg(target_os = "macos")]
pub(super) fn get_document_list_text(list: DocumentList, index: i32, column: usize) -> String {
	usize::try_from(index).map_or_else(|_| String::new(), |index| list.get_text_value(index, column))
}
