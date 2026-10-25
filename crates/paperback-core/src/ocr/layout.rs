//! Putting recognized text back in reading order, from where on the page each word was read.
//!
//! An OCR engine reads a page a line at a time and knows nothing about its layout. Text set in boxes or columns comes back with the lines of different boxes in between each other, and with two boxes that sit side by side read as one line (#1167). So each line is split where a gap between two words is far wider than a space, the pieces that sit one under another are stacked into blocks, and the blocks are read column by column. The lines of each block are then joined into paragraphs.

use std::cmp::{Ordering, Reverse};

use crate::parser::pdf::{join_wrapped_lines, join_wrapped_runs, keeps_line_end_hyphen};

/// One word an OCR engine read, and its box in the image, in pixels from the top left.
#[derive(Debug, Clone, PartialEq)]
pub struct RecognizedWord {
	pub text: String,
	pub x: f32,
	pub y: f32,
	pub width: f32,
	pub height: f32,
}

/// One line an OCR engine read: its text as the engine wrote it, and its words in reading order.
#[derive(Debug, Clone, PartialEq)]
pub struct RecognizedLine {
	pub text: String,
	pub words: Vec<RecognizedWord>,
}

/// A gap between two words of one line wider than this many times the line's height may not be a space. Windows reads two boxes side by side as one line with a gap of about one line height between them, where a space is a third of one.
const WIDE_GAP_HEIGHTS: f32 = 0.8;

/// A gap must also be this many times the narrowest gap in its line, so that a justified line, whose every gap is wide, stays whole.
const WIDE_GAP_SPACES: f32 = 2.5;

/// The only gap of a two-word line has nothing to be measured against, so it must be wider than this many line heights.
const LONE_GAP_HEIGHTS: f32 = 1.2;

/// How far a line above or below must reach past both sides of a wide gap, in line heights, to show that the gap is inside one box. An engine that drops a word or a symbol leaves a gap as wide as two boxes have between them, but the lines around it run straight across.
const BRIDGE_HEIGHTS: f32 = 0.5;

/// The most space between the lines of one block, in line heights. The lines of a paragraph or a box sit half a line height apart or less.
const LINE_GAP_HEIGHTS: f32 = 1.0;

/// How far one line may reach up into the one above it, in line heights, and still be the next line rather than the same one.
const LINE_OVERLAP_HEIGHTS: f32 = 0.5;

/// Lines set in sizes further apart than this are not one block: a heading is not the first line of the paragraph under it.
const SIZE_RATIO: f32 = 1.6;

/// The narrowest empty column, in line heights, that is taken for the gutter between two columns.
const GUTTER_HEIGHTS: f32 = 1.0;

/// How far apart two lines may start, in line heights, and still be set from the same left edge.
const ALIGNED_HEIGHTS: f32 = 1.0;

/// A line this many characters long is a line of a book, longer than any box or slide sets.
const PROSE_LINE_CHARS: usize = 40;

/// The text of `lines` in reading order, with each block's wrapped lines joined into paragraphs when `join` is set.
///
/// An engine that reports no word boxes leaves nothing to arrange by, so its lines are taken in the order it read them.
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
	// Consecutive blocks of prose go through the rules for wrapped paragraphs as one run of lines, as a whole page of recognized text did before it was arranged at all, so a page of plain prose reads as it did and prose running on from the foot of one column to the head of the next is one paragraph. Every run is measured against all of them together; see `join_wrapped_runs`.
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

/// Whether `next` carries on the sentence `text` stops in: `text` does not end one, and `next` starts in lower case.
fn carries_on(text: &str, next: &str) -> bool {
	!text.trim_end().ends_with(['.', '?', '!', ':', ';']) && starts_lowercase(next)
}

/// Whether the first letter of `line` is lower case, past any quotation mark or bracket opening it.
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

	/// How far the two overlap from left to right, negative for how far apart they are.
	const fn overlap(self, other: Self) -> f32 {
		self.right.min(other.right) - self.left.max(other.left)
	}
}

/// A run of words from one line with no gap between them wider than a space.
#[derive(Debug)]
struct Segment {
	text: String,
	bounds: Bounds,
	/// The median height of its words, which a descender or an accent does not throw off.
	height: f32,
}

/// A gap in a line wide enough that it may lie between two boxes: after which word, and where it runs from and to.
#[derive(Debug)]
struct Gap {
	after: usize,
	left: f32,
	right: f32,
}

/// The gaps in `line` far wider than a space.
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

/// Whether a line just above or below `line` runs straight across `gap`, which makes the gap a hole inside one box rather than the space between two.
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

