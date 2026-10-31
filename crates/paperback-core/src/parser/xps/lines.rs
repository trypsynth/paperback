use std::mem;

use super::page::{Page, Run};
use crate::parser::{
	pdf::{Line, restore_reading_order},
	util::bidi::{contains_rtl, reorder_line},
};

/// How far above the baseline a line's box starts, as a share of its size; the box is one size high, as the paragraph gap ratios expect of a line.
const ASCENT: f64 = 0.8;
/// A word space is about a quarter of the size; below this a gap is only the spacing between two runs of one word.
const MIN_WORD_GAP: f64 = 0.15;
/// `reorder_line` takes anything closer than half a unit for jitter, so a rise this big is what tells it the text is in visual order.
const MIN_CHARACTER_STEP: f64 = 0.6;

#[derive(Debug, Clone, Copy)]
pub(super) struct RunStyle {
	pub width: f64,
	pub bold: bool,
	pub monospaced: bool,
}

fn is_drawn_mark(run: &Run) -> bool {
	run.text.chars().all(|c| c.is_control() || matches!(c, '\u{E000}'..='\u{F8FF}' | '\u{F0000}'..='\u{10FFFD}'))
}

#[allow(clippy::cast_possible_truncation, reason = "a page coordinate fits an f32")]
fn positioned_characters(runs: &[(&Run, RunStyle)]) -> Vec<(char, f32)> {
	let mut characters = Vec::new();
	for (run, style) in runs {
		let count = run.text.chars().count();
		let step = (style.width / count as f64).max(MIN_CHARACTER_STEP);
		let step = if run.bidi_level % 2 == 1 { -step } else { step };
		characters.extend(run.text.chars().enumerate().map(|(index, c)| (c, (run.x + step * index as f64) as f32)));
	}
	characters
}

fn line_text(runs: &[(&Run, RunStyle)]) -> String {
	if contains_rtl(runs.iter().flat_map(|(run, _)| run.text.chars())) {
		return reorder_line(&positioned_characters(runs));
	}
	let mut text = String::new();
	let mut end = None;
	for (run, style) in runs {
		let spaced = text.ends_with(char::is_whitespace) || run.text.starts_with(char::is_whitespace);
		if !spaced && end.is_some_and(|end: f64| run.x - end > run.size * MIN_WORD_GAP) {
			text.push(' ');
		}
		text.push_str(&run.text);
		end = Some(run.x + style.width);
	}
	text
}

fn finish_line(runs: &[(&Run, RunStyle)], height: f64) -> Line {
	let main = runs.iter().map(|(run, _)| *run).max_by_key(|run| run.text.chars().count());
	let (size, baseline) = main.map_or((0.0, 0.0), |run| (run.size, run.y));
	let top = height - baseline + size * ASCENT;
	let mut visible = runs.iter().filter(|(run, _)| !run.text.trim().is_empty()).map(|(_, style)| style);
	Line {
		text: line_text(runs),
		size,
		top,
		bottom: top - size,
		monospaced: visible.clone().all(|style| style.monospaced),
		bold: visible.all(|style| style.bold),
	}
}

/// Nothing marks where a line ends and a run can stop anywhere, even mid-word, so only the baseline tells.
pub(super) fn page_lines(page: &Page, mut style_of: impl FnMut(&Run) -> RunStyle) -> Vec<Line> {
	let mut lines = Vec::new();
	let mut current: Vec<(&Run, RunStyle)> = Vec::new();
	for run in page.runs.iter().filter(|run| !is_drawn_mark(run)) {
		let moved = |(last, _): &(&Run, RunStyle)| (last.y - run.y).abs() > last.size.max(run.size) / 2.0;
		if current.last().is_some_and(moved) {
			lines.push(finish_line(&mem::take(&mut current), page.height));
		}
		current.push((run, style_of(run)));
	}
	if !current.is_empty() {
		lines.push(finish_line(&current, page.height));
	}
	restore_reading_order(lines)
}

#[cfg(test)]
mod tests {
	use super::*;

	const HEIGHT: f64 = 1000.0;

	fn run(text: &str, x: f64, y: f64) -> Run {
		Run { text: text.to_string(), x, y, size: 12.0, bidi_level: 0 }
	}

	/// Every character six units wide.
	fn measured(run: &Run) -> RunStyle {
		RunStyle { width: run.text.chars().count() as f64 * 6.0, bold: false, monospaced: false }
	}

	fn lines_of(runs: Vec<Run>) -> Vec<Line> {
		page_lines(&Page { height: HEIGHT, runs, ..Page::default() }, measured)
	}

	fn texts(runs: Vec<Run>) -> Vec<String> {
		lines_of(runs).into_iter().map(|line| line.text).collect()
	}

	#[test]
	fn runs_on_one_baseline_make_one_line_even_when_split_mid_word() {
		assert_eq!(texts(vec![run("whethe", 0.0, 100.0), run("r each line", 36.0, 100.0)]), ["whether each line"]);
	}

