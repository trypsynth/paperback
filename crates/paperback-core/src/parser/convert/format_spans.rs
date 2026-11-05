//! A stack per kind rather than one open position, so nested spans of the same kind close correctly.

use crate::{
	document::{DocLen, DocPos},
	types::FormatInfo,
};

pub(super) enum FormatKind {
	Bold,
	Italic,
	Underline,
	Preformatted,
	Code,
	Superscript,
	Subscript,
}

#[derive(Default)]
pub(super) struct FormatSpans {
	bolds: Vec<FormatInfo>,
	italics: Vec<FormatInfo>,
	underlines: Vec<FormatInfo>,
	preformatted: Vec<FormatInfo>,
	code: Vec<FormatInfo>,
	superscripts: Vec<FormatInfo>,
	subscripts: Vec<FormatInfo>,
	open_bolds: Vec<DocPos>,
	open_italics: Vec<DocPos>,
	open_underlines: Vec<DocPos>,
	open_preformatted: Vec<DocPos>,
	open_code: Vec<DocPos>,
	open_superscripts: Vec<DocPos>,
	open_subscripts: Vec<DocPos>,
}

impl FormatSpans {
	pub(super) fn clear(&mut self) {
		self.bolds.clear();
		self.italics.clear();
		self.underlines.clear();
		self.preformatted.clear();
		self.code.clear();
		self.superscripts.clear();
		self.subscripts.clear();
		self.open_bolds.clear();
		self.open_italics.clear();
		self.open_underlines.clear();
		self.open_preformatted.clear();
		self.open_code.clear();
		self.open_superscripts.clear();
		self.open_subscripts.clear();
	}

	pub(super) fn open(&mut self, kind: &FormatKind, position: DocPos) {
		match kind {
			FormatKind::Bold => self.open_bolds.push(position),
			FormatKind::Italic => self.open_italics.push(position),
			FormatKind::Underline => self.open_underlines.push(position),
			FormatKind::Preformatted => self.open_preformatted.push(position),
			FormatKind::Code => self.open_code.push(position),
			FormatKind::Superscript => self.open_superscripts.push(position),
			FormatKind::Subscript => self.open_subscripts.push(position),
		}
	}

	pub(super) fn close(&mut self, kind: &FormatKind, end_position: DocPos) {
		let (open_stack, spans) = match kind {
			FormatKind::Bold => (&mut self.open_bolds, &mut self.bolds),
			FormatKind::Italic => (&mut self.open_italics, &mut self.italics),
			FormatKind::Underline => (&mut self.open_underlines, &mut self.underlines),
			FormatKind::Preformatted => (&mut self.open_preformatted, &mut self.preformatted),
			FormatKind::Code => (&mut self.open_code, &mut self.code),
			FormatKind::Superscript => (&mut self.open_superscripts, &mut self.superscripts),
			FormatKind::Subscript => (&mut self.open_subscripts, &mut self.subscripts),
		};
		if let Some(start) = open_stack.pop() {
			let length = end_position.since(start);
			// A block ends after its last newline, which a section's text drops when it finishes, so counting it would reach into the next section.
			let length =
				if matches!(kind, FormatKind::Preformatted) { length.saturating_sub(DocLen::ONE) } else { length };
			spans.push(FormatInfo { offset: start, length });
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

	pub(super) fn preformatted(&self) -> &[FormatInfo] {
		&self.preformatted
	}

	pub(super) fn code(&self) -> &[FormatInfo] {
		&self.code
	}

	pub(super) fn superscripts(&self) -> &[FormatInfo] {
		&self.superscripts
	}

	pub(super) fn subscripts(&self) -> &[FormatInfo] {
		&self.subscripts
	}
}
