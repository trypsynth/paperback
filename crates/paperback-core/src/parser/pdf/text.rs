//! Reading the text off a PDF page: pdfium's per-character stream assembled into visual
//! lines, each reordered visual to logical so a right-to-left script comes out in reading
//! order, and each measured for the size and the face it is set in.
//!
//! What those lines mean is [`super::paragraphs`]. This module only reports what is on the
//! page. It is the fallback path for a page with no trustworthy structure tree (see
//! [`super::structure`]), and its [`sanitize_pdf_text`] helper is shared by the metadata and
//! table of contents readers as well.

use std::{cell::Cell, cmp::Ordering};

use crate::{
	parser::util::bidi,
	pdfium::{CharBox, PdfTextPage},
};

pub(super) fn sanitize_pdf_text(input: &str) -> String {
	input.chars().filter(|&ch| (!ch.is_control() || matches!(ch, '\n' | '\r' | '\t')) && ch != '\u{00AD}').collect()
}

/// One visual line of an untagged page.
#[derive(Clone, Debug)]
pub(super) struct Line {
	pub text: String,
	/// The point size it is set in.
	pub size: f64,
	/// The upper edge of its first glyph.
	pub top: f64,
	/// The lower edge of its lowest glyph.
	pub bottom: f64,
	/// Whether it is set in a monospaced face, which marks it as something whose own line
	/// breaks are the content: code, or anything else laid out by column.
	pub monospaced: bool,
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
/// How many of a page's spaces are measured to decide whether a covered space means anything there. Enough to see the font's habit, few enough that the pdfium calls stay a fixed cost per page.
const COVER_SAMPLE: usize = 64;

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
	prev_box.is_some_and(|prev| covers(prev, space_box))
}

/// Whether `prev`, on the same line, lies over most of `space_box`, the second shape [`space_is_invisible`] looks for.
fn covers(prev: CharBox, space_box: CharBox) -> bool {
	let width = space_box.right - space_box.left;
	let on_the_same_line =
		prev.bottom <= space_box.bottom + COORDINATE_EPSILON && prev.top >= space_box.top - COORDINATE_EPSILON;
	width > 0.0 && on_the_same_line && prev.right - space_box.left >= width * SWALLOWED_RATIO
}

/// Whether a space covered by the glyph before it is hidden by that glyph, given how many of a page's measured spaces are covered. A split ligature in front of a space is rare: the PDF of #808 has at most 4% of a page's spaces covered. A font whose glyph boxes are wider than their advances covers nearly all of them, 64% to 91% a page in the PDF of #993, and there being covered says nothing.
fn covering_hides(covered: usize, measured: usize) -> bool {
	covered * 4 <= measured
}

/// The top edge of one character, which is what gives a tagged block the height the structure
/// tree never records for it. See [`super::images::UnclaimedImages`].
pub(super) fn char_top(text_page: &PdfTextPage, index: i32) -> Option<f64> {
	text_page.char_box(index).map(|boxed| boxed.top)
}

/// [`space_is_invisible`] for the spaces of one page, fetching only the geometry each test actually needs. Costs three or four pdfium calls per space character and none at all for anything else, so a page pays for it in proportion to its spaces rather than its length - the per-character cost that #747 had to undo.
pub(super) struct SpaceFilter {
	char_count: i32,
	/// [`covering_hides`] for this page, measured the first time a covered space turns up, so a page without one never pays for it.
	covering_hides: Cell<Option<bool>>,
}

impl SpaceFilter {
	pub(super) const fn new(char_count: i32) -> Self {
		Self { char_count, covering_hides: Cell::new(None) }
	}

	/// Whether the space at `index` of `text_page` renders as no space at all.
	pub(super) fn hides(&self, text_page: &PdfTextPage, index: i32) -> bool {
		let (Some(origin), Some(space_box)) = (text_page.char_origin(index), text_page.char_box(index)) else {
			return false;
		};
		let next_origin = if index + 1 < self.char_count { text_page.char_origin(index + 1) } else { None };
		if space_is_invisible(origin, space_box, next_origin, None) {
			return true;
		}
		let prev_box = if index > 0 { text_page.char_box(index - 1) } else { None };
		space_is_invisible(origin, space_box, None, prev_box) && self.covering_hides(text_page)
	}

