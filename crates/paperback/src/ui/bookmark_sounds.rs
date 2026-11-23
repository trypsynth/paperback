//! Which bookmark sound a caret move plays, by the reader's choice in Settings.

use paperback_core::{
	config::{Bookmark, ConfigManager},
	document::{DocLen, DocPos},
};

const MODE_KEY: &str = "bookmark_sound_mode";
/// The checkbox the mode replaced.
const OLD_KEY: &str = "bookmark_sounds";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BookmarkSoundMode {
	Never,
	/// The caret enters a bookmark.
	OnBookmark,
	/// The word, line or paragraph moved to holds a bookmark.
	InUnit,
}

impl BookmarkSoundMode {
	/// The mode a stored value names.
	pub const fn from_config(mode: i32) -> Option<Self> {
		match mode {
			0 => Some(Self::Never),
			1 => Some(Self::OnBookmark),
			2 => Some(Self::InUnit),
			_ => None,
		}
	}

	/// The mode the old checkbox stands for.
	pub const fn from_old_checkbox(on: bool) -> Self {
		if on { Self::OnBookmark } else { Self::Never }
	}

	pub const fn to_config(self) -> i32 {
		match self {
			Self::Never => 0,
			Self::OnBookmark => 1,
			Self::InUnit => 2,
		}
	}

	pub fn load(config: &ConfigManager) -> Self {
		Self::from_config(config.get_app_int(MODE_KEY, -1))
			.unwrap_or_else(|| Self::from_old_checkbox(config.get_app_bool(OLD_KEY, true)))
	}

	pub fn save(self, config: &ConfigManager) {
		config.set_app_int(MODE_KEY, self.to_config());
	}
}

/// What a key moves the caret by.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MoveUnit {
	Character,
	Word,
	Line,
	Paragraph,
}

/// Ordered so a note's sound wins.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum BookmarkSound {
	Bookmark,
	Note,
}

impl BookmarkSound {
	pub fn play(self) {
		super::sounds::play_bookmark_sound(self == Self::Note);
	}
}

/// A stretch of the document moved to, such as a word or a line. An empty one stands for its position.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Unit {
	pub start: DocPos,
	pub end: DocPos,
}

impl Unit {
	fn holds(self, bookmark: &Bookmark) -> bool {
		bookmark.start < end_or_next(self.start, self.end) && end_or_next(bookmark.start, bookmark.end) > self.start
	}
}

/// The end of `start..end`, where an empty range covers its one position.
fn end_or_next(start: DocPos, end: DocPos) -> DocPos {
	if start == end { end + DocLen::from_raw(1) } else { end }
}

/// The sound for a move from `before` to `after`, and the unit to remember for the next move. With `unit`, the move sounds the bookmarks it holds and `previous` didn't. Without one, it sounds the bookmarks the caret entered.
pub fn sound_for_move(
	bookmarks: &[Bookmark],
	previous: Option<Unit>,
	before: DocPos,
	after: DocPos,
	unit: Option<Unit>,
) -> (Option<BookmarkSound>, Unit) {
	let sound = match unit {
		Some(unit) => new_in_unit(bookmarks, previous, unit),
		None => entered_bookmark(bookmarks, before, after),
	};
	(sound, remembered_unit(previous, unit, after))
}

/// The unit a move leaves remembered: its own, or else the remembered one while the caret is still on it, or else the caret's position.
pub fn remembered_unit(previous: Option<Unit>, unit: Option<Unit>, caret: DocPos) -> Unit {
	unit.or_else(|| previous.filter(|previous| previous.start <= caret && caret <= previous.end))
		.unwrap_or(Unit { start: caret, end: caret })
}

/// The sound for the bookmarks the caret entered between `before` and `after`. A note's sound wins.
fn entered_bookmark(bookmarks: &[Bookmark], before: DocPos, after: DocPos) -> Option<BookmarkSound> {
	let inside = |bookmark: &Bookmark, position: DocPos| {
		position >= bookmark.start && position < end_or_next(bookmark.start, bookmark.end)
	};
	sound_of(bookmarks.iter().filter(|bookmark| inside(bookmark, after) && !inside(bookmark, before)))
}

/// The sound for the bookmarks `unit` holds and `previous` didn't. A note's sound wins.
fn new_in_unit(bookmarks: &[Bookmark], previous: Option<Unit>, unit: Unit) -> Option<BookmarkSound> {
	sound_of(
		bookmarks
			.iter()
			.filter(|bookmark| unit.holds(bookmark) && !previous.is_some_and(|previous| previous.holds(bookmark))),
	)
}

