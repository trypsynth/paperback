//! [`DocumentBuffer`]: the assembled document text plus its markers and the per-char index
//! tables ([`char_to_byte_map`](DocumentBuffer), `display_len_at_char`) that translate between
//! byte, char, and display-unit offsets, the three coordinate systems a caret position can be
//! expressed in (see the fields' own docs for why all three are needed).

use rayon::prelude::*;

use super::marker::Marker;
use crate::util::text::{ch_width, display_len};

#[derive(Debug, Clone)]
pub struct DocumentBuffer {
	pub content: String,
	pub markers: Vec<Marker>,
	content_display_len: usize,
	content_char_count: usize,
	newline_char_positions: Vec<usize>,
	/// Byte offset of the `i`-th char, stored as `u32` rather than `usize`. This halves the
	/// table's footprint, which matters because it has one entry per char in the whole
	/// document (a large book can have hundreds of millions of chars). A document whose byte
	/// length exceeds `u32::MAX` (4 GiB of text) isn't supported; see `to_u32`.
	char_to_byte_map: Vec<u32>,
	/// `display_len_at_char[i]` is the display-unit offset (UTF-16 code units on
	/// Windows/macOS, Unicode scalars on GTK, see `util::text::display_len`) of the `i`-th
	/// char, with a trailing end-boundary entry equal to `content_display_len`, mirroring
	/// `char_to_byte_map`'s shape. Needed because `Marker.position`/the GUI caret use display
	/// units while `char_to_byte_map`/`newline_char_positions` use char units. On Windows and
	/// macOS these two diverge for any character outside the Basic Multilingual Plane. Stored
	/// as `u32` for the same reason as `char_to_byte_map`; display length never exceeds byte
	/// length so the same bound applies.
	display_len_at_char: Vec<u32>,
}

/// Narrows a byte/display offset to `u32` for storage in `DocumentBuffer`'s per-char index
/// tables. Debug-only bounds check: documents are read fully into memory elsewhere (e.g. the
/// EPUB parser) long before reaching 4 GiB, so this is unreachable in practice, and checking it
/// on every char in release builds would cost real time on large books.
#[inline]
#[allow(clippy::cast_possible_truncation)]
fn to_u32(value: usize) -> u32 {
	debug_assert!(u32::try_from(value).is_ok(), "DocumentBuffer offset exceeds u32::MAX");
	value as u32
}

/// A part's `[start, end)` span in the assembled buffer's display units (the same units as
/// `Marker::position` and `DocumentBuffer::current_position`), as returned by
/// [`DocumentBuffer::from_parts`].
#[derive(Debug, Clone, Copy)]
pub struct PartSpan {
	pub start: usize,
	pub end: usize,
}

/// The per-char indexing for one [`DocumentBuffer::from_parts`] part, computed independently of
/// every other part so it can run on a rayon worker in parallel. All offsets are local to the
/// part's own text (byte 0 / char 0 / display 0 is the part's first char), not including the
/// optional trailing newline `from_parts` adds when placing the part into the buffer.
struct PartIndex {
	/// `char_to_byte[i]` is the local byte offset of the part's `i`-th char.
	char_to_byte: Vec<u32>,
	/// `display_len_at_char[i]` is the local display-unit offset of the part's `i`-th char.
	display_len_at_char: Vec<u32>,
	/// Local char indices of every `\n` in the part's own text.
	newline_chars: Vec<usize>,
	byte_len: usize,
	char_len: usize,
	display_len: usize,
	/// Whether `from_parts` needs to add a `\n` after this part. Mirrors `append`'s callers that
	/// append a separator when the just-appended text didn't already end with one.
	trailing_newline: bool,
}

fn index_part(text: &str) -> PartIndex {
	let mut char_to_byte = Vec::with_capacity(text.len());
	let mut display_len_at_char = Vec::with_capacity(text.len());
	let mut newline_chars = Vec::new();
	let mut char_len = 0usize;
	let mut display_len = 0usize;
	for (byte_idx, c) in text.char_indices() {
		char_to_byte.push(to_u32(byte_idx));
		display_len_at_char.push(to_u32(display_len));
		if c == '\n' {
			newline_chars.push(char_len);
		}
		char_len += 1;
		display_len += ch_width(c);
	}
	let trailing_newline = !text.is_empty() && !text.ends_with('\n');
	PartIndex {
		char_to_byte,
		display_len_at_char,
		newline_chars,
		byte_len: text.len(),
		char_len,
		display_len,
		trailing_newline,
	}
}

