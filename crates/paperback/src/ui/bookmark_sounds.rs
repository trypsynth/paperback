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

	/// Off Windows, the unit setting reads as the caret setting.
	pub fn load(config: &ConfigManager) -> Self {
		let mode = Self::from_config(config.get_app_int(MODE_KEY, -1))
			.unwrap_or_else(|| Self::from_old_checkbox(config.get_app_bool(OLD_KEY, true)));
		if cfg!(target_os = "windows") || mode != Self::InUnit { mode } else { Self::OnBookmark }
	}

	pub fn save(self, config: &ConfigManager) {
		config.set_app_int(MODE_KEY, self.to_config());
		config.remove_app_key(OLD_KEY);
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

/// A note sorts after a bookmark.
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

	fn covers(self, position: DocPos) -> bool {
		self.start <= position && position < end_or_next(self.start, self.end)
	}
}

/// The end of `start..end`, where an empty range covers its one position.
fn end_or_next(start: DocPos, end: DocPos) -> DocPos {
	if start == end { end + DocLen::from_raw(1) } else { end }
}

/// The sound for a move from `before` to `after`, and the unit to remember for the next move. The move sounds the bookmarks the caret entered. With `unit`, it also sounds the bookmarks `unit` holds and `previous` didn't. Bookmarks with a playback time stay silent.
pub fn sound_for_move(
	bookmarks: &[Bookmark],
	previous: Option<Unit>,
	before: DocPos,
	after: DocPos,
	unit: Option<Unit>,
) -> (Option<BookmarkSound>, Unit) {
	let text_bookmarks = || bookmarks.iter().filter(|bookmark| bookmark.audio_ms.is_none());
	let entered = entered_bookmark(text_bookmarks(), before, after);
	let sound = unit.map_or(entered, |unit| new_in_unit(text_bookmarks(), previous, unit).max(entered));
	(sound, remembered_unit(previous, unit, after))
}

/// A rule giving a move's sound and the unit to remember, as [`sound_for_move`] does.
pub type SoundRule = fn(&[Bookmark], Option<Unit>, DocPos, DocPos, Option<Unit>) -> (Option<BookmarkSound>, Unit);

/// The sound for a caret move another process made, such as a screen reader, and the unit to remember, as [`sound_for_move`] gives them. With `line`, the visual line the caret landed on, a move back measures that line. A move forward measures from just past `before` to the line's end, so it also sounds the bookmarks it passed that `previous` didn't hold. Either remembers the part of `line` it measured, joined with `previous` when that covers `before`.
#[cfg(any(target_os = "windows", test))]
pub fn sound_for_external_move(
	bookmarks: &[Bookmark],
	previous: Option<Unit>,
	before: DocPos,
	after: DocPos,
	line: Option<Unit>,
) -> (Option<BookmarkSound>, Unit) {
	let Some(line) = line else {
		return sound_for_move(bookmarks, previous, before, after, None);
	};
	let start = if before < after { before + DocLen::from_raw(1) } else { line.start };
	let (sound, _) = sound_for_move(bookmarks, previous, before, after, Some(Unit { start, ..line }));
	let heard_from =
		previous.filter(|previous| previous.covers(before)).map_or(start, |previous| previous.start.min(start));
	(sound, Unit { start: heard_from.max(line.start), ..line })
}

/// The unit a move leaves remembered: its own, or else the remembered one while the caret is still on it, or else the caret's position.
pub fn remembered_unit(previous: Option<Unit>, unit: Option<Unit>, caret: DocPos) -> Unit {
	unit.or_else(|| previous.filter(|previous| previous.covers(caret))).unwrap_or(Unit { start: caret, end: caret })
}

/// The sound for the bookmarks the caret entered between `before` and `after`. A note's sound wins.
fn entered_bookmark<'a>(
	bookmarks: impl IntoIterator<Item = &'a Bookmark>,
	before: DocPos,
	after: DocPos,
) -> Option<BookmarkSound> {
	let inside = |bookmark: &Bookmark, position: DocPos| {
		position >= bookmark.start && position < end_or_next(bookmark.start, bookmark.end)
	};
	sound_of(bookmarks.into_iter().filter(|bookmark| inside(bookmark, after) && !inside(bookmark, before)))
}

