//! Continuous reading: paragraphs from the document, synthesized ahead on a worker thread, played through a feed the platform's audio output pulls from.
//!
//! The session stays on the UI thread, since OCR and navigation change it there, so [`ReadAloud`] reads paragraphs from it there and hands only their text to the worker. Each [`ReadAloud::start`] gives an [`AudioFeed`] for the audio thread, which records which paragraph and word is playing, for the UI to follow.

use std::{
	collections::VecDeque,
	sync::{
		Arc, Mutex, MutexGuard, PoisonError,
		atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering},
		mpsc::{self, Receiver, Sender},
	},
	thread,
};

use super::{SpeechEngine, SpeechRequest, SpeechRule, Voice, apply_rules};
use crate::{
	document::{DocLen, DocPos},
	session::{DocumentSession, SegmentDirectionFfi},
};

/// How many paragraphs are prepared beyond the one playing, so the next one is ready when it ends even on a slow engine.
const AHEAD: usize = 2;
/// Used until the first paragraph's audio arrives, so the feed has a rate to report.
const FALLBACK_SAMPLE_RATE: u32 = 22_050;

/// How read aloud speaks, fixed for one run of reading. Changing any of it means starting again from the current paragraph.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ReadAloudSettings {
	/// The chosen voice, or `None` for the engine's default. Its language decides which voice-specific rules apply.
	pub voice: Option<Voice>,
	pub rate_percent: u8,
	pub pitch_percent: u8,
	pub rules: Vec<SpeechRule>,
	/// Silence after each paragraph.
	pub paragraph_pause_ms: u32,
}

/// A paragraph to read: where it starts, where its text starts past any leading whitespace, and its text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SpeechParagraph {
	pub start: DocPos,
	pub text_start: DocPos,
	pub text: String,
}

/// A paragraph's audio, ready to play.
#[derive(Debug, Clone, PartialEq, Eq)]
struct SpokenParagraph {
	start: DocPos,
	sample_rate: u32,
	/// The speech, then the pause after it.
	samples: Vec<i16>,
	/// Where each word starts in `samples`, and its place in the document, in order.
	words: Vec<(usize, DocPos)>,
}

struct Job {
	generation: u64,
	paragraph: SpeechParagraph,
	settings: Arc<ReadAloudSettings>,
}

/// What read aloud, the worker and the feed share.
#[derive(Default)]
struct Shared {
	/// Bumped to throw away everything prepared or playing, when reading starts over or stops.
	generation: AtomicU64,
	ready: Mutex<VecDeque<(u64, SpokenParagraph)>>,
	/// Paragraphs handed to the worker in this generation.
	queued: AtomicUsize,
	/// Paragraphs played, or skipped because the engine failed, in this generation, so read aloud knows how far ahead it is.
	played: AtomicUsize,
	/// No more paragraphs will come in this generation.
	last_queued: AtomicBool,
	playing_paragraph: AtomicUsize,
	playing_word: AtomicUsize,
	/// Set while a paragraph is playing, so the positions above mean something.
	has_position: AtomicBool,
	finished: AtomicBool,
	errors: Mutex<Vec<String>>,
}

impl Shared {
	fn ready(&self) -> MutexGuard<'_, VecDeque<(u64, SpokenParagraph)>> {
		self.ready.lock().unwrap_or_else(PoisonError::into_inner)
	}
}

/// Where reading has got to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReadAloudProgress {
	/// The start of the paragraph being spoken, which the caret follows.
	pub paragraph: Option<DocPos>,
	/// The word being spoken, for resuming at it and saving the place exactly.
	pub word: Option<DocPos>,
	/// Every paragraph to the end of the document has been played.
	pub finished: bool,
}

/// Reads a document aloud, paragraph by paragraph, from the UI thread.
pub struct ReadAloud {
	shared: Arc<Shared>,
	jobs: Sender<Job>,
	settings: Arc<ReadAloudSettings>,
	/// The paragraph after which the next one to prepare comes, or `None` when there's nothing more to prepare.
	queued_up_to: Option<DocPos>,
}

impl ReadAloud {
	/// Read aloud speaking through `engine`.
	#[must_use]
	pub fn new(engine: Arc<dyn SpeechEngine>) -> Self {
		let shared = Arc::new(Shared::default());
		shared.finished.store(true, Ordering::SeqCst);
		let (jobs, received) = mpsc::channel();
		let worker_shared = Arc::clone(&shared);
		thread::spawn(move || synthesize_jobs(engine.as_ref(), &received, &worker_shared));
		Self { shared, jobs, settings: Arc::default(), queued_up_to: None }
	}

