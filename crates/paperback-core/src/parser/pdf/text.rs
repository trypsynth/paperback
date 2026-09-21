//! Reading the text off a PDF page: pdfium's per-character stream assembled into visual
//! lines, each reordered visual to logical so a right-to-left script comes out in reading
//! order, and each measured for the size and the face it is set in.
//!
//! What those lines mean is [`super::paragraphs`]. This module only reports what is on the
//! page. It is the fallback path for a page with no trustworthy structure tree (see
//! [`super::structure`]), and its [`sanitize_pdf_text`] helper is shared by the metadata and
//! table of contents readers as well.

use std::cmp::Ordering;

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

/// The top edge of one character, which is what gives a tagged block the height the structure
/// tree never records for it. See [`super::images::UnclaimedImages`].
pub(super) fn char_top(text_page: &PdfTextPage, index: i32) -> Option<f64> {
	text_page.char_box(index).map(|boxed| boxed.top)
}

/// [`space_is_invisible`] for the space at `index` of `text_page`, fetching only the geometry
/// each test actually needs. Costs three or four pdfium calls per space character and none at
/// all for anything else, so a page pays for it in proportion to its spaces rather than its
/// length - the per-character cost that #747 had to undo.
pub(super) fn is_invisible_space(text_page: &PdfTextPage, index: i32, char_count: i32) -> bool {
	let (Some(origin), Some(space_box)) = (text_page.char_origin(index), text_page.char_box(index)) else {
		return false;
	};
	let next_origin = if index + 1 < char_count { text_page.char_origin(index + 1) } else { None };
	if space_is_invisible(origin, space_box, next_origin, None) {
		return true;
	}
	let prev_box = if index > 0 { text_page.char_box(index - 1) } else { None };
	space_is_invisible(origin, space_box, None, prev_box)
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
		} else if ch != ' ' || !is_invisible_space(text_page, i, char_count) {
			current_chars.push((ch, i));
		}
		previous_char = Some(ch);
	}
	if !current_chars.is_empty() {
		result.push(measure_line(text_page, &current_chars));
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

#[cfg(test)]
mod tests {
	use super::{CharBox, ends_line, looks_monospaced, sanitize_pdf_text, space_is_invisible};

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
