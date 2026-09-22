//! The no-engine case: a build with neither Windows OCR nor Vision, which today means Linux.
//!
//! The parser tells those readers a page is image-only without offering OCR, and the menu item
//! and the Enter binding are compiled out, so nothing should reach this module. It exists so the
//! rest of the OCR code builds on every platform instead of being wrapped in `cfg` throughout.

use super::OcrError;

pub fn max_image_dimension() -> u32 {
	0
}

pub fn recognize_rgba(_rgba: &[u8], _width: u32, _height: u32) -> Result<String, OcrError> {
	Err(OcrError::Unavailable)
}
