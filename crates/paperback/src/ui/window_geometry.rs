//! Defaults and minimum are device-independent pixels so they mean the same at any scaling; the saved rect is physical pixels stored verbatim, since converting it would move the window off where the user put it.

use paperback_core::config::ConfigManager;
use wxdragon::prelude::*;

const DEFAULT_WIDTH: i32 = 800;
const DEFAULT_HEIGHT: i32 = 600;

/// Deliberately low: only stops a restored rect or stray drag collapsing the window to a title bar.
const MIN_WIDTH: i32 = 400;
const MIN_HEIGHT: i32 = 300;

const KEY_X: &str = "window_x";
const KEY_Y: &str = "window_y";
const KEY_WIDTH: &str = "window_width";
const KEY_HEIGHT: &str = "window_height";
const KEY_MAXIMIZED: &str = "window_maximized";

/// Not 0 or -1, since a window on a monitor left of or above the primary has negative coordinates.
const UNSET: i32 = i32::MIN;

/// Physical pixels, small enough to sample inside the title bar rather than measure it.
const TITLE_BAR_DEPTH: i32 = 8;
const TITLE_BAR_INSET: i32 = 24;

pub fn apply_defaults(frame: &Frame) {
	// Sized after building so `from_dip` scales for the display the frame lands on; it isn't shown yet, so there's no visible resize.
	frame.set_min_size(frame.from_dip(Size::new(MIN_WIDTH, MIN_HEIGHT)));
	frame.set_size(frame.from_dip(Size::new(DEFAULT_WIDTH, DEFAULT_HEIGHT)));
}

/// Call while the frame is still hidden, so the move isn't a visible jump.
pub fn restore(frame: &Frame, config: &ConfigManager) {
	match saved_rect(config) {
		// Used as-is even partly off the left or top edge, which Windows allows and is where the user left it.
		Some(rect) if is_reachable(rect) => frame.set_size_with_pos(rect.x, rect.y, rect.width, rect.height),
		// Its monitor is gone (unplugged or undocked): the size was a deliberate choice, so only the position is dropped.
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

pub fn save(frame: &Frame, config: &ConfigManager) {
	let maximized = frame.is_maximized();
	config.set_app_bool(KEY_MAXIMIZED, maximized);
	// A maximized rect is the whole display and a minimized one is parked off-screen; saving either would lose the size to un-maximize to.
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

/// Three points along the title bar, since a window nudged off one side is still draggable.
fn is_reachable(rect: Rect) -> bool {
	let y = rect.y + TITLE_BAR_DEPTH;
	[rect.x + TITLE_BAR_INSET, rect.x + rect.width / 2, rect.x + rect.width - TITLE_BAR_INSET]
		.into_iter()
		.any(|x| Display::from_point(Point::new(x, y)).is_some())
}

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

	/// Backed by a throwaway file, since the accessors are no-ops until initialized.
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
		assert!(saved_rect(&config).is_none());
	}

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
