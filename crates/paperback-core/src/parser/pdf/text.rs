//! Plain-text extraction from a PDF page: reading pdfium's per-character stream into
//! visual lines (with RTL reordering per line), then joining those lines into paragraphs
//! and flagging large-font lines as headings by comparing against the page's median body
//! font size. Used as the fallback path for pages with no trustworthy structure tree (see
//! [`super::structure`]), and its [`sanitize_pdf_text`] helper is shared by metadata and TOC
//! extraction as well.

use std::{cmp::Ordering, mem};

use pdfium::{PdfiumTextPage, pdfium_types::FS_MATRIX};

use crate::{
	parser::util::bidi,
	util::text::{collapse_whitespace, display_len, trim_string},
};

pub(super) fn sanitize_pdf_text(input: &str) -> String {
	input.chars().filter(|&ch| (!ch.is_control() || matches!(ch, '\n' | '\r' | '\t')) && ch != '\u{00AD}').collect()
}

/// One visual line of an untagged page: its text, the point size it is set in, and the top
/// and bottom edges of its glyphs.
pub(super) type Line = (String, f64, f64, f64);

/// A character's box in PDF user units, as pdfium reports it.
#[derive(Clone, Copy)]
pub(super) struct CharBox {
	pub left: f64,
	pub right: f64,
	pub bottom: f64,
	pub top: f64,
}

/// Tolerance, in PDF user units, for calling two baselines or two box edges the same. pdfium
/// reports characters set on one line with identical coordinates, so this only has to absorb
/// rounding.
const COORDINATE_EPSILON: f64 = 0.05;
/// A space that carries less than this fraction of its own width in horizontal advance is not
/// separating anything. Real spaces in a page measured for #808 all advance by at least 70% of
/// their width; the spurious ones advance by 0-5%.
const NO_ADVANCE_RATIO: f64 = 0.25;
/// A space this much of whose width lies inside the box of the glyph before it is being drawn
/// underneath that glyph rather than after it. The same page measured 58% at most for real
/// spaces (an `f` overhangs the space after it) and 92% at least for the spurious ones.
const SWALLOWED_RATIO: f64 = 0.8;

/// Whether a U+0020 in the text layer renders as no space at all, judged from where pdfium puts
/// it relative to its neighbours. Both shapes of this reach us as an ordinary space that pdfium
/// does not mark as generated, so only the geometry tells them apart from a real one:
///
/// * A space with (almost) no horizontal advance: the character after it starts where the space
///   itself starts, so nothing on the page moves for it. The PDF of #808 leaves these in the
///   middle of words ("Pref ace") and in front of punctuation ("skills . The").
/// * A space drawn inside the glyph before it, which is what an `fi`/`fl` ligature that pdfium
///   has split back into its two code points leaves behind ("fi eld"): the ligature's advance
///   covers the space, so the space sits on top of it.
///
/// `next_origin` and `prev_box` are the raw neighbouring characters, line breaks included; a
/// neighbour on another line fails both the baseline and the containment check and so is
/// ignored, which is what should happen for a space at either end of a line.
pub(super) fn space_is_invisible(
	space_origin: (f64, f64),
	space_box: CharBox,
	next_origin: Option<(f64, f64)>,
	prev_box: Option<CharBox>,
) -> bool {
	let width = space_box.right - space_box.left;
	if width <= 0.0 {
		// A space whose own box is empty gives nothing to measure against. Fonts that give the
		// space glyph no width of its own and position words by hand land here, and their spaces
		// are the only word separator the page has.
		return false;
	}
	if let Some((next_x, next_y)) = next_origin
		&& (next_y - space_origin.1).abs() < COORDINATE_EPSILON
		&& (next_x - space_origin.0).abs() < width * NO_ADVANCE_RATIO
	{
		return true;
	}
	if let Some(prev) = prev_box {
		let on_the_same_line =
			prev.bottom <= space_box.bottom + COORDINATE_EPSILON && prev.top >= space_box.top - COORDINATE_EPSILON;
		let covered_width = prev.right - space_box.left;
		if on_the_same_line && covered_width >= width * SWALLOWED_RATIO {
			return true;
		}
	}
	false
}

fn char_origin(text_page: &PdfiumTextPage, index: i32) -> Option<(f64, f64)> {
	let (mut x, mut y) = (0.0, 0.0);
	text_page.get_char_origin(index, &mut x, &mut y).ok()?;
	Some((x, y))
}

fn char_box(text_page: &PdfiumTextPage, index: i32) -> Option<CharBox> {
	let rect = text_page.get_char_box(index).ok()?;
	Some(CharBox {
		left: f64::from(rect.left),
		right: f64::from(rect.right),
		bottom: f64::from(rect.bottom),
		top: f64::from(rect.top),
	})
}

/// [`space_is_invisible`] for the space at `index` of `text_page`, fetching only the geometry
/// each test actually needs. Costs three or four pdfium calls per space character and none at
/// all for anything else, so a page pays for it in proportion to its spaces rather than its
/// length - the per-character cost that #747 had to undo.
pub(super) fn is_invisible_space(text_page: &PdfiumTextPage, index: i32, char_count: i32) -> bool {
	let (Some(origin), Some(space_box)) = (char_origin(text_page, index), char_box(text_page, index)) else {
		return false;
	};
	let next_origin = if index + 1 < char_count { char_origin(text_page, index + 1) } else { None };
	if space_is_invisible(origin, space_box, next_origin, None) {
		return true;
	}
	let prev_box = if index > 0 { char_box(text_page, index - 1) } else { None };
	space_is_invisible(origin, space_box, None, prev_box)
}

