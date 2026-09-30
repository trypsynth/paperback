//! Putting a document's text into the reading control: loading the window around the caret,
//! extending it, and rendering the bold, italic and underline runs the parser found.

use paperback_core::session::{DocumentSession, WindowSlice};
use wxdragon::prelude::*;

use super::document_manager::DocumentTab;
#[cfg(target_os = "windows")]
use super::rtf_stream::{append_rtf_into_ctrl, stream_rtf_into_ctrl};
#[cfg(target_os = "windows")]
use crate::rtf::{RtfFontInfo, build_rtf, sanitize_for_rich_edit, stored_display_len};
use crate::{
	text_format::{FormatSegment, merge_formatting_markers},
	text_window::{self, TextWindow},
};

pub(super) fn fill_text_ctrl(text_ctrl: TextCtrl, content: &str) {
	text_ctrl.set_value(content);
}

/// Loads into `text_ctrl` whichever window of `session`'s content contains `target_pos`: the
/// whole document for anything under `text_window::should_use_whole_document`'s threshold
/// (identical to the app's pre-windowing behavior), otherwise a bounded window centered on
/// `target_pos`. Returns the `TextWindow` describing what's now actually loaded.
pub(super) fn load_window_into_ctrl(
	text_ctrl: TextCtrl,
	session: &DocumentSession,
	target_pos: i64,
	doc_len: i64,
) -> TextWindow {
	let slice = if text_window::should_use_whole_document(doc_len) {
		session.get_window(0, doc_len)
	} else {
		let (raw_start, raw_end) = text_window::target_window_bounds(target_pos, doc_len);
		session.get_window(raw_start, raw_end)
	};
	let window = TextWindow::new(slice.start, slice.end);
	fill_text_ctrl_with_formatting(text_ctrl, &slice);
	window
}

/// Reloads `tab`'s window to one centered on `doc_offset`. Call sites go through
/// `navigation::jump_to_doc_offset`, which checks `TextWindow::needs_reload_for` first, so this
/// always actually reloads when called.
///
/// TODO(windowing, phase 2): this doesn't reapply readability/font/color formatting the way a
/// full rebuild does (`apply_readability_format_to_ctrl` and friends), since those need
/// `ConfigManager` values that aren't available at the navigation chokepoints this is called
/// from without changing their signatures (defeating the point of routing ~90 call sites
/// through two unchanged chokepoints). Bold/italic/underline markers are unaffected (handled by
/// `load_window_into_ctrl`/`fill_text_ctrl_with_formatting` from the document's own markers,
/// not from readability settings) - this only means a reload can momentarily show default
/// line/paragraph/letter spacing and alignment until something else (a settings change, a
/// word-wrap toggle) reapplies them across all tabs. Fix by caching the last-applied readability
/// values on `DocumentTab` itself, updated wherever `apply_line_spacing`/`apply_paragraph_spacing`/
/// `apply_letter_spacing`/`apply_text_alignment` already loop over every tab.
///
/// `reason` names the call site in the log. Moving the loaded start is the one thing that can
/// pull the ground out from under a screen reader mid-read, so every one of these is recorded:
/// when someone reports a Say-All losing its place, the log says whether the window moved and
/// which path moved it, without anyone having to reproduce it first. Equally, a report with no
/// line here is evidence the window is not what moved, which is worth just as much.
pub fn reload_window_around(tab: &mut DocumentTab, doc_offset: i64, reason: &'static str) {
	let doc_len = tab.session.document_len();
	let before = tab.window;
	tab.window = load_window_into_ctrl(tab.text_ctrl, &tab.session, doc_offset, doc_len);
	if before.start() != tab.window.start() {
		tracing::info!(
			reason,
			caret = doc_offset,
			from_start = before.start(),
			from_end = before.end(),
			to_start = tab.window.start(),
			to_end = tab.window.end(),
			doc_len,
			"loaded window start moved"
		);
	}
}

/// Appends `slice` to whatever the control already holds, preserving existing offsets.
///
/// The counterpart to [`fill_text_ctrl_with_formatting`], which replaces everything. Returns
/// false if the append did not complete, so the caller can rebuild instead of trusting a control
/// that is now part-way through a chunk.
pub(super) fn append_slice_to_ctrl(text_ctrl: TextCtrl, slice: &WindowSlice) -> bool {
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
			&RtfFontInfo { face_name: font.get_face_name(), point_size: font.get_point_size() },
		);
		if append_rtf_into_ctrl(text_ctrl, &rtf) {
			return true;
		}
		tracing::warn!("RTF append did not complete");
		return false;
	}
	// Plain-text path: the same one `fill_text_ctrl_with_formatting` falls back to, with the
	// segments shifted past what is already loaded so they land on the text just appended.
	let base = text_ctrl.get_last_position();
	text_ctrl.append_text(content);
	if text_ctrl.get_last_position() <= base {
		return false;
	}
	let shifted: Vec<FormatSegment> =
		segments.iter().map(|seg| FormatSegment { start: seg.start + base, end: seg.end + base, ..*seg }).collect();
	apply_formatting_markers_to_ctrl_from_segments(text_ctrl, &shifted);
	true
}