	/// Starts reading at `position`, or past the whitespace there, throwing away anything prepared or playing, and returns the feed this reading's audio comes out of. A feed from an earlier start is done from now on.
	///
	/// Returns `None`, and reads nothing, when there's no text at or after `position`, such as at the end of the document.
	pub fn start(
		&mut self,
		session: &DocumentSession,
		position: DocPos,
		settings: ReadAloudSettings,
	) -> Option<AudioFeed> {
		self.reset();
		let Some(first) = Self::first_paragraph(session, position) else {
			self.shared.finished.store(true, Ordering::SeqCst);
			return None;
		};
		self.settings = Arc::new(settings);
		self.shared.finished.store(false, Ordering::SeqCst);
		let feed = AudioFeed {
			shared: Arc::clone(&self.shared),
			generation: self.shared.generation.load(Ordering::SeqCst),
			playing: None,
			starts_at: first.start,
		};
		self.queued_up_to = Some(first.start);
		self.queue(first);
		self.refill(session);
		Some(feed)
	}

	/// The paragraph holding `position`, from `position` on, or the next paragraph when only whitespace follows it in its own.
	fn first_paragraph(session: &DocumentSession, position: DocPos) -> Option<SpeechParagraph> {
		let paragraph = session.speech_paragraph(position, SegmentDirectionFfi::Current)?;
		if let Some(rest) = from_caret(&paragraph, position) {
			return Some(rest);
		}
		session.speech_paragraph(paragraph.start, SegmentDirectionFfi::Next).filter(|next| next.start > paragraph.start)
	}

	/// Prepares paragraphs until [`AHEAD`] are waiting beyond the one playing. Call it often, such as on the timer that moves the caret.
	pub fn refill(&mut self, session: &DocumentSession) {
		while self.shared.queued.load(Ordering::SeqCst).saturating_sub(self.shared.played.load(Ordering::SeqCst))
			<= AHEAD
		{
			let Some(after) = self.queued_up_to else { return };
			let Some(next) =
				session.speech_paragraph(after, SegmentDirectionFfi::Next).filter(|next| next.start > after)
			else {
				self.queued_up_to = None;
				self.shared.last_queued.store(true, Ordering::SeqCst);
				return;
			};
			self.queued_up_to = Some(next.start);
			self.queue(next);
		}
	}

	/// The start of the paragraph before or after the one playing, to skip to. `None` at either end of the document, or when nothing has played yet.
	#[must_use]
	pub fn neighbour(&self, session: &DocumentSession, forward: bool) -> Option<DocPos> {
		let current = self.progress().paragraph?;
		let direction = if forward { SegmentDirectionFfi::Next } else { SegmentDirectionFfi::Previous };
		session.speech_paragraph(current, direction).map(|paragraph| paragraph.start).filter(|&start| start != current)
	}

	/// Stops reading and throws away everything prepared.
	pub fn stop(&mut self) {
		self.reset();
		self.shared.finished.store(true, Ordering::SeqCst);
	}

	#[must_use]
	pub fn progress(&self) -> ReadAloudProgress {
		let has_position = self.shared.has_position.load(Ordering::SeqCst);
		let at = |value: &AtomicUsize| has_position.then(|| DocPos::from_raw(value.load(Ordering::SeqCst)));
		ReadAloudProgress {
			paragraph: at(&self.shared.playing_paragraph),
			word: at(&self.shared.playing_word),
			finished: self.shared.finished.load(Ordering::SeqCst),
		}
	}

	/// What the engine reported going wrong since the last call, such as a paragraph it couldn't speak. Reading carries on past it.
	#[must_use]
	pub fn take_errors(&self) -> Vec<String> {
		std::mem::take(&mut *self.shared.errors.lock().unwrap_or_else(PoisonError::into_inner))
	}

	fn queue(&self, paragraph: SpeechParagraph) {
		let generation = self.shared.generation.load(Ordering::SeqCst);
		if self.jobs.send(Job { generation, paragraph, settings: Arc::clone(&self.settings) }).is_ok() {
			self.shared.queued.fetch_add(1, Ordering::SeqCst);
		}
	}

