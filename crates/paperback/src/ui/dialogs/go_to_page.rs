use wxdragon::{prelude::*, translations::translate as t};

use super::DIALOG_PADDING;

pub fn show_go_to_page_dialog(parent: &Frame, current_page: i32, max_page: i32) -> Option<i32> {
	let max_page = max_page.max(1);
	let dialog_title = t("Go to page");
	let dialog = Dialog::builder(parent, &dialog_title).build();
	let label_template = t("Go to page (%d/%d):");
	let label_text = label_template.replacen("%d", &current_page.clamp(1, max_page).to_string(), 1).replacen(
		"%d",
		&max_page.to_string(),
		1,
	);
	let label = StaticText::builder(&dialog).with_label(&label_text).build();
	let current = current_page.clamp(1, max_page);
	let page_ctrl = SpinCtrl::builder(&dialog)
		.with_range(1, max_page)
		.with_style(SpinCtrlStyle::Default | SpinCtrlStyle::ProcessEnter)
		.build();
	page_ctrl.set_value(current);
	let dialog_for_enter = dialog;
	page_ctrl.bind_internal(EventType::TEXT_ENTER, move |event| {
		event.skip(false);
		dialog_for_enter.end_modal(wxdragon::id::ID_OK);
	});
	let label_for_update = label;
	let label_template_for_update = label_template;
	page_ctrl.on_value_changed(move |event| {
		let text = label_template_for_update.replacen("%d", &event.get_value().to_string(), 1).replacen(
			"%d",
			&max_page.to_string(),
			1,
		);
		label_for_update.set_label(&text);
	});
	let page_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	page_sizer.add(&label, 0, SizerFlag::AlignCenterVertical | SizerFlag::Right, 5);
	page_sizer.add(&page_ctrl, 1, SizerFlag::Expand, 0);
	let ok_button = Button::builder(&dialog).with_id(wxdragon::id::ID_OK).with_label(&t("OK")).build();
	let cancel_button = Button::builder(&dialog).with_id(wxdragon::id::ID_CANCEL).with_label(&t("Cancel")).build();
	dialog.set_escape_id(wxdragon::id::ID_CANCEL);
	dialog.set_affirmative_id(wxdragon::id::ID_OK);
	let content_sizer = BoxSizer::builder(Orientation::Vertical).build();
	content_sizer.add_sizer(&page_sizer, 0, SizerFlag::Expand | SizerFlag::All, DIALOG_PADDING);
	let button_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	button_sizer.add_stretch_spacer(1);
	button_sizer.add(&ok_button, 0, SizerFlag::All, DIALOG_PADDING);
	button_sizer.add(&cancel_button, 0, SizerFlag::All, DIALOG_PADDING);
	content_sizer.add_sizer(&button_sizer, 0, SizerFlag::Expand, 0);
	dialog.set_sizer_and_fit(content_sizer, true);
	dialog.centre();
	page_ctrl.set_focus();
	if dialog.show_modal() == wxdragon::id::ID_OK { Some(page_ctrl.value().clamp(1, max_page)) } else { None }
}
