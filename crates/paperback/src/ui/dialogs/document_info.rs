use std::{fmt::Write, path::Path};

use paperback_core::document::DocumentStats;
use wxdragon::{prelude::*, translations::translate as t};

use super::{DIALOG_PADDING, bind_escape_to_close};

const DOC_INFO_WIDTH: i32 = 600;
const DOC_INFO_HEIGHT: i32 = 400;

pub fn show_document_info_dialog(parent: &Frame, path: &Path, title: &str, author: &str, stats: &DocumentStats) {
	let dialog_title = t("Document Info");
	let dialog = Dialog::builder(parent, &dialog_title).build();
	let info_ctrl = TextCtrl::builder(&dialog)
		.with_style(TextCtrlStyle::MultiLine | TextCtrlStyle::ReadOnly)
		.with_size(Size::new(DOC_INFO_WIDTH, DOC_INFO_HEIGHT))
		.build();
	let path_label = t("Path:");
	let title_label = t("Title:");
	let author_label = t("Author:");
	let words_label = t("Words:");
	let lines_label = t("Lines:");
	let characters_label = t("Characters:");
	let characters_no_spaces_label = t("Characters (excluding spaces):");
	let mut info = String::new();
	let _ = writeln!(info, "{path_label} {}", path.display());
	if !title.is_empty() {
		let _ = writeln!(info, "{title_label} {title}");
	}
	if !author.is_empty() {
		let _ = writeln!(info, "{author_label} {author}");
	}
	let _ = writeln!(info, "{} {}", words_label, stats.word_count);
	let _ = writeln!(info, "{lines_label} {}", stats.line_count);
	let _ = writeln!(info, "{characters_label} {}", stats.char_count);
	let _ = writeln!(info, "{characters_no_spaces_label} {}", stats.char_count_no_whitespace);
	info_ctrl.set_value(&info);
	bind_escape_to_close(&dialog, dialog);
	bind_escape_to_close(&info_ctrl, dialog);
	let ok_label = t("OK");
	let ok_button = Button::builder(&dialog).with_label(&ok_label).build();
	bind_escape_to_close(&ok_button, dialog);
	let dialog_copy = dialog;
	ok_button.on_click(move |_| {
		dialog_copy.end_modal(wxdragon::id::ID_OK);
	});
	let content_sizer = BoxSizer::builder(Orientation::Vertical).build();
	content_sizer.add(&info_ctrl, 1, SizerFlag::Expand | SizerFlag::All, DIALOG_PADDING);
	let button_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	button_sizer.add_stretch_spacer(1);
	button_sizer.add(&ok_button, 0, SizerFlag::All, DIALOG_PADDING);
	content_sizer.add_sizer(&button_sizer, 0, SizerFlag::Expand, 0);
	dialog.set_sizer_and_fit(content_sizer, true);
	dialog.centre();
	dialog.show_modal();
}