/// Whether `ch` ends the current visual line. pdfium writes a line break as the pair `"\r\n"`,
/// so counting both characters would end the line twice and leave an empty line between every
/// pair of real ones - which [`join_paragraphs`] reads as a paragraph break, splitting every
/// wrapped paragraph into one paragraph per line (#808).
fn ends_line(ch: char, prev: Option<char>) -> bool {
	ch == '\r' || (ch == '\n' && prev != Some('\r'))
}

fn is_cjk(c: char) -> bool {
	let u = c as u32;
	(0x4E00..=0x9FFF).contains(&u) || // CJK Unified Ideographs
	(0x3400..=0x4DBF).contains(&u) || // CJK Extension A
	(0x20000..=0x2A6DF).contains(&u) || // CJK Extension B
	(0x3040..=0x309F).contains(&u) || // Hiragana
	(0x30A0..=0x30FF).contains(&u) || // Katakana
	(0xAC00..=0xD7AF).contains(&u) // Hangul
}

fn char_x_origin(text_page: &PdfiumTextPage, i: i32) -> f32 {
	let (mut x, mut y) = (0.0, 0.0);
	let _ = text_page.get_char_origin(i, &mut x, &mut y);
	x as f32
}

/// Assemble one run of `(char, pdfium index)` pairs into text, reordering
/// visual→logical for RTL scripts. Fetches x origins (a per-char FFI call)
/// only when the run actually contains an RTL character, so pure-LTR runs
/// (the overwhelming majority) pay a single cheap classification scan instead.
pub(super) fn reorder_run(text_page: &PdfiumTextPage, chars: &[(char, i32)]) -> String {
	if !bidi::contains_rtl(chars.iter().map(|&(c, _)| c)) {
		return chars.iter().map(|&(c, _)| c).collect();
	}
	let with_origin: Vec<(char, f32)> = chars.iter().map(|&(c, i)| (c, char_x_origin(text_page, i))).collect();
	bidi::reorder_line(&with_origin)
}

pub(super) fn extract_text_lines(text_page: &PdfiumTextPage, page_index: i32) -> Vec<Line> {
	let Ok(char_count) = text_page.char_count() else {
		tracing::warn!(
			page_index,
			"page text char count unavailable, falling back to whole-page text blob, heading detection by font size will be degraded for this page"
		);
		let raw = sanitize_pdf_text(&text_page.full()).replace('\r', "");
		return raw.lines().map(|l| (l.to_string(), 0.0, f64::NEG_INFINITY, f64::NEG_INFINITY)).collect();
	};
	let mut result: Vec<Line> = Vec::new();
	// Chars of the current visual line with their pdfium index, so each line can be
	// reordered visual→logical (handles RTL scripts) before paragraph joining.
	let mut current_chars: Vec<(char, i32)> = Vec::new();
	let mut previous_char = None;
	for i in 0..char_count {
		let unicode = text_page.get_unicode(i);
		let Some(ch) = char::from_u32(unicode) else { continue };
		if ends_line(ch, previous_char) {
			let size = line_font_size(text_page, &current_chars);
			let (top, bottom) = line_edges(text_page, &current_chars);
			result.push((reorder_run(text_page, &mem::take(&mut current_chars)), size, top, bottom));
		} else if ch == '\n' || (ch.is_control() && !matches!(ch, '\t')) || ch == '\u{00AD}' {
			// The '\n' of a "\r\n" pair, and anything else with no text of its own: dropped, but
			// still the previous character as far as the next `ends_line` is concerned.
		} else if ch != ' ' || !is_invisible_space(text_page, i, char_count) {
			current_chars.push((ch, i));
		}
		previous_char = Some(ch);
	}
	if !current_chars.is_empty() {
		let size = line_font_size(text_page, &current_chars);
		let (top, bottom) = line_edges(text_page, &current_chars);
		result.push((reorder_run(text_page, &current_chars), size, top, bottom));
	}
	result
}

/// The top edge of a line, in PDF user units, taken from its first character. Y grows up the
/// page, so the line nearest the top of it has the largest one.
///
/// This is all an untagged page gives to say where an image on it belongs, since the page draws
/// its images and sets its text in the same coordinates and says nothing about the order of the
/// two. The first character is measured rather than the tallest, because a couple of points
/// either way decides nothing about which line an image falls between and every character
/// measured costs another call into pdfium.
/// The top and bottom edges of a line.
///
/// The top comes from the first character, because a couple of points either way decides
/// nothing about which line an image falls between. The bottom is the lowest of the line's
/// characters, and that one has to be exact: it is what the whitespace before the next line
/// is measured from, and pdfium runs two visual lines together often enough that a line's
/// box regularly reaches a whole line lower than its first character does.
fn line_edges(text_page: &PdfiumTextPage, chars: &[(char, i32)]) -> (f64, f64) {
	let top = chars.first().and_then(|(_, index)| char_box(text_page, *index)).map_or(f64::NEG_INFINITY, |b| b.top);
	let mut bottom = f64::INFINITY;
	for (_, index) in chars {
		if let Some(boxed) = char_box(text_page, *index) {
			bottom = bottom.min(boxed.bottom);
		}
	}
	(top, bottom)
}

