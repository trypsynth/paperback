use patois::t;
use wxdragon::prelude::*;

// Shared constants used by sub-modules that import via `use super::`.
pub(super) const DIALOG_PADDING: i32 = 10;
pub(super) const KEY_RETURN: i32 = 13;
pub(super) const KEY_NUMPAD_ENTER: i32 = 370;
pub(super) const KEY_DELETE: i32 = 127;
pub(super) const KEY_NUMPAD_DELETE: i32 = 330;

/// Binds `ctrl`'s Enter key (`TEXT_ENTER`) to confirm `dialog` as if its OK button were
/// clicked. Shared by the module's single-field entry dialogs (Go to Line/Page/Percent,
/// Sleep Timer), where pressing Enter in the spin control should submit the dialog.
pub(super) fn bind_enter_confirms<W: WxEvtHandler>(dialog: Dialog, ctrl: W) {
	ctrl.bind_internal(EventType::TEXT_ENTER, move |event| {
		event.skip(false);
		dialog.end_modal(ID_OK);
	});
}

/// Builds a standard OK/Cancel button pair, both parented to `dialog` with the stock
/// `ID_OK`/`ID_CANCEL` IDs (so a plain click ends the modal without extra wiring,
/// unless the caller attaches its own `on_click` to override that — as `elements.rs`
/// does to validate a selection first). Also wires `dialog`'s Escape key and
/// affirmative (Enter-default) behavior to Cancel/OK, and makes OK the visual
/// default button. `ok_label` lets callers use "OK" or a verb like "Go" where that
/// reads better.
///
/// Not a fit for every dialog: `toc.rs` deliberately gives its OK button a
/// non-stock ID so a custom handler can block ending the modal until a selection is
/// made, and `options.rs`'s `prompt_for_hotkey` parents its buttons to a `Panel`
/// (with an extra "Clear" button) rather than the dialog directly. Both build their
/// buttons by hand instead of calling this.
pub(super) fn build_ok_cancel_buttons(dialog: Dialog, ok_label: &str) -> (Button, Button) {
	let ok_button = Button::builder(&dialog).with_id(ID_OK).with_label(ok_label).build();
	// TRANSLATORS: Label for the cancellation button
	let cancel_button = Button::builder(&dialog).with_id(ID_CANCEL).with_label(&t("Cancel")).build();
	dialog.set_escape_id(ID_CANCEL);
	dialog.set_affirmative_id(ID_OK);
	ok_button.set_default();
	(ok_button, cancel_button)
}

/// Appends an OK/Cancel button row to `content_sizer` using a native
/// `wxStdDialogButtonSizer`, which reorders the buttons and applies spacing to match
/// the platform HIG (Cancel/OK on macOS, OK/Cancel on Windows). The shape shared by
/// most of this module's simpler dialogs.
pub(super) fn add_ok_cancel_footer(content_sizer: BoxSizer, ok_button: Button, cancel_button: Button) {
	let button_sizer = StdDialogButtonSizerBuilder::new().build();
	button_sizer.add_button(&ok_button);
	button_sizer.add_button(&cancel_button);
	button_sizer.realize();
	content_sizer.add_sizer(&button_sizer, 0, SizerFlag::Expand | SizerFlag::All, DIALOG_PADDING);
}

/// Appends a single-button row (e.g. "Close") to `content_sizer` via a native
/// `wxStdDialogButtonSizer`. The single-button counterpart to
/// [`add_ok_cancel_footer`], for dialogs (Document Info, View Note) that only need a
/// dismiss action.
pub(super) fn add_single_button_footer(content_sizer: BoxSizer, button: Button) {
	let button_sizer = StdDialogButtonSizerBuilder::new().build();
	button_sizer.add_button(&button);
	button_sizer.realize();
	content_sizer.add_sizer(&button_sizer, 0, SizerFlag::Expand | SizerFlag::All, DIALOG_PADDING);
}

mod about;
pub use about::show_about_dialog;
mod all_documents;
pub use all_documents::show_all_documents_dialog;
mod bookmark;
pub use bookmark::show_bookmark_dialog;
mod document_info;
pub use document_info::show_document_info_dialog;
mod elements;
pub use elements::show_elements_dialog;
mod go_to_line;
pub use go_to_line::show_go_to_line_dialog;
mod go_to_page;
pub use go_to_page::show_go_to_page_dialog;
mod go_to_percent;
pub use go_to_percent::show_go_to_percent_dialog;
mod note_entry;
pub use note_entry::show_note_entry_dialog;
mod open_as;
pub use open_as::show_open_as_dialog;
mod options;
pub use options::show_options_dialog;
mod sleep_timer;
pub use sleep_timer::show_sleep_timer_dialog;
mod toc;
pub use toc::show_toc_dialog;
mod view_note;
pub use view_note::show_view_note_dialog;
mod web_view;
pub use web_view::{ACTIVE_WEB_VIEW, show_web_view_dialog};
mod word_count;
pub use word_count::show_word_count_dialog;
