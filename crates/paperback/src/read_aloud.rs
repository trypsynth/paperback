//! Read aloud on the desktop: the speech engine, and core's read-aloud audio played on the output device audio books use.

#[cfg(windows)]
pub mod onecore;
#[cfg(windows)]
mod wav;

use std::{num::NonZero, rc::Rc, sync::Arc};

use anyhow::Result;
use paperback_core::{
	config::ConfigManager,
	document::DocPos,
	session::DocumentSession,
	speech::{AudioFeed, FeedSample, ReadAloud, ReadAloudSettings, SpeechEngine, choose_voice},
};
use rodio::{ChannelCount, MixerDeviceSink, Player, SampleRate, Source};

use crate::audio_player;

/// One step of the rate keys, as a percentage of the engine's range.
pub const RATE_STEP: i32 = 5;
const DEFAULT_PERCENT: i32 = 50;
const MAX_PAUSE_MS: i32 = 1000;
const VOLUME_STEP: f32 = 0.1;
const MAX_VOLUME: f32 = 2.0;

/// Reads the active document aloud through the system's speech engine.
pub struct Speaker {
	engine: Arc<dyn SpeechEngine>,
	read_aloud: ReadAloud,
	/// Dropping the last handle closes the output device.
	device: Rc<MixerDeviceSink>,
	/// A new one for each start, since clearing a player blocks the UI thread.
	player: Option<Player>,
	paused: bool,
	volume: f32,
	/// The paragraph the caret was last moved to, so it only moves when a new one starts.
	followed: Option<DocPos>,
}

impl Speaker {
	/// # Errors
	///
	/// Returns why read aloud can't run here, such as no speech engine on this system yet.
	pub fn new() -> Result<Self> {
		let engine = engine()?;
		Ok(Self {
			read_aloud: ReadAloud::new(Arc::clone(&engine)),
			engine,
			device: audio_player::shared_device()?,
			player: None,
			paused: false,
			volume: 1.0,
			followed: None,
		})
	}

	/// Starts reading at the paragraph holding `position`, with the book's voice and rate, or the app's.
	pub fn start(&mut self, session: &DocumentSession, position: DocPos, config: &ConfigManager, path: &str) {
		let settings = self.settings(config, path);
		let feed = self.read_aloud.start(session, position, settings);
		let player = Player::connect_new(self.device.mixer());
		player.set_volume(self.volume);
		player.append(FeedSource(feed));
		player.play();
		self.player = Some(player);
		self.paused = false;
		self.followed = None;
	}

	/// Starts the paragraph playing again, so a changed rate or voice is heard straight away.
	pub fn restart(&mut self, session: &DocumentSession, config: &ConfigManager, path: &str) {
		if let Some(paragraph) = self.read_aloud.progress().paragraph {
			self.start(session, paragraph, config, path);
		}
	}

	/// Moves to the paragraph before or after the one playing. Returns `false` at either end of the document.
	pub fn skip(&mut self, session: &DocumentSession, forward: bool, config: &ConfigManager, path: &str) -> bool {
		let Some(target) = self.read_aloud.neighbour(session, forward) else { return false };
		self.start(session, target, config, path);
		true
	}

	/// Whether reading has started and not stopped or reached the end, paused or not.
	#[must_use]
	pub fn is_reading(&self) -> bool {
		self.player.is_some() && !self.read_aloud.progress().finished
	}

	#[must_use]
	pub const fn is_paused(&self) -> bool {
		self.paused
	}

	pub fn pause(&mut self) {
		if let Some(player) = &self.player {
			player.pause();
			self.paused = true;
		}
	}

	pub fn resume(&mut self) {
		if let Some(player) = &self.player {
			player.play();
			self.paused = false;
		}
	}

	pub fn stop(&mut self) {
		self.read_aloud.stop();
		self.player = None;
		self.paused = false;
		self.followed = None;
	}

