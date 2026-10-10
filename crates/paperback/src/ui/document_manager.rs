use std::{
	cell::Cell,
	path::{Path, PathBuf},
	rc::Rc,
	sync::Mutex,
	time::Instant,
};

use paperback_core::{
	config::ConfigManager, parser::is_remote_url, session::DocumentSession, util::html::MATHML_STYLES,
};
use patois::t;
use wxdragon::prelude::*;

use super::navigation::{move_to_offset_and_record_history, persist_navigation_history};
use crate::{audio_player::AudioPlayer, text_window::TextWindow, ui::navigation::announce, working_copy::WorkingCopy};

mod appearance;
mod audio;
mod lifecycle;
mod ocr;
mod position;
mod reload;
mod tabs;
mod window;

use reload::{FileFingerprint, parse_settings, read_fingerprint};

pub struct DocumentTab {
	pub panel: Panel,
	pub text_ctrl: TextCtrl,
	pub session: DocumentSession,
	/// What the reader opened: a file, or for a document opened from a link the link itself. Its
	/// settings, its place in the recent documents and its tab are all found by this.
	pub file_path: PathBuf,
	/// The downloaded copy a document opened from a link is read from, deleted with the tab.
	pub working_copy: Option<WorkingCopy>,
	pub track: bool,
	pub audio_player: Option<AudioPlayer>,
	disk_fingerprint: Option<FileFingerprint>,
	/// The column an unbroken run of Up/Down presses is aiming for, so passing through a short
	/// line does not pull the caret left for good. Per tab: each document is read at its own
	/// column, and switching tabs must not carry one document's column into another.
	pub preferred_column: Cell<Option<i64>>,
	/// The document-absolute position marked as the beginning of a selection to copy from, or
	/// `None`. Per tab: a mark names a position in one document, and carrying one document's mark
	/// into another would have it name different text entirely. Document-absolute rather than a
	/// `text_ctrl` position for the same reason [`Self::window`] exists - the loaded window can be
	/// swapped out from under it between marking and copying.
	pub selection_mark: Cell<Option<i64>>,
	/// The document-absolute bounds of whatever's currently loaded into `text_ctrl`. See
	/// `text_window` - for most documents this covers the whole thing, same as before
	/// windowing existed; only huge documents actually get a partial window.
	pub window: TextWindow,
	/// The OCR job running for this tab, if one is. Written and read only on the UI thread.
	ocr_job: Option<ocr::OcrJob>,
}

pub fn title_or_filename(title: String, path: &Path) -> String {
	if title.is_empty() {
		// TRANSLATORS: Fallback document title shown in tabs and lists when a document has no title and its file name cannot be determined
		path.file_name().map_or_else(|| t("Untitled"), |s| s.to_string_lossy().to_string())
	} else {
		title
	}
}

impl DocumentTab {
	/// The file the document is read from: its working copy if it was opened from a link,
	/// otherwise its own file.
	pub fn local_path(&self) -> &Path {
		reading_path(&self.file_path, self.working_copy.as_ref())
	}
}

fn reading_path<'a>(file_path: &'a Path, working_copy: Option<&'a WorkingCopy>) -> &'a Path {
	working_copy.map_or(file_path, WorkingCopy::path)
}

pub fn display_title(tab: &DocumentTab) -> String {
	title_or_filename(tab.session.title(), tab.local_path())
}

/// The 0-based tab index a key press names, if it names one at all.
///
/// `Ctrl+1` through `Ctrl+9` name the first nine open documents in the order they were
/// opened, as they do in a browser. Only those, and only with Ctrl held and nothing else:
/// `Ctrl+Shift+digit` and `Ctrl+Alt+digit` belong to whatever the shortcut table or the
/// system has bound them to. The numpad row counts as well as the digits above it.
///
/// wxWidgets reports the macOS Command key as `control_down`, so this covers Cmd+1 there
/// without a special case - which is the same reasoning `reader_input`'s `document_edge_for_key`
/// is built on, and lands on the convention macOS users already have for their tabs.
pub fn tab_index_for_key(key: i32, control: bool, alt: bool, shift: bool) -> Option<usize> {
	if !control || alt || shift {
		return None;
	}
	// `Ctrl+0` names nothing. It is left alone rather than treated as the tenth tab, since ten
	// is a document this feature cannot reach and a key that does not work is worse than one
	// that never appeared.
	let digit = if (i32::from(b'1')..=i32::from(b'9')).contains(&key) {
		key - i32::from(b'0')
	} else if (WXK_NUMPAD1..=WXK_NUMPAD9).contains(&key) {
		key - WXK_NUMPAD1 + 1
	} else {
		return None;
	};
	usize::try_from(digit - 1).ok()
}

