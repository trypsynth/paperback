use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{config::StoredBookmark, document::DocPos};

pub(super) const PROTOCOL_VERSION: u8 = 1;
type Clock = BTreeMap<String, u64>;

#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResumePosition {
	pub text: DocPos,
	pub audio_ms: Option<u64>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReadingData {
	pub resume: ResumePosition,
	pub bookmarks: Vec<StoredBookmark>,
	pub temporary_bookmark: Option<DocPos>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct PositionChange {
	value: ResumePosition,
	clock: Clock,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Edit<T> {
	sequence: u64,
	value: T,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(super) struct Replica {
	pub version: u8,
	pub book: String,
	pub device: String,
	pub device_name: String,
	pub title: String,
	pub generation: u64,
	position: Option<PositionChange>,
	bookmarks: BTreeMap<String, Edit<Option<StoredBookmark>>>,
	temporary_bookmark: Option<Edit<Option<DocPos>>>,
}

#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResumeChoice {
	pub device_name: String,
	pub position: ResumePosition,
}

#[derive(Debug, Default)]
pub(super) struct MergedData {
	pub data: ReadingData,
	pub choices: Vec<ResumeChoice>,
}

pub(super) struct ReplicaOwner<'a> {
	pub book: &'a str,
	pub device: &'a str,
	pub name: &'a str,
	pub title: &'a str,
}

impl Replica {
	fn new(owner: &ReplicaOwner<'_>) -> Self {
		Self {
			version: PROTOCOL_VERSION,
			book: owner.book.into(),
			device: owner.device.into(),
			device_name: owner.name.into(),
			title: owner.title.into(),
			generation: 0,
			position: None,
			bookmarks: BTreeMap::new(),
			temporary_bookmark: None,
		}
	}
}

fn bookmark_key(bookmark: &StoredBookmark) -> String {
	bookmark
		.audio_ms
		.map_or_else(|| format!("text/{}/{}", bookmark.start.get(), bookmark.end.get()), |ms| format!("audio/{ms}"))
}

fn merge_clock(target: &mut Clock, source: &Clock) {
	for (device, &sequence) in source {
		let old = target.entry(device.clone()).or_default();
		*old = (*old).max(sequence);
	}
}

fn dominates(newer: &Clock, older: &Clock) -> bool {
	newer != older && older.iter().all(|(device, sequence)| newer.get(device).is_some_and(|n| n >= sequence))
}

pub(super) fn record(
	replicas: &mut BTreeMap<String, Replica>,
	owner: &ReplicaOwner<'_>,
	before: &ReadingData,
	current: &ReadingData,
	resolve_position: bool,
) -> bool {
	let device = owner.device;
	if !resolve_position && before == current && replicas.contains_key(device) {
		return false;
	}
	let generation = replicas.values().map(|replica| replica.generation).max().unwrap_or(0).saturating_add(1);
	let mut clock = Clock::new();
	for (source, replica) in replicas.iter() {
		if let Some(position) = replica.position.as_ref()
			&& (resolve_position || source == device || position.value == before.resume)
		{
			// A position can be fetched while its book is open, without the reader taking it up.
			merge_clock(&mut clock, &position.clock);
		}
	}
	clock.insert(device.into(), generation);
	let replica = replicas.entry(device.into()).or_insert_with(|| Replica::new(owner));
	replica.generation = generation;
	if resolve_position || before.resume != current.resume {
		replica.position = Some(PositionChange { value: current.resume, clock });
	}
	let before_marks: BTreeMap<_, _> =
		before.bookmarks.iter().map(|bookmark| (bookmark_key(bookmark), bookmark)).collect();
	let current_marks: BTreeMap<_, _> =
		current.bookmarks.iter().map(|bookmark| (bookmark_key(bookmark), bookmark)).collect();
	for (key, mark) in &current_marks {
		if before_marks.get(key) != Some(mark) {
			replica.bookmarks.insert(key.clone(), Edit { sequence: generation, value: Some((*mark).clone()) });
		}
	}
	for key in before_marks.into_keys().filter(|key| !current_marks.contains_key(key)) {
		replica.bookmarks.insert(key, Edit { sequence: generation, value: None });
	}
	if before.temporary_bookmark != current.temporary_bookmark {
		replica.temporary_bookmark = Some(Edit { sequence: generation, value: current.temporary_bookmark });
	}
	true
}

pub(super) fn merge(replicas: &BTreeMap<String, Replica>, local: ResumePosition) -> MergedData {
	let positions: Vec<_> = replicas
		.values()
		.filter_map(|replica| replica.position.as_ref().map(|position| (&replica.device_name, position)))
		.collect();
	let mut choices = Vec::new();
	for (device_name, position) in &positions {
		if positions.iter().any(|(_, other)| dominates(&other.clock, &position.clock)) {
			continue;
		}
		if !choices.iter().any(|choice: &ResumeChoice| choice.position == position.value) {
			choices.push(ResumeChoice { device_name: (*device_name).clone(), position: position.value });
		}
	}
	let resume = if let [only] = &choices[..] { only.position } else { local };
	if choices.len() <= 1 {
		choices.clear();
	}
	let mut edits: BTreeMap<&str, (&str, &Edit<Option<StoredBookmark>>)> = BTreeMap::new();
	for (device, replica) in replicas {
		for (key, edit) in &replica.bookmarks {
			let latest = edits.entry(key).or_insert((device, edit));
			if (edit.sequence, device.as_str()) > (latest.1.sequence, latest.0) {
				*latest = (device, edit);
			}
		}
	}
	let mut bookmarks: Vec<_> = edits.into_values().filter_map(|(_, edit)| edit.value.clone()).collect();
	bookmarks.sort_by_key(|bookmark| (bookmark.start, bookmark.audio_ms, bookmark.end));
	let temporary_bookmark = replicas
		.iter()
		.filter_map(|(device, replica)| replica.temporary_bookmark.as_ref().map(|edit| (device, edit)))
		.max_by_key(|(device, edit)| (edit.sequence, device.as_str()))
		.and_then(|(_, edit)| edit.value);
	MergedData { data: ReadingData { resume, bookmarks, temporary_bookmark }, choices }
}

#[cfg(test)]
mod tests;
