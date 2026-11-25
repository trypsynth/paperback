//! Reading order found from where the text sits, for pages whose tags and drawing order both jump about: Apple Pages tags and draws a page box by box in the order the boxes were made.

use std::iter;

use super::line::{Line, sorted_median};
use crate::parser::util::bidi;

/// In line heights: the lines of a paragraph sit about a fifth of one apart.
const BAND_GAP: f64 = 0.5;

/// In line heights: a word space is about a quarter of one.
const GUTTER: f64 = 1.0;

/// In line heights: a band whose column starts no farther than this from where the column above started carries it on.
const SAME_EDGE: f64 = 2.0;

/// In line heights: a column running on alone below its neighbour sits this close under it, where a footer is set farther off.
const CARRIES_ON: f64 = 3.0;

/// A line with no position, such as an empty one, travels with the line before it.
struct Item {
	line: Line,
	trailing: Vec<Line>,
}

#[derive(Clone, Copy)]
struct Metrics {
	unit: f64,
	right_to_left: bool,
}

/// Each gutter runs from the right edge of the column before it to where the column after it starts, left to right.
struct Grid {
	gutters: Vec<(f64, f64)>,
}

impl Grid {
	fn of(columns: &[Vec<Item>]) -> Self {
		let left = |column: &Vec<Item>| column.iter().map(|item| item.line.left).fold(f64::INFINITY, f64::min);
		let right = |column: &Vec<Item>| column.iter().map(|item| item.line.right).fold(f64::NEG_INFINITY, f64::max);
		Self { gutters: columns.windows(2).map(|pair| (right(&pair[0]), left(&pair[1]))).collect() }
	}

	/// A band can have more gutters than the section, such as a list set in two columns inside one, which that column finds when it is read.
	fn shared(&self, other: &Self, metrics: Metrics) -> Vec<(f64, f64)> {
		let matches = |ours: &(f64, f64), theirs: &(f64, f64)| {
			(ours.1 - theirs.1).abs() <= SAME_EDGE * metrics.unit && ours.0.max(theirs.0) < ours.1.min(theirs.1)
		};
		self.gutters
			.iter()
			.filter_map(|ours| {
				let theirs = other.gutters.iter().find(|theirs| matches(ours, theirs))?;
				Some((ours.0.max(theirs.0), ours.1.min(theirs.1)))
			})
			.collect()
	}

	fn column_of(&self, item: &Item, metrics: Metrics) -> usize {
		self.gutters.iter().filter(|gutter| item.line.left >= SAME_EDGE.mul_add(-metrics.unit, gutter.1)).count()
	}

	fn fits<'a>(&self, mut items: impl Iterator<Item = &'a Item>, metrics: Metrics) -> bool {
		items.all(|item| {
			self.gutters.get(self.column_of(item, metrics)).is_none_or(|gutter| item.line.right <= gutter.1)
		})
	}
}

struct Section {
	grid: Grid,
	items: Vec<Item>,
	bottom: f64,
}

/// How a band below a section stands to it.
enum Fit {
	/// It has a gutter where the section has one, and the section keeps only the gutters both have.
	Shares(Vec<(f64, f64)>),
	/// Nothing in it crosses a gutter, and it has text in more than one column, close under the section.
	Spans,
	/// Nothing in it crosses a gutter, and its text is all in one column, which may or may not carry the section on.
	Within,
	Breaks,
}

impl Section {
	fn starting(band: Band, metrics: Metrics, ordered: &mut Vec<Item>) -> Option<Self> {
		match band.content {
			Ok(columns) => Some(Self {
				grid: Grid::of(&columns),
				items: columns.into_iter().flatten().collect(),
				bottom: band.bottom,
			}),
			Err(items) => {
				order(items, metrics, ordered);
				None
			}
		}
	}

	/// `above` is the bottom of what the band would follow.
	fn fit(&self, band: &Band, above: f64, metrics: Metrics) -> Fit {
		if let Ok(columns) = &band.content {
			let shared = self.grid.shared(&Grid::of(columns), metrics);
			if !shared.is_empty() {
				return Fit::Shares(shared);
			}
		}
		if !self.grid.fits(band.items(), metrics) {
			return Fit::Breaks;
		}
		let mut columns = band.items().map(|item| self.grid.column_of(item, metrics));
		let first = columns.next();
		if columns.all(|column| Some(column) == first) {
			Fit::Within
		} else if near(band.top, above, metrics) {
			Fit::Spans
		} else {
			Fit::Breaks
		}
	}

