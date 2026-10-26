//! [`DocumentBuffer`]: the assembled document text plus its markers and the checkpoints that translate between positions ([`DocPos`], counted in chars) and byte offsets into the text ([`ContentBytePos`]).

use std::ops::{Range, RangeBounds};

use rayon::prelude::*;

use super::{
	marker::{Marker, MarkerType},
	position::{ContentBytePos, DocLen, DocPos},
};

/// How many chars apart the buffer notes where a char starts. Any other char is found by walking forward from the one noted before it, which decodes at most this many. Noting every char instead costs eight bytes a char, eight times the text itself for a book in English, which is what kept a text file of a gigabyte or so from opening in 8 GB of memory (#929).
const CHECKPOINT_CHARS: usize = 64;

#[derive(Debug, Clone)]
pub struct DocumentBuffer {
	pub content: String,
	pub markers: Vec<Marker>,
	char_count: usize,
	newlines: Vec<DocPos>,
	/// `checkpoints[i]` is the byte at which char `i * CHECKPOINT_CHARS` starts.
	checkpoints: Vec<usize>,
}

/// A part's `[start, end)` span in the assembled buffer, as returned by [`DocumentBuffer::from_parts`].
#[derive(Debug, Clone, Copy)]
pub struct PartSpan {
	pub start: DocPos,
	pub end: DocPos,
}

/// How much text one worker indexes at a time. A whole buffer is indexed across every core, since a text file of several gigabytes takes a single core longer than the reader should wait.
const INDEX_CHUNK_BYTES: usize = 1 << 20;

/// Notes `text`'s checkpoints and newlines as they fall in the buffer, where it starts at char `first_char` and byte `first_byte`, and returns its length in chars.
fn scan(
	text: &str,
	first_char: usize,
	first_byte: usize,
	mut checkpoint: impl FnMut(usize),
	mut newline: impl FnMut(DocPos),
) -> usize {
	let mut chars = 0usize;
	for (byte, c) in text.char_indices() {
		let index = first_char + chars;
		if index.is_multiple_of(CHECKPOINT_CHARS) {
			checkpoint(first_byte + byte);
		}
		if c == '\n' {
			newline(DocPos::from_raw(index));
		}
		chars += 1;
	}
	chars
}

/// How many checkpoints fall among the `chars` chars starting at char `first_char`.
const fn checkpoints_among(first_char: usize, chars: usize) -> usize {
	(first_char + chars).div_ceil(CHECKPOINT_CHARS) - first_char.div_ceil(CHECKPOINT_CHARS)
}

