use std::{collections::HashSet, sync::Mutex};

#[cfg(all(feature = "book-sync", feature = "uniffi"))]
mod drive_sync;

use crate::{
	config::{Bookmark, ConfigManager, DocumentSpeech, NavigationUnit},
	document::{DocPos, PageLayout},
	parser::ParserRegistry,
	reader_core::{AUDIO_BOOKMARK_REACH_MS, audio_bookmark_navigate, bookmark_navigate},
	speech::{RuleProblem, SpeechRule, Voice, apply_rules},
	types::{BookmarkFfi, BookmarkStepFfi},
};

#[cfg_attr(feature = "uniffi", derive(uniffi::Object))]
pub struct ConfigManagerFfi {
	inner: Mutex<ConfigManager>,
}

// Outside the exported block so it exists without the FFI; uniffi::export doesn't expand cfg_attr, so the constructor below needs a literal `uniffi::constructor`.
impl ConfigManagerFfi {
	#[must_use]
	pub fn new() -> Self {
		Self { inner: Mutex::new(ConfigManager::new()) }
	}

	#[cfg(test)]
	pub(crate) fn in_memory() -> Self {
		Self { inner: Mutex::new(ConfigManager::in_memory()) }
	}

	pub(crate) fn with<R>(&self, f: impl FnOnce(&ConfigManager) -> R) -> R {
		f(&self.inner.lock().unwrap())
	}
}

/// A negative time means none, as `set_document_audio_time_ffi` treats it.
fn audio_time(audio_ms: Option<i64>) -> Option<u64> {
	audio_ms.and_then(|ms| u64::try_from(ms).ok())
}

fn ffi_time(ms: Option<u64>) -> Option<i64> {
	ms.and_then(|ms| i64::try_from(ms).ok())
}

fn bookmark_ffi(bookmark: Option<Bookmark>) -> BookmarkFfi {
	bookmark.map_or_else(BookmarkFfi::default, |bm| BookmarkFfi {
		found: true,
		start: bm.start,
		end: bm.end,
		note: bm.note,
		audio_ms: ffi_time(bm.audio_ms),
	})
}

#[cfg(feature = "uniffi")]
#[uniffi::export]
impl ConfigManagerFfi {
	#[uniffi::constructor(name = "new")]
	fn uniffi_new() -> Self {
		Self::new()
	}
}

// Owned `String` params, because UniFFI's scaffolding passes the values it has just lifted and a borrow would not compile against it.
#[allow(clippy::needless_pass_by_value)]
#[cfg_attr(feature = "uniffi", uniffi::export)]
impl ConfigManagerFfi {
	pub fn initialize(&self, config_path: String) -> bool {
		self.inner.lock().unwrap().initialize(config_path.into())
	}

	pub fn get_doc_key(&self, path: String) -> String {
		self.inner.lock().unwrap().get_doc_key(&path)
	}

	pub fn get_app_string(&self, key: String, default_value: String) -> String {
		self.inner.lock().unwrap().get_app_string(&key, &default_value)
	}

	pub fn set_app_string(&self, key: String, value: String) {
		self.inner.lock().unwrap().set_app_string(&key, &value);
	}

	pub fn get_app_bool(&self, key: String, default_value: bool) -> bool {
		self.inner.lock().unwrap().get_app_bool(&key, default_value)
	}

	pub fn set_app_bool(&self, key: String, value: bool) {
		self.inner.lock().unwrap().set_app_bool(&key, value);
	}

	pub fn get_app_int(&self, key: String, default_value: i32) -> i32 {
		self.inner.lock().unwrap().get_app_int(&key, default_value)
	}

	pub fn set_app_int(&self, key: String, value: i32) {
		self.inner.lock().unwrap().set_app_int(&key, value);
	}

	pub fn associate_uri_with_local_file(&self, uri: String, local_path: String) {
		self.inner.lock().unwrap().associate_uri_with_local_file(&uri, &local_path);
	}

	pub fn set_document_position(&self, path: String, position: DocPos) {
		self.inner.lock().unwrap().set_document_position(&path, position);
	}

	pub fn get_document_position(&self, path: String) -> DocPos {
		self.inner.lock().unwrap().get_document_position(&path)
	}

	pub fn seed_document_position(&self, path: String, position: DocPos) {
		self.inner.lock().unwrap().seed_document_position(&path, position);
	}

	/// `time_ms < 0` leaves the stored value alone, so a document that hasn't started playing can't wipe it.
	pub fn set_document_audio_time_ffi(&self, path: String, time_ms: i64) {
		let time_ms = u64::try_from(time_ms).ok();
		self.inner.lock().unwrap().set_document_audio_time(&path, time_ms);
	}

