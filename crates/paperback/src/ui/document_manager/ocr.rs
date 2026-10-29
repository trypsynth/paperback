//! Workers are given page numbers, not offsets: applying one page's text moves every offset after it, so the UI thread re-resolves each placeholder as it applies it.

use std::{
	path::{Path, PathBuf},
	sync::{
		Arc,
		atomic::{AtomicBool, Ordering},
	},
	thread,
	time::{Duration, Instant},
};

use paperback_core::{
	config::ConfigManager,
	document::DocPos,
	ocr::{RecognizedLine, default_workers, load_pages, page_text, recognize_pages, save_pages, saved_pages_file},
	session::DocumentSession,
};
use patois::{nt, t};

use super::{DocumentManager, DocumentTab};
use crate::{
	ocr::{self, OcrError},
	ui::{
		navigation::{announce, announce_for_command, announce_waiting_for_command},
		text_render::reload_window_around,
	},
};

/// Time rather than a page count, since page speed varies several-fold between machines; it also bounds checkpoint rebuilds, which are linear in the document, so a long scan doesn't go quadratic.
const BATCH_OCR_UPDATE_EVERY: Duration = Duration::from_secs(4);

pub(super) struct OcrJob {
	id: u64,
	cancel: Arc<AtomicBool>,
	batch: bool,
	recognized: usize,
	/// Stopped by a re-read rather than by the reader, so its remaining flushes are dropped and nothing is announced.
	quiet_cancel: bool,
	saved_as: PathBuf,
	/// Whether a shortcut started it, since a fast result started from a menu lands while the menu is still closing.
	from_keyboard: bool,
}

/// Paragraph joining is PDF-only because the other OCR source is comics, whose lines are balloons and belong apart.
pub(super) fn is_pdf(file_path: &Path) -> bool {
	file_path.extension().is_some_and(|extension| extension.eq_ignore_ascii_case("pdf"))
}

/// The renderer hands anything but PDFs and comics to the PDF engine, so "Include text pages" on an EPUB would otherwise start a job it can't open.
fn is_ocr_able(file_path: &Path) -> bool {
	file_path.extension().is_some_and(|extension| {
		extension.eq_ignore_ascii_case("pdf")
			|| extension.eq_ignore_ascii_case("cbz")
			|| extension.eq_ignore_ascii_case("cbr")
	})
}

pub(super) fn joins_recognized_text(local_path: &Path, config: &ConfigManager) -> bool {
	is_pdf(local_path) && config.get_app_bool("join_pdf_paragraphs", true)
}

pub(super) fn saved_pages_of(local_path: &Path) -> PathBuf {
	saved_pages_file(&ocr::saved_pages_dir(), local_path)
}

pub(super) fn restore_saved_pages(session: &mut DocumentSession, saved: &Path, join: bool) {
	let saved_pages = load_pages(saved);
	let pages: Vec<(i32, String)> = saved_pages
		.into_iter()
		.map(|(page, lines)| (page, page_text(&lines, join)))
		.filter(|(_, text)| !text.trim().is_empty())
		.collect();
	if !pages.is_empty() {
		session.replace_ocr_pages(&pages);
		tracing::debug!(path = %saved.display(), pages = pages.len(), "put back the pages ocr read before");
	}
}

type PageResult = (i32, Result<Vec<RecognizedLine>, OcrError>);

fn index_running_job(job_ids: &[Option<u64>], id: u64) -> Option<usize> {
	job_ids.iter().position(|job_id| *job_id == Some(id))
}

impl DocumentManager {
	pub fn image_only_page_at_caret(&self) -> Option<DocPos> {
		let tab = self.active_tab()?;
		let position = tab.window.to_doc(tab.text_ctrl.get_insertion_point());
		tab.session.image_only_page_at(position)
	}

	pub fn batch_ocr_running(&self) -> bool {
		self.active_tab().is_some_and(|tab| tab.ocr_job.as_ref().is_some_and(|job| job.batch))
	}

	/// `quiet` is for a stop that comes from re-reading the document, where the count would describe discarded text.
	pub fn cancel_ocr(&mut self, quiet: bool) {
		if let Some(index) = self.active_tab_index() {
			self.stop_ocr_at(index, quiet);
		}
	}