	/// Prepares the paragraphs ahead, and returns a paragraph that has just started, for the caret to move to. Call it on every tick while reading.
	pub fn pump(&mut self, session: &DocumentSession) -> Option<DocPos> {
		self.player.as_ref()?;
		self.read_aloud.refill(session);
		for error in self.read_aloud.take_errors() {
			tracing::warn!(%error, "read aloud: a paragraph couldn't be spoken, so it was skipped");
		}
		let progress = self.read_aloud.progress();
		if progress.finished {
			self.player = None;
			return None;
		}
		if progress.paragraph == self.followed {
			return None;
		}
		self.followed = progress.paragraph;
		progress.paragraph
	}

	/// Returns the new volume, and whether it was already at its limit.
	pub fn change_volume(&mut self, increase: bool) -> (f32, bool) {
		let before = self.volume;
		let step = if increase { VOLUME_STEP } else { -VOLUME_STEP };
		self.volume = ((self.volume + step) * 10.0).round().clamp(1.0, MAX_VOLUME * 10.0) / 10.0;
		if let Some(player) = &self.player {
			player.set_volume(self.volume);
		}
		(self.volume, (self.volume - before).abs() < f32::EPSILON)
	}

	/// The key the app's rate is kept under, one per engine, since each engine's range differs.
	#[must_use]
	pub fn rate_key(&self) -> String {
		format!("tts_rate_{}", self.engine.id())
	}

	fn settings(&self, config: &ConfigManager, path: &str) -> ReadAloudSettings {
		let book = config.get_document_speech(path);
		let engine = self.engine.id();
		let app_voice = config.get_app_string(&format!("tts_voice_{engine}"), "");
		let voices = self.engine.voices().unwrap_or_else(|error| {
			tracing::warn!(%error, "read aloud: couldn't list voices, so the engine's default speaks");
			Vec::new()
		});
		let voice =
			choose_voice(&voices, &[book.voice.as_deref(), Some(app_voice.as_str()).filter(|v| !v.is_empty())], None)
				.cloned();
		let percent = |own: Option<i32>, key: &str| {
			let value = own.unwrap_or_else(|| config.get_app_int(&format!("{key}_{engine}"), DEFAULT_PERCENT));
			u8::try_from(value.clamp(0, 100)).unwrap_or(50)
		};
		ReadAloudSettings {
			voice,
			rate_percent: percent(book.rate_percent, "tts_rate"),
			pitch_percent: percent(book.pitch_percent, "tts_pitch"),
			rules: Vec::new(),
			paragraph_pause_ms: u32::try_from(config.get_app_int("tts_paragraph_pause_ms", 0).clamp(0, MAX_PAUSE_MS))
				.unwrap_or(0),
		}
	}
}

#[cfg(windows)]
#[allow(clippy::unnecessary_wraps, reason = "other systems have no engine yet, so callers handle failure")]
fn engine() -> Result<Arc<dyn SpeechEngine>> {
	Ok(Arc::new(onecore::OneCore))
}

#[cfg(not(windows))]
fn engine() -> Result<Arc<dyn SpeechEngine>> {
	anyhow::bail!("there's no speech engine for this system yet")
}

/// Core's read-aloud audio as a rodio source.
struct FeedSource(AudioFeed);

impl Iterator for FeedSource {
	type Item = rodio::Sample;

	fn next(&mut self) -> Option<Self::Item> {
		match self.0.next_sample() {
			FeedSample::Sample(sample) => Some(f32::from(sample) / 32_768.0),
			// The next paragraph is still being synthesized; ending here would end reading.
			FeedSample::Waiting => Some(0.0),
			FeedSample::Done => None,
		}
	}
}

impl Source for FeedSource {
	/// A paragraph at a time, since the rate could differ from one to the next; one sample at a time while waiting, so a paragraph that arrives is played at its own rate.
	fn current_span_len(&self) -> Option<usize> {
		Some(self.0.samples_left_in_paragraph().max(1))
	}

	fn channels(&self) -> ChannelCount {
		NonZero::<u16>::MIN
	}

	fn sample_rate(&self) -> SampleRate {
		NonZero::new(self.0.sample_rate()).unwrap_or(NonZero::<u32>::MIN)
	}

	fn total_duration(&self) -> Option<std::time::Duration> {
		None
	}
}