const POSITION_SAVE_INTERVAL_SECS: u64 = 3;

pub struct DocumentManager {
	pub(super) frame: Frame,
	notebook: Notebook,
	tabs: Vec<DocumentTab>,
	pub(super) config: Rc<Mutex<ConfigManager>>,
	live_region_label: StaticText,
	last_position_save: Cell<Option<Instant>>,
	last_sound_position: Cell<Option<i64>>,
	last_audio_seek_position: Cell<Option<i64>>,
	last_focus_in_text: Cell<bool>,
	recently_closed: Vec<PathBuf>,
	/// Set by the frame's char hook when a shortcut key is seen, and consumed by the menu
	/// dispatcher. See `menu_events::bind_key_source` for why the book's own key handler
	/// cannot do this.
	pub(super) from_keyboard: Rc<Cell<bool>>,
	/// Hands out an id per OCR job, so a worker whose tab has been closed is recognisable as stale rather than mistaken for one on a reopened tab.
	next_job_id: Cell<u64>,
}

impl DocumentManager {
	pub const fn new(
		frame: Frame,
		notebook: Notebook,
		config: Rc<Mutex<ConfigManager>>,
		live_region_label: StaticText,
		from_keyboard: Rc<Cell<bool>>,
	) -> Self {
		Self {
			frame,
			notebook,
			tabs: Vec::new(),
			config,
			live_region_label,
			last_position_save: Cell::new(None),
			last_sound_position: Cell::new(None),
			last_audio_seek_position: Cell::new(None),
			last_focus_in_text: Cell::new(true),
			recently_closed: Vec::new(),
			from_keyboard,
			next_job_id: Cell::new(0),
		}
	}

	pub fn active_tab_index(&self) -> Option<usize> {
		let selection = self.notebook.selection();
		if selection >= 0 { usize::try_from(selection).ok() } else { None }
	}

	pub fn active_tab(&self) -> Option<&DocumentTab> {
		self.active_tab_index().and_then(|i| self.tabs.get(i))
	}

	/// The column vertical navigation is aiming for in the active document, if any.
	pub fn preferred_column(&self) -> Option<i64> {
		self.active_tab().and_then(|tab| tab.preferred_column.get())
	}

	/// Records the column vertical navigation should keep aiming for in the active document.
	pub fn set_preferred_column(&self, column: Option<i64>) {
		if let Some(tab) = self.active_tab() {
			tab.preferred_column.set(column);
		}
	}

	/// The position marked as the beginning of a selection to copy from, if one is set.
	pub fn selection_mark(&self) -> Option<i64> {
		self.active_tab().and_then(|tab| tab.selection_mark.get())
	}

	/// Marks a position as the beginning of a selection to copy from, or clears the mark.
	pub fn set_selection_mark(&self, position: Option<i64>) {
		if let Some(tab) = self.active_tab() {
			tab.selection_mark.set(position);
		}
	}

	pub fn active_tab_mut(&mut self) -> Option<&mut DocumentTab> {
		self.active_tab_index().and_then(|i| self.tabs.get_mut(i))
	}

	pub fn get_tab(&self, index: usize) -> Option<&DocumentTab> {
		self.tabs.get(index)
	}

	pub const fn tab_count(&self) -> usize {
		self.tabs.len()
	}

	pub fn open_paths(&self) -> Vec<String> {
		self.tabs.iter().map(|tab| tab.file_path.to_string_lossy().to_string()).collect()
	}

	pub fn find_tab_by_path(&self, path: &Path) -> Option<usize> {
		let target = normalized_path_key(path);
		// A link only matches itself, so there is no file of any tab to look up on disk.
		if is_remote_url(&target) {
			return self.tabs.iter().position(|tab| tab.file_path == path);
		}
		self.tabs.iter().position(|tab| normalized_path_key(&tab.file_path) == target)
	}

	pub fn pop_recently_closed(&mut self) -> Option<PathBuf> {
		self.recently_closed.pop()
	}