/// The point size one character is set in. pdfium reports the `Tf` size, which is 1.0 in every
/// PDF that scales its text through the text matrix instead of through `Tf` - both the PDF of
/// #808 and the one of #813 do, and every line of both came back as size 1.0, so no line was
/// ever tall enough to be taken for a heading. The matrix's vertical scale is the rest of the
/// size, so the two together are the size the reader sees.
fn effective_font_size(text_page: &PdfiumTextPage, index: i32) -> f64 {
	let size = text_page.get_font_size(index);
	let mut matrix = FS_MATRIX { a: 1.0, b: 0.0, c: 0.0, d: 1.0, e: 0.0, f: 0.0 };
	if text_page.get_matrix(index, &mut matrix).is_ok() { size * f64::from(matrix.b.hypot(matrix.d)) } else { size }
}

/// How many characters of a line to measure. Reading the size off the line's first character
/// costs one call but believes a chapter opening's drop cap, which stands three times the size
/// of the line it starts; the median of a handful of characters spread across the line does not.
/// Still a per-line cost rather than a per-character one - the per-character font-size calls
/// that #747 had to undo.
const LINE_FONT_SIZE_SAMPLES: usize = 5;

/// The point size of a visual line: the median of [`effective_font_size`] over a few of its
/// characters, ignoring whitespace (which a font may set in a size of its own).
fn line_font_size(text_page: &PdfiumTextPage, chars: &[(char, i32)]) -> f64 {
	let indices: Vec<i32> = chars.iter().filter(|(c, _)| !c.is_whitespace()).map(|&(_, i)| i).collect();
	if indices.is_empty() {
		return 0.0;
	}
	let step = indices.len().div_ceil(LINE_FONT_SIZE_SAMPLES).max(1);
	let mut sizes: Vec<f64> =
		indices.iter().step_by(step).map(|&i| effective_font_size(text_page, i)).filter(|size| *size > 0.0).collect();
	sorted_median(&mut sizes)
}

fn sorted_median(values: &mut [f64]) -> f64 {
	if values.is_empty() {
		return 0.0;
	}
	values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
	values[values.len() / 2]
}

pub(super) fn median_line_font_size(line_infos: &[Line]) -> f64 {
	let mut sizes: Vec<f64> = line_infos
		.iter()
		.filter(|(text, size, ..)| !text.trim().is_empty() && *size > 0.0)
		.map(|(_, size, ..)| *size)
		.collect();
	sorted_median(&mut sizes)
}

/// Fraction of the lines on a page that must be at least as long as the length this returns.
/// pdfium runs two visual lines together into one line of text often enough - a third of the
/// lines on some pages of #813 - that the longest line on a page is routinely a doubled one.
/// Measuring the page by that made every real line count as short, and a short line followed by
/// one starting with a capital or a digit is where [`join_paragraphs`] breaks a paragraph, so
/// wrapped paragraphs came apart line by line. The 75th percentile is past any run of doubled
/// lines while still landing on a line that fills the measure.
const FULL_LINE_PERCENTILE_NUMERATOR: usize = 3;
const FULL_LINE_PERCENTILE_DENOMINATOR: usize = 4;

/// The length of a line that fills the page's measure, in display units.
fn full_line_len(lines: &[(String, bool, f64, f64, f64)]) -> usize {
	let mut lengths: Vec<usize> = lines.iter().map(|(line, ..)| display_len(line)).filter(|len| *len > 0).collect();
	if lengths.is_empty() {
		return 0;
	}
	lengths.sort_unstable();
	lengths[(lengths.len() - 1) * FULL_LINE_PERCENTILE_NUMERATOR / FULL_LINE_PERCENTILE_DENOMINATOR]
}

/// How much clear space above a line means it opens a paragraph, as a fraction of its own
/// point size. Set from the four documents of #813 and #826: within a paragraph the space
/// between two lines runs to about two thirds of a line's size, and between paragraphs it
/// starts at about five fourths, so anything past a full size is a break with room to spare
/// either side.
const PARAGRAPH_GAP_RATIO: f64 = 1.0;

/// How much larger than the body a single letter must be set to read as a drop cap.
const DROP_CAP_RATIO: f64 = 2.0;

