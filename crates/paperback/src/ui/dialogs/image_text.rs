//! Transient by design: text recognized here never reaches the document; that is what Batch OCR is for.

use paperback_core::util::html::escape;
use patois::t;
use wxdragon::prelude::Frame;

use super::show_web_view_dialog;

pub fn show_image_text_dialog(parent: &Frame, text: &str) {
	// OCR output is line by line, and HTML prose would run the lines together.
	let html = format!("<pre>{}</pre>", escape(text));
	// TRANSLATORS: Title of the view showing the text recognized in a picture
	show_web_view_dialog(parent, &t("Image Text"), &html, false, None);
}
