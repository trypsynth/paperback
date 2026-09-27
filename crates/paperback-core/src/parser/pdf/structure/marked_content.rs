//! Reading a page's marked content, which is what the structure tree is written against.
//!
//! A tagged page's content stream wraps each piece of text in a marked-content sequence with an
//! id, and the tree's leaves name those ids rather than carrying text of their own. So before the
//! tree can be walked, every glyph on the page has to be gathered under the id it belongs to.
//! [`PageMarkedContent`] is that gathering, and [`TreeFacts`] is the one pass over the tree it
//! needs first.

use std::collections::{HashMap, HashSet};

use super::super::text::{char_top, is_invisible_space, reorder_run};
use crate::pdfium::{PageObject, PdfTextPage, Tag, TagTree};

/// The mark parameter a tagged page stamps its glyphs with, and the one the tree points at.
const MCID_KEY: &str = "MCID";

/// Minimum fraction of visible text glyphs that must be associated with a marked-content id for
/// the tagged-extraction path to be trusted. Some PDFs advertise a structure tree while leaving
/// their text essentially untagged (no ids, or wrapped only in `/Artifact` marks); below this
/// threshold the structure tree is treated as unreliable and plain extraction is used instead.
pub(super) const MIN_MCID_COVERAGE: f64 = 0.5;

/// What one walk of the tree finds out before any text is read.
#[derive(Default)]
pub(super) struct TreeFacts {
	/// Every marked-content id the tree points at, so [`object_mcid`] knows which of an object's
	/// marks the tree is going to ask for.
	pub referenced: HashSet<i32>,
	/// Whether the tree names an image anywhere. A tree that names none leaves the page's images
	/// to be placed by the height they were drawn at; see
	/// [`crate::parser::pdf::images::UnclaimedImages`].
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

	fn gather(&mut self, elem: &Tag) {
		if elem.kind().unwrap_or_default() == "Figure" {
			self.claims_figures = true;
		}
		let count = elem.child_count();
		for i in 0..count {
			if let Some(child) = elem.child(i) {
				self.gather(&child);
			} else if let Some(mcid) = elem.child_mcid(i) {
				self.referenced.insert(mcid);
			}
		}
	}
}

/// One page's glyphs, gathered under the marked-content ids the tree names them by.
pub(super) struct PageMarkedContent {
	/// The text of each id, in reading order within the id.
	pub text: HashMap<i32, String>,
	/// The top edge of each id, which is where the text under it starts down the page. Only
	/// filled in when the caller asked for it; see [`read`].
	pub tops: HashMap<i32, f64>,
	/// The fraction of the page's visible glyphs that carry an id at all, against
	/// [`MIN_MCID_COVERAGE`].
	pub coverage: f64,
}

/// Walks the page's characters into [`PageMarkedContent`].
///
/// `want_tops` asks for each id's height as well, which costs one pdfium call per id. Only a page
/// placing images by height needs it, so a page whose tree names its figures does not pay for it.
pub(super) fn read(text_page: &PdfTextPage, facts: &TreeFacts, want_tops: bool) -> PageMarkedContent {
	let mut content = PageMarkedContent { text: HashMap::new(), tops: HashMap::new(), coverage: 1.0 };
	let mut real_char_count: usize = 0;
	let mut mcid_char_count: usize = 0;
	if let Some(char_count) = text_page.char_count() {
		let mut current_mcid = -1;
		// Chars of the current marked-content run with their pdfium index, so RTL
		// runs can be reordered visual→logical per run.
		let mut current_chars: Vec<(char, i32)> = Vec::new();
		for i in 0..char_count {
			let unicode = text_page.unicode_at(i);
			if let Some(ch) = char::from_u32(unicode) {
				if (ch.is_control() && !matches!(ch, '\n' | '\r' | '\t')) || ch == '\u{00AD}' {
					continue;
				}
				// A space the page never renders (see `is_invisible_space`) would land in the
				// middle of a word here just as it does in plain extraction.
				if ch == ' ' && is_invisible_space(text_page, i, char_count) {
					continue;
				}
				let is_generated = text_page.is_generated(i);
				let mut char_mcid = -1;
				if !is_generated && let Some(obj) = text_page.text_object(i) {
					char_mcid = object_mcid(&obj, &facts.referenced);
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
	content
}

/// The marked-content id a text object's glyphs belong to.
///
/// pdfium reports the outermost mark carrying an id, which is the whole answer for the single
/// mark a tagged PDF normally writes around a piece of text. A PDF exported from Apple Pages
/// nests them: the list's mark wraps the item's, which wraps the label's, so every glyph in the
/// list reports the list's id and the ids the structure tree actually points at are left holding
/// nothing. Taking the innermost mark the tree refers to puts the text where the tree looks for
/// it. An object with one mark, which is every object in a document written the usual way, skips
/// all of this and keeps the id pdfium gives.
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

/// The first marked-content id anywhere under an element, which is where its text starts on the
/// page.
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

/// All the text under an element, ids resolved and run together, for the places that want an
/// element's words rather than its layout: a heading's title, a list item's label, a table cell.
pub(super) fn collect_text(elem: &Tag, mcid_to_text: &HashMap<i32, String>, out: &mut String) {
	let count = elem.child_count();
	for i in 0..count {
		if let Some(child) = elem.child(i) {
			collect_text(&child, mcid_to_text, out);
		} else if let Some(mcid) = elem.child_mcid(i)
			&& let Some(text) = mcid_to_text.get(&mcid)
		{
			out.push_str(text);
		}
	}
}