	/// `-1` when no audio resume point has been stored for this document.
	pub fn get_document_audio_time_ffi(&self, path: String) -> i64 {
		self.inner.lock().unwrap().get_document_audio_time(&path).and_then(|ms| i64::try_from(ms).ok()).unwrap_or(-1)
	}

	pub fn set_document_speech(&self, path: String, speech: DocumentSpeech) {
		self.inner.lock().unwrap().set_document_speech(&path, &speech);
	}

	pub fn get_document_speech(&self, path: String) -> DocumentSpeech {
		self.inner.lock().unwrap().get_document_speech(&path)
	}

	pub fn set_document_navigation_unit(&self, path: String, unit: Option<NavigationUnit>) {
		self.inner.lock().unwrap().set_document_navigation_unit(&path, unit);
	}

	pub fn get_document_navigation_unit(&self, path: String) -> Option<NavigationUnit> {
		self.inner.lock().unwrap().get_document_navigation_unit(&path)
	}

	pub fn get_page_layout(&self) -> PageLayout {
		self.inner.lock().unwrap().get_page_layout()
	}

	pub fn set_page_layout(&self, layout: PageLayout) {
		self.inner.lock().unwrap().set_page_layout(layout);
	}

	/// `None` follows the app's page layout.
	pub fn get_document_page_layout(&self, path: String) -> Option<PageLayout> {
		self.inner.lock().unwrap().get_document_page_layout(&path)
	}

	pub fn set_document_page_layout(&self, path: String, layout: Option<PageLayout>) {
		self.inner.lock().unwrap().set_document_page_layout(&path, layout);
	}

	/// What to open the document at `path` with: its own page layout, or the app's.
	pub fn page_layout_for(&self, path: String) -> PageLayout {
		self.inner.lock().unwrap().page_layout_for(&path)
	}

	/// Only PDF and XPS documents have a page layout to choose, so the choice is offered for nothing else.
	pub fn page_layout_applies_to(&self, path: String) -> bool {
		PageLayout::applies_to(&path)
	}

	pub fn get_speech_rules(&self) -> Vec<SpeechRule> {
		self.inner.lock().unwrap().get_speech_rules()
	}

	pub fn set_speech_rules(&self, rules: Vec<SpeechRule>) {
		self.inner.lock().unwrap().set_speech_rules(rules);
	}

	/// Checked by core rather than the platform's own regular expressions, which accept patterns core can't apply.
	pub fn speech_rule_problem(&self, rule: SpeechRule) -> Option<RuleProblem> {
		rule.check().err()
	}

	/// The saved rules that fit the voice, applied to `text`; without a voice, only rules for every voice apply.
	pub fn apply_speech_rules(&self, text: String, voice_id: Option<String>, voice_language: Option<String>) -> String {
		let rules = self.inner.lock().unwrap().get_speech_rules();
		let voice = voice_id.map(|id| Voice { id, name: String::new(), language: voice_language.unwrap_or_default() });
		apply_rules(&rules, &text, voice.as_ref()).0
	}

	pub fn set_document_password(&self, path: String, password: String) {
		self.inner.lock().unwrap().set_document_password(&path, &password);
	}

	pub fn get_document_password(&self, path: String) -> String {
		self.inner.lock().unwrap().get_document_password(&path)
	}

	pub fn add_recent_document(&self, path: String) {
		self.inner.lock().unwrap().add_recent_document(&path);
	}

	pub fn get_recent_documents(&self) -> Vec<String> {
		self.inner.lock().unwrap().get_recent_documents()
	}

	pub fn clear_recent_documents(&self) {
		let inner = self.inner.lock().unwrap();
		inner.clear_recent_documents();
		inner.flush();
	}

	pub fn has_recent_documents(&self) -> bool {
		self.inner.lock().unwrap().has_recent_documents()
	}

	pub fn add_opened_document(&self, path: String) {
		self.inner.lock().unwrap().add_opened_document(&path);
	}

	pub fn remove_opened_document(&self, path: String) {
		self.inner.lock().unwrap().remove_opened_document(&path);
	}

	pub fn set_document_opened(&self, path: String, opened: bool) {
		self.inner.lock().unwrap().set_document_opened(&path, opened);
	}

	pub fn get_opened_documents(&self) -> Vec<String> {
		self.inner.lock().unwrap().get_opened_documents()
	}

	pub fn remove_document_history(&self, path: String) {
		self.inner.lock().unwrap().remove_document_history(&path);
	}