/// `text` cut into pieces of about `size` bytes, each ending on a char boundary.
fn chunks(text: &str, size: usize) -> Vec<&str> {
	let mut out = Vec::with_capacity(text.len() / size + 1);
	let mut rest = text;
	while rest.len() > size {
		let mut at = size;
		while !rest.is_char_boundary(at) {
			at += 1;
		}
		let (head, tail) = rest.split_at(at);
		out.push(head);
		rest = tail;
	}
	if !rest.is_empty() {
		out.push(rest);
	}
	out
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
			char_count: 0,
			newlines: Vec::new(),
			checkpoints: Vec::new(),
		}
	}

	#[must_use]
	pub fn with_content(content: String) -> Self {
		let mut buffer = Self { content, ..Self::new() };
		buffer.rebuild_indexes();
		buffer
	}

	/// Rebuilds the checkpoints and newline positions from `self.content`, which is the single
	/// source of truth for both. Called by [`Self::with_content`] and after any in-place content
	/// mutation (currently only [`Self::replace_ranges`]).
	///
	/// Done a chunk to a worker: one pass measures each chunk, which places them, and a second notes each chunk's checkpoints and newlines straight into where they go, so the two lists are never held twice.
	fn rebuild_indexes(&mut self) {
		self.rebuild_indexes_in(INDEX_CHUNK_BYTES);
	}

	/// [`Self::rebuild_indexes`], a chunk of about `chunk_bytes` to a worker.
	fn rebuild_indexes_in(&mut self, chunk_bytes: usize) {
		let chunks = chunks(&self.content, chunk_bytes);
		let measured: Vec<(usize, usize)> = chunks
			.par_iter()
			.map(|chunk| (chunk.chars().count(), memchr::memchr_iter(b'\n', chunk.as_bytes()).count()))
			.collect();
		let mut starts = Vec::with_capacity(chunks.len());
		let (mut byte, mut char, mut checkpoint_count, mut newline_count) = (0, 0, 0, 0);
		for (chunk, &(chars, newlines)) in chunks.iter().zip(&measured) {
			starts.push((char, byte));
			checkpoint_count += checkpoints_among(char, chars);
			newline_count += newlines;
			byte += chunk.len();
			char += chars;
		}
		let mut checkpoints = vec![0; checkpoint_count];
		let mut newlines = vec![DocPos::ZERO; newline_count];
		let checkpoint_lens: Vec<usize> =
			measured.iter().zip(&starts).map(|(&(chars, _), &(first, _))| checkpoints_among(first, chars)).collect();
		let newline_lens: Vec<usize> = measured.iter().map(|&(_, newlines)| newlines).collect();
		chunks
			.par_iter()
			.zip(&starts)
			.zip(split_mut_slices(&mut checkpoints, &checkpoint_lens))
			.zip(split_mut_slices(&mut newlines, &newline_lens))
			.for_each(|(((chunk, &(first_char, first_byte)), checkpoint_slots), newline_slots)| {
				let mut checkpoint_slots = checkpoint_slots.iter_mut();
				let mut newline_slots = newline_slots.iter_mut();
				scan(
					chunk,
					first_char,
					first_byte,
					|checkpoint| *checkpoint_slots.next().expect("counted when the chunk was measured") = checkpoint,
					|newline| *newline_slots.next().expect("counted when the chunk was measured") = newline,
				);
			});
		self.char_count = char;
		self.newlines = newlines;
		self.checkpoints = checkpoints;
	}

	/// Replaces the text spanning `range` with `replacement`, in place.
	/// A convenience wrapper over [`Self::replace_ranges`] for the single-edit case; see there
	/// for the marker rules. Returns the delta (`new_len - old_len`) in positions.
	pub fn replace_range(&mut self, range: Range<DocPos>, replacement: &str) -> i64 {
		self.replace_ranges(vec![Edit { start: range.start, end: range.end, text: replacement.to_string() }])
			.total_delta
	}

	/// Applies several replacements in one pass and rebuilds the checkpoints **once**.
	///
	/// Rebuilding is linear in the whole document, so applying edits one at a time is quadratic in
	/// the number of edits. Batch OCR of a long scan produces one edit per page, which is exactly
	/// the case that made this necessary.
	///
	/// `edits` may arrive in any order but must not overlap; they are sorted here and applied back
	/// to front so earlier byte offsets stay valid.
	///
	/// Marker rules, applied against the pre-edit positions: a marker strictly inside a replaced
	/// span is dropped, as is a [`MarkerType::ImageOnlyPage`] marker sitting exactly at a span's
	/// start (the placeholder it tags is precisely what the edit consumes). Every other marker
	/// survives and shifts by the total delta of the edits that end at or before it, so a page
	/// break at the start of a replaced line keeps its position.
	#[allow(clippy::cast_possible_wrap)]
	pub fn replace_ranges(&mut self, mut edits: Vec<Edit>) -> ReplaceOutcome {
		edits.sort_by_key(|edit| edit.start);
		debug_assert!(
			edits.windows(2).all(|pair| pair[0].end <= pair[1].start),
			"replace_ranges requires non-overlapping edits"
		);
		let mut deltas = Vec::with_capacity(edits.len());
		for edit in &edits {
			deltas.push(DocLen::of(&edit.text).get() as i64 - (edit.end - edit.start).get() as i64);
		}
		// Every edit's span is looked up before any is applied, since the checkpoints describe the content as it was.
		let spans: Vec<(usize, usize)> =
			edits.iter().map(|edit| (self.byte_at(edit.start).get(), self.byte_at(edit.end).get())).collect();
		for (edit, (start_byte, end_byte)) in edits.iter().zip(spans).rev() {
			self.content.replace_range(start_byte..end_byte, &edit.text);
		}
		self.rebuild_indexes();
		self.markers.retain_mut(|marker| shift_marker(marker, &edits, &deltas));
		let total_delta = deltas.iter().sum();
		// Running sums of the deltas, so callers can map a pre-edit position to its post-edit one
		// the same way the markers were mapped.
		let mut shifts = Vec::with_capacity(edits.len());
		let mut running = 0i64;
		for (edit, delta) in edits.iter().zip(&deltas) {
			running += delta;
			shifts.push((edit.end, running));
		}
		ReplaceOutcome { total_delta, shifts }
	}

	pub fn add_marker(&mut self, marker: Marker) {
		self.markers.push(marker);
	}

	pub fn append(&mut self, text: &str) {
		let chars = scan(
			text,
			self.char_count,
			self.content.len(),
			|checkpoint| self.checkpoints.push(checkpoint),
			|newline| self.newlines.push(newline),
		);
		self.content.push_str(text);
		self.char_count += chars;
	}

	/// Builds a buffer from independent parts (e.g. one EPUB spine item's converted text each) in
	/// parallel across cores, equivalent to calling [`Self::append`] with each part in order (each
	/// non-empty part gets a trailing `\n` if it doesn't already end with one, exactly as `append`
	/// callers that need a separator do today). Returns the buffer alongside each part's
	/// `[start, end)` span, so callers that need per-part offsets (to place markers,
	/// resolve id positions, etc., the reason `append` couldn't already be called in parallel, since
	/// those offsets come from the buffer's running position) don't need to re-derive them.
	///
	/// This trades `append`'s single incremental pass over the whole document for: a parallel pass
	/// measuring each part, a cheap sequential prefix-sum over parts (not chars) to place them, a
	/// parallel copy of each part into place, and the same parallel indexing any buffer gets.
	#[must_use]
	pub fn from_parts(parts: Vec<String>) -> (Self, Vec<PartSpan>) {
		// Each part's length in bytes and positions, and whether `from_parts` needs to add a `\n` after it, as `append`'s callers do when the text they appended didn't end with one.
		let measured: Vec<(usize, DocLen, bool)> = parts
			.par_iter()
			.map(|text| (text.len(), DocLen::of(text), !text.is_empty() && !text.ends_with('\n')))
			.collect();
		let mut spans = Vec::with_capacity(parts.len());
		let mut byte_lens = Vec::with_capacity(parts.len());
		let mut end = DocPos::ZERO;
		for &(byte_len, len, trailing_newline) in &measured {
			let extra = usize::from(trailing_newline);
			let start = end;
			end += len + DocLen::from_raw(extra);
			spans.push(PartSpan { start, end });
			byte_lens.push(byte_len + extra);
		}
		let mut content_bytes = vec![0u8; byte_lens.iter().sum()];
		parts.par_iter().zip(&measured).zip(split_mut_slices(content_bytes.as_mut_slice(), &byte_lens)).for_each(
			|((text, &(byte_len, _, trailing_newline)), content)| {
				content[..byte_len].copy_from_slice(text.as_bytes());
				if trailing_newline {
					content[byte_len] = b'\n';
				}
			},
		);
		drop(parts);
		let content = String::from_utf8(content_bytes)
			.expect("each part is a valid &str and concatenating valid UTF-8 stays valid UTF-8");
		(Self::with_content(content), spans)
	}

	/// The byte at which the char at `pos` starts, or the end of `content` for a position at or past the end.
	#[must_use]
	pub fn byte_at(&self, pos: DocPos) -> ContentBytePos {
		let index = pos.get();
		if index >= self.char_count {
			return self.content_end();
		}
		let from = self.checkpoints[index / CHECKPOINT_CHARS];
		let offset = self.content[from..].char_indices().nth(index % CHECKPOINT_CHARS).map_or(0, |(offset, _)| offset);
		ContentBytePos::from_raw(from + offset)
	}

	/// The position of the char that starts at `byte`. A `byte` inside a char rounds forward to the char after it, and anything past the end is the end.
	#[must_use]
	pub fn pos_at_byte(&self, byte: ContentBytePos) -> DocPos {
		let byte = byte.get();
		if byte >= self.content.len() {
			return self.end();
		}
		let at = self.checkpoints.partition_point(|&checkpoint| checkpoint <= byte).saturating_sub(1);
		let from = self.checkpoints[at];
		DocPos::from_raw(
			at * CHECKPOINT_CHARS
				+ self.content[from..].char_indices().take_while(|(offset, _)| from + offset < byte).count(),
		)
	}

	/// The text between two positions, which are clamped to the document. A range that runs backwards is empty.
	#[must_use]
	pub fn text(&self, range: Range<DocPos>) -> &str {
		let start = self.byte_at(range.start);
		self.slice(start..self.byte_at(range.end).max(start))
	}

	/// The position closest to `raw`, a value from outside the core, such as one passed through the FFI, or saved before a reparse made the document shorter.
	#[must_use]
	pub fn nearest_pos(&self, raw: i64) -> DocPos {
		DocPos::from_raw(usize::try_from(raw).unwrap_or(0)).min(self.end())
	}

	/// The byte offset one past the last byte of `content`.
	#[must_use]
	pub const fn content_end(&self) -> ContentBytePos {
		ContentBytePos::from_raw(self.content.len())
	}

	/// The text of `content` within `range`. A method rather than `Index` on the content,
	/// because `Index` can't be implemented on `str` for a range of a local type.
	#[must_use]
	pub fn slice(&self, range: impl RangeBounds<ContentBytePos>) -> &str {
		&self.content[(range.start_bound().map(|pos| pos.get()), range.end_bound().map(|pos| pos.get()))]
	}

	/// The position just past the last char, where the next appended text starts.
	#[must_use]
	pub const fn end(&self) -> DocPos {
		DocPos::from_raw(self.char_count)
	}

	/// How many positions the document takes up.
	#[must_use]
	pub const fn len(&self) -> DocLen {
		DocLen::from_raw(self.char_count)
	}

	#[must_use]
	pub const fn is_empty(&self) -> bool {
		self.char_count == 0
	}

	/// The position of every `\n`, in order.
	#[must_use]
	pub fn newline_positions(&self) -> &[DocPos] {
		&self.newlines
	}
}

