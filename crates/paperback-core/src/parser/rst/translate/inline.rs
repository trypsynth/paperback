//! Inline markup: `**bold**`, `*emphasis*`, ` ``literal`` `, hyperlink references, footnote/
//! citation references, `|substitutions|` and bare URLs, turned into an HTML fragment.
//!
//! Runs as a single left-to-right scan over the paragraph's characters. Nothing here recurses
//! into matched spans (RST itself disallows nested inline markup), so a false match can never
//! send the scanner into a loop; anything not recognized falls through as literal, HTML-escaped
//! text, which is what keeps this safe to run on arbitrary, possibly malformed, input.

use std::collections::HashMap;

use super::escape_html;

pub(super) fn render(text: &str, targets: &HashMap<String, String>, substitutions: &HashMap<String, String>) -> String {
	let chars: Vec<char> = text.chars().collect();
	let mut out = String::new();
	let mut i = 0;
	while i < chars.len() {
		if chars[i] == '\\' && i + 1 < chars.len() {
			out.push_str(&escape_html(&chars[i + 1].to_string()));
			i += 2;
			continue;
		}
		if let Some((len, html)) = match_markup(&chars, i, targets, substitutions) {
			out.push_str(&html);
			i += len;
			continue;
		}
		out.push_str(&escape_html(&chars[i].to_string()));
		i += 1;
	}
	out
}

fn match_markup(
	chars: &[char],
	i: usize,
	targets: &HashMap<String, String>,
	substitutions: &HashMap<String, String>,
) -> Option<(usize, String)> {
	if let Some(result) = match_role(chars, i) {
		return Some(result);
	}
	if let Some(result) = match_literal(chars, i) {
		return Some(result);
	}
	if let Some(result) = match_backtick_reference(chars, i, targets) {
		return Some(result);
	}
	if let Some(result) = match_strong(chars, i) {
		return Some(result);
	}
	if let Some(result) = match_emphasis(chars, i) {
		return Some(result);
	}
	if let Some(result) = match_footnote_ref(chars, i) {
		return Some(result);
	}
	if let Some(result) = match_substitution(chars, i, substitutions) {
		return Some(result);
	}
	if let Some(result) = match_bare_reference(chars, i, targets) {
		return Some(result);
	}
	if let Some(result) = match_url(chars, i) {
		return Some(result);
	}
	None
}

/// `` ``literal`` ``, not confused with `**strong**` by requiring a matching double backtick.
fn match_literal(chars: &[char], i: usize) -> Option<(usize, String)> {
	if !starts_with(chars, i, "``") {
		return None;
	}
	let close = find_seq(chars, i + 2, "``")?;
	if close == i + 2 {
		return None;
	}
	let inner: String = chars[i + 2..close].iter().collect();
	Some((close + 2 - i, format!("<code>{}</code>", escape_html(&inner))))
}

fn match_strong(chars: &[char], i: usize) -> Option<(usize, String)> {
	if !starts_with(chars, i, "**") {
		return None;
	}
	let close = find_seq(chars, i + 2, "**")?;
	if close == i + 2 {
		return None;
	}
	let inner: String = chars[i + 2..close].iter().collect();
	Some((close + 2 - i, format!("<strong>{}</strong>", escape_html(&inner))))
}

/// `*emphasis*`, single asterisk, never matching into a `**strong**` span's delimiters.
fn match_emphasis(chars: &[char], i: usize) -> Option<(usize, String)> {
	if chars.get(i) != Some(&'*') || chars.get(i + 1) == Some(&'*') {
		return None;
	}
	let mut j = i + 1;
	while j < chars.len() {
		if chars[j] == '*' && chars.get(j + 1) != Some(&'*') {
			if j == i + 1 {
				return None;
			}
			let inner: String = chars[i + 1..j].iter().collect();
			return Some((j + 1 - i, format!("<em>{}</em>", escape_html(&inner))));
		}
		j += 1;
	}
	None
}

/// `` :role:`text` ``, an interpreted-text role. The role itself carries no meaning to a reader,
/// so only the text survives.
fn match_role(chars: &[char], i: usize) -> Option<(usize, String)> {
	if chars.get(i) != Some(&':') {
		return None;
	}
	let close = find_seq(chars, i + 1, ":`")?;
	let role: String = chars[i + 1..close].iter().collect();
	if role.is_empty() || !role.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_') {
		return None;
	}
	let text_start = close + 2;
	let text_end = find_char(chars, text_start, '`')?;
	let inner: String = chars[text_start..text_end].iter().collect();
	Some((text_end + 1 - i, escape_html(&inner)))
}