/// Glue each drop cap onto the paragraph it opens.
///
/// A drop cap is set as its own line, several times the body size, and every size-based test
/// reads it as a heading of one letter. What gives it away is the line under it: a drop cap
/// is the first letter of a word, so what follows carries on in lower case, where a chapter
/// number stands over a title in capitals. A digit is never a drop cap, which keeps a
/// numbered chapter opening out of this entirely.
fn merge_drop_caps(raw_lines: &[Line], body_font_size: f64) -> Vec<Line> {
	if body_font_size <= 0.0 {
		return raw_lines.to_vec();
	}
	let mut merged: Vec<Line> = Vec::with_capacity(raw_lines.len());
	let mut skip_next = false;
	for (index, line) in raw_lines.iter().enumerate() {
		if skip_next {
			skip_next = false;
			continue;
		}
		let (text, size, top, bottom) = line;
		let letter = trim_string(&collapse_whitespace(text));
		let is_cap = letter.chars().count() == 1
			&& letter.chars().next().is_some_and(char::is_alphabetic)
			&& *size >= body_font_size * DROP_CAP_RATIO;
		let follows_in_lower_case = raw_lines
			.get(index + 1)
			.map(|(next, ..)| trim_string(&collapse_whitespace(next)))
			.is_some_and(|next| next.chars().next().is_some_and(char::is_lowercase));
		if is_cap && follows_in_lower_case {
			let (next, next_size, next_top, next_bottom) = &raw_lines[index + 1];
			merged.push((letter + next.trim_start(), *next_size, *next_top, *next_bottom));
			skip_next = true;
		} else {
			merged.push((text.clone(), *size, *top, *bottom));
		}
	}
	merged
}

const HEADING_FONT_RATIO: f64 = 1.2;
const HEADING_MAX_LEN: usize = 150;

/// The point size at which a line reads as a heading rather than as body text.
fn heading_threshold(body_font_size: f64) -> f64 {
	if body_font_size > 0.0 { body_font_size * HEADING_FONT_RATIO } else { f64::INFINITY }
}

/// Reads every line as a paragraph of its own, for a reader who has turned paragraph joining
/// off.
///
/// Nothing in a PDF says whether five lines at the same left edge are five lines of code or
/// one wrapped sentence, so a document whose line breaks are the content (a listing, a poem,
/// a transcript) can only be read with the joining out of the way. Headings are still marked,
/// because that is a question about size and not about where a paragraph ends.
pub(super) fn split_lines(raw_lines: &[Line], body_font_size: f64) -> Vec<(String, bool, usize)> {
	let heading_threshold = heading_threshold(body_font_size);
	raw_lines
		.iter()
		.enumerate()
		.filter_map(|(index, (text, size, ..))| {
			let trimmed = trim_string(&collapse_whitespace(text));
			let len = display_len(&trimmed);
			if len == 0 {
				return None;
			}
			Some((trimmed, *size >= heading_threshold && len <= HEADING_MAX_LEN, index))
		})
		.collect()
}