	#[test]
	fn a_new_baseline_opens_a_new_line() {
		assert_eq!(texts(vec![run("One", 0.0, 100.0), run("Two", 0.0, 118.0)]), ["One", "Two"]);
	}

	#[test]
	fn a_raised_run_stays_on_its_line() {
		assert_eq!(texts(vec![run("Footnote", 0.0, 100.0), run("1", 48.0, 96.0)]), ["Footnote1"]);
	}

	#[test]
	fn a_visible_gap_becomes_a_space() {
		assert_eq!(texts(vec![run("Cell one", 73.0, 100.0), run("Cell two", 409.0, 100.0)]), ["Cell one Cell two"]);
	}

	#[test]
	fn a_gap_beside_a_space_the_file_has_adds_none() {
		assert_eq!(texts(vec![run("Cell one ", 73.0, 100.0), run("Cell two", 409.0, 100.0)]), ["Cell one Cell two"]);
	}

	#[test]
	fn drawn_bullets_are_left_out() {
		let runs = vec![run("\u{9F}", 90.0, 100.0), run("\u{F0B7}", 90.0, 118.0), run("Item", 110.0, 118.0)];
		assert_eq!(texts(runs), ["Item"]);
	}

	#[test]
	fn a_number_stays_with_its_item() {
		assert_eq!(
			texts(vec![run("1.", 94.0, 100.0), run(" ", 107.0, 100.0), run("First", 118.0, 100.0)]),
			["1. First"]
		);
	}

	#[test]
	fn printed_hebrew_comes_back_in_logical_order() {
		let runs = vec![
			run("\u{05DD}\u{05D5}\u{05DC}\u{05E9}", 667.0, 100.0),
			run(" ", 662.0, 100.0),
			run("\u{05DD}\u{05DC}\u{05D5}\u{05E2}", 633.0, 100.0),
		];
		assert_eq!(texts(runs), ["\u{05E9}\u{05DC}\u{05D5}\u{05DD} \u{05E2}\u{05D5}\u{05DC}\u{05DD}"]);
	}

	#[test]
	fn right_to_left_text_already_in_logical_order_is_left_alone() {
		let text = "\u{05E9}\u{05DC}\u{05D5}\u{05DD} \u{05E2}\u{05D5}\u{05DC}\u{05DD}";
		let runs = vec![Run { bidi_level: 1, ..run(text, 672.0, 100.0) }];
		assert_eq!(texts(runs), [text]);
	}

	#[test]
	fn lines_measure_downward_as_pdf_lines_do() {
		let lines = lines_of(vec![run("One", 0.0, 100.0), run("Two", 0.0, 118.0)]);
		assert!(lines[0].top > lines[0].bottom);
		assert!((lines[0].top - lines[0].bottom - 12.0).abs() < 0.001);
		assert!(lines[0].bottom > lines[1].top);
	}

	#[test]
	fn a_line_takes_the_size_of_most_of_its_text() {
		let runs = vec![Run { size: 30.0, ..run("A", 0.0, 100.0) }, run("ll the rest of it", 20.0, 100.0)];
		assert_eq!(lines_of(runs)[0].size, 12.0);
	}

	#[test]
	fn a_line_is_bold_or_monospaced_only_if_all_of_it_is() {
		let style = |bold: bool| {
			move |run: &Run| RunStyle {
				bold: bold || run.text == "bold",
				monospaced: run.text == "code",
				..measured(run)
			}
		};
		let page = |runs| Page { height: HEIGHT, runs, ..Page::default() };
		let mixed = page_lines(&page(vec![run("bold", 0.0, 100.0), run(" plain", 24.0, 100.0)]), style(false));
		assert!(!mixed[0].bold);
		let bold = page_lines(&page(vec![run("bold", 0.0, 100.0), run(" ", 24.0, 100.0)]), style(false));
		assert!(bold[0].bold);
		let code = page_lines(&page(vec![run("code", 0.0, 100.0)]), style(false));
		assert!(code[0].monospaced);
	}

	#[test]
	fn a_page_drawn_lower_half_first_reads_from_the_top() {
		let runs = vec![
			run("Lower", 0.0, 700.0),
			run("Lower still", 0.0, 720.0),
			run("Upper", 0.0, 100.0),
			run("Upper still", 0.0, 120.0),
		];
		assert_eq!(texts(runs), ["Upper", "Upper still", "Lower", "Lower still"]);
	}

	#[test]
	fn columns_drawn_one_after_the_other_stay_in_that_order() {
		let runs = vec![
			run("Left one", 94.0, 226.0),
			run("Left two", 94.0, 249.0),
			run("Right one", 420.0, 226.0),
			run("Right two", 420.0, 249.0),
		];
		assert_eq!(texts(runs), ["Left one", "Left two", "Right one", "Right two"]);
	}
}