fn sound_of<'a>(bookmarks: impl Iterator<Item = &'a Bookmark>) -> Option<BookmarkSound> {
	bookmarks.map(|bookmark| if bookmark.note.is_empty() { BookmarkSound::Bookmark } else { BookmarkSound::Note }).max()
}

#[cfg(test)]
mod tests {
	use paperback_core::{config::Bookmark, document::DocPos};

	use super::{
		BookmarkSound, BookmarkSoundMode, Unit, entered_bookmark, new_in_unit, remembered_unit, sound_for_move,
	};

	fn pos(raw: usize) -> DocPos {
		DocPos::from_raw(raw)
	}

	/// Lines of 40 positions each, the first being 0.
	fn line(index: usize) -> Unit {
		unit(index * 40, index * 40 + 40)
	}

	/// The sounds of a run of moves, each to a caret position, by its unit if it has one, starting at 0.
	fn sounds(bookmarks: &[Bookmark], moves: &[(usize, Option<Unit>)]) -> Vec<Option<BookmarkSound>> {
		let mut previous = None;
		let mut before = pos(0);
		moves
			.iter()
			.map(|&(caret, unit)| {
				let (sound, remembered) = sound_for_move(bookmarks, previous, before, pos(caret), unit);
				previous = Some(remembered);
				before = pos(caret);
				sound
			})
			.collect()
	}

	#[test]
	fn a_step_along_the_line_keeps_it_so_the_bookmarks_next_line_is_silent() {
		let heard = sounds(&[bookmark(20, 100)], &[(30, Some(line(0))), (31, None), (71, Some(line(1)))]);
		assert_eq!(heard, [Some(BookmarkSound::Bookmark), None, None]);
	}

	#[test]
	fn steps_before_a_bookmark_starting_mid_line_keep_its_line() {
		let heard = sounds(&[bookmark(20, 100)], &[(5, Some(line(0))), (6, None), (46, Some(line(1)))]);
		assert_eq!(heard, [Some(BookmarkSound::Bookmark), None, None]);
	}

	#[test]
	fn coming_back_after_stepping_off_the_line_sounds_again() {
		let heard = sounds(&[bookmark(20, 25)], &[(39, Some(line(0))), (41, None), (1, Some(line(0)))]);
		assert_eq!(heard, [Some(BookmarkSound::Bookmark), None, Some(BookmarkSound::Bookmark)]);
	}

	#[test]
	fn another_bookmark_on_the_next_line_still_sounds() {
		let heard = sounds(&[bookmark(20, 25), note(50, 55)], &[(5, Some(line(0))), (6, None), (46, Some(line(1)))]);
		assert_eq!(heard, [Some(BookmarkSound::Bookmark), None, Some(BookmarkSound::Note)]);
	}

	#[test]
	fn a_move_remembers_its_unit_or_else_the_one_the_caret_is_still_on() {
		assert_eq!(remembered_unit(Some(line(0)), Some(unit(5, 9)), pos(5)), unit(5, 9), "a unit of its own");
		assert_eq!(remembered_unit(Some(line(0)), None, pos(12)), line(0), "still on the line");
		assert_eq!(remembered_unit(Some(line(0)), None, pos(40)), line(0), "at the line's end");
		assert_eq!(remembered_unit(Some(line(0)), None, pos(41)), unit(41, 41), "off the line");
		assert_eq!(remembered_unit(None, None, pos(7)), unit(7, 7), "nothing remembered");
	}

	fn bookmark(start: usize, end: usize) -> Bookmark {
		Bookmark { start: pos(start), end: pos(end), ..Bookmark::default() }
	}

	fn note(start: usize, end: usize) -> Bookmark {
		Bookmark { note: "a note".to_string(), ..bookmark(start, end) }
	}

	fn unit(start: usize, end: usize) -> Unit {
		Unit { start: pos(start), end: pos(end) }
	}

	#[test]
	fn a_stored_value_names_a_mode() {
		assert_eq!(BookmarkSoundMode::from_config(0), Some(BookmarkSoundMode::Never));
		assert_eq!(BookmarkSoundMode::from_config(1), Some(BookmarkSoundMode::OnBookmark));
		assert_eq!(BookmarkSoundMode::from_config(2), Some(BookmarkSoundMode::InUnit));
		assert_eq!(BookmarkSoundMode::from_config(-1), None, "none stored");
		assert_eq!(BookmarkSoundMode::from_config(7), None, "an unknown value");
	}

	#[test]
	fn the_old_checkbox_stands_for_a_mode() {
		assert_eq!(BookmarkSoundMode::from_old_checkbox(true), BookmarkSoundMode::OnBookmark);
		assert_eq!(BookmarkSoundMode::from_old_checkbox(false), BookmarkSoundMode::Never);
	}

