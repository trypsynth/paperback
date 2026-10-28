use super::DocumentTab;
use crate::text_window::CtrlPos;

/// `None` if the position has no line/column; `Some(None)` if the target line is outside the loaded window, so the caller reloads and retries.
#[allow(
	clippy::option_option,
	reason = "the outer None and the inner None mean different things to reader_input's retry"
)]
pub(super) fn try_navigate_line_by_column(
	tab: &DocumentTab,
	going_down: bool,
	pref_col: Option<i64>,
	start_of_line: bool,
) -> Option<Option<(CtrlPos, i64)>> {
	let text_ctrl = tab.text_ctrl;
	let current_pos = text_ctrl.get_insertion_point().max(CtrlPos::ZERO);
	let (current_col, current_line) = text_ctrl.position_to_xy(current_pos)?;
	let col = pref_col.unwrap_or(current_col);
	let target_line = if going_down { current_line + 1 } else { current_line - 1 };
	if target_line < 0 {
		return Some(None);
	}
	let target_line_start = text_ctrl.xy_to_position(0, target_line);
	if target_line_start < CtrlPos::ZERO {
		return Some(None);
	}
	if start_of_line {
		return Some(Some((target_line_start, 0)));
	}
	let target_line_len = i64::from(text_ctrl.get_line_length(target_line));
	let new_pos = target_line_start + col.min(target_line_len);
	Some(Some((new_pos, col)))
}
