//! Reading a document's pages on several threads at once.
//!
//! Recognizing scales across threads but rasterizing does not, because `pdfium-render`'s `thread_safe` feature serializes every call through one process-wide lock. So each worker both rasterizes and reads its own pages with a renderer of its own, and one worker's rasterizing overlaps another's reading.
//!
//! Pages are handed back in page order however they finish, because the app refreshes the reader's window and counts progress on the assumption that pages land in ascending order.

use std::{
	collections::HashMap,
	sync::{
		Mutex,
		atomic::{AtomicBool, AtomicUsize, Ordering},
		mpsc,
	},
	thread,
};

use crate::ocr::PageRenderer;

/// Past this, pages queue behind pdfium's lock, so more threads buy memory and contention rather than throughput.
const MAX_WORKERS: usize = 4;

/// One core is left free so the UI and the screen reader stay responsive while the pages are read.
#[must_use]
pub fn default_workers() -> usize {
	thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get).saturating_sub(1).clamp(1, MAX_WORKERS)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Run {
	/// True when the job stopped at a page boundary because `cancel` was raised, rather than
	/// reaching the end of the page list.
	pub stopped: bool,
}

/// Renders and reads `pages` (1-based page numbers) on up to `workers` threads, handing each page's result to `on_page` on the calling thread in ascending page order.
///
/// `recognize_page` is passed in because the OCR engine is platform-specific and lives in the front end. A page that fails to read still reaches `on_page`, so one bad page does not stop the rest. Raising `cancel` stops each worker at its next page boundary, and pages already read are still handed over so the caller can say how much was read.
///
/// # Errors
///
/// Returns an error if the document cannot be opened for rendering.
pub fn recognize_pages<E>(
	file_path: &str,
	password: Option<&str>,
	pages: &[i32],
	workers: usize,
	cancel: &AtomicBool,
	recognize_page: impl Fn(&mut PageRenderer, i32) -> Result<String, E> + Sync,
	mut on_page: impl FnMut(i32, Result<String, E>),
) -> anyhow::Result<Run>
where
	E: Send,
{
	if pages.is_empty() {
		return Ok(Run { stopped: false });
	}
	let workers = workers.clamp(1, pages.len());
	let cursor = AtomicUsize::new(0);
	// Every other worker would fail to open the document too, so one failure sends them all home.
	let stop = AtomicBool::new(false);
	let open_failure: Mutex<Option<anyhow::Error>> = Mutex::new(None);
	let (sender, receiver) = mpsc::channel::<(usize, Result<String, E>)>();
	let emitted = thread::scope(|scope| {
		// Borrowed so each `move` closure captures a reference rather than the first one taking the call.
		let recognize_page = &recognize_page;
		for _ in 0..workers {
			let sender = sender.clone();
			let cursor = &cursor;
			let stop = &stop;
			let open_failure = &open_failure;
			scope.spawn(move || {
				let mut renderer = match PageRenderer::open(file_path, password) {
					Ok(renderer) => renderer,
					Err(error) => {
						if let Ok(mut slot) = open_failure.lock()
							&& slot.is_none()
						{
							*slot = Some(error);
						}
						stop.store(true, Ordering::Relaxed);
						return;
					}
				};
				while !stop.load(Ordering::Relaxed) && !cancel.load(Ordering::Relaxed) {
					let index = cursor.fetch_add(1, Ordering::Relaxed);
					let Some(&page) = pages.get(index) else { break };
					if sender.send((index, recognize_page(&mut renderer, page))).is_err() {
						break;
					}
				}
			});
		}
		// Dropped so the channel closes once the workers are done.
		drop(sender);
		collect(receiver, pages, &mut on_page)
	});
	if let Ok(Some(error)) = open_failure.into_inner() {
		return Err(error);
	}
	Ok(Run { stopped: emitted < pages.len() })
}

/// Returns how many pages were handed over. A short count means the job stopped, since every claimed page is sent.
fn collect<E>(
	receiver: mpsc::Receiver<(usize, Result<String, E>)>,
	pages: &[i32],
	on_page: &mut impl FnMut(i32, Result<String, E>),
) -> usize {
	let mut held: HashMap<usize, Result<String, E>> = HashMap::new();
	let mut next = 0usize;
	let mut emitted = 0usize;
	for (index, result) in receiver {
		held.insert(index, result);
		while let Some(result) = held.remove(&next) {
			on_page(pages[next], result);
			next += 1;
			emitted += 1;
		}
	}
	// A stop can leave read pages behind an unread gap. They are still handed over, since the app reports how much was read.
	let mut rest: Vec<(usize, Result<String, E>)> = held.into_iter().collect();
	rest.sort_unstable_by_key(|(index, _)| *index);
	for (index, result) in rest {
		on_page(pages[index], result);
		emitted += 1;
	}
	emitted
}

#[cfg(test)]
mod tests {
	use std::{env, fs, io::Write as _, sync::atomic::AtomicUsize, time::Duration};