	#[test]
	fn a_mode_round_trips_through_its_stored_value() {
		for mode in [BookmarkSoundMode::Never, BookmarkSoundMode::OnBookmark, BookmarkSoundMode::InUnit] {
			assert_eq!(BookmarkSoundMode::from_config(mode.to_config()), Some(mode));
		}
	}

	#[test]
	fn entering_a_bookmark_plays_its_sound() {
		assert_eq!(entered_bookmark(&[bookmark(10, 20)], pos(5), pos(12)), Some(BookmarkSound::Bookmark));
		assert_eq!(entered_bookmark(&[note(10, 20)], pos(25), pos(19)), Some(BookmarkSound::Note));
	}

	#[test]
	fn staying_inside_or_outside_a_bookmark_is_silent() {
		assert_eq!(entered_bookmark(&[bookmark(10, 20)], pos(11), pos(15)), None, "inside");
		assert_eq!(entered_bookmark(&[bookmark(10, 20)], pos(2), pos(9)), None, "before it");
		assert_eq!(entered_bookmark(&[bookmark(10, 20)], pos(5), pos(20)), None, "its end is outside");
		assert_eq!(entered_bookmark(&[bookmark(10, 20)], pos(12), pos(12)), None, "no move");
	}

	#[test]
	fn a_zero_width_bookmark_sounds_only_on_its_position() {
		assert_eq!(entered_bookmark(&[bookmark(10, 10)], pos(9), pos(10)), Some(BookmarkSound::Bookmark));
		assert_eq!(entered_bookmark(&[bookmark(10, 10)], pos(9), pos(11)), None);
	}

	#[test]
	fn a_note_wins_over_a_bookmark_entered_together() {
		let bookmarks = [bookmark(10, 20), note(12, 14)];
		assert_eq!(entered_bookmark(&bookmarks, pos(5), pos(13)), Some(BookmarkSound::Note));
	}

	#[test]
	fn a_unit_holding_part_of_a_bookmark_plays_its_sound() {
		assert_eq!(
			new_in_unit(&[bookmark(8, 12)], None, unit(10, 20)),
			Some(BookmarkSound::Bookmark),
			"across the start"
		);
		assert_eq!(new_in_unit(&[note(15, 30)], None, unit(10, 20)), Some(BookmarkSound::Note), "across the end");
	}

	#[test]
	fn a_bookmark_ending_where_the_unit_starts_is_silent() {
		assert_eq!(new_in_unit(&[bookmark(5, 10)], None, unit(10, 20)), None);
		assert_eq!(new_in_unit(&[bookmark(20, 25)], None, unit(10, 20)), None, "one starting where it ends");
	}

	#[test]
	fn a_zero_width_bookmark_inside_the_unit_plays_its_sound() {
		assert_eq!(new_in_unit(&[bookmark(10, 10)], None, unit(10, 20)), Some(BookmarkSound::Bookmark));
		assert_eq!(new_in_unit(&[bookmark(20, 20)], None, unit(10, 20)), None, "on the unit's end");
	}

	#[test]
	fn an_empty_unit_holds_what_is_on_its_position() {
		assert_eq!(new_in_unit(&[bookmark(10, 10)], None, unit(10, 10)), Some(BookmarkSound::Bookmark));
		assert_eq!(new_in_unit(&[bookmark(8, 12)], None, unit(10, 10)), Some(BookmarkSound::Bookmark));
		assert_eq!(new_in_unit(&[bookmark(11, 12)], None, unit(10, 10)), None);
	}

	#[test]
	fn a_note_beside_a_bookmark_plays_the_note_sound() {
		let bookmarks = [bookmark(10, 12), note(15, 17)];
		assert_eq!(new_in_unit(&bookmarks, None, unit(10, 20)), Some(BookmarkSound::Note));
	}

	#[test]
	fn a_bookmark_the_previous_unit_held_is_silent() {
		let bookmarks = [bookmark(30, 35)];
		assert_eq!(new_in_unit(&bookmarks, Some(unit(10, 40)), unit(20, 40)), None);
		assert_eq!(
			new_in_unit(&bookmarks, Some(unit(0, 10)), unit(20, 40)),
			Some(BookmarkSound::Bookmark),
			"the previous unit didn't hold it"
		);
	}

	#[test]
	fn a_new_bookmark_sounds_beside_one_the_previous_unit_held() {
		let bookmarks = [bookmark(30, 35), note(38, 39)];
		assert_eq!(new_in_unit(&bookmarks, Some(unit(10, 36)), unit(20, 40)), Some(BookmarkSound::Note));
	}
}
