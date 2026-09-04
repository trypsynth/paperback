//! Persistence of the main window's placement between runs.
//!
//! Reopening where you were left is standard behaviour for a Windows desktop app, and it is the
//! only part of the window's appearance the user can set without a settings dialog: whatever
//! size and corner of which monitor they dragged the window to is a preference they expressed
//! by hand, so throwing it away on exit and re-centring an 800x600 box is a visible regression
//! every single launch.
//!
//! Sizes here are in two different units and the distinction matters. The defaults and the
//! minimum are written in device-independent pixels and passed through [`dpi`], because they are
//! measurements chosen by us that have to mean the same thing at any scaling factor. The saved
//! rect is in physical screen pixels, stored and restored verbatim, because it is not a
//! measurement at all - it is the exact placement wx last reported, and converting it would move
//! the window off the spot the user put it on.

use paperback_core::config::ConfigManager;
use wx_utils::dpi;
use wxdragon::prelude::*;

/// The size a first run opens at, in device-independent pixels.
const DEFAULT_WIDTH: i32 = 800;
const DEFAULT_HEIGHT: i32 = 600;

/// The smallest the window can be dragged to, in device-independent pixels. Deliberately low:
/// the point is only to stop a restored rect - or a stray drag - from collapsing the window to
/// a title bar with the menu, tab strip and status bar stacked on top of an empty text area.
const MIN_WIDTH: i32 = 400;
const MIN_HEIGHT: i32 = 300;

const KEY_X: &str = "window_x";
const KEY_Y: &str = "window_y";
const KEY_WIDTH: &str = "window_width";
const KEY_HEIGHT: &str = "window_height";
const KEY_MAXIMIZED: &str = "window_maximized";

/// Stand-in for "no position saved yet". A real window can legitimately sit at a negative x or y
/// (a monitor left of the primary one), so 0 and -1 are both usable coordinates and can't serve
/// as the sentinel.
const UNSET: i32 = i32::MIN;

/// How far down the saved rect to probe for the title bar when checking the window is still
/// reachable, and how far in from each end. Both in physical pixels, and both deliberately
/// small: the point is to sample somewhere inside the drag handle, not to measure it.
const TITLE_BAR_DEPTH: i32 = 8;
const TITLE_BAR_INSET: i32 = 24;

/// Gives the freshly built frame a sane size and a floor to resize against, before anything has
/// been read from the config. [`restore`] replaces the size a moment later if there is one
/// saved; this is what a first run gets, and what the window is measured as in the meantime.
pub fn apply_defaults(frame: &Frame) {
	// Sized after building rather than through the builder: the size has to be scaled for the
	// display the window actually lands on, and there is nothing to ask about that until the
	// frame exists. It isn't shown until later, so there's no visible resize.
	frame.set_min_size(dpi::scale_size(frame, Size::new(MIN_WIDTH, MIN_HEIGHT)));
	frame.set_size(dpi::scale_size(frame, Size::new(DEFAULT_WIDTH, DEFAULT_HEIGHT)));
}

/// Places the window where it was left. Call this while the frame is still hidden, so the move
/// happens before anything is on screen rather than as a visible jump after it.
pub fn restore(frame: &Frame, config: &ConfigManager) {
	match saved_rect(config) {
		// The saved spot is still on a monitor, so it is used as-is - including a position
		// partly off the left or top edge, which Windows itself allows and which is where the
		// user left the window.
		Some(rect) if is_reachable(rect) => frame.set_size_with_pos(rect.x, rect.y, rect.width, rect.height),
		// The monitor it was on is gone (unplugged, or a laptop undocked). The size is still a
		// deliberate choice worth keeping, so only the position is thrown away.
		Some(rect) => {
			frame.set_size(fit_to_primary(Size::new(rect.width, rect.height)));
			frame.centre();
		}
		None => frame.centre(),
	}
	if config.get_app_bool(KEY_MAXIMIZED, false) || config.get_app_bool("start_maximized", false) {
		frame.maximize(true);
	}
}

