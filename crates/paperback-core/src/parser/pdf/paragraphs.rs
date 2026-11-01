use std::mem;

use super::text::{Line, is_cjk, keeps_hyphen};
use crate::util::text::{collapse_leaders, collapse_whitespace, trim_string};

/// pdfium runs two visual lines into one often enough (a third of the lines on some pages of #813) that the longest line is a doubled one and every real line looked short; the 75th percentile is past them but still fills the measure.
const FULL_LINE_PERCENTILE_NUMERATOR: usize = 3;
const FULL_LINE_PERCENTILE_DENOMINATOR: usize = 4;

fn full_line_len(lines: &[(Line, bool)]) -> usize {
	let mut lengths: Vec<usize> =
		lines.iter().map(|(line, ..)| line.text.chars().count()).filter(|len| *len > 0).collect();
	if lengths.is_empty() {
		return 0;
	}
	lengths.sort_unstable();
	lengths[(lengths.len() - 1) * FULL_LINE_PERCENTILE_NUMERATOR / FULL_LINE_PERCENTILE_DENOMINATOR]
}

/// From the documents of #813 and #826: within a paragraph the gap runs to about two thirds of a line's size, between paragraphs from about five fourths.
const PARAGRAPH_GAP_RATIO: f64 = 1.0;

const DROP_CAP_RATIO: f64 = 2.0;

/// A drop cap carries on into a lower-case word, where a chapter number stands over a title in capitals; a digit is never one.
fn merge_drop_caps(raw_lines: &[Line], body_font_size: f64) -> Vec<Line> {
	if body_font_size <= 0.0 {
		return raw_lines.to_vec();
	}
	let mut merged: Vec<Line> = Vec::with_capacity(raw_lines.len());
	let mut skip_next = false;
	for (index, line) in raw_lines.iter().enumerate() {
		if skip_next {
			skip_next = false;
			continue;
		}
		let letter = trim_string(&collapse_whitespace(&line.text));
		let is_cap = letter.chars().count() == 1
			&& letter.chars().next().is_some_and(char::is_alphabetic)
			&& line.size >= body_font_size * DROP_CAP_RATIO;
		let follows_in_lower_case = raw_lines
			.get(index + 1)
			.map(|next| trim_string(&collapse_whitespace(&next.text)))
			.is_some_and(|next| next.chars().next().is_some_and(char::is_lowercase));
		if is_cap && follows_in_lower_case {
			let next = &raw_lines[index + 1];
			merged.push(Line { text: letter + next.text.trim_start(), ..next.clone() });
			skip_next = true;
		} else {
			merged.push(line.clone());
		}
	}
	merged
}

const HEADING_FONT_RATIO: f64 = 1.2;
const HEADING_MAX_LEN: usize = 150;

/// A heading set in the body's size is a few words; a bold line longer than this is emphasis.
const BOLD_HEADING_MAX_LEN: usize = 80;

struct HeadingRule {
	size: f64,
	bold_is_rare: bool,
}

impl HeadingRule {
	/// A page where most lines are bold sets its body in bold, and bold there says nothing.
	fn for_page(lines: &[Line], body_font_size: f64) -> Self {
		let text_lines = lines.iter().filter(|line| !line.text.trim().is_empty()).count();
		let bold_lines = lines.iter().filter(|line| line.bold && !line.text.trim().is_empty()).count();
		Self { size: heading_threshold(body_font_size), bold_is_rare: bold_lines * 2 < text_lines }
	}

	fn mark(&self, lines: &[Line], trimmed: &[(String, usize)]) -> Vec<bool> {
		(0..lines.len())
			.map(|index| {
				let (line, (text, len)) = (&lines[index], &trimmed[index]);
				if *len == 0 {
					return false;
				}
				if line.size >= self.size && *len <= HEADING_MAX_LEN {
					return true;
				}
				// A bold paragraph is emphasis however short its lines are, such as the capitals a licence opens with.
				let bold_beside = |other: Option<usize>| {
					other.and_then(|other| lines.get(other)).is_some_and(|other| {
						other.bold && !other.text.trim().is_empty() && (other.size - line.size).abs() < f64::EPSILON
					})
				};
				let set_in_bold = line.bold;
				self.bold_is_rare
					&& set_in_bold
					&& *len <= BOLD_HEADING_MAX_LEN
					&& reads_as_a_title(text)
					&& !bold_beside(index.checked_sub(1))
					&& !bold_beside(Some(index + 1))
			})
			.collect()
	}
}