/// The third field of each paragraph is the index, into `raw_lines`, of the line it starts
/// with. An untagged page needs it to place its images: the paragraphs no longer say where on
/// the page they were set, and that line does.
pub(super) fn join_paragraphs(raw_lines: &[Line], body_font_size: f64) -> Vec<(String, bool, usize)> {
	let heading_threshold = heading_threshold(body_font_size);
	let raw_lines = merge_drop_caps(raw_lines, body_font_size);
	let lines: Vec<(String, bool, f64, f64, f64)> = raw_lines
		.iter()
		.map(|(text, size, top, bottom)| {
			let trimmed = trim_string(&collapse_whitespace(text));
			let len = display_len(&trimmed);
			let is_heading_line = *size >= heading_threshold && len > 0 && len <= HEADING_MAX_LEN;
			(trimmed, is_heading_line, *size, *top, *bottom)
		})
		.collect();
	// Two ways a line can end a paragraph, needing different amounts of evidence. A line that
	// ends a sentence and stops any way short of the measure has ended the paragraph with it. A
	// line that ends mid-sentence has to fall well short before the next line starting like a
	// fresh sentence counts for anything, since a capital there is as likely to open a name.
	let full_line = full_line_len(&lines);
	let ended_sentence_threshold = full_line * 19 / 20;
	let short_line_threshold = full_line * 3 / 4;
	let mut paragraphs: Vec<(String, bool, usize)> = Vec::new();
	let mut current_paragraph = String::new();
	let mut current_is_heading = false;
	let mut current_heading_size = 0.0f64;
	let mut current_start_line = 0usize;
	let mut last_line_len = 0usize;
	let mut last_line_ends_with_punctuation = false;
	// The bottom edge of the last line that carried text, which the whitespace before the
	// next one is measured down from. Infinity until there is one, so the first line of a
	// page never reads as following a gap.
	let mut last_line_bottom = f64::INFINITY;
	for (line_index, (line, is_heading_line, size, top, bottom)) in lines.iter().enumerate() {
		if line.is_empty() {
			if !current_paragraph.is_empty() {
				paragraphs.push((mem::take(&mut current_paragraph), current_is_heading, current_start_line));
				current_is_heading = false;
			}
			last_line_len = 0;
			last_line_ends_with_punctuation = false;
			last_line_bottom = f64::INFINITY;
			continue;
		}
		let is_list_item = line.starts_with("- ") || line.starts_with("* ") || line.starts_with("• ");
		let first_char = line.chars().next();
		let starts_with_uppercase = first_char.is_some_and(char::is_uppercase);
		let starts_with_alpha = first_char.is_some_and(char::is_alphabetic);
		let len = display_len(line);
		if current_paragraph.is_empty() {
			current_paragraph = line.clone();
			current_is_heading = *is_heading_line;
			current_heading_size = *size;
			current_start_line = line_index;
		} else {
			let mut is_numbered = false;
			let mut chars = line.chars();
			if let Some(first) = chars.next()
				&& first.is_ascii_digit()
			{
				let mut found_space = false;
				for c in chars {
					if c.is_ascii_digit() || c == '.' || c == ')' {
						continue;
					} else if c.is_whitespace() {
						found_space = true;
						break;
					}
					break;
				}
				is_numbered = found_space;
			}
			// A heading set over several lines - a chapter title, most often - carries on rather
			// than becoming one heading per line, as long as the size does not change. A line
			// that opens with its own number is a heading of its own even so, which is what
			// keeps a numbered outline from running its sections together. Every other heading
			// boundary, and every list item, starts a new paragraph; otherwise the previous line
			// does when it stopped short of the measure.
			let continues_heading =
				*is_heading_line && current_is_heading && (*size - current_heading_size).abs() < f64::EPSILON;
			let previous_line_ended_paragraph = (last_line_ends_with_punctuation
				&& last_line_len < ended_sentence_threshold)
				|| (last_line_len < short_line_threshold && (starts_with_uppercase || !starts_with_alpha));
			// Extra whitespace above a line is the one signal a document sets deliberately and
			// consistently, and it is what separates a run of one-line paragraphs that every
			// length-based test reads as one block. It is measured from the previous line's
			// lowest glyph rather than from its top, because pdfium runs two visual lines
			// together often enough that a top-to-top pitch is doubled all over a normal
			// paragraph and would break it apart.
			let opened_by_a_gap = *size > 0.0 && last_line_bottom - *top > *size * PARAGRAPH_GAP_RATIO;
			let break_paragraph = is_list_item
				|| is_numbered
				|| opened_by_a_gap
				|| (!continues_heading && (*is_heading_line || current_is_heading || previous_line_ended_paragraph));
			if break_paragraph {
				paragraphs.push((mem::take(&mut current_paragraph), current_is_heading, current_start_line));
				current_paragraph = line.clone();
				current_is_heading = *is_heading_line;
				current_heading_size = *size;
				current_start_line = line_index;
			} else {
				let last_char = current_paragraph.chars().last().unwrap_or(' ');
				if current_paragraph.ends_with('-') {
					current_paragraph.pop();
					current_paragraph.push_str(line);
				} else if is_cjk(last_char) && line.chars().next().is_some_and(is_cjk) {
					current_paragraph.push_str(line);
				} else {
					current_paragraph.push(' ');
					current_paragraph.push_str(line);
				}
			}
		}
		last_line_len = len;
		last_line_bottom = *bottom;
		last_line_ends_with_punctuation = line.ends_with('.')
			|| line.ends_with('?')
			|| line.ends_with('!')
			|| line.ends_with(':')
			|| line.ends_with('"')
			|| line.ends_with('\u{201D}')
			|| line.ends_with('。')
			|| line.ends_with('？')
			|| line.ends_with('！')
			|| line.ends_with('：');
	}
	if !current_paragraph.is_empty() {
		paragraphs.push((current_paragraph, current_is_heading, current_start_line));
	}
	paragraphs
}

#[cfg(test)]
mod tests {
	use super::{
		CharBox, Line, ends_line, full_line_len, join_paragraphs, sanitize_pdf_text, space_is_invisible, split_lines,
	};

	/// The coordinates below come from what pdfium reports for the PDF attached to #808, so the
	/// ratios each case turns on are the ones real pages produce.
	fn char_box(left: f64, right: f64, bottom: f64, top: f64) -> CharBox {
		CharBox { left, right, bottom, top }
	}

	#[test]
	fn ends_line_treats_crlf_as_one_break() {
		assert!(ends_line('\r', Some('e')));
		assert!(!ends_line('\n', Some('\r')), "the '\\n' of a \"\\r\\n\" pair must not end a second line");
		assert!(ends_line('\n', Some('e')), "a lone '\\n' still ends the line");
		assert!(ends_line('\r', Some('\n')), "a following \"\\r\\n\" pair starts over");
		assert!(!ends_line('e', Some('\r')));
	}

	/// "Pref ace": the 'a' starts exactly where the space does, so the space moves nothing.
	#[test]
	fn space_is_invisible_when_it_has_no_advance() {
		assert!(space_is_invisible(
			(82.88, 592.48),
			char_box(82.88, 86.88, 592.48, 592.49),
			Some((82.88, 592.48)),
			None
		));
	}

	/// "fi eld": the split `fi` ligature's box covers the space that follows it.
	#[test]
	fn space_is_invisible_when_swallowed_by_the_previous_glyph() {
		let space = char_box(287.34, 289.84, 387.61, 387.62);
		// The advance alone looks ordinary here, so only the ligature's box gives it away.
		assert!(!space_is_invisible((287.34, 387.61), space, Some((290.12, 387.61)), None));
		assert!(space_is_invisible((287.34, 387.61), space, None, Some(char_box(284.87, 289.77, 387.61, 394.44))));
	}