impl Default for DocumentBuffer {
	fn default() -> Self {
		Self::new()
	}
}

/// One replacement for [`DocumentBuffer::replace_ranges`]: the span `start..end` and the text that takes its place.
#[derive(Debug, Clone)]
pub struct Edit {
	pub start: DocPos,
	pub end: DocPos,
	pub text: String,
}

/// What [`DocumentBuffer::replace_ranges`] changed: how many positions the document grew by, plus the
/// running shift after each edit so positions stored outside the buffer can be remapped.
#[derive(Debug, Clone)]
pub struct ReplaceOutcome {
	pub total_delta: i64,
	shifts: Vec<(DocPos, i64)>,
}

impl ReplaceOutcome {
	/// Maps a pre-edit position to its post-edit one. Positions inside a replaced span
	/// have no exact answer; they collapse to the shifted end of that span.
	#[must_use]
	pub fn shift(&self, position: DocPos) -> DocPos {
		let applied = self.shifts.partition_point(|(end, _)| *end <= position);
		if applied == 0 {
			return position;
		}
		shifted(position, self.shifts[applied - 1].1)
	}

	#[must_use]
	pub fn is_empty(&self) -> bool {
		self.shifts.is_empty()
	}
}

/// `position` moved by `delta` positions, stopping at the start of the document.
fn shifted(position: DocPos, delta: i64) -> DocPos {
	let distance = DocLen::from_raw(usize::try_from(delta.unsigned_abs()).unwrap_or(usize::MAX));
	if delta < 0 { position.saturating_sub(distance) } else { position + distance }
}

