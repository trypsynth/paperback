//! Untagged PDFs give no sign of their running heads, and before this one 269-page book extracted 396 of them as text, which landed mid-sentence once wrapped lines were joined.

use std::collections::{HashMap, HashSet};

use super::text::Line;

/// Two covers a page number on its own line above or below the running head itself.
pub(super) const EDGE_LINES: usize = 2;

/// A count rather than a share of the document, because a running head that changes with the chapter appears only on that chapter's pages.
const MIN_PAGES: usize = 4;

pub(super) struct PageEdges {
	pub first: Vec<String>,
	pub last: Vec<String>,
}

impl PageEdges {
	pub(super) fn of(lines: &[Line]) -> Self {
		let text = |line: &Line| line.text.clone();
		Self {
			first: lines.iter().take(EDGE_LINES).map(text).collect(),
			last: lines.iter().rev().take(EDGE_LINES).map(text).collect(),
		}
	}

	pub(super) fn of_texts(lines: &[String]) -> Self {
		Self {
			first: lines.iter().take(EDGE_LINES).cloned().collect(),
			last: lines.iter().rev().take(EDGE_LINES).cloned().collect(),
		}
	}
}

/// Roman numerals are left alone: too many real words are made of their letters.
fn signature(line: &str) -> String {
	let mut out = String::with_capacity(line.len());
	let mut in_digits = false;
	for ch in line.chars() {
		if ch.is_numeric() {
			if !in_digits {
				out.push('#');
				in_digits = true;
			}
		} else {
			out.push(ch);
			in_digits = false;
		}
	}
	out
}

/// A section heading opening a page repeats across chapters like a running head ("1.1 What's the buzz?" opens thirteen chapters of the PDF from #808), but is set larger than the body.
const MAX_BODY_RATIO: f64 = 1.05;

pub(super) struct RunningText {
	signatures: HashSet<String>,
	body_font_size: f64,
}

impl RunningText {
	/// Keeps the body font size, since that still tells a heading from body text.
	pub(super) fn recognizing_nothing(mut self) -> Self {
		self.signatures.clear();
		self
	}

	pub(super) fn contains(&self, line: &str, font_size: f64) -> bool {
		let trimmed = line.trim();
		if trimmed.is_empty() {
			return false;
		}
		if self.body_font_size > 0.0 && font_size > self.body_font_size * MAX_BODY_RATIO {
			return false;
		}
		self.signatures.contains(&signature(trimmed))
	}

	pub(super) fn len(&self) -> usize {
		self.signatures.len()
	}
}

pub(super) fn detect(pages: &[PageEdges], body_font_size: f64) -> RunningText {
	let signatures =
		edge_counts(pages).into_iter().filter(|(_, count)| *count >= MIN_PAGES).map(|(sig, _)| sig).collect();
	RunningText { signatures, body_font_size }
}

/// A tagged page's edge is a whole block, so at [`MIN_PAGES`] a manual lost the sentence opening seven of its sections and a book the figure placeholder ending a dozen pages; real tagged furniture is on nearly every page.
const MIN_TAGGED_SHARE: f64 = 0.6;

pub(super) fn detect_tagged(pages: &[PageEdges]) -> RunningText {
	#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, reason = "a share of the page count")]
	let needed = (pages.len() as f64 * MIN_TAGGED_SHARE).ceil() as usize;
	let signatures = edge_counts(pages)
		.into_iter()
		.filter(|(_, count)| *count >= MIN_PAGES && *count >= needed)
		.map(|(sig, _)| sig)
		.collect();
	// No size is needed for a tagged line: the tag that says it is a heading keeps it safe.
	RunningText { signatures, body_font_size: 0.0 }
}

/// One vote per page, so a page repeating a line at both edges does not carry it on its own.
fn edge_counts(pages: &[PageEdges]) -> HashMap<String, usize> {
	let mut counts: HashMap<String, usize> = HashMap::new();
	for page in pages {
		let mut seen = HashSet::new();
		for line in page.first.iter().chain(page.last.iter()) {
			let trimmed = line.trim();
			if trimmed.is_empty() {
				continue;
			}
			let signature = signature(trimmed);
			if seen.insert(signature.clone()) {
				*counts.entry(signature).or_default() += 1;
			}
		}
	}
	counts
}