	fn reset(&mut self) {
		self.shared.generation.fetch_add(1, Ordering::SeqCst);
		self.shared.ready().clear();
		self.shared.queued.store(0, Ordering::SeqCst);
		self.shared.played.store(0, Ordering::SeqCst);
		self.shared.last_queued.store(false, Ordering::SeqCst);
		self.shared.has_position.store(false, Ordering::SeqCst);
		self.queued_up_to = None;
	}
}

impl Drop for ReadAloud {
	fn drop(&mut self) {
		self.stop();
	}
}

/// One reading's audio, for the audio thread: mono 16-bit samples at [`AudioFeed::sample_rate`].
pub struct AudioFeed {
	shared: Arc<Shared>,
	/// The reading this feed belongs to. Once read aloud starts again or stops, the feed is done.
	generation: u64,
	playing: Option<Playing>,
	starts_at: DocPos,
}

struct Playing {
	paragraph: SpokenParagraph,
	next_sample: usize,
	next_word: usize,
}

/// What the feed has for the player next.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FeedSample {
	Sample(i16),
	/// The next paragraph isn't ready yet. Play silence rather than ending, since it's coming.
	Waiting,
	/// Reading has reached the end of the document, or was stopped or started again.
	Done,
}

impl AudioFeed {
	/// Where this reading starts, which [`ReadAloudProgress::paragraph`] is while its first paragraph plays.
	#[must_use]
	pub const fn starts_at(&self) -> DocPos {
		self.starts_at
	}

	/// The rate of the paragraph playing, or of the next one once this one ends.
	#[must_use]
	pub fn sample_rate(&self) -> u32 {
		self.playing.as_ref().map_or_else(
			|| self.shared.ready().front().map_or(FALLBACK_SAMPLE_RATE, |(_, paragraph)| paragraph.sample_rate),
			|playing| playing.paragraph.sample_rate,
		)
	}

	/// Samples left in the paragraph playing, so a player can tell when the rate may change.
	#[must_use]
	pub fn samples_left_in_paragraph(&self) -> usize {
		self.playing.as_ref().map_or(0, |playing| playing.paragraph.samples.len() - playing.next_sample)
	}

	pub fn next_sample(&mut self) -> FeedSample {
		loop {
			if self.shared.generation.load(Ordering::SeqCst) != self.generation {
				self.playing = None;
				return FeedSample::Done;
			}
			if let Some(playing) = self.playing.as_mut() {
				while let Some(&(sample, position)) = playing.paragraph.words.get(playing.next_word)
					&& sample <= playing.next_sample
				{
					self.shared.playing_word.store(position.get(), Ordering::SeqCst);
					playing.next_word += 1;
				}
				if let Some(&sample) = playing.paragraph.samples.get(playing.next_sample) {
					playing.next_sample += 1;
					return FeedSample::Sample(sample);
				}
				self.playing = None;
				self.shared.played.fetch_add(1, Ordering::SeqCst);
			}
			let next = self.shared.ready().pop_front();
			match next {
				Some((generation, paragraph)) if generation == self.generation => {
					self.shared.playing_paragraph.store(paragraph.start.get(), Ordering::SeqCst);
					self.shared.playing_word.store(paragraph.start.get(), Ordering::SeqCst);
					self.shared.has_position.store(true, Ordering::SeqCst);
					self.playing = Some(Playing { paragraph, next_sample: 0, next_word: 0 });
				}
				// Left over from an earlier reading.
				Some(_) => {}
				None if self.shared.last_queued.load(Ordering::SeqCst)
					&& self.shared.played.load(Ordering::SeqCst) >= self.shared.queued.load(Ordering::SeqCst) =>
				{
					self.shared.finished.store(true, Ordering::SeqCst);
					return FeedSample::Done;
				}
				None => return FeedSample::Waiting,
			}
		}
	}
}

/// `paragraph` from `position` on, past any whitespace there. `None` when only whitespace follows it in the paragraph.
fn from_caret(paragraph: &SpeechParagraph, position: DocPos) -> Option<SpeechParagraph> {
	if position <= paragraph.text_start {
		return Some(paragraph.clone());
	}
	let chars: Vec<char> = paragraph.text.chars().collect();
	let mut at = (position.get() - paragraph.text_start.get()).min(chars.len());
	while chars.get(at).is_some_and(|c| c.is_whitespace()) {
		at += 1;
	}
	if at == chars.len() {
		return None;
	}
	let text_start = paragraph.text_start + DocLen::from_raw(at);
	Some(SpeechParagraph { start: text_start, text_start, text: chars[at..].iter().collect() })
}

