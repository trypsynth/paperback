use crate::t;

/// So a slide of pictures alone can still be reached and the reader knows it is there.
#[must_use]
pub fn no_text_placeholder() -> String {
	// TRANSLATORS: Shown in place of a presentation slide that has no text, such as one holding only pictures
	t("This slide has no text.")
}
