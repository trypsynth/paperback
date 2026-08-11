use std::path::Path;

use patois::t;
use wxdragon::prelude::*;

use super::{DIALOG_PADDING, add_ok_cancel_footer, build_ok_cancel_buttons};

pub fn show_open_as_dialog(parent: &Frame, path: &Path) -> Option<String> {
	// TRANSLATORS: Title of the Open As dialog
	let title = t("Open As");
	let dialog = Dialog::builder(parent, &title).build();
	// TRANSLATORS: Prompt template informing the user that no parser was found for their file. The {} placeholder is replaced with the file path.
	let message_template = t("No suitable parser was found for {}.\nHow would you like to open this file?");
	let message = message_template.replace("{}", &path.display().to_string());
	let label = StaticText::builder(&dialog).with_label(&message).build();
	// TRANSLATORS: Label for the format selection dropdown
	let format_label_text = t("Open &as:");
	let format_label = StaticText::builder(&dialog).with_label(&format_label_text).build();
	let format_combo = Choice::builder(&dialog).build();
	// TRANSLATORS: Choice option to open a file as plain text
	format_combo.append(&t("Plain Text"));
	// TRANSLATORS: Choice option to open a file as HTML
	format_combo.append(&t("HTML"));
	// TRANSLATORS: Choice option to open a file as Markdown
	format_combo.append(&t("Markdown"));
	format_combo.set_selection(0);
	#[cfg(target_os = "macos")]
	format_combo.set_accessibility_label(format_label_text.replace('&', "").trim_end_matches(':').trim());
	let (ok_button, cancel_button) = build_ok_cancel_buttons(dialog, &t("OK"));
	let content_sizer = BoxSizer::builder(Orientation::Vertical).build();
	content_sizer.add(&label, 0, SizerFlag::All, DIALOG_PADDING / 2);
	let format_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	format_sizer.add(&format_label, 0, SizerFlag::AlignCenterVertical | SizerFlag::Right, DIALOG_PADDING);
	format_sizer.add(&format_combo, 1, SizerFlag::Expand, 0);
	content_sizer.add_sizer(&format_sizer, 0, SizerFlag::Expand | SizerFlag::All, DIALOG_PADDING / 2);
	add_ok_cancel_footer(content_sizer, ok_button, cancel_button);
	dialog.set_sizer_and_fit(content_sizer, true);
	dialog.centre();
	format_combo.set_focus();
	if dialog.show_modal() != ID_OK {
		return None;
	}
	let selection = format_combo.get_selection();
	let format = match selection {
		Some(1) => "html",
		Some(2) => "md",
		_ => "txt",
	};
	Some(format.to_string())
}
