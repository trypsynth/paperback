use objc::{msg_send, runtime::Object, sel, sel_impl};
use wxdragon::prelude::*;

#[repr(C)]
#[derive(Default)]
struct NativeRange {
	location: usize,
	length: usize,
}

/// Cocoa's PositionToXY/XYToPosition count paragraphs rather than wrapped visual lines.
/// Ask the layout manager for adjacent line fragments, whose character offsets use the same
/// UTF-16 units as `wxTextCtrl`. Movement commands are ineffective on a read-only `NSTextView`.
// Match reader_input's retry protocol: unknown position vs. a known loaded-window boundary.
#[allow(clippy::option_option)]
pub(super) fn try_adjacent_line_start(text_ctrl: TextCtrl, going_down: bool) -> Option<Option<(i64, i64)>> {
	#[repr(C)]
	struct NativePoint {
		x: f64,
		y: f64,
	}
	#[repr(C)]
	struct NativeRect {
		origin: NativePoint,
		size: NativePoint,
	}
	let scroll_view = text_ctrl.get_handle().cast::<Object>();
	if scroll_view.is_null() {
		return None;
	}
	// wxWidgets owns both native views for the control's lifetime. This key handler runs on
	// the UI thread; ranges and rectangles below match AppKit's 64-bit C ABI.
	unsafe {
		let text_view: *mut Object = msg_send![scroll_view, documentView];
		if text_view.is_null() {
			return None;
		}
		let layout: *mut Object = msg_send![text_view, layoutManager];
		if layout.is_null() {
			return None;
		}
		let glyph_count: usize = msg_send![layout, numberOfGlyphs];
		if glyph_count == 0 {
			return Some(None);
		}
		let text_len = usize::try_from(text_ctrl.get_last_position()).ok()?;
		let current_pos = usize::try_from(text_ctrl.get_insertion_point().max(0)).ok()?.min(text_len);
		let string: *mut Object = msg_send![text_view, string];
		let last_char: u16 = msg_send![string, characterAtIndex: text_len - 1];
		let has_empty_last_line = matches!(last_char, 10 | 13 | 0x2028 | 0x2029);
		let line_range = |glyph| {
			let mut range = NativeRange::default();
			let _: NativeRect =
				msg_send![layout, lineFragmentRectForGlyphAtIndex: glyph effectiveRange: &raw mut range];
			range
		};
		let target_glyph = if current_pos == text_len && has_empty_last_line {
			if going_down {
				return Some(None);
			}
			glyph_count - 1
		} else {
			let glyph: usize = msg_send![layout, glyphIndexForCharacterAtIndex: current_pos.min(text_len - 1)];
			let mut current_line = line_range(glyph);
			let affinity: usize = msg_send![text_view, selectionAffinity];
			let line_start: usize = msg_send![layout, characterIndexForGlyphAtIndex: current_line.location];
			// At a soft wrap, the same offset can be the end of the preceding visual line.
			if affinity == 0 && current_pos > 0 && current_pos == line_start {
				let previous_char: u16 = msg_send![string, characterAtIndex: current_pos - 1];
				if !matches!(previous_char, 10 | 13 | 0x2028 | 0x2029) {
					let previous_glyph: usize = msg_send![layout, glyphIndexForCharacterAtIndex: current_pos - 1];
					current_line = line_range(previous_glyph);
				}
			}
			if going_down {
				let next = current_line.location + current_line.length;
				if next >= glyph_count {
					return Some(has_empty_last_line.then_some((i64::try_from(text_len).ok()?, 0)));
				}
				next
			} else if current_line.location > 0 {
				current_line.location - 1
			} else {
				return Some(None);
			}
		};
		let target_line = line_range(target_glyph);
		let new_pos: usize = msg_send![layout, characterIndexForGlyphAtIndex: target_line.location];
		Some(Some((i64::try_from(new_pos).ok()?, 0)))
	}
}

/// Downstream affinity puts a wrap-boundary caret at the beginning of the next visual line.
pub(super) fn set_line_start(text_ctrl: TextCtrl, position: i64) {
	let range = NativeRange { location: usize::try_from(position).unwrap_or(0), length: 0 };
	// The caller has just navigated this live control on the UI thread.
	unsafe {
		let scroll_view = text_ctrl.get_handle().cast::<Object>();
		let text_view: *mut Object = msg_send![scroll_view, documentView];
		let _: () = msg_send![text_view, setSelectedRange: range affinity: 1_usize stillSelecting: false];
	}
	text_ctrl.show_position(position);
}