	fn covering_hides(&self, text_page: &PdfTextPage) -> bool {
		if let Some(hides) = self.covering_hides.get() {
			return hides;
		}
		let (mut measured, mut covered) = (0, 0);
		for index in 1..self.char_count {
			if measured == COVER_SAMPLE {
				break;
			}
			if text_page.unicode_at(index) != u32::from(' ') {
				continue;
			}
			let (Some(space_box), Some(prev)) = (text_page.char_box(index), text_page.char_box(index - 1)) else {
				continue;
			};
			measured += 1;
			if covers(prev, space_box) {
				covered += 1;
			}
		}
		let hides = covering_hides(covered, measured);
		self.covering_hides.set(Some(hides));
		hides
	}
}

/// Whether `ch` ends the current visual line. pdfium writes a line break as the pair `"\r\n"`,
/// so counting both characters would end the line twice and leave an empty line between every
/// pair of real ones - which [`join_paragraphs`] reads as a paragraph break, splitting every
/// wrapped paragraph into one paragraph per line (#808).
fn ends_line(ch: char, prev: Option<char>) -> bool {
	ch == '\r' || (ch == '\n' && prev != Some('\r'))
}

pub(super) fn is_cjk(c: char) -> bool {
	let u = c as u32;
	(0x4E00..=0x9FFF).contains(&u) || // CJK Unified Ideographs
	(0x3400..=0x4DBF).contains(&u) || // CJK Extension A
	(0x20000..=0x2A6DF).contains(&u) || // CJK Extension B
	(0x3040..=0x309F).contains(&u) || // Hiragana
	(0x30A0..=0x30FF).contains(&u) || // Katakana
	(0xAC00..=0xD7AF).contains(&u) // Hangul
}

fn char_x_origin(text_page: &PdfTextPage, i: i32) -> f32 {
	text_page.char_origin(i).map_or(0.0, |(x, _)| x as f32)
}

/// Assemble one run of `(char, pdfium index)` pairs into text, reordering
/// visual→logical for RTL scripts. Fetches x origins (a per-char FFI call)
/// only when the run actually contains an RTL character, so pure-LTR runs
/// (the overwhelming majority) pay a single cheap classification scan instead.
pub(super) fn reorder_run(text_page: &PdfTextPage, chars: &[(char, i32)]) -> String {
	if !bidi::contains_rtl(chars.iter().map(|&(c, _)| c)) {
		return chars.iter().map(|&(c, _)| c).collect();
	}
	let with_origin: Vec<(char, f32)> = chars.iter().map(|&(c, i)| (c, char_x_origin(text_page, i))).collect();
	bidi::reorder_line(&with_origin)
}

/// Reads one visual line: its text in logical order, and everything measured about it.
fn measure_line(text_page: &PdfTextPage, chars: &[(char, i32)]) -> Line {
	let (top, bottom) = line_edges(text_page, chars);
	Line {
		size: line_font_size(text_page, chars),
		monospaced: line_is_monospaced(text_page, chars),
		text: reorder_run(text_page, chars),
		top,
		bottom,
	}
}

