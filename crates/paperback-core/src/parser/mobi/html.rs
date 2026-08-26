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
	let mut result = html.to_string();
	for (size, level) in [(7u8, 1u8), (6, 2), (5, 3), (4, 4)] {
		let Ok(re) = regex::Regex::new(&format!(r#"(?is)<font\b[^>]*\bsize=["']?{size}["']?[^>]*>(.*?)</font>"#))
		else {
			continue;
		};
		result = re
			.replace_all(&result, |caps: &regex::Captures<'_>| format!("<h{level}>{}</h{level}>", &caps[1]))
			.into_owned();
	}
	result
}
