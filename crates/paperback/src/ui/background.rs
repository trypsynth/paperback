//! The window's periodic work: the timers that tick while the app runs, and the resize
//! handling that goes with them.
//!
//! These lived in `bind_menu_events` because that is where the frame was to hand, not because
//! they have anything to do with menus. Nothing here is triggered by the user choosing
//! something; it all runs on its own.

use std::{cell::Cell, rc::Rc, sync::Mutex};

use wxdragon::{prelude::*, timer::Timer};

use super::{document_manager::DocumentManager, sleep_timer, status};

/// How often the status bar refreshes its sleep-timer countdown.
const STATUS_INTERVAL_MS: i32 = 1000;
/// How often audio playback position is pumped.
const AUDIO_SYNC_INTERVAL_MS: i32 = 250;
/// How often the loaded text window is extended ahead of the caret.
const WINDOW_EXTEND_INTERVAL_MS: i32 = 250;

/// Starts the window's periodic timers and returns them.
///
/// The caller must hold the returned timers for as long as the window lives: `Timer`'s `Drop`
/// destroys the underlying wx timer.
pub fn start_timers(frame: &Frame, doc_manager: &Rc<Mutex<DocumentManager>>) -> Vec<Rc<Timer<Frame>>> {
	vec![
		status_timer(frame, doc_manager),
		pump_timer(frame, doc_manager, AUDIO_SYNC_INTERVAL_MS, DocumentManager::pump_audio),
		pump_timer(frame, doc_manager, WINDOW_EXTEND_INTERVAL_MS, DocumentManager::pump_window_extend),
	]
}

/// Refreshes the status bar while a sleep timer counts down.
fn status_timer(frame: &Frame, doc_manager: &Rc<Mutex<DocumentManager>>) -> Rc<Timer<Frame>> {
	let timer = Rc::new(Timer::new(frame));
	let dm = Rc::clone(doc_manager);
	let frame_for_tick = *frame;
	timer.on_tick(move |_| {
		if !sleep_timer::is_running() {
			return;
		}
		let Ok(dm) = dm.try_lock() else {
			return;
		};
		status::update_status_bar_with_sleep_timer(
			&frame_for_tick,
			&dm,
			sleep_timer::start_ms(),
			sleep_timer::duration_minutes(),
		);
	});
	timer.start(STATUS_INTERVAL_MS, false);
	timer
}

/// A timer that calls one method on the [`DocumentManager`], skipping a tick it cannot lock for.
///
/// Both pumps run on a tick that must never block: the lock they want is held by whatever the
/// user is doing, and waiting for it would stall the UI thread.
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

/// Compacts the loaded text window when the frame actually changes shape.
///
/// A resize forces `RichEdit` to rewrap, whose cost scales with how much is loaded ahead of the
/// caret. Give back any growth a long read accumulated before paying for that.
///
/// Only when the frame actually changed shape. Compaction moves the loaded start, which
/// silently invalidates the offsets a screen reader is reading from, so a size event that
/// resizes nothing must not trigger it: those arrive from ordinary layout work, including
/// while a Say-All is running, where the user has done nothing that would stop the read.
pub fn bind_resize(frame: &Frame, doc_manager: &Rc<Mutex<DocumentManager>>) {
	let dm = Rc::clone(doc_manager);
	let last_frame_size = Rc::new(Cell::new((0, 0)));
	let frame_for_resize = *frame;
	frame.on_size(move |event| {
		event.skip(true);
		let size = frame_for_resize.get_size();
		let dimensions = (size.width, size.height);
		if last_frame_size.replace(dimensions) == dimensions {
			return;
		}
		if let Ok(mut dm) = dm.try_lock() {
			dm.compact_window_if_grown();
		}
	});
}