	/// Advisory rather than a wait, since the UI thread can't block on OCR; the stop lands at the worker's next page boundary.
	pub(crate) fn stop_ocr_at(&mut self, index: usize, quiet: bool) {
		if let Some(job) = self.tabs.get_mut(index).and_then(|tab| tab.ocr_job.as_mut()) {
			job.cancel.store(true, Ordering::Relaxed);
			job.quiet_cancel = quiet;
		}
	}

	pub fn start_ocr_for_current_page(&mut self) {
		let label = self.live_region_label;
		let Some(offset) = self.image_only_page_at_caret() else {
			return;
		};
		let Some(tab) = self.active_tab() else {
			return;
		};
		if tab.ocr_job.is_some() {
			// TRANSLATORS: Announced when Enter is pressed on an OCR placeholder while OCR is already running
			announce(label, t("OCR in progress."));
			return;
		}
		let page = tab.session.current_page(offset);
		self.spawn_ocr(vec![page], false, true);
	}

	/// Also for pages that already have text, which may come from an older or worse OCR pass (#814).
	pub fn recognize_current_page(&mut self, from_keyboard: bool) {
		let label = self.live_region_label;
		let Some(tab) = self.active_tab() else {
			// TRANSLATORS: Announced when Recognize Current Page is chosen with no document open
			announce_for_command(label, from_keyboard, t("No document open."));
			return;
		};
		if tab.ocr_job.is_some() {
			// TRANSLATORS: Announced when Recognize Current Page is chosen while OCR is already running
			announce_for_command(label, from_keyboard, t("OCR already in progress."));
			return;
		}
		if !is_ocr_able(tab.local_path()) {
			// TRANSLATORS: Announced when Recognize Current Page is chosen in a document that is not a PDF or a comic book
			announce_for_command(label, from_keyboard, t("Only PDF and comic book pages can be recognized."));
			return;
		}
		let position = tab.window.to_doc(tab.text_ctrl.get_insertion_point());
		// Text before the first page break still belongs to the first page.
		let page = tab.session.current_page(position).max(1);
		self.spawn_ocr(vec![page], false, from_keyboard);
	}

	/// A document the renderer can't open keeps the image-only behaviour even with `include_text_pages`, since a completed job that recognized nothing would be worse than saying the range held no image-only pages.
	pub fn start_batch_ocr(&mut self, start: i32, end: i32, include_text_pages: bool) {
		let label = self.live_region_label;
		let Some(tab) = self.active_tab() else {
			// TRANSLATORS: Announced when batch OCR is triggered with no document open
			announce(label, t("No document open."));
			return;
		};
		if tab.ocr_job.is_some() {
			// TRANSLATORS: Announced when batch OCR is started while another OCR job is running
			announce(label, t("OCR already in progress."));
			return;
		}
		let (start, end) = if start <= end { (start, end) } else { (end, start) };
		let pages: Vec<i32> = if include_text_pages && is_ocr_able(tab.local_path()) {
			let last = i32::try_from(tab.session.page_count()).unwrap_or(i32::MAX);
			// Empty when the range starts past the last page, which falls through to the nothing-to-do announcement rather than a job for a missing page.
			(start..=end.min(last)).collect()
		} else {
			tab.session
				.image_only_pages()
				.into_iter()
				.filter_map(|(page, _)| (page >= start && page <= end).then_some(page))
				.collect()
		};
		if pages.is_empty() {
			// TRANSLATORS: Announced when the chosen batch OCR range contains no image-only pages
			announce(label, t("No image-only pages in the given range."));
			return;
		}
		self.spawn_ocr(pages, true, true);
	}

