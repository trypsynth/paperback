//! Turning the lines of a page into paragraphs.
//!
//! A PDF says where every glyph sits and nothing about what any of it means, so where one
//! paragraph ends and the next begins has to be worked out from how the page is set: the
//! clear space above a line, the size it is set in, the face it is set in, and how far the
//! line before it fell short of the measure. Nothing here talks to pdfium; it all reads
//! [`Line`]s that [`super::text`] has already measured.

use std::mem;

use super::text::{Line, is_cjk};
use crate::util::text::{collapse_whitespace, display_len, trim_string};

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
fn full_line_len(lines: &[(Line, bool)]) -> usize {
	let mut lengths: Vec<usize> =
		lines.iter().map(|(line, ..)| display_len(&line.text)).filter(|len| *len > 0).collect();
	if lengths.is_empty() {
		return 0;
	}
	lengths.sort_unstable();
	lengths[(lengths.len() - 1) * FULL_LINE_PERCENTILE_NUMERATOR / FULL_LINE_PERCENTILE_DENOMINATOR]
}

/// How much clear space above a line means it opens a paragraph, as a fraction of its own
/// point size. Set from the four documents of #813 and #826: within a paragraph the space
/// between two lines runs to about two thirds of a line's size, and between paragraphs it
/// starts at about five fourths, so anything past a full size is a break with room to spare
/// either side.
const PARAGRAPH_GAP_RATIO: f64 = 1.0;

/// How much larger than the body a single letter must be set to read as a drop cap.
const DROP_CAP_RATIO: f64 = 2.0;

/// Glue each drop cap onto the paragraph it opens.
///
/// A drop cap is set as its own line, several times the body size, and every size-based test
/// reads it as a heading of one letter. What gives it away is the line under it: a drop cap
/// is the first letter of a word, so what follows carries on in lower case, where a chapter
/// number stands over a title in capitals. A digit is never a drop cap, which keeps a
/// numbered chapter opening out of this entirely.
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

/// The point size at which a line reads as a heading rather than as body text.
fn heading_threshold(body_font_size: f64) -> f64 {
	if body_font_size > 0.0 { body_font_size * HEADING_FONT_RATIO } else { f64::INFINITY }
}

/// Reads every line as a paragraph of its own, for a reader who has turned paragraph joining
/// off.
///
/// Nothing in a PDF says whether five lines at the same left edge are five lines of code or
/// one wrapped sentence, so a document whose line breaks are the content (a listing, a poem,
/// a transcript) can only be read with the joining out of the way. Headings are still marked,
/// because that is a question about size and not about where a paragraph ends.
pub(super) fn split_lines(raw_lines: &[Line], body_font_size: f64) -> Vec<(String, bool, usize)> {
	let heading_threshold = heading_threshold(body_font_size);
	raw_lines
		.iter()
		.enumerate()
		.filter_map(|(index, line)| {
			let trimmed = trim_string(&collapse_whitespace(&line.text));
			let len = display_len(&trimmed);
			if len == 0 {
				return None;
			}
			Some((trimmed, line.size >= heading_threshold && len <= HEADING_MAX_LEN, index))
		})
		.collect()
}

