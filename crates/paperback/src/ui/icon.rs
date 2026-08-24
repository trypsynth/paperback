//! The application icon, for the places that need a live `wxBitmap` at runtime: the frame's
//! title bar and Alt+Tab entry, and the notification area.
//!
//! On Windows the same artwork is also compiled into the executable as an icon resource by
//! `build.rs`. That resource is what Explorer, the taskbar, the Start menu, the uninstall
//! entry and the file associations the installer registers all read, since none of them ever
//! run our code - this module covers only what has to be handed over as a bitmap instead.

use wxdragon::prelude::*;

/// The icon artwork, shared with the Android and iOS apps (`android/store_assets`) so all
/// three platforms stay visually in step, and with the `.ico` `build.rs` embeds.
const ICON_PNG: &[u8] = include_bytes!("../../assets/paperback.png");

/// Sizes to offer the notification area. Windows picks 16 at 100% scaling and 32 at 200%, and
/// wants 20 or 24 at the scalings in between; anything else is scaled from the nearest of these.
const NOTIFICATION_AREA_SIZES: [u32; 4] = [16, 20, 24, 32];

/// The size handed to `Frame::set_icon`, which takes a single bitmap rather than a bundle. wx
/// derives both the title bar's small icon and the window's large icon from it, so this is a
/// compromise between the two rather than a match for either.
const FRAME_ICON_SIZE: u32 = 32;

/// Decodes the artwork once and rescales it to each of `sizes`, as raw RGBA. Returns an empty
/// vector if the embedded PNG can't be decoded, which would mean the build itself is broken -
/// every caller treats that as "carry on without an icon" rather than failing.
fn rgba_at(sizes: &[u32]) -> Vec<(u32, Vec<u8>)> {
	let Ok(source) = image::load_from_memory_with_format(ICON_PNG, image::ImageFormat::Png) else {
		tracing::warn!("could not decode the embedded application icon");
		return Vec::new();
	};
	sizes
		.iter()
		.map(|&size| {
			let scaled = source.resize_exact(size, size, image::imageops::FilterType::Lanczos3);
			(size, scaled.to_rgba8().into_raw())
		})
		.collect()
}

/// The artwork as wx bitmaps, one per requested size.
fn bitmaps_at(sizes: &[u32]) -> Vec<Bitmap> {
	rgba_at(sizes).into_iter().filter_map(|(size, rgba)| Bitmap::from_rgba(&rgba, size, size)).collect()
}

/// The application icon as a bundle covering the notification area's sizes, so the shell can
/// pick the right one for the display it is drawn on instead of rescaling a single bitmap.
#[must_use]
pub fn notification_area_bundle() -> Option<BitmapBundle> {
	let bitmaps = bitmaps_at(&NOTIFICATION_AREA_SIZES);
	if bitmaps.is_empty() {
		return None;
	}
	Some(BitmapBundle::from_bitmaps(&bitmaps))
}

/// The application icon at the size [`Frame::set_icon`] wants.
#[must_use]
pub fn frame_bitmap() -> Option<Bitmap> {
	bitmaps_at(&[FRAME_ICON_SIZE]).pop()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn the_embedded_artwork_decodes_at_every_size_it_is_asked_for() {
		// Guards the `include_bytes!` path and the PNG feature on the `image` dependency:
		// both fail at runtime, in a code path (the notification area) that is easy to miss.
		let mut sizes: Vec<u32> = NOTIFICATION_AREA_SIZES.to_vec();
		sizes.push(FRAME_ICON_SIZE);
		let decoded = rgba_at(&sizes);
		assert_eq!(decoded.len(), sizes.len(), "not every requested size came back");
		for (size, rgba) in decoded {
			let expected = (size * size * 4) as usize;
			assert_eq!(rgba.len(), expected, "{size}x{size} is not RGBA at the requested size");
		}
	}

	#[test]
	fn the_artwork_is_not_blank() {
		// The iOS app ships a placeholder icon that is a solid white square; this makes sure
		// the same thing can never quietly happen here.
		let (_, rgba) = rgba_at(&[32]).pop().expect("artwork should decode");
		let first = &rgba[..4];
		assert!(rgba.chunks_exact(4).any(|px| px != first), "the icon artwork is a single flat colour");
	}
}