/// The PDF of #993 sets every section title in bold a size smaller than the body, so no size rule could find them.
fn reads_as_a_title(text: &str) -> bool {
	let opens_like_a_title = text.chars().next().is_some_and(|c| c.is_uppercase() || c.is_ascii_digit());
	let has_a_word = text.split_whitespace().any(|word| word.chars().filter(|c| c.is_alphabetic()).count() >= 2);
	opens_like_a_title && has_a_word && !text.ends_with(['.', ',', ';', ':']) && !text.contains("...")
}

fn heading_threshold(body_font_size: f64) -> f64 {
	if body_font_size > 0.0 { body_font_size * HEADING_FONT_RATIO } else { f64::INFINITY }
}

/// Headings are still marked with joining off, since that is a question of size, not of where a paragraph ends.
pub(super) fn split_lines(raw_lines: &[Line], body_font_size: f64) -> Vec<(String, bool, usize)> {
	let trimmed = trimmed_lines(raw_lines);
	let marked = HeadingRule::for_page(raw_lines, body_font_size).mark(raw_lines, &trimmed);
	trimmed
		.into_iter()
		.zip(marked)
		.enumerate()
		.filter(|(_, ((_, len), _))| *len > 0)
		.map(|(index, ((text, _), is_heading))| (text, is_heading, index))
		.collect()
}

fn trimmed_lines(lines: &[Line]) -> Vec<(String, usize)> {
	lines
		.iter()
		.map(|line| {
			let text = trim_string(&collapse_whitespace(&line.text));
			let len = text.chars().count();
			(text, len)
		})
		.collect()
}

/// A lettered label is one letter, its punctuation and a space, which keeps "e.g. the rest of a sentence" out.
fn opens_with_a_label(line: &str) -> bool {
	let mut chars = line.chars();
	let Some(first) = chars.next() else { return false };
	if first.is_ascii_digit() {
		for c in chars {
			if c.is_ascii_digit() || c == '.' || c == ')' {
				continue;
			}
			return c.is_whitespace();
		}
		return false;
	}
	first.is_ascii_alphabetic()
		&& matches!(chars.next(), Some('.' | ')'))
		&& chars.next().is_some_and(char::is_whitespace)
}

/// OCR says nothing about size and only ratios matter, so one size for every line leaves length and punctuation to decide.
const RECOGNIZED_TEXT_SIZE: f64 = 12.0;

/// OCR lines are laid out with no gaps, so only the length and punctuation rules decide where paragraphs end.
#[must_use]
pub fn join_wrapped_lines(text: &str) -> String {
	join_paragraphs(&recognized_lines(text), RECOGNIZED_TEXT_SIZE)
		.into_iter()
		.map(|(paragraph, ..)| paragraph)
		.collect::<Vec<String>>()
		.join("\n")
}

/// Judges short lines against the measure of all runs together: a page whose prose comes in runs between headings and boxes has one measure, and a run of a few short lines has none of its own.
#[must_use]
pub fn join_wrapped_runs(runs: &[&str]) -> Vec<String> {
	let runs: Vec<Vec<Line>> = runs.iter().map(|text| recognized_lines(text)).collect();
	let every_line: Vec<Line> = runs.iter().flatten().cloned().collect();
	let trimmed: Vec<(Line, bool)> = every_line
		.iter()
		.zip(trimmed_lines(&every_line))
		.map(|(line, (text, _))| (Line { text, ..line.clone() }, false))
		.collect();
	let measure = full_line_len(&trimmed);
	runs.iter()
		.map(|lines| {
			join_paragraphs_measured(lines, RECOGNIZED_TEXT_SIZE, Some(measure))
				.into_iter()
				.map(|(paragraph, ..)| paragraph)
				.collect::<Vec<String>>()
				.join("\n")
		})
		.collect()
}