	fn spawn_ocr(&mut self, pages: Vec<i32>, batch: bool, from_keyboard: bool) {
		let label = self.live_region_label;
		let Some(tab) = self.active_tab() else {
			return;
		};
		let path = tab.file_path.clone();
		let password = self.config.lock().unwrap().get_document_password(&path.to_string_lossy());
		let password = (!password.is_empty()).then_some(password);
		let path_str = tab.local_path().to_string_lossy().to_string();
		let saved_as = saved_pages_of(tab.local_path());
		let cancel = Arc::new(AtomicBool::new(false));
		let worker_cancel = Arc::clone(&cancel);
		let id = self.next_job_id.get();
		self.next_job_id.set(id.wrapping_add(1));
		let job = OcrJob { id, cancel, batch, recognized: 0, quiet_cancel: false, saved_as, from_keyboard };
		let worker = thread::Builder::new()
			.name("paperback-ocr".into())
			.spawn(move || run_ocr(&path_str, password.as_deref(), &pages, &path, &worker_cancel, batch, id));
		match worker {
			Ok(_) => {
				if let Some(tab) = self.active_tab_mut() {
					tab.ocr_job = Some(job);
				} else {
					job.cancel.store(true, Ordering::Relaxed);
				}
			}
			Err(err) => {
				tracing::warn!(error = %err, "failed to spawn the ocr worker thread");
				announce_waiting_for_command(label, from_keyboard, ocr::message(&OcrError::Failed(err.to_string())));
			}
		}
	}

	/// By job id, not path: closing a tab drops its job but not the worker, which can outlive its document and find the same path reopened.
	fn tab_running_job(&self, id: u64) -> Option<usize> {
		let job_ids: Vec<Option<u64>> = self.tabs.iter().map(|tab| tab.ocr_job.as_ref().map(|job| job.id)).collect();
		index_running_job(&job_ids, id)
	}

	/// The worker only checks the cancel flag between pages, so pages recognized before F5 can still arrive after it and are dropped here.
	pub(crate) fn apply_ocr_results(&mut self, id: u64, file_path: &Path, results: Vec<PageResult>) {
		let label = self.live_region_label;
		let Some(index) = self.tab_running_job(id) else {
			tracing::debug!(path = %file_path.display(), job = id, "dropping an ocr flush for a job that is no longer open");
			return;
		};
		if self.tabs[index].ocr_job.as_ref().is_some_and(|job| job.quiet_cancel) {
			tracing::debug!(path = %file_path.display(), "dropping an ocr flush for a job the reader stopped by re-reading");
			return;
		}
		let is_active = self.active_tab_index() == Some(index);
		let from_keyboard = self.tabs[index].ocr_job.as_ref().is_none_or(|job| job.from_keyboard);
		let join = joins_recognized_text(self.tabs[index].local_path(), &self.config.lock().unwrap());
		let tab = &mut self.tabs[index];
		let mut pages = Vec::with_capacity(results.len());
		let mut read = Vec::with_capacity(results.len());
		// Read now rather than captured at job start, since earlier applies in this job have moved everything after them.
		let mut starts = Vec::with_capacity(results.len());
		for (page, result) in results {
			let lines = match result {
				Ok(lines) => lines,
				Err(err) => {
					tracing::warn!(page, error = %err, "ocr failed on page");
					if matches!(err, OcrError::NoLanguage) {
						announce_waiting_for_command(label, from_keyboard, ocr::message(&err));
					}
					continue;
				}
			};
			let text = page_text(&lines, join);
			// An empty result keeps the page's old text, since blanking a text page the reader distrusts would be strictly worse.
			if text.trim().is_empty() {
				continue;
			}
			// A page the session no longer holds has no position, so its result is dropped instead of panicking.
			if let Some(start) = tab.session.page_position(page) {
				starts.push(start);
				pages.push((page, text));
				read.push((page, lines));
			}
		}
		if pages.is_empty() {
			return;
		}
		let caret = tab.window.to_doc(tab.text_ctrl.get_insertion_point());
		let loaded_end = tab.window.end();
		let outcome = tab.session.replace_ocr_pages(&pages);
		if let Some(job) = tab.ocr_job.as_mut() {
			job.recognized += pages.len();
			if let Err(error) = save_pages(&job.saved_as, &read) {
				tracing::warn!(path = %job.saved_as.display(), %error, "failed to keep the recognized pages");
			}
		}
		let buffer = &tab.session.handle().document().buffer;
		let caret = outcome.shift(caret.min(buffer.end()));
		// The mark is document-absolute, so the insertions above moved it like the caret.
		if let Some(mark) = tab.selection_mark.get() {
			tab.selection_mark.set(Some(outcome.shift(mark.min(buffer.end()))));
		}
		refresh_after_ocr(tab, &starts, loaded_end, caret, is_active);
		// The config's reading position, history and bookmarks are document-absolute, so the edits above moved them too.
		self.config.lock().unwrap().shift_document_positions(&file_path.to_string_lossy(), &outcome);
	}

