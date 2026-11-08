use std::{
	cell::Cell,
	collections::{HashMap, HashSet},
};

use smallvec::SmallVec;

use crate::{
	parser::{
		layout::{COORDINATE_EPSILON, Line, keeps_line_end_hyphen, restore_reading_order, sorted_median},
		util::bidi,
	},
	pdf_engine::{FontFlags, PageChar, Rect, TextPage},
};

pub(super) fn sanitize_pdf_text(input: &str) -> String {
	let chars: Vec<char> = input.chars().collect();
	let evidence = HyphenEvidence::from_chars(&chars);
	let mut out = String::with_capacity(input.len());
	for (i, &ch) in chars.iter().enumerate() {
		if ch == LINE_END_HYPHEN {
			let (left, right) = line_end_halves(&chars, i);
			if keeps_hyphen(&left, &right, &evidence) {
				out.push('-');
			}
		} else if (!ch.is_control() || matches!(ch, '\n' | '\r' | '\t')) && ch != '\u{00AD}' {
			out.push(ch);
		}
	}
	out
}

pub(super) fn page_chars(text_page: &TextPage) -> Vec<Option<char>> {
	text_page.chars().map(|ch| char::from_u32(ch.unicode())).collect()
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
	hyphenated: HashMap<String, usize>,
	words: HashMap<String, usize>,
}

impl WordPages {
	/// A thread reads its pages in order, so the first page a word is added for is the first of them it is written on.
	pub(super) fn add(&mut self, page: usize, evidence: HyphenEvidence) {
		for word in evidence.hyphenated {
			self.hyphenated.entry(word).or_insert(page);
		}
		for word in evidence.words {
			self.words.entry(word).or_insert(page);
		}
	}
}

fn written_before(pages: &[WordPages], page: usize, hyphenated: &str, whole: &str) -> Written {
	let before = |words: &HashMap<String, usize>, word: &str| words.get(word).is_some_and(|&first| first < page);
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
	page: usize,
	earlier: &'a [WordPages],
	own: HyphenEvidence,
	judgements: Vec<Judgement>,
}

