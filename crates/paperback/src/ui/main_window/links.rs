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
	DocumentManager, dialogs, ensure_parser_ready_for_path,
	parser_ready::{browse_zip, choose_format},
	rebuild_menu_bar, update_title_from_manager,
};
use crate::{
	links::{self, Outcome, Stage},
	working_copy::WorkingCopy,
};

pub fn open_links(
	frame: &Frame,
	dm: &Rc<Mutex<DocumentManager>>,
	config: &Rc<Mutex<ConfigManager>>,
	links: Vec<String>,
) {
	open_links_then(frame, dm, config, links, || {});
}

pub fn open_links_then(
	frame: &Frame,
	dm: &Rc<Mutex<DocumentManager>>,
	config: &Rc<Mutex<ConfigManager>>,
	links: Vec<String>,
	then: impl FnOnce() + 'static,
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
		then();
		return;
	}
	let (frame, dm, config) = (*frame, Rc::clone(dm), Rc::clone(config));
	run_with_progress(
		&frame,
		// TRANSLATORS: Title of the window that shows documents being downloaded from links
		&t("Open from URL"),
		// TRANSLATORS: First message in the window that downloads documents from links, before the first server answers
		&t("Connecting..."),
		move |progress| {
			let count = to_download.len();
			to_download
				.iter()
				.enumerate()
				.take_while(|_| !progress.is_cancelled())
				.map(|(index, link)| download(link, index + 1, count, progress))
				.collect()
		},
		move |outcomes, ended| {
			// Cancelling opens nothing; dropping the outcomes deletes what was downloaded.
			if ended == Ended::Completed {
				open_downloads(&frame, &dm, &config, outcomes);
			}
			then();
		},
	);
}

pub fn download_again(
	frame: &Frame,
	dm: &Rc<Mutex<DocumentManager>>,
	link: String,
	reloaded: impl FnOnce(&DocumentManager) + 'static,
) {
	let (frame, dm) = (*frame, Rc::clone(dm));
	run_with_progress(
		&frame,
		// TRANSLATORS: Title of the window that shows a document opened from a link being downloaded again
		&t("Reload"),
		&t("Connecting..."),
		move |progress| download(&link, 1, 1, progress),
		move |outcome, ended| {
			if ended != Ended::Completed {
				return;
			}
			match outcome {
				Outcome::Downloaded { link, copy, .. } => {
					let mut dm_ref = dm.lock().unwrap();
					// The tab may have closed meanwhile; then there is nothing to read again.
					let Some(index) = dm_ref.find_tab_by_path(Path::new(&link)) else {
						return;
					};
					if dm_ref.reload_link(index, copy) {
						reloaded(&dm_ref);
					}
				}
				Outcome::Failed { .. } => {
					show_error(&frame, links::report(std::slice::from_ref(&outcome)).join("\n"), &t("Reload"));
				}
				Outcome::Skipped => {}
			}
		},
	);
}

fn download(link: &str, n: usize, count: usize, progress: &Progress) -> Outcome {
	let failed = |error: FetchError| Outcome::Failed { link: link.to_string(), error };
	progress.set(0, None);
	progress.set_message(&links::step(Stage::Checking, n, count, link));
	if let Some(extension) = fetch::refused_extension(link) {
		return failed(FetchError::Refused(extension));
	}
	let owned_link = link.to_string();
	let remote = match links::unless_cancelled(
		|| progress.is_cancelled(),
		|_, _| {},
		move |_, _| fetch::open(&owned_link, fetch::DEFAULT_MAX_SIZE),
	) {
		None => return Outcome::Skipped,
		Some(Ok(remote)) => remote,
		Some(Err(error)) => return failed(error),
	};
	let info = remote.info().clone();
	let warned = match info.verdict(link) {
		Verdict::Pass => false,
		Verdict::Refuse(extension) => return failed(FetchError::Refused(extension)),
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
	let path = copy.path().to_path_buf();
	match links::unless_cancelled(
		|| progress.is_cancelled(),
		|done, total| progress.set(done, total),
		move |cancel, moved| remote.save(&path, cancel, moved),
	) {
		Some(Ok(_)) => Outcome::Downloaded { link: link.to_string(), copy, warned },
		None | Some(Err(FetchError::Cancelled)) => Outcome::Skipped,
		Some(Err(error)) => failed(error),
	}
}

fn open_downloads(
	frame: &Frame,
	dm: &Rc<Mutex<DocumentManager>>,
	config: &Rc<Mutex<ConfigManager>>,
	outcomes: Vec<Outcome>,
) {
	let problems = links::report(&outcomes);
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
	if !problems.is_empty() {
		// TRANSLATORS: Title of the error listing the links that could not be opened
		show_error(frame, problems.join("\n"), &t("Open from URL"));
	}
}

fn open_download(
	frame: &Frame,
	dm: &Rc<Mutex<DocumentManager>>,
	config: &Rc<Mutex<ConfigManager>>,
	link: &str,
	copy: WorkingCopy,
	warned: bool,
) -> bool {
	if links::needs_open_as(warned, copy.path()) && !choose_format(frame, Path::new(link), config) {
		return false;
	}
	// A zip of several books offers the same list as one opened from disk, unless a format was picked for the link.
	if config.lock().unwrap().get_document_format(link).is_empty() {
		match browse_zip(frame, copy.path(), false) {
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
