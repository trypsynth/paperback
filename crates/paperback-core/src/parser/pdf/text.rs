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

pub(super) fn extract_text_lines(text_page: &PdfiumTextPage, page_index: i32) -> Vec<(String, f64)> {
	let Ok(char_count) = text_page.char_count() else {
		tracing::warn!(
			page_index,
			"page text char count unavailable, falling back to whole-page text blob, heading detection by font size will be degraded for this page"
		);
		let raw = sanitize_pdf_text(&text_page.full()).replace('\r', "");
		return raw.lines().map(|l| (l.to_string(), 0.0)).collect();
	};
	let mut result: Vec<(String, f64)> = Vec::new();
	// Chars of the current visual line with their pdfium index, so each line can be
	// reordered visual→logical (handles RTL scripts) before paragraph joining.
	let mut current_chars: Vec<(char, i32)> = Vec::new();
	let mut previous_char = None;
	for i in 0..char_count {
		let unicode = text_page.get_unicode(i);
		let Some(ch) = char::from_u32(unicode) else { continue };
		if ends_line(ch, previous_char) {
			let size = line_font_size(text_page, &current_chars);
			result.push((reorder_run(text_page, &mem::take(&mut current_chars)), size));
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
		result.push((reorder_run(text_page, &current_chars), size));
	}
	result
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

pub(super) fn median_line_font_size(line_infos: &[(String, f64)]) -> f64 {
	let mut sizes: Vec<f64> = line_infos
		.iter()
		.filter(|(text, size)| !text.trim().is_empty() && *size > 0.0)
		.map(|(_, size)| *size)
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
fn full_line_len(lines: &[(String, bool, f64)]) -> usize {
	let mut lengths: Vec<usize> = lines.iter().map(|(line, ..)| display_len(line)).filter(|len| *len > 0).collect();
	if lengths.is_empty() {
		return 0;
	}
	lengths.sort_unstable();
	lengths[(lengths.len() - 1) * FULL_LINE_PERCENTILE_NUMERATOR / FULL_LINE_PERCENTILE_DENOMINATOR]
}

pub(super) fn join_paragraphs(raw_lines: &[(String, f64)], body_font_size: f64) -> Vec<(String, bool)> {
	const HEADING_FONT_RATIO: f64 = 1.2;
	const HEADING_MAX_LEN: usize = 150;
	let heading_threshold = if body_font_size > 0.0 { body_font_size * HEADING_FONT_RATIO } else { f64::INFINITY };
	let lines: Vec<(String, bool, f64)> = raw_lines
		.iter()
		.map(|(text, size)| {
			let trimmed = trim_string(&collapse_whitespace(text));
			let len = display_len(&trimmed);
			let is_heading_line = *size >= heading_threshold && len > 0 && len <= HEADING_MAX_LEN;
			(trimmed, is_heading_line, *size)
		})
		.collect();
	// Two ways a line can end a paragraph, needing different amounts of evidence. A line that
	// ends a sentence and stops any way short of the measure has ended the paragraph with it. A
	// line that ends mid-sentence has to fall well short before the next line starting like a
	// fresh sentence counts for anything, since a capital there is as likely to open a name.
	let full_line = full_line_len(&lines);
	let ended_sentence_threshold = full_line * 19 / 20;
	let short_line_threshold = full_line * 3 / 4;
	let mut paragraphs: Vec<(String, bool)> = Vec::new();
	let mut current_paragraph = String::new();
	let mut current_is_heading = false;
	let mut current_heading_size = 0.0f64;
	let mut last_line_len = 0usize;
	let mut last_line_ends_with_punctuation = false;
	for (line, is_heading_line, size) in &lines {
		if line.is_empty() {
			if !current_paragraph.is_empty() {
				paragraphs.push((mem::take(&mut current_paragraph), current_is_heading));
				current_is_heading = false;
			}
			last_line_len = 0;
			last_line_ends_with_punctuation = false;
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
			// than becoming one heading per line, as long as the size does not change. Every
			// other heading boundary, and every list item, starts a new paragraph; otherwise the
			// previous line does when it stopped short of the measure.
			let continues_heading =
				*is_heading_line && current_is_heading && (*size - current_heading_size).abs() < f64::EPSILON;
			let previous_line_ended_paragraph = (last_line_ends_with_punctuation
				&& last_line_len < ended_sentence_threshold)
				|| (last_line_len < short_line_threshold && (starts_with_uppercase || !starts_with_alpha));
			let break_paragraph = !continues_heading
				&& (*is_heading_line
					|| current_is_heading
					|| is_list_item || is_numbered
					|| previous_line_ended_paragraph);
			if break_paragraph {
				paragraphs.push((mem::take(&mut current_paragraph), current_is_heading));
				current_paragraph = line.clone();
				current_is_heading = *is_heading_line;
				current_heading_size = *size;
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
		paragraphs.push((current_paragraph, current_is_heading));
	}
	paragraphs
}

#[cfg(test)]
mod tests {
	use super::{CharBox, ends_line, full_line_len, join_paragraphs, sanitize_pdf_text, space_is_invisible};

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
	fn issue_813_page() -> Vec<(String, f64)> {
		[
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
			("Ministry of Aviation Industries of China (AVIC) in Beijing, discussing human resources for a potential joint venture between", 11.0),
			("AlliedSignal, the company for which I was then international", 11.0),
			("human resources vice president, and AVIC. When the meeting", 11.0),
			("ended, one of my hosts from the ministry graciously accompanied me outside. As we waited for the car, he noticed that I was", 11.0),
			("looking up and down the street and then skyward. Visibility was", 11.0),
			("only about a hundred yards in any direction. I was shocked. The", 11.0),
			("executive looked at me and said, \u{201C}My children do not know the", 11.0),
			("sky is blue.\u{201D}", 11.0),
		]
		.into_iter()
		.map(|(text, size)| (text.to_string(), size))
		.collect()
	}

	/// The page's measure comes from a line that fills it, not from one of the doubled lines
	/// pdfium produced, which are nearly twice as long.
	#[test]
	fn full_line_len_ignores_doubled_lines() {
		let lines: Vec<(String, bool, f64)> =
			issue_813_page().into_iter().map(|(text, size)| (text, false, size)).collect();
		let full = full_line_len(&lines);
		assert!((53..=64).contains(&full), "expected the length of a full line, got {full}");
	}

	/// #813: the body paragraph survives its second line opening with a year and its later lines
	/// opening with capitals, and the chapter title is one heading rather than one per line.
	#[test]
	fn join_paragraphs_keeps_a_chapter_opening_whole() {
		let result = join_paragraphs(&issue_813_page(), 11.0);
		let texts: Vec<&str> = result.iter().map(|(text, _)| text.as_str()).collect();
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

	/// The chapter number is set in its own size, so it does not join the title below it.
	#[test]
	fn join_paragraphs_keeps_headings_of_different_sizes_apart() {
		let lines = vec![("PART ONE".to_string(), 30.0), ("Getting Started".to_string(), 20.0)];
		let result = join_paragraphs(&lines, 11.0);
		assert_eq!(result.len(), 2);
	}

	/// A line that ends a sentence ends the paragraph even when it reaches most of the way across
	/// the measure - the case a single short-line threshold gets wrong.
	#[test]
	fn join_paragraphs_breaks_after_a_sentence_that_nearly_fills_the_line() {
		let lines = vec![
			("This book is part of the English for Research series of guides for non-native English".to_string(), 10.0),
			("academics of all disciplines who work in an international field.".to_string(), 10.0),
			("EAP trainers can use this book in conjunction with: English for Academic Research:".to_string(), 10.0),
			("A Guide for Teachers.".to_string(), 10.0),
		];
		let result = join_paragraphs(&lines, 10.0);
		assert_eq!(result.len(), 2, "got {result:?}");
		assert!(result[0].0.ends_with("international field."));
		assert!(result[1].0.starts_with("EAP trainers"));
	}

	/// A paragraph that pdfium hands back as wrapped lines is one paragraph, not one per line.
	#[test]
	fn join_paragraphs_merges_a_wrapped_list_item() {
		let lines = vec![
			("1) Look at your Inbox in your email account. Analyse 10-20 subject lines and".to_string(), 9.5),
			("decide some criteria for judging how effective the subject lines are. Compare".to_string(), 9.5),
			("your criteria with a colleague's.".to_string(), 9.5),
		];
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
		let lines = vec![("The suggestion appears here.".to_string(), 12.0), ("And here.".to_string(), 12.0)];
		let result = join_paragraphs(&lines, 12.0);
		assert_eq!(result.len(), 1);
		assert_eq!(result[0].0, "The suggestion appears here. And here.");
		assert!(!result[0].1);
	}

	#[test]
	fn join_paragraphs_flags_large_font_lines_as_headings() {
		let lines =
			vec![("Chapter One".to_string(), 18.0), ("This is the body text of the document.".to_string(), 12.0)];
		let result = join_paragraphs(&lines, 12.0);
		assert_eq!(result.len(), 2);
		assert_eq!(result[0].0, "Chapter One");
		assert!(result[0].1);
		assert_eq!(result[1].0, "This is the body text of the document.");
		assert!(!result[1].1);
	}
}