	pub fn push_recently_closed(&mut self, path: PathBuf) {
		self.recently_closed.push(path);
	}

	pub const fn has_recently_closed(&self) -> bool {
		!self.recently_closed.is_empty()
	}

	pub const fn notebook(&self) -> &Notebook {
		&self.notebook
	}

	pub fn activate_current_link(&mut self) {
		let history_update = {
			let Some(tab) = self.active_tab_mut() else {
				return;
			};
			let pos = tab.window.to_doc(tab.text_ctrl.get_insertion_point());
			let result = tab.session.activate_link(pos);
			if !result.found {
				return;
			}
			match result.action {
				paperback_core::session::LinkAction::Internal => {
					let update = move_to_offset_and_record_history(tab, result.offset);
					tab.track.then_some(update)
				}
				paperback_core::session::LinkAction::External => {
					launch_default_browser(&result.url, BrowserLaunchFlags::Default);
					return;
				}
				paperback_core::session::LinkAction::NotFound => return,
			}
		};
		// TRANSLATORS: Announcement read by screen readers after following an internal link within the document
		announce(self.live_region_label, t("Navigated to internal link."));
		persist_navigation_history(&self.config, history_update.as_ref());
	}
	pub fn activate_current_table(&self) -> Option<String> {
		self.active_tab().and_then(|tab| {
			let pos = tab.window.to_doc(tab.text_ctrl.get_insertion_point());
			tab.session.get_table_at_position(pos)
		})
	}

	pub fn activate_current_formula(&self) -> Option<String> {
		self.active_tab().and_then(|tab| {
			let pos = tab.window.to_doc(tab.text_ctrl.get_insertion_point());
			tab.session.get_formula_at_position(pos).map(|mathml| {
				format!(
					"<style>{MATHML_STYLES}\nmath {{ font-size: 2.5em; }} body {{ display: flex; justify-content: center; margin-top: 2em; }}</style>{mathml}"
				)
			})
		})
	}
}

fn normalized_path_key(path: &Path) -> String {
	let as_given = path.to_string_lossy();
	if is_remote_url(&as_given) {
		return as_given.into_owned();
	}
	let normalized = dunce::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
	let value = normalized.to_string_lossy().to_string();
	#[cfg(target_os = "windows")]
	{
		value.to_ascii_lowercase()
	}
	#[cfg(not(target_os = "windows"))]
	{
		value
	}
}

#[cfg(test)]
mod tests {
	use std::{
		env, fs,
		path::{Path, PathBuf},
		process,
	};

	use wxdragon::prelude::{WXK_NUMPAD1, WXK_NUMPAD3, WXK_NUMPAD9};

	use super::{
		normalized_path_key, position::position_announcement, read_fingerprint, reading_path, tab_index_for_key,
	};
	use crate::working_copy::WorkingCopy;

	/// A document opened from a link is known by the link but read from its downloaded copy.
	#[test]
	fn a_tab_with_a_working_copy_is_read_from_the_copy() {
		let copy = WorkingCopy::new("book.epub").unwrap();
		assert_eq!(reading_path(Path::new("https://example.org/book.epub"), Some(&copy)), copy.path());
	}

	#[test]
	fn a_tab_without_a_working_copy_is_read_from_its_file() {
		assert_eq!(reading_path(Path::new("books/book.epub"), None), Path::new("books/book.epub"));
	}

	/// The path of a link is case-sensitive, so two links that differ only in case are two
	/// documents, even on Windows.
	#[test]
	fn a_link_is_matched_as_written() {
		assert_eq!(normalized_path_key(Path::new("https://example.org/Book.EPUB")), "https://example.org/Book.EPUB");
	}

	/// Ctrl+1 is the first document opened, and so maps to tab 0. Everything is checked against
	/// the same function both key handlers call, because the mapping is the only part of this
	/// that can be wrong without a window to look at.
	#[test]
	fn ctrl_digit_names_a_tab_by_the_order_the_documents_were_opened() {
		for (digit, expected) in [(1, 0), (2, 1), (5, 4), (9, 8)] {
			assert_eq!(tab_index_for_key(i32::from(b'0') + digit, true, false, false), Some(expected));
		}
	}

	#[test]
	fn the_numpad_row_names_the_same_tab_as_the_digits() {
		for offset in 0..9 {
			let key = WXK_NUMPAD1 + offset;
			assert_eq!(
				tab_index_for_key(key, true, false, false),
				tab_index_for_key(i32::from(b'1') + offset, true, false, false)
			);
		}
	}