	use zip::{ZipWriter, write::SimpleFileOptions};

	use super::*;

	fn png(width: u32, height: u32) -> Vec<u8> {
		let mut buffer = Vec::new();
		let encoder = image::codecs::png::PngEncoder::new(&mut buffer);
		let pixels = vec![255u8; (width * height) as usize];
		image::ImageEncoder::write_image(encoder, &pixels, width, height, image::ExtendedColorType::L8).unwrap();
		buffer
	}

	/// A comic archive is the cheapest real document to hand the pool, since it needs no OCR engine.
	fn comic(test_name: &str, pages: usize) -> String {
		let dir = env::temp_dir().join("paperback-ocr-pool-tests");
		fs::create_dir_all(&dir).unwrap();
		let path = dir.join(format!("{test_name}.cbz"));
		let file = fs::File::create(&path).unwrap();
		let mut zip = ZipWriter::new(file);
		for page in 0..pages {
			zip.start_file(format!("{:03}.png", page + 1), SimpleFileOptions::default()).unwrap();
			zip.write_all(&png(20, 10)).unwrap();
		}
		zip.finish().unwrap();
		path.to_string_lossy().into_owned()
	}

	type Read = Result<String, String>;

	fn says_which_page(_renderer: &mut PageRenderer, page: i32) -> Read {
		Ok(format!("text of page {page}"))
	}

	fn run(path: &str, pages: &[i32], workers: usize) -> (Run, Vec<(i32, Read)>) {
		let cancel = AtomicBool::new(false);
		let mut seen = Vec::new();
		let outcome = recognize_pages(path, None, pages, workers, &cancel, says_which_page, |page, result| {
			seen.push((page, result));
		})
		.unwrap();
		(outcome, seen)
	}

	#[test]
	fn every_page_comes_back_once_with_its_own_text() {
		let path = comic("each-once", 6);
		let pages = [1, 2, 3, 4, 5, 6];
		let (outcome, seen) = run(&path, &pages, 3);
		assert!(!outcome.stopped);
		assert_eq!(seen.iter().map(|(page, _)| *page).collect::<Vec<_>>(), pages);
		for (page, result) in &seen {
			assert_eq!(result.as_deref(), Ok(format!("text of page {page}").as_str()));
		}
	}

	/// The earliest page is made the slowest, so passing results through as they arrived would come back backwards.
	#[test]
	fn pages_are_handed_over_in_page_order_however_they_finish() {
		let path = comic("order", 8);
		let cancel = AtomicBool::new(false);
		let mut seen = Vec::new();
		recognize_pages(
			&path,
			None,
			&[1, 2, 3, 4, 5, 6, 7, 8],
			4,
			&cancel,
			|_renderer, page| -> Read {
				thread::sleep(Duration::from_millis(u64::try_from(9 - page).unwrap() * 20));
				Ok(format!("text of page {page}"))
			},
			|page, _| seen.push(page),
		)
		.unwrap();
		assert_eq!(seen, vec![1, 2, 3, 4, 5, 6, 7, 8]);
	}

	/// Without this the workers could all run one after another and every other test would still pass.
	#[test]
	fn pages_are_read_on_several_threads_at_once() {
		let path = comic("concurrent", 8);
		let cancel = AtomicBool::new(false);
		let in_flight = AtomicUsize::new(0);
		let most_at_once = AtomicUsize::new(0);
		recognize_pages(
			&path,
			None,
			&[1, 2, 3, 4, 5, 6, 7, 8],
			4,
			&cancel,
			|_renderer, page| -> Read {
				let now = in_flight.fetch_add(1, Ordering::Relaxed) + 1;
				most_at_once.fetch_max(now, Ordering::Relaxed);
				thread::sleep(Duration::from_millis(30));
				in_flight.fetch_sub(1, Ordering::Relaxed);
				Ok(format!("text of page {page}"))
			},
			|_, _| {},
		)
		.unwrap();
		assert!(most_at_once.load(Ordering::Relaxed) >= 2, "pages were read one at a time, so the pool is not pooling");
	}

	#[test]
	fn a_page_that_will_not_read_does_not_stop_the_rest() {
		let path = comic("one-bad-page", 5);
		let cancel = AtomicBool::new(false);
		let mut seen = Vec::new();
		let outcome = recognize_pages(
			&path,
			None,
			&[1, 2, 3, 4, 5],
			3,
			&cancel,
			|_renderer, page| {
				if page == 3 { Err("the engine gave up".to_string()) } else { Ok(format!("text of page {page}")) }
			},
			|page, result| seen.push((page, result)),
		)
		.unwrap();
		assert!(!outcome.stopped);
		assert_eq!(seen.len(), 5);
		assert_eq!(seen[2], (3, Err("the engine gave up".to_string())));
		assert!(seen.iter().enumerate().filter(|(index, _)| *index != 2).all(|(_, (_, result))| result.is_ok()));
	}