pub(super) fn extract_text_lines(text_page: &PdfTextPage, page_index: i32) -> Vec<Line> {
	let Some(char_count) = text_page.char_count() else {
		tracing::warn!(
			page_index,
			"page text char count unavailable, falling back to whole-page text blob, heading detection by font size will be degraded for this page"
		);
		let raw = sanitize_pdf_text(&text_page.text()).replace('\r', "");
		return raw
			.lines()
			.map(|line| Line {
				text: line.to_string(),
				size: 0.0,
				top: f64::NEG_INFINITY,
				bottom: f64::NEG_INFINITY,
				monospaced: false,
			})
			.collect();
	};
	let mut result: Vec<Line> = Vec::new();
	// Chars of the current visual line with their pdfium index, so each line can be
	// reordered visual→logical (handles RTL scripts) before paragraph joining.
	let mut current_chars: Vec<(char, i32)> = Vec::new();
	let mut previous_char = None;
	let spaces = SpaceFilter::new(char_count);
	for i in 0..char_count {
		let unicode = text_page.unicode_at(i);
		let Some(ch) = char::from_u32(unicode) else { continue };
		if ends_line(ch, previous_char) {
			let line = measure_line(text_page, &current_chars);
			current_chars.clear();
			result.push(line);
		} else if ch == '\n' || (ch.is_control() && !matches!(ch, '\t')) || ch == '\u{00AD}' {
			// The '\n' of a "\r\n" pair, and anything else with no text of its own: dropped, but
			// still the previous character as far as the next `ends_line` is concerned.
		} else if ch != ' ' || !spaces.hides(text_page, i) {
			current_chars.push((ch, i));
		}
		previous_char = Some(ch);
	}
	if !current_chars.is_empty() {
		result.push(measure_line(text_page, &current_chars));
	}
	restore_reading_order(result)
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
fn line_edges(text_page: &PdfTextPage, chars: &[(char, i32)]) -> (f64, f64) {
	let top = chars.first().and_then(|(_, index)| text_page.char_box(*index)).map_or(f64::NEG_INFINITY, |b| b.top);
	let mut bottom = f64::INFINITY;
	for (_, index) in chars {
		if let Some(boxed) = text_page.char_box(*index) {
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
fn effective_font_size(text_page: &PdfTextPage, index: i32) -> f64 {
	let size = text_page.font_size(index);
	text_page.text_matrix(index).map_or(size, |matrix| size * f64::from(matrix.b.hypot(matrix.d)))
}

/// How many characters of a line to measure. Reading the size off the line's first character
/// costs one call but believes a chapter opening's drop cap, which stands three times the size
/// of the line it starts; the median of a handful of characters spread across the line does not.
/// Still a per-line cost rather than a per-character one - the per-character font-size calls
/// that #747 had to undo.
const LINE_FONT_SIZE_SAMPLES: usize = 5;

/// The point size of a visual line: the median of [`effective_font_size`] over a few of its
/// characters, ignoring whitespace (which a font may set in a size of its own).
fn line_font_size(text_page: &PdfTextPage, chars: &[(char, i32)]) -> f64 {
	let indices: Vec<i32> = chars.iter().filter(|(c, _)| !c.is_whitespace()).map(|&(_, i)| i).collect();
	if indices.is_empty() {
		return 0.0;
	}
	let step = indices.len().div_ceil(LINE_FONT_SIZE_SAMPLES).max(1);
	let mut sizes: Vec<f64> =
		indices.iter().step_by(step).map(|&i| effective_font_size(text_page, i)).filter(|size| *size > 0.0).collect();
	sorted_median(&mut sizes)
}

/// Bit 1 of a PDF font descriptor's flags, which a font sets when all its glyphs are the
/// same width. Reliable when it is set and worth nothing when it is not: the Computer Modern
/// typewriter faces LaTeX sets code in leave it clear.
const FIXED_PITCH_FLAG: i32 = 1;

/// Whether a font name belongs to a monospaced face.
///
/// Names come subset-tagged as `ABCDEF+Consolas`, so the tag comes off first. `monotype` is
/// removed before looking for `mono` because it is a foundry name that says nothing about the
/// widths: Monotype Corsiva is a script face.
fn looks_monospaced(font_name: &str) -> bool {
	const HINTS: [&str; 6] = ["mono", "courier", "consol", "menlo", "typewriter", "inconsolata"];
	let name = font_name.rsplit('+').next().unwrap_or(font_name).to_ascii_lowercase();
	let name = name.replace("monotype", "");
	HINTS.iter().any(|hint| name.contains(hint))
		// Computer Modern, which is what a LaTeX document sets a listing in, names its
		// typewriter faces cmtt, cmitt, cmsltt and cmvtt. No other face in the family has a
		// double t in its name.
		|| (name.starts_with("cm") && name.contains("tt"))
}

/// Whether one character is set in a monospaced face, by the font's descriptor flags first and
/// its name second.
fn char_is_monospaced(text_page: &PdfTextPage, index: i32) -> bool {
	let Some(font) = text_page.font(index) else { return false };
	if font.flags & FIXED_PITCH_FLAG != 0 {
		return true;
	}
	looks_monospaced(&font.name)
}

/// Whether a line is set in a monospaced face, over the same handful of characters the size is
/// measured across. Most of them have to agree: one word of code quoted in a sentence of prose
/// does not make the sentence a listing.
fn line_is_monospaced(text_page: &PdfTextPage, chars: &[(char, i32)]) -> bool {
	let indices: Vec<i32> = chars.iter().filter(|(c, _)| !c.is_whitespace()).map(|&(_, i)| i).collect();
	if indices.is_empty() {
		return false;
	}
	let step = indices.len().div_ceil(LINE_FONT_SIZE_SAMPLES).max(1);
	let sampled: Vec<i32> = indices.iter().step_by(step).copied().collect();
	let monospaced = sampled.iter().filter(|&&i| char_is_monospaced(text_page, i)).count();
	monospaced * 2 > sampled.len()
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
		.filter(|line| !line.text.trim().is_empty() && line.size > 0.0)
		.map(|line| line.size)
		.collect();
	sorted_median(&mut sizes)
}

/// How far above a page's first line other text has to sit, as a share of the height its text spans, before the page counts as drawn out of order. A running header drawn after the body sits well inside this, and a second column starts about where the first did.
const OUT_OF_ORDER_SHARE: f64 = 0.25;

/// Puts a page whose content stream starts partway down it back into top-down order. Some form generators draw the lower half of a page first, and an untagged page has only the stream to go on, so the form read from its middle. Only such a page is touched, and it is reordered by region, each a stretch the stream draws without leaving that part of the page, kept whole and placed by where it starts: every page drawn in order, columns included, comes back unchanged. Lines the stream draws first at the bottom of everything drawn after them, such as a footer, are not where it starts.
fn restore_reading_order(lines: Vec<Line>) -> Vec<Line> {
	// A line with no position, such as an empty one, says nothing about the order and travels with the run it is drawn in.
	let placed = |line: &&Line| line.top.is_finite() && line.bottom.is_finite();
	// Walks back from the end keeping the lowest top drawn after each line, so this ends on the earliest line that has a lower one drawn after it.
	let mut lowest_after = f64::INFINITY;
	let mut first = None;
	for line in lines.iter().rev().filter(placed) {
		if line.top > lowest_after + COORDINATE_EPSILON {
			first = Some(line);
		}
		lowest_after = lowest_after.min(line.top);
	}
	let Some(first) = first else { return lines };
	let highest = lines.iter().filter(placed).map(|line| line.top).fold(f64::MIN, f64::max);
	let lowest = lines.iter().filter(placed).map(|line| line.bottom).fold(f64::MAX, f64::min);
	let far = (highest - lowest) * OUT_OF_ORDER_SHARE;
	if far <= 0.0 || highest - first.top < far {
		return lines;
	}
	let mut runs: Vec<(f64, Vec<Line>)> = Vec::new();
	for line in lines {
		// Only a jump across a good part of the page, up or down, leaves one region of it for another. A form steps back up a line or two all the time within a row, and the footer is drawn straight after whatever comes before it.
		let previous = runs.last().and_then(|(_, run)| run.iter().rev().find(placed));
		let jumps = placed(&&line) && previous.is_none_or(|prev| (line.top - prev.top).abs() > far);
		match runs.last_mut() {
			Some((_, run)) if !jumps => run.push(line),
			_ => runs.push((if placed(&&line) { line.top } else { f64::MAX }, vec![line])),
		}
	}
	runs.sort_by(|a, b| b.0.total_cmp(&a.0));
	runs.into_iter().flat_map(|(_, run)| run).collect()
}

#[cfg(test)]
mod tests {
	use super::{
		CharBox, Line, covering_hides, ends_line, looks_monospaced, restore_reading_order, sanitize_pdf_text,
		space_is_invisible,
	};

	fn line(text: &str, top: f64) -> Line {
		Line { text: text.to_string(), size: 10.0, top, bottom: top - 10.0, monospaced: false }
	}

	fn texts(lines: &[Line]) -> Vec<&str> {
		lines.iter().map(|line| line.text.as_str()).collect()
	}

	#[test]
	fn a_page_drawn_top_down_keeps_its_order() {
		let lines = vec![line("title", 700.0), line("one", 680.0), line("two", 660.0), line("three", 100.0)];
		assert_eq!(texts(&restore_reading_order(lines)), ["title", "one", "two", "three"]);
	}

	#[test]
	fn a_second_column_is_not_moved_ahead_of_the_first() {
		let lines = vec![
			line("left 1", 700.0),
			line("left 2", 400.0),
			line("left 3", 100.0),
			line("right 1", 702.0),
			line("right 2", 400.0),
		];
		assert_eq!(texts(&restore_reading_order(lines)), ["left 1", "left 2", "left 3", "right 1", "right 2"]);
	}

	#[test]
	fn a_running_header_drawn_last_stays_where_the_stream_put_it() {
		let lines = vec![line("body 1", 690.0), line("body 2", 400.0), line("body 3", 100.0), line("header", 740.0)];
		assert_eq!(texts(&restore_reading_order(lines)), ["body 1", "body 2", "body 3", "header"]);
	}

	/// The shape of the ADA dental claim form: the stream starts at the table halfway down, reaches the bottom, then goes back for the top half, a column at a time, and draws the title last.
	#[test]
	fn a_form_drawn_from_its_middle_is_read_from_the_top() {
		let lines = vec![
			line("24. Procedure Date", 460.0),
			line("35. Remarks", 330.0),
			line("37. Authorize", 240.0),
			line("48. Name", 130.0),
			line("1. Type of Transaction", 720.0),
			line("7. Gender", 610.0),
			line("M F", 600.0),
			line("8. Subscriber ID", 612.0),
			line("11. Other Insurance", 520.0),
			line("12. Policyholder Name", 690.0),
			line("23. Patient ID", 500.0),
			line("© American Dental Association", 40.0),
			line("Dental Claim Form", 760.0),
		];
		assert_eq!(
			texts(&restore_reading_order(lines)),
			[
				"Dental Claim Form",
				"1. Type of Transaction",
				"7. Gender",
				"M F",
				"8. Subscriber ID",
				"11. Other Insurance",
				"12. Policyholder Name",
				"23. Patient ID",
				"24. Procedure Date",
				"35. Remarks",
				"37. Authorize",
				"48. Name",
				"© American Dental Association",
			]
		);
	}

	/// The real form has an empty line with no position in it, which must not stop the page being put in order.
	#[test]
	fn a_line_with_no_position_does_not_keep_a_form_out_of_order() {
		let empty = Line {
			text: String::new(),
			size: 0.0,
			top: f64::NEG_INFINITY,
			bottom: f64::NEG_INFINITY,
			monospaced: false,
		};
		let lines = vec![
			line("24. Procedure Date", 450.0),
			empty,
			line("35. Remarks", 330.0),
			line("1. Type of Transaction", 720.0),
		];
		assert_eq!(
			texts(&restore_reading_order(lines)),
			["1. Type of Transaction", "24. Procedure Date", "", "35. Remarks"]
		);
	}

	/// The shape of a magazine page: the master page's footer lines come first, then the header, both columns top-down, and the title, intro and pull quote last. The intro sits above the first column, so the second column starts higher than the first.
	#[test]
	fn a_footer_drawn_first_does_not_reorder_the_columns() {
		let stream = [
			("folio", 38.0),
			("footer", 38.0),
			("header", 811.0),
			("left 1", 395.0),
			("left 2", 250.0),
			("left 3", 83.0),
			("right 1", 573.0),
			("right 2", 400.0),
			("right 3", 250.0),
			("right 4", 83.0),
			("title", 710.0),
			("lead", 585.0),
			("pull quote", 415.0),
		];
		let lines = stream.iter().map(|&(text, top)| line(text, top)).collect();
		let expected: Vec<&str> = stream.iter().map(|&(text, _)| text).collect();
		assert_eq!(texts(&restore_reading_order(lines)), expected);
	}

	#[test]
	fn a_form_with_its_footer_drawn_first_is_still_read_from_the_top() {
		let lines = vec![
			line("footer", 40.0),
			line("middle 1", 460.0),
			line("middle 2", 330.0),
			line("middle 3", 200.0),
			line("top 1", 720.0),
			line("top 2", 600.0),
		];
		assert_eq!(
			texts(&restore_reading_order(lines)),
			["top 1", "top 2", "middle 1", "middle 2", "middle 3", "footer"]
		);
	}

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

	/// Shares of covered spaces measured on pages of both PDFs: #808's, where a covered space is a split ligature, and #993's, whose glyph boxes overhang nearly every space.
	#[test]
	fn covering_hides_a_space_only_where_covered_spaces_are_rare() {
		assert!(covering_hides(4, 229), "#808 page 4");
		assert!(covering_hides(12, 362), "#808 page 26, its most");
		assert!(!covering_hides(338, 386), "#993 page 4");
		assert!(!covering_hides(95, 153), "#993 page 2, its fewest");
		assert!(covering_hides(0, 0), "a page with no spaces to measure");
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

	#[test]
	fn sanitize_pdf_text_strips_control_chars_and_soft_hyphens() {
		assert_eq!(sanitize_pdf_text("sugges\u{0002}tion\tline\r\nnext"), "suggestion\tline\r\nnext");
		assert_eq!(sanitize_pdf_text("hy\u{00AD}phen"), "hyphen");
	}

	#[test]
	fn monospaced_faces_are_known_by_name() {
		// What LaTeX sets a listing in, subset tag and all.
		assert!(looks_monospaced("CMTT9"));
		assert!(looks_monospaced("CMITT10"));
		assert!(looks_monospaced("ABCDEF+CMSLTT10"));
		assert!(looks_monospaced("Courier"));
		assert!(looks_monospaced("AAAAAA+Consolas-Bold"));
		assert!(looks_monospaced("DejaVuSansMono"));
		assert!(looks_monospaced("Menlo-Regular"));
	}

	#[test]
	fn body_faces_are_not_mistaken_for_monospaced_ones() {
		// Every face the three prose books of #813 and #826 are set in.
		for name in [
			"BaskOldFace",
			"AMDJKP+TimesNewRomanPSMT",
			"TimesNewRomanPS-BoldMT",
			"MVBoli",
			"ASJHEV+SymbolMT",
			"SabonLTStd-Roman",
			"UniversLTStd-Bold",
			"Helvetica",
			"ElectraLTStd-BoldCursive",
			"TradeGothicLTStd-BdCn20Obl",
			"ZapfDingbatsStd",
			"AlternateGothicNo2BT-Regular",
			"GrotesqueMT",
			"MetaNormalLF-Roman",
			// Monotype is a foundry, not a width. Corsiva is a script face.
			"MonotypeCorsiva",
			// Computer Modern roman, bold extended and sans, which are not the typewriter.
			"CMR10",
			"CMBX12",
			"CMSS10",
		] {
			assert!(!looks_monospaced(name), "{name} is not a monospaced face");
		}
	}
}
