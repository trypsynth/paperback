use std::collections::{HashMap, HashSet};

use super::super::text::{
	LINE_END_HYPHEN, PageEvidence, SpaceFilter, char_top, context_chars, page_chars, reorder_run,
};
use crate::pdf_engine::{PageObject, PdfTextPage, Tag, TagTree};

const MCID_KEY: &str = "MCID";

/// Some PDFs advertise a structure tree while leaving their text essentially untagged (no ids, or only `/Artifact` marks).
pub(super) const MIN_MCID_COVERAGE: f64 = 0.5;

#[derive(Default)]
pub(super) struct TreeFacts {
	pub referenced: HashSet<i32>,
	/// Ids named only once are not in it.
	pub repeats: HashMap<i32, usize>,
	pub claims_figures: bool,
}

impl TreeFacts {
	pub(super) fn of(struct_tree: &TagTree) -> Self {
		let mut facts = Self::default();
		for i in 0..struct_tree.child_count() {
			if let Some(child) = struct_tree.child(i) {
				facts.gather(&child);
			}
		}
		facts
	}

	pub(super) fn note_reference(&mut self, mcid: i32) {
		if !self.referenced.insert(mcid) {
			*self.repeats.entry(mcid).or_default() += 1;
		}
	}

	fn gather(&mut self, elem: &Tag) {
		if elem.kind().unwrap_or_default() == "Figure" {
			self.claims_figures = true;
		}
		let count = elem.child_count();
		for i in 0..count {
			if let Some(child) = elem.child(i) {
				self.gather(&child);
			} else if let Some(mcid) = elem.child_mcid(i) {
				self.note_reference(mcid);
			}
		}
	}
}

pub(super) struct PageMarkedContent {
	pub text: HashMap<i32, String>,
	pub tops: HashMap<i32, f64>,
	pub coverage: f64,
}

pub(super) struct PageText<'a> {
	pub text: &'a HashMap<i32, String>,
	read: HashSet<i32>,
}

impl<'a> PageText<'a> {
	pub(super) fn new(text: &'a HashMap<i32, String>) -> Self {
		Self { text, read: HashSet::new() }
	}

	pub(super) fn take(&mut self, mcid: i32) -> Option<&'a str> {
		if !self.read.insert(mcid) {
			return None;
		}
		self.text.get(&mcid).map(String::as_str)
	}
}

/// `want_tops` costs one more lookup per id, so only a page placing images by height asks for it.
pub(super) fn read(
	text_page: &PdfTextPage,
	facts: &TreeFacts,
	want_tops: bool,
	evidence: &mut PageEvidence,
) -> PageMarkedContent {
	let mut content = PageMarkedContent { text: HashMap::new(), tops: HashMap::new(), coverage: 1.0 };
	let mut real_char_count: usize = 0;
	let mut mcid_char_count: usize = 0;
	let want_order = !facts.repeats.is_empty();
	let mut order: Vec<i32> = Vec::new();
	let mut artifacts: HashSet<i32> = HashSet::new();
	let mut seen: HashSet<i32> = HashSet::new();
	if let Some(char_count) = text_page.char_count() {
		let mut current_mcid = -1;
		// Kept with their pdfium index so RTL runs can be reordered visual to logical per run.
		let mut current_chars: Vec<(char, i32)> = Vec::new();
		let spaces = SpaceFilter::new(char_count);
		let page = page_chars(text_page, char_count);
		let context = context_chars(&page);
		evidence.add_chars(&context);
		for (at, (i, &ch)) in (0..char_count).zip(page.iter()).enumerate() {
			if let Some(ch) = ch {
				let ch = match ch {
					LINE_END_HYPHEN if evidence.keeps_line_end_hyphen(&context, at) => '-',
					_ => ch,
				};
				if (ch.is_control() && !matches!(ch, '\n' | '\r' | '\t')) || ch == '\u{00AD}' {
					continue;
				}
				// A space the page never renders would land mid-word here just as in plain extraction.
				if ch == ' ' && spaces.hides(text_page, i) {
					continue;
				}
				let is_generated = text_page.is_generated(i);
				let mut char_mcid = -1;
				if !is_generated && let Some(obj) = text_page.text_object(i) {
					char_mcid = object_mcid(&obj, &facts.referenced);
					if want_order && char_mcid >= 0 && char_mcid != current_mcid && seen.insert(char_mcid) {
						order.push(char_mcid);
						if !facts.referenced.contains(&char_mcid) && is_artifact(&obj) {
							artifacts.insert(char_mcid);
						}
					}
				}
				if !is_generated && !ch.is_whitespace() {
					real_char_count += 1;
					if char_mcid >= 0 {
						mcid_char_count += 1;
					}
				}
				if char_mcid >= 0
					&& want_tops
					&& !content.tops.contains_key(&char_mcid)
					&& let Some(top) = char_top(text_page, i)
				{
					content.tops.insert(char_mcid, top);
				}
				if char_mcid >= 0 && char_mcid != current_mcid {
					if current_mcid >= 0 && !current_chars.is_empty() {
						content.text.entry(current_mcid).or_default().push_str(&reorder_run(text_page, &current_chars));
					}
					current_chars.clear();
					current_mcid = char_mcid;
				}
				current_chars.push((ch, i));
			}
		}
		if current_mcid >= 0 && !current_chars.is_empty() {
			content.text.entry(current_mcid).or_default().push_str(&reorder_run(text_page, &current_chars));
		}
	}
	if real_char_count > 0 {
		content.coverage = mcid_char_count as f64 / real_char_count as f64;
	}
	fold_repeated_references(&order, &artifacts, facts, &mut content.text);
	content
}

