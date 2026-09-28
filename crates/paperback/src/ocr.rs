//! The built-in OCR engine, for image-only PDF pages.
//!
//! What the user chooses between is this (local, free, immediate, plain text) and Scribe
//! (remote, paid, whole document, structured), which is a different feature on a different path
//! rather than another engine here.

pub use native_ocr::{Error as OcrError, max_image_dimension, recognize_rgba};
use patois::t;

/// The message to announce when OCR fails.
pub fn message(error: &OcrError) -> String {
	match error {
		// TRANSLATORS: Announced when OCR can't run because the system has no OCR language installed
		OcrError::NoLanguage => t("No OCR language is installed. Add one in your system language settings."),
		// TRANSLATORS: Announced when OCR is requested on a system that has no OCR engine
		OcrError::Unavailable => t("OCR is not available on this system."),
		// TRANSLATORS: Announced when the OCR engine fails on a page
		OcrError::InvalidImage(_) | OcrError::Failed(_) => t("OCR failed."),
	}
}
