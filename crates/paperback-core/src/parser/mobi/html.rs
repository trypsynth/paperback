//! Post-processing of the decoded MOBI HTML content: old-style Mobipocket files use
//! `<font size="N">` instead of semantic `<h1>`-`<h6>` tags, so this rewrites them into real
//! headings when the document has none, letting the heading-based TOC builder pick them up.

use std::sync::LazyLock;

pub(super) fn rewrite_font_size_headings(html: &str) -> String {
	static RE_H1_6: LazyLock<regex::Regex> = LazyLock::new(|| regex::Regex::new(r"(?i)<h[1-6]\b").unwrap());
	if RE_H1_6.is_match(html) {
		tracing::debug!("existing heading tags found, skipping font size heading heuristic");
		return html.to_string();
	}
	tracing::debug!("no existing heading tags found, applying font size heading heuristic");
	// One pass over the whole book, not one per size: each pass was a full scan and a full copy
	// of a string that runs to a hundred megabytes on the largest books. Where a font tag is left
	// unclosed, the size that opens first now wins rather than the largest size; see
	// `an_unclosed_font_tag_is_taken_in_the_order_it_opens`.
	static RE_FONT: LazyLock<regex::Regex> =
		LazyLock::new(|| regex::Regex::new(r#"(?is)<font\b[^>]*\bsize=["']?([4-7])["']?[^>]*>(.*?)</font>"#).unwrap());
	RE_FONT
		.replace_all(html, |caps: &regex::Captures<'_>| {
			// Size 7 is the largest and becomes <h1>, down to size 4 becoming <h4>.
			let level = b'7' - caps[1].as_bytes()[0] + 1;
			format!("<h{level}>{}</h{level}>", &caps[2])
		})
		.into_owned()
}

#[cfg(test)]
mod tests;
