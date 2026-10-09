use patois::t;
use wx_utils::{add_yes_no_footer, build_yes_no_buttons, dialog_padding};
use wxdragon::prelude::*;

use super::{DIALOG_PADDING, add_ok_cancel_footer, bind_enter_submits, build_ok_cancel_buttons};

/// Asks for the links to open documents from, one per line, starting with `initial`. Enter opens
/// them and Shift+Enter starts a new line.
pub fn show_open_url_dialog(parent: &Frame, initial: &str) -> Option<String> {
	// TRANSLATORS: Title of the dialog that asks for web links to open documents from
	let dialog = Dialog::builder(parent, &t("Open from URL")).build();
	// TRANSLATORS: Label of the field in Open from URL; each line is one web link
	let label_text = t("&Links, one per line:");
	let label = StaticText::builder(&dialog).with_label(&label_text).build();
	let links_ctrl = TextCtrl::builder(&dialog)
		.with_value(initial)
		.with_style(TextCtrlStyle::MultiLine)
		.with_size(dialog.from_dip(Size::new(500, 150)))
		.build();
	#[cfg(target_os = "macos")]
	links_ctrl.set_accessibility_label(label_text.replace('&', "").trim_end_matches(':').trim());
	// TRANSLATORS: Button in Open from URL that downloads and opens the documents the links name
	let (ok_button, cancel_button) = build_ok_cancel_buttons(&dialog, &t("&Open"));
	bind_enter_submits(dialog, links_ctrl);
	let content_sizer = BoxSizer::builder(Orientation::Vertical).build();
	content_sizer.add(&label, 0, SizerFlag::All, DIALOG_PADDING);
	content_sizer.add(
		&links_ctrl,
		1,
		SizerFlag::Expand | SizerFlag::Left | SizerFlag::Right | SizerFlag::Bottom,
		DIALOG_PADDING,
	);
	add_ok_cancel_footer(content_sizer, ok_button, cancel_button);
	dialog.set_sizer_and_fit(content_sizer, true);
	dialog.centre();
	links_ctrl.set_focus();
	links_ctrl.set_insertion_point_end();
	if dialog.show_modal() == ID_OK { Some(links_ctrl.get_value()) } else { None }
}

/// The security warning before downloading a link, with No as the default answer.
pub fn confirm_download(parent: &dyn WxWidget, warning: &str) -> bool {
	// TRANSLATORS: Title of the security warning shown before downloading a document from a link
	let dialog = Dialog::builder(parent, &t("Security Warning")).build();
	let padding = dialog_padding(&dialog);
	let label = StaticText::builder(&dialog).with_label(warning).build();
	let (yes_button, no_button) = build_yes_no_buttons(&dialog);
	no_button.set_default();
	let content = BoxSizer::builder(Orientation::Vertical).build();
	content.add(&label, 0, SizerFlag::All, padding);
	add_yes_no_footer(content, yes_button, no_button);
	dialog.set_sizer_and_fit(content, true);
	dialog.centre();
	no_button.set_focus();
	dialog.show_modal() == ID_YES
}
