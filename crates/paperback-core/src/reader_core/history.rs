use crate::document::DocPos;

fn normalize_index(positions: &[DocPos], index: usize) -> usize {
	if positions.is_empty() {
		return 0;
	}
	index.min(positions.len().saturating_sub(1))
}

fn trim_history(positions: &mut Vec<DocPos>, index: &mut usize, max_len: usize) {
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

pub fn record_history_position(positions: &mut Vec<DocPos>, index: &mut usize, current_pos: DocPos, max_len: usize) {
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
	pub target: Option<DocPos>,
	pub positions: Vec<DocPos>,
	pub index: usize,
}

#[must_use]
pub fn history_go_previous(
	history: &[DocPos],
	history_index: usize,
	current_pos: DocPos,
	max_len: usize,
) -> HistoryNavResult {
	if history.is_empty() {
		return HistoryNavResult { target: None, positions: Vec::new(), index: 0 };
	}
	let mut positions = history.to_vec();
	let mut index = history_index;
	record_history_position(&mut positions, &mut index, current_pos, max_len);
	if index > 0 {
		index -= 1;
		return HistoryNavResult { target: positions.get(index).copied(), positions, index };
	}
	HistoryNavResult { target: None, positions, index }
}

#[must_use]
pub fn history_go_next(
	history: &[DocPos],
	history_index: usize,
	current_pos: DocPos,
	max_len: usize,
) -> HistoryNavResult {
	if history.is_empty() {
		return HistoryNavResult { target: None, positions: Vec::new(), index: 0 };
	}
	let mut positions = history.to_vec();
	let mut index = history_index;
	record_history_position(&mut positions, &mut index, current_pos, max_len);
	if index + 1 < positions.len() {
		index += 1;
		return HistoryNavResult { target: positions.get(index).copied(), positions, index };
	}
	HistoryNavResult { target: None, positions, index }
}

#[cfg(test)]
mod tests {
	use super::*;

	fn at(raw: &[usize]) -> Vec<DocPos> {
		raw.iter().map(|&p| p.into()).collect()
	}

	#[test]
	fn record_history_position_appends_and_trims() {
		let mut positions = at(&[1, 2, 3]);
		let mut index = 2;
		record_history_position(&mut positions, &mut index, 4.into(), 3);
		assert_eq!(positions, at(&[2, 3, 4]));
		assert_eq!(index, 2);
	}

	#[test]
	fn record_history_position_truncates_forward_history() {
		let mut positions = at(&[10, 20, 30]);
		let mut index = 1;
		record_history_position(&mut positions, &mut index, 25.into(), 10);
		assert_eq!(positions, at(&[10, 20, 25]));
		assert_eq!(index, 2);
	}

	#[test]
	fn history_go_previous_and_next() {
		let history = at(&[10, 20, 30]);
		let prev = history_go_previous(&history, 2, 30.into(), 10);
		assert_eq!(prev.target, Some(20.into()));
		assert_eq!(prev.index, 1);
		let next = history_go_next(&history, 0, 10.into(), 10);
		assert_eq!(next.target, Some(20.into()));
		assert_eq!(next.index, 1);
	}

	#[test]
	fn record_history_position_does_not_duplicate_current_position() {
		let mut positions = at(&[10, 20, 30]);
		let mut index = 2;
		record_history_position(&mut positions, &mut index, 30.into(), 10);
		assert_eq!(positions, at(&[10, 20, 30]));
		assert_eq!(index, 2);
	}

	#[test]
	fn history_go_previous_returns_not_found_for_empty_history() {
		let result = history_go_previous(&[], 0, 0.into(), 10);
		assert_eq!(result.target, None);
		assert_eq!(result.positions, Vec::<DocPos>::new());
	}

	#[test]
	fn history_go_next_returns_not_found_at_end() {
		let history = at(&[10, 20]);
		let result = history_go_next(&history, 1, 20.into(), 10);
		assert_eq!(result.target, None);
		assert_eq!(result.index, 1);
	}
}
