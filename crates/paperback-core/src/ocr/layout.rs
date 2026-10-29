//! An OCR engine knows nothing of layout: boxes side by side come back as one line and their lines interleave (#1167), so lines are split at wide gaps, stacked into blocks and read column by column.

use std::cmp::{Ordering, Reverse};

use crate::parser::pdf::{join_wrapped_lines, join_wrapped_runs, keeps_line_end_hyphen};

/// Pixels from the top left of the image.
#[derive(Debug, Clone, PartialEq)]
pub struct RecognizedWord {
	pub text: String,
	pub x: f32,
	pub y: f32,
	pub width: f32,
	pub height: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RecognizedLine {
	pub text: String,
	pub words: Vec<RecognizedWord>,
}

/// Windows reads two boxes side by side as one line with a gap of about a line height between them, where a space is a third of one.
const WIDE_GAP_HEIGHTS: f32 = 0.8;

/// So that a justified line, whose every gap is wide, stays whole.
const WIDE_GAP_SPACES: f32 = 2.5;

/// The only gap of a two-word line has nothing to be measured against.
const LONE_GAP_HEIGHTS: f32 = 1.2;

/// A dropped word leaves a gap as wide as two boxes have between them, but the lines around it run straight across.
const BRIDGE_HEIGHTS: f32 = 0.5;

/// The lines of a paragraph or a box sit half a line height apart or less.
const LINE_GAP_HEIGHTS: f32 = 1.0;

const LINE_OVERLAP_HEIGHTS: f32 = 0.5;

/// A heading is not the first line of the paragraph under it.
const SIZE_RATIO: f32 = 1.6;

const GUTTER_HEIGHTS: f32 = 1.0;

const ALIGNED_HEIGHTS: f32 = 1.0;

/// Longer than any box or slide sets.
const PROSE_LINE_CHARS: usize = 40;

#[must_use]
pub fn page_text(lines: &[RecognizedLine], arranged: bool) -> String {
	if arranged {
		arrange(lines, true)
	} else {
		lines.iter().map(|line| line.text.as_str()).collect::<Vec<_>>().join("\n")
	}
}

#[must_use]
pub fn arrange(lines: &[RecognizedLine], join: bool) -> String {
	if lines.iter().any(|line| line.words.is_empty() && !line.text.trim().is_empty()) {
		let text = lines.iter().map(|line| line.text.as_str()).collect::<Vec<_>>().join("\n");
		return if join { join_wrapped_lines(text.trim()) } else { text };
	}
	let lines: Vec<&RecognizedLine> = lines.iter().filter(|line| !line.words.is_empty()).collect();
	let gaps: Vec<Vec<Gap>> = lines.iter().map(|line| wide_gaps(line)).collect();
	let pieces: Vec<Vec<Segment>> = lines
		.iter()
		.zip(&gaps)
		.map(|(line, gaps)| split(line, &gaps.iter().map(|gap| gap.after).collect::<Vec<_>>()))
		.collect();
	let mut segments = Vec::new();
	for (index, (line, gaps)) in lines.iter().zip(&gaps).enumerate() {
		let around: Vec<&Segment> =
			pieces.iter().enumerate().filter(|(other, _)| *other != index).flat_map(|(_, pieces)| pieces).collect();
		let line_piece = whole(line);
		let kept: Vec<usize> =
			gaps.iter().filter(|gap| !bridged(gap, &line_piece, &around)).map(|gap| gap.after).collect();
		segments.extend(split(line, &kept));
	}
	let blocks = stack(segments);
	let gutter =
		GUTTER_HEIGHTS * median(blocks.iter().flat_map(|block| block.segments.iter().map(|segment| segment.height)));
	let mut ordered = Vec::with_capacity(blocks.len());
	read_in_order(blocks, gutter, &mut ordered);
	if !join {
		return ordered.iter().map(|block| lines_of(block).join("\n")).collect::<Vec<_>>().join("\n");
	}
	// Consecutive prose blocks are joined as one run, so plain prose reads as it did before arranging and prose running from one column's foot to the next's head is one paragraph.
	let mut pieces: Vec<Result<usize, String>> = Vec::new();
	let mut runs: Vec<String> = Vec::new();
	for block in &ordered {
		if !is_wrapped_prose(block) {
			pieces.push(Err(join_box(block)));
		} else if let Some(Ok(run)) = pieces.last() {
			runs[*run].push('\n');
			runs[*run].push_str(&lines_of(block).join("\n"));
		} else {
			pieces.push(Ok(runs.len()));
			runs.push(lines_of(block).join("\n"));
		}
	}
	let mut joined = join_wrapped_runs(&runs.iter().map(String::as_str).collect::<Vec<_>>()).into_iter();
	let mut text = String::new();
	for piece in pieces {
		let piece = match piece {
			Ok(_) => joined.next().unwrap_or_default(),
			Err(text) => text,
		};
		if piece.trim().is_empty() {
			continue;
		}
		// A sentence that runs on from the foot of one column to the head of the next comes as two blocks, and the second starts in lower case.
		if !text.is_empty() {
			text.push(if carries_on(&text, &piece) { ' ' } else { '\n' });
		}
		text.push_str(&piece);
	}
	text
}

fn carries_on(text: &str, next: &str) -> bool {
	!text.trim_end().ends_with(['.', '?', '!', ':', ';']) && starts_lowercase(next)
}

fn starts_lowercase(line: &str) -> bool {
	line.chars().find(|c| c.is_alphanumeric()).is_some_and(char::is_lowercase)
}

#[derive(Debug, Clone, Copy)]
struct Bounds {
	left: f32,
	top: f32,
	right: f32,
	bottom: f32,
}

impl Bounds {
	fn of(word: &RecognizedWord) -> Self {
		Self { left: word.x, top: word.y, right: word.x + word.width, bottom: word.y + word.height }
	}

