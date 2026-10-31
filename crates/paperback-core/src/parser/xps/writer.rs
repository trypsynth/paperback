use super::{
	page::Page,
	structure::{Block, Fragment, FragmentKind},
};
use crate::{
	document::DocumentBuffer,
	util::text::{collapse_whitespace, trim_string},
};

/// Writes a page that declares its structure.
pub(super) struct PageWriter<'a> {
	buffer: &'a mut DocumentBuffer,
	page: &'a Page,
	/// A fragment need not name everything on its page, and text nothing names must still reach the reader.
	read: Vec<bool>,
}

impl<'a> PageWriter<'a> {
	pub fn new(buffer: &'a mut DocumentBuffer, page: &'a Page) -> Self {
		Self { buffer, page, read: vec![false; page.runs.len()] }
	}

	fn text_of(&mut self, names: &[String]) -> String {
		let mut text = String::new();
		for name in names {
			for index in self.page.groups.get(name).cloned().unwrap_or_default() {
				self.read[index] = true;
				text.push_str(&self.page.runs[index].text);
			}
		}
		trim_string(&collapse_whitespace(&text))
	}

	fn write_line(&mut self, text: &str) {
		if !text.is_empty() {
			self.buffer.append(text);
			self.buffer.append("\n");
		}
	}

	fn write_blocks(&mut self, blocks: &[Block]) {
		for block in blocks {
			match block {
				Block::Paragraph(names) => {
					let text = self.text_of(names);
					self.write_line(&text);
				}
			}
		}
	}

	fn write_unnamed_runs(&mut self) {
		for index in 0..self.read.len() {
			if !self.read[index] {
				self.write_line(&trim_string(&collapse_whitespace(&self.page.runs[index].text)));
			}
		}
	}

	pub fn write(mut self, fragments: &[Fragment]) {
		for kind in [FragmentKind::Header, FragmentKind::Content, FragmentKind::Footer] {
			for fragment in fragments.iter().filter(|fragment| fragment.kind == kind) {
				self.write_blocks(&fragment.blocks);
			}
		}
		self.write_unnamed_runs();
	}
}
