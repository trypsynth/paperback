//! Bytes are read on the UI thread because the session isn't `Send`; decoding and recognition go to a worker since the engine's first call costs a few hundred milliseconds.

use std::{path::PathBuf, thread};

use paperback_core::ocr::{decode_image, page_text};
use patois::t;

use super::DocumentManager;
use crate::{
	ocr::{self, OcrError},
	ui::{dialogs::show_image_text_dialog, navigation::announce},
};

enum ImageOcrOutcome {
	Text(String),
	NoText,
	Unreadable,
	Failed(OcrError),
}

impl DocumentManager {
	pub fn read_picture_at_caret(&mut self) {
		let label = self.live_region_label;
		// A second Enter is answered rather than queued, since the reader has nothing else to do until the view opens.
		if self.image_ocr_job.is_some() {
			// TRANSLATORS: Announced when Enter is pressed on a picture while its text is already being recognized
			announce(label, t("OCR in progress."));
			return;
		}
		let Some(tab) = self.active_tab() else { return };
		let position = tab.window.to_doc(tab.text_ctrl.get_insertion_point());
		let bytes = match tab.session.image_bytes_at(position) {
			Ok(bytes) => bytes,
			Err(error) => {
				tracing::warn!(%error, "the picture under the caret could not be read");
				// TRANSLATORS: Announced when the picture under the caret cannot be read out of the document
				announce(label, t("The picture could not be read."));
				return;
			}
		};
		let id = self.next_job_id.get();
		self.next_job_id.set(id.wrapping_add(1));
		self.image_ocr_job = Some((id, tab.local_path().to_path_buf()));
		let max_dimension = ocr::max_image_dimension();
		let spawned = thread::Builder::new()
			.name("paperback-image-ocr".into())
			.spawn(move || run_image_ocr(&bytes, max_dimension, id));
		if let Err(error) = spawned {
			self.image_ocr_job = None;
			announce(label, ocr::message(&OcrError::Failed(error.to_string())));
		}
	}

	fn take_finished_image_ocr(&mut self, id: u64) -> Option<PathBuf> {
		match self.image_ocr_job.take() {
			Some((job_id, path)) if job_id == id => Some(path),
			other => {
				self.image_ocr_job = other;
				None
			}
		}
	}
}

fn run_image_ocr(bytes: &[u8], max_dimension: u32, id: u64) {
	let outcome = match decode_image(bytes, max_dimension) {
		Ok(page) => {
			match ocr::recognize_lines(&page.rgba, page.width, page.height).map(|lines| page_text(&lines, true)) {
				Ok(text) if text.trim().is_empty() => ImageOcrOutcome::NoText,
				Ok(text) => ImageOcrOutcome::Text(text),
				Err(error) => ImageOcrOutcome::Failed(error),
			}
		}
		Err(error) => {
			tracing::warn!(%error, "the picture under the caret could not be decoded for ocr");
			ImageOcrOutcome::Unreadable
		}
	};
	post_image_ocr_result(id, outcome);
}

fn post_image_ocr_result(id: u64, outcome: ImageOcrOutcome) {
	wxdragon::call_after(Box::new(move || {
		let Some(window) = crate::ui::app::main_window_from_ptr() else { return };
		// The lock is released before the dialog opens, since its handlers lock the manager again and would deadlock.
		let (frame, label) = {
			let mut manager = window.document_manager().lock().unwrap();
			let Some(path) = manager.take_finished_image_ocr(id) else { return };
			if manager.active_tab().is_none_or(|tab| tab.local_path() != path) {
				return;
			}
			(manager.frame, manager.live_region_label)
		};
		match outcome {
			ImageOcrOutcome::Text(text) => show_image_text_dialog(&frame, &text),
			// TRANSLATORS: Announced when a picture is read and the OCR engine finds no text in it
			ImageOcrOutcome::NoText => announce(label, t("No text was found in the picture.")),
			// TRANSLATORS: Announced when the picture under the caret is not an image this build can decode
			ImageOcrOutcome::Unreadable => announce(label, t("The picture could not be read.")),
			ImageOcrOutcome::Failed(error) => announce(label, ocr::message(&error)),
		}
	}));
	wxdragon::wake_up_idle();
}
