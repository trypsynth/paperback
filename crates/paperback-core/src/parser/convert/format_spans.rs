//! Shared bold/italic/underline span tracking for [`super::xml_to_text::XmlToText`] and
//! [`super::html_to_text::HtmlToText`]: an opening `<b>`/`<i>`/`<u>` (or `<strong>`/`<em>`)
//! pushes the current position onto a per-kind stack, and the matching close pops it and
//! records a [`FormatInfo`] span - nested spans of the same kind are handled by the stack
//! rather than a single "currently open" position.

use crate::types::FormatInfo;

pub(super) enum FormatKind {
	Bold,
	Italic,
	Underline,
}

#[derive(Default)]
pub(super) struct FormatSpans {
	bolds: Vec<FormatInfo>,
	italics: Vec<FormatInfo>,
	underlines: Vec<FormatInfo>,
	open_bolds: Vec<usize>,
	open_italics: Vec<usize>,
	open_underlines: Vec<usize>,
}

impl FormatSpans {
	pub(super) fn clear(&mut self) {
		self.bolds.clear();
		self.italics.clear();
		self.underlines.clear();
		self.open_bolds.clear();
		self.open_italics.clear();
		self.open_underlines.clear();
	}

	pub(super) fn open(&mut self, kind: &FormatKind, position: usize) {
		match kind {
			FormatKind::Bold => self.open_bolds.push(position),
			FormatKind::Italic => self.open_italics.push(position),
			FormatKind::Underline => self.open_underlines.push(position),
		}
	}

	/// Pops the innermost open span of `kind` (if any) and records it as `[start, end_position)`.
	pub(super) fn close(&mut self, kind: &FormatKind, end_position: usize) {
		let (open_stack, spans) = match kind {
			FormatKind::Bold => (&mut self.open_bolds, &mut self.bolds),
			FormatKind::Italic => (&mut self.open_italics, &mut self.italics),
			FormatKind::Underline => (&mut self.open_underlines, &mut self.underlines),
		};
		if let Some(start) = open_stack.pop() {
			spans.push(FormatInfo { offset: start, length: end_position.saturating_sub(start) });
		}
	}

	pub(super) fn bolds(&self) -> &[FormatInfo] {
		&self.bolds
	}

	pub(super) fn italics(&self) -> &[FormatInfo] {
		&self.italics
	}

	pub(super) fn underlines(&self) -> &[FormatInfo] {
		&self.underlines
	}
}