	const fn union(self, other: Self) -> Self {
		Self {
			left: self.left.min(other.left),
			top: self.top.min(other.top),
			right: self.right.max(other.right),
			bottom: self.bottom.max(other.bottom),
		}
	}

	/// Negative for how far apart they are.
	const fn overlap(self, other: Self) -> f32 {
		self.right.min(other.right) - self.left.max(other.left)
	}
}

#[derive(Debug)]
struct Segment {
	text: String,
	bounds: Bounds,
	/// The median, which a descender or an accent does not throw off.
	height: f32,
}

#[derive(Debug)]
struct Gap {
	after: usize,
	left: f32,
	right: f32,
}

fn wide_gaps(line: &RecognizedLine) -> Vec<Gap> {
	let words = &line.words;
	let height = median(words.iter().map(|word| word.height));
	// Measured either way, for a right-to-left line reads its words from right to left.
	let gaps: Vec<Gap> = words
		.windows(2)
		.enumerate()
		.map(|(after, pair)| {
			let (a, b) = (Bounds::of(&pair[0]), Bounds::of(&pair[1]));
			if b.left >= a.right {
				Gap { after, left: a.right, right: b.left }
			} else {
				Gap { after, left: b.right, right: a.left }
			}
		})
		.collect();
	let width = |gap: &Gap| gap.right - gap.left;
	let threshold = if gaps.len() >= 2 {
		let narrowest = gaps.iter().map(width).fold(f32::INFINITY, f32::min);
		(WIDE_GAP_HEIGHTS * height).max(WIDE_GAP_SPACES * narrowest)
	} else {
		LONE_GAP_HEIGHTS * height
	};
	gaps.into_iter().filter(|gap| width(gap) > threshold).collect()
}

/// A line just above or below running straight across `gap` makes it a hole inside one box rather than the space between two.
fn bridged(gap: &Gap, line: &Segment, around: &[&Segment]) -> bool {
	let reach = BRIDGE_HEIGHTS * line.height;
	around.iter().any(|other| {
		let height = line.height.max(other.height);
		let above = line.bounds.top - other.bounds.bottom;
		let below = other.bounds.top - line.bounds.bottom;
		let next_to = |distance: f32| (-LINE_OVERLAP_HEIGHTS * height..=LINE_GAP_HEIGHTS * height).contains(&distance);
		(next_to(above) || next_to(below))
			&& other.bounds.left <= gap.left - reach
			&& other.bounds.right >= gap.right + reach
	})
}

fn whole(line: &RecognizedLine) -> Segment {
	Segment {
		text: line.text.clone(),
		bounds: around(&line.words),
		height: median(line.words.iter().map(|word| word.height)),
	}
}

/// `words` must not be empty.
fn around(words: &[RecognizedWord]) -> Bounds {
	words[1..].iter().map(Bounds::of).fold(Bounds::of(&words[0]), Bounds::union)
}

fn split(line: &RecognizedLine, after: &[usize]) -> Vec<Segment> {
	if after.is_empty() {
		return vec![whole(line)];
	}
	let words = &line.words;
	let spans = word_spans(line);
	let mut runs = Vec::with_capacity(after.len() + 1);
	let mut start = 0;
	for &end in after {
		runs.push(start..end + 1);
		start = end + 1;
	}
	runs.push(start..words.len());
	runs.into_iter()
		.map(|run| {
			let text = spans.as_ref().map_or_else(
				|| words[run.clone()].iter().map(|word| word.text.as_str()).collect::<Vec<_>>().join(" "),
				|spans| line.text[spans[run.start].0..spans[run.end - 1].1].to_string(),
			);
			let run = &words[run];
			Segment { text, bounds: around(run), height: median(run.iter().map(|word| word.height)) }
		})
		.collect()
}

/// So a run is cut from the text as the engine wrote it, with the spacing its script uses.
fn word_spans(line: &RecognizedLine) -> Option<Vec<(usize, usize)>> {
	let mut from = 0;
	line.words
		.iter()
		.map(|word| {
			let start = from + line.text[from..].find(word.text.as_str())?;
			from = start + word.text.len();
			Some((start, from))
		})
		.collect()
}

#[derive(Debug)]
struct Block {
	segments: Vec<Segment>,
	bounds: Bounds,
}

fn stack(mut segments: Vec<Segment>) -> Vec<Block> {
	segments.sort_by(|a, b| compare(a.bounds.top, b.bounds.top).then(compare(a.bounds.left, b.bounds.left)));
	let mut blocks: Vec<Block> = Vec::new();
	for segment in segments {
		let below = blocks
			.iter_mut()
			.filter_map(|block| {
				let last = block.segments.last()?;
				let height = last.height.max(segment.height);
				let gap = segment.bounds.top - last.bounds.bottom;
				let sizes = height / last.height.min(segment.height).max(f32::EPSILON);
				let fits = gap <= LINE_GAP_HEIGHTS * height
					&& gap >= -LINE_OVERLAP_HEIGHTS * height
					&& sizes <= SIZE_RATIO
					&& segment.bounds.overlap(last.bounds) > 0.0;
				fits.then_some((gap, block))
			})
			.min_by(|a, b| compare(a.0, b.0));
		match below {
			Some((_, block)) => {
				block.bounds = block.bounds.union(segment.bounds);
				block.segments.push(segment);
			}
			None => blocks.push(Block { bounds: segment.bounds, segments: vec![segment] }),
		}
	}
	blocks
}

/// Columns are tried before rows because two columns often break paragraphs at the same height, and reading by rows would go back and forth between them.
fn read_in_order(blocks: Vec<Block>, gutter: f32, ordered: &mut Vec<Block>) {
	if blocks.len() <= 1 {
		ordered.extend(blocks);
		return;
	}
	let blocks = match groups(blocks, |bounds| (bounds.left, bounds.right), gutter) {
		Ok(columns) => {
			for column in columns {
				read_in_order(column, gutter, ordered);
			}
			return;
		}
		Err(blocks) => blocks,
	};
	let mut blocks = match groups(blocks, |bounds| (bounds.top, bounds.bottom), 0.0) {
		Ok(rows) => {
			for row in rows {
				read_in_order(row, gutter, ordered);
			}
			return;
		}
		Err(blocks) => blocks,
	};
	blocks.sort_by(|a, b| compare(a.bounds.top, b.bounds.top).then(compare(a.bounds.left, b.bounds.left)));
	ordered.extend(blocks);
}

fn groups(
	mut blocks: Vec<Block>,
	span: impl Fn(Bounds) -> (f32, f32),
	least: f32,
) -> Result<Vec<Vec<Block>>, Vec<Block>> {
	blocks.sort_by(|a, b| compare(span(a.bounds).0, span(b.bounds).0));
	let mut groups: Vec<Vec<Block>> = Vec::new();
	let mut reach = f32::NEG_INFINITY;
	for block in blocks {
		let (start, end) = span(block.bounds);
		match groups.last_mut() {
			Some(group) if start - reach <= least => group.push(block),
			_ => groups.push(vec![block]),
		}
		reach = reach.max(end);
	}
	if groups.len() > 1 { Ok(groups) } else { Err(groups.into_iter().flatten().collect()) }
}

fn lines_of(block: &Block) -> Vec<&str> {
	block.segments.iter().map(|segment| segment.text.as_str()).collect()
}

/// The wrapped-paragraph rules break where a line falls short of the measure; a box, caption or heading has no measure, and they would run its title into the text under it.
fn join_box(block: &Block) -> String {
	let mut text = String::new();
	for line in lines_of(block).into_iter().map(str::trim).filter(|line| !line.is_empty()) {
		if text.ends_with('-') {
			let left = text[..text.len() - 1].rsplit(|c: char| !c.is_alphanumeric()).next().unwrap_or("");
			let right: String = line.chars().take_while(|c| c.is_alphanumeric()).collect();
			if !keeps_line_end_hyphen(left, &right) {
				text.pop();
			}
		} else if text.ends_with([',', '&']) || (!text.is_empty() && starts_lowercase(line)) {
			text.push(' ');
		} else if !text.is_empty() {
			text.push('\n');
		}
		text.push_str(line);
	}
	text
}

/// Centred lines start wherever their length puts them and box lines are short; how full lines run says nothing, since a box of centred lines fills its width too.
fn is_wrapped_prose(block: &Block) -> bool {
	let height = median(block.segments.iter().map(|segment| segment.height));
	left_edges(block, ALIGNED_HEIGHTS * height) * 4 >= block.segments.len() * 3
		&& block.segments.iter().any(|segment| segment.text.chars().count() >= PROSE_LINE_CHARS)
}

/// Two edges, for prose's margin and paragraph indent.
fn left_edges(block: &Block, tolerance: f32) -> usize {
	let mut lefts: Vec<f32> = block.segments.iter().map(|segment| segment.bounds.left).collect();
	lefts.sort_by(|a, b| compare(*a, *b));
	let mut edges: Vec<(f32, usize)> = Vec::new();
	for left in lefts {
		match edges.last_mut() {
			Some((start, count)) if left - *start <= tolerance => *count += 1,
			_ => edges.push((left, 1)),
		}
	}
	edges.sort_by_key(|&(_, count)| Reverse(count));
	edges.iter().take(2).map(|(_, count)| count).sum()
}

fn compare(a: f32, b: f32) -> Ordering {
	a.partial_cmp(&b).unwrap_or(Ordering::Equal)
}

fn median(values: impl Iterator<Item = f32>) -> f32 {
	let mut values: Vec<f32> = values.collect();
	if values.is_empty() {
		return 0.0;
	}
	values.sort_by(|a, b| compare(*a, *b));
	values[values.len() / 2]
}

#[cfg(test)]
mod tests {
	use super::{RecognizedLine, RecognizedWord, arrange};

