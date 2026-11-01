use std::{
	cell::Cell,
	cmp::Ordering,
	collections::{HashMap, HashSet},
};

use crate::{
	parser::util::bidi,
	pdf_engine::{CharBox, PdfTextPage},
};

pub(super) fn sanitize_pdf_text(input: &str) -> String {
	let chars: Vec<char> = input.chars().collect();
	let evidence = HyphenEvidence::from_chars(&chars);
	let mut out = String::with_capacity(input.len());
	for (i, &ch) in chars.iter().enumerate() {
		if ch == LINE_END_HYPHEN {
			let (left, right) = line_end_halves(&chars, i);
			if keeps_hyphen(&left, &right, Some(&evidence)) {
				out.push('-');
			}
		} else if (!ch.is_control() || matches!(ch, '\n' | '\r' | '\t')) && ch != '\u{00AD}' {
			out.push(ch);
		}
	}
	out
}

pub(super) fn page_chars(text_page: &PdfTextPage, char_count: i32) -> Vec<Option<char>> {
	(0..char_count).map(|i| char::from_u32(text_page.unicode_at(i))).collect()
}

pub(super) fn context_chars(page: &[Option<char>]) -> Vec<char> {
	page.iter().map(|c| c.unwrap_or(' ')).collect()
}

/// pdfium's stand-in for a hyphen ending a line, which it takes for one that splits a word.
pub(super) const LINE_END_HYPHEN: char = '\u{0002}';

#[derive(Default)]
pub(super) struct HyphenEvidence {
	hyphenated: HashSet<String>,
	words: HashSet<String>,
}

impl HyphenEvidence {
	pub(super) fn from_chars(chars: &[char]) -> Self {
		let mut evidence = Self::default();
		evidence.add_chars(chars);
		evidence
	}

	pub(super) fn add_chars(&mut self, chars: &[char]) {
		let evidence = self;
		let mut word = String::new();
		let mut broken = false;
		for &ch in chars.iter().chain(std::iter::once(&' ')) {
			if ch.is_alphanumeric() || ch == '-' {
				word.extend(ch.to_lowercase());
				continue;
			}
			if ch == LINE_END_HYPHEN {
				// A word broken at a line end is evidence for itself neither way.
				broken = true;
				word.clear();
				continue;
			}
			let token = word.trim_matches('-');
			if !broken && !token.is_empty() {
				if token.contains('-') {
					evidence.hyphenated.insert(token.to_string());
				} else {
					evidence.words.insert(token.to_string());
				}
			}
			broken = false;
			word.clear();
		}
	}
}

#[derive(Clone, Copy, Default)]
struct Written {
	hyphenated: bool,
	whole: bool,
}

impl Written {
	fn of(evidence: &HyphenEvidence, hyphenated: &str, whole: &str) -> Self {
		Self { hyphenated: evidence.hyphenated.contains(hyphenated), whole: evidence.words.contains(whole) }
	}

	const fn or(self, other: Self) -> Self {
		Self { hyphenated: self.hyphenated || other.hyphenated, whole: self.whole || other.whole }
	}
}

#[derive(Default)]
pub(super) struct WordPages {
	hyphenated: HashMap<String, i32>,
	words: HashMap<String, i32>,
}

impl WordPages {
	/// A thread reads its pages in order, so the first page a word is added for is the first of them it is written on.
	pub(super) fn add(&mut self, page: i32, evidence: HyphenEvidence) {
		for word in evidence.hyphenated {
			self.hyphenated.entry(word).or_insert(page);
		}
		for word in evidence.words {
			self.words.entry(word).or_insert(page);
		}
	}
}

fn written_before(pages: &[WordPages], page: i32, hyphenated: &str, whole: &str) -> Written {
	let before = |words: &HashMap<String, i32>, word: &str| words.get(word).is_some_and(|&first| first < page);
	Written {
		hyphenated: pages.iter().any(|words| before(&words.hyphenated, hyphenated)),
		whole: pages.iter().any(|words| before(&words.words, whole)),
	}
}

/// Kept so the judgement can be checked against every earlier page once all of them are read.
pub(super) struct Judgement {
	left: String,
	right: String,
	/// The one part the check cannot look up afterwards.
	own: Written,
	kept: bool,
}

