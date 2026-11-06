use std::time::Instant;

use paperback_core::{
	document::DocPos,
	session::{DocumentSession, WindowSlice},
};

use super::{
	document_manager::DocumentTab,
	readability::{ReadabilityStyle, build_font_from_readability},
	reader_ctrl::ReaderCtrl,
};
use crate::{
	text_format::merge_formatting_markers,
	text_window::{TextWindow, control_pos},
};

/// Loads the whole document into the control and returns the loaded range.
pub(super) fn load_document_into_ctrl(
	text_ctrl: ReaderCtrl,
	session: &DocumentSession,
	plain_text: bool,
) -> TextWindow {
	let doc_len = session.document_len();
	let slice = session.get_window(DocPos::ZERO, doc_len.as_pos());
	let window = TextWindow::of(&slice);
	fill_text_ctrl_with_formatting(text_ctrl, &slice, session, plain_text);
	window
}

/// Reloads the whole document after its text changed, keeping the caret where it was.
pub(super) fn reload_document(tab: &mut DocumentTab, doc_pos: DocPos, plain_text: bool) {
	tab.window = load_document_into_ctrl(tab.text_ctrl, &tab.session, plain_text);
	let local = tab.to_local(doc_pos);
	tab.text_ctrl.set_insertion_point(local);
	tab.text_ctrl.show_position(local);
}

pub(super) fn style_and_fill(
	text_ctrl: ReaderCtrl,
	slice: &WindowSlice,
	session: &DocumentSession,
	style: &ReadabilityStyle,
	plain_text: bool,
) {
	if let Some(font) = build_font_from_readability(&style.rf) {
		text_ctrl.set_font(&font);
	}
	text_ctrl.apply_foreground_color(style.rf.color);
	text_ctrl.apply_background_color(style.bg_color);
	fill_text_ctrl_with_formatting(text_ctrl, slice, session, plain_text);
	text_ctrl.apply_readability_format(style);
}

pub(super) fn fill_text_ctrl_with_formatting(
	text_ctrl: ReaderCtrl,
	slice: &WindowSlice,
	session: &DocumentSession,
	plain_text: bool,
) {
	let content = slice.text.as_str();
	let started = Instant::now();
	text_ctrl.set_value(content);
	let fill = started.elapsed();
	// Formatting is the style runs alone; the control-level font, colours and spacing apply either way.
	let style_started = Instant::now();
	let spans = if plain_text {
		0
	} else {
		let segments = merge_formatting_markers(&slice.markers);
		let place = |offset: DocPos| control_pos(session, slice.start + offset.from_start());
		let spans = segments.len();
		text_ctrl.apply_span_styles(&segments, place);
		spans
	};
	tracing::debug!(
		chars = slice.text.len(),
		spans,
		plain = plain_text,
		fill_ms = fill.as_millis(),
		style_ms = style_started.elapsed().as_millis(),
		"filled the reading control"
	);
}