fn recognized_lines(text: &str) -> Vec<Line> {
	let mut lines: Vec<Line> = Vec::new();
	let mut top = 0.0;
	for line in text.lines() {
		lines.push(Line {
			text: line.to_string(),
			size: RECOGNIZED_TEXT_SIZE,
			top,
			bottom: top - RECOGNIZED_TEXT_SIZE,
			monospaced: false,
			bold: false,
		});
		top -= RECOGNIZED_TEXT_SIZE;
	}
	lines
}

/// The third field indexes the paragraph's first line in `raw_lines`, which an untagged page needs to place its images.
pub(super) fn join_paragraphs(raw_lines: &[Line], body_font_size: f64) -> Vec<(String, bool, usize)> {
	join_paragraphs_measured(raw_lines, body_font_size, None)
}

fn join_paragraphs_measured(
	raw_lines: &[Line],
	body_font_size: f64,
	measure: Option<usize>,
) -> Vec<(String, bool, usize)> {
	let raw_lines = merge_drop_caps(raw_lines, body_font_size);
	let trimmed = trimmed_lines(&raw_lines);
	let marked = HeadingRule::for_page(&raw_lines, body_font_size).mark(&raw_lines, &trimmed);
	let lines: Vec<(Line, bool)> = raw_lines
		.iter()
		.zip(trimmed)
		.zip(marked)
		.map(|((line, (text, _)), is_heading_line)| (Line { text, ..line.clone() }, is_heading_line))
		.collect();
	// A line ending a sentence need only stop short; one ending mid-sentence must fall well short, since a capital after it is as likely to open a name.
	let full_line = measure.unwrap_or_else(|| full_line_len(&lines));
	let ended_sentence_threshold = full_line * 19 / 20;
	let short_line_threshold = full_line * 3 / 4;
	let mut paragraphs: Vec<(String, bool, usize)> = Vec::new();
	let mut current_paragraph = String::new();
	let mut current_is_heading = false;
	let mut current_heading_size = 0.0f64;
	let mut current_start_line = 0usize;
	let mut last_line_len = 0usize;
	let mut last_line_ends_with_punctuation = false;
	// Infinity so the first line of a page never reads as following a gap.
	let mut last_line_bottom = f64::INFINITY;
	let mut previous_was_monospaced = false;
	for (line_index, (line_info, is_heading_line)) in lines.iter().enumerate() {
		let Line { text: line, size, top, bottom, monospaced, .. } = line_info;
		if line.is_empty() {
			if !current_paragraph.is_empty() {
				paragraphs.push((mem::take(&mut current_paragraph), current_is_heading, current_start_line));
				current_is_heading = false;
			}
			last_line_len = 0;
			last_line_ends_with_punctuation = false;
			last_line_bottom = f64::INFINITY;
			previous_was_monospaced = false;
			continue;
		}
		let is_list_item = line.starts_with("- ") || line.starts_with("* ") || line.starts_with("• ");
		let first_char = line.chars().next();
		let starts_with_uppercase = first_char.is_some_and(char::is_uppercase);
		let starts_with_alpha = first_char.is_some_and(char::is_alphabetic);
		let len = line.chars().count();
		if current_paragraph.is_empty() {
			current_paragraph.clone_from(line);
			current_is_heading = *is_heading_line;
			current_heading_size = *size;
			current_start_line = line_index;
		} else {
			// A multi-line heading carries on while its size holds, but a line opening with its own number starts a new one, which keeps a numbered outline apart.
			let continues_heading =
				*is_heading_line && current_is_heading && (*size - current_heading_size).abs() < f64::EPSILON;
			let previous_line_ended_paragraph = (last_line_ends_with_punctuation
				&& last_line_len < ended_sentence_threshold)
				|| (last_line_len < short_line_threshold && (starts_with_uppercase || !starts_with_alpha));
			// Measured from the previous line's bottom, not its top, because pdfium's doubled lines would make a top-to-top pitch break normal paragraphs apart.
			let opened_by_a_gap = *size > 0.0 && last_line_bottom - *top > *size * PARAGRAPH_GAP_RATIO;
			// A monospaced line's breaks are the content, and geometry can't see it: a listing shares the prose's left edge and leading.
			let set_apart_by_its_face = *monospaced || previous_was_monospaced;
			let break_paragraph = is_list_item
				|| opens_with_a_label(line)
				|| opened_by_a_gap
				|| set_apart_by_its_face
				|| (!continues_heading && (*is_heading_line || current_is_heading || previous_line_ended_paragraph));
			if break_paragraph {
				paragraphs.push((mem::take(&mut current_paragraph), current_is_heading, current_start_line));
				current_paragraph.clone_from(line);
				current_is_heading = *is_heading_line;
				current_heading_size = *size;
				current_start_line = line_index;
			} else {
				let last_char = current_paragraph.chars().last().unwrap_or(' ');
				if current_paragraph.ends_with('-') {
					// Unless it joins a compound such as NFB-NEWSLINE or COVID-19 that only happened to wrap there.
					let left = current_paragraph[..current_paragraph.len() - 1]
						.rsplit(|c: char| !c.is_alphanumeric())
						.next()
						.unwrap_or("");
					let right: String = line.chars().take_while(|c| c.is_alphanumeric()).collect();
					if !keeps_hyphen(left, &right, None) {
						current_paragraph.pop();
					}
				} else if !(is_cjk(last_char) && line.chars().next().is_some_and(is_cjk)) {
					current_paragraph.push(' ');
				}
				current_paragraph.push_str(line);
			}
		}
		last_line_len = len;
		last_line_bottom = *bottom;
		previous_was_monospaced = *monospaced;
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
		paragraphs.push((current_paragraph, current_is_heading, current_start_line));
	}
	// A contents page's dot leaders would otherwise be read out dot by dot.
	for (text, _, _) in &mut paragraphs {
		*text = collapse_leaders(text);
	}
	paragraphs
}