/// Splits `slice` into consecutive, disjoint sub-slices with the given lengths (which must sum to
/// `slice.len()`), so each can be handed to a different rayon task and written into independently.
fn split_mut_slices<'a, T>(mut slice: &'a mut [T], lens: &[usize]) -> Vec<&'a mut [T]> {
	let mut out = Vec::with_capacity(lens.len());
	for &len in lens {
		let (head, tail) = slice.split_at_mut(len);
		out.push(head);
		slice = tail;
	}
	out
}

impl DocumentBuffer {
	#[must_use]
	pub const fn new() -> Self {
		Self {
			content: String::new(),
			markers: Vec::new(),
			content_display_len: 0,
			content_char_count: 0,
			newline_char_positions: Vec::new(),
			char_to_byte_map: Vec::new(),
			display_len_at_char: Vec::new(),
		}
	}

	#[must_use]
	pub fn with_content(content: String) -> Self {
		let mut char_count = 0usize;
		let mut display_count = 0usize;
		let mut newline_char_positions = Vec::new();
		let mut char_to_byte_map = Vec::with_capacity(content.len().min(1024));
		let mut display_len_at_char = Vec::with_capacity(content.len().min(1024));
		for (byte_idx, c) in content.char_indices() {
			char_to_byte_map.push(to_u32(byte_idx));
			display_len_at_char.push(to_u32(display_count));
			if c == '\n' {
				newline_char_positions.push(char_count);
			}
			char_count += 1;
			display_count += ch_width(c);
		}
		char_to_byte_map.push(to_u32(content.len())); // append end boundary
		display_len_at_char.push(to_u32(display_count));
		debug_assert_eq!(display_count, display_len(&content));
		Self {
			content,
			markers: Vec::new(),
			content_display_len: display_count,
			content_char_count: char_count,
			newline_char_positions,
			char_to_byte_map,
			display_len_at_char,
		}
	}

	pub fn add_marker(&mut self, marker: Marker) {
		self.markers.push(marker);
	}

	pub fn append(&mut self, text: &str) {
		let base = self.content_char_count;
		let mut count = 0usize;
		let mut display_count = self.content_display_len;
		// Remove the end boundary temporarily
		if !self.char_to_byte_map.is_empty() {
			self.char_to_byte_map.pop();
		}
		if !self.display_len_at_char.is_empty() {
			self.display_len_at_char.pop();
		}
		let start_byte = self.content.len();
		for (byte_idx, c) in text.char_indices() {
			self.char_to_byte_map.push(to_u32(start_byte + byte_idx));
			self.display_len_at_char.push(to_u32(display_count));
			if c == '\n' {
				self.newline_char_positions.push(base + count);
			}
			count += 1;
			display_count += ch_width(c);
		}
		self.content.push_str(text);
		self.char_to_byte_map.push(to_u32(self.content.len())); // append end boundary back
		self.display_len_at_char.push(to_u32(display_count));
		self.content_display_len = display_count;
		self.content_char_count += count;
	}

