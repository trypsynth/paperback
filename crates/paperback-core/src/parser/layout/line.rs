use std::cmp::Ordering;

#[derive(Clone, Debug)]
pub(in crate::parser) struct Line {
	pub text: String,
	pub size: f64,
	pub top: f64,
	pub bottom: f64,
	pub monospaced: bool,
	pub bold: bool,
}

/// pdfium reports characters on one line with identical coordinates, so this only has to absorb rounding.
pub(in crate::parser) const COORDINATE_EPSILON: f64 = 0.05;

pub(in crate::parser) fn is_cjk(c: char) -> bool {
	let u = c as u32;
	(0x4E00..=0x9FFF).contains(&u) || // CJK Unified Ideographs
	(0x3400..=0x4DBF).contains(&u) || // CJK Extension A
	(0x20000..=0x2A6DF).contains(&u) || // CJK Extension B
	(0x3040..=0x309F).contains(&u) || // Hiragana
	(0x30A0..=0x30FF).contains(&u) || // Katakana
	(0xAC00..=0xD7AF).contains(&u) // Hangul
}

/// Judged by the word's shape alone: a hyphen beside an abbreviation, a capital or a number belongs to a compound (`NFB-NEWSLINE`, `COVID-19`).
pub(crate) fn keeps_line_end_hyphen(left: &str, right: &str) -> bool {
	if left.is_empty() || right.is_empty() {
		return false;
	}
	let left_is_abbreviation =
		left.chars().count() >= 2 && left.chars().all(|c| c.is_uppercase() || c.is_ascii_digit());
	let right_opens_a_name = right.chars().next().is_some_and(|c| c.is_uppercase() || c.is_ascii_digit());
	left_is_abbreviation || right_opens_a_name || left.chars().any(|c| c.is_ascii_digit())
}

pub(in crate::parser) fn sorted_median(values: &mut [f64]) -> f64 {
	if values.is_empty() {
		return 0.0;
	}
	values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
	values[values.len() / 2]
}

pub(in crate::parser) fn median_line_font_size<'a>(line_infos: impl IntoIterator<Item = &'a Line>) -> f64 {
	let mut sizes: Vec<f64> = line_infos
		.into_iter()
		.filter(|line| !line.text.trim().is_empty() && line.size > 0.0)
		.map(|line| line.size)
		.collect();
	sorted_median(&mut sizes)
}

/// A running header drawn after the body sits well inside this, and a second column starts about where the first did.
const OUT_OF_ORDER_SHARE: f64 = 0.25;

/// Some form generators draw a page's lower half first, so the form read from its middle; whole regions are moved, so a page drawn in order, columns and a footer drawn first included, comes back unchanged.
pub(in crate::parser) fn restore_reading_order(lines: Vec<Line>) -> Vec<Line> {
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
	use super::{Line, keeps_line_end_hyphen, restore_reading_order};

	#[test]
	fn hyphens_without_evidence_follow_the_shape_of_the_word() {
		assert!(!keeps_line_end_hyphen("infor", "mation"));
		assert!(keeps_line_end_hyphen("NFB", "NEWSLINE"));
		assert!(keeps_line_end_hyphen("Victor", "Reader"));
		assert!(keeps_line_end_hyphen("MP3", "player"));
		assert!(!keeps_line_end_hyphen("", "word"), "nothing before the hyphen: not a compound");
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
}
