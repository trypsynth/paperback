//! pdfium's `CPDF_LinkExtract` (`core/fpdftext/cpdf_linkextract.cpp`): finding the bare URLs and email addresses in a page's text.
//!
//! It walks the page's characters and reads the page's text by the same positions, which pdfium does too even though the two can drift apart where a character has no text. The ranges it reports are only right if that is kept, since the parser reads a link's words back by character position.
//!
//! Portions are derived from `PDFium`, used under its BSD-style license:
//! Copyright 2014 The `PDFium` Authors. Original code copyright 2014 Foxit Software Inc. <http://www.foxitsoftware.com>

/// What the extractor needs to know about one character of the text layer.
pub(super) struct LinkChar {
	pub(super) generated: bool,
	pub(super) hyphen: bool,
	pub(super) normal: bool,
	pub(super) unicode: u32,
}

const SPACE: u32 = 0x20;

fn is_digit(c: u32) -> bool {
	(u32::from('0')..=u32::from('9')).contains(&c)
}

fn is_alphanumeric(c: u32) -> bool {
	char::from_u32(c).is_some_and(char::is_alphanumeric)
}

fn lower(text: &[u32]) -> Vec<u32> {
	text.iter().map(|&c| char::from_u32(c).map_or(c, |ch| u32::from(ch.to_lowercase().next().unwrap_or(ch)))).collect()
}

fn find(text: &[u32], needle: &[u32], from: usize) -> Option<usize> {
	(from..=text.len().checked_sub(needle.len())?).find(|&at| text[at..at + needle.len()] == *needle)
}

fn encode(s: &str) -> Vec<u32> {
	s.chars().map(u32::from).collect()
}

fn decode(text: &[u32]) -> String {
	text.iter().filter_map(|&c| char::from_u32(c)).collect()
}

/// Every link on the page as (first character, character count, URL).
pub(super) fn extract(chars: &[LinkChar], page_text: &[u32]) -> Vec<(i32, i32, String)> {
	let total = chars.len();
	let mut links = Vec::new();
	let (mut start, mut pos) = (0, 0);
	let (mut after_hyphen, mut line_break) = (false, false);
	while pos < total {
		let info = &chars[pos];
		if !info.generated && info.unicode != SPACE && pos != total - 1 {
			after_hyphen = info.hyphen || (info.normal && info.unicode == u32::from('-'));
			pos += 1;
			continue;
		}
		let mut count = pos - start;
		if pos == total - 1 {
			count += 1;
		} else if after_hyphen && (info.unicode == u32::from('\n') || info.unicode == u32::from('\r')) {
			line_break = true;
			pos += 1;
			continue;
		}
		let mut candidate: Vec<u32> = page_text.iter().skip(start).take(count).copied().collect();
		if line_break {
			candidate.retain(|&c| c != u32::from('\n') && c != u32::from('\r'));
			line_break = false;
		}
		for c in &mut candidate {
			if *c == 0xFFFE {
				*c = u32::from('-');
			}
		}
		if candidate.len() > 5 {
			while let Some(&last) = candidate.last() {
				if !matches!(char::from_u32(last), Some(')' | ',' | '>' | '.')) {
					break;
				}
				candidate.pop();
				count = count.saturating_sub(1);
			}
			if count > 5 {
				if let Some((link_start, link_count, url)) = check_web_link(&candidate) {
					links.push((to_i32(link_start + start), to_i32(link_count), url));
				} else if let Some(url) = check_mail_link(candidate) {
					links.push((to_i32(start), to_i32(count), url));
				}
			}
		}
		pos += 1;
		start = pos;
	}
	links
}

fn to_i32(value: usize) -> i32 {
	i32::try_from(value).unwrap_or(i32::MAX)
}

fn check_web_link(original: &[u32]) -> Option<(usize, usize, String)> {
	let text = lower(original);
	let http = encode("http");
	if let Some(start) = find(&text, &http, 0) {
		let mut off = start + http.len();
		if text.len() > off + 4 {
			if text[off] == u32::from('s') {
				off += 1;
			}
			if text[off] == u32::from(':') && text[off + 1] == u32::from('/') && text[off + 2] == u32::from('/') {
				off += 3;
				let end = find_web_link_ending(&text, off, trim_external_brackets(&text, start, text.len() - 1));
				if end > off {
					let count = end - start + 1;
					return Some((start, count, decode(&original[start..start + count])));
				}
			}
		}
	}
	let www = encode("www.");
	if let Some(start) = find(&text, &www, 0) {
		let off = start + www.len();
		if text.len() > off {
			let end = find_web_link_ending(&text, start, trim_external_brackets(&text, start, text.len() - 1));
			if end > off {
				let count = end - start + 1;
				return Some((start, count, format!("http://{}", decode(&original[start..start + count]))));
			}
		}
	}
	None
}

