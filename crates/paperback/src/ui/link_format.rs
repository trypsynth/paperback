//! Marks links with `RichEdit`'s link effect.

use paperback_core::{
	document::{DocPos, MarkerType},
	session::LineMarker,
};
use windows::{
	Win32::UI::Controls::RichEdit::{CFE_LINK, CFM_LINK, ITextDocument, ITextRange2, tomResume, tomSuspend},
	core::Interface,
};
use wxdragon::prelude::*;

use super::paragraph_format::{change_text_document, text_document, text_range};
use crate::{text_window::CtrlPos, ui::reader_ctrl::ReaderCtrl};

/// Every format change in the reading control makes `RichEdit` update each UI Automation range a screen reader holds on it, each counted from the start of the text, so marking the thousands of links at the end of a big book in place took minutes (#1331). They're marked in a hidden copy instead, which no screen reader holds ranges on, and come back in one replace.
pub fn apply_link_marks(text_ctrl: ReaderCtrl, place: impl Fn(DocPos) -> CtrlPos, markers: &[LineMarker]) {
	let links: Vec<&LineMarker> = markers.iter().filter(|marker| marker.mtype == MarkerType::Link).collect();
	let (Some(start), Some(end)) = (
		links.iter().map(|link| place(link.position)).min(),
		links.iter().map(|link| place(link.position + link.length)).max(),
	) else {
		return;
	};
	if mark_in_copy(text_ctrl, &place, &links, start, end).is_none() {
		tracing::warn!("could not mark links in a copy, so marking them in place");
		change_text_document(text_ctrl, |document| mark_all(document, &place, &links));
	}
}

/// Copies `start..end` of the control into a hidden control, marks the links there, and puts the result back.
fn mark_in_copy(
	text_ctrl: ReaderCtrl,
	place: &impl Fn(DocPos) -> CtrlPos,
	links: &[&LineMarker],
	start: CtrlPos,
	end: CtrlPos,
) -> Option<()> {
	let parent = text_ctrl.widget().get_parent()?;
	let copy = TextCtrl::builder(&parent).with_style(TextCtrlStyle::MultiLine | TextCtrlStyle::Rich2).build();
	copy.show(false);
	let result = (|| {
		let source = text_document(text_ctrl.widget())?;
		let target = text_document(copy)?;
		let (start, end) = (to_i32(start), to_i32(end));
		unsafe { target.Range(0, i32::MAX).ok()?.SetFormattedText(&source.Range(start, end).ok()?).ok()? };
		let local = |position: DocPos| place(position) + i64::from(-start);
		mark_all(&target, &local, links);
		let marked = unsafe { target.Range(0, end - start).ok()? };
		let (selection_start, selection_end) = text_ctrl.get_selection();
		let mut copied = None;
		change_text_document(text_ctrl, |document| unsafe {
			// The replace would otherwise keep the whole window's old formatting for undo, in a control nobody can edit.
			let _ = document.Undo(tomSuspend.0);
			copied = document.Range(start, end).and_then(|range| range.SetFormattedText(&marked)).ok();
			let _ = document.Undo(tomResume.0);
		});
		text_ctrl.set_selection(selection_start, selection_end);
		copied
	})();
	copy.destroy();
	result
}

fn to_i32(position: CtrlPos) -> i32 {
	i32::try_from(position.get()).unwrap_or(i32::MAX)
}

fn mark_all(document: &ITextDocument, place: &impl Fn(DocPos) -> CtrlPos, links: &[&LineMarker]) {
	for link in links {
		if let Err(error) = mark(document, place, link) {
			tracing::warn!(%error, "could not mark a link");
		}
	}
}

fn mark(document: &ITextDocument, place: &impl Fn(DocPos) -> CtrlPos, link: &LineMarker) -> windows::core::Result<()> {
	let range: ITextRange2 = text_range(document, place, link.position, link.position + link.length)?.cast()?;
	unsafe { range.GetFont2()?.SetEffects(CFE_LINK.0.cast_signed(), CFM_LINK.0.cast_signed()) }
}
