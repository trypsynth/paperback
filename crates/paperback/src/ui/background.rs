use std::{cell::Cell, rc::Rc, sync::Mutex};

use wxdragon::{prelude::*, timer::Timer};

use super::{document_manager::DocumentManager, sleep_timer};

const STATUS_INTERVAL_MS: i32 = 1000;
const AUDIO_SYNC_INTERVAL_MS: i32 = 250;
const WINDOW_EXTEND_INTERVAL_MS: i32 = 250;

/// Hold the returned timers for the window's lifetime: `Timer`'s `Drop` destroys the wx timer.
pub fn start_timers(frame: &Frame, doc_manager: &Rc<Mutex<DocumentManager>>) -> Vec<Rc<Timer<Frame>>> {
	vec![
		status_timer(frame, doc_manager),
		pump_timer(frame, doc_manager, AUDIO_SYNC_INTERVAL_MS, DocumentManager::pump_audio),
		pump_timer(frame, doc_manager, WINDOW_EXTEND_INTERVAL_MS, DocumentManager::pump_window_extend),
	]
}

fn status_timer(frame: &Frame, doc_manager: &Rc<Mutex<DocumentManager>>) -> Rc<Timer<Frame>> {
	let timer = Rc::new(Timer::new(frame));
	let dm = Rc::clone(doc_manager);
	timer.on_tick(move |_| {
		if !sleep_timer::is_running() {
			return;
		}
		let Ok(dm) = dm.try_lock() else {
			return;
		};
		dm.update_status_bar();
	});
	timer.start(STATUS_INTERVAL_MS, false);
	timer
}

/// Skips a tick it cannot lock for, since the lock is held by whatever the user is doing and waiting would stall the UI thread.
fn pump_timer(
	frame: &Frame,
	doc_manager: &Rc<Mutex<DocumentManager>>,
	interval_ms: i32,
	pump: fn(&mut DocumentManager),
) -> Rc<Timer<Frame>> {
	let timer = Rc::new(Timer::new(frame));
	let dm = Rc::clone(doc_manager);
	timer.on_tick(move |_| {
		if let Ok(mut dm) = dm.try_lock() {
			pump(&mut dm);
		}
	});
	timer.start(interval_ms, false);
	timer
}

/// A resize makes `RichEdit` rewrap everything loaded, so a long read's growth is given back first.
pub fn bind_resize(frame: &Frame, doc_manager: &Rc<Mutex<DocumentManager>>) {
	let dm = Rc::clone(doc_manager);
	let last_frame_size = Rc::new(Cell::new((0, 0)));
	let frame_for_resize = *frame;
	frame.on_size(move |event| {
		event.skip(true);
		let size = frame_for_resize.get_size();
		let dimensions = (size.width, size.height);
		// Layout work sends size events mid Say-All, and compaction moves the loaded start out from under the screen reader, so only a real shape change compacts.
		if last_frame_size.replace(dimensions) == dimensions {
			return;
		}
		if let Ok(mut dm) = dm.try_lock() {
			dm.compact_window_if_grown();
		}
	});
}
