//! Opening documents from links: one download window for all the links given at once, then a tab
//! for each document that came down.

use std::{path::Path, rc::Rc, sync::Mutex};

use paperback_core::{
	config::ConfigManager,
	fetch::{self, FetchError, Verdict},
};
use patois::t;
use wx_utils::{
	progress::{Ended, Progress, run_with_progress},
	show_error,
};
use wxdragon::prelude::*;

use super::{
	DocumentManager, dialogs, ensure_parser_ready_for_path, parser_ready::choose_format, rebuild_menu_bar,
	resolve_zip_path, update_title_from_manager,
};
use crate::{
	links::{self, Outcome, Stage},
	working_copy::WorkingCopy,
};

/// Downloads `links` in one window and opens each document that came down, then lists what could
/// not be opened, `not_links` first. A link that is already open only selects its tab.
pub fn open_links(
	frame: &Frame,
	dm: &Rc<Mutex<DocumentManager>>,
	config: &Rc<Mutex<ConfigManager>>,
	links: Vec<String>,
	not_links: Vec<String>,
) {
	let (notebook, open_tabs): (Notebook, Vec<Option<usize>>) = {
		let dm_ref = dm.lock().unwrap();
		(*dm_ref.notebook(), links.iter().map(|link| dm_ref.find_tab_by_path(Path::new(link))).collect())
	};
	if let Some(&index) = open_tabs.iter().flatten().last() {
		notebook.set_selection(index);
	}
	let to_download: Vec<String> =
		links.into_iter().zip(open_tabs).filter_map(|(link, open_tab)| open_tab.is_none().then_some(link)).collect();
	if to_download.is_empty() {
		report(frame, &links::report(&[], &not_links));
		return;
	}
	let (frame, dm, config) = (*frame, Rc::clone(dm), Rc::clone(config));
	run_with_progress(
		&frame,
		// TRANSLATORS: Title of the window that shows documents being downloaded from links
		&t("Open from URL"),
		// TRANSLATORS: First message in the window that downloads documents from links, before the first server answers
		&t("Connecting..."),
		move |progress| download_all(&to_download, progress),
		move |outcomes, ended| {
			// Cancelling opens nothing; dropping the outcomes deletes what was downloaded.
			if ended == Ended::Completed {
				open_downloads(&frame, &dm, &config, outcomes, &not_links);
			}
		},
	);
}

fn download_all(links: &[String], progress: &Progress) -> Vec<Outcome> {
	let mut outcomes = Vec::new();
	for (index, link) in links.iter().enumerate() {
		if progress.is_cancelled() {
			break;
		}
		progress.set(0, None);
		progress.set_message(&links::step(Stage::Checking, index + 1, links.len(), link));
		outcomes.push(download(link, index + 1, links.len(), progress));
	}
	outcomes
}

fn download(link: &str, n: usize, count: usize, progress: &Progress) -> Outcome {
	let refused = |extension: String| Outcome::Refused { link: link.to_string(), extension };
	let failed = |error: FetchError| Outcome::Failed { link: link.to_string(), error };
	if let Some(extension) = fetch::refused_extension(link) {
		return refused(extension);
	}
	let remote = match fetch::open(link, fetch::DEFAULT_MAX_SIZE) {
		Ok(remote) => remote,
		Err(error) => return failed(error),
	};
	let info = remote.info().clone();
	let warned = match info.verdict(link) {
		Verdict::Pass => false,
		Verdict::Refuse(extension) => return refused(extension),
		Verdict::Warn => {
			let warning = links::warning_text(link, info.content_type.as_deref());
			if progress.ask(move |parent| dialogs::confirm_download(parent, &warning)) != Some(true) {
				return Outcome::Skipped;
			}
			true
		}
	};
	let copy = match WorkingCopy::new(&info.file_name) {
		Ok(copy) => copy,
		Err(error) => return failed(error.into()),
	};
	progress.set_message(&links::step(Stage::Downloading, n, count, &info.file_name));
	match remote.save(copy.path(), progress.cancel_flag(), |done, total| progress.set(done, total)) {
		Ok(_) => Outcome::Downloaded { link: link.to_string(), copy, warned },
		Err(FetchError::Cancelled) => Outcome::Skipped,
		Err(error) => failed(error),
	}
}

fn open_downloads(
	frame: &Frame,
	dm: &Rc<Mutex<DocumentManager>>,
	config: &Rc<Mutex<ConfigManager>>,
	outcomes: Vec<Outcome>,
	not_links: &[String],
) {
	let problems = links::report(&outcomes, not_links);
	let mut opened_any = false;
	for outcome in outcomes {
		let Outcome::Downloaded { link, copy, warned } = outcome else {
			continue;
		};
		if open_download(frame, dm, config, &link, copy, warned) {
			opened_any = true;
		}
	}
	if opened_any {
		update_title_from_manager(frame, &dm.lock().unwrap());
		rebuild_menu_bar(frame, dm, config);
		dm.lock().unwrap().focus_document_text();
	}
	report(frame, &problems);
}

fn open_download(
	frame: &Frame,
	dm: &Rc<Mutex<DocumentManager>>,
	config: &Rc<Mutex<ConfigManager>>,
	link: &str,
	copy: WorkingCopy,
	warned: bool,
) -> bool {
	let saved_format = config.lock().unwrap().get_document_format(link);
	let copy_name = copy.path().file_name().map(|name| name.to_string_lossy().into_owned()).unwrap_or_default();
	if links::needs_open_as(warned, &copy_name, &saved_format) && !choose_format(frame, Path::new(link), config) {
		return false;
	}
	// A zip of several books offers the same list as one opened from disk.
	if saved_format.is_empty() {
		match resolve_zip_path(frame, copy.path(), config, false) {
			None => return false,
			Some(extracted) if extracted != copy.path() => {
				return ensure_parser_ready_for_path(frame, &extracted, config)
					&& dm.lock().unwrap().open_file(dm, &extracted);
			}
			Some(_) => {}
		}
	}
	dm.lock().unwrap().open_link(dm, link, copy)
}

fn report(frame: &Frame, problems: &[String]) {
	if !problems.is_empty() {
		// TRANSLATORS: Title of the error listing the links that could not be opened
		show_error(frame, problems.join("\n"), &t("Open from URL"));
	}
}
