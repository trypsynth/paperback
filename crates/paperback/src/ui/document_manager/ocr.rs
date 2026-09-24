//! Running the built-in OCR engine over pages that hold a picture and no text, one page at a
//! time or a range in bulk, and folding the recognized text back into the open document.
//!
//! Everything that touches a `DocumentTab` runs on the UI thread. The worker thread holds only
//! the file path and a list of page numbers: it renders and recognizes, then hands text back
//! through `wxdragon::call_after`. Pages are addressed by number rather than by offset for a
//! reason. Applying one page's text moves every offset after it, so an offset captured before
//! the job started would be stale by the time the worker's next result lands. Page numbers stay
//! true, and the UI thread re-resolves each one's placeholder as it applies it.

use std::{
	path::{Path, PathBuf},
	sync::{
		Arc,
		atomic::{AtomicBool, Ordering},
	},
	thread,
};

use paperback_core::{ocr::PageRenderer, parser::pdf::join_wrapped_lines};
use patois::{nt, t};
use wxdragon::prelude::*;

use super::{DocumentManager, DocumentTab};
use crate::{
	ocr::{self, OcrError},
	text_window::TextWindow,
	ui::{navigation::announce, text_render::reload_window_around},
};

/// How many pages between spoken progress announcements during a batch OCR job.
const BATCH_OCR_PROGRESS_EVERY: usize = 20;

/// How many recognized pages to hold before folding them into the document together.
///
/// Every apply rebuilds the buffer's per-char index tables, which is linear in the whole
/// document, so applying page by page would be quadratic over a long scan. Flushing in groups
/// keeps that cost proportional to the document while still letting the reader reach the early
/// pages long before a several-hundred-page job finishes. Matched to the progress interval so
/// the text appears exactly when the progress announcement says it has.
const BATCH_OCR_FLUSH_EVERY: usize = BATCH_OCR_PROGRESS_EVERY;

/// A running OCR job, one per tab. Owned by the UI thread; the worker sees only `cancel`.
pub(super) struct OcrJob {
	/// Raised by the UI thread to stop a batch after the page it is working on.
	cancel: Arc<AtomicBool>,
	/// True for a range job, false for a single page. Only a batch is worth offering to cancel.
	batch: bool,
	/// Pages whose text has made it into the document so far, for the closing announcement.
	recognized: usize,
}

/// Whether a document is a PDF, which is what the paragraph joining setting is about. The
/// other thing OCR runs on is a comic, whose lines are the balloons and belong apart.
fn is_pdf(file_path: &Path) -> bool {
	file_path.extension().is_some_and(|extension| extension.eq_ignore_ascii_case("pdf"))
}

/// Whether [`PageRenderer::open`] can actually open this document. It rasterizes PDFs and decodes
/// comic archive pages, and hands anything else to pdfium, which will not read an EPUB or a DOCX.
/// "Include text pages" collects every page in the range, so without this a book with no
/// image-only pages at all would send a job straight into a renderer that cannot open it.
fn is_ocr_able(file_path: &Path) -> bool {
	file_path.extension().is_some_and(|extension| {
		extension.eq_ignore_ascii_case("pdf")
			|| extension.eq_ignore_ascii_case("cbz")
			|| extension.eq_ignore_ascii_case("cbr")
	})
}

/// One page's outcome, as the worker reports it.
type PageResult = (i32, Result<String, OcrError>);

impl DocumentManager {
	/// The offset of the OCR placeholder the caret is sitting on, if it is on one.
	pub fn image_only_page_at_caret(&self) -> Option<i64> {
		let tab = self.active_tab()?;
		let position = tab.window.to_doc(tab.text_ctrl.get_insertion_point());
		tab.session.image_only_page_at(position)
	}

	/// Whether a batch OCR job is running for the active tab, and so worth offering to cancel.
	pub fn batch_ocr_running(&self) -> bool {
		self.active_tab().is_some_and(|tab| tab.ocr_job.as_ref().is_some_and(|job| job.batch))
	}

	/// Asks a running batch to stop after the page it is on. The completion path announces how
	/// much was recognized before the stop, so there is nothing to announce here.
	pub fn cancel_ocr(&mut self) {
		if let Some(job) = self.active_tab().and_then(|tab| tab.ocr_job.as_ref()) {
			job.cancel.store(true, Ordering::Relaxed);
		}
	}

