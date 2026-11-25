use super::*;

const DESKTOP: ReplicaOwner<'_> = ReplicaOwner { book: "book", device: "a", name: "Windows", title: "Book" };
const PHONE: ReplicaOwner<'_> = ReplicaOwner { book: "book", device: "b", name: "Android", title: "Book" };

fn reading(offset: usize, audio_ms: Option<u64>, bookmarks: Vec<StoredBookmark>) -> ReadingData {
	ReadingData {
		resume: ResumePosition { text: DocPos::from_raw(offset), audio_ms },
		bookmarks,
		temporary_bookmark: None,
	}
}

fn bookmark(offset: usize, note: &str) -> StoredBookmark {
	StoredBookmark { start: DocPos::from_raw(offset), end: DocPos::from_raw(offset), note: note.into(), audio_ms: None }
}

#[test]
fn independent_offline_positions_are_kept_until_the_reader_chooses() {
	let empty = ReadingData::default();
	let desktop_position = reading(50, Some(849_311), vec![]);
	let phone_position = reading(42, Some(802_001), vec![]);
	let mut desktop = BTreeMap::new();
	let mut phone = BTreeMap::new();
	assert!(record(&mut desktop, &DESKTOP, &empty, &desktop_position, false));
	assert!(record(&mut phone, &PHONE, &empty, &phone_position, false));
	desktop.extend(phone);
	let merged = merge(&desktop, desktop_position.resume);
	assert_eq!(merged.choices.len(), 2);
	assert_eq!(merged.data.resume, desktop_position.resume);
	assert!(record(&mut desktop, &DESKTOP, &desktop_position, &phone_position, true));
	let resolved = merge(&desktop, desktop_position.resume);
	assert!(resolved.choices.is_empty());
	assert_eq!(resolved.data.resume, phone_position.resume);
}

#[test]
fn moving_back_to_the_start_after_sync_is_a_new_position() {
	let empty = ReadingData::default();
	let late = reading(55, Some(94_350), vec![]);
	let zero = reading(0, Some(0), vec![]);
	let mut replicas = BTreeMap::new();
	record(&mut replicas, &DESKTOP, &empty, &late, false);
	record(&mut replicas, &PHONE, &late, &zero, false);
	let merged = merge(&replicas, late.resume);
	assert_eq!(merged.data.resume, zero.resume);
	assert!(merged.choices.is_empty());
}

#[test]
fn fetching_another_position_without_applying_it_does_not_resolve_the_conflict() {
	let empty = ReadingData::default();
	let desktop_start = reading(15, None, vec![]);
	let desktop_next = reading(21, None, vec![]);
	let phone_next = reading(38, None, vec![]);
	let mut replicas = BTreeMap::new();
	record(&mut replicas, &DESKTOP, &empty, &desktop_start, false);
	record(&mut replicas, &PHONE, &desktop_start, &phone_next, false);
	record(&mut replicas, &DESKTOP, &desktop_start, &desktop_next, false);
	let merged = merge(&replicas, desktop_next.resume);
	assert_eq!(merged.choices.len(), 2);
	assert_eq!(merged.data.resume, desktop_next.resume);
}

#[test]
fn independent_bookmarks_merge_and_a_deleted_bookmark_stays_deleted() {
	let empty = ReadingData::default();
	let alpha = reading(0, None, vec![bookmark(10, "One")]);
	let beta = reading(0, None, vec![bookmark(20, "Two")]);
	let mut replicas = BTreeMap::new();
	record(&mut replicas, &DESKTOP, &empty, &alpha, false);
	record(&mut replicas, &PHONE, &empty, &beta, false);
	let merged = merge(&replicas, empty.resume);
	assert_eq!(merged.data.bookmarks, vec![bookmark(10, "One"), bookmark(20, "Two")]);
	record(&mut replicas, &PHONE, &merged.data, &beta, false);
	assert_eq!(merge(&replicas, empty.resume).data.bookmarks, beta.bookmarks);
}