/// Repositions `marker` for a batch of edits, or reports that it should be dropped. See the
/// marker rules on [`DocumentBuffer::replace_ranges`].
fn shift_marker(marker: &mut Marker, edits: &[Edit], deltas: &[i64]) -> bool {
	let mut shift = 0i64;
	for (edit, delta) in edits.iter().zip(deltas) {
		if edit.end <= marker.position {
			shift += delta;
			continue;
		}
		if marker.position > edit.start {
			return false; // strictly inside the replaced span
		}
		if marker.position == edit.start && marker.mtype == MarkerType::ImageOnlyPage {
			return false; // the placeholder this marker tags is what the edit consumes
		}
		break; // edits are sorted, so nothing later can affect this marker
	}
	marker.position = shifted(marker.position, shift);
	true
}

#[cfg(test)]
mod tests {
	use rstest::rstest;

	use super::*;
	use crate::document::marker::{Marker, MarkerType};

	fn pos(raw: usize) -> DocPos {
		DocPos::from_raw(raw)
	}

	#[test]
	fn document_buffer_append_updates_position() {
		let mut buffer = DocumentBuffer::new();
		assert_eq!(buffer.end(), 0);
		buffer.append("abc");
		buffer.append("de");
		assert_eq!(buffer.end(), 5);
	}