fn find_web_link_ending(text: &[u32], start: usize, mut end: usize) -> usize {
	if text[start..].contains(&u32::from('/')) {
		return end;
	}
	if text[start] == u32::from('[') {
		if let Some(close) = find(text, &[u32::from(']')], start + 1) {
			end = close;
			if end > start + 1 {
				let mut off = end + 1;
				if off < text.len() && text[off] == u32::from(':') {
					off += 1;
					while off < text.len() && is_digit(text[off]) {
						off += 1;
					}
					if off > end + 2 && off <= text.len() {
						end = off - 1;
					}
				}
			}
		}
		return end;
	}
	while end > start && text[end] < 0x80 {
		let c = text[end];
		if is_digit(c) || (u32::from('a')..=u32::from('z')).contains(&c) || c == u32::from('.') {
			break;
		}
		end -= 1;
	}
	end
}

fn trim_backwards_to_char(text: &[u32], wanted: char, start: usize, end: &mut usize) {
	let mut pos = *end;
	loop {
		if text[pos] == u32::from(wanted) {
			*end = pos.saturating_sub(1);
			break;
		}
		if pos == start || pos == 0 {
			break;
		}
		pos -= 1;
	}
}

fn trim_external_brackets(text: &[u32], start: usize, mut end: usize) -> usize {
	for &c in &text[..start] {
		let closing = match char::from_u32(c) {
			Some('(') => ')',
			Some('[') => ']',
			Some('{') => '}',
			Some('<') => '>',
			Some('"') => '"',
			Some('\'') => '\'',
			_ => continue,
		};
		trim_backwards_to_char(text, closing, start, &mut end);
	}
	end
}

fn check_mail_link(mut text: Vec<u32>) -> Option<String> {
	let at = text.iter().position(|&c| c == u32::from('@'))?;
	if at == 0 || at == text.len() - 1 {
		return None;
	}
	let mut p_pos = at;
	for i in (1..=at).rev() {
		let ch = text[i - 1];
		if ch == u32::from('_') || ch == u32::from('-') || is_alphanumeric(ch) {
			continue;
		}
		if ch != u32::from('.') || i == p_pos || i == 1 {
			if i == at {
				return None;
			}
			let removed = if i == p_pos { i + 1 } else { i };
			text.drain(..removed.min(text.len()));
			break;
		}
		p_pos = i - 1;
	}
	let at = text.iter().position(|&c| c == u32::from('@')).filter(|&at| at > 0)?;
	while text.last() == Some(&u32::from('.')) {
		text.pop();
	}
	let dot = find(&text, &[u32::from('.')], at + 1)?;
	if dot == at + 1 {
		return None;
	}
	let mut p_pos = 0;
	let length = text.len();
	for i in at + 1..length {
		let ch = text[i];
		if ch == u32::from('-') || is_alphanumeric(ch) {
			continue;
		}
		if ch != u32::from('.') || i == p_pos + 1 {
			let host_end = if i == p_pos + 1 { i.saturating_sub(2) } else { i - 1 };
			if p_pos > 0 && host_end.saturating_sub(at) >= 3 {
				text.truncate(host_end + 1);
				break;
			}
			return None;
		}
		p_pos = i;
	}
	let address = decode(&text);
	Some(if address.contains("mailto:") { address } else { format!("mailto:{address}") })
}

#[cfg(test)]
mod tests {
	use super::{LinkChar, extract};

	fn page(text: &str) -> (Vec<LinkChar>, Vec<u32>) {
		let unicode: Vec<u32> = text.chars().map(u32::from).collect();
		let chars =
			unicode.iter().map(|&c| LinkChar { generated: false, hyphen: false, normal: true, unicode: c }).collect();
		(chars, unicode)
	}

	#[test]
	fn a_url_in_running_text_is_found_without_its_full_stop() {
		let (chars, text) = page("Visit https://paperback.dev/docs. Then read on");
		assert_eq!(extract(&chars, &text), vec![(6, 26, "https://paperback.dev/docs".to_string())]);
	}

	#[test]
	fn a_bare_www_address_gains_a_scheme() {
		let (chars, text) = page("see www.example.org today");
		assert_eq!(extract(&chars, &text), vec![(4, 15, "http://www.example.org".to_string())]);
	}

	#[test]
	fn an_email_address_becomes_a_mailto_link() {
		let (chars, text) = page("write to someone@example.com now");
		assert_eq!(extract(&chars, &text), vec![(9, 19, "mailto:someone@example.com".to_string())]);
	}
}