/// Fills `text_ctrl` with `slice`'s text and bold/italic/underline markers. `slice` may be a
/// window into a much larger document (see `text_window`) rather than its full content;
/// this function has no notion of "the whole document" and just fills whatever it's handed.
///
/// On Windows this streams a single RTF blob into the native `RichEdit` control
/// via `EM_STREAMIN` (see `rtf_stream::stream_rtf_into_ctrl`) instead of issuing
/// one `SetStyle` call per formatting span, which is far cheaper on documents
/// with thousands of spans. `wxTextCtrl::SetValue` can't be used for this, since it
/// does not forward to the native `WM_SETTEXT` handler that auto-detects a
/// `{\rtf` prefix, so it would just store the markup as literal text. If
/// streaming doesn't round-trip back to the original content, this falls back
/// to the plain-text + per-segment path used on every other platform.
/// The number of formatted spans at which streaming the whole window as RTF starts to be worth
/// it, rather than setting the text plainly and styling each span afterwards.
///
/// Both paths end up in the same place, but they cost differently. Measured on a 443,000
/// character Chinese book: streaming costs around 3.5 seconds however little formatting the
/// markup carries, plain text around 2, and each span another 10 milliseconds to style. So the
/// RTF only pays for itself past a hundred spans or so. Below that it is pure loss, and a book
/// whose only formatting was one bold word waited seconds for a five megabyte RTF that existed
/// to embolden four characters (<https://github.com/trypsynth/paperback/issues/920>).
#[cfg(target_os = "windows")]
const RTF_STREAM_MIN_SEGMENTS: usize = 128;

pub(super) fn fill_text_ctrl_with_formatting(text_ctrl: TextCtrl, slice: &WindowSlice) {
	let content = slice.text.as_str();
	let segments = merge_formatting_markers(&slice.markers);
	#[cfg(target_os = "windows")]
	if segments.len() >= RTF_STREAM_MIN_SEGMENTS
		&& let Some(font) = text_ctrl.get_font()
	{
		// What RichEdit will actually end up holding, which is not always what it is handed -
		// see `sanitize_for_rich_edit`. Everything below compares against this rather
		// than against `content`.
		let expected = sanitize_for_rich_edit(content);
		let rtf = build_rtf(
			&expected,
			&segments,
			&RtfFontInfo { face_name: font.get_face_name(), point_size: font.get_point_size() },
		);
		if stream_rtf_into_ctrl(text_ctrl, &rtf) {
			let round_tripped = text_ctrl.get_value();
			// RichEdit's document model implicitly terminates the buffer, so a
			// wholly-trailing "\par" (with no content after it) doesn't manifest
			// as a stored character. Tolerate exactly that one known, harmless
			// discrepancy rather than falling back over it: the very last
			// position of *whatever we streamed in* ends up one short of `expected`,
			// which only matters at its literal last character. This applies the same
			// way to a windowed slice as to the whole document - RichEdit has no notion
			// of "there's more after this that isn't loaded"; from its perspective
			// `expected` (window or not) *is* the whole buffer it was asked to store.
			let matched = round_tripped == *expected
				|| (expected.ends_with('\n')
					&& round_tripped.len() + 1 == expected.len()
					&& expected.starts_with(round_tripped.as_str()));
			if matched {
				return;
			}
			// Not identical, but harmless as long as it cost no display units: every position
			// the app hands the control is an offset into this buffer, so a length change
			// breaks the caret, bookmarks and `text_window`'s translation alike, whereas a
			// same-width substitution is only cosmetic. RichEdit does make a few of those on
			// its own - U+2028 comes back as a vertical tab, U+FDD0..=U+FDEF as spaces - and
			// falling back over those would cost seconds per window load to fix nothing. A
			// length check is still decisive against the failure this guards: unparsed RTF
			// stored as literal text would be tens of thousands of display units longer than
			// the content it encodes.
			let expected_len = stored_display_len(&expected);
			let stored_len = text_ctrl.get_last_position();
			if stored_len == expected_len {
				tracing::debug!(expected_len, "RTF round-trip was substituted but not resized; keeping it");
				return;
			}
			tracing::warn!(stored_len, expected_len, "RTF fast path changed the content's length; falling back");
		} else {
			tracing::warn!("RTF stream-in did not complete; falling back");
		}
		// Never leave raw RTF markup on screen for an accessibility user;
		// fall back below to the plain-text + segment-loop path.
	}
	fill_text_ctrl(text_ctrl, content);
	apply_formatting_markers_to_ctrl_from_segments(text_ctrl, &segments);
}

fn apply_formatting_markers_to_ctrl_from_segments(text_ctrl: TextCtrl, segments: &[FormatSegment]) {
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
			// No base font to preserve; fall back to per-attribute flags.
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
		text_ctrl.set_style(seg.start, seg.end, &attr);
	}
	text_ctrl.thaw();
}