	fn take(&mut self, band: Band) {
		self.bottom = band.bottom;
		match band.content {
			Ok(columns) => self.items.extend(columns.into_iter().flatten()),
			Err(items) => self.items.extend(items),
		}
	}
}

struct Band {
	top: f64,
	bottom: f64,
	/// Split into its columns where it has more than one.
	content: Result<Vec<Vec<Item>>, Vec<Item>>,
}

impl Band {
	fn items(&self) -> impl Iterator<Item = &Item> {
		let (columns, items) = match &self.content {
			Ok(columns) => (columns.as_slice(), None),
			Err(items) => (&[][..], Some(items)),
		};
		columns.iter().flatten().chain(items.into_iter().flatten())
	}

	fn into_items(self) -> Vec<Item> {
		match self.content {
			Ok(columns) => columns.into_iter().flatten().collect(),
			Err(items) => items,
		}
	}
}

fn near(top: f64, above: f64, metrics: Metrics) -> bool {
	above - top <= CARRIES_ON * metrics.unit
}

/// Rows are tried before columns, but a column found in one band of rows is read on down through every band below that carries it on, since two columns often leave a gap at the same height.
pub(in crate::parser) fn read_by_columns(lines: Vec<Line>) -> Vec<Line> {
	let mut leading = Vec::new();
	let mut items: Vec<Item> = Vec::new();
	for line in lines {
		if is_placed(&line) {
			items.push(Item { line, trailing: Vec::new() });
		} else if let Some(last) = items.last_mut() {
			last.trailing.push(line);
		} else {
			leading.push(line);
		}
	}
	let mut heights: Vec<f64> = items.iter().map(|item| item.line.top - item.line.bottom).collect();
	let unit = sorted_median(&mut heights);
	let right_to_left =
		items.iter().filter(|item| bidi::contains_rtl(item.line.text.chars())).count() * 2 > items.len();
	let ordered = if unit > 0.0 {
		let mut ordered = Vec::with_capacity(items.len());
		order(items, Metrics { unit, right_to_left }, &mut ordered);
		ordered
	} else {
		items
	};
	leading.into_iter().chain(ordered.into_iter().flat_map(|item| iter::once(item.line).chain(item.trailing))).collect()
}

fn is_placed(line: &Line) -> bool {
	[line.top, line.bottom, line.left, line.right].iter().all(|edge| edge.is_finite())
		&& line.left <= line.right
		&& line.top >= line.bottom
}

fn order(items: Vec<Item>, metrics: Metrics, ordered: &mut Vec<Item>) {
	if items.len() <= 1 {
		ordered.extend(items);
		return;
	}
	let mut bands = bands(items, metrics);
	if bands.len() == 1 {
		match bands.pop().map(|band| band.content) {
			Some(Ok(columns)) => read_columns(columns, metrics, ordered),
			Some(Err(items)) => ordered.extend(rows(items, metrics)),
			None => {}
		}
		return;
	}
	let mut bands = bands.into_iter().peekable();
	let mut open: Option<Section> = None;
	while let Some(band) = bands.next() {
		let Some(section) = open.as_mut() else {
			open = Section::starting(band, metrics, ordered);
			continue;
		};
		match section.fit(&band, section.bottom, metrics) {
			Fit::Shares(gutters) => {
				section.grid.gutters = gutters;
				section.take(band);
			}
			Fit::Spans => section.take(band),
			Fit::Breaks => {
				close(open.take(), metrics, ordered);
				open = Section::starting(band, metrics, ordered);
			}
			Fit::Within => {
				let mut run = vec![band];
				while let Some(next) =
					bands.next_if(|next| matches!(section.fit(next, section.bottom, metrics), Fit::Within))
				{
					run.push(next);
				}
				let last_bottom = run.last().map_or(section.bottom, |band| band.bottom);
				let next = bands.peek();
				let resumes = next
					.is_some_and(|next| matches!(section.fit(next, last_bottom, metrics), Fit::Shares(_) | Fit::Spans));
				// A column running on alone belongs to the section when its neighbour comes back below it, or when nothing follows close behind, as a footer is set apart; a heading over text set right across the page has that text straight after it.
				let absorbed = if resumes {
					run.len()
				} else if next.is_some_and(|next| near(next.top, last_bottom, metrics)) {
					0
				} else {
					let mut above = section.bottom;
					run.iter()
						.take_while(|band| {
							let carries_on = near(band.top, above, metrics);
							above = band.bottom;
							carries_on
						})
						.count()
				};
				let mut run = run.into_iter();
				for band in run.by_ref().take(absorbed) {
					section.take(band);
				}
				let rest: Vec<Band> = run.collect();
				if !rest.is_empty() {
					close(open.take(), metrics, ordered);
					for band in rest {
						order(band.into_items(), metrics, ordered);
					}
				}
			}
		}
	}
	close(open, metrics, ordered);
}