#[cfg(test)]
mod tests {
	use super::{Line, full_line_len, join_paragraphs, join_wrapped_lines, opens_with_a_label, split_lines};

	/// Page 67 of the PDF attached to #813, with the sizes and doubled lines pdfium reports for it.
	fn issue_813_page() -> Vec<Line> {
		tight(&[
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
			(
				"Ministry of Aviation Industries of China (AVIC) in Beijing, discussing human resources for a potential joint venture between",
				11.0,
			),
			("AlliedSignal, the company for which I was then international", 11.0),
			("human resources vice president, and AVIC. When the meeting", 11.0),
			(
				"ended, one of my hosts from the ministry graciously accompanied me outside. As we waited for the car, he noticed that I was",
				11.0,
			),
			("looking up and down the street and then skyward. Visibility was", 11.0),
			("only about a hundred yards in any direction. I was shocked. The", 11.0),
			("executive looked at me and said, \u{201C}My children do not know the", 11.0),
			("sky is blue.\u{201D}", 11.0),
		])
	}

	/// #822: OCR text arrives broken at every printed line end.
	#[test]
	fn join_wrapped_lines_joins_the_lines_of_a_recognized_paragraph() {
		let recognized = concat!(
			"The first thing to understand about the way a page is set is that a line ends
",
			"where the measure runs out and not where the sentence does, which is why a
",
			"paragraph read line by line is so hard to follow.
",
			"A second paragraph, set the same way, begins on a line of its own and runs on
",
			"until it too has said what it has to say."
		);
		assert_eq!(
			join_wrapped_lines(recognized),
			concat!(
				"The first thing to understand about the way a page is set is that a line ends where the ",
				"measure runs out and not where the sentence does, which is why a paragraph read line by ",
				"line is so hard to follow.
",
				"A second paragraph, set the same way, begins on a line of its own and runs on until it ",
				"too has said what it has to say."
			)
		);
	}

