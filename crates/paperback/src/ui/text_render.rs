use paperback_core::{
	document::{DocLen, DocPos},
	session::{DocumentSession, WindowSlice},
};
use wxdragon::prelude::*;

#[cfg(target_os = "windows")]
use super::reader_ctrl::ctrl_len;
use super::{document_manager::DocumentTab, reader_ctrl::ReaderCtrl};
#[cfg(target_os = "windows")]
use super::{
	paragraph_format::apply_paragraph_formats,
	rtf_stream::{append_rtf_into_ctrl, stream_rtf_into_ctrl},
};
#[cfg(target_os = "windows")]
use crate::rtf::{RtfFontInfo, build_rtf, build_rtf_for_whole_control, sanitize_for_rich_edit};
use crate::{
	text_format::{FormatSegment, merge_formatting_markers},
	text_window::{self, CtrlPos, TextWindow},
};

pub(super) fn fill_text_ctrl(text_ctrl: ReaderCtrl, content: &str) {
	text_ctrl.set_value(content);
}

pub(super) fn load_window_into_ctrl(
	text_ctrl: ReaderCtrl,
	session: &DocumentSession,
	target_pos: DocPos,
	doc_len: DocLen,
) -> TextWindow {
	let slice = if text_window::should_use_whole_document(doc_len) {
		session.get_window(DocPos::ZERO, doc_len.as_pos())
	} else {
		let (raw_start, raw_end) = text_window::target_window_bounds(target_pos, doc_len);
		session.get_window(raw_start, raw_end)
	};
	let window = TextWindow::of(&slice);
	fill_text_ctrl_with_formatting(text_ctrl, &slice, &window);
	window
}

/// TODO(windowing): doesn't reapply readability spacing/alignment, since the navigation chokepoints calling this have no `ConfigManager`; fix by caching the last-applied values on `DocumentTab`.
pub fn reload_window_around(tab: &mut DocumentTab, doc_offset: DocPos, reason: &'static str) {
	let doc_len = tab.session.document_len();
	let (from_start, from_end) = (tab.window.start(), tab.window.end());
	tab.window = load_window_into_ctrl(tab.text_ctrl, &tab.session, doc_offset, doc_len);
	// Moving the loaded start is what can lose a screen reader's place mid Say-All, so every move is logged with the path that caused it.
	if from_start != tab.window.start() {
		tracing::info!(
			reason,
			caret = doc_offset.get(),
			from_start = from_start.get(),
			from_end = from_end.get(),
			to_start = tab.window.start().get(),
			to_end = tab.window.end().get(),
			doc_len = doc_len.get(),
			"loaded window start moved"
		);
	}
}

/// Returns false if the append did not complete, so the caller rebuilds rather than trusting a control left part-way through a chunk.
pub(super) fn append_slice_to_ctrl(text_ctrl: ReaderCtrl, slice: &WindowSlice, window: &TextWindow) -> bool {
	let content = slice.text.as_str();
	let segments = merge_formatting_markers(&slice.markers);
	#[cfg(target_os = "windows")]
	if !segments.is_empty()
		&& let Some(font) = text_ctrl.get_font()
	{
		let expected = sanitize_for_rich_edit(content);
		let rtf = build_rtf(
			&expected,
			&segments,
			&slice.paragraphs,
			&RtfFontInfo { face_name: font.get_face_name(), point_size: font.get_point_size() },
		);
		if append_rtf_into_ctrl(text_ctrl.widget(), &rtf) {
			return true;
		}
		tracing::warn!("RTF append did not complete");
		return false;
	}
	let base = text_ctrl.get_last_position();
	text_ctrl.append_text(content);
	if text_ctrl.get_last_position() <= base {
		return false;
	}
	let place = |offset: DocPos| window.to_local(slice.start + offset.from_start());
	#[cfg(target_os = "windows")]
	apply_paragraph_formats(text_ctrl, place, &slice.paragraphs);
	apply_formatting_markers_to_ctrl_from_segments(text_ctrl, &segments, place);
	true
}

