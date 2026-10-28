use std::{cell::RefCell, path::Path, rc::Rc, sync::Mutex};

use paperback_core::{config::ConfigManager, parser::is_remote_url};
use wxdragon::prelude::*;

use super::{
	DocumentManager, ensure_parser_ready_for_path, links::open_links_then, rebuild_menu_bar, update_title_from_manager,
};

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
		let plan = {
			let cfg = config.lock().unwrap();
			plan(paths, |link| cfg.get_document_download_size(link), RESTORE_DOWNLOAD_LIMIT)
		};
		let failed = restore_each(plan.files, |path_str| {
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
			// A link too large to download again stays in the recent documents only.
			for path in failed.iter().chain(&plan.dropped) {
				config.remove_opened_document(path);
			}
			config.flush();
		}
		let (dm, cfg) = (Rc::clone(&doc_manager), Rc::clone(&config));
		// After the links open, since opening them selects their tabs.
		open_links_then(&frame, &doc_manager, &config, plan.links, move || {
			select_restored_tab(&dm, pre_restore_active, &active_path);
			update_title_from_manager(&frame, &dm.lock().unwrap());
			rebuild_menu_bar(&frame, &dm, &cfg);
			dm.lock().unwrap().restore_focus();
			// A large book keeps the window busy for seconds, and a screen reader told about focus meanwhile is left silent on it (#920); repeating it costs nothing otherwise.
			focus_again_once_settled(&frame, &dm);
		});
	});
}

const RESTORE_DOWNLOAD_LIMIT: u64 = 2 * 1024 * 1024;

fn select_restored_tab(doc_manager: &Rc<Mutex<DocumentManager>>, pre_restore_active: Option<usize>, active_path: &str) {
	let dm = doc_manager.lock().unwrap();
	let target = pre_restore_active
		.or_else(|| if active_path.is_empty() { None } else { dm.find_tab_by_path(Path::new(active_path)) });
	if let Some(index) = target {
		dm.notebook().set_selection(index);
	}
}

/// Long enough for a large book to finish laying out, short enough not to move a reader from under their first keystroke.
const FOCUS_SETTLE_DELAY_MS: i32 = 250;

fn focus_again_once_settled(frame: &Frame, doc_manager: &Rc<Mutex<DocumentManager>>) {
	let timer_holder: Rc<RefCell<Option<Timer<Frame>>>> = Rc::new(RefCell::new(None));
	let holder = Rc::clone(&timer_holder);
	let doc_manager = Rc::clone(doc_manager);
	let timer = Timer::new(frame);
	timer.on_tick(move |_event| {
		let dm = doc_manager.lock().unwrap();
		dm.restore_focus();
		// Focus is usually already on the text, and refocusing tells a screen reader nothing, so the retry announces itself.
		#[cfg(target_os = "windows")]
		dm.announce_focus();
		drop(dm);
		*holder.borrow_mut() = None;
	});
	if timer.start(FOCUS_SETTLE_DELAY_MS, true) {
		*timer_holder.borrow_mut() = Some(timer);
	}
}

#[derive(Debug, Default, PartialEq, Eq)]
pub(super) struct RestorePlan {
	files: Vec<String>,
	links: Vec<String>,
	dropped: Vec<String>,
}

pub(super) fn plan(paths: Vec<String>, size_of: impl Fn(&str) -> Option<u64>, limit: u64) -> RestorePlan {
	let mut plan = RestorePlan::default();
	for path in paths {
		if !is_remote_url(&path) {
			plan.files.push(path);
		} else if size_of(&path).is_some_and(|size| size <= limit) {
			plan.links.push(path);
		} else {
			plan.dropped.push(path);
		}
	}
	plan
}

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

	fn strings(items: &[&str]) -> Vec<String> {
		items.iter().map(ToString::to_string).collect()
	}

	#[test]
	fn plan_downloads_again_only_the_links_whose_last_download_was_small() {
		let sizes = |link: &str| match link {
			"https://a.org/small.epub" => Some(100),
			"https://a.org/limit.epub" => Some(1_000),
			"https://a.org/large.pdf" => Some(1_001),
			_ => None,
		};
		let paths = strings(&[
			"https://a.org/large.pdf",
			"C:\\books\\one.epub",
			"https://a.org/small.epub",
			"https://a.org/never-sized.epub",
			"C:\\books\\two.epub",
			"https://a.org/limit.epub",
		]);
		assert_eq!(
			plan(paths, sizes, 1_000),
			RestorePlan {
				files: strings(&["C:\\books\\one.epub", "C:\\books\\two.epub"]),
				links: strings(&["https://a.org/small.epub", "https://a.org/limit.epub"]),
				dropped: strings(&["https://a.org/large.pdf", "https://a.org/never-sized.epub"]),
			}
		);
	}
}