	pub fn rename_document_path(&self, old_path: String, new_path: String) {
		self.inner.lock().unwrap().rename_document_path(&old_path, &new_path);
	}

	pub fn get_supported_extensions(&self) -> Vec<String> {
		let mut exts = HashSet::new();
		for parser in ParserRegistry::global().all_parsers() {
			for ext in parser.extensions() {
				exts.insert((*ext).to_string());
			}
		}
		exts.into_iter().collect()
	}

	pub fn get_find_history(&self) -> Vec<String> {
		self.inner.lock().unwrap().get_find_history()
	}

	pub fn add_find_history(&self, text: String, max_len: i32) {
		self.inner.lock().unwrap().add_find_history(&text, usize::try_from(max_len).unwrap_or(0));
	}

	pub fn import_document_settings(&self, path: String) {
		self.inner.lock().unwrap().import_document_settings(&path);
	}

	pub fn import_settings_from_file(&self, doc_path: String, import_path: String) {
		self.inner.lock().unwrap().import_settings_from_file(&doc_path, &import_path);
	}

	pub fn export_document_settings(&self, doc_path: String, export_path: String) {
		self.inner.lock().unwrap().export_document_settings(&doc_path, &export_path);
	}

	pub fn flush(&self) {
		self.inner.lock().unwrap().flush();
	}

	/// The bookmark the reader is on: for an audio-only book (`audio_ms` set) the one within a moment of that time, otherwise the first whose start is in `start..end`, such as the paragraph being read.
	pub fn bookmark_at(&self, path: String, start: DocPos, end: DocPos, audio_ms: Option<i64>) -> BookmarkFfi {
		let inner = self.inner.lock().unwrap();
		bookmark_ffi(match audio_time(audio_ms) {
			Some(ms) => inner.audio_bookmark_near(&path, ms, AUDIO_BOOKMARK_REACH_MS),
			None => inner.text_bookmark_in(&path, start, end),
		})
	}

	/// A point bookmark at `position`, or at the playback time for an audio-only book. Saved straight away, as desktop does.
	pub fn add_bookmark(&self, path: String, position: DocPos, audio_ms: Option<i64>, note: String) {
		let inner = self.inner.lock().unwrap();
		match audio_time(audio_ms) {
			Some(ms) => inner.add_audio_bookmark(&path, position, ms, &note),
			None => inner.add_bookmark(&path, position, position, &note),
		}
		inner.flush();
	}

	/// Takes the bookmark as [`Self::bookmark_at`] or a row gave it, so a range bookmark made on desktop is removed whole.
	pub fn remove_bookmark(&self, path: String, start: DocPos, end: DocPos, audio_ms: Option<i64>) {
		let inner = self.inner.lock().unwrap();
		match audio_time(audio_ms) {
			Some(ms) => inner.remove_audio_bookmark(&path, ms),
			None => inner.remove_bookmark(&path, start, end),
		}
		inner.flush();
	}

	pub fn set_bookmark_note(&self, path: String, start: DocPos, end: DocPos, audio_ms: Option<i64>, note: String) {
		let inner = self.inner.lock().unwrap();
		match audio_time(audio_ms) {
			Some(ms) => inner.update_audio_bookmark_note(&path, ms, &note),
			None => inner.update_bookmark_note(&path, start, end, &note),
		}
		inner.flush();
	}

	/// Every bookmark in the document, in order, without the text each marks, for an app to keep and look up in without asking again.
	pub fn bookmarks(&self, path: String) -> Vec<BookmarkFfi> {
		self.inner.lock().unwrap().get_bookmarks(&path).into_iter().map(|bm| bookmark_ffi(Some(bm))).collect()
	}

	pub fn has_bookmarks(&self, path: String) -> bool {
		!self.inner.lock().unwrap().get_bookmarks(&path).is_empty()
	}

	/// The previous or next bookmark (or note, with `notes_only`) from `position`, or from the playback time for an audio-only book.
	pub fn step_bookmark(
		&self,
		path: String,
		position: DocPos,
		audio_ms: Option<i64>,
		wrap: bool,
		next: bool,
		notes_only: bool,
	) -> BookmarkStepFfi {
		let inner = self.inner.lock().unwrap();
		let audio = audio_time(audio_ms);
		let any = inner
			.get_bookmarks(&path)
			.iter()
			.any(|bm| (!notes_only || !bm.note.is_empty()) && (audio.is_none() || bm.audio_ms.is_some()));
		let nowhere = BookmarkStepFfi { any, ..BookmarkStepFfi::default() };
		if let Some(ms) = audio {
			let Some(hit) = audio_bookmark_navigate(&inner, &path, ms, wrap, next, notes_only) else { return nowhere };
			return BookmarkStepFfi {
				found: true,
				any,
				wrapped: hit.wrapped,
				start: hit.start,
				audio_ms: ffi_time(Some(hit.audio_ms)),
				note: hit.note,
				number: i32::try_from(hit.index + 1).unwrap_or(i32::MAX),
			};
		}
		let hit = bookmark_navigate(&inner, &path, position, wrap, next, notes_only);
		if !hit.found {
			return nowhere;
		}
		BookmarkStepFfi {
			found: true,
			any,
			wrapped: hit.wrapped,
			start: hit.start,
			audio_ms: None,
			note: hit.note,
			number: hit.index + 1,
		}
	}
}

