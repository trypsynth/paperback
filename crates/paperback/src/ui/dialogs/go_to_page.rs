use std::{cell::Cell, rc::Rc};

use patois::t;
use wxdragon::prelude::*;

use super::DIALOG_PADDING;

pub fn show_go_to_page_dialog(
	parent: &Frame,
	current_page: i32,
	max_page: i32,
	live_region_label: StaticText,
) -> Option<i32> {
	let max_page = max_page.max(1);
	// TRANSLATORS: Title of the Go to page dialog
	let dialog_title = t("Go to page");
	let dialog = Dialog::builder(parent, &dialog_title).build();
	// TRANSLATORS: Label/prompt template for the page selection dialog. The %d placeholders represent current_page and max_pages respectively.
	let label_template = t("Go to page (%d/%d):");
	let label_text = label_template.replacen("%d", &current_page.clamp(1, max_page).to_string(), 1).replacen(
		"%d",
		&max_page.to_string(),
		1,
	);
	let label = StaticText::builder(&dialog).with_label(&label_text).build();
	let current = current_page.clamp(1, max_page);
	// A plain text field rather than a spin control: out-of-range pages must be rejected,
	// not silently clamped, and (later) relative +n/-n input has no spinner equivalent.
	let page_ctrl =
		TextCtrl::builder(&dialog).with_value(&current.to_string()).with_style(TextCtrlStyle::ProcessEnter).build();
	let result = Rc::new(Cell::new(None::<i32>));
	// Enter in the field and the Go button both submit through the same validation.
	let page_ctrl_for_enter = page_ctrl;
	let result_for_enter = Rc::clone(&result);
	let dialog_for_enter = dialog;
	page_ctrl_for_enter.bind_internal(EventType::TEXT_ENTER, move |event| {
		event.skip(false);
		submit_go_to_page(
			dialog_for_enter,
			page_ctrl_for_enter,
			&result_for_enter,
			live_region_label,
			current_page,
			max_page,
		);
	});
	// TRANSLATORS: Label for the button that jumps to the entered page
	let ok_button = Button::builder(&dialog).with_label(&t("Go")).build();
	// TRANSLATORS: Label for the button that closes the dialog without navigating
	let cancel_button = Button::builder(&dialog).with_id(ID_CANCEL).with_label(&t("Cancel")).build();
	dialog.set_escape_id(ID_CANCEL);
	ok_button.set_default();
	let page_ctrl_for_ok = page_ctrl;
	let result_for_ok = Rc::clone(&result);
	let dialog_for_ok = dialog;
	ok_button.on_click(move |_| {
		submit_go_to_page(dialog_for_ok, page_ctrl_for_ok, &result_for_ok, live_region_label, current_page, max_page);
	});
	let page_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	page_sizer.add(&label, 0, SizerFlag::AlignCenterVertical | SizerFlag::Right, 5);
	page_sizer.add(&page_ctrl, 1, SizerFlag::Expand, 0);
	let button_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	button_sizer.add_stretch_spacer(1);
	button_sizer.add(&ok_button, 0, SizerFlag::Right, DIALOG_PADDING);
	button_sizer.add(&cancel_button, 0, SizerFlag::Right, DIALOG_PADDING);
	let content_sizer = BoxSizer::builder(Orientation::Vertical).build();
	content_sizer.add_sizer(&page_sizer, 0, SizerFlag::Expand | SizerFlag::All, DIALOG_PADDING);
	content_sizer.add_sizer(&button_sizer, 0, SizerFlag::Expand | SizerFlag::Bottom | SizerFlag::Right, DIALOG_PADDING);
	dialog.set_sizer_and_fit(content_sizer, true);
	dialog.centre();
	page_ctrl.set_focus();
	page_ctrl.select_all();
	if dialog.show_modal() == ID_OK { result.get() } else { None }
}

/// Resolves the entered `text` to a page number in `1..=max_page`, or `None` when the text
/// is not a valid page expression or resolves out of range.
///
/// A bare number is the page itself; a leading `+`/`-` moves that many pages forward or
/// backward from `current_page`, so "+5" is five pages ahead and "-3" three pages back.
fn resolve_page(text: &str, current_page: i32, max_page: i32) -> Option<i32> {
	let trimmed = text.trim();
	if trimmed.is_empty() {
		return None;
	}
	let (sign, digits) = match trimmed.as_bytes()[0] {
		b'+' | b'-' => (trimmed.as_bytes()[0], &trimmed[1..]),
		_ => (0, trimmed),
	};
	let amount = digits.parse::<i64>().ok()?;
	let current = i64::from(current_page);
	let resolved = match sign {
		b'+' => current.checked_add(amount)?,
		b'-' => current.checked_sub(amount)?,
		_ => amount,
	};
	if (1..=i64::from(max_page)).contains(&resolved) { i32::try_from(resolved).ok() } else { None }
}

/// Validates the field and either confirms the dialog with the resolved page or, when the
/// entry is invalid, announces the rejection and keeps the dialog open so the user can retry.
fn submit_go_to_page(
	dialog: Dialog,
	page_ctrl: TextCtrl,
	result: &Rc<Cell<Option<i32>>>,
	live_region_label: StaticText,
	current_page: i32,
	max_page: i32,
) {
	let Some(page) = resolve_page(&page_ctrl.get_value(), current_page, max_page) else {
		// TRANSLATORS: Announced when the entered page number is outside the document's range
		live_region::announce(live_region_label, &t("Page out of range."));
		page_ctrl.set_focus();
		page_ctrl.select_all();
		return;
	};
	result.set(Some(page));
	dialog.end_modal(ID_OK);
}

#[cfg(test)]
mod tests {
	use super::resolve_page;

	#[test]
	fn absolute_pages_resolve_and_reject_out_of_range() {
		assert_eq!(resolve_page("5", 10, 100), Some(5));
		assert_eq!(resolve_page(" 42 ", 10, 100), Some(42));
		assert_eq!(resolve_page("0", 10, 100), None);
		assert_eq!(resolve_page("101", 10, 100), None);
		assert_eq!(resolve_page("99999999999", 10, 100), None);
	}

	#[test]
	fn relative_offsets_resolve_from_the_current_page() {
		assert_eq!(resolve_page("+5", 10, 100), Some(15));
		assert_eq!(resolve_page("-3", 10, 100), Some(7));
		assert_eq!(resolve_page("+0", 10, 100), Some(10));
		assert_eq!(resolve_page("-0", 10, 100), Some(10));
	}

	#[test]
	fn relative_offsets_out_of_range_are_rejected() {
		assert_eq!(resolve_page("-20", 10, 100), None);
		assert_eq!(resolve_page("+100", 10, 100), None);
	}

	#[test]
	fn garbage_is_rejected() {
		assert_eq!(resolve_page("", 10, 100), None);
		assert_eq!(resolve_page("   ", 10, 100), None);
		assert_eq!(resolve_page("abc", 10, 100), None);
		assert_eq!(resolve_page("5+3", 10, 100), None);
		assert_eq!(resolve_page("+", 10, 100), None);
		assert_eq!(resolve_page("-", 10, 100), None);
	}
}
