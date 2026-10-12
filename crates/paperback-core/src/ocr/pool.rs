//! Reading a document's pages on several threads at once.
//!
//! The two halves of reading a page cost very different things, and that is what shapes this.
//! Measured on a scanned 20-page PDF rendered at 300 DPI: about 55 ms to rasterize a page and
//! 135 ms to read the text off it. Recognizing scales close to linearly across threads, but
//! rasterizing barely scales at all, because `pdfium-render`'s `thread_safe` feature serializes
//! every call through one process-wide lock and `FPDF_RenderPageBitmap` is a single call. Eight
//! threads made rasterizing only 1.45x faster while the same eight made recognizing 6.4x faster.
//!
//! So a page is claimed, rasterized and read by the same worker, and each worker opens a renderer
//! of its own. The serialized half is worth having several threads in only because the other half
//! runs alongside it; pacing the two against each other is what [`MAX_WORKERS`] is about.
//!
//! Order is kept on the way out. Pages finish in whatever order they happen to take, but the
//! reader watching the document fill in, and the caller's progress announcements, should see
//! pages land in the order a single-threaded pass would have put them in.

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

/// The most workers a job will use, however many cores the machine has.
///
/// Past this, pages are being claimed faster than pdfium's one-at-a-time rasterizer can serve
/// them, so the extra threads buy memory and lock contention rather than throughput. Looking at
/// it the other way: a worker spends about 55 ms rasterizing and 135 ms reading, so three or four
/// of them keep the rasterizer busy without queueing behind it.
const MAX_WORKERS: usize = 4;

/// How many workers a job uses when the caller has no opinion.
///
/// One core is left for the thread driving the run, which in the app is the one that has to stay
/// responsive while the pages land.
#[must_use]
pub fn default_workers() -> usize {
	thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get).saturating_sub(1).clamp(1, MAX_WORKERS)
}

/// What a run did.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Run {
	/// True when the job stopped at a page boundary because `cancel` was raised, rather than
	/// reaching the end of the page list.
	pub stopped: bool,
}

/// Renders and reads `pages` (1-based page numbers) on up to `workers` threads, handing each
/// page's result to `on_page` in ascending page order.
///
/// `recognize_page` is the platform's OCR engine, which lives in the front end rather than here,
/// so it arrives as a call made with the renderer this worker opened and the page number to read.
/// It has to be callable from several threads at once.
///
/// `on_page` runs on the calling thread, one page at a time, in page order -- the order a
/// single-threaded pass would have produced, so a caller's progress announcements and window
/// refreshes see what they have always seen. Recognitions finishing out of order are absorbed
/// here rather than passed on. Every page in `pages` reaches `on_page` exactly once, a page that
/// failed to read included: an `Err` for one page does not stop the rest.
///
/// Raising `cancel` stops the job at each worker's next page boundary. Pages already read are
/// still handed to `on_page`, which is what lets the app say how much was read before the stop.
///
/// # Errors
///
/// Returns an error if the document cannot be opened for rendering, which every worker would fail
/// at and which no page can be read without. Each worker opens the document itself, since a
/// renderer belongs to the thread that made it, so this is the price of the arrangement: the
/// document is parsed once per worker.
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
	// Raised by a worker that could not open the document. Every other worker would find the same
	// thing, so this is what sends them all home rather than each reporting it separately.
	let stop = AtomicBool::new(false);
	let open_failure: Mutex<Option<anyhow::Error>> = Mutex::new(None);
	let (sender, receiver) = mpsc::channel::<(usize, Result<String, E>)>();

	let emitted = thread::scope(|scope| {
		// Borrowed once so every worker closure captures the reference rather than moving the
		// call itself into the first one.
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
				// The claim comes before the check, so a worker that finds the list exhausted
				// leaves and the ones still working are not asked to stop.
				while !stop.load(Ordering::Relaxed) && !cancel.load(Ordering::Relaxed) {
					let index = cursor.fetch_add(1, Ordering::Relaxed);
					let Some(&page) = pages.get(index) else { break };
					// A send can only fail once the caller has stopped collecting, which is what
					// the end of the run looks like from in here, so there is nothing to do but
					// leave.
					if sender.send((index, recognize_page(&mut renderer, page))).is_err() {
						break;
					}
				}
			});
		}
		// The caller's own sender, dropped so the channel closes once the workers are done.
		drop(sender);
		collect(receiver, pages, &mut on_page)
	});

	if let Ok(Some(error)) = open_failure.into_inner() {
		return Err(error);
	}
	Ok(Run { stopped: emitted < pages.len() })
}