	/// Builds a buffer from independent parts (e.g. one EPUB spine item's converted text each) in
	/// parallel across cores, equivalent to calling [`Self::append`] with each part in order (each
	/// non-empty part gets a trailing `\n` if it doesn't already end with one, exactly as `append`
	/// callers that need a separator do today). Returns the buffer alongside each part's
	/// display-unit `[start, end)` span, so callers that need per-part offsets (to place markers,
	/// resolve id positions, etc., the reason `append` couldn't already be called in parallel, since
	/// those offsets come from the buffer's running position) don't need to re-derive them.
	///
	/// This trades `append`'s single incremental pass over the whole document for: a parallel pass
	/// indexing each part's own chars, a cheap sequential prefix-sum over parts (not chars) to place
	/// them, and a parallel pass filling preallocated buffers at those positions.
	#[must_use]
	pub fn from_parts(parts: Vec<String>) -> (Self, Vec<PartSpan>) {
		let indexed: Vec<(String, PartIndex)> = parts
			.into_par_iter()
			.map(|text| {
				let idx = index_part(&text);
				(text, idx)
			})
			.collect();
		struct PartStart {
			byte: usize,
			char: usize,
			display: usize,
		}
		let mut starts = Vec::with_capacity(indexed.len());
		let mut spans = Vec::with_capacity(indexed.len());
		let mut byte_acc = 0usize;
		let mut char_acc = 0usize;
		let mut display_acc = 0usize;
		for (_, idx) in &indexed {
			let extra = usize::from(idx.trailing_newline);
			starts.push(PartStart { byte: byte_acc, char: char_acc, display: display_acc });
			spans.push(PartSpan { start: display_acc, end: display_acc + idx.display_len + extra });
			byte_acc += idx.byte_len + extra;
			char_acc += idx.char_len + extra;
			display_acc += idx.display_len + extra; // ch_width('\n') == 1 on every platform
		}
		if char_acc == 0 {
			// No part contributed any text, so no `append` equivalent ever ran; match `new()`'s
			// all-empty (not one-boundary-entry) shape rather than building degenerate arrays.
			return (Self::new(), spans);
		}
		let mut content_bytes = vec![0u8; byte_acc];
		let mut char_to_byte_map = vec![0u32; char_acc + 1];
		let mut display_len_at_char = vec![0u32; char_acc + 1];
		let (char_to_byte_main, char_to_byte_boundary) = char_to_byte_map.split_at_mut(char_acc);
		let (display_main, display_boundary) = display_len_at_char.split_at_mut(char_acc);
		char_to_byte_boundary[0] = to_u32(byte_acc);
		display_boundary[0] = to_u32(display_acc);
		let byte_lens: Vec<usize> =
			indexed.iter().map(|(_, idx)| idx.byte_len + usize::from(idx.trailing_newline)).collect();
		let char_lens: Vec<usize> =
			indexed.iter().map(|(_, idx)| idx.char_len + usize::from(idx.trailing_newline)).collect();
		let content_slices = split_mut_slices(content_bytes.as_mut_slice(), &byte_lens);
		let char_to_byte_slices = split_mut_slices(char_to_byte_main, &char_lens);
		let display_slices = split_mut_slices(display_main, &char_lens);
		let mut newline_char_positions = Vec::new();
		for ((_, idx), start) in indexed.iter().zip(&starts) {
			for &pos in &idx.newline_chars {
				newline_char_positions.push(start.char + pos);
			}
			if idx.trailing_newline {
				newline_char_positions.push(start.char + idx.char_len);
			}
		}
		struct PartWork<'a> {
			text: &'a str,
			idx: &'a PartIndex,
			start_byte: usize,
			start_display: usize,
			content: &'a mut [u8],
			char_to_byte: &'a mut [u32],
			display: &'a mut [u32],
		}
		let work: Vec<PartWork<'_>> = indexed
			.iter()
			.zip(&starts)
			.zip(content_slices)
			.zip(char_to_byte_slices)
			.zip(display_slices)
			.map(|((((entry, start), content), char_to_byte), display)| {
				let (text, idx) = entry;
				PartWork {
					text,
					idx,
					start_byte: start.byte,
					start_display: start.display,
					content,
					char_to_byte,
					display,
				}
			})
			.collect();
		work.into_par_iter().for_each(|w| {
			w.content[..w.idx.byte_len].copy_from_slice(w.text.as_bytes());
			if w.idx.trailing_newline {
				w.content[w.idx.byte_len] = b'\n';
			}
			for i in 0..w.idx.char_len {
				w.char_to_byte[i] = to_u32(w.start_byte + w.idx.char_to_byte[i] as usize);
				w.display[i] = to_u32(w.start_display + w.idx.display_len_at_char[i] as usize);
			}
			if w.idx.trailing_newline {
				w.char_to_byte[w.idx.char_len] = to_u32(w.start_byte + w.idx.byte_len);
				w.display[w.idx.char_len] = to_u32(w.start_display + w.idx.display_len);
			}
		});
		let content = String::from_utf8(content_bytes)
			.expect("each part is a valid &str and concatenating valid UTF-8 stays valid UTF-8");
		(
			Self {
				content,
				markers: Vec::new(),
				content_display_len: display_acc,
				content_char_count: char_acc,
				newline_char_positions,
				char_to_byte_map,
				display_len_at_char,
			},
			spans,
		)
	}

	#[must_use]
	pub fn byte_index_for_char(&self, char_index: usize) -> usize {
		self.char_to_byte_map.get(char_index).map_or(self.content.len(), |&v| v as usize)
	}

	#[must_use]
	pub fn char_index_for_byte(&self, byte_index: usize) -> usize {
		self.char_to_byte_map.binary_search(&to_u32(byte_index)).unwrap_or_else(|idx| idx)
	}

	/// The display-unit offset of the `char_index`-th char (or the document's total display
	/// length, for a char index at or past the end).
	#[must_use]
	pub fn display_index_for_char(&self, char_index: usize) -> usize {
		self.display_len_at_char.get(char_index).map_or(self.content_display_len, |&v| v as usize)
	}

	/// The char index whose char covers `display_index`, or the nearest one before it if
	/// `display_index` lands inside a surrogate pair (shouldn't happen for a well-formed
	/// caller, but this stays well-defined rather than panicking).
	#[must_use]
	pub fn char_index_for_display(&self, display_index: usize) -> usize {
		self.display_len_at_char.binary_search(&to_u32(display_index)).unwrap_or_else(|idx| idx)
	}

	/// The byte offset corresponding to `display_index`, composing [`Self::char_index_for_display`]
	/// with [`Self::byte_index_for_char`].
	#[must_use]
	pub fn byte_index_for_display(&self, display_index: usize) -> usize {
		self.byte_index_for_char(self.char_index_for_display(display_index))
	}

	/// The display-unit offset corresponding to `byte_index`, composing
	/// [`Self::char_index_for_byte`] with [`Self::display_index_for_char`].
	#[must_use]
	pub fn display_index_for_byte(&self, byte_index: usize) -> usize {
		self.display_index_for_char(self.char_index_for_byte(byte_index))
	}

	#[must_use]
	pub const fn current_position(&self) -> usize {
		self.content_display_len
	}

	/// Total document length in display units (UTF-16 code units on Windows/macOS, Unicode
	/// scalars on GTK), the same unit the GUI caret and `Marker.position` use. An alias of
	/// [`Self::current_position`] under the name callers building a window/range API actually
	/// want; `current_position` is kept for the parser-cursor callers already using that name.
	#[must_use]
	pub const fn total_display_len(&self) -> usize {
		self.content_display_len
	}

	#[must_use]
	pub const fn char_count(&self) -> usize {
		self.content_char_count
	}

	#[must_use]
	pub fn newline_positions(&self) -> &[usize] {
		&self.newline_char_positions
	}
}

