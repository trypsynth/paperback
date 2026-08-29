//! Back/forward reading-position history: [`record_history_position`] appends/dedupes/trims,
//! [`history_go_previous`]/[`history_go_next`] step through it.

fn normalize_index(positions: &[i64], index: usize) -> usize {
	if positions.is_empty() {
		return 0;
	}
	index.min(positions.len().saturating_sub(1))
}

fn trim_history(positions: &mut Vec<i64>, index: &mut usize, max_len: usize) {
	if max_len == 0 {
		return;
	}
	while positions.len() > max_len {
		positions.remove(0);
		if *index > 0 {
			*index -= 1;
		}
	}
}

pub fn record_history_position(positions: &mut Vec<i64>, index: &mut usize, current_pos: i64, max_len: usize) {
	if positions.is_empty() {
		positions.push(current_pos);
		*index = 0;
		trim_history(positions, index, max_len);
		return;
	}
	*index = normalize_index(positions, *index);
	if positions[*index] != current_pos {
		if *index + 1 < positions.len() {
			if positions[*index + 1] != current_pos {
				positions.truncate(*index + 1);
				positions.push(current_pos);
			}
		} else {
			positions.push(current_pos);
		}
		*index += 1;
	}
	trim_history(positions, index, max_len);
}

#[derive(Debug, Clone)]
pub struct HistoryNavResult {
	pub found: bool,
	pub target: i64,
	pub positions: Vec<i64>,
	pub index: usize,
}

#[must_use]
pub fn history_go_previous(
	history: &[i64],
	history_index: usize,
	current_pos: i64,
	max_len: usize,
) -> HistoryNavResult {
	if history.is_empty() {
		return HistoryNavResult { found: false, target: -1, positions: Vec::new(), index: 0 };
	}
	let mut positions = history.to_vec();
	let mut index = history_index;
	record_history_position(&mut positions, &mut index, current_pos, max_len);
	if index > 0 {
		index -= 1;
		let target = positions.get(index).copied().unwrap_or(-1);
		return HistoryNavResult { found: target >= 0, target, positions, index };
	}
	HistoryNavResult { found: false, target: -1, positions, index }
}

#[must_use]
pub fn history_go_next(history: &[i64], history_index: usize, current_pos: i64, max_len: usize) -> HistoryNavResult {
	if history.is_empty() {
		return HistoryNavResult { found: false, target: -1, positions: Vec::new(), index: 0 };
	}
	let mut positions = history.to_vec();
	let mut index = history_index;
	record_history_position(&mut positions, &mut index, current_pos, max_len);
	if index + 1 < positions.len() {
		index += 1;
		let target = positions.get(index).copied().unwrap_or(-1);
		return HistoryNavResult { found: target >= 0, target, positions, index };
	}
	HistoryNavResult { found: false, target: -1, positions, index }
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn record_history_position_appends_and_trims() {
		let mut positions = vec![1, 2, 3];
		let mut index = 2;
		record_history_position(&mut positions, &mut index, 4, 3);
		assert_eq!(positions, vec![2, 3, 4]);
		assert_eq!(index, 2);
	}

	#[test]
	fn record_history_position_truncates_forward_history() {
		let mut positions = vec![10, 20, 30];
		let mut index = 1;
		record_history_position(&mut positions, &mut index, 25, 10);
		assert_eq!(positions, vec![10, 20, 25]);
		assert_eq!(index, 2);
	}

	#[test]
	fn history_go_previous_and_next() {
		let history = vec![10, 20, 30];
		let prev = history_go_previous(&history, 2, 30, 10);
		assert!(prev.found);
		assert_eq!(prev.target, 20);
		assert_eq!(prev.index, 1);
		let next = history_go_next(&history, 0, 10, 10);
		assert!(next.found);
		assert_eq!(next.target, 20);
		assert_eq!(next.index, 1);
	}

	#[test]
	fn record_history_position_does_not_duplicate_current_position() {
		let mut positions = vec![10, 20, 30];
		let mut index = 2;
		record_history_position(&mut positions, &mut index, 30, 10);
		assert_eq!(positions, vec![10, 20, 30]);
		assert_eq!(index, 2);
	}

	#[test]
	fn history_go_previous_returns_not_found_for_empty_history() {
		let result = history_go_previous(&[], 0, 0, 10);
		assert!(!result.found);
		assert_eq!(result.target, -1);
		assert_eq!(result.positions, Vec::<i64>::new());
	}

	#[test]
	fn history_go_next_returns_not_found_at_end() {
		let history = vec![10, 20];
		let result = history_go_next(&history, 1, 20, 10);
		assert!(!result.found);
		assert_eq!(result.target, -1);
		assert_eq!(result.index, 1);
	}
}
