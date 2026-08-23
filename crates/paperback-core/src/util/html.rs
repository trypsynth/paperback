/// Escapes a single character for safe use in HTML text content (`&`, `<`, `>`), appending
/// the result to `out`. Kept separate from [`escape`] since some callers stream characters
/// one at a time rather than escaping a whole string up front.
pub fn push_escaped(ch: char, out: &mut String) {
	match ch {
		'&' => out.push_str("&amp;"),
		'<' => out.push_str("&lt;"),
		'>' => out.push_str("&gt;"),
		c => out.push(c),
	}
}

/// Escapes `&`, `<`, and `>` for safe use in HTML text content.
#[must_use]
pub fn escape(s: &str) -> String {
	let mut out = String::with_capacity(s.len());
	for ch in s.chars() {
		push_escaped(ch, &mut out);
	}
	out
}

/// Escapes `&` and `"` for safe use inside a double-quoted HTML attribute value.
#[must_use]
pub fn escape_attr(s: &str) -> String {
	s.replace('&', "&amp;").replace('"', "&quot;")
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn escape_replaces_html_special_chars() {
		assert_eq!(escape("<a> & </a>"), "&lt;a&gt; &amp; &lt;/a&gt;");
	}

	#[test]
	fn escape_leaves_ordinary_text_untouched() {
		assert_eq!(escape("hello world"), "hello world");
	}

	#[test]
	fn escape_attr_escapes_ampersand_before_quote() {
		// & must be escaped first, or the & inserted by the quote replacement would itself
		// get mangled.
		assert_eq!(escape_attr(r#"a & "b""#), "a &amp; &quot;b&quot;");
	}

	#[test]
	fn push_escaped_appends_to_existing_buffer() {
		let mut out = String::from("prefix:");
		push_escaped('&', &mut out);
		push_escaped('x', &mut out);
		assert_eq!(out, "prefix:&amp;x");
	}
}
