//! Gives text already in the native `RichEdit` control its heading and list paragraph formatting,
//! for the paths that load plain text rather than an RTF stream; `rtf::build_rtf` writes the same
//! formatting into its markup. A screen reader reading the control through the Text Object Model
//! reports these paragraphs as headings and list items.

use std::{ffi::c_void, ptr::null_mut};

use paperback_core::session::ParagraphFormat;
use windows::{
	Win32::{
		Foundation::{HWND, LPARAM, WPARAM},
		UI::{
			Controls::RichEdit::{EM_GETOLEINTERFACE, IRichEditOle, ITextDocument, tomListBullet, tomListNoNumber},
			WindowsAndMessaging::SendMessageW,
		},
	},
	core::Interface,
};
use wxdragon::prelude::*;

use crate::{text_window::CtrlPos, ui::reader_ctrl::ReaderCtrl};

/// Gives each of `paragraphs`, whose offsets `place` puts in the control, its heading style and list membership, leaving
/// the text looking exactly as it did.
///
/// Each paragraph's whole format is copied, changed and applied in one go: setting only the style
/// would also apply `RichEdit`'s own heading spacing. List items are bulleted with no bullet drawn,
/// since Paperback's bullets and numbers are already part of the text.
///
/// The Text Object Model refuses to change a read-only control, so the control is made editable
/// for the duration. That raises no accessibility event, and nothing here moves the caret.
pub fn apply_paragraph_formats(text_ctrl: ReaderCtrl, place: impl Fn(i64) -> CtrlPos, paragraphs: &[ParagraphFormat]) {
	if paragraphs.is_empty() {
		return;
	}
	let Some(document) = text_document(text_ctrl.widget()) else {
		return;
	};
	let editable = text_ctrl.is_editable();
	text_ctrl.freeze();
	text_ctrl.set_editable(true);
	for paragraph in paragraphs {
		if let Err(error) = apply(&document, &place, paragraph) {
			tracing::warn!(%error, "could not set a paragraph's heading or list formatting");
		}
	}
	text_ctrl.set_editable(editable);
	text_ctrl.thaw();
}

fn text_document(text_ctrl: TextCtrl) -> Option<ITextDocument> {
	let hwnd_ptr = text_ctrl.get_handle();
	if hwnd_ptr.is_null() {
		return None;
	}
	let mut ole: *mut c_void = null_mut();
	unsafe {
		SendMessageW(HWND(hwnd_ptr), EM_GETOLEINTERFACE, Some(WPARAM(0)), Some(LPARAM(&raw mut ole as isize)));
	}
	if ole.is_null() {
		return None;
	}
	unsafe { IRichEditOle::from_raw(ole) }.cast().ok()
}

fn apply(
	document: &ITextDocument,
	place: &impl Fn(i64) -> CtrlPos,
	paragraph: &ParagraphFormat,
) -> windows::core::Result<()> {
	let position = |offset: i64| i32::try_from(place(offset).get()).unwrap_or(i32::MAX);
	unsafe {
		let para = document.Range(position(paragraph.start), position(paragraph.end))?.GetPara()?;
		let format = para.GetDuplicate()?;
		if let Some(level) = paragraph.heading_level {
			format.SetStyle(-(i32::from(level) + 1))?;
		}
		if paragraph.list_item {
			format.SetListType(tomListBullet.0 | tomListNoNumber.0)?;
		}
		para.SetDuplicate(&format)
	}
}
