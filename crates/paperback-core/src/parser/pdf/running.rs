//! Recognizing the running headers and footers a book repeats at the top and bottom of every
//! page - the book's title, the chapter's title, the page number, a typesetter's slug - so that
//! the extracted text is the text of the book and not the furniture around it.
//!
//! An untagged PDF gives no sign of which lines these are: they arrive in the same text stream
//! as the body, at the page's edges. What gives them away is that they repeat. Take the first
//! and last lines of every page, blank out the digits (the page number is the part that
//! changes), and a line whose result recurs across pages is furniture. A per-chapter running
//! head recurs only over its own chapter, so a handful of pages is enough to count.
//!
//! Before this, those lines were extracted as text: 396 of them in one 269-page book. They read
//! as a paragraph of their own between two paragraphs, and once wrapped lines began joining into
//! paragraphs properly they started landing inside one, mid-sentence.

use std::collections::{HashMap, HashSet};

/// How many lines at each edge of a page can be running text. Two covers a page number on its
/// own line above or below the running head itself.
pub(super) const EDGE_LINES: usize = 2;

/// How many pages must share a line before it is furniture rather than text. A running head
/// that changes with the chapter appears only on that chapter's pages, so this cannot be a
/// fraction of the document; it is instead a plain count high enough that body text does not
/// reach it by chance.
const MIN_PAGES: usize = 4;

/// The lines at one page's edges, as [`detect`] wants them: the first and last [`EDGE_LINES`]
/// of the page, which is all a running header or footer can be.
pub(super) struct PageEdges {
	pub first: Vec<String>,
	pub last: Vec<String>,
}

impl PageEdges {
	pub(super) fn of(lines: &[(String, f64, f64)]) -> Self {
		let text = |line: &(String, f64, f64)| line.0.clone();
		Self {
			first: lines.iter().take(EDGE_LINES).map(text).collect(),
			last: lines.iter().rev().take(EDGE_LINES).map(text).collect(),
		}
	}
}

/// What a line looks like with the parts that vary from page to page taken out: digits become
/// `#`, and a run of them collapses, so "DREAM CAR 17" and "DREAM CAR 18" are one signature.
/// Roman numerals are left alone - too many real words are made of their letters.
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

/// How much larger than the document's body text a line may be and still be furniture. A
/// running head is set in the body's size or smaller; a section heading that opens a page is
/// set larger, and repeats across the chapters exactly as a running head does ("1.1 What's the
/// buzz?" opens thirteen chapters of the PDF from #808), so without this it would be mistaken
/// for one and the book would lose its headings.
const MAX_BODY_RATIO: f64 = 1.05;

/// The set of signatures that recur at the pages' edges, and the size the body text is set in.
pub(super) struct RunningText {
	signatures: HashSet<String>,
	body_font_size: f64,
}

impl RunningText {
	/// Whether a line at a page edge is running text. A line is only ever tested when it sits
	/// within [`EDGE_LINES`] of an edge, so a sentence in the body that happens to read like a
	/// running head is never at risk.
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

/// Survey every page's edges and return the signatures that recur. `body_font_size` is the size
/// the document's text is mostly set in, which keeps a heading from being taken for furniture.
pub(super) fn detect(pages: &[PageEdges], body_font_size: f64) -> RunningText {
	let mut counts: HashMap<String, usize> = HashMap::new();
	for page in pages {
		// One page can only vote once for a signature, so a page repeating a line at both edges
		// does not carry it on its own.
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
	let signatures = counts.into_iter().filter(|(_, count)| *count >= MIN_PAGES).map(|(sig, _)| sig).collect();
	RunningText { signatures, body_font_size }
}

#[cfg(test)]
mod tests {
	use super::{PageEdges, detect, signature};

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

	const BODY: f64 = 11.0;

	/// The running heads and footer of the book in #813's follow-up: a verso carrying the book's
	/// title, a recto carrying the chapter's, and a typesetter's slug at the foot of each page.
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

	/// A bare page number recurs like any other running text, its digits being the whole of what
	/// changes from page to page.
	#[test]
	fn detect_finds_a_bare_page_number() {
		let pages: Vec<PageEdges> = (1..=10).map(|n| edges(&[], &[&n.to_string()])).collect();
		let running = detect(&pages, BODY);
		assert!(running.contains("204", BODY));
		assert!(!running.contains("Chapter 204", BODY));
	}

	/// A chapter's running head covers only its own pages, so the count that decides has to be
	/// low enough to catch it.
	#[test]
	fn detect_finds_a_head_that_runs_for_one_chapter_only() {
		let mut pages: Vec<PageEdges> = (1..=40).map(|n| edges(&[&format!("EARLY CHAPTER {n}")], &[])).collect();
		pages.extend((41..=45).map(|n| edges(&[&format!("A LATE SHORT CHAPTER {n}")], &[])));
		let running = detect(&pages, BODY);
		assert!(running.contains("A LATE SHORT CHAPTER 41", BODY));
	}

	/// A line that recurs on only a page or two is not furniture.
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

	/// A section heading that opens a page in every chapter repeats exactly as a running head
	/// does. It is set larger than the body, and that is what keeps it.
	#[test]
	fn a_heading_that_repeats_every_chapter_is_not_furniture() {
		let pages: Vec<PageEdges> = (1..=13).map(|n| edges(&[&format!("{n}.1 What's the buzz?")], &[])).collect();
		let running = detect(&pages, BODY);
		assert!(running.contains("1.1 What's the buzz?", BODY), "it does look like a running head");
		assert!(!running.contains("1.1 What's the buzz?", BODY * 1.2), "but a heading's size saves it");
	}
}