	/// Renders for real, because a page the document does not have is a rendering failure.
	#[test]
	fn a_page_the_document_does_not_have_is_reported_not_skipped() {
		let path = comic("short-document", 2);
		let cancel = AtomicBool::new(false);
		let mut seen = Vec::new();
		let outcome = recognize_pages(
			&path,
			None,
			&[1, 2, 3],
			2,
			&cancel,
			|renderer, page| -> Read {
				renderer
					.render(page - 1, 4000)
					.map(|_| format!("text of page {page}"))
					.map_err(|error| error.to_string())
			},
			|page, result| seen.push((page, result)),
		)
		.unwrap();
		assert!(!outcome.stopped);
		assert_eq!(seen.len(), 3);
		assert!(seen[0].1.is_ok(), "page 1 of a two-page comic should have read");
		assert!(seen[2].1.is_err(), "page 3 of a two-page comic should have failed");
	}

	#[test]
	fn pages_are_rendered_and_read_by_the_workers_themselves() {
		let path = comic("rendered", 5);
		let cancel = AtomicBool::new(false);
		let mut seen = Vec::new();
		let outcome = recognize_pages(
			&path,
			None,
			&[1, 2, 3, 4, 5],
			3,
			&cancel,
			|renderer, page| -> Read {
				let rendered = renderer.render(page - 1, 4000).map_err(|error| error.to_string())?;
				Ok(format!("page {page} is {}x{}", rendered.width, rendered.height))
			},
			|page, result| seen.push((page, result)),
		)
		.unwrap();
		assert!(!outcome.stopped);
		assert_eq!(seen.len(), 5);
		assert!(seen.iter().all(|(_, result)| result.is_ok()), "{seen:?}");
		assert_eq!(seen[2].1.as_deref(), Ok("page 3 is 20x10"));
	}

	#[test]
	fn a_stopped_job_reads_no_more_and_hands_over_what_it_read() {
		let path = comic("stopped", 16);
		let pages: Vec<i32> = (1..=16).collect();
		let cancel = AtomicBool::new(false);
		let mut seen = Vec::new();
		let outcome = recognize_pages(
			&path,
			None,
			&pages,
			4,
			&cancel,
			|_renderer, page| -> Read {
				thread::sleep(Duration::from_millis(15));
				Ok(format!("text of page {page}"))
			},
			|page, result| {
				seen.push((page, result));
				if seen.len() == 2 {
					cancel.store(true, Ordering::Relaxed);
				}
			},
		)
		.unwrap();
		assert!(outcome.stopped, "a job stopped after two pages should say so");
		assert!(seen.len() < 16, "the job read the whole range after being stopped: {} pages", seen.len());
		// Whatever it did read arrives in order and without duplicates.
		let order: Vec<i32> = seen.iter().map(|(page, _)| *page).collect();
		let mut sorted = order.clone();
		sorted.sort_unstable();
		sorted.dedup();
		assert_eq!(order, sorted);
		assert!(seen.iter().all(|(_, result)| result.is_ok()));
	}

	#[test]
	fn a_job_stopped_before_it_starts_reads_nothing() {
		let path = comic("stopped-first", 4);
		let cancel = AtomicBool::new(true);
		let mut seen = Vec::new();
		let outcome = recognize_pages(&path, None, &[1, 2, 3, 4], 2, &cancel, says_which_page, |page, result| {
			seen.push((page, result));
		})
		.unwrap();
		assert!(outcome.stopped);
		assert!(seen.is_empty());
	}

	#[test]
	fn no_pages_to_read_is_not_an_error() {
		let cancel = AtomicBool::new(false);
		let mut seen = Vec::new();
		let outcome = recognize_pages(
			"a-path-that-does-not-exist.pdf",
			None,
			&[],
			4,
			&cancel,
			says_which_page,
			|page, result| seen.push((page, result)),
		)
		.unwrap();
		assert_eq!(outcome, Run { stopped: false });
		assert!(seen.is_empty());
	}

	#[test]
	fn a_document_that_will_not_open_is_an_error() {
		let cancel = AtomicBool::new(false);
		let mut seen = Vec::new();
		let outcome = recognize_pages(
			"a-path-that-does-not-exist.pdf",
			None,
			&[1, 2, 3],
			4,
			&cancel,
			says_which_page,
			|page, result| seen.push((page, result)),
		);
		assert!(outcome.is_err(), "opening a missing file should have failed the run");
		assert!(seen.is_empty(), "a job that never opened the document read pages anyway");
	}

	/// The single-page job the app runs from Enter asks for one page with a batch's worker count.
	#[test]
	fn more_workers_than_pages_still_reads_them_all() {
		let path = comic("one-page", 1);
		let (outcome, seen) = run(&path, &[1], 4);
		assert!(!outcome.stopped);
		assert_eq!(seen.len(), 1);
	}

	#[test]
	fn the_worker_count_is_bounded_and_never_zero() {
		let workers = default_workers();
		assert!((1..=MAX_WORKERS).contains(&workers), "default_workers gave {workers}");
	}
}