fn synthesize_jobs(engine: &dyn SpeechEngine, jobs: &Receiver<Job>, shared: &Shared) {
	for job in jobs {
		if job.generation != shared.generation.load(Ordering::SeqCst) {
			continue;
		}
		let spoken = speak(engine, &job.paragraph, &job.settings);
		if job.generation != shared.generation.load(Ordering::SeqCst) {
			continue;
		}
		match spoken {
			Ok(spoken) => shared.ready().push_back((job.generation, spoken)),
			Err(error) => {
				shared.errors.lock().unwrap_or_else(PoisonError::into_inner).push(format!("{error:#}"));
				// Counted as played, so reading moves on past it.
				shared.played.fetch_add(1, Ordering::SeqCst);
			}
		}
	}
}

fn speak(
	engine: &dyn SpeechEngine,
	paragraph: &SpeechParagraph,
	settings: &ReadAloudSettings,
) -> anyhow::Result<SpokenParagraph> {
	let (text, map) = apply_rules(&settings.rules, &paragraph.text, settings.voice.as_ref());
	let request = SpeechRequest {
		text,
		voice: settings.voice.as_ref().map(|voice| voice.id.clone()),
		rate_percent: settings.rate_percent,
		pitch_percent: settings.pitch_percent,
	};
	let mut synthesis = engine.synthesize(&request)?;
	let sample_rate = synthesis.sample_rate();
	let mut samples = Vec::new();
	let mut words = Vec::new();
	while let Some(chunk) = synthesis.next_chunk() {
		let chunk = chunk?;
		for word in &chunk.words {
			let offset = DocLen::from_raw(map.to_original(word.text_offset));
			words.push((samples.len() + word.sample, paragraph.text_start + offset));
		}
		samples.extend_from_slice(&chunk.samples);
	}
	let pause = usize::try_from(u64::from(sample_rate) * u64::from(settings.paragraph_pause_ms) / 1000).unwrap_or(0);
	samples.resize(samples.len() + pause, 0);
	Ok(SpokenParagraph { start: paragraph.start, sample_rate, samples, words })
}

#[cfg(test)]
mod tests {
	use std::time::{Duration, Instant};

	use super::*;
	use crate::speech::{AudioChunk, MatchType, RuleScope, Synthesis, WordBoundary};

	const SAMPLES_PER_CHAR: usize = 10;

	/// Ten samples a char at 1000 a second, the char's code as each sample, and a word boundary at each word's start. Fails on any paragraph containing "FAIL".
	struct FakeEngine;

	struct FakeSynthesis(Option<AudioChunk>);

	impl Synthesis for FakeSynthesis {
		fn sample_rate(&self) -> u32 {
			1000
		}

		fn next_chunk(&mut self) -> Option<anyhow::Result<AudioChunk>> {
			self.0.take().map(Ok)
		}
	}

	impl SpeechEngine for FakeEngine {
		fn id(&self) -> &'static str {
			"fake"
		}

		fn voices(&self) -> anyhow::Result<Vec<Voice>> {
			Ok(Vec::new())
		}