fn close(section: Option<Section>, metrics: Metrics, ordered: &mut Vec<Item>) {
	let Some(section) = section else { return };
	let mut columns: Vec<Vec<Item>> = (0..=section.grid.gutters.len()).map(|_| Vec::new()).collect();
	for item in section.items {
		let column = section.grid.column_of(&item, metrics);
		columns[column].push(item);
	}
	columns.retain(|column| !column.is_empty());
	read_columns(columns, metrics, ordered);
}

fn read_columns(mut columns: Vec<Vec<Item>>, metrics: Metrics, ordered: &mut Vec<Item>) {
	// One column handed back to `order` whole would never finish.
	if columns.len() <= 1 {
		ordered.extend(rows(columns.into_iter().flatten().collect(), metrics));
		return;
	}
	if metrics.right_to_left {
		columns.reverse();
	}
	for column in columns {
		order(column, metrics, ordered);
	}
}

fn bands(mut items: Vec<Item>, metrics: Metrics) -> Vec<Band> {
	items.sort_by(|a, b| b.line.top.total_cmp(&a.line.top));
	let mut bands: Vec<(f64, f64, Vec<Item>)> = Vec::new();
	for item in items {
		match bands.last_mut() {
			Some((_, bottom, band)) if *bottom - item.line.top < BAND_GAP * metrics.unit => {
				*bottom = bottom.min(item.line.bottom);
				band.push(item);
			}
			_ => bands.push((item.line.top, item.line.bottom, vec![item])),
		}
	}
	bands.into_iter().map(|(top, bottom, items)| Band { top, bottom, content: columns(items, metrics) }).collect()
}

fn columns(mut items: Vec<Item>, metrics: Metrics) -> Result<Vec<Vec<Item>>, Vec<Item>> {
	items.sort_by(|a, b| a.line.left.total_cmp(&b.line.left));
	let mut columns: Vec<Vec<Item>> = Vec::new();
	let mut reach = f64::NEG_INFINITY;
	for item in items {
		let right = item.line.right;
		match columns.last_mut() {
			Some(column) if item.line.left - reach < GUTTER * metrics.unit => column.push(item),
			_ => columns.push(vec![item]),
		}
		reach = reach.max(right);
	}
	if columns.len() > 1 { Ok(columns) } else { Err(columns.into_iter().flatten().collect()) }
}

/// Lines side by side whose tops fall within the upper half of the first are a row, read across.
fn rows(mut items: Vec<Item>, metrics: Metrics) -> Vec<Item> {
	items.sort_by(|a, b| b.line.top.total_cmp(&a.line.top));
	let across = |a: &Item, b: &Item| {
		let ordering = a.line.left.total_cmp(&b.line.left);
		if metrics.right_to_left { ordering.reverse() } else { ordering }
	};
	let mut ordered: Vec<Item> = Vec::with_capacity(items.len());
	let mut row: Vec<Item> = Vec::new();
	for item in items {
		let in_row = row.first().is_some_and(|first| item.line.top > f64::midpoint(first.line.top, first.line.bottom));
		if !in_row {
			row.sort_by(across);
			ordered.append(&mut row);
		}
		row.push(item);
	}
	row.sort_by(across);
	ordered.append(&mut row);
	ordered
}

#[cfg(test)]
mod tests {
	use super::{Line, read_by_columns};

	fn boxed(text: &str, left: f64, right: f64, bottom: f64, top: f64) -> Line {
		Line { text: text.to_string(), size: top - bottom, top, bottom, left, right, monospaced: false, bold: false }
	}