/// The sound for the bookmarks `unit` holds and `previous` didn't. A note's sound wins.
fn new_in_unit<'a>(
	bookmarks: impl IntoIterator<Item = &'a Bookmark>,
	previous: Option<Unit>,
	unit: Unit,
) -> Option<BookmarkSound> {
	sound_of(
		bookmarks
			.into_iter()
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
		BookmarkSound, BookmarkSoundMode, SoundRule, Unit, entered_bookmark, new_in_unit, remembered_unit,
		sound_for_external_move, sound_for_move,
	};

	fn pos(raw: usize) -> DocPos {
		DocPos::from_raw(raw)
	}

	/// Lines of 40 positions each, the first being 0.
	fn line(index: usize) -> Unit {
		unit(index * 40, index * 40 + 40)
	}

	/// The sounds of a run of moves by `rule` from `start`, each to a caret position, by its unit if it has one.
	fn sounds_by(
		rule: SoundRule,
		bookmarks: &[Bookmark],
		start: usize,
		moves: &[(usize, Option<Unit>)],
	) -> Vec<Option<BookmarkSound>> {
		let mut previous = None;
		let mut before = pos(start);
		moves
			.iter()
			.map(|&(caret, unit)| {
				let (sound, remembered) = rule(bookmarks, previous, before, pos(caret), unit);
				previous = Some(remembered);
				before = pos(caret);
				sound
			})
			.collect()
	}

	/// The sounds of a run of moves, each to a caret position, by its unit if it has one, starting at 0.
	fn sounds(bookmarks: &[Bookmark], moves: &[(usize, Option<Unit>)]) -> Vec<Option<BookmarkSound>> {
		sounds_by(sound_for_move, bookmarks, 0, moves)
	}

	#[test]
	fn a_step_along_the_line_keeps_it_so_the_bookmarks_next_line_is_silent() {
		let heard = sounds(&[bookmark(20, 100)], &[(30, Some(line(0))), (31, None), (71, Some(line(1)))]);
		assert_eq!(heard, [Some(BookmarkSound::Bookmark), None, None]);
	}

	#[test]
	fn a_line_move_into_a_bookmark_its_line_announced_sounds_again() {
		let heard = sounds(&[bookmark(20, 100)], &[(5, Some(line(0))), (6, None), (46, Some(line(1)))]);
		assert_eq!(heard, [Some(BookmarkSound::Bookmark), None, Some(BookmarkSound::Bookmark)]);
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
		assert_eq!(remembered_unit(Some(line(0)), None, pos(39)), line(0), "on the line's break");
		assert_eq!(remembered_unit(Some(line(0)), None, pos(40)), unit(40, 40), "on the next line's start");
		assert_eq!(remembered_unit(Some(unit(7, 7)), None, pos(7)), unit(7, 7), "a position stays");
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

	#[test]
	fn a_bookmark_with_a_playback_time_is_silent_for_moves() {
		let timed = Bookmark { audio_ms: Some(60_000), ..bookmark(0, 0) };
		let timed = [timed];
		assert_eq!(sounds(&timed, &[(0, None)]), [None], "the caret landing on it");
		assert_eq!(sounds(&timed, &[(0, Some(line(0)))]), [None], "a line holding it");
	}

	#[test]
	fn a_step_into_a_bookmark_its_line_announced_sounds_again() {
		let heard = sounds(&[bookmark(20, 25)], &[(5, Some(line(0))), (20, None)]);
		assert_eq!(heard, [Some(BookmarkSound::Bookmark), Some(BookmarkSound::Bookmark)]);
	}

	#[test]
	fn a_word_move_into_a_bookmark_its_line_announced_sounds_again() {
		let heard = sounds(&[bookmark(20, 25)], &[(5, Some(line(0))), (20, Some(unit(20, 26)))]);
		assert_eq!(heard, [Some(BookmarkSound::Bookmark), Some(BookmarkSound::Bookmark)]);
	}

	#[test]
	fn a_word_move_onto_a_word_holding_a_bookmark_its_line_announced_is_silent() {
		let heard = sounds(&[bookmark(22, 22)], &[(5, Some(line(0))), (20, Some(unit(20, 26)))]);
		assert_eq!(heard, [Some(BookmarkSound::Bookmark), None]);
	}
	#[test]
	fn stepping_onto_the_next_lines_start_leaves_the_line() {
		let heard = sounds(&[bookmark(20, 25)], &[(39, Some(line(0))), (40, None), (1, Some(line(0)))]);
		assert_eq!(heard, [Some(BookmarkSound::Bookmark), None, Some(BookmarkSound::Bookmark)]);
	}

	#[test]
	fn a_step_into_a_bookmark_after_leaving_its_line_sounds() {
		let heard = sounds(&[bookmark(20, 25)], &[(5, Some(line(0))), (45, None), (20, None)]);
		assert_eq!(heard, [Some(BookmarkSound::Bookmark), None, Some(BookmarkSound::Bookmark)]);
	}

	/// The 40-position line `caret` is on.
	fn line_of(caret: usize) -> Unit {
		line(caret / 40)
	}

	/// The sounds of external caret moves from `start`, each onto its line.
	fn external_move_sounds(bookmarks: &[Bookmark], start: usize, carets: &[usize]) -> Vec<Option<BookmarkSound>> {
		let moves: Vec<_> = carets.iter().map(|&caret| (caret, Some(line_of(caret)))).collect();
		sounds_by(sound_for_external_move, bookmarks, start, &moves)
	}

	#[test]
	fn an_external_move_remembers_the_part_of_its_line_it_measured() {
		let remembered = |previous: Option<Unit>, before: usize, after: usize| {
			sound_for_external_move(&[], previous, pos(before), pos(after), Some(line_of(after))).1
		};
		assert_eq!(remembered(None, 10, 50), line(1), "forward from an earlier line");
		assert_eq!(remembered(None, 45, 60), unit(46, 80), "forward along the line");
		assert_eq!(remembered(None, 90, 50), line(1), "backward");
		assert_eq!(remembered(Some(line(1)), 50, 60), line(1), "forward along a line already heard");
		assert_eq!(remembered(Some(unit(45, 45)), 45, 60), unit(45, 80), "forward from a position");
		assert_eq!(remembered(Some(line(0)), 30, 50), line(1), "forward from a line heard before");
	}

	#[test]
	fn a_bookmark_an_external_move_sounded_stays_silent_after_it_moves_on_along_the_line() {
		let bookmarks = [bookmark(42, 42)];
		let (first, remembered) = sound_for_external_move(&bookmarks, None, pos(0), pos(50), Some(line_of(50)));
		let (_, remembered) =
			sound_for_external_move(&bookmarks, Some(remembered), pos(50), pos(60), Some(line_of(60)));
		let (back, _) = sound_for_external_move(&bookmarks, Some(remembered), pos(60), pos(45), Some(line_of(45)));
		assert_eq!((first, back), (Some(BookmarkSound::Bookmark), None));
	}

	#[test]
	fn an_external_move_into_a_bookmark_on_a_line_it_reached_sounds_again() {
		let heard = external_move_sounds(&[bookmark(55, 55)], 0, &[45, 55]);
		assert_eq!(heard, [Some(BookmarkSound::Bookmark), Some(BookmarkSound::Bookmark)]);
	}

	#[test]
	fn a_step_back_into_a_bookmark_an_external_move_passed_on_its_line_sounds() {
		let bookmarks = [bookmark(42, 42)];
		let (_, remembered) = sound_for_external_move(&bookmarks, None, pos(45), pos(60), Some(line_of(60)));
		let (sound, _) = sound_for_move(&bookmarks, Some(remembered), pos(60), pos(42), None);
		assert_eq!(sound, Some(BookmarkSound::Bookmark));
	}

	#[test]
	fn an_external_move_backward_sounds_a_bookmark_earlier_on_its_line() {
		assert_eq!(external_move_sounds(&[bookmark(45, 45)], 100, &[79]), [Some(BookmarkSound::Bookmark)]);
	}

	#[test]
	fn a_line_an_external_move_passed_sounds_when_moved_back_to() {
		let bookmarks = [bookmark(45, 45)];
		let (_, remembered) = sound_for_external_move(&bookmarks, None, pos(0), pos(90), Some(line_of(90)));
		let (sound, _) = sound_for_move(&bookmarks, Some(remembered), pos(90), pos(50), Some(line(1)));
		assert_eq!(sound, Some(BookmarkSound::Bookmark));
	}

	#[test]
	fn a_step_into_a_bookmark_an_external_move_passed_sounds() {
		let bookmarks = [bookmark(45, 50)];
		let (_, remembered) = sound_for_external_move(&bookmarks, None, pos(0), pos(90), Some(line_of(90)));
		let (sound, _) = sound_for_move(&bookmarks, Some(remembered), pos(90), pos(49), None);
		assert_eq!(sound, Some(BookmarkSound::Bookmark));
	}

	#[test]
	fn without_a_unit_an_external_move_sounds_only_a_bookmark_it_lands_in() {
		assert_eq!(sound_for_external_move(&[bookmark(45, 45)], None, pos(0), pos(90), None).0, None);
		assert_eq!(
			sound_for_external_move(&[bookmark(85, 95)], None, pos(0), pos(90), None).0,
			Some(BookmarkSound::Bookmark)
		);
	}

	#[test]
	fn an_external_move_sounds_the_bookmarks_it_skipped() {
		assert_eq!(external_move_sounds(&[bookmark(45, 45)], 0, &[90]), [Some(BookmarkSound::Bookmark)]);
		assert_eq!(external_move_sounds(&[note(45, 50)], 0, &[90]), [Some(BookmarkSound::Note)]);
	}

	#[test]
	fn an_external_move_sounds_a_bookmark_later_on_its_line() {
		assert_eq!(external_move_sounds(&[bookmark(70, 70)], 0, &[50]), [Some(BookmarkSound::Bookmark)]);
	}

	#[test]
	fn a_bookmark_heard_with_an_earlier_line_is_silent_when_passed() {
		let heard = external_move_sounds(&[bookmark(30, 30)], 0, &[10, 90]);
		assert_eq!(heard, [Some(BookmarkSound::Bookmark), None]);
	}

	#[test]
	fn a_bookmark_on_the_old_position_is_silent() {
		assert_eq!(external_move_sounds(&[bookmark(10, 10)], 10, &[90]), [None]);
	}

	#[test]
	fn an_external_move_backward_sounds_only_its_line() {
		assert_eq!(external_move_sounds(&[bookmark(50, 50)], 100, &[10]), [None]);
		assert_eq!(external_move_sounds(&[bookmark(20, 20)], 100, &[10]), [Some(BookmarkSound::Bookmark)]);
	}
}