	/// Runs OCR on the image-only page the caret is on.
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
		self.spawn_ocr(vec![page], false);
	}

	/// Runs OCR on every image-only page in `start..=end` (1-based page numbers), or on every
	/// page in the range when `include_text_pages` is set.
	///
	/// The wider sweep exists for pages whose text came from an older or worse OCR pass, or that
	/// carry no text for material living only in the page's images. It still includes the
	/// image-only pages, which are a subset of the range, so one list covers both kinds.
	///
	/// A document the renderer cannot open keeps the narrow behaviour even with the box ticked:
	/// there is nothing to render a page of an EPUB with, and reporting a completed job that
	/// recognized nothing would be worse than saying the range held no image-only pages.
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
		let pages: Vec<i32> = if include_text_pages && is_ocr_able(&tab.file_path) {
			let last = i32::try_from(tab.session.page_count()).unwrap_or(i32::MAX);
			// Left to go empty when the range starts past the last page, which is what sends it
			// to the "nothing to do" announcement below rather than to a job for a page the
			// document does not have.
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
		self.spawn_ocr(pages, true);
	}

	/// Starts the worker for `pages` and records the job on the active tab.
	fn spawn_ocr(&mut self, pages: Vec<i32>, batch: bool) {
		let label = self.live_region_label;
		let Some(tab) = self.active_tab() else {
			return;
		};
		let path = tab.file_path.clone();
		let path_str = path.to_string_lossy().to_string();
		let password = self.config.lock().unwrap().get_document_password(&path_str);
		let password = (!password.is_empty()).then_some(password);
		let cancel = Arc::new(AtomicBool::new(false));
		let worker_cancel = Arc::clone(&cancel);
		let worker = thread::Builder::new()
			.name("paperback-ocr".into())
			.spawn(move || run_ocr(&path_str, password.as_deref(), &pages, &path, &worker_cancel, batch));
		match worker {
			Ok(_) => {
				if let Some(tab) = self.active_tab_mut() {
					tab.ocr_job = Some(OcrJob { cancel, batch, recognized: 0 });
				}
			}
			Err(err) => {
				tracing::warn!(error = %err, "failed to spawn the ocr worker thread");
				announce(label, OcrError::Failed(err.to_string()).message());
			}
		}
	}

	/// Folds a group of recognized pages into the document. Called on the UI thread from the
	/// worker, once per flush.
	pub(crate) fn apply_ocr_results(&mut self, file_path: &Path, results: Vec<PageResult>) {
		let label = self.live_region_label;
		let Some(index) = self.tabs.iter().position(|tab| tab.file_path.as_path() == file_path) else {
			return;
		};
		let is_active = self.active_tab_index() == Some(index);
		// An OCR engine reads a page a printed line at a time, so its text arrives broken at
		// every line end. The reader who asked for wrapped lines to be joined asked about the
		// text of a page, not about where it was read from.
		let join_paragraphs =
			is_pdf(file_path) && self.config.lock().unwrap().get_app_bool("join_pdf_paragraphs", true);
		let tab = &mut self.tabs[index];
		let mut pages = Vec::with_capacity(results.len());
		// Where each page being replaced starts, read before the edit for the same reason the
		// session re-resolves the span rather than trusting a captured offset: earlier applies in
		// this job have already moved everything after them.
		let mut starts = Vec::with_capacity(results.len());
		for (page, result) in results {
			let text = match result {
				Ok(text) => text,
				Err(err) => {
					tracing::warn!(page, error = %err, "ocr failed on page");
					if matches!(err, OcrError::NoLanguage) {
						announce(label, err.message());
					}
					continue;
				}
			};
			// A page that came back empty keeps the text it had. For an image-only page that is
			// the placeholder; for a text page it is the text the reader is distrusting, and
			// blanking it would be strictly worse than leaving it alone.
			if text.trim().is_empty() {
				continue;
			}
			let text = if join_paragraphs { join_wrapped_lines(&text) } else { text };
			// `page_offset` answers -1 for a page this document does not have, which is how a
			// result for a page the session no longer holds gets dropped instead of panicking.
			let start = tab.session.page_offset(page);
			if start >= 0 {
				starts.push(start);
				pages.push((page, text));
			}
		}
		if pages.is_empty() {
			return;
		}
		let caret = tab.window.to_doc(tab.text_ctrl.get_insertion_point());
		let window = tab.window;
		let outcome = tab.session.replace_ocr_pages(&pages);
		if let Some(job) = tab.ocr_job.as_mut() {
			job.recognized += pages.len();
		}
		let caret = i64::try_from(outcome.shift(usize::try_from(caret.max(0)).unwrap_or(0))).unwrap_or(caret);
		// The mark is a document-absolute offset like the caret's, so the insertions above moved
		// it the same way. Left where it was it would name different text once OCR had finished.
		if let Some(mark) = tab.selection_mark.get() {
			let shifted = outcome.shift(usize::try_from(mark.max(0)).unwrap_or(0));
			tab.selection_mark.set(Some(i64::try_from(shifted).unwrap_or(mark)));
		}
		refresh_after_ocr(tab, &starts, window, caret, is_active);
		// The config stores document-absolute offsets (reading position, navigation history,
		// bookmarks), every one of which the edits above just moved.
		self.config.lock().unwrap().shift_document_positions(&file_path.to_string_lossy(), &outcome);
	}

	/// Announces batch progress. Called on the UI thread from the worker.
	pub(crate) fn announce_ocr_progress(&self, done: usize, total: usize) {
		// TRANSLATORS: Batch OCR progress announcement; the two %d placeholders are the pages done and the total pages
		let message = t("OCR %d of %d.").replacen("%d", &done.to_string(), 1).replacen("%d", &total.to_string(), 1);
		announce(self.live_region_label, message);
	}

	/// Clears the job and announces the outcome. Called on the UI thread when the worker ends.
	pub(crate) fn finish_ocr(&mut self, file_path: &Path, canceled: bool) {
		let label = self.live_region_label;
		let Some(tab) = self.tabs.iter_mut().find(|tab| tab.file_path.as_path() == file_path) else {
			return;
		};
		let Some(job) = tab.ocr_job.take() else {
			return;
		};
		if !job.batch {
			let message = if job.recognized > 0 {
				// TRANSLATORS: Announced after OCR replaces an image-only page with the recognized text
				t("OCR complete.")
			} else {
				// TRANSLATORS: Announced when OCR runs on a page but finds no recognizable text
				t("No text found.")
			};
			announce(label, message);
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

/// Reloads the on-screen window when a replaced page was in or before it, so the recognized text
/// appears and local offsets keep matching document ones.
fn refresh_after_ocr(tab: &mut DocumentTab, starts: &[i64], window: TextWindow, caret: i64, is_active: bool) {
	if !starts.iter().any(|start| *start < window.end()) {
		return;
	}
	let caret = caret.clamp(0, tab.session.document_len());
	reload_window_around(tab, caret, "ocr");
	if is_active {
		let local = tab.window.to_local(caret);
		tab.text_ctrl.set_insertion_point(local);
		tab.text_ctrl.show_position(local);
	}
}

/// The worker thread: opens the document once, then renders and recognizes each page in turn,
/// posting results back to the UI thread in flushes.
fn run_ocr(path_str: &str, password: Option<&str>, pages: &[i32], file_path: &Path, cancel: &AtomicBool, batch: bool) {
	let mut renderer = match PageRenderer::open(path_str, password) {
		Ok(renderer) => renderer,
		Err(err) => {
			tracing::warn!(error = %err, "failed to open the document for ocr");
			post_finish(file_path, false);
			return;
		}
	};
	let total = pages.len();
	let max_dimension = ocr::max_image_dimension();
	let mut pending: Vec<PageResult> = Vec::with_capacity(BATCH_OCR_FLUSH_EVERY);
	let mut canceled = false;
	for (done, page) in pages.iter().enumerate() {
		if cancel.load(Ordering::Relaxed) {
			canceled = true;
			break;
		}
		// `render` takes a 0-based page index; the job list is 1-based.
		let result = renderer
			.render(page - 1, max_dimension)
			.map_err(|err| OcrError::Failed(err.to_string()))
			.and_then(|rendered| ocr::recognize_rgba(&rendered.rgba, rendered.width, rendered.height));
		pending.push((*page, result));
		let done = done + 1;
		if pending.len() >= BATCH_OCR_FLUSH_EVERY && done != total {
			post_apply(file_path, std::mem::take(&mut pending));
		}
		if batch && done % BATCH_OCR_PROGRESS_EVERY == 0 && done != total {
			post_progress(done, total);
		}
	}
	if !pending.is_empty() {
		post_apply(file_path, pending);
	}
	post_finish(file_path, canceled);
}

fn post_apply(file_path: &Path, results: Vec<PageResult>) {
	let path = file_path.to_path_buf();
	run_on_ui_thread(move |manager| manager.apply_ocr_results(&path, results));
}

fn post_progress(done: usize, total: usize) {
	run_on_ui_thread(move |manager| manager.announce_ocr_progress(done, total));
}

fn post_finish(file_path: &Path, canceled: bool) {
	let path: PathBuf = file_path.to_path_buf();
	run_on_ui_thread(move |manager| manager.finish_ocr(&path, canceled));
}

/// Queues `action` to run on the UI thread with the document manager locked. Every call lands in
/// order behind the ones before it, which is what lets a flush rely on the previous flush having
/// already moved the offsets it is about to resolve.
fn run_on_ui_thread(action: impl FnOnce(&mut DocumentManager) + Send + 'static) {
	wxdragon::call_after(Box::new(move || {
		if let Some(window) = crate::ui::app::main_window_from_ptr() {
			let mut manager = window.document_manager().lock().unwrap();
			action(&mut manager);
		}
	}));
	wxdragon::wake_up_idle();
}