	#[test]
	fn join_wrapped_lines_keeps_blocks_apart_at_a_blank_line() {
		let recognized = "A line that stops short

Another line that stops short";
		assert_eq!(
			join_wrapped_lines(recognized),
			"A line that stops short
Another line that stops short"
		);
	}

	#[test]
	fn join_wrapped_lines_leaves_empty_text_empty() {
		assert_eq!(join_wrapped_lines(""), "");
	}

	/// OCR gives no bullet glyph, and short items ending in nothing look like a wrapped paragraph.
	#[test]
	fn join_wrapped_lines_keeps_a_lettered_list_apart() {
		let recognized = concat!(
			"1. The most important difference between computers and other
",
			"machines is that computers
",
			"a. are faster
",
			"b. have a memory
",
			"c. can print words"
		);
		assert_eq!(
			join_wrapped_lines(recognized),
			concat!(
				"1. The most important difference between computers and other machines is that computers
",
				"a. are faster
b. have a memory
c. can print words"
			)
		);
	}

	#[test]
	fn opens_with_a_label_knows_a_label_from_a_sentence() {
		assert!(opens_with_a_label("3. The third one"));
		assert!(opens_with_a_label("3.1 A section"));
		assert!(opens_with_a_label("b) The second one"));
		assert!(opens_with_a_label("l. What OCR made of a 1"));
		assert!(!opens_with_a_label("e.g. a sentence carrying on"));
		assert!(!opens_with_a_label("A man walked in"));
		assert!(!opens_with_a_label("and the rest of the sentence"));
	}

	#[test]
	fn full_line_len_ignores_doubled_lines() {
		let lines: Vec<(Line, bool)> = issue_813_page().into_iter().map(|line| (line, false)).collect();
		let full = full_line_len(&lines);
		assert!((53..=64).contains(&full), "expected the length of a full line, got {full}");
	}

