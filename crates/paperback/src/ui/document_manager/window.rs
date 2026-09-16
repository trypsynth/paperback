//! The loaded-window management for a document: keeping the text control's slice of a large
//! document ahead of a forward read, compacting it when the caret runs out of text behind it or
//! the layout changes, copying the whole document when a windowed Select All only reached a slice,
//! and jumping to the true document edges rather than the loaded window's. Split out of the main
//! `DocumentManager` impl; see [`super`] for the windowing model in `ui::text_window`.

use wxdragon::{clipboard::Clipboard, prelude::*};

use super::{DocumentManager, DocumentTab};
use crate::ui::{
	navigation::{move_to_offset_and_record_history, persist_navigation_history},
	text_render::{append_slice_to_ctrl, reload_window_around},
	text_window,
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

	/// Copies the current selection, widening it to the whole document when everything loaded is
	/// selected but only part of the document is.
	///
	/// Select All can only ever reach what the control holds, which for a windowed document is a
	/// slice of the book rather than the book. The window is not something a reader can see, name
	/// or reason about, so a selection covering all of it means all of it in the only sense the
	/// user has - and copying half a chapter when they asked for the book is a silent wrong answer.
	///
	/// Deliberately keyed off the selection rather than a remembered Select All: clicking anywhere
	/// collapses the selection, so this stops applying on its own with no flag to keep in step.
	///
	/// Returns whether it copied. False means nothing special applied and the caller should let
	/// the control's own copy run.
	pub fn copy_whole_document_if_all_selected(&self) -> bool {
		let Some(tab) = self.active_tab() else {
			return false;
		};
		let (from, to) = tab.text_ctrl.get_selection();
		if from >= to {
			return false;
		}
		let doc_len = tab.session.document_len();
		// Only the widened case is handled here. Anything else is left to the control's own copy,
		// which is better tested than anything this could put in its place.
		let covers_everything_loaded = from <= 0 && to >= tab.text_ctrl.get_last_position();
		if !covers_everything_loaded || tab.window.is_whole_document(doc_len) {
			return false;
		}
		let text = tab.session.get_text_range(0, doc_len);
		if text.is_empty() {
			return false;
		}
		Clipboard::get().set_text(&text)
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
		if !append_slice_to_ctrl(tab.text_ctrl, &slice) {
			return false;
		}
		let previous_end = tab.window.end();
		tab.window.extend_end_to(slice.end);
		// The safe half, logged at debug so a report can be read alongside the window moves above.
		// A healthy forward read extends about once per chunk; a burst of these with the caret
		// pinned at the loaded end is the signature of the caret being moved by the append itself,
		// which is what `append_rtf_into_ctrl` saves and restores the selection to prevent.
		tracing::debug!(
			caret = tab.window.to_doc(tab.text_ctrl.get_insertion_point()),
			from_end = previous_end,
			to_end = tab.window.end(),
			loaded = tab.window.loaded_len(),
			"extended loaded window"
		);
		// The control and the window have to agree on how much is loaded, give or take the single
		// display unit RichEdit does not store for a wholly-trailing paragraph mark (see
		// `write::stored_display_len`). Drift past that would offset every translation from here on.
		let drift = tab.text_ctrl.get_last_position() - tab.window.loaded_len();
		if !(-1..=0).contains(&drift) {
			tracing::warn!(drift, "window extension left the control and the window disagreeing");
			return false;
		}
		true
	}

	/// Jumps the caret to the very first or very last character of the *document*, reloading the
	/// text control's window at that edge (see `ui::text_window`).
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
			let offset = if to_end { tab.session.document_len().max(0) } else { 0 };
			let update = move_to_offset_and_record_history(tab, offset);
			(tab.track, update)
		};
		persist_navigation_history(&self.config, track.then_some(&update));
	}
}
