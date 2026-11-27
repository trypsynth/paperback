//! Services > Scribe > My Documents: the documents in the reader's Scribe account, to open one again or move it to the trash.

use std::{cell::RefCell, rc::Rc};

use live_region::{Priority, announce_with_priority};
use paperback_core::scribe::DocumentSummary;
use patois::{nt, t};
use wx_utils::{
	show_error, show_info,
	tasks::{background, spawn_for},
};
use wxdragon::prelude::*;

use super::{add_ok_cancel_footer, build_ok_cancel_buttons, dialog_padding};
use crate::{scribe, ui::scribe_conversion::describe};

const LIST_WIDTH: i32 = 560;
const LIST_HEIGHT: i32 = 300;
const COLUMN_WIDTHS: [i32; 3] = [320, 90, 130];

/// The document to open, or `None`. Documents moved to the trash here are gone from the list at once.
pub fn show_scribe_documents(
	parent: &Frame,
	documents: Vec<DocumentSummary>,
	live_region: StaticText,
) -> Option<DocumentSummary> {
	// TRANSLATORS: Title of the dialog listing the documents in the reader's Scribe for Documents account
	let title = t("My Scribe Documents");
	if documents.is_empty() {
		// TRANSLATORS: Shown by Services > Scribe > My Documents when the reader's account has no documents
		show_info(parent, t("You have no documents in Scribe yet."), &title);
		return None;
	}
	let dialog = Dialog::builder(parent, &title).build();
	let _destroy = super::DestroyOnDrop(dialog);
	let padding = dialog_padding(&dialog);
	// TRANSLATORS: Label of the list of documents in the reader's Scribe account
	let list_label = StaticText::builder(&dialog).with_label(&t("&Documents:")).build();
	let list = ListCtrl::builder(&dialog)
		.with_style(ListCtrlStyle::Report | ListCtrlStyle::SingleSel)
		.with_size(dialog.from_dip(Size::new(LIST_WIDTH, LIST_HEIGHT)))
		.build();
	let columns = [
		// TRANSLATORS: Column of the My Scribe Documents list
		t("Title"),
		// TRANSLATORS: Column of the My Scribe Documents list: how many pages the document has
		t("Pages"),
		// TRANSLATORS: Column of the My Scribe Documents list: the date the document was uploaded
		t("Added"),
	];
	for (index, (heading, width)) in (0..).zip(columns.iter().zip(COLUMN_WIDTHS)) {
		list.insert_column(index, heading, ListColumnFormat::Left, dialog.from_dip_int(width));
	}
	for (index, document) in (0..).zip(&documents) {
		let name = document.title.clone().filter(|title| !title.trim().is_empty()).unwrap_or_else(|| t("Untitled"));
		list.insert_item(index, &name, None);
		let pages = document
			.page_count
			// TRANSLATORS: How many pages a document in the My Scribe Documents list has. Plural form is chosen by that number.
			.map(|pages| nt("%d page", "%d pages", pages.unsigned_abs()).replacen("%d", &pages.to_string(), 1))
			.unwrap_or_default();
		list.set_item_text_by_column(index, 1, &pages);
		// The server's timestamps are ISO 8601, so the date is its first ten characters.
		list.set_item_text_by_column(index, 2, document.inserted_at.get(..10).unwrap_or(&document.inserted_at));
	}
	select(list, 0);
	let documents = Rc::new(RefCell::new(documents));
	// Made before Open and Close so it comes after the list in the tab order, as it does on screen.
	// TRANSLATORS: Button of the My Scribe Documents dialog. Scribe keeps trashed documents for 7 days before deleting them.
	let trash_button = Button::builder(&dialog).with_label(&t("Move to &Trash")).build();
	let (open_button, close_button) = build_ok_cancel_buttons(&dialog, &t("&Open"));
	let sizer = BoxSizer::builder(Orientation::Vertical).build();
	sizer.add(&list_label, 0, SizerFlag::All, padding / 2);
	sizer.add(&list, 1, SizerFlag::Expand | SizerFlag::All, padding / 2);
	sizer.add(&trash_button, 0, SizerFlag::All, padding / 2);
	add_ok_cancel_footer(sizer, open_button, close_button);
	let trash = {
		let documents = Rc::clone(&documents);
		move || trash_selected(dialog, list, &documents, live_region)
	};
	let trash = Rc::new(trash);
	let trash_for_button = Rc::clone(&trash);
	trash_button.on_click(move |_| trash_for_button());
	list.on_item_activated(move |_| dialog.end_modal(ID_OK));
	list.bind_internal(EventType::KEY_DOWN, move |event| {
		if let Some(key) = event.get_key_code()
			&& (key == WXK_DELETE || key == WXK_NUMPAD_DELETE)
		{
			trash();
			event.skip(false);
			return;
		}
		event.skip(true);
	});
	dialog.set_sizer_and_fit(sizer, true);
	dialog.centre();
	list.set_focus();
	if dialog.show_modal() != ID_OK {
		return None;
	}
	let index = usize::try_from(selected(list)).ok()?;
	documents.borrow().get(index).cloned()
}

fn selected(list: ListCtrl) -> i32 {
	let selected = list.get_first_selected_item();
	if selected >= 0 { selected } else { list.get_next_item(-1, ListNextItemFlag::All, ListItemState::Focused) }
}

fn select(list: ListCtrl, index: i32) {
	let state = ListItemState::Selected | ListItemState::Focused;
	list.set_item_state(i64::from(index), state, state);
	list.ensure_visible(i64::from(index));
}

/// Takes the row out once Scribe confirms, found again by the document's id since the list may have changed meanwhile.
fn trash_selected(
	dialog: Dialog,
	list: ListCtrl,
	documents: &Rc<RefCell<Vec<DocumentSummary>>>,
	live_region: StaticText,
) {
	let Some(id) = usize::try_from(selected(list))
		.ok()
		.and_then(|index| documents.borrow().get(index).map(|document| document.id.clone()))
	else {
		return;
	};
	let documents = Rc::clone(documents);
	spawn_for(&dialog, async move {
		let scribe = scribe::shared();
		let trashing = id.clone();
		if let Err(error) = background(async move { scribe.trash(&trashing).await }).await {
			// TRANSLATORS: Title of the error when a document can't be moved to the trash in the My Scribe Documents dialog
			return show_error(&dialog, describe(&error), &t("Move to Trash"));
		}
		let mut documents = documents.borrow_mut();
		let Some(index) = documents.iter().position(|document| document.id == id) else { return };
		documents.remove(index);
		list.delete_item(i64::try_from(index).unwrap_or(i64::MAX));
		if documents.is_empty() {
			dialog.end_modal(ID_CANCEL);
		} else {
			select(list, i32::try_from(index.min(documents.len() - 1)).unwrap_or(0));
		}
		// TRANSLATORS: Announced after a document is moved to the trash in the My Scribe Documents dialog
		announce_with_priority(
			live_region,
			&t("Moved to the trash. Scribe deletes it for good in 7 days."),
			Priority::Low,
		);
	});
}