	/// Silent for a closed or re-read job, or a queued count would cut off "Document reloaded."
	pub(crate) fn announce_ocr_progress(&self, id: u64, done: usize, total: usize) {
		let Some(index) = self.tab_running_job(id) else {
			return;
		};
		if self.tabs[index].ocr_job.as_ref().is_some_and(|job| job.quiet_cancel) {
			return;
		}
		// TRANSLATORS: Batch OCR progress announcement; the two %d placeholders are the pages done and the total pages
		let message = t("OCR %d of %d.").replacen("%d", &done.to_string(), 1).replacen("%d", &total.to_string(), 1);
		announce(self.live_region_label, message);
	}

	pub(crate) fn finish_ocr(&mut self, id: u64, file_path: &Path, canceled: bool) {
		let label = self.live_region_label;
		let Some(index) = self.tab_running_job(id) else {
			tracing::debug!(path = %file_path.display(), job = id, "ocr finished for a job that is no longer open");
			return;
		};
		let tab = &mut self.tabs[index];
		let Some(job) = tab.ocr_job.take() else {
			return;
		};
		if job.quiet_cancel {
			return;
		}
		if !job.batch {
			let message = if job.recognized > 0 {
				// TRANSLATORS: Announced after OCR replaces an image-only page with the recognized text
				t("OCR complete.")
			} else {
				// TRANSLATORS: Announced when OCR runs on a page but finds no recognizable text
				t("No text found.")
			};
			announce_waiting_for_command(label, job.from_keyboard, message);
			return;
		}
		let count = u64::try_from(job.recognized).unwrap_or(0);
		let message = if canceled {
			// TRANSLATORS: Announced when batch OCR is stopped early; %d is the number of pages recognized before it stopped
			nt("Batch OCR stopped. %d page recognized.", "Batch OCR stopped. %d pages recognized.", count)
		} else {
			// TRANSLATORS: Announced after batch OCR finishes; %d is the number of pages recognized
			nt("Batch OCR complete. %d page recognized.", "Batch OCR complete. %d pages recognized.", count)
		};
		announce(label, message.replacen("%d", &job.recognized.to_string(), 1));
	}
}

/// `loaded_end` is where the window ended before the replacement; any page at or before it shifted local offsets, so the window must reload.
fn refresh_after_ocr(tab: &mut DocumentTab, starts: &[DocPos], loaded_end: DocPos, caret: DocPos, is_active: bool) {
	if !starts.iter().any(|start| *start < loaded_end) {
		return;
	}
	let caret = caret.min(tab.session.document_len().as_pos());
	reload_window_around(tab, caret, "ocr");
	if is_active {
		let local = tab.window.to_local(caret);
		tab.text_ctrl.set_insertion_point(local);
		tab.text_ctrl.show_position(local);
	}
}

fn run_ocr(
	path_str: &str,
	password: Option<&str>,
	pages: &[i32],
	file_path: &Path,
	cancel: &AtomicBool,
	batch: bool,
	id: u64,
) {
	let max_dimension = ocr::max_image_dimension();
	let total = pages.len();
	let mut pending: Vec<PageResult> = Vec::new();
	let mut done = 0usize;
	let mut last_update = Instant::now();
	let run = recognize_pages(
		path_str,
		password,
		pages,
		default_workers(),
		cancel,
		|renderer, page| {
			// `render` takes a 0-based page index; the job list is 1-based.
			renderer
				.render(page - 1, max_dimension)
				.map_err(|error| OcrError::Failed(error.to_string()))
				.and_then(|rendered| ocr::recognize_lines(&rendered.rgba, rendered.width, rendered.height))
		},
		|page, result| {
			done += 1;
			pending.push((page, result));
			if done != total && last_update.elapsed() >= BATCH_OCR_UPDATE_EVERY {
				post_apply(id, file_path, std::mem::take(&mut pending));
				if batch {
					post_progress(id, done, total);
				}
				last_update = Instant::now();
			}
		},
	);
	match run {
		Ok(run) => {
			if !pending.is_empty() {
				post_apply(id, file_path, pending);
			}
			post_finish(id, file_path, run.stopped);
		}
		Err(error) => {
			tracing::warn!(error = %error, "failed to open the document for ocr");
			post_finish(id, file_path, false);
		}
	}
}

