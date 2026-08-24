//! Converts the sizes written throughout the UI from device-independent pixels into the
//! physical pixels wx actually wants.
//!
//! `build.rs` declares this app `DpiAwareness::PerMonitorV2`, which tells Windows the app will
//! handle scaling itself: nothing gets stretched for us, and a window asked for 800x600 is
//! 800x600 *physical* pixels. On a 150% display that is 533x400 as far as the user is
//! concerned, which is how the whole app ended up opening two thirds of its intended size.
//!
//! So every explicit size in the UI is written as what it should measure at 100% scaling, and
//! passed through here on the way to wx. This is what `wxWindow::FromDIP` does; wxdragon
//! doesn't expose it, so this is the same conversion built on `wxDisplay::GetPPI`, which
//! reports the DPI of the display a given window is actually on rather than a single global
//! value - a dialog opened on a second monitor with different scaling gets sized for that
//! monitor.
//!
//! Windows-only by design. macOS and GTK hand wx logical coordinates and do their own scaling,
//! so converting there would apply the scale factor twice.

use wxdragon::prelude::*;

/// The DPI that device-independent pixels are defined against: a size passes through unchanged
/// on a display at 100% scaling.
const BASELINE_DPI: i32 = 96;

/// Converts `value` from device-independent pixels to physical pixels for the display
/// `reference` is on. `reference` can be any live window - typically the parent of whatever is
/// being sized, or the window itself once built.
///
/// Values that aren't real measurements are returned untouched: `wxDefaultCoord` (-1) means
/// "work it out from the content", and scaling it would turn it into a different negative
/// number that no longer means that.
#[must_use]
pub fn scale(reference: &dyn WxWidget, value: i32) -> i32 {
	if value <= 0 {
		return value;
	}
	scale_for_dpi(value, dpi_of(reference))
}

/// [`scale`] for both axes of a size.
#[must_use]
pub fn scale_size(reference: &dyn WxWidget, size: Size) -> Size {
	let dpi = dpi_of(reference);
	Size::new(
		if size.width <= 0 { size.width } else { scale_for_dpi(size.width, dpi) },
		if size.height <= 0 { size.height } else { scale_for_dpi(size.height, dpi) },
	)
}

/// The conversion itself, rounded to the nearest pixel rather than truncated, so a column of
/// several scaled sizes doesn't drift visibly short. Split out from [`scale`] so the arithmetic
/// can be tested without a display attached.
fn scale_for_dpi(value: i32, dpi: i32) -> i32 {
	if dpi == BASELINE_DPI {
		return value;
	}
	let scaled = (i64::from(value) * i64::from(dpi) + i64::from(BASELINE_DPI) / 2) / i64::from(BASELINE_DPI);
	i32::try_from(scaled).unwrap_or(i32::MAX)
}

/// The DPI of the display `reference` sits on, falling back to the primary display (for a
/// window that isn't placed yet) and then to [`BASELINE_DPI`] (for a headless or otherwise
/// unreadable display, where leaving sizes alone is the safe answer).
#[cfg(target_os = "windows")]
fn dpi_of(reference: &dyn WxWidget) -> i32 {
	Display::from_window(reference)
		.or_else(|| Display::new(0))
		.map(|display| display.ppi().width)
		.filter(|&ppi| ppi > 0)
		.unwrap_or(BASELINE_DPI)
}

#[cfg(not(target_os = "windows"))]
fn dpi_of(_reference: &dyn WxWidget) -> i32 {
	BASELINE_DPI
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn sizes_pass_through_unchanged_at_100_percent() {
		assert_eq!(scale_for_dpi(800, 96), 800);
		assert_eq!(scale_for_dpi(1, 96), 1);
	}

	#[test]
	fn sizes_scale_with_the_display() {
		assert_eq!(scale_for_dpi(800, 144), 1200); // 150%
		assert_eq!(scale_for_dpi(800, 192), 1600); // 200%
		assert_eq!(scale_for_dpi(600, 120), 750); // 125%
	}

	#[test]
	fn scaling_rounds_rather_than_truncating() {
		// 250 at 125% is 312.5; truncating would lose half a pixel on every such value.
		assert_eq!(scale_for_dpi(250, 120), 313);
		assert_eq!(scale_for_dpi(5, 144), 8); // 7.5
	}

	#[test]
	fn absurd_sizes_clamp_instead_of_wrapping_negative() {
		// No real size comes close to this, but wrapping would turn a size into a negative
		// number, which wx reads as "work it out yourself" - a silent, confusing failure.
		assert_eq!(scale_for_dpi(i32::MAX / 2, 192), i32::MAX / 2 * 2);
		assert_eq!(scale_for_dpi(i32::MAX, 192), i32::MAX);
	}
}
