//! The pieces the Go to line, page and percent dialogs share: a text entry that takes a
//! bare number or a relative `+n`/`-n` offset, validation that keeps the dialog open and
//! speaks a rejection when the entry is out of range, and the Go/Cancel footer.

use std::{cell::Cell, rc::Rc};

use patois::t;
use wxdragon::prelude::*;

/// The numeric entry of a Go to dialog together with the range it validates against and
/// the value it resolved to when the dialog was confirmed.
pub(super) struct NumberEntry {
	pub ctrl: TextCtrl,
	current: i32,
	min: i32,
	max: i32,
	result: Rc<Cell<Option<i32>>>,
}

impl NumberEntry {
	/// Builds the entry seeded with `current`, accepting only the keystrokes a number
	/// expression can contain. A plain text field rather than a spin control: out-of-range
	/// values must be rejected, not silently clamped, and relative `+n`/`-n` input has no
	/// spinner equivalent.
	pub fn new(dialog: Dialog, current: i32, min: i32, max: i32) -> Self {
		let ctrl =
			TextCtrl::builder(&dialog).with_value(&current.to_string()).with_style(TextCtrlStyle::ProcessEnter).build();
		restrict_to_number_input(ctrl);
		Self { ctrl, current, min, max, result: Rc::new(Cell::new(None)) }
	}

	/// The number the field currently resolves to, or `None` when it is invalid or out of range.
	pub fn resolve(&self) -> Option<i32> {
		resolve_number(&self.ctrl.get_value(), self.current, self.min, self.max)
	}

	/// The value the dialog was confirmed with, if it was.
	pub fn result(&self) -> Option<i32> {
		self.result.get()
	}

	/// Puts the caret in the field with its contents selected, ready to be overtyped.
	pub fn focus(&self) {
		self.ctrl.set_focus();
		self.ctrl.select_all();
	}

	/// Validates the field and either confirms `dialog` with the resolved number or, when
	/// the entry is invalid, announces `out_of_range` and keeps the dialog open for a retry.
	pub fn submit(&self, dialog: Dialog, live_region_label: StaticText, out_of_range: &str) {
		let Some(value) = self.resolve() else {
			live_region::announce(live_region_label, out_of_range);
			self.focus();
			return;
		};
		self.result.set(Some(value));
		dialog.end_modal(ID_OK);
	}
}

/// What a single-field Go to dialog (line or page) shows and accepts.
pub(super) struct NumberPrompt<'a> {
	pub title: &'a str,
	pub label: &'a str,
	/// Announced when the entry does not resolve to a number in `min..=max`.
	pub out_of_range: &'a str,
	pub current: i32,
	pub min: i32,
	pub max: i32,
}

/// Shows a modal dialog with one labelled [`NumberEntry`] and a Go/Cancel footer, and
/// returns the number it was confirmed with.
pub(super) fn show_number_dialog(parent: &Frame, prompt: &NumberPrompt, live_region_label: StaticText) -> Option<i32> {
	let dialog = Dialog::builder(parent, prompt.title).build();
	let label = StaticText::builder(&dialog).with_label(prompt.label).build();
	let entry = Rc::new(NumberEntry::new(dialog, prompt.current, prompt.min, prompt.max));
	let entry_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	entry_sizer.add(&label, 0, SizerFlag::AlignCenterVertical | SizerFlag::Right, 5);
	entry_sizer.add(&entry.ctrl, 1, SizerFlag::Expand, 0);
	let content_sizer = BoxSizer::builder(Orientation::Vertical).build();
	content_sizer.add_sizer(&entry_sizer, 0, SizerFlag::Expand | SizerFlag::All, super::DIALOG_PADDING);
	let out_of_range = prompt.out_of_range.to_string();
	let entry_for_submit = Rc::clone(&entry);
	add_go_cancel_footer(dialog, content_sizer, entry.ctrl, move || {
		entry_for_submit.submit(dialog, live_region_label, &out_of_range);
	});
	dialog.set_sizer_and_fit(content_sizer, true);
	dialog.centre();
	entry.focus();
	if dialog.show_modal() == ID_OK { entry.result() } else { None }
}

/// Appends the Go/Cancel row to `content_sizer`. Enter in `entry` and a click on Go both
/// run `submit`, which is expected to validate and end the modal itself; Escape and
/// Cancel close the dialog without navigating.
///
/// Go gets a non-stock ID so a click does not end the modal before `submit` has had a
/// chance to reject the entry. The `wxStdDialogButtonSizer` still lays the pair out in
/// platform order, since it takes custom affirmative and cancel buttons explicitly.
pub(super) fn add_go_cancel_footer(
	dialog: Dialog,
	content_sizer: BoxSizer,
	entry: TextCtrl,
	submit: impl Fn() + 'static,
) {
	let submit = Rc::new(submit);
	// TRANSLATORS: Label for the button that jumps to the entered position (a line, page, or percentage, depending on the dialog)
	let go_button = Button::builder(&dialog).with_label(&t("Go")).build();
	// TRANSLATORS: Label for the cancellation button
	let cancel_button = Button::builder(&dialog).with_id(ID_CANCEL).with_label(&t("Cancel")).build();
	dialog.set_escape_id(ID_CANCEL);
	go_button.set_default();
	let submit_for_enter = Rc::clone(&submit);
	entry.bind_internal(EventType::TEXT_ENTER, move |event| {
		event.skip(false);
		submit_for_enter();
	});
	go_button.on_click(move |_| submit());
	let button_sizer = StdDialogButtonSizerBuilder::new().build();
	button_sizer.set_affirmative_button(&go_button);
	button_sizer.set_cancel_button(&cancel_button);
	button_sizer.realize();
	content_sizer.add_sizer(&button_sizer, 0, SizerFlag::Expand | SizerFlag::All, super::DIALOG_PADDING);
}

