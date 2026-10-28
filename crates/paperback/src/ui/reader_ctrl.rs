//! The reader's text control, which gives and takes positions only as [`CtrlPos`].
//!
//! A position in the control is counted from the start of what is loaded into it, and in the control's own units, so it means nothing to the session until [`crate::text_window::TextWindow`] turns it into a document position. Wrapping the control is what makes the compiler find every place a position crosses between the two (#1062): the control's own position methods are shadowed here by typed ones of the same names, and everything else reaches the control unchanged.

use std::ops::Deref;

use wxdragon::{prelude::*, widgets::textctrl::TextAttr};

use crate::text_window::CtrlPos;

/// How far `text` reaches in the control, which counts in UTF-16 code units.
#[cfg(target_os = "windows")]
pub fn ctrl_len(text: &str) -> CtrlPos {
	CtrlPos::from_raw(i64::try_from(text.encode_utf16().count()).unwrap_or(i64::MAX))
}

#[derive(Clone, Copy)]
pub struct ReaderCtrl(TextCtrl);

impl ReaderCtrl {
	pub const fn new(ctrl: TextCtrl) -> Self {
		Self(ctrl)
	}

	/// The control itself, for what takes it whole: sizers, native handles, the helpers that only set its look.
	pub const fn widget(self) -> TextCtrl {
		self.0
	}

	pub fn get_insertion_point(self) -> CtrlPos {
		CtrlPos::from_raw(self.0.get_insertion_point())
	}

	pub fn set_insertion_point(self, pos: CtrlPos) {
		self.0.set_insertion_point(pos.get());
	}

	pub fn show_position(self, pos: CtrlPos) {
		self.0.show_position(pos.get());
	}

	pub fn get_last_position(self) -> CtrlPos {
		CtrlPos::from_raw(self.0.get_last_position())
	}

	pub fn get_selection(self) -> (CtrlPos, CtrlPos) {
		let (from, to) = self.0.get_selection();
		(CtrlPos::from_raw(from), CtrlPos::from_raw(to))
	}

	pub fn set_selection(self, from: CtrlPos, to: CtrlPos) {
		self.0.set_selection(from.get(), to.get());
	}

	pub fn set_style(self, start: CtrlPos, end: CtrlPos, style: &TextAttr) {
		self.0.set_style(start.get(), end.get(), style);
	}

	#[cfg(target_os = "windows")]
	pub fn xy_to_position(self, x: i64, y: i64) -> CtrlPos {
		CtrlPos::from_raw(self.0.xy_to_position(x, y))
	}

	#[cfg(target_os = "windows")]
	pub fn position_to_xy(self, pos: CtrlPos) -> Option<(i64, i64)> {
		self.0.position_to_xy(pos.get())
	}
}

impl Deref for ReaderCtrl {
	type Target = TextCtrl;

	fn deref(&self) -> &TextCtrl {
		&self.0
	}
}