impl Default for DocumentBuffer {
	fn default() -> Self {
		Self::new()
	}
}

#[cfg(test)]
mod tests {
	use rstest::rstest;

	use super::*;

	#[test]
	fn document_buffer_append_updates_position() {
		let mut buffer = DocumentBuffer::new();
		assert_eq!(buffer.current_position(), 0);
		buffer.append("abc");
		buffer.append("de");
		assert_eq!(buffer.current_position(), 5);
	}

	// `ch_width`/`display_len` return UTF-16 code-unit counts on Windows/macOS and Unicode
	// scalar counts on GTK, so a BMP-only string ("abc") behaves identically on every
	// platform and is the right fixture for testing the display<->char<->byte bridge itself
	// without also depending on which platform the test runs on.
	#[test]
	fn display_char_byte_indices_agree_for_bmp_only_content() {
		let buffer = DocumentBuffer::with_content("abc".to_string());
		for i in 0..=3 {
			assert_eq!(buffer.display_index_for_char(i), i);
			assert_eq!(buffer.char_index_for_display(i), i);
			assert_eq!(buffer.byte_index_for_display(i), i);
			assert_eq!(buffer.display_index_for_byte(i), i);
		}
		assert_eq!(buffer.total_display_len(), 3);
	}

	#[cfg(any(windows, target_os = "macos"))]
	#[test]
	fn display_index_accounts_for_surrogate_pairs_on_windows_and_macos() {
		// U+1F600 (an astral-plane emoji) is 1 char, 4 UTF-8 bytes, but 2 UTF-16 code units.
		let buffer = DocumentBuffer::with_content("a\u{1F600}b".to_string());
		// chars: 'a' (char 0), emoji (char 1), 'b' (char 2), end boundary (char 3)
		assert_eq!(buffer.display_index_for_char(0), 0); // 'a' starts at display 0
		assert_eq!(buffer.display_index_for_char(1), 1); // emoji starts at display 1
		assert_eq!(buffer.display_index_for_char(2), 3); // 'b' starts at display 3 (emoji took 2 units)
		assert_eq!(buffer.total_display_len(), 4);
		assert_eq!(buffer.char_index_for_display(0), 0);
		assert_eq!(buffer.char_index_for_display(1), 1);
		assert_eq!(buffer.char_index_for_display(3), 2);
		// byte<->display composition round-trips through the char index for every char start.
		assert_eq!(buffer.byte_index_for_display(0), 0);
		assert_eq!(buffer.byte_index_for_display(1), 1);
		assert_eq!(buffer.byte_index_for_display(3), 5); // 1 byte 'a' + 4 byte emoji
		assert_eq!(buffer.display_index_for_byte(0), 0);
		assert_eq!(buffer.display_index_for_byte(1), 1);
		assert_eq!(buffer.display_index_for_byte(5), 3);
	}