	/// "name of paper": an `f` overhangs the space after it by more than half its width, and that
	/// space advances normally. It stays.
	#[test]
	fn space_after_an_overhanging_glyph_is_kept() {
		assert!(!space_is_invisible(
			(105.09, 387.61),
			char_box(105.09, 107.59, 387.61, 387.62),
			Some((107.59, 387.61)),
			Some(char_box(102.50, 106.54, 387.61, 394.44))
		));
	}

	/// A neighbour on another line decides nothing: a space at the end of a line keeps whatever
	/// the line below happens to sit under.
	#[test]
	fn neighbours_on_another_line_are_ignored() {
		let space = char_box(287.34, 289.84, 387.61, 387.62);
		assert!(!space_is_invisible((287.34, 387.61), space, Some((287.34, 375.61)), None));
		assert!(!space_is_invisible((287.34, 387.61), space, None, Some(char_box(284.87, 289.77, 375.61, 382.44))));
	}

	/// A font whose space glyph has no width of its own gives nothing to measure, and its spaces
	/// may be the only word separators on the page.
	#[test]
	fn zero_width_space_glyph_is_kept() {
		assert!(!space_is_invisible(
			(82.88, 592.48),
			char_box(82.88, 82.88, 592.48, 592.49),
			Some((82.88, 592.48)),
			None
		));
	}

	/// The lines of page 67 of the PDF attached to #813, with the sizes and the doubled lines
	/// pdfium reports for it: a chapter number, a four-line chapter title, an epigraph over two
	/// lines, and a paragraph whose second line opens with a year.
	fn issue_813_page() -> Vec<Line> {
		tight(&[
			("4", 17.0),
			("INCONVENIENT OR", 24.0),
			("NOT, THE TRUTH IS", 24.0),
			("THAT CLIMATE CHANGE", 24.0),
			("IS NOT THE ISSUE", 24.0),
			("Let's quit debating global warming and manage", 12.0),
			("gaseous wastes as we do other trash.", 12.0),
			("On a hot, muggy August afternoon during the summer of", 11.0),
			("1996, I was hard at work in a conference room of the", 11.0),
			// One line of text, two visual lines: pdfium ran them together.
			(
				"Ministry of Aviation Industries of China (AVIC) in Beijing, discussing human resources for a potential joint venture between",
				11.0,
			),
			("AlliedSignal, the company for which I was then international", 11.0),
			("human resources vice president, and AVIC. When the meeting", 11.0),
			(
				"ended, one of my hosts from the ministry graciously accompanied me outside. As we waited for the car, he noticed that I was",
				11.0,
			),
			("looking up and down the street and then skyward. Visibility was", 11.0),
			("only about a hundred yards in any direction. I was shocked. The", 11.0),
			("executive looked at me and said, \u{201C}My children do not know the", 11.0),
			("sky is blue.\u{201D}", 11.0),
		])
	}

	/// The page's measure comes from a line that fills it, not from one of the doubled lines
	/// pdfium produced, which are nearly twice as long.
	#[test]
	fn full_line_len_ignores_doubled_lines() {
		let lines: Vec<(String, bool, f64, f64, f64)> =
			issue_813_page().into_iter().map(|(text, size, top, bottom)| (text, false, size, top, bottom)).collect();
		let full = full_line_len(&lines);
		assert!((53..=64).contains(&full), "expected the length of a full line, got {full}");
	}

	/// #813: the body paragraph survives its second line opening with a year and its later lines
	/// opening with capitals, and the chapter title is one heading rather than one per line.
	#[test]
	fn join_paragraphs_keeps_a_chapter_opening_whole() {
		let result = join_paragraphs(&issue_813_page(), 11.0);
		let texts: Vec<&str> = result.iter().map(|(text, ..)| text.as_str()).collect();
		assert_eq!(
			texts,
			vec![
				"4",
				"INCONVENIENT OR NOT, THE TRUTH IS THAT CLIMATE CHANGE IS NOT THE ISSUE",
				"Let's quit debating global warming and manage gaseous wastes as we do other trash.",
				"On a hot, muggy August afternoon during the summer of 1996, I was hard at work in a conference room of \
				 the Ministry of Aviation Industries of China (AVIC) in Beijing, discussing human resources for a \
				 potential joint venture between AlliedSignal, the company for which I was then international human \
				 resources vice president, and AVIC. When the meeting ended, one of my hosts from the ministry \
				 graciously accompanied me outside. As we waited for the car, he noticed that I was looking up and \
				 down the street and then skyward. Visibility was only about a hundred yards in any direction. I was \
				 shocked. The executive looked at me and said, \u{201C}My children do not know the sky is blue.\u{201D}",
			]
		);
		assert!(result[1].1, "the chapter title is a heading");
		assert!(!result[3].1, "the body paragraph is not");
	}

	/// A numbered outline sets every level in one size, so the sections must not run together
	/// the way the lines of a single title do.
	#[test]
	fn join_paragraphs_keeps_numbered_headings_apart() {
		let lines = tight(&[
			("2.1.1 Piano Roll mode", 14.0),
			("2.1.1.1 Show / Hide Strings", 14.0),
			("2.1.1.2 Color Indicates String or Velocity", 14.0),
		]);
		let result = join_paragraphs(&lines, 10.0);
		assert_eq!(result.len(), 3, "got {result:?}");
	}