impl Default for ConfigManagerFfi {
	fn default() -> Self {
		Self::new()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn config() -> ConfigManagerFfi {
		ConfigManagerFfi::in_memory()
	}

	#[test]
	fn the_saved_speech_rules_apply_to_the_voices_they_name() {
		use crate::speech::{MatchType, RuleScope, VoiceFilter};
		let config = config();
		let finnish = SpeechRule {
			pattern: "Dr.".into(),
			replacement: "tohtori".into(),
			voice_filter: VoiceFilter::Language("fi-FI".into()),
			..SpeechRule::default()
		};
		let everyone = SpeechRule { pattern: "&".into(), replacement: " and ".into(), ..SpeechRule::default() };
		config.set_speech_rules(vec![finnish, everyone]);
		assert_eq!(config.get_speech_rules().len(), 2);
		assert_eq!(
			config.apply_speech_rules("Dr. Who & co".into(), Some("mikko".into()), Some("fi-FI".into())),
			"tohtori Who  and  co"
		);
		assert_eq!(config.apply_speech_rules("Dr. Who & co".into(), None, None), "Dr. Who  and  co");
		let broken = SpeechRule {
			scope: RuleScope::Paragraph,
			match_type: MatchType::Regex,
			pattern: "(".into(),
			..SpeechRule::default()
		};
		assert!(matches!(config.speech_rule_problem(broken), Some(RuleProblem::InvalidRegex(_))));
		assert_eq!(config.speech_rule_problem(SpeechRule { pattern: "x".into(), ..SpeechRule::default() }), None);
	}

	#[test]
	fn a_bookmark_added_in_a_paragraph_is_found_there_and_removed_whole() {
		let config = config();
		config.add_bookmark("b.epub".into(), 12.into(), None, String::new());
		let here = config.bookmark_at("b.epub".into(), 10.into(), 20.into(), None);
		assert!(here.found);
		config.set_bookmark_note("b.epub".into(), here.start, here.end, None, "n".into());
		assert_eq!("n", config.bookmark_at("b.epub".into(), 10.into(), 20.into(), None).note);
		config.remove_bookmark("b.epub".into(), here.start, here.end, None);
		assert!(!config.has_bookmarks("b.epub".into()));
	}

	#[test]
	fn audio_bookmarks_are_found_by_time_and_stepped_by_time() {
		let config = config();
		config.add_bookmark("b.mp3".into(), 0.into(), Some(10_000), String::new());
		config.add_bookmark("b.mp3".into(), 0.into(), Some(60_000), "later".into());
		assert_eq!(Some(10_000), config.bookmark_at("b.mp3".into(), 0.into(), 0.into(), Some(10_900)).audio_ms);
		let step = config.step_bookmark("b.mp3".into(), 0.into(), Some(20_000), false, true, false);
		assert_eq!((true, Some(60_000), 2, "later"), (step.found, step.audio_ms, step.number, step.note.as_str()));
	}

	#[test]
	fn a_step_with_nowhere_to_go_says_whether_there_were_any() {
		let config = config();
		let none = config.step_bookmark("b.epub".into(), 0.into(), None, false, true, false);
		assert!(!none.found && !none.any);
		config.add_bookmark("b.epub".into(), 5.into(), None, String::new());
		let past = config.step_bookmark("b.epub".into(), 9.into(), None, false, true, false);
		assert!(!past.found && past.any);
		let wrapped = config.step_bookmark("b.epub".into(), 9.into(), None, true, true, false);
		assert_eq!((true, true, DocPos::from(5)), (wrapped.found, wrapped.wrapped, wrapped.start));
	}

	#[test]
	fn notes_only_ignores_plain_bookmarks_when_saying_whether_there_are_any() {
		let config = config();
		config.add_bookmark("b.epub".into(), 5.into(), None, String::new());
		assert!(!config.step_bookmark("b.epub".into(), 0.into(), None, true, true, true).any);
	}
}