	fn line(text: &str, left: f64, right: f64, top: f64) -> Line {
		boxed(text, left, right, top - 10.0, top)
	}

	fn texts(lines: &[Line]) -> Vec<&str> {
		lines.iter().map(|line| line.text.as_str()).collect()
	}

	/// Page 1 of the crochet pattern from Mastodon, made in Apple Pages, given in the order Pages drew and tagged it.
	#[test]
	fn a_page_drawn_box_by_box_is_read_down_each_column() {
		let lines = vec![
			boxed("Colorful", 388.0, 461.0, 691.0, 724.0),
			boxed("Shell Blanket", 357.0, 491.0, 651.0, 687.0),
			boxed("crochet pattern", 332.0, 516.0, 633.0, 647.0),
			boxed("PAGE 1 of 8", 484.0, 550.0, 51.0, 58.0),
			boxed("JOTOTHEWORLD.COM", 61.0, 209.0, 51.0, 58.0),
			line("This pattern and all images cannot be copied", 303.0, 549.0, 615.0),
			boxed("A Note from Jo", 309.0, 435.0, 548.0, 575.0),
			boxed("Materials", 131.0, 201.0, 414.0, 441.0),
			line("Thank you so much for downloading this pattern.", 303.0, 557.0, 535.0),
			line("I hope you enjoy making this pattern.", 304.0, 557.0, 479.0),
			line("approx. = approximately", 304.0, 424.0, 174.0),
			line("dc = double crochet", 304.0, 424.0, 161.0),
			line("sk = skip", 304.0, 424.0, 148.0),
			line("sl st = slip stitch", 443.0, 546.0, 174.0),
			line("st/sts = stitch/stitches", 443.0, 546.0, 161.0),
			boxed("Size", 149.0, 182.0, 527.0, 554.0),
			line("The pattern creates a square blanket.", 69.0, 265.0, 519.0),
			line("Instructions for adjusting the size are included.", 69.0, 265.0, 480.0),
			line("Yarn: You can use any size 4 yarn.", 67.0, 262.0, 407.0),
			line("Color A: 310 yds.", 68.0, 156.0, 379.0),
			line("I used Soft Classic yarn.", 68.0, 264.0, 281.0),
			line("This yarn is size 4.", 67.0, 239.0, 204.0),
			line("Crochet Hook Size: J/10.", 68.0, 232.0, 148.0),
			line("Other Materials: Yarn needle.", 68.0, 261.0, 120.0),
			boxed("Video Tutorial", 313.0, 404.0, 326.0, 352.0),
			boxed("Terminology & Abbreviations", 310.0, 506.0, 230.0, 258.0),
			line("Watch the step-by-step video tutorial here:", 304.0, 498.0, 310.0),
			boxed("Skill Level", 313.0, 391.0, 408.0, 435.0),
			line("This pattern is written for right-handed crochet.", 304.0, 542.0, 213.0),
			line("The skill level is easy.", 302.0, 398.0, 393.0),
		];
		assert_eq!(
			texts(&read_by_columns(lines)),
			[
				"Colorful",
				"Shell Blanket",
				"crochet pattern",
				"This pattern and all images cannot be copied",
				"Size",
				"The pattern creates a square blanket.",
				"Instructions for adjusting the size are included.",
				"Materials",
				"Yarn: You can use any size 4 yarn.",
				"Color A: 310 yds.",
				"I used Soft Classic yarn.",
				"This yarn is size 4.",
				"Crochet Hook Size: J/10.",
				"Other Materials: Yarn needle.",
				"A Note from Jo",
				"Thank you so much for downloading this pattern.",
				"I hope you enjoy making this pattern.",
				"Skill Level",
				"The skill level is easy.",
				"Video Tutorial",
				"Watch the step-by-step video tutorial here:",
				"Terminology & Abbreviations",
				"This pattern is written for right-handed crochet.",
				"approx. = approximately",
				"dc = double crochet",
				"sk = skip",
				"sl st = slip stitch",
				"st/sts = stitch/stitches",
				"JOTOTHEWORLD.COM",
				"PAGE 1 of 8",
			]
		);
	}

