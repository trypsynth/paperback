//! Reopening the documents that were open when the app last exited, and forgetting the ones
//! that can no longer be reopened.

use std::{path::Path, rc::Rc, sync::Mutex};

use paperback_core::config::ConfigManager;
use wxdragon::prelude::*;

use super::{DocumentManager, ensure_parser_ready_for_path, menu, update_title_from_manager};

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
		let dm_ref = doc_manager.lock().unwrap();
		update_title_from_manager(&frame, &dm_ref);
		let has_docs = dm_ref.tab_count() > 0;
		let menu_bar = menu::create_menu_bar(&config.lock().unwrap());
		frame.set_menu_bar(menu_bar);
		menu::update_menu_item_states(&frame, has_docs);
		menu::update_reopen_state(&frame, false);
		dm_ref.restore_focus();
	});
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
