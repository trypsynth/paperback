use super::{DocumentManager, DocumentTab};
use crate::{
	text_window::{self, CtrlPos},
	ui::{
		navigation::{move_to_offset_and_record_history, persist_navigation_history},
		text_render::{append_slice_to_ctrl, reload_window_around},
	},
};

impl DocumentManager {
	/// Extends the loaded window ahead of a caret moving forward, on a timer because NVDA's Say-All reads through UI Automation and never reaches our key handlers.
	///
	/// It only ever appends. Say-All keeps its own offsets into the control, and recentring or compacting here once walked a Say-All back to the top of a large file. Compaction happens where the user moves instead (see [`Self::compact_window_after_user_move`]).
	pub fn pump_window_extend(&mut self) {
		let Some(tab) = self.active_tab_mut() else {
			return;
		};
		let doc_pos = tab.window.to_doc(tab.text_ctrl.get_insertion_point());
		let doc_len = tab.session.document_len();
		if !tab.window.wants_extension_for(doc_pos, doc_len) {
			return;
		}
		if !Self::extend_window_forward(tab) {
			// The control and window now disagree on what's loaded; reloading moves the loaded start, but a control whose contents no one can locate is worse than a read that loses its place.
			reload_window_around(tab, doc_pos, "append failed");
			let local = tab.window.to_local(doc_pos);
			tab.text_ctrl.set_insertion_point(local);
		}
	}

	/// Moving the loaded start invalidates a screen reader's offsets, so it only happens on a keypress or click, which already stops a read.
	/// A held arrow sends no key up until release, but the reload margin is over a minute of held arrow, so release compacts well before the loaded start.
	pub fn compact_window_after_user_move(&mut self) {
		let Some(tab) = self.active_tab_mut() else {
			return;
		};
		let doc_pos = tab.window.to_doc(tab.text_ctrl.get_insertion_point());
		let doc_len = tab.session.document_len();
		if !tab.window.needs_compaction_for(doc_pos, doc_len) {
			return;
		}
		reload_window_around(tab, doc_pos, "caret ran out of text behind it");
		let local = tab.window.to_local(doc_pos);
		tab.text_ctrl.set_insertion_point(local);
		tab.text_ctrl.show_position(local);
	}

	pub fn text_to_copy(&self) -> Option<String> {
		let tab = self.active_tab()?;
		let doc_len = tab.session.document_len();
		let source = copy_source(
			tab.text_ctrl.get_selection(),
			tab.text_ctrl.get_last_position(),
			tab.window.is_whole_document(doc_len),
		);
		let text = match source {
			CopySource::Nothing => return None,
			CopySource::Selection => tab.text_ctrl.get_string_selection(),
			CopySource::WholeDocument => tab.session.content(),
		};
		(!text.is_empty()).then_some(text)
	}

	/// The document's count rather than the selection's, since selecting all of a windowed slice is asking for the whole book, which is what copy will put on the clipboard.
	#[cfg(not(target_os = "macos"))]
	pub fn select_all_and_count(&self) -> Option<usize> {
		let tab = self.active_tab()?;
		tab.text_ctrl.select_all();
		Some(tab.session.handle().document().buffer.len().get())
	}

	/// Growth is free while reading (a screen reader's per-line work stays about 1ms on a 16.5M-character document) but rewrapping costs tens of seconds, so it is paid back only before a relayout.
	/// Self-limiting during a drag: the first resize brings the window under the threshold.
	pub fn compact_window_if_grown(&mut self) {
		let Some(tab) = self.active_tab_mut() else {
			return;
		};
		if tab.window.loaded_len() <= text_window::compaction_threshold() {
			return;
		}
		let doc_pos = tab.window.to_doc(tab.text_ctrl.get_insertion_point());
		reload_window_around(tab, doc_pos, "resize");
		let local = tab.window.to_local(doc_pos);
		tab.text_ctrl.set_insertion_point(local);
		tab.text_ctrl.show_position(local);
	}

	fn extend_window_forward(tab: &mut DocumentTab) -> bool {
		let doc_len = tab.session.document_len();
		let (from, to) = tab.window.extend_bounds(doc_len);
		if to <= from {
			return true;
		}
		// `from` is already paragraph-snapped and snapping is idempotent, so the slice abuts the loaded text with no gap or repeat.
		let slice = tab.session.get_window(from, to);
		if slice.end <= from {
			return true;
		}
		let mut extended = tab.window.clone();
		extended.extend_with(&slice);
		if !append_slice_to_ctrl(tab.text_ctrl, &slice, &extended) {
			return false;
		}
		let previous_end = tab.window.end();
		tab.window = extended;
		// A burst of these with the caret pinned at the loaded end means the append moved the caret, which `append_rtf_into_ctrl` saves the selection to prevent.
		tracing::debug!(
			caret = tab.window.to_doc(tab.text_ctrl.get_insertion_point()).get(),
			from_end = previous_end.get(),
			to_end = tab.window.end().get(),
			loaded = tab.window.loaded_len().get(),
			"extended loaded window"
		);
		let drift = tab.text_ctrl.get_last_position().get() - tab.window.to_local(tab.window.end()).get();
		if drift != 0 {
			tracing::warn!(drift, "window extension left the control and the window disagreeing");
			return false;
		}
		true
	}

	/// The control's own Ctrl+Home/End only reach the loaded window's ends, an arbitrary spot mid-book on a huge document, so `build_text_ctrl` routes them here.
	pub fn jump_to_document_edge(&mut self, to_end: bool) {
		let (track, update) = {
			let Some(tab) = self.active_tab_mut() else {
				return;
			};
			let offset =
				if to_end { tab.session.document_len().as_pos() } else { paperback_core::document::DocPos::ZERO };
			let update = move_to_offset_and_record_history(tab, offset);
			(tab.track, update)
		};
		persist_navigation_history(&self.config, track.then_some(&update));
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CopySource {
	Nothing,
	Selection,
	WholeDocument,
}

pub const fn copy_source(
	selection: (CtrlPos, CtrlPos),
	last_position: CtrlPos,
	window_is_whole_document: bool,
) -> CopySource {
	let (from, to) = (selection.0.get(), selection.1.get());
	if from >= to {
		CopySource::Nothing
	} else if from <= 0 && to >= last_position.get() && !window_is_whole_document {
		CopySource::WholeDocument
	} else {
		CopySource::Selection
	}
}

#[cfg(test)]
mod tests {
	use super::{CopySource, CtrlPos, copy_source};

	fn copying(from: i64, to: i64, last: i64, whole: bool) -> CopySource {
		copy_source((CtrlPos::from_raw(from), CtrlPos::from_raw(to)), CtrlPos::from_raw(last), whole)
	}

	#[test]
	fn nothing_selected_copies_nothing() {
		assert_eq!(copying(5, 5, 100, false), CopySource::Nothing);
	}

	#[test]
	fn part_of_the_loaded_text_copies_the_selection() {
		assert_eq!(copying(10, 20, 100, false), CopySource::Selection);
	}

	#[test]
	fn all_loaded_text_of_a_partly_loaded_document_copies_the_whole_document() {
		assert_eq!(copying(0, 100, 100, false), CopySource::WholeDocument);
	}

	#[test]
	fn all_text_of_a_fully_loaded_document_copies_the_selection() {
		assert_eq!(copying(0, 100, 100, true), CopySource::Selection);
	}
}