	/// The chapter number is set in its own size, so it does not join the title below it.
	#[test]
	fn join_paragraphs_keeps_headings_of_different_sizes_apart() {
		let lines = tight(&[("PART ONE", 30.0), ("Getting Started", 20.0)]);
		let result = join_paragraphs(&lines, 11.0);
		assert_eq!(result.len(), 2);
	}

	/// A line that ends a sentence ends the paragraph even when it reaches most of the way across
	/// the measure - the case a single short-line threshold gets wrong.
	#[test]
	fn join_paragraphs_breaks_after_a_sentence_that_nearly_fills_the_line() {
		let lines = tight(&[
			("This book is part of the English for Research series of guides for non-native English", 10.0),
			("academics of all disciplines who work in an international field.", 10.0),
			("EAP trainers can use this book in conjunction with: English for Academic Research:", 10.0),
			("A Guide for Teachers.", 10.0),
		]);
		let result = join_paragraphs(&lines, 10.0);
		assert_eq!(result.len(), 2, "got {result:?}");
		assert!(result[0].0.ends_with("international field."));
		assert!(result[1].0.starts_with("EAP trainers"));
	}

	/// A paragraph that pdfium hands back as wrapped lines is one paragraph, not one per line.
	#[test]
	fn join_paragraphs_merges_a_wrapped_list_item() {
		let lines = tight(&[
			("1) Look at your Inbox in your email account. Analyse 10-20 subject lines and", 9.5),
			("decide some criteria for judging how effective the subject lines are. Compare", 9.5),
			("your criteria with a colleague's.", 9.5),
		]);
		let result = join_paragraphs(&lines, 9.5);
		assert_eq!(result.len(), 1, "got {result:?}");
	}

	#[test]
	fn sanitize_pdf_text_strips_control_chars_and_soft_hyphens() {
		assert_eq!(sanitize_pdf_text("sugges\u{0002}tion\tline\r\nnext"), "suggestion\tline\r\nnext");
		assert_eq!(sanitize_pdf_text("hy\u{00AD}phen"), "hyphen");
	}

	#[test]
	fn join_paragraphs_merges_continuation_lines() {
		let lines = tight(&[("The suggestion appears here.", 12.0), ("And here.", 12.0)]);
		let result = join_paragraphs(&lines, 12.0);
		assert_eq!(result.len(), 1);
		assert_eq!(result[0].0, "The suggestion appears here. And here.");
		assert!(!result[0].1);
	}

	/// Lay `lines` out down a page with ordinary leading and no gap between any two, so a test
	/// written about the text alone is not accidentally testing the paragraph-gap rule.
	fn tight(lines: &[(&str, f64)]) -> Vec<Line> {
		let mut top = 700.0;
		let mut out = Vec::new();
		for (text, size) in lines {
			let bottom = top - size;
			out.push(((*text).to_string(), *size, top, bottom));
			// A fifth of a line of clear space: what sits between two lines of one paragraph.
			top = bottom - size * 0.2;
		}
		out
	}

	/// Lay `lines` out with a full line of clear space above each one that is marked, which is
	/// what a document puts between two paragraphs.
	fn spaced(lines: &[(&str, f64, bool)]) -> Vec<Line> {
		let mut top = 700.0;
		let mut out = Vec::new();
		for (text, size, gap_above) in lines {
			if *gap_above {
				top -= size * 1.4;
			}
			let bottom = top - size;
			out.push(((*text).to_string(), *size, top, bottom));
			top = bottom - size * 0.2;
		}
		out
	}

	// #813 again: a run of one-line paragraphs is what every length test reads as one block,
	// since no line among them stops short of a measure they never reach. The space a document
	// leaves above each one is the only thing that tells them apart.
	#[test]
	fn join_paragraphs_breaks_where_a_document_leaves_a_gap() {
		let lines = spaced(&[
			("Before We Get Started", 12.0, false),
			("About the Author: Valentina Romano", 12.0, true),
			("Lesson One", 12.0, true),
			("Lesson Two", 12.0, true),
		]);
		let result = join_paragraphs(&lines, 12.0);
		let texts: Vec<&str> = result.iter().map(|(text, ..)| text.as_str()).collect();
		assert_eq!(texts, ["Before We Get Started", "About the Author: Valentina Romano", "Lesson One", "Lesson Two"]);
	}

	// The same document sets a wrapped paragraph with no such gap, and that has to stay whole.
	#[test]
	fn join_paragraphs_keeps_a_wrapped_paragraph_whole_across_the_gap_rule() {
		let lines = spaced(&[
			("INSTRUCTOR: Let's review. In the following exercise you will play the role of", 12.0, false),
			("the male speaker ONLY. Be sure to make your response before the male speaker", 12.0, false),
			("and then repeat his answer after him.", 12.0, false),
			("FEMALE: Parli italiano?", 12.0, true),
		]);
		let result = join_paragraphs(&lines, 12.0);
		assert_eq!(result.len(), 2, "got {result:?}");
		assert!(result[0].0.ends_with("repeat his answer after him."));
		assert_eq!(result[1].0, "FEMALE: Parli italiano?");
	}