	const HEIGHT: f32 = 16.0;
	const CHAR: f32 = 8.0;
	const SPACE: f32 = 5.0;

	fn line(text: &str, x: f32, y: f32) -> RecognizedLine {
		let mut words = Vec::new();
		let mut left = x;
		for word in text.split(' ') {
			let width: f32 = word.chars().map(|_| CHAR).sum();
			words.push(RecognizedWord { text: word.to_string(), x: left, y, width, height: HEIGHT });
			left += width + SPACE;
		}
		RecognizedLine { text: text.to_string(), words }
	}

	fn merged(first: &RecognizedLine, second: &RecognizedLine) -> RecognizedLine {
		let mut words = first.words.clone();
		words.extend(second.words.clone());
		RecognizedLine { text: format!("{} {}", first.text, second.text), words }
	}

	/// The reported case (#1167): Windows read the bottom lines of two side-by-side boxes as one line.
	#[test]
	fn two_boxes_read_as_one_line_come_apart() {
		let lines = vec![
			line("Hands-on projects grounded in local", 0.0, 0.0),
			merged(&line("stories and environment", 0.0, 22.0), &line("Collaborative & Social Learning", 270.0, 22.0)),
			line("Encourage community ties through", 270.0, 44.0),
		];
		assert_eq!(
			arrange(&lines, false),
			"Hands-on projects grounded in local\nstories and environment\nCollaborative & Social Learning\nEncourage community ties through"
		);
	}

