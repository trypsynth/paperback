//! Marks links with `RichEdit`'s link effect.

use paperback_core::{
	document::{DocPos, MarkerType},
	session::LineMarker,
};
use windows::{
	Win32::UI::Controls::RichEdit::{CFE_LINK, CFM_LINK, ITextDocument, ITextRange2},
	core::Interface,
};

use super::paragraph_format::{change_text_document, text_range};
use crate::{text_window::CtrlPos, ui::reader_ctrl::ReaderCtrl};

pub fn apply_link_marks(text_ctrl: ReaderCtrl, place: impl Fn(DocPos) -> CtrlPos, markers: &[LineMarker]) {
	let is_link = |marker: &&LineMarker| marker.mtype == MarkerType::Link;
	if !markers.iter().any(|marker| is_link(&marker)) {
		return;
	}
	change_text_document(text_ctrl, |document| {
		for link in markers.iter().filter(is_link) {
			if let Err(error) = mark(document, &place, link) {
				tracing::warn!(%error, "could not mark a link");
			}
		}
	});
}

fn mark(document: &ITextDocument, place: &impl Fn(DocPos) -> CtrlPos, link: &LineMarker) -> windows::core::Result<()> {
	let range: ITextRange2 = text_range(document, place, link.position, link.position + link.length)?.cast()?;
	unsafe { range.GetFont2()?.SetEffects(CFE_LINK.0.cast_signed(), CFM_LINK.0.cast_signed()) }
}
