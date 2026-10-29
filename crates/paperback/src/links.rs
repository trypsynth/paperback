use std::{
	path::Path,
	sync::{
		Arc,
		atomic::{AtomicBool, Ordering},
		mpsc::{self, RecvTimeoutError},
	},
	thread,
	time::Duration,
};

pub use paperback_core::fetch::{not_a_link, prefill, warning_text};
use paperback_core::{fetch::FetchError, parser::parser_supports_path};
use patois::t;

use crate::working_copy::WorkingCopy;

#[derive(Debug, Clone, Copy)]
pub enum Stage {
	Checking,
	Downloading,
}

pub fn step(stage: Stage, n: usize, m: usize, subject: &str) -> String {
	let template = match (stage, m > 1) {
		// TRANSLATORS: Download window message while Paperback asks a web server about a link; %s is the link
		(Stage::Checking, false) => t("Checking %s"),
		// TRANSLATORS: Download window message while Paperback asks a web server about one of several links; the first %d is the link's number, the second how many links there are, and %s is the link
		(Stage::Checking, true) => t("Checking %d of %d: %s"),
		// TRANSLATORS: Download window message while a document opened from a link downloads; %s is its file name
		(Stage::Downloading, false) => t("Downloading %s"),
		// TRANSLATORS: Download window message while one of several documents opened from links downloads; the first %d is its number, the second how many there are, and %s is its file name
		(Stage::Downloading, true) => t("Downloading %d of %d: %s"),
	};
	// The subject goes in last, so a % sign in a link is never taken for a placeholder.
	template.replacen("%d", &n.to_string(), 1).replacen("%d", &m.to_string(), 1).replacen("%s", subject, 1)
}

/// How often a step waiting on the network looks for Cancel: soon enough that the window seems to close at once.
const CANCEL_POLL: Duration = Duration::from_millis(100);

/// Runs `step` on a thread of its own and waits for it, passing on its progress, unless `cancelled` turns true first.
///
/// A server can take minutes to answer or to send the next part of a document, and nothing can interrupt a read that is waiting on one. So Cancel stops the waiting instead: `step` is told through its flag, and whatever it returns once the server does answer is dropped.
pub fn unless_cancelled<T: Send + 'static>(
	cancelled: impl Fn() -> bool,
	mut on_progress: impl FnMut(u64, Option<u64>),
	step: impl FnOnce(&AtomicBool, &dyn Fn(u64, Option<u64>)) -> T + Send + 'static,
) -> Option<T> {
	let cancel = Arc::new(AtomicBool::new(false));
	let step_cancel = Arc::clone(&cancel);
	let (result_tx, result_rx) = mpsc::channel();
	let (moved_tx, moved_rx) = mpsc::channel();
	thread::spawn(move || {
		let result = step(&step_cancel, &|done, total| {
			let _ = moved_tx.send((done, total));
		});
		let _ = result_tx.send(result);
	});
	loop {
		while let Ok((done, total)) = moved_rx.try_recv() {
			on_progress(done, total);
		}
		match result_rx.recv_timeout(CANCEL_POLL) {
			Ok(result) => return Some(result),
			Err(RecvTimeoutError::Timeout) if cancelled() => {
				cancel.store(true, Ordering::Relaxed);
				return None;
			}
			Err(RecvTimeoutError::Timeout) => {}
			Err(RecvTimeoutError::Disconnected) => {
				tracing::warn!("a link download's network thread ended without a result");
				return None;
			}
		}
	}
}

pub fn needs_open_as(warned: bool, copy: &Path) -> bool {
	warned || !parser_supports_path(copy)
}

pub enum Outcome {
	Downloaded { link: String, copy: WorkingCopy, warned: bool },
	Failed { link: String, error: FetchError },
	Skipped,
}

pub fn report(outcomes: &[Outcome]) -> Vec<String> {
	outcomes
		.iter()
		.filter_map(|outcome| match outcome {
			Outcome::Failed { link, error } => Some(format!("{link}: {error}")),
			Outcome::Downloaded { .. } | Outcome::Skipped => None,
		})
		.collect()
}

#[cfg(test)]
mod tests {
	use rstest::rstest;

	use super::*;
	use crate::{test_locale, working_copy::WorkingCopy};

	#[test]
	fn a_step_s_result_and_progress_come_back_when_nothing_cancels_it() {
		let mut seen = Vec::new();
		let result = unless_cancelled(
			|| false,
			|done, total| seen.push((done, total)),
			|_, progress| {
				progress(5, Some(10));
				progress(10, Some(10));
				thread::sleep(CANCEL_POLL * 2);
				"saved"
			},
		);
		assert_eq!(result, Some("saved"));
		assert_eq!(seen, vec![(5, Some(10)), (10, Some(10))]);
	}

