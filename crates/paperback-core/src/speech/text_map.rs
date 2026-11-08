//! Where a word the engine speaks came from in the document, after pronunciation rules have changed the text.

/// One replacement, in the coordinates of the text it produced: `new_len` chars at `start` stand for `old_len` chars of the text before it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Edit {
	start: usize,
	new_len: usize,
	old_len: usize,
}

/// Maps a char offset in rewritten text back to the original, so a word the engine reports lands on the right place in the document.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TextMap {
	/// One list per rewrite, in the order they were made, each sorted by `start`.
	steps: Vec<Vec<Edit>>,
}

impl TextMap {
	/// Records one rewrite's replacements, given in the coordinates of the text before it, as `(start, old_len, new_len)` in order.
	pub(super) fn push_step(&mut self, replacements: &[(usize, usize, usize)]) {
		if replacements.is_empty() {
			return;
		}
		let mut shift: isize = 0;
		let edits = replacements
			.iter()
			.map(|&(start, old_len, new_len)| {
				let edit = Edit { start: start.saturating_add_signed(shift), new_len, old_len };
				shift += new_len.cast_signed() - old_len.cast_signed();
				edit
			})
			.collect();
		self.steps.push(edits);
	}

	/// The char offset in the original text that `offset` in the rewritten text came from. An offset inside a replacement maps to where the replaced text started.
	#[must_use]
	pub fn to_original(&self, offset: usize) -> usize {
		self.steps.iter().rev().fold(offset, |offset, edits| {
			let mut shift: isize = 0;
			for edit in edits {
				if offset < edit.start {
					break;
				}
				if offset < edit.start + edit.new_len {
					return edit.start.saturating_add_signed(-shift);
				}
				shift += edit.new_len.cast_signed() - edit.old_len.cast_signed();
			}
			offset.saturating_add_signed(-shift)
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn text_outside_a_replacement_keeps_its_place() {
		// "Dr Smith" became "Doctor Smith": "Dr" (2 chars) at 0 became "Doctor" (6).
		let mut map = TextMap::default();
		map.push_step(&[(0, 2, 6)]);
		assert_eq!(map.to_original(7), 3);
		assert_eq!(map.to_original(0), 0);
		assert_eq!(map.to_original(4), 0);
	}

	#[test]
	fn several_replacements_and_rewrites_map_back_in_turn() {
		// "a b a" with each "a" (1 char) replaced by "xyz" (3): "xyz b xyz".
		let mut map = TextMap::default();
		map.push_step(&[(0, 1, 3), (4, 1, 3)]);
		assert_eq!(map.to_original(4), 2);
		assert_eq!(map.to_original(6), 4);
		// Then "b" (now at 4) removed entirely: "xyz  xyz".
		map.push_step(&[(4, 1, 0)]);
		assert_eq!(map.to_original(5), 4);
		assert_eq!(map.to_original(0), 0);
	}

	#[test]
	fn no_rewrites_is_the_identity() {
		assert_eq!(TextMap::default().to_original(42), 42);
	}
}
