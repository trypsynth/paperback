//! Reopening the documents that were open when the app last exited, and forgetting the ones
//! that can no longer be reopened.

use std::{cell::RefCell, path::Path, rc::Rc, sync::Mutex};

use paperback_core::config::ConfigManager;
use wxdragon::prelude::*;

use super::{DocumentManager, ensure_parser_ready_for_path, rebuild_menu_bar, update_title_from_manager};

#[derive(Default)]
struct RestoreState {
	restored: bool,
	closing: bool,
}

pub(super) fn schedule_restore_documents(
	frame: Frame,
	doc_manager: Rc<Mutex<DocumentManager>>,
	config: Rc<Mutex<ConfigManager>>,
) {
	let restore = config.lock().unwrap().get_app_bool("restore_previous_documents", true);
	if !restore {
		return;
	}
	let state = Rc::new(Mutex::new(RestoreState::default()));
	let state_for_close = Rc::clone(&state);
	frame.on_close(move |_event| {
		state_for_close.lock().unwrap().closing = true;
	});
	let state_for_destroy = Rc::clone(&state);
	frame.on_destroy(move |_event| {
		state_for_destroy.lock().unwrap().closing = true;
	});
	let state_for_idle = Rc::clone(&state);
	frame.on_idle(move |_event| {
		let mut state = state_for_idle.lock().unwrap();
		if state.restored || state.closing {
			return;
		}
		state.restored = true;
		drop(state);
		let pre_restore_active = doc_manager.lock().unwrap().active_tab_index();
		let active_path = config.lock().unwrap().get_app_string("active_document", "");
		let paths = config.lock().unwrap().get_opened_documents();
		tracing::info!(count = paths.len(), "restoring previously open documents");
		let failed = restore_each(paths, |path_str| {
			let path = Path::new(path_str);
			if !path.exists() {
				tracing::info!(path = %path.display(), "previously open document is missing; forgetting it");
				return false;
			}
			if !ensure_parser_ready_for_path(&frame, path, &config) {
				return false;
			}
			doc_manager.lock().unwrap().open_file_restore(&doc_manager, path)
		});
		{
			let config = config.lock().unwrap();
			for path in &failed {
				config.remove_opened_document(path);
			}
			config.flush();
		}
		let mut target_idx = pre_restore_active;
		if target_idx.is_none() && !active_path.is_empty() {
			target_idx = doc_manager.lock().unwrap().find_tab_by_path(Path::new(&active_path));
		}
		if let Some(idx) = target_idx {
			doc_manager.lock().unwrap().notebook().set_selection(idx);
		}
		update_title_from_manager(&frame, &doc_manager.lock().unwrap());
		rebuild_menu_bar(&frame, &doc_manager, &config);
		doc_manager.lock().unwrap().restore_focus();
		// A large book keeps the window busy for seconds after this, and a screen reader that
		// was told about the focus during that stretch has nothing to read: the reader is left
		// on a document where no key says anything until it is closed and opened again
		// (<https://github.com/trypsynth/paperback/issues/920>). Saying it again once the queue
		// has drained costs nothing when the first one did arrive, since focus does not move.
		focus_again_once_settled(&frame, &doc_manager);
	});
}

/// How long after the restore to say where focus is a second time. Long enough for the window
/// to have finished laying out a large book, short enough that a reader who starts pressing keys
/// straight away is not moved from under their own first keystroke.
const FOCUS_SETTLE_DELAY_MS: i32 = 250;

/// Focuses the restored document again once the window has stopped working, so a screen reader
/// that missed the first focus while the text was still loading hears this one.
///
/// The one-shot `wxTimer` keeps itself alive through its single tick by the `Rc`/`RefCell` it
/// hands its own callback, the same way delayed announcements do.
fn focus_again_once_settled(frame: &Frame, doc_manager: &Rc<Mutex<DocumentManager>>) {
	let timer_holder: Rc<RefCell<Option<Timer<Frame>>>> = Rc::new(RefCell::new(None));
	let holder = Rc::clone(&timer_holder);
	let doc_manager = Rc::clone(doc_manager);
	let timer = Timer::new(frame);
	timer.on_tick(move |_event| {
		doc_manager.lock().unwrap().restore_focus();
		*holder.borrow_mut() = None;
	});
	if timer.start(FOCUS_SETTLE_DELAY_MS, true) {
		*timer_holder.borrow_mut() = Some(timer);
	}
}

/// Runs `open` on every stored path in order and returns the ones it could not reopen.
pub(super) fn restore_each(paths: Vec<String>, mut open: impl FnMut(&str) -> bool) -> Vec<String> {
	paths.into_iter().filter(|path| !open(path)).collect()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn restore_each_returns_the_paths_that_failed() {
		let mut seen = Vec::new();
		let failed = restore_each(vec!["ok".to_string(), "bad".to_string(), "ok2".to_string()], |path| {
			seen.push(path.to_string());
			path != "bad"
		});
		assert_eq!(failed, vec!["bad"]);
		assert_eq!(seen, vec!["ok", "bad", "ok2"]);
	}

	#[test]
	fn restore_each_returns_empty_when_all_succeed() {
		let failed = restore_each(vec!["a".to_string(), "b".to_string()], |_| true);
		assert!(failed.is_empty());
	}
}
