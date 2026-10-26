//! The loaded-window management for a document: keeping the text control's slice of a large document ahead of a forward read, compacting it when the caret runs out of text behind it or the layout changes, copying the whole document when a windowed Select All only reached a slice, and jumping to the true document edges rather than the loaded window's. Split out of the main `DocumentManager` impl; see [`super`] for the windowing model in `text_window`.

use super::{DocumentManager, DocumentTab};
use crate::{
	text_window::{self, CtrlPos},
	ui::{
		navigation::{move_to_offset_and_record_history, persist_navigation_history},
		text_render::{append_slice_to_ctrl, reload_window_around},
	},
};

impl DocumentManager {
	/// Keeps the loaded window ahead of a caret that is moving forward, by appending to it.
	///
	/// A timer rather than an input hook because a screen reader's own text-walking never reaches
	/// this app's key handlers: NVDA's Say-All (and similar continuous-reading features) drives
	/// RichEdit's UI Automation text pattern directly. Without polling, reaching the loaded end
	/// during a Say-All would look exactly like reaching the real end of the document, and reading
	/// would stop mid-paragraph with millions of characters still unread. Reacting to where the
	/// caret actually is sidesteps having to know what put it there. `RELOAD_MARGIN`, a quarter of
	/// the window, gives even fast reading time to load more well before it runs out.
	///
	/// Because it runs on a timer, it must never do anything a reader could notice. Appending
	/// qualifies: every offset already handed out still points at the same character. Rebuilding
	/// the window does not.
	///
	/// The bug that motivated the split: NVDA's Say-All holds its own offsets into the control,
	/// advances them itself rather than re-reading the caret, and has no handling for the text
	/// changing underneath it (`_TextReader` only checks whether the object died). Recentring the
	/// window swapped the whole buffer and moved `start`, so those offsets silently came to mean a
	/// different place in the book. Say-All read on from the wrong spot, moved the real caret there
	/// with it, and this timer then saw *that* position, decided it was near an edge, and recentred
	/// again - walking the window backwards to the top of the document a few hundred thousand
	/// characters at a time while the user listened. Reported as reading randomly jumping to the
	/// start of large text files.
	///
	/// Splitting extension from compaction was not enough on its own, because compaction stayed
	/// reachable from here for a caret that had run out of text behind it, on the reasoning that a
	/// forward reader could never be in that position. A reader whose offsets have gone stale can:
	/// compaction moves the loaded start out from under offsets their holder never revisits, which
	/// leaves those offsets sitting near the new start, which is the very condition to compact
	/// again. Each tick walked the window another `RELOAD_MARGIN` back, and the read reached the
	/// top of the document in about three seconds. A tick that can only extend cannot start that
	/// loop, so this one cannot.
	///
	/// Compaction happens elsewhere: at the navigation chokepoints, on the key and mouse events
	/// that end a read (see [`Self::compact_window_after_user_move`]), and on a resize, where the
	/// cost it exists to avoid is actually billed.
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
			// The append did not land cleanly, so the control and the window no longer agree on
			// what is loaded. Every later position translation would be wrong by the difference;
			// rebuild to get back to a state that is at least consistent. This does move the
			// loaded start, so it is the one thing here a reader can notice, but a control whose
			// contents no one can locate is worse than a read that loses its place.
			reload_window_around(tab, doc_pos, "append failed");
			let local = tab.window.to_local(doc_pos);
			tab.text_ctrl.set_insertion_point(local);
		}
	}

	/// Compacts the window around the caret if the caret has run out of text behind it.
	///
	/// The counterpart to [`Self::pump_window_extend`], called from the key and mouse handlers
	/// rather than the timer. Anything that moves the loaded start invalidates the offsets a
	/// screen reader is reading from, so it may only happen where the user has already done
	/// something that stops a read, and a keypress or a click is exactly that.
	///
	/// Holding Up-arrow is the case this exists for. Auto-repeat sends key *down* events and no
	/// key up until release, so a held key eats into [`RELOAD_MARGIN`] without compacting; that
	/// margin is a quarter of the window, which is over a minute of held arrow before the caret
	/// reaches the loaded start, and releasing the key compacts well before then.
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

	/// The text Edit > Copy puts on the clipboard: the control's selection, or the whole document
	/// when everything loaded is selected but only part of the document is loaded. `None` when
	/// nothing is selected.
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

	/// Selects everything the reading control holds and reports the document's own character count,
	/// or `None` when there is no document open.
	///
	/// The count is the document's rather than the control's selection: a windowed document holds a
	/// slice of the book, and selecting all of that slice is the reader asking for all of it - which
	/// is what a copy of it will put on the clipboard, so the two counts agree.
	#[cfg(not(target_os = "macos"))]
	pub fn select_all_and_count(&self) -> Option<usize> {
		let tab = self.active_tab()?;
		tab.text_ctrl.select_all();
		Some(tab.session.handle().document().buffer.len().get())
	}

	/// Collapses a window that grew during a long read back to target size around the caret.
	///
	/// Called before a relayout, which is the one place a deep caret in a big loaded buffer is
	/// actually billed. Measured on a 16.5M-character document, the per-line work a screen reader
	/// does costs about a millisecond even at the far end - nothing against the seconds spent
	/// speaking that line - whereas rewrapping the same buffer costs tens of seconds. So growth is
	/// free while reading and only has to be paid back when the layout actually changes.
	///
	/// Resize events arrive continuously during a drag; this is self-limiting because the first
	/// one brings the window back under the threshold and the rest return immediately.
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

	/// Appends the next chunk of the document to the end of the loaded window, leaving
	/// `window.start` and every offset already in play untouched.
	///
	/// Returns false if the append did not land cleanly, meaning the caller should compact rather
	/// than carry on with a control and a `TextWindow` that disagree.
	fn extend_window_forward(tab: &mut DocumentTab) -> bool {
		let doc_len = tab.session.document_len();
		let (from, to) = tab.window.extend_bounds(doc_len);
		if to <= from {
			return true;
		}
		// `from` is the current loaded end, which the session snapped to a paragraph boundary when
		// it produced this window. Snapping a boundary again is a no-op, so this slice abuts the
		// loaded text exactly: no gap, no repeated paragraph.
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
		// The safe half, logged at debug so a report can be read alongside the window moves above.
		// A healthy forward read extends about once per chunk; a burst of these with the caret
		// pinned at the loaded end is the signature of the caret being moved by the append itself,
		// which is what `append_rtf_into_ctrl` saves and restores the selection to prevent.
		tracing::debug!(
			caret = tab.window.to_doc(tab.text_ctrl.get_insertion_point()).get(),
			from_end = previous_end.get(),
			to_end = tab.window.end().get(),
			loaded = tab.window.loaded_len().get(),
			"extended loaded window"
		);
		// The control and the window have to agree on how much is loaded. Any drift would offset
		// every translation from here on.
		let drift = tab.text_ctrl.get_last_position().get() - tab.window.to_local(tab.window.end()).get();
		if drift != 0 {
			tracing::warn!(drift, "window extension left the control and the window disagreeing");
			return false;
		}
		true
	}

	/// Jumps the caret to the very first or very last character of the *document*, reloading the
	/// text control's window at that edge (see `text_window`).
	///
	/// `text_ctrl`'s own Ctrl+Home/Ctrl+End can only ever reach the ends of whatever window is
	/// currently loaded, which on a huge document is an arbitrary spot mid-book rather than the
	/// start or end of the document - and reaching a loaded edge that way then makes
	/// `pump_window_reload` recentre the window on it, so the keys both landed in the wrong place
	/// and paid for a reload to get there. `build_text_ctrl` intercepts them and routes here
	/// instead, which also keeps them consistent with every other jump in the app (history,
	/// audio sync, focus).
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

/// What a copy takes from the reading control.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CopySource {
	Nothing,
	/// The text selected in the control.
	Selection,
	/// The whole document, of which the control holds only part.
	WholeDocument,
}

/// What copying `selection` takes, for a control holding text up to `last_position` that is the
/// whole document or only part of it.
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