#[cfg(test)]
mod tests {
	use std::collections::HashSet;

	use super::{PageEdges, RunningText, detect, signature};

	fn edges(first: &[&str], last: &[&str]) -> PageEdges {
		PageEdges {
			first: first.iter().map(|s| (*s).to_string()).collect(),
			last: last.iter().map(|s| (*s).to_string()).collect(),
		}
	}

	#[test]
	fn signature_blanks_out_the_page_number() {
		assert_eq!(signature("DREAM CAR 17"), signature("DREAM CAR 209"));
		assert_ne!(signature("DREAM CAR 17"), signature("DREAM ON 17"));
	}

	#[test]
	fn a_survey_asked_to_recognise_nothing_takes_nothing_out() {
		let mut survey = RunningText { signatures: HashSet::new(), body_font_size: BODY };
		survey.signatures.insert("RUNNING HEADER".to_string());
		assert!(survey.contains("RUNNING HEADER", BODY));
		let none = survey.recognizing_nothing();
		assert!(!none.contains("RUNNING HEADER", BODY));
		assert!((none.body_font_size - BODY).abs() < f64::EPSILON, "the body size has to survive");
	}

	const BODY: f64 = 11.0;

	/// The book in #813's follow-up: the book's title on versos, the chapter's on rectos, and a typesetter's slug at each foot.
	#[test]
	fn detect_finds_running_heads_and_footers() {
		let pages: Vec<PageEdges> = (1..=10)
			.map(|n| {
				let head = if n % 2 == 0 { "Carjacked".to_string() } else { format!("DREAM CAR {n}") };
				edges(&[&head], &[&format!("01 lutz-fernandez text:03 wasik ch1 11/9/09 11:21 AM Page {n}")])
			})
			.collect();
		let running = detect(&pages, BODY);
		assert_eq!(running.len(), 3, "the two running heads and the footer");
		assert!(running.contains("Carjacked", BODY));
		assert!(running.contains("DREAM CAR 17", BODY));
		assert!(running.contains("01 lutz-fernandez text:03 wasik ch1 11/9/09 11:21 AM Page 204", BODY));
		assert!(!running.contains("On a hot, muggy August afternoon during the summer of", BODY));
	}

	#[test]
	fn detect_finds_a_bare_page_number() {
		let pages: Vec<PageEdges> = (1..=10).map(|n| edges(&[], &[&n.to_string()])).collect();
		let running = detect(&pages, BODY);
		assert!(running.contains("204", BODY));
		assert!(!running.contains("Chapter 204", BODY));
	}

	#[test]
	fn detect_finds_a_head_that_runs_for_one_chapter_only() {
		let mut pages: Vec<PageEdges> = (1..=40).map(|n| edges(&[&format!("EARLY CHAPTER {n}")], &[])).collect();
		pages.extend((41..=45).map(|n| edges(&[&format!("A LATE SHORT CHAPTER {n}")], &[])));
		let running = detect(&pages, BODY);
		assert!(running.contains("A LATE SHORT CHAPTER 41", BODY));
	}

	#[test]
	fn detect_leaves_a_line_that_barely_recurs_alone() {
		let pages: Vec<PageEdges> = (1..=20)
			.map(
				|n| if n <= 2 { edges(&["A repeated opening"], &[]) } else { edges(&[&format!("Body line {n}")], &[]) },
			)
			.collect();
		let running = detect(&pages, BODY);
		assert!(!running.contains("A repeated opening", BODY));
	}

	#[test]
	fn a_heading_that_repeats_every_chapter_is_not_furniture() {
		let pages: Vec<PageEdges> = (1..=13).map(|n| edges(&[&format!("{n}.1 What's the buzz?")], &[])).collect();
		let running = detect(&pages, BODY);
		assert!(running.contains("1.1 What's the buzz?", BODY), "it does look like a running head");
		assert!(!running.contains("1.1 What's the buzz?", BODY * 1.2), "but a heading's size saves it");
	}
}