	#[test]
	fn a_hole_inside_a_box_does_not_split_its_line() {
		let mut hole = line("Indigenous & Global South traditional", 0.0, 22.0);
		hole.words.remove(1);
		hole.text = "Indigenous Global South traditional".to_string();
		let lines = vec![
			line("Syncretism and Adaptation in art", 0.0, 0.0),
			hole,
			line("blended with new contexts again", 0.0, 44.0),
		];
		assert_eq!(arrange(&lines, false).lines().nth(1), Some("Indigenous Global South traditional"));
	}

	#[test]
	fn columns_are_read_one_after_the_other() {
		let lines = vec![
			merged(&line("left one", 0.0, 0.0), &line("right one", 300.0, 0.0)),
			merged(&line("left two", 0.0, 22.0), &line("right two", 300.0, 22.0)),
		];
		assert_eq!(arrange(&lines, false), "left one\nleft two\nright one\nright two");
	}

	#[test]
	fn a_box_keeps_its_title_apart_and_joins_its_wrapped_lines() {
		let lines = vec![
			line("Learner Agency", 40.0, 0.0),
			line("Creative, non-traditional", 12.0, 22.0),
			line("assessments that encourage", 4.0, 44.0),
			line("critical thinking", 40.0, 66.0),
		];
		assert_eq!(
			arrange(&lines, true),
			"Learner Agency\nCreative, non-traditional assessments that encourage critical thinking"
		);
	}