/// Resolves the entered `text` to a number in `min..=max`, or `None` when the text is not
/// a valid number expression or resolves out of range.
///
/// A bare number is the value itself; a leading `+`/`-` moves that far forward or
/// backward from `current`, so "+5" is five past the current value and "-3" three before.
pub(super) fn resolve_number(text: &str, current: i32, min: i32, max: i32) -> Option<i32> {
	let trimmed = text.trim();
	if trimmed.is_empty() {
		return None;
	}
	let (sign, digits) = match trimmed.as_bytes()[0] {
		b'+' | b'-' => (trimmed.as_bytes()[0], &trimmed[1..]),
		_ => (0, trimmed),
	};
	let amount = digits.parse::<i64>().ok()?;
	let current = i64::from(current);
	let resolved = match sign {
		b'+' => current.checked_add(amount)?,
		b'-' => current.checked_sub(amount)?,
		_ => amount,
	};
	if (i64::from(min)..=i64::from(max)).contains(&resolved) { i32::try_from(resolved).ok() } else { None }
}

/// Swallows any keystroke that cannot be part of a number expression: digits, `+`/`-`
/// (for relative jumps), and Enter (which submits). Letters and punctuation never reach
/// the field, so an invalid entry can't be typed in the first place.
///
/// Only `CHAR` events are filtered, which fire for character insertion. Everything that
/// is not a printable Latin-1 character passes through untouched, so every navigation,
/// editing and shortcut key keeps working: control codes below space (Tab, Backspace,
/// Enter, Escape), Delete, the special keys at [`WXK_START`] and above (arrows, Home/End,
/// Page keys, Insert, F-keys, numpad Enter), and all Ctrl/Cmd shortcuts. Characters
/// outside Latin-1 report no key code and slip through too; submit still rejects them.
fn restrict_to_number_input(ctrl: TextCtrl) {
	ctrl.bind_internal(EventType::CHAR, move |event| {
		let key = event.get_key_code().unwrap_or(0);
		let allowed = event.control_down()
			|| event.cmd_down()
			|| key < i32::from(b' ')
			|| key == WXK_DELETE
			|| key >= WXK_START
			|| (i32::from(b'0')..=i32::from(b'9')).contains(&key)
			|| key == i32::from(b'+')
			|| key == i32::from(b'-');
		event.skip(allowed);
	});
}

#[cfg(test)]
mod tests {
	use super::resolve_number;

	#[test]
	fn absolute_values_resolve_and_reject_out_of_range() {
		assert_eq!(resolve_number("5", 10, 1, 100), Some(5));
		assert_eq!(resolve_number(" 42 ", 10, 1, 100), Some(42));
		assert_eq!(resolve_number("0", 10, 1, 100), None);
		assert_eq!(resolve_number("0", 40, 0, 100), Some(0));
		assert_eq!(resolve_number("100", 40, 0, 100), Some(100));
		assert_eq!(resolve_number("101", 10, 1, 100), None);
		assert_eq!(resolve_number("99999999999", 10, 1, 100), None);
	}

	#[test]
	fn relative_offsets_resolve_from_the_current_value() {
		assert_eq!(resolve_number("+5", 10, 1, 100), Some(15));
		assert_eq!(resolve_number("-3", 10, 1, 100), Some(7));
		assert_eq!(resolve_number("+0", 10, 1, 100), Some(10));
		assert_eq!(resolve_number("-0", 10, 1, 100), Some(10));
	}

	#[test]
	fn relative_offsets_out_of_range_are_rejected() {
		assert_eq!(resolve_number("-20", 10, 1, 100), None);
		assert_eq!(resolve_number("+100", 10, 1, 100), None);
		assert_eq!(resolve_number("-60", 45, 0, 100), None);
	}

	#[test]
	fn garbage_is_rejected() {
		assert_eq!(resolve_number("", 10, 1, 100), None);
		assert_eq!(resolve_number("   ", 10, 1, 100), None);
		assert_eq!(resolve_number("abc", 10, 1, 100), None);
		assert_eq!(resolve_number("5+3", 10, 1, 100), None);
		assert_eq!(resolve_number("+", 10, 1, 100), None);
		assert_eq!(resolve_number("-", 10, 1, 100), None);
	}
}
