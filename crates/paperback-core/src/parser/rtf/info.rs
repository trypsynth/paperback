use crate::util::text::collapse_whitespace;

/// Reads RTF that has been through `normalize_escapes`, so codepage escapes are characters already and `\uN` escapes stand on their own.
pub(super) fn title_and_author(rtf: &str) -> (Option<String>, Option<String>) {
	let Some(info) = group(rtf, "info") else { return (None, None) };
	let field = |name| {
		group(info, name)
			.map(|text| collapse_whitespace(&decode(text)).trim().to_string())
			.filter(|text| !text.is_empty())
	};
	(field("title"), field("author"))
}

fn group<'a>(rtf: &'a str, name: &str) -> Option<&'a str> {
	let opening = format!("{{\\{name}");
	let mut search = 0;
	let start = loop {
		let found = search + rtf[search..].find(&opening)?;
		let after = found + opening.len();
		// `{\titlepg` is a different control word that happens to begin the same way.
		if !rtf[after..].starts_with(|c: char| c.is_ascii_alphabetic()) {
			break after;
		}
		search = after;
	};
	let mut depth = 1;
	let mut escaped = false;
	for (offset, ch) in rtf[start..].char_indices() {
		match ch {
			_ if escaped => escaped = false,
			'\\' => escaped = true,
			'{' => depth += 1,
			'}' => {
				depth -= 1;
				if depth == 0 {
					return Some(&rtf[start..start + offset]);
				}
			}
			_ => {}
		}
	}
	None
}

fn decode(group: &str) -> String {
	let mut text = String::new();
	let mut chars = group.chars().peekable();
	let mut high_surrogate = None;
	while let Some(ch) = chars.next() {
		match ch {
			'{' | '}' | '\n' => {}
			'\\' => match chars.peek().copied() {
				Some(escaped @ ('\\' | '{' | '}')) => {
					chars.next();
					text.push(escaped);
				}
				// The braces and backslash `normalize_escapes` leaves as hex, so the lexer still sees them as escaped.
				Some('\'') => {
					chars.next();
					let hex: String = [chars.next(), chars.next()].into_iter().flatten().collect();
					text.extend(u8::from_str_radix(&hex, 16).ok().map(char::from));
				}
				Some(letter) if letter.is_ascii_alphabetic() => {
					let mut word = String::new();
					while let Some(letter) = chars.next_if(char::is_ascii_alphabetic) {
						word.push(letter);
					}
					let mut number = String::new();
					while let Some(digit) = chars.next_if(|c| c.is_ascii_digit() || (*c == '-' && number.is_empty())) {
						number.push(digit);
					}
					chars.next_if_eq(&' ');
					if word == "u"
						&& let Ok(code) = number.parse::<i32>()
					{
						let unit = u16::try_from(if code < 0 { code + 0x10000 } else { code }).unwrap_or(0);
						match unit {
							0xD800..=0xDBFF => high_surrogate = Some(unit),
							0xDC00..=0xDFFF => {
								if let Some(high) = high_surrogate.take() {
									text.extend(char::decode_utf16([high, unit]).flatten());
								}
							}
							_ => text.extend(char::from_u32(u32::from(unit))),
						}
					}
				}
				_ => {}
			},
			other => text.push(other),
		}
	}
	text
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn the_info_group_gives_the_title_and_author() {
		let rtf = r"{\rtf1\ansi{\fonttbl{\f0 Arial;}}{\info{\title Annual Report}{\author Ann Smith}{\creatim\yr2026}}\pard Body\par}";
		assert_eq!(title_and_author(rtf), (Some("Annual Report".into()), Some("Ann Smith".into())));
	}

	#[test]
	fn unicode_escapes_and_escaped_braces_are_read() {
		let rtf = r"{\rtf1{\info{\title Caf\u233 \{draft\} \'7bv2\'7d}{\author \u-10179 \u-8704 }}}";
		assert_eq!(title_and_author(rtf), (Some("Caf\u{e9}{draft} {v2}".into()), Some("\u{1F600}".into())));
	}

	#[test]
	fn a_document_without_one_has_none() {
		assert_eq!(title_and_author(r"{\rtf1{\info{\title  }}\titlepg Body}"), (None, None));
		assert_eq!(title_and_author(r"{\rtf1 Body}"), (None, None));
	}
}
