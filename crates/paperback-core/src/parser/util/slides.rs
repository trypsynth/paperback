//! What the presentation parsers share, for `.pptx`, `.ppt` and `.odp` slides alike.

use crate::t;

/// The line a slide with no text shows, so it can still be reached and the reader knows it is there. A slide of pictures alone is the usual one.
pub fn no_text_placeholder() -> String {
	// TRANSLATORS: Shown in place of a presentation slide that has no text, such as one holding only pictures
	t("This slide has no text.")
}