/// Hands the workers' results to `on_page` in page order, holding back any that arrive before an
/// earlier page has been read. Returns how many pages were handed over.
///
/// A short count means the job stopped: every page is claimed and sent unless a stop cut the
/// claiming short, so pages are missing from the count only when they were never read.
fn collect<E>(
	receiver: mpsc::Receiver<(usize, Result<String, E>)>,
	pages: &[i32],
	on_page: &mut impl FnMut(i32, Result<String, E>),
) -> usize {
	// Pages read out of order wait here for the ones before them. What is held is a page of text,
	// not the bitmap it came from, so a whole document waiting costs kilobytes.
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
	// A stop leaves the pages after the gap unread and the ones already read behind it. Those are
	// still worth handing over: the text was read, and the app reports how much it got.
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

	/// A greyscale PNG of `width` by `height`, all white. Small, but a real decodable image.
	fn png(width: u32, height: u32) -> Vec<u8> {
		let mut buffer = Vec::new();
		let encoder = image::codecs::png::PngEncoder::new(&mut buffer);
		let pixels = vec![255u8; (width * height) as usize];
		image::ImageEncoder::write_image(encoder, &pixels, width, height, image::ExtendedColorType::L8).unwrap();
		buffer
	}

	/// A comic archive of `pages` renderable pages, which is the cheapest real document to hand
	/// the pool: no OCR engine is involved, only pdfium's comic path and the image decoder.
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

	/// A recognized page as this module's stand-in engine reports it.
	type Read = Result<String, String>;

	/// A recognizer that reads the page number off the page it was handed and says so.
	fn says_which_page(_renderer: &mut PageRenderer, page: i32) -> Read {
		Ok(format!("text of page {page}"))
	}

	/// Collects the pages in the order `on_page` was called with them, alongside their text.
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

	/// The reason recognition order is not handed straight on: a reader watching the document
	/// fill in should see pages land in the order they are numbered, however the workers happen
	/// to finish. The sleeps make the earliest page the slowest, so a pass that passed results
	/// through as they arrived would come back backwards.
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

	/// The point of the pool. Without this the workers could all be running one after another and
	/// every other test here would still pass.
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

	/// A page the engine cannot read is that page's problem. The rest of the document still gets
	/// read, and the caller hears about this one rather than about a job that failed.
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

	/// A page out of the range the document has is reported as that page's failure rather than
	/// panicking the worker or being quietly dropped from the count. This one renders for real,
	/// because a page the document does not have is a rendering failure and nothing else.
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

	/// One pass over real pages of a real archive, rendered and read by the workers rather than
	/// by a stand-in, so the pool is exercised against the rendering it actually drives.
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

	/// Stopping is a page boundary, and what was already read is still handed over: the app
	/// reports how much was recognized before the stop, so dropping it would make that count
	/// describe text nobody got.
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

	/// A cancel raised before anything is claimed reads nothing and still reports a stop rather
	/// than a completed run of no pages.
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

	/// Nothing to read is not a failure and does not open the document, so a range that turned
	/// out empty reports a completed run of no pages.
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

	/// A document no worker can open ends the job as an error, the same one every worker would
	/// have found, rather than as a run that read nothing.
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

	/// More workers than pages is ordinary -- the single-page job the app runs from Enter asks
	/// for one page and passes whatever the machine would use for a batch.
	#[test]
	fn more_workers_than_pages_still_reads_them_all() {
		let path = comic("one-page", 1);
		let (outcome, seen) = run(&path, &[1], 4);
		assert!(!outcome.stopped);
		assert_eq!(seen.len(), 1);
	}

	/// The pacing rule, stated as a test because it is the whole reason the pool is not simply
	/// "as many threads as the machine has".
	#[test]
	fn the_worker_count_is_bounded_and_never_zero() {
		let workers = default_workers();
		assert!((1..=MAX_WORKERS).contains(&workers), "default_workers gave {workers}");
	}
}
