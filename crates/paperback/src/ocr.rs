//! The built-in OCR engine, for image-only PDF pages.
//!
//! One engine per platform, picked at compile time: `Windows.Media.Ocr` on Windows, Vision on
//! macOS. There is no provider list and no engine picker. What the user chooses between is this
//! (local, free, immediate, plain text) and Scribe (remote, paid, whole document, structured),
//! which is a different feature on a different path rather than another entry here.
//!
//! [`recognize_rgba`] runs on a worker thread and the caller marshals the result back to the UI
//! thread with `wxdragon::call_after`.

use patois::t;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(not(any(target_os = "windows", target_os = "macos")))]
mod unavailable;
#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "macos")]
use macos as engine;
#[cfg(not(any(target_os = "windows", target_os = "macos")))]
use unavailable as engine;
#[cfg(target_os = "windows")]
use windows as engine;

/// Why an OCR attempt produced no text.
#[derive(Debug)]
pub enum OcrError {
	/// The engine exists but has no recognizer for any of the user's languages. On Windows this
	/// means no OCR language pack is installed, which is the single most likely failure for a
	/// reader outside the English-speaking world, so it gets its own message rather than being
	/// folded into a generic failure.
	NoLanguage,
	/// The engine refused the image, or failed while recognizing it.
	Failed(String),
	/// This build has no OCR engine (Linux, where neither Windows OCR nor Vision exists). The
	/// paths that would reach it are compiled out, so this is a backstop, not a normal outcome,
	/// and only the stub engine ever raises it.
	#[allow(dead_code)]
	Unavailable,
}

impl OcrError {
	/// The message to announce for this failure.
	pub fn message(&self) -> String {
		match self {
			// TRANSLATORS: Announced when OCR can't run because the system has no OCR language installed
			Self::NoLanguage => t("No OCR language is installed. Add one in your system language settings."),
			// TRANSLATORS: Announced when the OCR engine fails on a page
			Self::Failed(_) => t("OCR failed."),
			// TRANSLATORS: Announced when OCR is requested on a system that has no OCR engine
			Self::Unavailable => t("OCR is not available on this system."),
		}
	}
}

impl std::fmt::Display for OcrError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::NoLanguage => write!(f, "no OCR language installed"),
			Self::Failed(detail) => write!(f, "{detail}"),
			Self::Unavailable => write!(f, "no OCR engine on this platform"),
		}
	}
}

/// The largest image side the engine accepts. Pages are rendered no larger than this, since
/// anything bigger is downscaled by the engine anyway.
pub fn max_image_dimension() -> u32 {
	engine::max_image_dimension()
}

/// Recognizes text in an RGBA8 bitmap (4 bytes per pixel, row-major, `width * height * 4`
/// bytes), returning the recognized lines joined with newlines.
///
/// # Errors
///
/// Returns [`OcrError::NoLanguage`] when the system has no usable OCR language, and
/// [`OcrError::Failed`] when the engine rejects the image or fails to recognize it.
pub fn recognize_rgba(rgba: &[u8], width: u32, height: u32) -> Result<String, OcrError> {
	let expected = (width as usize) * (height as usize) * 4;
	if rgba.len() != expected {
		return Err(OcrError::Failed(format!("bitmap byte count mismatch (got {}, expected {expected})", rgba.len())));
	}
	engine::recognize_rgba(rgba, width, height)
}