	/// The digits below Ctrl belong to heading navigation and the digits under Alt to the menu
	/// bar's own mnemonics, and both must keep working.
	#[test]
	fn a_digit_without_control_is_not_a_tab() {
		assert_eq!(tab_index_for_key(i32::from(b'1'), false, false, false), None);
		assert_eq!(tab_index_for_key(i32::from(b'1'), false, true, false), None);
		assert_eq!(tab_index_for_key(i32::from(b'1'), false, false, true), None);
	}

	/// Shift and Alt chords on a digit belong to the shortcut table and to the system, not here.
	#[test]
	fn ctrl_shift_and_ctrl_alt_digits_are_left_alone() {
		assert_eq!(tab_index_for_key(i32::from(b'3'), true, false, true), None);
		assert_eq!(tab_index_for_key(i32::from(b'3'), true, true, false), None);
		assert_eq!(tab_index_for_key(WXK_NUMPAD3, true, false, true), None);
	}

	/// There is no tenth document to name, and a tenth tab is not something this feature reaches.
	#[test]
	fn ctrl_zero_names_no_tab() {
		assert_eq!(tab_index_for_key(i32::from(b'0'), true, false, false), None);
	}

	/// The other keys that share this range of codes: a Ctrl that happens to land on a neighbouring
	/// key code must not be read as a digit. Numpad 0 sits directly below Numpad 1, and the
	/// digit row's neighbours are punctuation.
	#[test]
	fn keys_beside_the_digits_name_no_tab() {
		assert_eq!(tab_index_for_key(i32::from(b'0') - 1, true, false, false), None);
		assert_eq!(tab_index_for_key(i32::from(b'9') + 1, true, false, false), None);
		assert_eq!(tab_index_for_key(WXK_NUMPAD1 - 1, true, false, false), None);
		assert_eq!(tab_index_for_key(WXK_NUMPAD9 + 1, true, false, false), None);
	}

	struct TempFile {
		path: PathBuf,
	}

	impl TempFile {
		fn with_content(name: &str, content: &[u8]) -> Self {
			let path = env::temp_dir().join(format!("paperback-fingerprint-{}-{name}", process::id()));
			fs::write(&path, content).unwrap();
			Self { path }
		}
	}

	impl Drop for TempFile {
		fn drop(&mut self) {
			let _ = fs::remove_file(&self.path);
		}
	}

	#[test]
	fn fingerprint_of_missing_path_is_none() {
		let path = env::temp_dir().join(format!("paperback-fingerprint-{}-does-not-exist", process::id()));
		assert_eq!(read_fingerprint(&path), None);
	}

	/// The percentage is reported on its own for a document with no pages at all, exactly as it was
	/// before pages joined it - so a markdown or plain text reader hears the same "15%" they always
	/// have, rather than a page they do not have.
	#[test]
	fn announcement_without_a_page_is_the_bare_percentage() {
		assert_eq!(position_announcement(15, None), "15%");
	}

	#[test]
	fn announcement_with_a_page_names_both() {
		assert_eq!(position_announcement(15, Some(30)), "15%, page 30");
	}

	/// The percentage's own `%` must survive substitution: it rides in on the `%s` argument rather
	/// than sitting in the msgid, because `t()` is a plain catalog lookup and does no printf-style
	/// unescaping - a `%%` in the format string would reach the listener doubled.
	#[test]
	fn announcement_keeps_the_percentage_sign_intact() {
		assert!(position_announcement(7, Some(412)).starts_with("7%,"));
	}

	#[test]
	fn unchanged_file_keeps_the_same_fingerprint() {
		let file = TempFile::with_content("unchanged", b"stable content");
		let first = read_fingerprint(&file.path);
		assert!(first.is_some());
		assert_eq!(first, read_fingerprint(&file.path));
	}

	#[test]
	fn rewriting_with_a_different_length_changes_the_fingerprint() {
		let file = TempFile::with_content("grows", b"short");
		let before = read_fingerprint(&file.path);
		fs::write(&file.path, b"content that is clearly longer").unwrap();
		let after = read_fingerprint(&file.path);
		assert!(before.is_some() && after.is_some());
		assert_ne!(before, after);
	}
}