/// `line` as one segment.
fn whole(line: &RecognizedLine) -> Segment {
	Segment {
		text: line.text.clone(),
		bounds: around(&line.words),
		height: median(line.words.iter().map(|word| word.height)),
	}
}

/// The box around `words`, of which there is at least one.
fn around(words: &[RecognizedWord]) -> Bounds {
	words[1..].iter().map(Bounds::of).fold(Bounds::of(&words[0]), Bounds::union)
}

/// `line` cut after each word in `after`.
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

/// Where each word is in the line's text, so a run of them is cut from the text as the engine wrote it, with the spacing its script uses. `None` when a word is not found there in order.
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

/// Lines stacked one under another, as a box or a column of a page sets them.
#[derive(Debug)]
struct Block {
	segments: Vec<Segment>,
	bounds: Bounds,
}

/// Stacks each segment under the block whose last line sits right above it, or starts a block of its own.
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

/// Appends `blocks` to `ordered` in reading order: column by column where an empty column at least `gutter` wide runs between them, otherwise row by row, and within what neither splits, from the top.
///
/// Columns are tried first, because two columns of text often have their paragraph breaks at the same height, and reading them row by row would go back and forth between the columns.
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

/// Splits `blocks` into groups wherever nothing covers a stretch of more than `least` along the axis `span` measures, in order along it. Hands the blocks back when nothing splits them.
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

/// The lines of a block that is not prose joined where one plainly carries on: the next starts in lower case, or this one ends on a comma, an ampersand or a hyphen.
///
/// Only prose goes through the rules for wrapped paragraphs, which break a paragraph where a line falls short of the measure. A box in a diagram or a slide, a caption or a heading has no measure to fall short of, and those rules would run its title into the text under it.
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

/// Whether a block is wrapped prose: lines set from the margin or the indent, at least one of them as long as a line of a book. Centred lines start wherever their length puts them, and the lines of a box or a slide are short, which is what tells them from a paragraph.
///
/// How full its lines run says nothing either way, for a box of centred lines of much the same length fills its width as well as a paragraph does. A narrow column of prose is left to the joining a box gets, which joins most of its lines all the same.
fn is_wrapped_prose(block: &Block) -> bool {
	let height = median(block.segments.iter().map(|segment| segment.height));
	left_edges(block, ALIGNED_HEIGHTS * height) * 4 >= block.segments.len() * 3
		&& block.segments.iter().any(|segment| segment.text.chars().count() >= PROSE_LINE_CHARS)
}

/// How many of a block's lines start at one of its two most common left edges, counting edges within `tolerance` of each other as one: the margin and the paragraph indent, for prose.
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

	/// A line set from `x` at `y`, its words as wide as their letters with a space between each.
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

	/// Two lines an engine read as one, the second starting at `x` with a gap of `gap` before it.
	fn merged(first: &RecognizedLine, second: &RecognizedLine) -> RecognizedLine {
		let mut words = first.words.clone();
		words.extend(second.words.clone());
		RecognizedLine { text: format!("{} {}", first.text, second.text), words }
	}

	/// The reported case: two boxes side by side, whose bottom lines Windows read as one line with a wide gap between them. The left box's line above ends where the right box starts.
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

	/// A word the engine dropped leaves a hole as wide as the space between two boxes, but the lines above and below run straight across it.
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

	/// Two columns are read one after the other, not line by line across both.
	#[test]
	fn columns_are_read_one_after_the_other() {
		let lines = vec![
			merged(&line("left one", 0.0, 0.0), &line("right one", 300.0, 0.0)),
			merged(&line("left two", 0.0, 22.0), &line("right two", 300.0, 22.0)),
		];
		assert_eq!(arrange(&lines, false), "left one\nleft two\nright one\nright two");
	}

	/// A box's title stays apart from the text under it, which a centred box sets no wider.
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

	/// Prose goes through the rules for wrapped paragraphs, which join a line that fills the measure whatever the next one starts with, and break after one that ends a sentence short of it. Its indented first lines are still set from its left edge.
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

	/// Without word boxes there is nothing to arrange by, so the lines come as the engine read them.
	#[test]
	fn lines_without_boxes_keep_the_engine_s_order() {
		let lines = vec![
			RecognizedLine { text: "second".to_string(), words: Vec::new() },
			RecognizedLine { text: "first".to_string(), words: Vec::new() },
		];
		assert_eq!(arrange(&lines, false), "second\nfirst");
	}

	/// A run cut from a line keeps the line's own spacing, which a script written without spaces does not have.
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
