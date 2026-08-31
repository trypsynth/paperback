use std::{fmt::Write, path::Path};

use paperback_core::document::DocumentStats;
use patois::t;
use wx_utils::dpi;
use wxdragon::prelude::*;

use super::{DIALOG_PADDING, add_single_button_footer};

const DOC_INFO_WIDTH: i32 = 600;
const DOC_INFO_HEIGHT: i32 = 400;

pub fn show_document_info_dialog(parent: &Frame, path: &Path, title: &str, author: &str, stats: &DocumentStats) {
	// TRANSLATORS: Title of the Document Info dialog
	let dialog_title = t("Document Info");
	let dialog = Dialog::builder(parent, &dialog_title).build();
	dialog.set_escape_id(ID_CANCEL);
	let info_ctrl = TextCtrl::builder(&dialog)
		.with_style(TextCtrlStyle::MultiLine | TextCtrlStyle::ReadOnly)
		.with_size(dpi::scale_size(&dialog, Size::new(DOC_INFO_WIDTH, DOC_INFO_HEIGHT)))
		.build();
	// TRANSLATORS: Label for the document's file path
	let path_label = t("Path:");
	// TRANSLATORS: Label for the document's title
	let title_label = t("Title:");
	// TRANSLATORS: Label for the document's author
	let author_label = t("Author:");
	// TRANSLATORS: Label for the number of words in the document
	let words_label = t("Words:");
	// TRANSLATORS: Label for the number of lines in the document
	let lines_label = t("Lines:");
	// TRANSLATORS: Label for the number of characters in the document
	let characters_label = t("Characters:");
	// TRANSLATORS: Label for the number of characters in the document excluding space characters
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
	// TRANSLATORS: Label for a button that closes the Document Info dialog
	let ok_label = t("Close");
	let ok_button = Button::builder(&dialog).with_id(ID_CANCEL).with_label(&ok_label).build();
	let content_sizer = BoxSizer::builder(Orientation::Vertical).build();
	content_sizer.add(&info_ctrl, 1, SizerFlag::Expand | SizerFlag::All, DIALOG_PADDING);
	add_single_button_footer(content_sizer, ok_button);
	dialog.set_sizer_and_fit(content_sizer, true);
	dialog.centre();
	dialog.show_modal();
}