/// `` `text <url>`_ `` and `` `text <url>`__ `` (embedded URI), and `` `text`_ ``/`` `text`__ ``
/// (reference to a `.. _text:` target, or a section's implicit one, defined elsewhere).
fn match_backtick_reference(chars: &[char], i: usize, targets: &HashMap<String, String>) -> Option<(usize, String)> {
	if chars.get(i) != Some(&'`') || chars.get(i + 1) == Some(&'`') {
		return None;
	}
	let close = find_char(chars, i + 1, '`')?;
	if chars.get(close + 1) == Some(&'`') {
		return None;
	}
	let mut end = close + 1;
	let underscores = chars[end..].iter().take_while(|&&c| c == '_').count();
	if underscores == 0 {
		return None;
	}
	end += underscores;
	let inner: String = chars[i + 1..close].iter().collect();
	if let Some(lt) = inner.rfind('<')
		&& inner.ends_with('>')
	{
		let label = inner[..lt].trim().to_string();
		// A long URL may be wrapped onto a second source line; docutils joins it back up by
		// dropping the whitespace, since real URLs never contain any.
		let url: String = inner[lt + 1..inner.len() - 1].split_whitespace().collect();
		return Some((end - i, format!(r#"<a href="{}">{}</a>"#, escape_html(&url), escape_html(&label))));
	}
	let key = inner.trim().to_ascii_lowercase().replace(['_', ' '], "-");
	if let Some(url) = targets.get(&key) {
		return Some((end - i, format!(r#"<a href="{}">{}</a>"#, escape_html(url), escape_html(&inner))));
	}
	Some((end - i, escape_html(&inner)))
}

/// A bare `word_` reference to a target, without backticks. Left untouched (underscore and all)
/// when no matching target is defined, since a trailing underscore in prose is otherwise
/// meaningless markup rather than something worth stripping speculatively.
fn match_bare_reference(chars: &[char], i: usize, targets: &HashMap<String, String>) -> Option<(usize, String)> {
	let c = *chars.get(i)?;
	if !c.is_alphanumeric() {
		return None;
	}
	if i > 0 && (chars[i - 1].is_alphanumeric() || chars[i - 1] == '_') {
		return None;
	}
	let mut j = i;
	while j < chars.len() && (chars[j].is_alphanumeric() || matches!(chars[j], '.' | '-' | '+')) {
		j += 1;
	}
	if chars.get(j) != Some(&'_') || chars.get(j + 1) == Some(&'_') {
		return None;
	}
	let word: String = chars[i..j].iter().collect();
	let key = word.to_ascii_lowercase();
	let url = targets.get(&key)?;
	Some((j + 1 - i, format!(r#"<a href="{}">{}</a>"#, escape_html(url), escape_html(&word))))
}

/// `[1]_`, `[#]_`, `[*]_`, `[Name]_`: a footnote or citation reference. Rendered as its bracketed
/// label with no link, since resolving it to the matching footnote body is not attempted.
fn match_footnote_ref(chars: &[char], i: usize) -> Option<(usize, String)> {
	if chars.get(i) != Some(&'[') {
		return None;
	}
	let close = find_char(chars, i + 1, ']')?;
	if chars.get(close + 1) != Some(&'_') {
		return None;
	}
	let label: String = chars[i + 1..close].iter().collect();
	if label.is_empty() || (label.contains(' ') && !label.chars().next().is_some_and(char::is_uppercase)) {
		return None;
	}
	Some((close + 2 - i, format!("[{}]", escape_html(&label))))
}

/// `|name|`, replaced by its recorded `.. |name| replace::` text, or left as-is if undefined or
/// carrying no plain-text value (an image substitution, say).
fn match_substitution(chars: &[char], i: usize, substitutions: &HashMap<String, String>) -> Option<(usize, String)> {
	if chars.get(i) != Some(&'|') {
		return None;
	}
	let close = find_char(chars, i + 1, '|')?;
	let name: String = chars[i + 1..close].iter().collect();
	let key = name.trim().to_ascii_lowercase();
	let replacement = substitutions.get(&key)?;
	let text = if replacement.is_empty() { format!("|{name}|") } else { replacement.clone() };
	Some((close + 1 - i, escape_html(&text)))
}

fn match_url(chars: &[char], i: usize) -> Option<(usize, String)> {
	let prefix = if starts_with(chars, i, "https://") {
		8
	} else if starts_with(chars, i, "http://") {
		7
	} else {
		return None;
	};
	let mut j = i + prefix;
	while j < chars.len() && !chars[j].is_whitespace() && !matches!(chars[j], '<' | '>') {
		j += 1;
	}
	while j > i + prefix && matches!(chars[j - 1], '.' | ',' | ')' | '"' | '\'') {
		j -= 1;
	}
	if j == i + prefix {
		return None;
	}
	let url: String = chars[i..j].iter().collect();
	Some((j - i, format!(r#"<a href="{}">{}</a>"#, escape_html(&url), escape_html(&url))))
}

fn starts_with(chars: &[char], i: usize, needle: &str) -> bool {
	let needle: Vec<char> = needle.chars().collect();
	i + needle.len() <= chars.len() && chars[i..i + needle.len()] == needle[..]
}

fn find_seq(chars: &[char], from: usize, needle: &str) -> Option<usize> {
	let needle: Vec<char> = needle.chars().collect();
	(from..=chars.len().saturating_sub(needle.len())).find(|&j| chars[j..j + needle.len()] == needle[..])
}

fn find_char(chars: &[char], from: usize, c: char) -> Option<usize> {
	(from..chars.len()).find(|&j| chars[j] == c)
}
