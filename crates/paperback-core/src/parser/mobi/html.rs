//! Old Mobipocket files use `<font size="N">` instead of heading tags, so a document with none gets them rewritten into headings the TOC builder can find.

use std::{borrow::Cow, sync::LazyLock};

pub(super) fn rewrite_font_size_headings(html: String) -> String {
	static RE_H1_6: LazyLock<regex::Regex> = LazyLock::new(|| regex::Regex::new(r"(?i)<h[1-6]\b").unwrap());
	if RE_H1_6.is_match(&html) {
		tracing::debug!("existing heading tags found, skipping font size heading heuristic");
		return html;
	}
	tracing::debug!("no existing heading tags found, applying font size heading heuristic");
	// One pass, not one per size: each pass was a full scan and copy of a string that runs to a hundred megabytes on the largest books. An unclosed font tag now goes to the size that opens first; see `an_unclosed_font_tag_is_taken_in_the_order_it_opens`.
	static RE_FONT: LazyLock<regex::Regex> =
		LazyLock::new(|| regex::Regex::new(r#"(?is)<font\b[^>]*\bsize=["']?([4-7])["']?[^>]*>(.*?)</font>"#).unwrap());
	let rewritten = RE_FONT.replace_all(&html, |caps: &regex::Captures<'_>| {
		let level = b'7' - caps[1].as_bytes()[0] + 1;
		format!("<h{level}>{}</h{level}>", &caps[2])
	});
	match rewritten {
		Cow::Owned(rewritten) => rewritten,
		Cow::Borrowed(_) => html,
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn rewrites_font_size_seven_to_h1() {
		let html = r#"<p>Before</p><font size="7">Title</font><p>After</p>"#;
		assert_eq!(rewrite_font_size_headings(html.to_string()), "<p>Before</p><h1>Title</h1><p>After</p>");
	}

	#[test]
	fn rewrites_each_recognized_size_to_its_own_level() {
		let html = concat!(
			r#"<font size="7">one</font>"#,
			r#"<font size="6">two</font>"#,
			r#"<font size="5">three</font>"#,
			r#"<font size="4">four</font>"#,
		);
		assert_eq!(rewrite_font_size_headings(html.to_string()), "<h1>one</h1><h2>two</h2><h3>three</h3><h4>four</h4>");
	}

	#[test]
	fn leaves_unrecognized_font_sizes_untouched() {
		let html = r#"<font size="2">small print</font>"#;
		assert_eq!(rewrite_font_size_headings(html.to_string()), html);
	}

	#[test]
	fn does_nothing_when_semantic_headings_already_exist() {
		let html = r#"<h2>Existing Heading</h2><font size="7">Not a heading</font>"#;
		assert_eq!(rewrite_font_size_headings(html.to_string()), html);
	}

	#[test]
	fn matches_size_attribute_regardless_of_quote_style_or_case() {
		let html = "<FONT SIZE=7>Title</FONT>";
		assert_eq!(rewrite_font_size_headings(html.to_string()), "<h1>Title</h1>");
	}

	#[test]
	fn leaves_content_with_no_font_tags_untouched() {
		let html = "<p>Just a paragraph.</p>";
		assert_eq!(rewrite_font_size_headings(html.to_string()), html);
	}

	// Old Mobipocket HTML leaves font tags unclosed; taking them in the order they open keeps the text after an unclosed tag inside the heading it opened rather than stranded in a block of its own.
	#[test]
	fn an_unclosed_font_tag_is_taken_in_the_order_it_opens() {
		let html = r#"<font size="4">outer<font size="7">inner</font>"#;
		assert_eq!(rewrite_font_size_headings(html.to_string()), r#"<h4>outer<font size="7">inner</h4>"#);
	}
}
