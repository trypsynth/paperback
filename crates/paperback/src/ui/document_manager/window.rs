use super::DocumentManager;
use crate::{
	text_window::CtrlPos,
	ui::navigation::{move_to_offset_and_record_history, persist_navigation_history},
};

impl DocumentManager {
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

	/// Jumps to the document's first or last position, recording the move in the navigation history.
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
