//! The Windows OCR engine (`Windows.Media.Ocr`), reached through the `windows` crate.
//!
//! Recognition quality depends on the installed OCR language packs matching the document, and
//! the API exposes no preprocessing or accuracy knobs, so there is nothing to tune here. The
//! newer NPU-accelerated Windows AI text recognition would be faster and more accurate but
//! needs Copilot+ class hardware and the Windows App SDK, so it stays out; swapping it in later
//! means replacing this file and nothing else.

use std::ptr::addr_of_mut;

use windows::{
	Graphics::Imaging::{BitmapBufferAccessMode, BitmapPixelFormat, SoftwareBitmap},
	Media::Ocr::OcrEngine,
	Win32::System::{
		Com::{COINIT_MULTITHREADED, CoInitializeEx, CoUninitialize},
		WinRT::IMemoryBufferByteAccess,
	},
	core::Interface,
};

use super::OcrError;

/// What the engine reports as its largest accepted image side, or the documented 2600 px if it
/// can't be read.
pub fn max_image_dimension() -> u32 {
	OcrEngine::MaxImageDimension().unwrap_or(2600)
}

/// Recognizes `rgba` on the calling thread, which must not already be COM-initialized: this
/// sets up a multithreaded apartment and tears it down on the way out.
pub fn recognize_rgba(rgba: &[u8], width: u32, height: u32) -> Result<String, OcrError> {
	// A worker thread of ours, so the apartment is ours to set up. S_FALSE means it was already
	// initialized compatibly, which is fine; only a hard error is worth reporting.
	unsafe { CoInitializeEx(None, COINIT_MULTITHREADED) }
		.ok()
		.map_err(|err| OcrError::Failed(format!("COM init failed: {err}")))?;
	let result = recognize_inner(rgba, width, height);
	unsafe {
		CoUninitialize();
	}
	result
}

fn recognize_inner(rgba: &[u8], width: u32, height: u32) -> Result<String, OcrError> {
	let engine = create_engine()?;
	let bitmap = build_bitmap(rgba, width, height)?;
	let result = engine
		.RecognizeAsync(&bitmap)
		.and_then(|op| op.join())
		.map_err(|err| OcrError::Failed(format!("recognition failed: {err}")))?;
	let lines = result.Lines().map_err(|err| OcrError::Failed(format!("reading OCR lines failed: {err}")))?;
	let mut text = String::new();
	for line in lines {
		let Ok(line_text) = line.Text() else {
			continue;
		};
		text.push_str(&line_text.to_string_lossy());
		text.push('\n');
	}
	Ok(text.trim_end().to_string())
}

/// Builds an engine for the user's profile languages, telling a missing language pack apart
/// from a real failure: the API returns nothing at all when it has no recognizer to offer.
fn create_engine() -> Result<OcrEngine, OcrError> {
	match OcrEngine::TryCreateFromUserProfileLanguages() {
		Ok(engine) => Ok(engine),
		Err(err) => {
			let has_any = OcrEngine::AvailableRecognizerLanguages().is_ok_and(|langs| langs.Size().unwrap_or(0) > 0);
			if has_any {
				Err(OcrError::Failed(format!("creating the OCR engine failed: {err}")))
			} else {
				Err(OcrError::NoLanguage)
			}
		}
	}
}

/// Copies the rendered page into a `SoftwareBitmap` the engine can read.
fn build_bitmap(rgba: &[u8], width: u32, height: u32) -> Result<SoftwareBitmap, OcrError> {
	let expected = (width as usize) * (height as usize) * 4;
	let bitmap = SoftwareBitmap::Create(
		BitmapPixelFormat::Rgba8,
		i32::try_from(width).map_err(|_| OcrError::Failed("bitmap width out of range".to_string()))?,
		i32::try_from(height).map_err(|_| OcrError::Failed("bitmap height out of range".to_string()))?,
	)
	.map_err(|err| failed("creating the bitmap", &err))?;
	let buffer = bitmap.LockBuffer(BitmapBufferAccessMode::Write).map_err(|err| failed("locking the bitmap", &err))?;
	let reference = buffer.CreateReference().map_err(|err| failed("referencing the bitmap buffer", &err))?;
	let access: IMemoryBufferByteAccess =
		reference.cast().map_err(|err| failed("accessing the bitmap buffer", &err))?;
	let mut data: *mut u8 = std::ptr::null_mut();
	let mut capacity: u32 = 0;
	unsafe { access.GetBuffer(addr_of_mut!(data), addr_of_mut!(capacity)) }
		.map_err(|err| failed("reading the bitmap buffer", &err))?;
	if data.is_null() || capacity as usize != expected {
		return Err(OcrError::Failed(format!(
			"bitmap buffer size mismatch (capacity {capacity}, expected {expected})"
		)));
	}
	unsafe {
		std::ptr::copy_nonoverlapping(rgba.as_ptr(), data, expected);
	}
	// The write lock and its buffer references drop as this returns, before the engine reads
	// the bitmap.
	Ok(bitmap)
}

/// Wraps a `WinRT` error with the step that produced it, for the log.
fn failed(context: &str, err: &windows::core::Error) -> OcrError {
	OcrError::Failed(format!("{context}: {err}"))
}