	/// Page 4 of the same pattern, where Pages drew Round 4, on the right, before Round 3.
	#[test]
	fn the_left_column_comes_first_however_it_was_drawn() {
		let lines = vec![
			line("Round 4: Ch 1.", 312.0, 511.0, 461.0),
			line("In the next st, make a Shell.", 311.0, 441.0, 447.0),
			line("Round 3: Change colors.", 45.0, 284.0, 618.0),
			line("Ch 1. Reminder: Start in the same st.", 45.0, 231.0, 478.0),
			line("Repeat from * to * around.", 46.0, 166.0, 436.0),
		];
		assert_eq!(
			texts(&read_by_columns(lines)),
			[
				"Round 3: Change colors.",
				"Ch 1. Reminder: Start in the same st.",
				"Repeat from * to * around.",
				"Round 4: Ch 1.",
				"In the next st, make a Shell.",
			]
		);
	}

	#[test]
	fn a_column_running_on_below_the_other_is_finished_before_the_next() {
		let mut lines = Vec::new();
		for row in 0..6 {
			let top = f64::from(row).mul_add(-25.0, 700.0);
			lines.push(line(&format!("left {row}"), 50.0, 280.0, top));
			if row < 3 {
				lines.push(line(&format!("right {row}"), 320.0, 550.0, top));
			}
		}
		assert_eq!(
			texts(&read_by_columns(lines)),
			["left 0", "left 1", "left 2", "left 3", "left 4", "left 5", "right 0", "right 1", "right 2"]
		);
	}

	#[test]
	fn a_heading_over_text_set_across_the_page_is_not_taken_into_a_column() {
		let lines = vec![
			line("left 0", 50.0, 280.0, 700.0),
			line("right 0", 320.0, 550.0, 700.0),
			line("left 1", 50.0, 280.0, 685.0),
			line("right 1", 320.0, 550.0, 685.0),
			line("References", 50.0, 150.0, 660.0),
			line("A line set right across the page", 50.0, 550.0, 640.0),
		];
		assert_eq!(
			texts(&read_by_columns(lines)),
			["left 0", "left 1", "right 0", "right 1", "References", "A line set right across the page"]
		);
	}

	#[test]
	fn a_footer_below_the_left_column_stays_at_the_foot_of_the_page() {
		let lines = vec![
			line("left 0", 50.0, 280.0, 700.0),
			line("right 0", 320.0, 550.0, 700.0),
			line("left 1", 50.0, 280.0, 685.0),
			line("right 1", 320.0, 550.0, 685.0),
			line("example.com", 50.0, 150.0, 60.0),
		];
		assert_eq!(texts(&read_by_columns(lines)), ["left 0", "left 1", "right 0", "right 1", "example.com"]);
	}

	#[test]
	fn a_page_of_one_column_reads_top_down() {
		let lines = vec![
			line("Chapter One", 200.0, 400.0, 700.0),
			line("It was a dark night,", 72.0, 540.0, 660.0),
			line("and the rain fell.", 72.0, 200.0, 646.0),
			line("Indented, the next one.", 90.0, 540.0, 620.0),
			line("12", 295.0, 305.0, 40.0),
		];
		assert_eq!(
			texts(&read_by_columns(lines)),
			["Chapter One", "It was a dark night,", "and the rain fell.", "Indented, the next one.", "12"]
		);
	}

	#[test]
	fn columns_of_right_to_left_text_are_read_from_the_right() {
		let lines = vec![
			line("\u{5E9}\u{5DC}\u{5D5}\u{5DD} \u{5D0}", 50.0, 280.0, 700.0),
			line("\u{5E9}\u{5DC}\u{5D5}\u{5DD} \u{5D1}", 320.0, 550.0, 700.0),
		];
		let ordered = read_by_columns(lines);
		assert!(ordered[0].text.ends_with('\u{5D1}'), "the right column first");
	}

	#[test]
	fn a_line_with_no_position_travels_with_the_line_before_it() {
		let empty =
			Line { text: String::new(), top: f64::NEG_INFINITY, bottom: f64::INFINITY, ..line("", 0.0, 0.0, 0.0) };
		let lines = vec![line("right", 320.0, 550.0, 700.0), empty, line("left", 50.0, 280.0, 700.0)];
		let ordered = read_by_columns(lines);
		assert_eq!(texts(&ordered), ["left", "right", ""]);
		assert!(ordered[2].top.is_infinite());
	}
}
