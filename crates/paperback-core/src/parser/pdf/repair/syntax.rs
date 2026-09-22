//! Reading PDF syntax out of raw file bytes, far enough to find the objects
//! [`super::rebuild`] has to change and no further.
//!
//! This is not a PDF parser and must not grow into one. The repair needs a handful of questions
//! answered (what does this key point at, where does this object end, where is the newest
//! trailer) over a file it never rewrites in place, so each answer is a scan for a token rather
//! than a parse into a model. A key is only ever found where it stands as a whole token, which is
//! what keeps `/Page` out of `/Pages` and `stream` out of `endstream`.

/// Everything from the file's last `trailer` keyword on, which is where its newest trailer
/// dictionary sits.
pub(super) fn last_trailer(bytes: &[u8]) -> Option<&[u8]> {
	find_last(bytes, b"trailer").map(|keyword| &bytes[keyword..])
}

/// The offset the file's last `startxref` names.
pub(super) fn last_startxref(bytes: &[u8]) -> Option<u64> {
	let mut at = find_last(bytes, b"startxref")? + b"startxref".len();
	let value = read_integer(bytes, &mut at)?;
	u64::try_from(value).ok()
}

/// The name a key is set to, as in the `Page` of `/Type /Page`.
pub(super) fn name_value<'a>(body: &'a [u8], key: &[u8]) -> Option<&'a [u8]> {
	let mut at = find_token(body, key)? + key.len();
	while at < body.len() && is_whitespace(body[at]) {
		at += 1;
	}
	if body.get(at) != Some(&b'/') {
		return None;
	}
	at += 1;
	let start = at;
	while at < body.len() && !is_delimiter(Some(body[at])) {
		at += 1;
	}
	Some(&body[start..at])
}

/// The object number a key refers to, as in the `4` of `/Pg 4 0 R`.
pub(super) fn ref_value(body: &[u8], key: &[u8]) -> Option<u32> {
	let mut at = find_token(body, key)? + key.len();
	let number = read_integer(body, &mut at)?;
	read_integer(body, &mut at)?;
	while at < body.len() && is_whitespace(body[at]) {
		at += 1;
	}
	if body.get(at) != Some(&b'R') {
		return None;
	}
	u32::try_from(number).ok()
}

/// The number a key is set to, as in the `0` of `/StructParents 0`.
pub(super) fn int_value(body: &[u8], key: &[u8]) -> Option<i64> {
	let mut at = find_token(body, key)? + key.len();
	read_integer(body, &mut at)
}

pub(super) fn read_integer(body: &[u8], at: &mut usize) -> Option<i64> {
	while *at < body.len() && is_whitespace(body[*at]) {
		*at += 1;
	}
	let start = *at;
	while *at < body.len() && body[*at].is_ascii_digit() {
		*at += 1;
	}
	if start == *at {
		return None;
	}
	std::str::from_utf8(&body[start..*at]).ok()?.parse().ok()
}

/// Finds a key only where it stands as a whole token, so `/Page` never matches inside `/Pages`
/// and `stream` never matches inside `endstream`.
pub(super) fn find_token(body: &[u8], key: &[u8]) -> Option<usize> {
	find_token_from(body, key, 0)
}

pub(super) fn find_token_from(body: &[u8], key: &[u8], from: usize) -> Option<usize> {
	let mut at = from;
	while let Some(found) = find_from(body, key, at) {
		at = found + 1;
		let after = is_delimiter(body.get(found + key.len()).copied());
		let before = found == 0 || is_delimiter(Some(body[found - 1]));
		if before && after {
			return Some(found);
		}
	}
	None
}

pub(super) fn find_from(haystack: &[u8], needle: &[u8], from: usize) -> Option<usize> {
	if from >= haystack.len() || needle.len() > haystack.len() - from {
		return None;
	}
	haystack[from..].windows(needle.len()).position(|window| window == needle).map(|found| found + from)
}

pub(super) fn find_last(haystack: &[u8], needle: &[u8]) -> Option<usize> {
	if needle.len() > haystack.len() {
		return None;
	}
	haystack.windows(needle.len()).rposition(|window| window == needle)
}

pub(super) fn skip_whitespace_back(bytes: &[u8], mut at: usize) -> usize {
	while at > 0 && is_whitespace(bytes[at - 1]) {
		at -= 1;
	}
	at
}

pub(super) fn skip_digits_back(bytes: &[u8], mut at: usize) -> usize {
	while at > 0 && bytes[at - 1].is_ascii_digit() {
		at -= 1;
	}
	at
}

pub(super) const fn is_whitespace(byte: u8) -> bool {
	matches!(byte, b' ' | b'\t' | b'\r' | b'\n' | b'\x0c' | b'\0')
}

/// True where a name or keyword ends: PDF's whitespace and delimiter characters, and the end of
/// the buffer.
pub(super) const fn is_delimiter(byte: Option<u8>) -> bool {
	match byte {
		None => true,
		Some(byte) => {
			is_whitespace(byte) || matches!(byte, b'(' | b')' | b'<' | b'>' | b'[' | b']' | b'{' | b'}' | b'/' | b'%')
		}
	}
}
