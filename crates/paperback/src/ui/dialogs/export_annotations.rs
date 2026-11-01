use std::{rc::Rc, sync::Mutex};

use paperback_core::{config::ConfigManager, export::annotations::AnnotationOptions, types::BookmarkFilterType};
use patois::t;
use wxdragon::prelude::*;

use super::{DIALOG_PADDING, add_ok_cancel_footer, build_ok_cancel_buttons};

pub struct AnnotationsExportChoice {
	pub filter: BookmarkFilterType,
	pub options: AnnotationOptions,
}

const FILTER_KEY: &str = "export_annotations_filter";
const PASSAGE_KEY: &str = "export_annotations_include_passage";
const NUMBERED_KEY: &str = "export_annotations_numbered";

pub fn show_export_annotations_dialog(
	parent: &Frame,
	config: &Rc<Mutex<ConfigManager>>,
) -> Option<AnnotationsExportChoice> {
	let (filter_index, include_passage, numbered) = {
		let cfg = config.lock().unwrap();
		(cfg.get_app_int(FILTER_KEY, 0), cfg.get_app_bool(PASSAGE_KEY, true), cfg.get_app_bool(NUMBERED_KEY, false))
	};
	// TRANSLATORS: Title of the dialog asking what to include when exporting notes and bookmarks as text or HTML
	let dialog = Dialog::builder(parent, &t("Export notes and bookmarks")).build();
	// TRANSLATORS: Label for the dropdown choosing which entries to export
	let what_label = StaticText::builder(&dialog).with_label(&t("&Export:")).build();
	let what_choice = Choice::builder(&dialog).build();
	// TRANSLATORS: Option to export both bookmarks and notes
	what_choice.append(&t("Bookmarks and notes"));
	// TRANSLATORS: Option to export only bookmarks that have no note
	what_choice.append(&t("Bookmarks only"));
	// TRANSLATORS: Option to export only bookmarks that have a note
	what_choice.append(&t("Notes only"));
	what_choice.set_selection(u32::try_from(filter_index).ok().filter(|&i| i < 3).unwrap_or(0));
	// TRANSLATORS: Checkbox to include the bookmarked text next to each note
	let passage_label = t("Include the &passage each note is attached to");
	let passage_check = CheckBox::builder(&dialog).with_label(&passage_label).build();
	passage_check.set_value(include_passage);
	// TRANSLATORS: Checkbox to number the exported entries
	let number_check = CheckBox::builder(&dialog).with_label(&t("&Number the entries")).build();
	number_check.set_value(numbered);
	// TRANSLATORS: Label for the button that continues to the save dialog
	let (ok_button, cancel_button) = build_ok_cancel_buttons(&dialog, &t("Export"));
	let what_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	what_sizer.add(&what_label, 0, SizerFlag::AlignCenterVertical | SizerFlag::Right, 5);
	what_sizer.add(&what_choice, 1, SizerFlag::Expand, 0);
	let content_sizer = BoxSizer::builder(Orientation::Vertical).build();
	content_sizer.add_sizer(&what_sizer, 0, SizerFlag::Expand | SizerFlag::All, DIALOG_PADDING);
	content_sizer.add(&passage_check, 0, SizerFlag::All, DIALOG_PADDING);
	content_sizer.add(&number_check, 0, SizerFlag::All, DIALOG_PADDING);
	add_ok_cancel_footer(content_sizer, ok_button, cancel_button);
	dialog.set_sizer_and_fit(content_sizer, true);
	dialog.centre();
	what_choice.set_focus();
	if dialog.show_modal() != ID_OK {
		return None;
	}
	let index = what_choice.get_selection().filter(|&i| i < 3).unwrap_or(0);
	let filter = match index {
		1 => BookmarkFilterType::BookmarksOnly,
		2 => BookmarkFilterType::NotesOnly,
		_ => BookmarkFilterType::All,
	};
	{
		let cfg = config.lock().unwrap();
		cfg.set_app_int(FILTER_KEY, i32::try_from(index).unwrap_or(0));
		cfg.set_app_bool(PASSAGE_KEY, passage_check.is_checked());
		cfg.set_app_bool(NUMBERED_KEY, number_check.is_checked());
	}
	Some(AnnotationsExportChoice {
		filter,
		options: AnnotationOptions { include_passage: passage_check.is_checked(), numbered: number_check.is_checked() },
	})
}