fn post_apply(id: u64, file_path: &Path, results: Vec<PageResult>) {
	let path = file_path.to_path_buf();
	run_on_ui_thread(move |manager| manager.apply_ocr_results(id, &path, results));
}

fn post_progress(id: u64, done: usize, total: usize) {
	run_on_ui_thread(move |manager| manager.announce_ocr_progress(id, done, total));
}

fn post_finish(id: u64, file_path: &Path, canceled: bool) {
	let path: PathBuf = file_path.to_path_buf();
	run_on_ui_thread(move |manager| manager.finish_ocr(id, &path, canceled));
}

/// Calls land in order, which lets a flush rely on the previous flush having already moved the offsets it resolves.
fn run_on_ui_thread(action: impl FnOnce(&mut DocumentManager) + Send + 'static) {
	wxdragon::call_after(Box::new(move || {
		if let Some(window) = crate::ui::app::main_window_from_ptr() {
			let mut manager = window.document_manager().lock().unwrap();
			action(&mut manager);
		}
	}));
	wxdragon::wake_up_idle();
}

#[cfg(test)]
mod tests {
	use std::io::{Cursor, Write as _};

	use paperback_core::{
		document::ParseSettings,
		ocr::{RecognizedLine, RecognizedWord, save_pages},
		session::DocumentSession,
	};

	use super::{index_running_job, restore_saved_pages};

	fn comic(dir: &std::path::Path, pages: usize) -> String {
		let mut png = Vec::new();
		image::RgbaImage::new(4, 4).write_to(&mut Cursor::new(&mut png), image::ImageFormat::Png).unwrap();
		let path = dir.join("comic.cbz");
		let mut zip = zip::ZipWriter::new(std::fs::File::create(&path).unwrap());
		for page in 1..=pages {
			zip.start_file(format!("{page:03}.png"), zip::write::SimpleFileOptions::default()).unwrap();
			zip.write_all(&png).unwrap();
		}
		zip.finish().unwrap();
		path.to_string_lossy().into_owned()
	}

	#[test]
	fn a_kept_page_comes_back_in_place_of_its_placeholder() {
		let dir = std::env::temp_dir().join(format!("paperback-restore-saved-{}", std::process::id()));
		let _ = std::fs::remove_dir_all(&dir);
		std::fs::create_dir_all(&dir).unwrap();
		let path = comic(&dir, 2);
		let mut session = DocumentSession::new(&path, "", "", ParseSettings::default()).unwrap();
		assert_eq!(session.image_only_pages().len(), 2);
		let saved = dir.join("comic.ocr");
		let word = RecognizedWord { text: "Recognized".to_string(), x: 0.0, y: 0.0, width: 80.0, height: 16.0 };
		save_pages(&saved, &[(2, vec![RecognizedLine { text: "Recognized".to_string(), words: vec![word] }])]).unwrap();
		restore_saved_pages(&mut session, &saved, false);
		assert!(session.content().contains("Recognized"), "{}", session.content());
		assert_eq!(session.image_only_pages().iter().map(|(page, _)| *page).collect::<Vec<_>>(), vec![1]);
	}

	#[test]
	fn a_worker_whose_tab_was_closed_finds_nothing_when_the_document_is_reopened() {
		// Tab 0, reopened, is running job 2. The worker's own job 1 is gone with the closed tab.
		let open = [Some(2)];
		assert_eq!(index_running_job(&open, 1), None);
		assert_eq!(index_running_job(&open, 2), Some(0));
	}

	#[test]
	fn an_idle_tab_is_never_the_tab_a_worker_means() {
		let open = [None, None, None];
		for id in 0..3 {
			assert_eq!(index_running_job(&open, id), None);
		}
	}

	#[test]
	fn the_job_s_own_tab_is_found_among_several_open_documents() {
		let open = [None, Some(7), Some(3)];
		assert_eq!(index_running_job(&open, 7), Some(1));
		assert_eq!(index_running_job(&open, 3), Some(2));
		assert_eq!(index_running_job(&open, 4), None);
	}
}