		fn synthesize(&self, request: &SpeechRequest) -> anyhow::Result<Box<dyn Synthesis>> {
			anyhow::ensure!(!request.text.contains("FAIL"), "the engine can't say {:?}", request.text);
			let chars: Vec<char> = request.text.chars().collect();
			let samples =
				chars.iter().flat_map(|&c| [i16::try_from(u32::from(c)).unwrap_or(1); SAMPLES_PER_CHAR]).collect();
			let words = (0..chars.len())
				.filter(|&i| !chars[i].is_whitespace() && (i == 0 || chars[i - 1].is_whitespace()))
				.map(|i| WordBoundary { sample: i * SAMPLES_PER_CHAR, text_offset: i, text_len: 1 })
				.collect();
			Ok(Box::new(FakeSynthesis(Some(AudioChunk { samples, words }))))
		}
	}

	fn read_aloud() -> ReadAloud {
		ReadAloud::new(Arc::new(FakeEngine))
	}

	/// Plays `feed` to the end, as the platform would, refilling as it goes, and returns the samples and each paragraph start heard.
	fn play(reader: &mut ReadAloud, session: &DocumentSession, feed: &mut AudioFeed) -> (Vec<i16>, Vec<DocPos>) {
		let deadline = Instant::now() + Duration::from_secs(10);
		let (mut samples, mut paragraphs) = (Vec::new(), Vec::new());
		loop {
			assert!(Instant::now() < deadline, "reading never finished");
			match feed.next_sample() {
				FeedSample::Sample(sample) => samples.push(sample),
				FeedSample::Waiting => {
					reader.refill(session);
					thread::sleep(Duration::from_millis(1));
				}
				FeedSample::Done => return (samples, paragraphs),
			}
			if let Some(paragraph) = reader.progress().paragraph
				&& paragraphs.last() != Some(&paragraph)
			{
				paragraphs.push(paragraph);
			}
		}
	}

	fn spoken(samples: &[i16]) -> String {
		samples
			.chunks(SAMPLES_PER_CHAR)
			.map(|chunk| char::from_u32(u32::try_from(chunk[0]).unwrap_or(0)).unwrap_or('_'))
			.map(|c| if c == '\0' { '|' } else { c })
			.collect()
	}

	#[test]
	fn every_paragraph_with_text_is_read_in_order_from_the_position() {
		let text = "One two.\n\nThree four.\n   \nFive.";
		let session = DocumentSession::from_text(text);
		let mut reader = read_aloud();
		let mut feed = reader
			.start(&session, DocPos::from_raw(text.find("four").unwrap()), ReadAloudSettings::default())
			.expect("there is text to read");
		let (samples, paragraphs) = play(&mut reader, &session, &mut feed);
		assert_eq!(spoken(&samples), "four.Five.");
		let starts: Vec<usize> = paragraphs.iter().map(|p| p.get()).collect();
		assert_eq!(starts, [text.find("four").unwrap(), text.find("Five").unwrap()]);
		assert!(reader.progress().finished);
	}

	#[test]
	fn a_start_inside_a_word_reads_from_there_without_going_back() {
		let text = "One word. Two.";
		let session = DocumentSession::from_text(text);
		let mut reader = read_aloud();
		let mut feed = reader
			.start(&session, DocPos::from_raw(text.find('.').unwrap()), ReadAloudSettings::default())
			.expect("there is text to read");
		let (samples, _) = play(&mut reader, &session, &mut feed);
		assert_eq!(spoken(&samples), ". Two.");
	}

	#[test]
	fn a_start_between_words_reads_from_the_next_word() {
		let text = "One two  three.\nFour.";
		let session = DocumentSession::from_text(text);
		let mut reader = read_aloud();
		let mut feed = reader
			.start(&session, DocPos::from_raw(text.find("  three").unwrap() + 1), ReadAloudSettings::default())
			.expect("there is text to read");
		let (samples, _) = play(&mut reader, &session, &mut feed);
		assert_eq!(spoken(&samples), "three.Four.");
	}

	#[test]
	fn a_start_past_a_paragraphs_last_word_reads_from_the_next_paragraph() {
		let text = "One two.   \nThree.";
		let session = DocumentSession::from_text(text);
		let mut reader = read_aloud();
		let mut feed = reader
			.start(&session, DocPos::from_raw(text.find('\n').unwrap() - 1), ReadAloudSettings::default())
			.expect("there is text to read");
		let (samples, _) = play(&mut reader, &session, &mut feed);
		assert_eq!(spoken(&samples), "Three.");
	}

	#[test]
	fn a_start_partway_through_still_places_each_word_in_the_document() {
		let text = "Ask Dr Smith now.";
		let session = DocumentSession::from_text(text);
		let mut reader = read_aloud();
		let mut feed = reader
			.start(&session, DocPos::from_raw(text.find("Smith").unwrap()), ReadAloudSettings::default())
			.expect("there is text to read");
		// "Smith now." has "now" at char 6.
		let mut heard = 0;
		while heard <= 6 * SAMPLES_PER_CHAR {
			if matches!(feed.next_sample(), FeedSample::Sample(_)) {
				heard += 1;
			} else {
				thread::sleep(Duration::from_millis(1));
			}
		}
		assert_eq!(reader.progress().word, Some(DocPos::from_raw(text.find("now").unwrap())));
		assert_eq!(reader.progress().paragraph, Some(DocPos::from_raw(text.find("Smith").unwrap())));
	}

	#[test]
	fn the_paragraph_pause_is_silence_after_each_paragraph() {
		let session = DocumentSession::from_text("Ab\nCd");
		let mut reader = read_aloud();
		let settings = ReadAloudSettings { paragraph_pause_ms: 20, ..ReadAloudSettings::default() };
		let mut feed = reader.start(&session, DocPos::from_raw(0), settings).expect("there is text to read");
		let (samples, _) = play(&mut reader, &session, &mut feed);
		assert_eq!(spoken(&samples), "Ab||Cd||");
	}

	#[test]
	fn the_word_playing_is_its_place_in_the_document_even_after_a_rule_rewrote_it() {
		let text = "  Ask Dr Smith.";
		let session = DocumentSession::from_text(text);
		let mut reader = read_aloud();
		let rule = SpeechRule {
			scope: RuleScope::Word,
			match_type: MatchType::Literal,
			pattern: "Dr".into(),
			replacement: "Doctor".into(),
			..SpeechRule::default()
		};
		let settings = ReadAloudSettings { rules: vec![rule], ..ReadAloudSettings::default() };
		let mut feed = reader.start(&session, DocPos::from_raw(0), settings).expect("there is text to read");
		// "Ask Doctor Smith." has "Smith" at char 11.
		let mut heard = 0;
		while heard <= 11 * SAMPLES_PER_CHAR {
			if matches!(feed.next_sample(), FeedSample::Sample(_)) {
				heard += 1;
			} else {
				thread::sleep(Duration::from_millis(1));
			}
		}
		assert_eq!(reader.progress().word, Some(DocPos::from_raw(text.find("Smith").unwrap())));
		assert_eq!(reader.progress().paragraph, Some(DocPos::from_raw(0)));
	}

	#[test]
	fn starting_again_ends_the_old_feed_and_reads_from_the_new_place() {
		let text = "First.\nSecond.\nThird.";
		let session = DocumentSession::from_text(text);
		let mut reader = read_aloud();
		let mut old =
			reader.start(&session, DocPos::from_raw(0), ReadAloudSettings::default()).expect("there is text to read");
		let mut new = reader
			.start(&session, DocPos::from_raw(text.find("Third").unwrap()), ReadAloudSettings::default())
			.expect("there is text to read");
		assert_eq!(old.next_sample(), FeedSample::Done);
		let (samples, _) = play(&mut reader, &session, &mut new);
		assert_eq!(spoken(&samples), "Third.");
	}

	#[test]
	fn a_paragraph_the_engine_fails_on_is_skipped_and_reported() {
		let session = DocumentSession::from_text("One.\nFAIL here.\nThree.");
		let mut reader = read_aloud();
		let mut feed =
			reader.start(&session, DocPos::from_raw(0), ReadAloudSettings::default()).expect("there is text to read");
		let (samples, _) = play(&mut reader, &session, &mut feed);
		assert_eq!(spoken(&samples), "One.Three.");
		assert_eq!(reader.take_errors().len(), 1);
	}

	#[test]
	fn the_neighbours_of_the_paragraph_playing_are_found_for_skipping() {
		let text = "One.

Two.
Three.";
		let session = DocumentSession::from_text(text);
		let mut reader = read_aloud();
		assert_eq!(reader.neighbour(&session, true), None, "nothing has played yet");
		let mut feed = reader
			.start(&session, DocPos::from_raw(text.find("Two").unwrap()), ReadAloudSettings::default())
			.expect("there is text to read");
		while reader.progress().paragraph.is_none() {
			if feed.next_sample() == FeedSample::Waiting {
				thread::sleep(Duration::from_millis(1));
			}
		}
		assert_eq!(reader.neighbour(&session, true), Some(DocPos::from_raw(text.find("Three").unwrap())));
		assert_eq!(reader.neighbour(&session, false), Some(DocPos::from_raw(0)));
	}

	#[test]
	fn nothing_is_read_past_the_last_paragraph() {
		let text = "Some text.\n\n";
		let session = DocumentSession::from_text(text);
		let mut reader = read_aloud();
		assert!(reader.start(&session, DocPos::from_raw(text.len()), ReadAloudSettings::default()).is_none());
		assert!(reader.progress().finished);
		assert!(
			reader.start(&DocumentSession::from_text(""), DocPos::from_raw(0), ReadAloudSettings::default()).is_none()
		);
	}

	#[test]
	fn stopping_ends_the_feed() {
		let mut reader = read_aloud();
		let session = DocumentSession::from_text("Some text.");
		let mut feed =
			reader.start(&session, DocPos::from_raw(0), ReadAloudSettings::default()).expect("there is text to read");
		reader.stop();
		assert_eq!(feed.next_sample(), FeedSample::Done);
		assert!(reader.progress().finished);
	}
}