	// An ASCII string, where positions and bytes line up, so what is tested is the bridge itself.
	#[test]
	fn positions_and_bytes_agree_for_ascii_content() {
		let buffer = DocumentBuffer::with_content("abc".to_string());
		for i in 0..=3 {
			assert_eq!(buffer.byte_at(pos(i)), i);
			assert_eq!(buffer.pos_at_byte(i.into()), i);
		}
		assert_eq!(buffer.len(), 3);
	}

	/// An astral char is one position like any other, on every platform, so that a saved position means the same place wherever it is read (#1062).
	#[test]
	fn an_astral_char_takes_one_position() {
		// U+1F600 is one char and four UTF-8 bytes.
		let buffer = DocumentBuffer::with_content("a\u{1F600}b".to_string());
		assert_eq!(buffer.end(), 3);
		assert_eq!(buffer.byte_at(pos(2)), 5); // 1 byte 'a' + 4 byte emoji
		assert_eq!(buffer.pos_at_byte(5.into()), 2);
		assert_eq!(buffer.pos_at_byte(2.into()), 2); // inside the emoji's bytes: rounds forward to 'b'
		assert_eq!(buffer.text(pos(1)..pos(3)), "\u{1F600}b");
	}

	#[test]
	fn lookups_past_the_end_stop_at_the_end() {
		let buffer = DocumentBuffer::with_content("abc".to_string());
		assert_eq!(buffer.byte_at(pos(999)), 3);
		assert_eq!(buffer.pos_at_byte(999.into()), 3);
		assert_eq!(buffer.text(pos(1)..pos(999)), "bc");
		assert_eq!(buffer.text(pos(2)..pos(1)), "");
		assert_eq!(buffer.nearest_pos(-5), 0);
		assert_eq!(buffer.nearest_pos(999), 3);
	}

	/// Every lookup agrees with a table of every char, across many checkpoints and with wide chars falling on either side of them, however the buffer was built and however finely it was cut up to be indexed.
	#[test]
	fn lookups_between_checkpoints_match_a_table_of_every_char() {
		let piece = "ab\u{1F600}日本\nc\u{00E9}\u{1F389}d\n";
		let text = piece.repeat(40);
		let starts: Vec<(usize, char)> = text.char_indices().collect();
		let newlines: Vec<DocPos> =
			starts.iter().enumerate().filter(|(_, (_, c))| *c == '\n').map(|(index, _)| pos(index)).collect();
		let mut appended = DocumentBuffer::new();
		for chunk in text.split_inclusive('\n') {
			appended.append(chunk);
		}
		let parts: Vec<String> = text.split_inclusive('\n').map(ToString::to_string).collect();
		let (from_parts, _) = DocumentBuffer::from_parts(parts);
		let mut chunked = Vec::new();
		for chunk_bytes in [1, 5, 64] {
			let mut buffer = DocumentBuffer::with_content(text.clone());
			buffer.rebuild_indexes_in(chunk_bytes);
			chunked.push(buffer);
		}
		for buffer in [DocumentBuffer::with_content(text.clone()), appended, from_parts].into_iter().chain(chunked) {
			assert_eq!(buffer.content, text);
			assert_eq!(buffer.len(), starts.len());
			assert_eq!(buffer.newline_positions(), newlines);
			for (index, &(byte, c)) in starts.iter().enumerate() {
				assert_eq!(buffer.byte_at(pos(index)), byte, "byte of char {index}");
				assert_eq!(buffer.pos_at_byte(byte.into()), index, "char at byte {byte}");
				for inside in byte + 1..byte + c.len_utf8() {
					assert_eq!(buffer.pos_at_byte(inside.into()), index + 1, "char inside byte {inside}");
				}
			}
			assert_eq!(buffer.pos_at_byte(text.len().into()), starts.len());
			assert_eq!(buffer.pos_at_byte((text.len() + 1).into()), starts.len());
			assert_eq!(buffer.byte_at(pos(starts.len())), text.len());
		}
	}

