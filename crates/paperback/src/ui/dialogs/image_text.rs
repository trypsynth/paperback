//! The text recognized in a picture, shown the way the table and formula views are shown: in the
//! web view, which the reader leaves with Escape.
//!
//! Transient by design: what is read here never reaches the document, which is what Batch OCR is
//! for.

use paperback_core::util::html::escape;
use patois::t;
use wxdragon::prelude::Frame;

use super::show_web_view_dialog;

pub fn show_image_text_dialog(parent: &Frame, text: &str) {
	// The engine hands its output back line by line, so it is written the way it arrived rather
	// than as HTML prose that would run the lines together.
	let html = format!("<pre>{}</pre>", escape(text));
	// TRANSLATORS: Title of the view showing the text recognized in a picture
	show_web_view_dialog(parent, &t("Image Text"), &html, false, None);
}