impl<'a> PageEvidence<'a> {
	pub(super) fn new(page: usize, earlier: &'a [WordPages]) -> Self {
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
pub(super) fn confirms(judgements: &[Judgement], page: usize, pages: &[WordPages]) -> bool {
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

/// What the document writes elsewhere decides first; failing that, the word's shape does.
fn keeps_hyphen(left: &str, right: &str, evidence: &HyphenEvidence) -> bool {
	if left.is_empty() || right.is_empty() {
		return false;
	}
	let (hyphenated, whole) = written_forms(left, right);
	keeps_written_hyphen(left, right, Written::of(evidence, &hyphenated, &whole))
}

fn keeps_written_hyphen(left: &str, right: &str, written: Written) -> bool {
	if written.hyphenated {
		return true;
	}
	if written.whole {
		return false;
	}
	keeps_line_end_hyphen(left, right)
}

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
	space_box: Rect,
	next_origin: Option<(f64, f64)>,
	prev_box: Option<Rect>,
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

fn covers(prev: Rect, space_box: Rect) -> bool {
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
pub(super) fn char_top(text_page: &TextPage, index: usize) -> Option<f64> {
	text_page.char(index).map(|ch| ch.bounds().top)
}

/// Fetches geometry only for space characters, so a page pays in proportion to its spaces rather than its length, the per-character cost #747 had to undo.
pub(super) struct SpaceFilter {
	char_count: usize,
	/// Measured the first time a covered space turns up, so a page without one never pays for it.
	covering_hides: Cell<Option<bool>>,
}

impl SpaceFilter {
	pub(super) const fn new(char_count: usize) -> Self {
		Self { char_count, covering_hides: Cell::new(None) }
	}

	pub(super) fn hides(&self, text_page: &TextPage, index: usize) -> bool {
		let Some(space) = text_page.char(index) else { return false };
		let origin = space.origin();
		let (origin, space_box) = ((origin.x, origin.y), space.bounds());
		let next_origin = text_page.char(index + 1).map(PageChar::origin).map(|next| (next.x, next.y));
		if space_is_invisible(origin, space_box, next_origin, None) {
			return true;
		}
		let prev_box = index.checked_sub(1).and_then(|prev| text_page.char(prev)).map(PageChar::bounds);
		space_is_invisible(origin, space_box, None, prev_box) && self.covering_hides(text_page)
	}

	fn covering_hides(&self, text_page: &TextPage) -> bool {
		if let Some(hides) = self.covering_hides.get() {
			return hides;
		}
		let (mut measured, mut covered) = (0, 0);
		for index in 1..self.char_count {
			if measured == COVER_SAMPLE {
				break;
			}
			let (Some(space), Some(prev)) = (text_page.char(index), text_page.char(index - 1)) else { continue };
			if space.unicode() != u32::from(' ') {
				continue;
			}
			measured += 1;
			if covers(prev.bounds(), space.bounds()) {
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

#[allow(clippy::cast_possible_truncation, reason = "a page coordinate fits an f32")]
fn char_x_origin(text_page: &TextPage, i: usize) -> f32 {
	text_page.char(i).map_or(0.0, |ch| ch.origin().x as f32)
}

/// Fetches x origins (a lookup per char) only for runs containing an RTL character, so LTR runs pay a single classification scan.
pub(super) fn reorder_run(text_page: &TextPage, chars: &[(char, usize)]) -> String {
	if !bidi::contains_rtl(chars.iter().map(|&(c, _)| c)) {
		return chars.iter().map(|&(c, _)| c).collect();
	}
	let with_origin: Vec<(char, f32)> = chars.iter().map(|&(c, i)| (c, char_x_origin(text_page, i))).collect();
	bidi::reorder_line(&with_origin)
}

fn measure_line(text_page: &TextPage, chars: &[(char, usize)]) -> Line {
	let (top, bottom) = line_edges(text_page, chars);
	let sampled = sampled_chars(chars);
	Line {
		size: line_font_size(text_page, &sampled),
		monospaced: line_is_monospaced(text_page, &sampled),
		bold: line_is_bold(text_page, &sampled),
		text: reorder_run(text_page, chars),
		top,
		bottom,
	}
}

pub(super) fn extract_text_lines(text_page: &TextPage, evidence: &mut PageEvidence) -> Vec<Line> {
	let char_count = text_page.char_count();
	let mut result: Vec<Line> = Vec::new();
	// Kept with their pdfium index so each line can be reordered visual to logical before paragraph joining.
	let mut current_chars: Vec<(char, usize)> = Vec::new();
	let mut previous_char = None;
	let spaces = SpaceFilter::new(char_count);
	let page = page_chars(text_page);
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
fn line_edges(text_page: &TextPage, chars: &[(char, usize)]) -> (f64, f64) {
	let top =
		chars.first().and_then(|(_, index)| text_page.char(*index)).map_or(f64::NEG_INFINITY, |ch| ch.bounds().top);
	let mut bottom = f64::INFINITY;
	for (_, index) in chars {
		if let Some(ch) = text_page.char(*index) {
			bottom = bottom.min(ch.bounds().bottom);
		}
	}
	(top, bottom)
}

/// The `Tf` size alone is 1.0 in PDFs that scale text through the matrix instead (#808 and #813 both do), so no line was ever tall enough to be a heading.
fn effective_font_size(text_page: &TextPage, index: usize) -> f64 {
	text_page.char(index).map_or(0.0, |ch| {
		let matrix = ch.text_matrix();
		ch.font_size() * f64::from(matrix.b.hypot(matrix.d))
	})
}

/// The first character alone believes a drop cap three times the line's size; a median of a few does not, and stays a per-line cost rather than the per-character one #747 had to undo.
const LINE_FONT_SIZE_SAMPLES: usize = 5;

/// Whitespace is skipped because a font may set it in a size of its own.
/// The line's non-space characters at an even step, the same few for its size, its boldness and its spacing; each used to collect the whole line again.
fn sampled_chars(chars: &[(char, usize)]) -> SmallVec<[usize; LINE_FONT_SIZE_SAMPLES + 1]> {
	let printing = || chars.iter().filter(|(c, _)| !c.is_whitespace()).map(|&(_, i)| i);
	let step = printing().count().div_ceil(LINE_FONT_SIZE_SAMPLES).max(1);
	printing().step_by(step).collect()
}

fn line_font_size(text_page: &TextPage, sampled: &[usize]) -> f64 {
	if sampled.is_empty() {
		return 0.0;
	}
	let mut sizes: SmallVec<[f64; LINE_FONT_SIZE_SAMPLES + 1]> =
		sampled.iter().map(|&i| effective_font_size(text_page, i)).filter(|size| *size > 0.0).collect();
	sorted_median(&mut sizes)
}

/// `monotype` comes out before looking for `mono` because it is a foundry, not a width: Monotype Corsiva is a script face.
fn looks_monospaced(font_name: &str) -> bool {
	const HINTS: [&str; 6] = ["mono", "courier", "consol", "menlo", "typewriter", "inconsolata"];
	let name = font_name.rsplit('+').next().unwrap_or(font_name).to_ascii_lowercase();
	let name = name.replace("monotype", "");
	HINTS.iter().any(|hint| name.contains(hint))
		// Computer Modern's typewriter faces are cmtt, cmitt, cmsltt and cmvtt; no other face in the family has a double t.
		|| (name.starts_with("cm") && name.contains("tt"))
}

fn char_is_monospaced(text_page: &TextPage, index: usize) -> bool {
	let Some(font) = text_page.char(index).and_then(PageChar::font) else { return false };
	// Reliable when set and worth nothing when not: the Computer Modern typewriter faces LaTeX sets code in leave it clear.
	font.flags.contains(FontFlags::FIXED_PITCH) || looks_monospaced(font.name)
}

const BOLD_WEIGHT: i32 = 600;

fn looks_bold(font_name: &str) -> bool {
	const HINTS: [&str; 5] = ["bold", "black", "heavy", "semibold", "demi"];
	let name = font_name.rsplit('+').next().unwrap_or(font_name).to_ascii_lowercase();
	HINTS.iter().any(|hint| name.contains(hint))
}

fn char_is_bold(text_page: &TextPage, index: usize) -> bool {
	let Some(ch) = text_page.char(index) else { return false };
	if let Some(weight) = ch.font_weight() {
		return weight >= BOLD_WEIGHT;
	}
	ch.font().is_some_and(|font| looks_bold(font.name))
}

/// Every sampled character has to be bold: a paragraph that opens with a bold word is not a heading.
fn line_is_bold(text_page: &TextPage, sampled: &[usize]) -> bool {
	!sampled.is_empty() && sampled.iter().all(|&i| char_is_bold(text_page, i))
}

/// Most sampled characters have to agree: one word of code quoted in prose does not make the sentence a listing.
fn line_is_monospaced(text_page: &TextPage, sampled: &[usize]) -> bool {
	let monospaced = sampled.iter().filter(|&&i| char_is_monospaced(text_page, i)).count();
	!sampled.is_empty() && monospaced * 2 > sampled.len()
}

#[cfg(test)]
mod tests {
	use super::{
		PageEvidence, Rect, WordPages, confirms, covering_hides, ends_line, looks_monospaced, sanitize_pdf_text,
		space_is_invisible,
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

	/// The coordinates in these tests are what pdfium reports for the PDF attached to #808.
	fn char_box(left: f64, right: f64, bottom: f64, top: f64) -> Rect {
		Rect { left, bottom, right, top }
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