	/// #813: a second line opening with a year, later lines opening with capitals, and a four-line title.
	#[test]
	fn join_paragraphs_keeps_a_chapter_opening_whole() {
		let result = join_paragraphs(&issue_813_page(), 11.0);
		let texts: Vec<&str> = result.iter().map(|(text, ..)| text.as_str()).collect();
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

	#[test]
	fn join_paragraphs_keeps_numbered_headings_apart() {
		let lines = tight(&[
			("2.1.1 Piano Roll mode", 14.0),
			("2.1.1.1 Show / Hide Strings", 14.0),
			("2.1.1.2 Color Indicates String or Velocity", 14.0),
		]);
		let result = join_paragraphs(&lines, 10.0);
		assert_eq!(result.len(), 3, "got {result:?}");
	}

	#[test]
	fn join_paragraphs_keeps_headings_of_different_sizes_apart() {
		let lines = tight(&[("PART ONE", 30.0), ("Getting Started", 20.0)]);
		let result = join_paragraphs(&lines, 11.0);
		assert_eq!(result.len(), 2);
	}

	/// The case a single short-line threshold gets wrong.
	#[test]
	fn join_paragraphs_breaks_after_a_sentence_that_nearly_fills_the_line() {
		let lines = tight(&[
			("This book is part of the English for Research series of guides for non-native English", 10.0),
			("academics of all disciplines who work in an international field.", 10.0),
			("EAP trainers can use this book in conjunction with: English for Academic Research:", 10.0),
			("A Guide for Teachers.", 10.0),
		]);
		let result = join_paragraphs(&lines, 10.0);
		assert_eq!(result.len(), 2, "got {result:?}");
		assert!(result[0].0.ends_with("international field."));
		assert!(result[1].0.starts_with("EAP trainers"));
	}

	#[test]
	fn join_paragraphs_merges_a_wrapped_list_item() {
		let lines = tight(&[
			("1) Look at your Inbox in your email account. Analyse 10-20 subject lines and", 9.5),
			("decide some criteria for judging how effective the subject lines are. Compare", 9.5),
			("your criteria with a colleague's.", 9.5),
		]);
		let result = join_paragraphs(&lines, 9.5);
		assert_eq!(result.len(), 1, "got {result:?}");
	}

	#[test]
	fn join_paragraphs_merges_continuation_lines() {
		let lines = tight(&[("The suggestion appears here.", 12.0), ("And here.", 12.0)]);
		let result = join_paragraphs(&lines, 12.0);
		assert_eq!(result.len(), 1);
		assert_eq!(result[0].0, "The suggestion appears here. And here.");
		assert!(!result[0].1);
	}

	/// No gap between any two lines, so a test about the text alone isn't accidentally testing the paragraph-gap rule.
	fn tight(lines: &[(&str, f64)]) -> Vec<Line> {
		let mut top = 700.0;
		let mut out = Vec::new();
		for (text, size) in lines {
			let bottom = top - size;
			out.push(Line { text: (*text).to_string(), size: *size, top, bottom, monospaced: false, bold: false });
			top = bottom - size * 0.2;
		}
		out
	}

	fn spaced(lines: &[(&str, f64, bool)]) -> Vec<Line> {
		let mut top = 700.0;
		let mut out = Vec::new();
		for (text, size, gap_above) in lines {
			if *gap_above {
				top -= size * 1.4;
			}
			let bottom = top - size;
			out.push(Line { text: (*text).to_string(), size: *size, top, bottom, monospaced: false, bold: false });
			top = bottom - size * 0.2;
		}
		out
	}

	fn mixed(lines: &[(&str, f64, bool)]) -> Vec<Line> {
		let mut out = tight(&lines.iter().map(|(text, size, _)| (*text, *size)).collect::<Vec<_>>());
		for (line, (.., monospaced)) in out.iter_mut().zip(lines) {
			line.monospaced = *monospaced;
		}
		out
	}

	// #833: a listing shares the prose's left edge and leading, so only its face sets it apart.
	#[test]
	fn join_paragraphs_keeps_a_monospaced_listing_apart() {
		let lines = mixed(&[
			("Listing 1: Five lines of Python", 10.0, false),
			("def greet(name):", 9.0, true),
			("# say hello", 9.0, true),
			("message = \"Hello, \" + name", 9.0, true),
			("print(message)", 9.0, true),
			("return message", 9.0, true),
			("The function above greets whoever it is handed, and returns", 10.0, false),
			("the greeting it built.", 10.0, false),
		]);
		let result = join_paragraphs(&lines, 10.0);
		let texts: Vec<&str> = result.iter().map(|(text, ..)| text.as_str()).collect();
		assert_eq!(
			texts,
			[
				"Listing 1: Five lines of Python",
				"def greet(name):",
				"# say hello",
				"message = \"Hello, \" + name",
				"print(message)",
				"return message",
				"The function above greets whoever it is handed, and returns the greeting it built."
			]
		);
	}

	#[test]
	fn join_paragraphs_still_joins_the_prose_around_a_listing() {
		let lines = mixed(&[("The suggestion appears here.", 12.0, false), ("And here.", 12.0, false)]);
		assert_eq!(join_paragraphs(&lines, 12.0).len(), 1);
	}

	// #813: a run of one-line paragraphs never stops short of a measure it never reaches, so only the gap tells them apart.
	#[test]
	fn join_paragraphs_breaks_where_a_document_leaves_a_gap() {
		let lines = spaced(&[
			("Before We Get Started", 12.0, false),
			("About the Author: Valentina Romano", 12.0, true),
			("Lesson One", 12.0, true),
			("Lesson Two", 12.0, true),
		]);
		let result = join_paragraphs(&lines, 12.0);
		let texts: Vec<&str> = result.iter().map(|(text, ..)| text.as_str()).collect();
		assert_eq!(texts, ["Before We Get Started", "About the Author: Valentina Romano", "Lesson One", "Lesson Two"]);
	}

	#[test]
	fn join_paragraphs_keeps_a_wrapped_paragraph_whole_across_the_gap_rule() {
		let lines = spaced(&[
			("INSTRUCTOR: Let's review. In the following exercise you will play the role of", 12.0, false),
			("the male speaker ONLY. Be sure to make your response before the male speaker", 12.0, false),
			("and then repeat his answer after him.", 12.0, false),
			("FEMALE: Parli italiano?", 12.0, true),
		]);
		let result = join_paragraphs(&lines, 12.0);
		assert_eq!(result.len(), 2, "got {result:?}");
		assert!(result[0].0.ends_with("repeat his answer after him."));
		assert_eq!(result[1].0, "FEMALE: Parli italiano?");
	}

	#[test]
	fn join_paragraphs_is_not_fooled_by_a_line_pdfium_doubled() {
		let mut lines = tight(&[
			("On a hot, muggy August afternoon during the summer of", 11.0),
			("1996, I was hard at work in a conference room of the", 11.0),
			("Ministry of Aviation Industries of China in Beijing, discussing human resources", 11.0),
			("AlliedSignal, the company for which I was then international", 11.0),
		]);
		lines[2].bottom -= 11.0;
		lines[3].top -= 11.0;
		lines[3].bottom -= 11.0;
		let result = join_paragraphs(&lines, 11.0);
		assert_eq!(result.len(), 1, "got {result:?}");
	}

	// #826.
	#[test]
	fn join_paragraphs_glues_a_drop_cap_to_its_paragraph() {
		let lines = tight(&[("T", 153.5), ("his morning, people all over the planet got out of bed.", 12.0)]);
		let result = join_paragraphs(&lines, 12.0);
		assert_eq!(result.len(), 1, "got {result:?}");
		assert_eq!(result[0].0, "This morning, people all over the planet got out of bed.");
		assert!(!result[0].1, "the paragraph it opens is not a heading");
	}

	#[test]
	fn join_paragraphs_leaves_a_chapter_number_alone() {
		let digit = tight(&[("4", 40.0), ("INCONVENIENT OR NOT", 24.0)]);
		assert_eq!(join_paragraphs(&digit, 11.0).len(), 2);
		// A roman numeral is a letter, so only the case of what follows tells them apart.
		let roman = tight(&[("I", 40.0), ("The Long Road Home", 24.0)]);
		assert_eq!(join_paragraphs(&roman, 11.0).len(), 2);
	}

	#[test]
	fn join_paragraphs_leaves_an_ordinary_capital_alone() {
		let lines = tight(&[("A", 12.0), ("small letter follows.", 12.0)]);
		assert_eq!(join_paragraphs(&lines, 12.0).len(), 1);
	}

	#[test]
	fn split_lines_keeps_every_line_apart() {
		let lines = tight(&[
			("Listing 1: Five lines of Python", 11.0),
			("def greet(name):", 11.0),
			("# say hello", 11.0),
			("message = \"Hello, \" + name", 11.0),
			("print(message)", 11.0),
			("return message", 11.0),
		]);
		let result = split_lines(&lines, 11.0);
		let texts: Vec<&str> = result.iter().map(|(text, ..)| text.as_str()).collect();
		assert_eq!(
			texts,
			[
				"Listing 1: Five lines of Python",
				"def greet(name):",
				"# say hello",
				"message = \"Hello, \" + name",
				"print(message)",
				"return message"
			]
		);
	}

	#[test]
	fn join_paragraphs_runs_the_same_listing_together() {
		let lines = tight(&[("def greet(name):", 11.0), ("# say hello", 11.0), ("return message", 11.0)]);
		assert_eq!(join_paragraphs(&lines, 11.0).len(), 1);
	}

	#[test]
	fn split_lines_still_marks_headings() {
		let lines = tight(&[("A Short Code Listing", 20.0), ("def greet(name):", 11.0)]);
		let result = split_lines(&lines, 11.0);
		assert_eq!(result.len(), 2);
		assert!(result[0].1, "the large line is a heading");
		assert!(!result[1].1, "the body line is not");
	}

	// A dropped blank line must not shift the indices after it.
	#[test]
	fn split_lines_reports_the_line_each_paragraph_came_from() {
		let lines = tight(&[("first", 11.0), ("   ", 11.0), ("third", 11.0)]);
		let result = split_lines(&lines, 11.0);
		assert_eq!(result.len(), 2);
		assert_eq!(result[0].2, 0);
		assert_eq!(result[1].2, 2);
	}

	#[test]
	fn join_paragraphs_flags_large_font_lines_as_headings() {
		let lines = tight(&[("Chapter One", 18.0), ("This is the body text of the document.", 12.0)]);
		let result = join_paragraphs(&lines, 12.0);
		assert_eq!(result.len(), 2);
		assert_eq!(result[0].0, "Chapter One");
		assert!(result[0].1);
		assert_eq!(result[1].0, "This is the body text of the document.");
		assert!(!result[1].1);
	}

	fn weighted(lines: &[(&str, f64, bool)]) -> Vec<Line> {
		let mut out = tight(&lines.iter().map(|(text, size, _)| (*text, *size)).collect::<Vec<_>>());
		for (line, (_, _, bold)) in out.iter_mut().zip(lines) {
			line.bold = *bold;
		}
		out
	}

	/// Page 11 of the PDF attached to #993: section titles in bold Segoe UI at 13.5pt, between paragraphs of 15.5pt body text.
	#[test]
	fn a_bold_title_smaller_than_the_body_is_a_heading() {
		let lines = weighted(&[
			("and the surgery. Some things just happen.", 15.5, false),
			("Emergency Management", 13.5, true),
			("The events that surround the injury set the stage for what follows,", 15.5, false),
			("and the writer has to know them.", 15.5, false),
		]);
		let result = join_paragraphs(&lines, 15.5);
		assert_eq!(result.len(), 3);
		assert_eq!(result[1].0, "Emergency Management");
		assert!(result[1].1, "the bold title is a heading");
		assert!(!result[0].1 && !result[2].1, "the body around it is not");
	}

	#[test]
	fn a_bold_sentence_is_emphasis_not_a_heading() {
		let lines = weighted(&[
			("Never move a patient with a suspected spinal injury.", 12.0, true),
			("The body follows.", 12.0, false),
		]);
		assert!(!join_paragraphs(&lines, 12.0)[0].1);
	}

	/// The Apple Developer Program License Agreement opens with a paragraph of bold capitals whose lines each look like a title.
	#[test]
	fn a_bold_paragraph_is_not_a_run_of_headings() {
		let lines = weighted(&[
			("PLEASE READ THE FOLLOWING APPLE DEVELOPER PROGRAM LICENSE AGREEMENT", 10.0, true),
			("TERMS AND CONDITIONS CAREFULLY BEFORE DOWNLOADING OR USING THE APPLE", 10.0, true),
			("SOFTWARE. THESE TERMS CONSTITUTE A LEGAL AGREEMENT.", 10.0, true),
			("You would like to use the Apple Software to develop one or more Applications", 10.0, false),
			("for Apple-branded products, and Apple is willing to grant You a license.", 10.0, false),
			("And more body text follows here, set in the regular weight of the face.", 10.0, false),
			("And more body text follows here, set in the regular weight of the face.", 10.0, false),
		]);
		assert!(join_paragraphs(&lines, 10.0).iter().all(|(_, is_heading, _)| !is_heading));
	}

	#[test]
	fn a_label_a_contents_line_and_a_row_of_figures_are_not_titles() {
		assert!(super::reads_as_a_title("Emergency Management"));
		assert!(super::reads_as_a_title("3.1 Service Agreement"));
		assert!(!super::reads_as_a_title("Tip:"));
		assert!(!super::reads_as_a_title("9 UPGRADE PROCEDURE FROM REV E TO REV H....... 53"));
		assert!(!super::reads_as_a_title("18-19 19-20 20-21 4.000 6.000"));
	}

	#[test]
	fn bold_means_nothing_on_a_page_mostly_set_in_bold() {
		let lines =
			weighted(&[("Claim Form", 12.0, true), ("Patient Name", 12.0, true), ("Date of Birth", 12.0, false)]);
		assert!(join_paragraphs(&lines, 12.0).iter().all(|(_, is_heading, _)| !is_heading));
	}
}