	#[test]
	fn a_paragraph_of_prose_is_joined_whole() {
		let lines = vec![
			line("The committee met on Tuesday afternoon to go over the plans with", 30.0, 0.0),
			line("Mary Smith and her colleagues, who had come down from the north", 0.0, 22.0),
			line("office to talk about the budget.", 0.0, 44.0),
			line("The next meeting is set for the spring.", 30.0, 66.0),
		];
		assert_eq!(
			arrange(&lines, true),
			"The committee met on Tuesday afternoon to go over the plans with Mary Smith and her colleagues, who had come down from the north office to talk about the budget.
The next meeting is set for the spring."
		);
	}

	#[test]
	fn lines_without_boxes_keep_the_engine_s_order() {
		let lines = vec![
			RecognizedLine { text: "second".to_string(), words: Vec::new() },
			RecognizedLine { text: "first".to_string(), words: Vec::new() },
		];
		assert_eq!(arrange(&lines, false), "second\nfirst");
	}

	#[test]
	fn a_split_line_keeps_the_engine_s_spacing() {
		let word =
			|text: &str, x: f32| RecognizedWord { text: text.to_string(), x, y: 0.0, width: 16.0, height: HEIGHT };
		let line = RecognizedLine {
			text: "中文书名作者".to_string(),
			words: vec![word("中文", 0.0), word("书名", 18.0), word("作者", 200.0)],
		};
		assert_eq!(arrange(&[line], false), "中文书名\n作者");
	}
}
