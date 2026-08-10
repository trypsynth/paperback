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

/// Appends a right-aligned OK/Cancel button row to `content_sizer`: a stretch spacer,
/// then `ok_button`, then `cancel_button`, each padded by [`DIALOG_PADDING`] on all
/// sides. The shape shared by most of this module's simpler dialogs.
pub(super) fn add_ok_cancel_footer(content_sizer: BoxSizer, ok_button: Button, cancel_button: Button) {
	let button_sizer = BoxSizer::builder(Orientation::Horizontal).build();
	button_sizer.add_stretch_spacer(1);
	button_sizer.add(&ok_button, 0, SizerFlag::All, DIALOG_PADDING);
	button_sizer.add(&cancel_button, 0, SizerFlag::All, DIALOG_PADDING);
	content_sizer.add_sizer(&button_sizer, 0, SizerFlag::Expand, 0);
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