/// The third field of each paragraph is the index, into `raw_lines`, of the line it starts
/// with. An untagged page needs it to place its images: the paragraphs no longer say where on
/// the page they were set, and that line does.
pub(super) fn join_paragraphs(raw_lines: &[Line], body_font_size: f64) -> Vec<(String, bool, usize)> {
	let heading_threshold = heading_threshold(body_font_size);
	let raw_lines = merge_drop_caps(raw_lines, body_font_size);
	let lines: Vec<(Line, bool)> = raw_lines
		.iter()
		.map(|line| {
			let trimmed = trim_string(&collapse_whitespace(&line.text));
			let len = display_len(&trimmed);
			let is_heading_line = line.size >= heading_threshold && len > 0 && len <= HEADING_MAX_LEN;
			(Line { text: trimmed, ..line.clone() }, is_heading_line)
		})
		.collect();
	// Two ways a line can end a paragraph, needing different amounts of evidence. A line that
	// ends a sentence and stops any way short of the measure has ended the paragraph with it. A
	// line that ends mid-sentence has to fall well short before the next line starting like a
	// fresh sentence counts for anything, since a capital there is as likely to open a name.
	let full_line = full_line_len(&lines);
	let ended_sentence_threshold = full_line * 19 / 20;
	let short_line_threshold = full_line * 3 / 4;
	let mut paragraphs: Vec<(String, bool, usize)> = Vec::new();
	let mut current_paragraph = String::new();
	let mut current_is_heading = false;
	let mut current_heading_size = 0.0f64;
	let mut current_start_line = 0usize;
	let mut last_line_len = 0usize;
	let mut last_line_ends_with_punctuation = false;
	// The bottom edge of the last line that carried text, which the whitespace before the
	// next one is measured down from. Infinity until there is one, so the first line of a
	// page never reads as following a gap.
	let mut last_line_bottom = f64::INFINITY;
	let mut previous_was_monospaced = false;
	for (line_index, (line_info, is_heading_line)) in lines.iter().enumerate() {
		let Line { text: line, size, top, bottom, monospaced } = line_info;
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
		let len = display_len(line);
		if current_paragraph.is_empty() {
			current_paragraph = line.clone();
			current_is_heading = *is_heading_line;
			current_heading_size = *size;
			current_start_line = line_index;
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
			// than becoming one heading per line, as long as the size does not change. A line
			// that opens with its own number is a heading of its own even so, which is what
			// keeps a numbered outline from running its sections together. Every other heading
			// boundary, and every list item, starts a new paragraph; otherwise the previous line
			// does when it stopped short of the measure.
			let continues_heading =
				*is_heading_line && current_is_heading && (*size - current_heading_size).abs() < f64::EPSILON;
			let previous_line_ended_paragraph = (last_line_ends_with_punctuation
				&& last_line_len < ended_sentence_threshold)
				|| (last_line_len < short_line_threshold && (starts_with_uppercase || !starts_with_alpha));
			// Extra whitespace above a line is the one signal a document sets deliberately and
			// consistently, and it is what separates a run of one-line paragraphs that every
			// length-based test reads as one block. It is measured from the previous line's
			// lowest glyph rather than from its top, because pdfium runs two visual lines
			// together often enough that a top-to-top pitch is doubled all over a normal
			// paragraph and would break it apart.
			let opened_by_a_gap = *size > 0.0 && last_line_bottom - *top > *size * PARAGRAPH_GAP_RATIO;
			// A line set in a monospaced face is code, or a table, or something else laid out by
			// column, and its own line breaks are the content. It joins with nothing, and nothing
			// joins onto it. This is the one case geometry cannot see: a listing is set at the
			// same left edge and the same leading as the prose around it.
			let set_apart_by_its_face = *monospaced || previous_was_monospaced;
			let break_paragraph = is_list_item
				|| is_numbered
				|| opened_by_a_gap
				|| set_apart_by_its_face
				|| (!continues_heading && (*is_heading_line || current_is_heading || previous_line_ended_paragraph));
			if break_paragraph {
				paragraphs.push((mem::take(&mut current_paragraph), current_is_heading, current_start_line));
				current_paragraph = line.clone();
				current_is_heading = *is_heading_line;
				current_heading_size = *size;
				current_start_line = line_index;
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
	paragraphs
}

#[cfg(test)]
mod tests {
	use super::{Line, full_line_len, join_paragraphs, split_lines};

	/// The lines of page 67 of the PDF attached to #813, with the sizes and the doubled lines
	/// pdfium reports for it: a chapter number, a four-line chapter title, an epigraph over two
	/// lines, and a paragraph whose second line opens with a year.
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

	/// The page's measure comes from a line that fills it, not from one of the doubled lines
	/// pdfium produced, which are nearly twice as long.
	#[test]
	fn full_line_len_ignores_doubled_lines() {
		let lines: Vec<(Line, bool)> = issue_813_page().into_iter().map(|line| (line, false)).collect();
		let full = full_line_len(&lines);
		assert!((53..=64).contains(&full), "expected the length of a full line, got {full}");
	}

	/// #813: the body paragraph survives its second line opening with a year and its later lines
	/// opening with capitals, and the chapter title is one heading rather than one per line.
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

	/// A numbered outline sets every level in one size, so the sections must not run together
	/// the way the lines of a single title do.
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

	/// The chapter number is set in its own size, so it does not join the title below it.
	#[test]
	fn join_paragraphs_keeps_headings_of_different_sizes_apart() {
		let lines = tight(&[("PART ONE", 30.0), ("Getting Started", 20.0)]);
		let result = join_paragraphs(&lines, 11.0);
		assert_eq!(result.len(), 2);
	}

	/// A line that ends a sentence ends the paragraph even when it reaches most of the way across
	/// the measure - the case a single short-line threshold gets wrong.
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

	/// A paragraph that pdfium hands back as wrapped lines is one paragraph, not one per line.
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

	/// Lay `lines` out down a page with ordinary leading and no gap between any two, so a test
	/// written about the text alone is not accidentally testing the paragraph-gap rule.
	fn tight(lines: &[(&str, f64)]) -> Vec<Line> {
		let mut top = 700.0;
		let mut out = Vec::new();
		for (text, size) in lines {
			let bottom = top - size;
			out.push(Line { text: (*text).to_string(), size: *size, top, bottom, monospaced: false });
			// A fifth of a line of clear space: what sits between two lines of one paragraph.
			top = bottom - size * 0.2;
		}
		out
	}

	/// Lay `lines` out with a full line of clear space above each one that is marked, which is
	/// what a document puts between two paragraphs.
	fn spaced(lines: &[(&str, f64, bool)]) -> Vec<Line> {
		let mut top = 700.0;
		let mut out = Vec::new();
		for (text, size, gap_above) in lines {
			if *gap_above {
				top -= size * 1.4;
			}
			let bottom = top - size;
			out.push(Line { text: (*text).to_string(), size: *size, top, bottom, monospaced: false });
			top = bottom - size * 0.2;
		}
		out
	}

	/// Lay `lines` out with the same tight leading as [`tight`], marking which are set in a
	/// monospaced face.
	fn mixed(lines: &[(&str, f64, bool)]) -> Vec<Line> {
		let mut out = tight(&lines.iter().map(|(text, size, _)| (*text, *size)).collect::<Vec<_>>());
		for (line, (.., monospaced)) in out.iter_mut().zip(lines) {
			line.monospaced = *monospaced;
		}
		out
	}

	// #833: a listing is set at the same left edge and the same leading as the prose around
	// it, so geometry cannot see it. The face can: the lines are monospaced and the prose is
	// not.
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

	// The prose on either side of a listing still joins as prose.
	#[test]
	fn join_paragraphs_still_joins_the_prose_around_a_listing() {
		let lines = mixed(&[("The suggestion appears here.", 12.0, false), ("And here.", 12.0, false)]);
		assert_eq!(join_paragraphs(&lines, 12.0).len(), 1);
	}

	// #813 again: a run of one-line paragraphs is what every length test reads as one block,
	// since no line among them stops short of a measure they never reach. The space a document
	// leaves above each one is the only thing that tells them apart.
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

	// The same document sets a wrapped paragraph with no such gap, and that has to stay whole.
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

	// pdfium runs two visual lines together into one line of text all over a normal page. Such
	// a line reaches a whole line lower than its neighbours, so measuring the space above the
	// next line from its top rather than its bottom would break paragraphs everywhere.
	#[test]
	fn join_paragraphs_is_not_fooled_by_a_line_pdfium_doubled() {
		let mut lines = tight(&[
			("On a hot, muggy August afternoon during the summer of", 11.0),
			("1996, I was hard at work in a conference room of the", 11.0),
			("Ministry of Aviation Industries of China in Beijing, discussing human resources", 11.0),
			("AlliedSignal, the company for which I was then international", 11.0),
		]);
		// The third line is two visual lines run together, so its box reaches a line lower.
		lines[2].bottom -= 11.0;
		lines[3].top -= 11.0;
		lines[3].bottom -= 11.0;
		let result = join_paragraphs(&lines, 11.0);
		assert_eq!(result.len(), 1, "got {result:?}");
	}

	// #826: a drop cap is one big letter set beside the paragraph it opens, and reads as a
	// heading of one letter to anything that goes by size.
	#[test]
	fn join_paragraphs_glues_a_drop_cap_to_its_paragraph() {
		let lines = tight(&[("T", 153.5), ("his morning, people all over the planet got out of bed.", 12.0)]);
		let result = join_paragraphs(&lines, 12.0);
		assert_eq!(result.len(), 1, "got {result:?}");
		assert_eq!(result[0].0, "This morning, people all over the planet got out of bed.");
		assert!(!result[0].1, "the paragraph it opens is not a heading");
	}

	// A chapter number is also one large character on a line of its own. What separates it
	// from a drop cap is that a title follows, not the rest of a word.
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
		// Body-sized, so nothing to do with a drop cap however the next line starts.
		let lines = tight(&[("A", 12.0), ("small letter follows.", 12.0)]);
		assert_eq!(join_paragraphs(&lines, 12.0).len(), 1);
	}

	// #813: nothing in a PDF separates five lines of code from five wrapped lines of prose,
	// so a reader whose document is a listing turns the joining off and gets the lines back.
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

	// The same lines run together when the joining is on, which is the whole reason for the
	// switch.
	#[test]
	fn join_paragraphs_runs_the_same_listing_together() {
		let lines = tight(&[("def greet(name):", 11.0), ("# say hello", 11.0), ("return message", 11.0)]);
		assert_eq!(join_paragraphs(&lines, 11.0).len(), 1);
	}

	// A heading is a question about size, not about where a paragraph ends, so it is still
	// marked with the joining off. Navigating by heading has to keep working.
	#[test]
	fn split_lines_still_marks_headings() {
		let lines = tight(&[("A Short Code Listing", 20.0), ("def greet(name):", 11.0)]);
		let result = split_lines(&lines, 11.0);
		assert_eq!(result.len(), 2);
		assert!(result[0].1, "the large line is a heading");
		assert!(!result[1].1, "the body line is not");
	}

	// Each paragraph reports the line it came from, which is what places the images of an
	// untagged page. A blank line is dropped and must not shift the ones after it.
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
}