	#[test]
	fn cancel_stops_waiting_on_a_step_that_is_still_blocked_and_tells_it() {
		let told = Arc::new(AtomicBool::new(false));
		let told_by_step = Arc::clone(&told);
		let started = std::time::Instant::now();
		let result = unless_cancelled(
			|| started.elapsed() > CANCEL_POLL,
			|_, _| {},
			move |cancel, _| {
				while !cancel.load(Ordering::Relaxed) {
					thread::sleep(Duration::from_millis(10));
				}
				told_by_step.store(true, Ordering::Relaxed);
			},
		);
		assert_eq!(result, None);
		assert!(started.elapsed() < Duration::from_secs(2), "{:?}", started.elapsed());
		thread::sleep(CANCEL_POLL);
		assert!(told.load(Ordering::Relaxed));
	}

	#[test]
	fn prefill_takes_the_first_link_on_the_clipboard() {
		assert_eq!(prefill("notes\r\n  https://a.org/x.epub \nhttps://b.org/y.pdf"), "https://a.org/x.epub");
	}

	#[test]
	fn prefill_ignores_a_link_inside_a_sentence() {
		assert_eq!(prefill("Read this: https://a.org/x.epub"), "");
	}

	#[test]
	fn not_a_link_names_what_was_typed() {
		let _locale = test_locale::pinned_to("en");
		assert_eq!(not_a_link("see chapter 3"), "see chapter 3: this is not a web link");
	}

	#[rstest]
	#[case::one_link(Stage::Checking, 1, 1, "https://a.org/x.epub", "Checking https://a.org/x.epub")]
	#[case::one_of_several(Stage::Downloading, 2, 5, "book.epub", "Downloading 2 of 5: book.epub")]
	#[case::one_download(Stage::Downloading, 1, 1, "book.epub", "Downloading book.epub")]
	#[case::percent_signs_in_the_link(
		Stage::Checking,
		1,
		2,
		"https://a.org/100%d%s.pdf",
		"Checking 1 of 2: https://a.org/100%d%s.pdf"
	)]
	fn step_names_the_link_and_its_place(
		#[case] stage: Stage,
		#[case] n: usize,
		#[case] m: usize,
		#[case] subject: &str,
		#[case] expected: &str,
	) {
		let _locale = test_locale::pinned_to("en");
		assert_eq!(step(stage, n, m, subject), expected);
	}

	#[rstest]
	#[case::a_web_page(
		Some("text/html; charset=utf-8"),
		"https://a.org/book.pdf\nThe server sends a web page (text/html; charset=utf-8) instead of the document the link names.\nDownload it anyway?"
	)]
	#[case::a_type_paperback_does_not_read(
		Some("application/octet-stream"),
		"https://a.org/book.pdf\nThe server sends application/octet-stream, which is not a format Paperback reads.\nDownload it anyway?"
	)]
	#[case::no_type(None, "https://a.org/book.pdf\nThe server does not say what it sends.\nDownload it anyway?")]
	fn warning_text_says_what_the_server_sends(#[case] content_type: Option<&str>, #[case] expected: &str) {
		let _locale = test_locale::pinned_to("en");
		assert_eq!(warning_text("https://a.org/book.pdf", content_type), expected);
	}

	#[rstest]
	#[case::a_name_paperback_reads(false, "book.epub", false)]
	#[case::the_reader_accepted_the_warning(true, "book.epub", true)]
	#[case::a_name_without_an_extension(false, "download", true)]
	#[case::a_name_paperback_does_not_read(false, "download.php", true)]
	#[case::a_gzipped_man_page(false, "ls.1.gz", false)]
	fn needs_open_as_after_a_warning_or_for_a_file_paperback_does_not_read(
		#[case] warned: bool,
		#[case] copy_name: &str,
		#[case] expected: bool,
	) {
		assert_eq!(needs_open_as(warned, Path::new(copy_name)), expected);
	}

	#[test]
	fn report_lists_what_was_not_opened_and_why() {
		let _locale = test_locale::pinned_to("en");
		let outcomes = vec![
			Outcome::Downloaded {
				link: "https://a.org/x.epub".to_string(),
				copy: WorkingCopy::new("x.epub").unwrap(),
				warned: false,
			},
			Outcome::Failed {
				link: "https://a.org/setup.exe".to_string(),
				error: FetchError::Refused("exe".to_string()),
			},
			Outcome::Failed { link: "https://a.org/gone.pdf".to_string(), error: FetchError::Http(404) },
			Outcome::Skipped,
		];
		assert_eq!(
			report(&outcomes),
			vec![
				"https://a.org/setup.exe: .exe files cannot be read, so the link was not downloaded",
				"https://a.org/gone.pdf: the server answered with status 404",
			]
		);
	}
}