/// A paragraph running onto a new page names that page's first id once more for each piece of text it lost, so that many unnamed non-artifact ids after it are folded back in.
pub(super) fn fold_repeated_references(
	order: &[i32],
	artifacts: &HashSet<i32>,
	facts: &TreeFacts,
	text: &mut HashMap<i32, String>,
) {
	for (&mcid, &extra) in &facts.repeats {
		let Some(start) = order.iter().position(|id| *id == mcid) else { continue };
		let lost = order[start + 1..]
			.iter()
			.copied()
			.filter(|id| !artifacts.contains(id))
			.take_while(|id| !facts.referenced.contains(id))
			.take(extra);
		for id in lost {
			if let Some(found) = text.remove(&id) {
				text.entry(mcid).or_default().push_str(&found);
			}
		}
	}
}

/// Apple Pages nests marks (the list's wraps the item's wraps the label's), so the outermost id pdfium reports leaves the ids the tree points at empty; the innermost referenced one is taken.
fn object_mcid(obj: &PageObject, referenced: &HashSet<i32>) -> i32 {
	let outermost = obj.marked_content_id().unwrap_or(-1);
	let count = obj.mark_count();
	if count <= 1 {
		return outermost;
	}
	let mut deepest = outermost;
	for index in 0..count {
		let Some(mark) = obj.mark(index) else { continue };
		if let Some(mcid) = mark.param_int(MCID_KEY)
			&& referenced.contains(&mcid)
		{
			deepest = mcid;
		}
	}
	deepest
}

fn is_artifact(obj: &PageObject) -> bool {
	(0..obj.mark_count()).filter_map(|index| obj.mark(index)).any(|mark| mark.name().as_deref() == Some("Artifact"))
}

pub(super) fn first_marked_content_id(elem: &Tag) -> Option<i32> {
	let count = elem.child_count();
	for i in 0..count {
		if let Some(child) = elem.child(i) {
			if let Some(mcid) = first_marked_content_id(&child) {
				return Some(mcid);
			}
		} else if let Some(mcid) = elem.child_mcid(i) {
			return Some(mcid);
		}
	}
	None
}

pub(super) fn collect_text(elem: &Tag, mcid_to_text: &HashMap<i32, String>, out: &mut String) {
	collect_unread_text(elem, &mut PageText::new(mcid_to_text), out);
}

fn collect_unread_text(elem: &Tag, page_text: &mut PageText, out: &mut String) {
	let count = elem.child_count();
	for i in 0..count {
		if let Some(child) = elem.child(i) {
			collect_unread_text(&child, page_text, out);
		} else if let Some(mcid) = elem.child_mcid(i)
			&& let Some(text) = page_text.take(mcid)
		{
			out.push_str(text);
		}
	}
}