pub(super) struct PageEvidence<'a> {
	page: i32,
	earlier: &'a [WordPages],
	own: HyphenEvidence,
	judgements: Vec<Judgement>,
}

impl<'a> PageEvidence<'a> {
	pub(super) fn new(page: i32, earlier: &'a [WordPages]) -> Self {
		Self { page, earlier, own: HyphenEvidence::default(), judgements: Vec::new() }
	}

	pub(super) fn add_chars(&mut self, chars: &[char]) {
		self.own.add_chars(chars);
	}

	pub(super) fn keeps_line_end_hyphen(&mut self, chars: &[char], at: usize) -> bool {
		let (left, right) = line_end_halves(chars, at);
		if left.is_empty() || right.is_empty() {
			return false;
		}
		let (hyphenated, whole) = written_forms(&left, &right);
		let own = Written::of(&self.own, &hyphenated, &whole);
		let kept =
			keeps_written_hyphen(&left, &right, own.or(written_before(self.earlier, self.page, &hyphenated, &whole)));
		self.judgements.push(Judgement { left, right, own, kept });
		kept
	}

	pub(super) fn finish(self) -> (HyphenEvidence, Vec<Judgement>) {
		(self.own, self.judgements)
	}
}

/// Once every page is read, `pages` judge a hyphen as reading the pages in order would have.
pub(super) fn confirms(judgements: &[Judgement], page: i32, pages: &[WordPages]) -> bool {
	judgements.iter().all(|judgement| {
		let (hyphenated, whole) = written_forms(&judgement.left, &judgement.right);
		let written = judgement.own.or(written_before(pages, page, &hyphenated, &whole));
		keeps_written_hyphen(&judgement.left, &judgement.right, written) == judgement.kept
	})
}

fn line_end_halves(chars: &[char], at: usize) -> (String, String) {
	let left_start = chars[..at].iter().rposition(|c| !c.is_alphanumeric()).map_or(0, |p| p + 1);
	let left: String = chars[left_start..at].iter().collect();
	let right_start =
		chars[at + 1..].iter().position(|c| !c.is_whitespace() && !c.is_control()).map_or(chars.len(), |p| at + 1 + p);
	let right: String = chars[right_start..].iter().take_while(|c| c.is_alphanumeric()).collect();
	(left, right)
}

fn written_forms(left: &str, right: &str) -> (String, String) {
	let (l, r) = (left.to_lowercase(), right.to_lowercase());
	(format!("{l}-{r}"), format!("{l}{r}"))
}

/// What the document writes elsewhere decides first; failing that, a hyphen beside an abbreviation, a capital or a number belongs to a compound (`NFB-NEWSLINE`, `COVID-19`).
pub(super) fn keeps_hyphen(left: &str, right: &str, evidence: Option<&HyphenEvidence>) -> bool {
	if left.is_empty() || right.is_empty() {
		return false;
	}
	let written = evidence.map_or_else(Written::default, |evidence| {
		let (hyphenated, whole) = written_forms(left, right);
		Written::of(evidence, &hyphenated, &whole)
	});
	keeps_written_hyphen(left, right, written)
}

fn keeps_written_hyphen(left: &str, right: &str, written: Written) -> bool {
	if written.hyphenated {
		return true;
	}
	if written.whole {
		return false;
	}
	let left_is_abbreviation =
		left.chars().count() >= 2 && left.chars().all(|c| c.is_uppercase() || c.is_ascii_digit());
	let right_opens_a_name = right.chars().next().is_some_and(|c| c.is_uppercase() || c.is_ascii_digit());
	left_is_abbreviation || right_opens_a_name || left.chars().any(|c| c.is_ascii_digit())
}

#[derive(Clone, Debug)]
pub(super) struct Line {
	pub text: String,
	pub size: f64,
	pub top: f64,
	pub bottom: f64,
	pub monospaced: bool,
	pub bold: bool,
}

/// pdfium reports characters on one line with identical coordinates, so this only has to absorb rounding.
const COORDINATE_EPSILON: f64 = 0.05;
/// Real spaces on a page measured for #808 all advance by at least 70% of their width; the spurious ones by 0-5%.
const NO_ADVANCE_RATIO: f64 = 0.25;
/// The same page measured 58% at most for real spaces (an `f` overhangs the space after it) and 92% at least for the spurious ones.
const SWALLOWED_RATIO: f64 = 0.8;
/// Enough to see the font's habit, few enough that the lookups stay a fixed cost per page.
const COVER_SAMPLE: usize = 64;

