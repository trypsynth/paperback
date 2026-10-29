pub(super) use wx_utils::{
	DIALOG_PADDING, add_ok_cancel_footer, add_single_button_footer, bind_enter_confirms, build_ok_cancel_buttons,
};

#[cfg(target_os = "macos")]
mod activation;
#[cfg(target_os = "macos")]
use activation::bind_enter_button;

/// NVDA counts the focused item's siblings with one cross-process message each on every tree move, so more than this under one parent are split into groups to keep arrow keys from lagging.
#[cfg(target_os = "windows")]
const MAX_TREE_SIBLINGS: usize = 500;

#[cfg(target_os = "windows")]
fn tree_group_label(first: &str, last: &str) -> String {
	// TRANSLATORS: A group of entries in the table of contents or the headings tree, when a book has too many to list under one heading. The first {} is the first entry in the group, the second the last.
	patois::t("{} to {}").replacen("{}", first, 1).replacen("{}", last, 1)
}

mod about;
pub use about::show_about_dialog;
mod batch_ocr;
pub use batch_ocr::show_batch_ocr_dialog;
mod all_documents;
pub use all_documents::show_all_documents_dialog;
mod bookmark;
pub use bookmark::show_bookmark_dialog;
mod document_info;
mod export_annotations;
pub use document_info::show_document_info_dialog;
pub use export_annotations::show_export_annotations_dialog;
mod elements;
pub use elements::{ElementsKind, show_elements_dialog};
mod go_to;
mod go_to_line;
pub use go_to_line::show_go_to_line_dialog;
mod go_to_page;
pub use go_to_page::show_go_to_page_dialog;
mod go_to_percent;
pub use go_to_percent::show_go_to_percent_dialog;
mod image_text;
pub use image_text::show_image_text_dialog;
#[cfg(target_os = "linux")]
mod linux_setup;
#[cfg(target_os = "linux")]
pub use linux_setup::{AssociationChoice, ChoiceAction, show_linux_setup_dialog};
mod note_entry;
pub use note_entry::show_note_entry_dialog;
mod open_as;
pub use open_as::show_open_as_dialog;
mod open_url;
pub use open_url::{confirm_download, show_open_url_dialog};
mod options;
pub(crate) use options::AUDIO_SEEK_AMOUNTS_SECONDS;
pub use options::show_options_dialog;
mod shortcuts;
pub use shortcuts::{Keymap, prompt_for_shortcuts};
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
mod zip_entries;
pub use zip_entries::{ZipChoice, show_zip_entries_dialog};
