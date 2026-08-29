//! Windows OCR of RGBA page renders, for image-only PDF pages.
//!
//! [`recognize_rgba`] runs on a worker thread: it initializes a `COM` apartment, builds a
//! `WinRT` `SoftwareBitmap` from the pdfium-rendered RGBA bytes, and asks the user-profile OCR
//! engine to recognize the text. The caller marshals the result back to the UI thread with
//! `wxdragon::call_after`.

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

/// Recognizes text in an RGBA8 bitmap (4 bytes/pixel, row-major, `width * height * 4` bytes)
/// using the Windows OCR engine for the user's profile languages. Returns the recognized lines
/// joined with `\n`, or an error if no OCR language is available or recognition fails.
///
/// Must be called from a thread that is not otherwise COM-initialized; this function sets up a
/// multithreaded apartment and tears it down on the way out.
pub fn recognize_rgba(rgba: &[u8], width: u32, height: u32) -> anyhow::Result<String> {
	unsafe { CoInitializeEx(None, COINIT_MULTITHREADED) }.ok()?;
	let result = recognize_rgba_inner(rgba, width, height);
	unsafe {
		CoUninitialize();
	}
	result
}

fn recognize_rgba_inner(rgba: &[u8], width: u32, height: u32) -> anyhow::Result<String> {
	let rgba_len = rgba.len();
	let expected = (width as usize) * (height as usize) * 4;
	if rgba_len != expected {
		return Err(anyhow::anyhow!("OCR bitmap byte count mismatch (got {rgba_len}, expected {expected})"));
	}
	let bitmap = SoftwareBitmap::Create(
		BitmapPixelFormat::Rgba8,
		i32::try_from(width).map_err(|_| anyhow::anyhow!("OCR bitmap width out of range"))?,
		i32::try_from(height).map_err(|_| anyhow::anyhow!("OCR bitmap height out of range"))?,
	)?;
	// Copy the pdfium render into the SoftwareBitmap's buffer.
	{
		let buffer = bitmap.LockBuffer(BitmapBufferAccessMode::Write)?;
		let reference = buffer.CreateReference()?;
		let access: IMemoryBufferByteAccess = reference.cast()?;
		let mut data: *mut u8 = std::ptr::null_mut();
		let mut capacity: u32 = 0;
		unsafe { access.GetBuffer(addr_of_mut!(data), addr_of_mut!(capacity))? };
		if data.is_null() || capacity as usize != expected {
			return Err(anyhow::anyhow!(
				"SoftwareBitmap buffer size mismatch (capacity {capacity}, expected {expected})"
			));
		}
		unsafe {
			std::ptr::copy_nonoverlapping(rgba.as_ptr(), data, expected);
		}
	}
	let engine = OcrEngine::TryCreateFromUserProfileLanguages()?;
	let result = engine.RecognizeAsync(&bitmap)?.join()?;
	let mut text = String::new();
	for line in result.Lines()? {
		text.push_str(&line.Text()?.to_string_lossy());
		text.push('\n');
	}
	Ok(text.trim_end().to_string())
}
