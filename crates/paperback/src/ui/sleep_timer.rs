//! The sleep timer: shuts the app down after a chosen number of minutes, saving reading
//! positions on the way out.
//!
//! The countdown lives in atomics rather than on the [`SleepTimer`] itself: the status bar and
//! `document_manager` both display it, and neither holds the timer.

use std::{
	rc::Rc,
	sync::{
		Mutex,
		atomic::{AtomicBool, AtomicI32, AtomicI64, Ordering},
	},
	time::{SystemTime, UNIX_EPOCH},
};

use paperback_core::config::ConfigManager;
use patois::{nt, t};
use wxdragon::prelude::*;

use super::{dialogs, document_manager::DocumentManager, main_window::update_title_from_manager, navigation};

/// When the running timer was set, as milliseconds since the epoch, or 0 when none is running.
static START_MS: AtomicI64 = AtomicI64::new(0);
/// How long the running timer was set for, in minutes, or 0 when none is running.
static DURATION_MINUTES: AtomicI32 = AtomicI32::new(0);
/// Whether a timer is currently counting down.
///
/// Kept separately rather than inferred from `START_MS` being non-zero: a clock that cannot be
/// read falls back to 0, and a timer that is running should not look cancelled because of it.
static RUNNING: AtomicBool = AtomicBool::new(false);

/// Whether a sleep timer is counting down.
pub fn is_running() -> bool {
	RUNNING.load(Ordering::SeqCst)
}

/// When the running timer started, in milliseconds since the epoch. 0 when none is running.
pub fn start_ms() -> i64 {
	START_MS.load(Ordering::SeqCst)
}

/// How many minutes the running timer was set for. 0 when none is running.
pub fn duration_minutes() -> i32 {
	DURATION_MINUTES.load(Ordering::SeqCst)
}

fn now_ms() -> i64 {
	SystemTime::now().duration_since(UNIX_EPOCH).ok().and_then(|d| i64::try_from(d.as_millis()).ok()).unwrap_or(0)
}

/// Owns the wx timer. Hold one for the window's lifetime: dropping it destroys the timer.
pub struct SleepTimer {
	timer: Rc<Timer<Frame>>,
}

impl SleepTimer {
	/// Creates the timer and installs the tick that closes the app when the deadline passes.
	pub fn new(frame: &Frame, doc_manager: &Rc<Mutex<DocumentManager>>, config: &Rc<Mutex<ConfigManager>>) -> Self {
		let timer = Rc::new(Timer::new(frame));
		let timer_for_tick = Rc::clone(&timer);
		let frame_for_tick = *frame;
		let dm_for_tick = Rc::clone(doc_manager);
		let config_for_tick = Rc::clone(config);
		timer.on_tick(move |_| {
			// `Timer::on_tick` binds `EventType::TIMER` on the *owner*, not on the timer, and
			// wxdragon gives its timers no distinguishing id, so every timer parented to this
			// frame delivers its ticks to every handler bound here. This one shuts the app down,
			// so it has to confirm the deadline really passed rather than trust that being
			// called means its own timer fired.
			if !is_running() {
				return;
			}
			let deadline_ms = start_ms().saturating_add(i64::from(duration_minutes()) * 60_000);
			if now_ms() < deadline_ms {
				return;
			}
			tracing::info!("sleep timer fired, closing application");
			timer_for_tick.stop();
			clear();
			{
				let dm = dm_for_tick.lock().unwrap();
				let cfg = config_for_tick.lock().unwrap();
				for i in 0..dm.tab_count() {
					if let Some(tab) = dm.get_tab(i) {
						let current_pos = navigation::doc_caret(tab);
						let path_str = tab.file_path.to_string_lossy();
						cfg.set_document_position(&path_str, current_pos);
					}
				}
				cfg.flush();
			}
			frame_for_tick.close(true);
		});
		Self { timer }
	}

	/// The wx timer, to be held alongside the window's other timers.
	pub const fn timer(&self) -> &Rc<Timer<Frame>> {
		&self.timer
	}

	/// Cancels a running timer, or asks for a duration and starts one.
	pub fn toggle(
		&self,
		frame: &Frame,
		dm: &Rc<Mutex<DocumentManager>>,
		config: &Rc<Mutex<ConfigManager>>,
		live_region_label: StaticText,
	) {
		if is_running() {
			self.timer.stop();
			clear();
			tracing::info!("sleep timer cancelled");
			let dm_ref = dm.lock().unwrap();
			update_title_from_manager(frame, &dm_ref);
			// TRANSLATORS: Announced when the user cancels a running sleep timer
			live_region::announce(live_region_label, &t("Sleep timer cancelled."));
			return;
		}
		let initial_duration = config.lock().unwrap().get_app_int("sleep_timer_duration", 30);
		if let Some(duration) = dialogs::show_sleep_timer_dialog(frame, initial_duration) {
			{
				let cfg = config.lock().unwrap();
				cfg.set_app_int("sleep_timer_duration", duration);
				cfg.flush();
			}
			let duration_ms = u64::try_from(duration).unwrap_or(0) * 60 * 1000;
			self.timer.start(i32::try_from(duration_ms).unwrap_or(i32::MAX), true);
			tracing::info!(duration_minutes = duration, "sleep timer started");
			START_MS.store(now_ms(), Ordering::SeqCst);
			DURATION_MINUTES.store(duration, Ordering::SeqCst);
			RUNNING.store(true, Ordering::SeqCst);
			// TRANSLATORS: Announcement when the sleep timer is set. The %d placeholder is replaced with the number of minutes.
			let msg = nt(
				"Sleep timer set for %d minute.",
				"Sleep timer set for %d minutes.",
				u64::try_from(duration).unwrap_or(0),
			)
			.replacen("%d", &duration.to_string(), 1);
			live_region::announce(live_region_label, &msg);
		}
	}
}

fn clear() {
	RUNNING.store(false, Ordering::SeqCst);
	START_MS.store(0, Ordering::SeqCst);
	DURATION_MINUTES.store(0, Ordering::SeqCst);
}

#[cfg(test)]
mod tests {
	use super::*;

	/// The status bar reads these to show the countdown, so a cancelled timer that left a
	/// stale start time behind would keep displaying one that is no longer running.
	#[test]
	fn clear_resets_every_field() {
		START_MS.store(1234, Ordering::SeqCst);
		DURATION_MINUTES.store(30, Ordering::SeqCst);
		RUNNING.store(true, Ordering::SeqCst);
		clear();
		assert!(!is_running());
		assert_eq!(start_ms(), 0);
		assert_eq!(duration_minutes(), 0);
	}
}
