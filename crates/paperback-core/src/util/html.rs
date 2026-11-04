/// Browser fixes required by the `MathML` emitted by `math-core`.
pub const MATHML_STYLES: &str = include_str!("mathml.css");

pub fn push_escaped(ch: char, out: &mut String) {
	match ch {
		'&' => out.push_str("&amp;"),
		'<' => out.push_str("&lt;"),
		'>' => out.push_str("&gt;"),
		c => out.push(c),
	}
}

pub fn push_escaped_str(text: &str, out: &mut String) {
	let mut start = 0;
	for at in memchr::memchr3_iter(b'&', b'<', b'>', text.as_bytes()) {
		out.push_str(&text[start..at]);
		out.push_str(match text.as_bytes()[at] {
			b'&' => "&amp;",
			b'<' => "&lt;",
			_ => "&gt;",
		});
		start = at + 1;
	}
	out.push_str(&text[start..]);
}

#[must_use]
pub fn escape(s: &str) -> String {
	let mut out = String::with_capacity(s.len());
	push_escaped_str(s, &mut out);
	out
}

#[must_use]
pub fn escape_attr(s: &str) -> String {
	// `&` first, or the `&` of each `&quot;` would be escaped again.
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
		assert_eq!(escape_attr(r#"a & "b""#), "a &amp; &quot;b&quot;");
	}

	#[test]
	fn push_escaped_str_escapes_every_special_character_and_keeps_the_rest() {
		let mut out = String::from("start ");
		push_escaped_str("é & <b> ü", &mut out);
		push_escaped_str("&", &mut out);
		push_escaped_str("", &mut out);
		assert_eq!(out, "start é &amp; &lt;b&gt; ü&amp;");
	}

	#[test]
	fn push_escaped_appends_to_existing_buffer() {
		let mut out = String::from("prefix:");
		push_escaped('&', &mut out);
		push_escaped('x', &mut out);
		assert_eq!(out, "prefix:&amp;x");
	}
}
