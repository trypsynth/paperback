//! Only the runtime bitmaps; on Windows, Explorer, the taskbar and file associations read the icon resource `build.rs` embeds, since they never run our code.

use wxdragon::prelude::*;

/// Shared with the Android and iOS apps (`android/store_assets`) and the `.ico` `build.rs` embeds, so every platform stays in step.
const ICON_PNG: &[u8] = include_bytes!("../../assets/paperback.png");

/// Windows picks 16 at 100% scaling and 32 at 200%, and wants 20 or 24 in between.
#[cfg(any(target_os = "windows", test))]
const NOTIFICATION_AREA_SIZES: [u32; 4] = [16, 20, 24, 32];

/// `Frame::set_icon` takes one bitmap from which wx derives both the small and large icons, so this is a compromise between them.
const FRAME_ICON_SIZE: u32 = 32;

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

fn bitmaps_at(sizes: &[u32]) -> Vec<Bitmap> {
	rgba_at(sizes).into_iter().filter_map(|(size, rgba)| Bitmap::from_rgba(&rgba, size, size)).collect()
}

#[must_use]
#[cfg(target_os = "windows")]
pub fn notification_area_bundle() -> Option<BitmapBundle> {
	let bitmaps = bitmaps_at(&NOTIFICATION_AREA_SIZES);
	if bitmaps.is_empty() {
		return None;
	}
	Some(BitmapBundle::from_bitmaps(&bitmaps))
}

#[must_use]
pub fn frame_bitmap() -> Option<Bitmap> {
	bitmaps_at(&[FRAME_ICON_SIZE]).pop()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn the_embedded_artwork_decodes_at_every_size_it_is_asked_for() {
		// Guards the `include_bytes!` path and the `image` PNG feature, which only fail at runtime in the easily missed notification area.
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
		// The iOS app ships a solid white placeholder icon; this keeps that from quietly happening here.
		let (_, rgba) = rgba_at(&[32]).pop().expect("artwork should decode");
		let first: [u8; 4] = rgba[..4].try_into().unwrap();
		assert!(rgba.as_chunks::<4>().0.iter().any(|px| *px != first), "the icon artwork is a single flat colour");
	}
}