/// Two shapes reach us as ordinary spaces: one with no advance, which #808 leaves mid-word ("Pref ace") and before punctuation, and one drawn inside a split `fi`/`fl` ligature ("fi eld").
/// A neighbour on another line fails both checks, as it should for a space at either end of a line.
pub(super) fn space_is_invisible(
	space_origin: (f64, f64),
	space_box: CharBox,
	next_origin: Option<(f64, f64)>,
	prev_box: Option<CharBox>,
) -> bool {
	let width = space_box.right - space_box.left;
	if width <= 0.0 {
		// Fonts that give the space glyph no width and position words by hand land here, and their spaces are the page's only word separators.
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

fn covers(prev: CharBox, space_box: CharBox) -> bool {
	let width = space_box.right - space_box.left;
	let on_the_same_line =
		prev.bottom <= space_box.bottom + COORDINATE_EPSILON && prev.top >= space_box.top - COORDINATE_EPSILON;
	let covered_width = prev.right - space_box.left;
	width > 0.0 && on_the_same_line && covered_width >= width * SWALLOWED_RATIO
}

/// A split ligature before a space is rare (at most 4% of a page's spaces in #808), but a font whose glyph boxes outrun their advances covers 64% to 91% a page in #993, and there it says nothing.
fn covering_hides(covered: usize, measured: usize) -> bool {
	covered * 4 <= measured
}

/// The structure tree never records a block's height, so its first character's top stands in.
pub(super) fn char_top(text_page: &PdfTextPage, index: i32) -> Option<f64> {
	text_page.char_box(index).map(|boxed| boxed.top)
}

/// Fetches geometry only for space characters, so a page pays in proportion to its spaces rather than its length, the per-character cost #747 had to undo.
pub(super) struct SpaceFilter {
	char_count: i32,
	/// Measured the first time a covered space turns up, so a page without one never pays for it.
	covering_hides: Cell<Option<bool>>,
}

impl SpaceFilter {
	pub(super) const fn new(char_count: i32) -> Self {
		Self { char_count, covering_hides: Cell::new(None) }
	}

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

/// pdfium writes a line break as `"\r\n"`, and counting both ended every line twice, splitting every wrapped paragraph into one per line (#808).
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

#[allow(clippy::cast_possible_truncation, reason = "a page coordinate fits an f32")]
fn char_x_origin(text_page: &PdfTextPage, i: i32) -> f32 {
	text_page.char_origin(i).map_or(0.0, |(x, _)| x as f32)
}

/// Fetches x origins (a lookup per char) only for runs containing an RTL character, so LTR runs pay a single classification scan.
pub(super) fn reorder_run(text_page: &PdfTextPage, chars: &[(char, i32)]) -> String {
	if !bidi::contains_rtl(chars.iter().map(|&(c, _)| c)) {
		return chars.iter().map(|&(c, _)| c).collect();
	}
	let with_origin: Vec<(char, f32)> = chars.iter().map(|&(c, i)| (c, char_x_origin(text_page, i))).collect();
	bidi::reorder_line(&with_origin)
}

fn measure_line(text_page: &PdfTextPage, chars: &[(char, i32)]) -> Line {
	let (top, bottom) = line_edges(text_page, chars);
	Line {
		size: line_font_size(text_page, chars),
		monospaced: line_is_monospaced(text_page, chars),
		bold: line_is_bold(text_page, chars),
		text: reorder_run(text_page, chars),
		top,
		bottom,
	}
}

pub(super) fn extract_text_lines(text_page: &PdfTextPage, evidence: &mut PageEvidence) -> Vec<Line> {
	let char_count = text_page.char_count();
	let mut result: Vec<Line> = Vec::new();
	// Kept with their pdfium index so each line can be reordered visual to logical before paragraph joining.
	let mut current_chars: Vec<(char, i32)> = Vec::new();
	let mut previous_char = None;
	let spaces = SpaceFilter::new(char_count);
	let page = page_chars(text_page, char_count);
	let context = context_chars(&page);
	evidence.add_chars(&context);
	for (at, (i, &ch)) in (0..char_count).zip(page.iter()).enumerate() {
		let Some(ch) = ch else { continue };
		if ends_line(ch, previous_char) {
			let line = measure_line(text_page, &current_chars);
			current_chars.clear();
			result.push(line);
		} else if ch == LINE_END_HYPHEN {
			if evidence.keeps_line_end_hyphen(&context, at) {
				current_chars.push(('-', i));
			}
		} else if ch == '\n' || (ch.is_control() && !matches!(ch, '\t')) || ch == '\u{00AD}' {
			// Dropped, but still the previous character as far as the next `ends_line` is concerned.
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

/// The top is the first character's, since a few points decide nothing about where an image falls; the bottom must be the lowest, since the next line's gap is measured from it and pdfium's doubled lines reach a whole line lower.
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

/// The `Tf` size alone is 1.0 in PDFs that scale text through the matrix instead (#808 and #813 both do), so no line was ever tall enough to be a heading.
fn effective_font_size(text_page: &PdfTextPage, index: i32) -> f64 {
	let size = text_page.font_size(index);
	text_page.text_matrix(index).map_or(size, |matrix| size * f64::from(matrix.b.hypot(matrix.d)))
}

/// The first character alone believes a drop cap three times the line's size; a median of a few does not, and stays a per-line cost rather than the per-character one #747 had to undo.
const LINE_FONT_SIZE_SAMPLES: usize = 5;

/// Whitespace is skipped because a font may set it in a size of its own.
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

/// Reliable when set and worth nothing when not: the Computer Modern typewriter faces LaTeX sets code in leave it clear.
const FIXED_PITCH_FLAG: i32 = 1;

/// `monotype` comes out before looking for `mono` because it is a foundry, not a width: Monotype Corsiva is a script face.
fn looks_monospaced(font_name: &str) -> bool {
	const HINTS: [&str; 6] = ["mono", "courier", "consol", "menlo", "typewriter", "inconsolata"];
	let name = font_name.rsplit('+').next().unwrap_or(font_name).to_ascii_lowercase();
	let name = name.replace("monotype", "");
	HINTS.iter().any(|hint| name.contains(hint))
		// Computer Modern's typewriter faces are cmtt, cmitt, cmsltt and cmvtt; no other face in the family has a double t.
		|| (name.starts_with("cm") && name.contains("tt"))
}

fn char_is_monospaced(text_page: &PdfTextPage, index: i32) -> bool {
	let Some(font) = text_page.font(index) else { return false };
	if font.flags & FIXED_PITCH_FLAG != 0 {
		return true;
	}
	looks_monospaced(&font.name)
}

const BOLD_WEIGHT: i32 = 600;

fn looks_bold(font_name: &str) -> bool {
	const HINTS: [&str; 5] = ["bold", "black", "heavy", "semibold", "demi"];
	let name = font_name.rsplit('+').next().unwrap_or(font_name).to_ascii_lowercase();
	HINTS.iter().any(|hint| name.contains(hint))
}

fn char_is_bold(text_page: &PdfTextPage, index: i32) -> bool {
	if let Some(weight) = text_page.font_weight(index) {
		return weight >= BOLD_WEIGHT;
	}
	text_page.font(index).is_some_and(|font| looks_bold(&font.name))
}

/// Every sampled character has to be bold: a paragraph that opens with a bold word is not a heading.
fn line_is_bold(text_page: &PdfTextPage, chars: &[(char, i32)]) -> bool {
	let indices: Vec<i32> = chars.iter().filter(|(c, _)| !c.is_whitespace()).map(|&(_, i)| i).collect();
	if indices.is_empty() {
		return false;
	}
	let step = indices.len().div_ceil(LINE_FONT_SIZE_SAMPLES).max(1);
	indices.iter().step_by(step).all(|&i| char_is_bold(text_page, i))
}

/// Most sampled characters have to agree: one word of code quoted in prose does not make the sentence a listing.
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

/// A running header drawn after the body sits well inside this, and a second column starts about where the first did.
const OUT_OF_ORDER_SHARE: f64 = 0.25;

/// Some form generators draw a page's lower half first, so the form read from its middle; whole regions are moved, so a page drawn in order, columns and a footer drawn first included, comes back unchanged.
fn restore_reading_order(lines: Vec<Line>) -> Vec<Line> {
	// A line with no position, such as an empty one, says nothing about the order and travels with the run it is drawn in.
	let placed = |line: &&Line| line.top.is_finite() && line.bottom.is_finite();
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
		CharBox, Line, PageEvidence, WordPages, confirms, covering_hides, ends_line, keeps_hyphen, looks_monospaced,
		restore_reading_order, sanitize_pdf_text, space_is_invisible,
	};

	/// The SELVAS NFB-NEWSLINE guide and the Mantis Q40 conformance report both lost real hyphens here.
	#[test]
	fn line_end_hyphens_are_kept_only_where_they_belong_to_the_word() {
		assert_eq!(sanitize_pdf_text("sugges\u{0002}tion"), "suggestion");
		assert_eq!(
			sanitize_pdf_text("We have developed NFB\u{0002}NEWSLINE apps"),
			"We have developed NFB-NEWSLINE apps"
		);
		assert_eq!(sanitize_pdf_text("a COVID\u{0002}19 update"), "a COVID-19 update");
		// The page writes "non-web" elsewhere, so the wrapped one keeps its hyphen too.
		assert_eq!(
			sanitize_pdf_text("non-web software or non\u{0002}web documents"),
			"non-web software or non-web documents"
		);
		// The page writes the word whole elsewhere, so the split is joined even next to a capital.
		assert_eq!(sanitize_pdf_text("Braille notes. Braille\u{0002}Note"), "Braille notes. Braille-Note");
		assert_eq!(sanitize_pdf_text("BrailleNote and Braille\u{0002}Note"), "BrailleNote and BrailleNote");
	}

	/// Page 0 writes "OpenSolaris" whole, so a thread that never saw it must not keep the hyphen in page 1's "Open-Solaris".
	#[test]
	fn a_hyphen_judged_without_an_earlier_page_is_caught() {
		let chars = |text: &str| text.chars().collect::<Vec<char>>();
		let mut first = PageEvidence::new(0, &[]);
		first.add_chars(&chars("Built on OpenSolaris."));
		let mut every_page = WordPages::default();
		every_page.add(0, first.finish().0);
		let every_page = std::slice::from_ref(&every_page);
		let wrapped = chars("Open\u{0002}Solaris");
		let mut alone = PageEvidence::new(1, &[]);
		alone.add_chars(&wrapped);
		assert!(alone.keeps_line_end_hyphen(&wrapped, 4));
		assert!(!confirms(&alone.finish().1, 1, every_page));
		let mut in_order = PageEvidence::new(1, every_page);
		in_order.add_chars(&wrapped);
		assert!(!in_order.keeps_line_end_hyphen(&wrapped, 4));
		assert!(confirms(&in_order.finish().1, 1, every_page));
	}

	#[test]
	fn hyphens_without_evidence_follow_the_shape_of_the_word() {
		assert!(!keeps_hyphen("infor", "mation", None));
		assert!(keeps_hyphen("NFB", "NEWSLINE", None));
		assert!(keeps_hyphen("Victor", "Reader", None));
		assert!(keeps_hyphen("MP3", "player", None));
		assert!(!keeps_hyphen("", "word", None), "nothing before the hyphen: not a compound");
	}

	fn line(text: &str, top: f64) -> Line {
		Line { text: text.to_string(), size: 10.0, top, bottom: top - 10.0, monospaced: false, bold: false }
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
			bold: false,
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

	/// The coordinates in these tests are what pdfium reports for the PDF attached to #808.
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

	/// "name of paper": an `f` overhangs the space after it by more than half its width, but that space advances normally.
	#[test]
	fn space_after_an_overhanging_glyph_is_kept() {
		assert!(!space_is_invisible(
			(105.09, 387.61),
			char_box(105.09, 107.59, 387.61, 387.62),
			Some((107.59, 387.61)),
			Some(char_box(102.50, 106.54, 387.61, 394.44))
		));
	}

	#[test]
	fn neighbours_on_another_line_are_ignored() {
		let space = char_box(287.34, 289.84, 387.61, 387.62);
		assert!(!space_is_invisible((287.34, 387.61), space, Some((287.34, 375.61)), None));
		assert!(!space_is_invisible((287.34, 387.61), space, None, Some(char_box(284.87, 289.77, 375.61, 382.44))));
	}

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
