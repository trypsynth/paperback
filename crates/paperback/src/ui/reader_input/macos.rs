use objc2::rc::Retained;
use objc2_app_kit::{NSScrollView, NSSelectionAffinity, NSTextView};
use objc2_foundation::NSRange;
use wxdragon::prelude::*;

use crate::{text_window::CtrlPos, ui::reader_ctrl::ReaderCtrl};

/// The text view inside the control's scroll view, which wxWidgets owns for the control's lifetime.
fn text_view(text_ctrl: ReaderCtrl) -> Option<Retained<NSTextView>> {
	let scroll_view = text_ctrl.get_handle().cast::<NSScrollView>();
	if scroll_view.is_null() {
		return None;
	}
	// wxWidgets' text control is an NSScrollView around an NSTextView, and this runs on the UI thread.
	unsafe { &*scroll_view }.documentView()?.downcast::<NSTextView>().ok()
}

// Cocoa's `PositionToXY` counts paragraphs, not wrapped lines, and movement commands do nothing on a read-only `NSTextView`, so this asks the layout manager.
// `None`: position unknown; `Some(None)`: target is outside the loaded window, as in reader_input's retry.
#[allow(clippy::option_option)]
pub(super) fn try_adjacent_line_start(text_ctrl: ReaderCtrl, going_down: bool) -> Option<Option<(CtrlPos, i64)>> {
	let text_view = text_view(text_ctrl)?;
	// wxWidgets' text view lays out with TextKit 1, so asking for its layout manager doesn't switch it over.
	let layout = unsafe { text_view.layoutManager() }?;
	let glyph_count = layout.numberOfGlyphs();
	if glyph_count == 0 {
		return Some(None);
	}
	let text_len = usize::try_from(text_ctrl.get_last_position().get()).ok()?;
	let current_pos = usize::try_from(text_ctrl.get_insertion_point().get().max(0)).ok()?.min(text_len);
	let string = text_view.string();
	let last_char = string.characterAtIndex(text_len - 1);
	let has_empty_last_line = matches!(last_char, 10 | 13 | 0x2028 | 0x2029);
	let line_range = |glyph| {
		let mut range = NSRange::new(0, 0);
		// The glyph is below the layout manager's glyph count, and the range is ours to write.
		unsafe { layout.lineFragmentRectForGlyphAtIndex_effectiveRange(glyph, &raw mut range) };
		range
	};
	let target_glyph = if current_pos == text_len && has_empty_last_line {
		if going_down {
			return Some(None);
		}
		glyph_count - 1
	} else {
		let glyph = layout.glyphIndexForCharacterAtIndex(current_pos.min(text_len - 1));
		let mut current_line = line_range(glyph);
		let line_start = layout.characterIndexForGlyphAtIndex(current_line.location);
		// At a soft wrap, the same offset can be the end of the preceding visual line.
		if text_view.selectionAffinity() == NSSelectionAffinity::Upstream
			&& current_pos > 0
			&& current_pos == line_start
		{
			let previous_char = string.characterAtIndex(current_pos - 1);
			if !matches!(previous_char, 10 | 13 | 0x2028 | 0x2029) {
				current_line = line_range(layout.glyphIndexForCharacterAtIndex(current_pos - 1));
			}
		}
		if going_down {
			let next = current_line.location + current_line.length;
			if next >= glyph_count {
				return Some(has_empty_last_line.then_some((CtrlPos::from_raw(i64::try_from(text_len).ok()?), 0)));
			}
			next
		} else if current_line.location > 0 {
			current_line.location - 1
		} else {
			return Some(None);
		}
	};
	let new_pos = layout.characterIndexForGlyphAtIndex(line_range(target_glyph).location);
	Some(Some((CtrlPos::from_raw(i64::try_from(new_pos).ok()?), 0)))
}

/// Downstream affinity puts a wrap-boundary caret at the beginning of the next visual line.
pub(super) fn set_line_start(text_ctrl: ReaderCtrl, position: CtrlPos) {
	if let Some(text_view) = text_view(text_ctrl) {
		let range = NSRange::new(usize::try_from(position.get()).unwrap_or(0), 0);
		text_view.setSelectedRange_affinity_stillSelecting(range, NSSelectionAffinity::Downstream, false);
	}
	text_ctrl.show_position(position);
}
