//! Structure only an export can show, since reading gets the same text with or without it.

use crate::{
	document::{DocLen, DocPos},
	types::BlockInfo,
};

const WRAPPERS: &[&str] = &["blockquote", "dl", "dt", "dd", "figure", "figcaption", "aside"];

pub(crate) const LINE_BREAK: &str = "br";

#[derive(Default)]
pub(super) struct BlockSpans {
	spans: Vec<BlockInfo>,
	/// Recorded when they open, so an outer block sorts ahead of an inner one starting at the same position.
	open: Vec<(usize, &'static str)>,
}

impl BlockSpans {
	pub(super) fn clear(&mut self) {
		self.spans.clear();
		self.open.clear();
	}

	pub(super) fn open(&mut self, tag_name: &str, position: DocPos) {
		if let Some(&tag) = WRAPPERS.iter().find(|tag| tag.eq_ignore_ascii_case(tag_name)) {
			self.open.push((self.spans.len(), tag));
			self.spans.push(BlockInfo { offset: position, length: DocLen::ZERO, tag });
		}
	}

	pub(super) fn close(&mut self, tag_name: &str, end_position: DocPos) {
		let Some(depth) = self.open.iter().rposition(|(_, tag)| tag.eq_ignore_ascii_case(tag_name)) else { return };
		let (index, _) = self.open[depth];
		self.open.truncate(depth);
		let span = &mut self.spans[index];
		// A block ends after its last newline, which a section's text drops when it finishes, so counting it would reach into the next section.
		span.length = end_position.since(span.offset).saturating_sub(DocLen::ONE);
	}

	/// The position is where the next line starts, since the line's own end isn't known until its trailing whitespace has been trimmed.
	pub(super) fn line_break(&mut self, next_line: DocPos) {
		self.spans.push(BlockInfo { offset: next_line, length: DocLen::ZERO, tag: LINE_BREAK });
	}

	/// A break just before a block ends or begins separates nothing, and would join two paragraphs.
	pub(super) fn drop_break_at(&mut self, position: DocPos) {
		if self.spans.last().is_some_and(|span| span.tag == LINE_BREAK && span.offset == position) {
			self.spans.pop();
		}
	}

	pub(super) fn spans(&self) -> &[BlockInfo] {
		&self.spans
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn nested_blocks_keep_the_order_they_opened_in() {
		let mut blocks = BlockSpans::default();
		blocks.open("dl", 0.into());
		blocks.open("DT", 0.into());
		blocks.close("dt", 5.into());
		blocks.close("dl", 5.into());
		let tags: Vec<_> = blocks.spans().iter().map(|span| (span.tag, span.length.get())).collect();
		assert_eq!(tags, [("dl", 4), ("dt", 4)]);
	}

	#[test]
	fn other_tags_are_not_recorded() {
		let mut blocks = BlockSpans::default();
		blocks.open("div", 0.into());
		blocks.close("div", 5.into());
		assert!(blocks.spans().is_empty());
	}

	#[test]
	fn only_a_break_at_the_position_is_dropped() {
		let mut blocks = BlockSpans::default();
		blocks.line_break(4.into());
		blocks.drop_break_at(9.into());
		assert_eq!(blocks.spans().len(), 1);
		blocks.drop_break_at(4.into());
		assert!(blocks.spans().is_empty());
	}
}
