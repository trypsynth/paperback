use patois::t;
use wx_utils::{add_yes_no_footer, build_yes_no_buttons, dialog_padding};
use wxdragon::prelude::*;

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