	// pdfium runs two visual lines together into one line of text all over a normal page. Such
	// a line reaches a whole line lower than its neighbours, so measuring the space above the
	// next line from its top rather than its bottom would break paragraphs everywhere.
	#[test]
	fn join_paragraphs_is_not_fooled_by_a_line_pdfium_doubled() {
		let mut lines = tight(&[
			("On a hot, muggy August afternoon during the summer of", 11.0),
			("1996, I was hard at work in a conference room of the", 11.0),
			("Ministry of Aviation Industries of China in Beijing, discussing human resources", 11.0),
			("AlliedSignal, the company for which I was then international", 11.0),
		]);
		// The third line is two visual lines run together, so its box reaches a line lower.
		lines[2].3 -= 11.0;
		lines[3].2 -= 11.0;
		lines[3].3 -= 11.0;
		let result = join_paragraphs(&lines, 11.0);
		assert_eq!(result.len(), 1, "got {result:?}");
	}

	// #826: a drop cap is one big letter set beside the paragraph it opens, and reads as a
	// heading of one letter to anything that goes by size.
	#[test]
	fn join_paragraphs_glues_a_drop_cap_to_its_paragraph() {
		let lines = tight(&[("T", 153.5), ("his morning, people all over the planet got out of bed.", 12.0)]);
		let result = join_paragraphs(&lines, 12.0);
		assert_eq!(result.len(), 1, "got {result:?}");
		assert_eq!(result[0].0, "This morning, people all over the planet got out of bed.");
		assert!(!result[0].1, "the paragraph it opens is not a heading");
	}

	// A chapter number is also one large character on a line of its own. What separates it
	// from a drop cap is that a title follows, not the rest of a word.
	#[test]
	fn join_paragraphs_leaves_a_chapter_number_alone() {
		let digit = tight(&[("4", 40.0), ("INCONVENIENT OR NOT", 24.0)]);
		assert_eq!(join_paragraphs(&digit, 11.0).len(), 2);
		// A roman numeral is a letter, so only the case of what follows tells them apart.
		let roman = tight(&[("I", 40.0), ("The Long Road Home", 24.0)]);
		assert_eq!(join_paragraphs(&roman, 11.0).len(), 2);
	}

	#[test]
	fn join_paragraphs_leaves_an_ordinary_capital_alone() {
		// Body-sized, so nothing to do with a drop cap however the next line starts.
		let lines = tight(&[("A", 12.0), ("small letter follows.", 12.0)]);
		assert_eq!(join_paragraphs(&lines, 12.0).len(), 1);
	}

	// #813: nothing in a PDF separates five lines of code from five wrapped lines of prose,
	// so a reader whose document is a listing turns the joining off and gets the lines back.
	#[test]
	fn split_lines_keeps_every_line_apart() {
		let lines = tight(&[
			("Listing 1: Five lines of Python", 11.0),
			("def greet(name):", 11.0),
			("# say hello", 11.0),
			("message = \"Hello, \" + name", 11.0),
			("print(message)", 11.0),
			("return message", 11.0),
		]);
		let result = split_lines(&lines, 11.0);
		let texts: Vec<&str> = result.iter().map(|(text, ..)| text.as_str()).collect();
		assert_eq!(
			texts,
			[
				"Listing 1: Five lines of Python",
				"def greet(name):",
				"# say hello",
				"message = \"Hello, \" + name",
				"print(message)",
				"return message"
			]
		);
	}

	// The same lines run together when the joining is on, which is the whole reason for the
	// switch.
	#[test]
	fn join_paragraphs_runs_the_same_listing_together() {
		let lines = tight(&[("def greet(name):", 11.0), ("# say hello", 11.0), ("return message", 11.0)]);
		assert_eq!(join_paragraphs(&lines, 11.0).len(), 1);
	}

	// A heading is a question about size, not about where a paragraph ends, so it is still
	// marked with the joining off. Navigating by heading has to keep working.
	#[test]
	fn split_lines_still_marks_headings() {
		let lines = tight(&[("A Short Code Listing", 20.0), ("def greet(name):", 11.0)]);
		let result = split_lines(&lines, 11.0);
		assert_eq!(result.len(), 2);
		assert!(result[0].1, "the large line is a heading");
		assert!(!result[1].1, "the body line is not");
	}

	// Each paragraph reports the line it came from, which is what places the images of an
	// untagged page. A blank line is dropped and must not shift the ones after it.
	#[test]
	fn split_lines_reports_the_line_each_paragraph_came_from() {
		let lines = tight(&[("first", 11.0), ("   ", 11.0), ("third", 11.0)]);
		let result = split_lines(&lines, 11.0);
		assert_eq!(result.len(), 2);
		assert_eq!(result[0].2, 0);
		assert_eq!(result[1].2, 2);
	}

	#[test]
	fn join_paragraphs_flags_large_font_lines_as_headings() {
		let lines = tight(&[("Chapter One", 18.0), ("This is the body text of the document.", 12.0)]);
		let result = join_paragraphs(&lines, 12.0);
		assert_eq!(result.len(), 2);
		assert_eq!(result[0].0, "Chapter One");
		assert!(result[0].1);
		assert_eq!(result[1].0, "This is the body text of the document.");
		assert!(!result[1].1);
	}
}
