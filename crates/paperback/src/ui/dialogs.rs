// `build_ok_cancel_buttons` translates its own "Cancel" label via patois, since
// wx-utils is itself a `[package.metadata.patois] translatable = true` crate whose
// `t()` calls patois-build folds into this app's own catalog at build time (see
// `build_translations` in build.rs) — the same mechanism ship-shape uses. `ok_label`
// is still supplied by each dialog, since it varies ("OK" vs. a verb like "Go").
//
// Not a fit for every dialog: `toc.rs` deliberately gives its OK button a non-stock
// ID so a custom handler can block ending the modal until a selection is made, and
// `options.rs`'s `prompt_for_hotkey` parents its buttons to a `Panel` (with an extra
// "Clear" button) rather than the dialog directly. Both build their buttons by hand
// instead of calling this.
pub(super) use wx_utils::{
	DIALOG_PADDING, add_ok_cancel_footer, add_single_button_footer, bind_enter_confirms, build_ok_cancel_buttons,
};

mod about;
pub use about::show_about_dialog;
mod all_documents;
pub use all_documents::show_all_documents_dialog;
mod bookmark;
pub use bookmark::show_bookmark_dialog;
mod document_info;
pub use document_info::show_document_info_dialog;
mod duration_format;
mod elements;
pub use elements::show_elements_dialog;
mod go_to;
mod go_to_line;
pub use go_to_line::show_go_to_line_dialog;
mod go_to_page;
pub use go_to_page::show_go_to_page_dialog;
mod go_to_percent;
pub use go_to_percent::show_go_to_percent_dialog;
#[cfg(target_os = "linux")]
mod linux_setup;
#[cfg(target_os = "linux")]
pub use linux_setup::{AssociationChoice, show_linux_setup_dialog};
mod note_entry;
pub use note_entry::show_note_entry_dialog;
mod open_as;
pub use open_as::show_open_as_dialog;
mod options;
pub(crate) use options::AUDIO_SEEK_AMOUNTS_SECONDS;
pub use options::show_options_dialog;
mod shortcuts;
pub use shortcuts::prompt_for_shortcuts;
mod sleep_timer;
pub use sleep_timer::show_sleep_timer_dialog;
mod toc;
pub use toc::show_toc_dialog;
mod view_note;
pub use view_note::show_view_note_dialog;
mod web_view;
#[cfg(any(target_os = "linux", target_os = "windows"))]
pub use web_view::ACTIVE_WEB_VIEW;
pub use web_view::show_web_view_dialog;
mod word_count;
pub use word_count::{AudioOnlySummary, show_word_count_dialog};