	#[test]
	fn display_index_for_char_clamps_past_the_end() {
		let buffer = DocumentBuffer::with_content("abc".to_string());
		assert_eq!(buffer.display_index_for_char(999), buffer.total_display_len());
	}

	#[test]
	fn append_extends_the_display_index_incrementally() {
		let mut buffer = DocumentBuffer::new();
		buffer.append("ab");
		buffer.append("cd");
		assert_eq!(buffer.total_display_len(), 4);
		for i in 0..=4 {
			assert_eq!(buffer.display_index_for_char(i), i, "char {i}");
		}
	}

	/// Mirrors how `epub.rs::convert_spine_items` used to build a buffer one section at a time
	/// with sequential `append` calls (a trailing `\n` added per non-empty part unless it already
	/// ends with one), so `from_parts_matches_sequential_append_for_any_parts` can check the new
	/// parallel path agrees with it exactly.
	fn naive_buffer_from_parts(parts: &[&str]) -> (DocumentBuffer, Vec<PartSpan>) {
		let mut buffer = DocumentBuffer::new();
		let mut spans = Vec::with_capacity(parts.len());
		for part in parts {
			let start = buffer.current_position();
			if !part.is_empty() {
				buffer.append(part);
				if !buffer.content.ends_with('\n') {
					buffer.append("\n");
				}
			}
			spans.push(PartSpan { start, end: buffer.current_position() });
		}
		(buffer, spans)
	}

	fn assert_buffers_equivalent(a: &DocumentBuffer, b: &DocumentBuffer) {
		assert_eq!(a.content, b.content);
		assert_eq!(a.char_count(), b.char_count());
		assert_eq!(a.total_display_len(), b.total_display_len());
		assert_eq!(a.newline_positions(), b.newline_positions());
		for i in 0..=a.char_count() {
			assert_eq!(a.byte_index_for_char(i), b.byte_index_for_char(i), "byte_index_for_char({i})");
			assert_eq!(a.display_index_for_char(i), b.display_index_for_char(i), "display_index_for_char({i})");
		}
		for byte in 0..=a.content.len() {
			assert_eq!(a.char_index_for_byte(byte), b.char_index_for_byte(byte), "char_index_for_byte({byte})");
		}
		for display in 0..=a.total_display_len() {
			assert_eq!(
				a.char_index_for_display(display),
				b.char_index_for_display(display),
				"char_index_for_display({display})"
			);
		}
	}

	#[rstest]
	#[case(&[])]
	#[case(&[""])]
	#[case(&["", "", ""])]
	#[case(&["a"])]
	#[case(&["a", "b", "c"])]
	#[case(&["no trailing newline", "another one", "and another"])]
	#[case(&["ends with newline\n", "also ends with newline\n"])]
	#[case(&["mixed\n", "no newline here", "\n", "final"])]
	#[case(&["", "empty parts", "", "sprinkled", "", "in", "", ""])]
	#[case(&["multi\nline\ntext\nhere", "and\nmore\nlines"])]
	#[case(&["münchhausen café naïve", "日本語 text here", "emoji 🎉🎉 straddling", "naïve again"])]
	#[case(&["🎉", "🎉🎉", "a🎉b", "\n🎉\n"])]
	fn from_parts_matches_sequential_append_for_any_parts(#[case] parts: &[&str]) {
		let owned: Vec<String> = parts.iter().map(ToString::to_string).collect();
		let (naive_buffer, naive_spans) = naive_buffer_from_parts(parts);
		let (parallel_buffer, parallel_spans) = DocumentBuffer::from_parts(owned);
		assert_buffers_equivalent(&naive_buffer, &parallel_buffer);
		assert_eq!(naive_spans.len(), parallel_spans.len());
		for (i, (naive, parallel)) in naive_spans.iter().zip(&parallel_spans).enumerate() {
			assert_eq!(naive.start, parallel.start, "part {i} start");
			assert_eq!(naive.end, parallel.end, "part {i} end");
		}
	}
}
