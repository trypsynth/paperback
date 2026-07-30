//! `patois-build`'s `.pot` generation shells out to `xgettext --language=C` to scan Rust
//! source. C's tokenizer doesn't understand two extremely common pieces of Rust syntax:
//! lifetimes (`'a`, `'static`, `'_`) and raw strings (`r"..."`, `r#"..."#`). It sees the lone
//! `'` of a lifetime and starts scanning for a closing quote, or sees `r#"` and doesn't
//! recognize the `#`-delimited terminator, and in both cases runs on past the real end of the
//! construct — sometimes swallowing or splicing together unrelated strings/comments later in
//! the file (or even in a later file, since every file is fed to one `xgettext` invocation).
//!
//! This neutralizes both constructs before the source ever reaches `xgettext`, while leaving
//! actual double-quoted string literals and `//`/`/* */` comments byte-for-byte untouched
//! (comments matter here since `// TRANSLATORS:` lines must survive verbatim). Replacements are
//! always same-length-per-line (characters become spaces, newlines stay newlines), so line
//! numbers — and therefore comment-to-string adjacency — never shift.

pub fn sanitize_for_xgettext(src: &str) -> String {
	let chars: Vec<char> = src.chars().collect();
	let n = chars.len();
	let mut out = String::with_capacity(src.len());
	let mut i = 0;
	while i < n {
		let c = chars[i];
		if c == '/' && i + 1 < n && chars[i + 1] == '/' {
			while i < n && chars[i] != '\n' {
				out.push(chars[i]);
				i += 1;
			}
		} else if c == '/' && i + 1 < n && chars[i + 1] == '*' {
			out.push('/');
			out.push('*');
			i += 2;
			let mut depth = 1usize;
			while i < n && depth > 0 {
				if i + 1 < n && chars[i] == '/' && chars[i + 1] == '*' {
					out.push('/');
					out.push('*');
					i += 2;
					depth += 1;
				} else if i + 1 < n && chars[i] == '*' && chars[i + 1] == '/' {
					out.push('*');
					out.push('/');
					i += 2;
					depth -= 1;
				} else {
					out.push(chars[i]);
					i += 1;
				}
			}
		} else if c == '"' {
			out.push(c);
			i += 1;
			while i < n {
				let ch = chars[i];
				out.push(ch);
				if ch == '\\' && i + 1 < n {
					out.push(chars[i + 1]);
					i += 2;
					continue;
				}
				i += 1;
				if ch == '"' {
					break;
				}
			}
		} else if c == 'r' {
			if let Some(hashes) = raw_string_hash_count(&chars, i) {
				for _ in 0..(hashes + 2) {
					out.push(' ');
				}
				i += hashes + 2;
				blank_raw_string_body(&chars, &mut i, hashes, &mut out);
			} else {
				out.push(c);
				i += 1;
			}
		} else if c == '\'' {
			if let Some(end) = char_literal_end(&chars, i) {
				for k in i..=end {
					out.push(chars[k]);
				}
				i = end + 1;
			} else {
				out.push(' ');
				i += 1;
			}
		} else {
			out.push(c);
			i += 1;
		}
	}
	out
}

/// If `chars[i]` is `'r'` starting a raw string (`r"`, `r#"`, `r##"`, ...), return the hash count.
fn raw_string_hash_count(chars: &[char], i: usize) -> Option<usize> {
	let n = chars.len();
	let mut j = i + 1;
	let mut hashes = 0usize;
	while j < n && chars[j] == '#' {
		hashes += 1;
		j += 1;
	}
	if j < n && chars[j] == '"' { Some(hashes) } else { None }
}

/// Consume (and blank) the body of a raw string starting right after its opening delimiter,
/// stopping once the matching `"` + `hashes` `#`s closer is found (also blanked).
fn blank_raw_string_body(chars: &[char], i: &mut usize, hashes: usize, out: &mut String) {
	let n = chars.len();
	while *i < n {
		if chars[*i] == '"' {
			let mut k = *i + 1;
			let mut matched = 0usize;
			while k < n && matched < hashes && chars[k] == '#' {
				matched += 1;
				k += 1;
			}
			if matched == hashes {
				for _ in 0..(hashes + 1) {
					out.push(' ');
				}
				*i = k;
				return;
			}
		}
		if chars[*i] == '\n' {
			out.push('\n');
		} else {
			out.push(' ');
		}
		*i += 1;
	}
}

/// If `chars[i]` is a `'` starting a plausible short char literal (`'x'`, `'\n'`, `'\''`,
/// `'\u{2603}'`), return the index of its closing `'`. Otherwise `None` — it's a lifetime.
fn char_literal_end(chars: &[char], i: usize) -> Option<usize> {
	let n = chars.len();
	let mut j = i + 1;
	if j >= n {
		return None;
	}
	if chars[j] == '\\' {
		j += 1;
		if j >= n {
			return None;
		}
		if chars[j] == 'u' && j + 1 < n && chars[j + 1] == '{' {
			let mut k = j + 2;
			while k < n && chars[k] != '}' {
				k += 1;
			}
			if k >= n {
				return None;
			}
			j = k + 1;
		} else {
			j += 1;
		}
	} else {
		j += 1;
	}
	if j < n && chars[j] == '\'' { Some(j) } else { None }
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn preserves_normal_strings_and_comments() {
		let src = "// TRANSLATORS: hi\nlet x = t(\"hello 'world'\");\n";
		assert_eq!(sanitize_for_xgettext(src), src);
	}

	#[test]
	fn neutralizes_lifetimes_without_changing_length_or_lines() {
		let src = "fn f<'a>(s: &'a str) -> &'a str { s }\n";
		let out = sanitize_for_xgettext(src);
		assert_eq!(out.chars().count(), src.chars().count());
		assert_eq!(out.lines().count(), src.lines().count());
		assert!(!out.contains('\''));
	}

	#[test]
	fn preserves_char_literals() {
		let src = "let c = '{'; let d = '\\n'; let e = '\\'';\n";
		let out = sanitize_for_xgettext(src);
		assert!(out.contains("'{'"));
		assert!(out.contains("'\\n'"));
		assert!(out.contains("'\\''"));
	}

	#[test]
	fn blanks_raw_strings_but_keeps_line_count() {
		let src = "let xml = r#\"\n<a href=\"x\">it's</a>\n\"#;\nlet y = t(\"real\");\n";
		let out = sanitize_for_xgettext(src);
		assert_eq!(out.lines().count(), src.lines().count());
		assert!(out.contains("t(\"real\")"));
		assert!(!out.contains("href"));
	}
}
