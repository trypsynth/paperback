//! The built-in OCR engine, for image-only PDF pages.
//!
//! What the user chooses between is this (local, free, immediate, plain text) and Scribe
//! (remote, paid, whole document, structured), which is a different feature on a different path
//! rather than another engine here.

pub use native_ocr::{Error as OcrError, max_image_dimension};
use native_ocr::{Line, recognize_lines, recognize_rgba};
use paperback_core::ocr::{RecognizedLine, RecognizedWord, arrange};
use patois::t;

/// Reads the text in an RGBA8 bitmap. With `arranged`, its lines are put back in reading order from where each word was read and joined into paragraphs (see [`arrange`]); without, they come as the engine read them, one to a line.
pub fn recognize_text(rgba: &[u8], width: u32, height: u32, arranged: bool) -> Result<String, OcrError> {
	if !arranged {
		return recognize_rgba(rgba, width, height);
	}
	let lines: Vec<RecognizedLine> = recognize_lines(rgba, width, height)?.into_iter().map(recognized).collect();
	Ok(arrange(&lines, true))
}

fn recognized(line: Line) -> RecognizedLine {
	let words = line
		.words
		.into_iter()
		.map(|word| RecognizedWord {
			text: word.text,
			x: word.bounds.x,
			y: word.bounds.y,
			width: word.bounds.width,
			height: word.bounds.height,
		})
		.collect();
	RecognizedLine { text: line.text, words }
}

/// The message to announce when OCR fails.
pub fn message(error: &OcrError) -> String {
	match error {
		OcrError::NoLanguage => no_language_message(),
		// TRANSLATORS: Announced on Linux when OCR is requested but the Tesseract OCR engine isn't installed
		OcrError::NotInstalled => t("OCR needs Tesseract. Install it from your package manager."),
		// TRANSLATORS: Announced when the OCR engine fails on a page
		OcrError::InvalidImage(_) | OcrError::Failed(_) => t("OCR failed."),
		// TRANSLATORS: Announced when OCR is requested on a system that has no OCR engine
		OcrError::Unavailable => t("OCR is not available on this system."),
	}
}

#[cfg(not(target_os = "linux"))]
fn no_language_message() -> String {
	// TRANSLATORS: Announced when OCR can't run because the system has no OCR language installed
	t("No OCR language is installed. Add one in your system language settings.")
}

/// Linux's languages are Tesseract data packages rather than a system setting.
#[cfg(target_os = "linux")]
fn no_language_message() -> String {
	// TRANSLATORS: Announced on Linux when Tesseract is installed but has no language data for OCR
	t("No OCR language is installed. Install a Tesseract language package from your package manager.")
}
