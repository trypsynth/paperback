use patois::t;
use wxdragon::prelude::*;

use super::{DIALOG_PADDING, add_ok_cancel_footer, bind_enter_confirms, build_ok_cancel_buttons};

/// Dialog for choosing a page range to batch-OCR. Pre-fills the whole document
/// (start = 1, end = last page) since "OCR everything" is the common case.
pub fn show_batch_ocr_dialog(parent: &Frame, max_page: i32) -> Option<(i32, i32)> {
	let max_page = max_page.max(1);
	// TRANSLATORS: Title of the Batch OCR dialog
	let dialog_title = t("Batch OCR");
	let dialog = Dialog::builder(parent, &dialog_title).build();
	// TRANSLATORS: Label for the starting page field of the Batch OCR range
	let start_label = StaticText::builder(&dialog).with_label(&t("&Start page:")).build();
	let start_ctrl = SpinCtrl::builder(&dialog)
		.with_range(1, max_page)
		.with_style(SpinCtrlStyle::Default | SpinCtrlStyle::ProcessEnter)
		.build();
	start_ctrl.set_value(1);
	bind_enter_confirms(&dialog, start_ctrl);
	// TRANSLATORS: Label for the ending page field of the Batch OCR range
	let end_label = StaticText::builder(&dialog).with_label(&t("&End page:")).build();
	let end_ctrl = SpinCtrl::builder(&dialog)
		.with_range(1, max_page)
		.with_style(SpinCtrlStyle::Default | SpinCtrlStyle::ProcessEnter)
		.build();
	end_ctrl.set_value(max_page);
	bind_enter_confirms(&dialog, end_ctrl);
	let start_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	start_sizer.add(&start_label, 0, SizerFlag::AlignCenterVertical | SizerFlag::Right, 5);
	start_sizer.add(&start_ctrl, 1, SizerFlag::Expand, 0);
	let end_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	end_sizer.add(&end_label, 0, SizerFlag::AlignCenterVertical | SizerFlag::Right, 5);
	end_sizer.add(&end_ctrl, 1, SizerFlag::Expand, 0);
	// TRANSLATORS: Label for the button that starts the batch OCR
	let (ok_button, cancel_button) = build_ok_cancel_buttons(&dialog, &t("OCR"));
	let content_sizer = BoxSizer::builder(Orientation::Vertical).build();
	content_sizer.add_sizer(&start_sizer, 0, SizerFlag::Expand | SizerFlag::All, DIALOG_PADDING);
	content_sizer.add_sizer(&end_sizer, 0, SizerFlag::Expand | SizerFlag::All, DIALOG_PADDING);
	add_ok_cancel_footer(content_sizer, ok_button, cancel_button);
	dialog.set_sizer_and_fit(content_sizer, true);
	dialog.centre();
	start_ctrl.set_focus();
	if dialog.show_modal() == ID_OK {
		Some((start_ctrl.value().clamp(1, max_page), end_ctrl.value().clamp(1, max_page)))
	} else {
		None
	}
}