/// Streaming RTF costs about 3.5s however little formatting it carries, against about 2s for plain text plus 10ms per styled span, so it only pays past a hundred spans or so (#920).
#[cfg(target_os = "windows")]
const RTF_STREAM_MIN_SEGMENTS: usize = 128;

pub(super) fn fill_text_ctrl_with_formatting(text_ctrl: ReaderCtrl, slice: &WindowSlice, window: &TextWindow) {
	let content = slice.text.as_str();
	let segments = merge_formatting_markers(&slice.markers);
	#[cfg(target_os = "windows")]
	if segments.len() >= RTF_STREAM_MIN_SEGMENTS
		&& let Some(font) = text_ctrl.get_font()
	{
		// RichEdit does not always store exactly what it is handed, so everything below compares against this rather than `content`.
		let expected = sanitize_for_rich_edit(content);
		let rtf = build_rtf_for_whole_control(
			&expected,
			&segments,
			&slice.paragraphs,
			&RtfFontInfo { face_name: font.get_face_name(), point_size: font.get_point_size() },
		);
		if stream_rtf_into_ctrl(text_ctrl.widget(), &rtf) {
			if text_ctrl.get_value() == *expected {
				return;
			}
			// RichEdit makes same-width substitutions of its own (U+2028 to a vertical tab, U+FDD0..=U+FDEF to spaces) that are only cosmetic; a length change would break every offset, and unparsed RTF stored as text is always far longer.
			let expected_len = ctrl_len(&expected);
			let stored_len = text_ctrl.get_last_position();
			if stored_len == expected_len {
				tracing::debug!(
					expected_len = expected_len.get(),
					"RTF round-trip was substituted but not resized; keeping it"
				);
				return;
			}
			tracing::warn!(
				stored_len = stored_len.get(),
				expected_len = expected_len.get(),
				"RTF fast path changed the content's length; falling back"
			);
		} else {
			tracing::warn!("RTF stream-in did not complete; falling back");
		}
	}
	fill_text_ctrl(text_ctrl, content);
	let place = |offset: DocPos| window.to_local(slice.start + offset.from_start());
	#[cfg(target_os = "windows")]
	apply_paragraph_formats(text_ctrl, place, &slice.paragraphs);
	apply_formatting_markers_to_ctrl_from_segments(text_ctrl, &segments, place);
}

fn apply_formatting_markers_to_ctrl_from_segments(
	text_ctrl: ReaderCtrl,
	segments: &[FormatSegment],
	place: impl Fn(DocPos) -> CtrlPos,
) {
	if segments.is_empty() {
		return;
	}
	let base_font = text_ctrl.get_font();
	text_ctrl.freeze();
	for seg in segments {
		let mut attr = wxdragon::widgets::textctrl::TextAttr::new();
		if let Some(base) = &base_font {
			let style = if seg.italic { FontStyle::Italic } else { base.get_style() };
			let weight = if seg.bold { FontWeight::Bold } else { base.get_weight() };
			let underlined = seg.underline || base.is_underlined();
			if let Some(mut font) = Font::new_with_details(
				base.get_point_size(),
				base.get_family().as_i32(),
				style.as_i32(),
				weight.as_i32(),
				underlined,
				&base.get_face_name(),
			) {
				if base.is_strikethrough() {
					font.set_strikethrough(true);
				}
				let encoding = base.get_encoding();
				if encoding != 0 {
					font.set_encoding(encoding);
				}
				attr.set_font(&font);
			}
		} else {
			if seg.bold {
				attr.set_font_weight(FontWeight::Bold);
			}
			if seg.italic {
				attr.set_font_style(FontStyle::Italic);
			}
			if seg.underline {
				attr.set_font_underlined(true);
			}
		}
		text_ctrl.set_style(place(seg.start), place(seg.end), &attr);
	}
	text_ctrl.thaw();
}
