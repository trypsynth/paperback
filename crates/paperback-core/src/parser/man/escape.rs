/// Roff sets one font at a time and `\fP` means whatever was in use before, so a renderer keeps the previous font rather than a stack.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Font {
	Roman,
	Bold,
	Italic,
	Previous,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum Piece {
	Text(String),
	Font(Font),
}

/// Only the ones manual pages use; anything else is dropped, which loses a glyph rather than reading its roff name out loud.
fn special_character(name: &str) -> Option<&'static str> {
	Some(match name {
		"aq" | "cq" | "oq" => "'",
		"dq" => "\"",
		"lq" => "\u{201C}",
		"rq" => "\u{201D}",
		"em" => "\u{2014}",
		"en" => "\u{2013}",
		"hy" | "mi" => "-",
		"bu" => "\u{2022}",
		"co" => "\u{00A9}",
		"rg" | "R" => "\u{00AE}",
		"tm" => "\u{2122}",
		"de" => "\u{00B0}",
		"mu" => "\u{00D7}",
		"di" => "\u{00F7}",
		"pl" => "+",
		"eq" => "=",
		"la" => "<",
		"ra" => ">",
		"<=" => "\u{2264}",
		">=" => "\u{2265}",
		"!=" => "\u{2260}",
		"->" => "\u{2192}",
		"<-" => "\u{2190}",
		"ti" => "~",
		"ha" => "^",
		"ga" => "`",
		"sh" => "#",
		"at" => "@",
		"or" | "ba" | "bv" | "br" => "|",
		"ul" => "_",
		"rs" => "\\",
		"lB" => "[",
		"rB" => "]",
		"lC" => "{",
		"rC" => "}",
		"nb" | "nc" => "",
		_ => return None,
	})
}

fn font_named(name: &str) -> Font {
	match name {
		"B" | "BI" | "CB" | "bold" | "bolditalic" => Font::Bold,
		"I" | "CI" | "italic" => Font::Italic,
		"P" | "" => Font::Previous,
		_ => Font::Roman,
	}
}

fn escape_argument(rest: &[char]) -> (String, usize) {
	match rest.first() {
		Some('(') => (rest.iter().skip(1).take(2).collect(), 3.min(rest.len())),
		Some('[') => {
			let end = rest.iter().position(|&c| c == ']').unwrap_or(rest.len());
			(rest[1..end.min(rest.len())].iter().collect(), (end + 1).min(rest.len()))
		}
		Some(&c) => (c.to_string(), 1),
		None => (String::new(), 0),
	}
}

fn size_change_len(rest: &[char]) -> usize {
	let mut used = 0;
	if matches!(rest.first(), Some('+' | '-')) {
		used += 1;
	}
	if let Some('(' | '[') = rest.get(used) {
		used + escape_argument(&rest[used..]).1
	} else {
		while rest.get(used).is_some_and(char::is_ascii_digit) {
			used += 1;
		}
		used
	}
}

fn quoted_argument_len(rest: &[char]) -> usize {
	let Some(&delimiter) = rest.first() else { return 0 };
	rest.iter().skip(1).position(|&c| c == delimiter).map_or(rest.len(), |end| end + 2)
}

pub(super) fn pieces(line: &str) -> Vec<Piece> {
	let chars: Vec<char> = line.chars().collect();
	let mut out: Vec<Piece> = Vec::new();
	let mut text = String::new();
	let mut index = 0;
	while index < chars.len() {
		let c = chars[index];
		if c != '\\' {
			text.push(c);
			index += 1;
			continue;
		}
		index += 1;
		let Some(&escape) = chars.get(index) else {
			// A trailing backslash continues the line, and the caller has joined it already.
			break;
		};
		index += 1;
		match escape {
			'f' => {
				let (name, used) = escape_argument(&chars[index..]);
				index += used;
				if !text.is_empty() {
					out.push(Piece::Text(std::mem::take(&mut text)));
				}
				out.push(Piece::Font(font_named(&name)));
			}
			'(' | '[' => {
				let (name, used) = escape_argument(&chars[index - 1..]);
				index += used - 1;
				if let Some(replacement) = special_character(&name) {
					text.push_str(replacement);
				}
			}
			'*' => {
				// The predefined strings a manual page uses stand for quotation marks and the registered sign, which the special characters already name.
				let (name, used) = escape_argument(&chars[index..]);
				index += used;
				if let Some(replacement) = special_character(&name) {
					text.push_str(replacement);
				}
			}
			's' => index += size_change_len(&chars[index..]),
			'h' | 'v' | 'w' | 'l' | 'L' | 'b' | 'o' | 'x' | 'X' | 'A' | 'Z' => {
				index += quoted_argument_len(&chars[index..]);
			}
			'n' => index += escape_argument(&chars[index..]).1,
			'-' => text.push('-'),
			'e' | '\\' => text.push('\\'),
			' ' | '~' | '0' | '^' | '|' => text.push(' '),
			'.' => text.push('.'),
			// The rest of the line is a comment.
			'"' => break,
			// Zero-width and formatting-only escapes, which leave nothing behind.
			'&' | '%' | 'c' | 'z' | 'k' | 'd' | 'u' | 'r' | '{' | '}' | 't' | 'p' | 'a' => {}
			other => text.push(other),
		}
	}
	if !text.is_empty() {
		out.push(Piece::Text(text));
	}
	out
}

pub(super) fn plain(line: &str) -> String {
	plain_pieces(&pieces(line))
}

pub(super) fn plain_pieces(pieces: &[Piece]) -> String {
	pieces
		.iter()
		.filter_map(|piece| match piece {
			Piece::Text(text) => Some(text.as_str()),
			Piece::Font(_) => None,
		})
		.collect()
}