/// Records where the window is, for the next run to restore.
pub fn save(frame: &Frame, config: &ConfigManager) {
	let maximized = frame.is_maximized();
	config.set_app_bool(KEY_MAXIMIZED, maximized);
	// A maximized window's own rect is the whole display, and a minimized one's is off in the
	// far corner Windows parks minimized windows at. Writing either over the stored rect would
	// destroy the size the user actually chose, leaving the next un-maximize with nothing
	// sensible to go back to, so in both cases the last normal rect is left alone.
	if maximized || frame.is_iconized() {
		return;
	}
	let position = frame.get_position();
	let size = frame.get_size();
	if size.width <= 0 || size.height <= 0 {
		return;
	}
	config.set_app_int(KEY_X, position.x);
	config.set_app_int(KEY_Y, position.y);
	config.set_app_int(KEY_WIDTH, size.width);
	config.set_app_int(KEY_HEIGHT, size.height);
}

/// The stored rect, or `None` when there isn't a complete and plausible one.
fn saved_rect(config: &ConfigManager) -> Option<Rect> {
	let x = config.get_app_int(KEY_X, UNSET);
	let y = config.get_app_int(KEY_Y, UNSET);
	let width = config.get_app_int(KEY_WIDTH, 0);
	let height = config.get_app_int(KEY_HEIGHT, 0);
	if x == UNSET || y == UNSET || width <= 0 || height <= 0 {
		return None;
	}
	Some(Rect::new(x, y, width, height))
}

/// Whether enough of the window's title bar lands on a connected display to drag it by.
///
/// Three points along the bar rather than one: a window nudged off the left edge, or off the
/// right, is still perfectly usable, and only a rect whose entire drag handle has gone missing
/// with its monitor is worth discarding.
fn is_reachable(rect: Rect) -> bool {
	let y = rect.y + TITLE_BAR_DEPTH;
	[rect.x + TITLE_BAR_INSET, rect.x + rect.width / 2, rect.x + rect.width - TITLE_BAR_INSET]
		.into_iter()
		.any(|x| Display::from_point(Point::new(x, y)).is_some())
}

/// Shrinks a size to what the primary display can actually show, for the case where the saved
/// one came from a larger monitor that is no longer attached.
fn fit_to_primary(size: Size) -> Size {
	let Some(area) = Display::new(0).map(|display| display.client_area()) else {
		return size;
	};
	Size::new(size.width.min(area.width), size.height.min(area.height))
}

#[cfg(test)]
mod tests {
	use std::{env, fs, process};

	use super::*;

	/// A `ConfigManager` backed by a throwaway file, since the accessors are all no-ops until
	/// one has been initialized.
	fn scratch_config(name: &str) -> ConfigManager {
		let path = env::temp_dir().join(format!("paperback-geometry-{}-{name}.toml", process::id()));
		let _ = fs::remove_file(&path);
		let mut config = ConfigManager::new();
		config.initialize(path);
		config
	}

	#[test]
	fn a_config_with_nothing_saved_has_no_rect() {
		assert!(saved_rect(&scratch_config("empty")).is_none());
	}

	#[test]
	fn a_half_written_rect_is_rejected_rather_than_half_used() {
		let config = scratch_config("partial");
		config.set_app_int(KEY_X, 100);
		config.set_app_int(KEY_Y, 100);
		config.set_app_int(KEY_WIDTH, 900);
		// Height never made it, so there is no rect to restore.
		assert!(saved_rect(&config).is_none());
	}

	// The reason the sentinel is i32::MIN and not 0 or -1: a window on a monitor to the left of
	// the primary one legitimately sits at a negative x, and one dragged up under the top edge
	// at a negative y. Treating either as "unset" would re-centre those windows every launch.
	#[test]
	fn negative_coordinates_are_a_real_position_not_an_absent_one() {
		let config = scratch_config("negative");
		config.set_app_int(KEY_X, -1920);
		config.set_app_int(KEY_Y, -8);
		config.set_app_int(KEY_WIDTH, 900);
		config.set_app_int(KEY_HEIGHT, 700);
		assert_eq!(saved_rect(&config), Some(Rect::new(-1920, -8, 900, 700)));
	}

	#[test]
	fn a_collapsed_size_is_rejected() {
		let config = scratch_config("collapsed");
		config.set_app_int(KEY_X, 10);
		config.set_app_int(KEY_Y, 10);
		config.set_app_int(KEY_WIDTH, 0);
		config.set_app_int(KEY_HEIGHT, 0);
		assert!(saved_rect(&config).is_none());
	}
}