	/// Builds the buffer one part at a time with sequential `append` calls (a trailing `\n` per
	/// non-empty part unless it already ends with one), so
	/// `from_parts_matches_sequential_append_for_any_parts` can check that the parallel
	/// `from_parts` agrees with it exactly.
	fn naive_buffer_from_parts(parts: &[&str]) -> (DocumentBuffer, Vec<PartSpan>) {
		let mut buffer = DocumentBuffer::new();
		let mut spans = Vec::with_capacity(parts.len());
		for part in parts {
			let start = buffer.end();
			if !part.is_empty() {
				buffer.append(part);
				if !buffer.content.ends_with('\n') {
					buffer.append("\n");
				}
			}
			spans.push(PartSpan { start, end: buffer.end() });
		}
		(buffer, spans)
	}

	fn assert_buffers_equivalent(a: &DocumentBuffer, b: &DocumentBuffer) {
		assert_eq!(a.content, b.content);
		assert_eq!(a.len(), b.len());
		assert_eq!(a.newline_positions(), b.newline_positions());
		for i in 0..=a.len().get() {
			assert_eq!(a.byte_at(pos(i)), b.byte_at(pos(i)), "byte_at({i})");
		}
		for byte in a.content.char_indices().map(|(byte, _)| byte).chain([a.content.len()]) {
			assert_eq!(a.pos_at_byte(byte.into()), b.pos_at_byte(byte.into()), "pos_at_byte({byte})");
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

	fn positions(buffer: &DocumentBuffer) -> Vec<DocPos> {
		buffer.markers.iter().map(|m| m.position).collect()
	}

	#[test]
	fn replace_range_matches_sequential_build_and_shifts_markers() {
		let mut buffer = DocumentBuffer::with_content("aaa\nbbb\nccc".to_string());
		// Marker at the start of the "bbb" line (4) and of the "ccc" line (8),
		// plus one strictly inside the span being replaced (5, should be dropped).
		buffer.add_marker(Marker::new(MarkerType::Heading1, pos(4)));
		buffer.add_marker(Marker::new(MarkerType::Heading2, pos(5)));
		buffer.add_marker(Marker::new(MarkerType::Heading1, pos(8)));
		let delta = buffer.replace_range(pos(4)..pos(7), "XY");
		assert_eq!(delta, -1); // 2 positions replacing 3
		let expected = DocumentBuffer::with_content("aaa\nXY\nccc".to_string());
		assert_buffers_equivalent(&buffer, &expected);
		assert_eq!(buffer.content, "aaa\nXY\nccc");
		// The "bbb"-start marker (== start) is kept; the inside marker is dropped; the "ccc"
		// marker shifted from 8 to 7.
		assert_eq!(positions(&buffer), vec![pos(4), pos(7)]);
	}

	#[test]
	fn replace_range_can_insert_and_grow() {
		let mut buffer = DocumentBuffer::with_content("aaa\nccc".to_string());
		buffer.add_marker(Marker::new(MarkerType::PageBreak, pos(4)));
		let delta = buffer.replace_range(pos(4)..pos(4), "bbb\n"); // insert at start of the "ccc" line
		assert_eq!(delta, 4);
		let expected = DocumentBuffer::with_content("aaa\nbbb\nccc".to_string());
		assert_buffers_equivalent(&buffer, &expected);
		// The marker at 4 shifts forward by the inserted length.
		assert_eq!(buffer.markers[0].position, 8);
	}

	#[test]
	fn replace_range_after_an_astral_char() {
		// "a" then an astral emoji (one position but four bytes) before the replaced span, so the
		// position/byte conversion must stay correct through the mutation.
		let mut buffer = DocumentBuffer::with_content("a\u{1F600}b\nXYZ\nc".to_string());
		// positions: a=0, emoji=1, b=2, \n=3, X=4, Y=5, Z=6, \n=7, c=8 (len 9).
		buffer.add_marker(Marker::new(MarkerType::PageBreak, pos(4)));
		buffer.add_marker(Marker::new(MarkerType::PageBreak, pos(8)));
		let delta = buffer.replace_range(pos(4)..pos(7), "Q");
		assert_eq!(delta, -2);
		let expected = DocumentBuffer::with_content("a\u{1F600}b\nQ\nc".to_string());
		assert_buffers_equivalent(&buffer, &expected);
		// The marker at 4 (== start) stays; the marker at 8 shifts by -2 to 6.
		assert_eq!(positions(&buffer), vec![pos(4), pos(6)]);
		assert_eq!(buffer.len(), 7);
		assert_eq!(buffer.byte_at(pos(4)), 7);
	}

	#[test]
	fn replace_ranges_applies_every_edit_in_one_pass() {
		let mut buffer = DocumentBuffer::with_content("aaa\nbbb\nccc\nddd".to_string());
		// Replace the "bbb" line (4..7) and the "ddd" line (12..15) in one call.
		buffer.add_marker(Marker::new(MarkerType::PageBreak, pos(4)));
		buffer.add_marker(Marker::new(MarkerType::PageBreak, pos(8)));
		buffer.add_marker(Marker::new(MarkerType::PageBreak, pos(12)));
		let outcome = buffer.replace_ranges(vec![
			Edit { start: pos(12), end: pos(15), text: "WXYZ".to_string() },
			Edit { start: pos(4), end: pos(7), text: "B".to_string() },
		]);
		assert_eq!(buffer.content, "aaa\nB\nccc\nWXYZ");
		let expected = DocumentBuffer::with_content("aaa\nB\nccc\nWXYZ".to_string());
		assert_buffers_equivalent(&buffer, &expected);
		assert_eq!(outcome.total_delta, -1); // -2 for the shrinking edit, +1 for the growing one
		// The markers at each edit's start survive; the one after the first edit shifts by -2.
		assert_eq!(positions(&buffer), vec![pos(4), pos(6), pos(10)]);
	}

	#[test]
	fn replace_ranges_shifts_positions_the_way_it_shifts_markers() {
		let mut buffer = DocumentBuffer::with_content("aaa\nbbb\nccc\nddd".to_string());
		let outcome = buffer.replace_ranges(vec![
			Edit { start: pos(4), end: pos(7), text: "B".to_string() },
			Edit { start: pos(12), end: pos(15), text: "WXYZ".to_string() },
		]);
		assert_eq!(outcome.shift(pos(0)), 0); // before every edit
		assert_eq!(outcome.shift(pos(8)), 6); // after the first edit only
		assert_eq!(outcome.shift(pos(15)), 14); // after both: the old end of a document that shrank by one
	}

	#[test]
	fn replace_ranges_drops_the_image_only_marker_it_consumes() {
		let mut buffer = DocumentBuffer::with_content("aaa\nbbb\nccc".to_string());
		// A page break and an image-only marker share the placeholder line's start. The page
		// break has to survive OCR; the image-only marker is exactly what OCR consumes.
		buffer.add_marker(Marker::new(MarkerType::PageBreak, pos(4)));
		buffer.add_marker(Marker::new(MarkerType::ImageOnlyPage, pos(4)));
		buffer.replace_ranges(vec![Edit { start: pos(4), end: pos(7), text: "recognized".to_string() }]);
		let kinds: Vec<MarkerType> = buffer.markers.iter().map(|m| m.mtype).collect();
		assert_eq!(kinds, vec![MarkerType::PageBreak]);
		assert_eq!(buffer.markers[0].position, 4);
	}
}
