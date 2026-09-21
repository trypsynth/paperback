use std::{
	cell::RefCell,
	collections::hash_map::DefaultHasher,
	env, fs,
	hash::{Hash, Hasher},
	io::BufReader,
	path::{Path, PathBuf},
	rc::{Rc, Weak},
	sync::{Arc, Mutex},
	time::Duration,
};

use anyhow::{Context, Result};
use paperback_core::{
	audio::{AudioLocation, AudioTimeline, TimelinePoint},
	util::zip::extract_zip_entry_to_file_with_password,
};
use rodio::{Decoder, DeviceSinkBuilder, MixerDeviceSink, Player, Source};
use rodio_wsola::WsolaSourceExt;
use zip::ZipArchive;

/// How far short of a source's end a seek past that end lands, leaving enough to play out
/// before moving on to the next source.
const END_MARGIN_MS: u64 = 100;

/// The slowest and fastest playback speed multipliers `increase_speed`/`decrease_speed` will
/// step to, and the size of each step.
const MIN_SPEED: f32 = 0.5;
const MAX_SPEED: f32 = 3.0;
const SPEED_STEP: f32 = 0.1;

/// How often the live source checks `shared_speed` for a change made while it's already
/// playing (see `AudioPlayer::speed`'s doc comment).
const SPEED_POLL_PERIOD: Duration = Duration::from_millis(20);

/// A source decoded straight from its file, with the file's length handed to the decoder so a
/// constant-bitrate MP3 carrying no duration header still reports a usable `total_duration()`.
type FileDecoder = Decoder<BufReader<fs::File>>;

thread_local! {
	/// The one audio output device every open document plays through, opened on the first
	/// document that has narration and closed again once the last of them goes away.
	static AUDIO_DEVICE: RefCell<Weak<MixerDeviceSink>> = const { RefCell::new(Weak::new()) };
}

/// Plays a DAISY audiobook's narration against its `AudioTimeline` through rodio, decoding in
/// process rather than through whatever the OS happens to have installed. `Decoder` needs a
/// seekable file, so zip-embedded sources are extracted on use.
///
/// One rodio `Player` holds one source at a time. Switching source builds a fresh `Player`
/// and drops the old one, which stops it without blocking the UI thread the way `clear()` and
/// `stop()` both would.
pub struct AudioPlayer {
	/// Kept alive for as long as this player exists; dropping the last handle closes the
	/// output device.
	device: Rc<MixerDeviceSink>,
	player: Option<Player>,
	timeline: AudioTimeline,
	current_source: Option<usize>,
	/// The loaded source's own length as its decoder reports it, which a format that declares
	/// no duration leaves as `None`.
	current_length_ms: Option<u64>,
	/// Added to what the player reports, to turn it into a position within the source's file.
	/// A source is loaded by seeking its decoder before handing it over, and rodio counts from
	/// zero from there, so this holds that head start. A seek through the player instead makes
	/// rodio's own count absolute, and zeroes this.
	position_base_ms: u64,
	playing: bool,
	/// A seek requested while paused, applied lazily on resume. Applying it eagerly would
	/// decode a fresh source per keystroke while navigating with audio nobody is hearing.
	pending_target_ms: Option<u64>,
	/// `(source, seek_ms)` of the most recent seek, so navigation resolving to the same pair
	/// doesn't restart correct audio.
	last_seek_target: Option<(usize, u64)>,
	/// Where extracted zip-embedded sources are cached, keyed by archive+entry.
	cache_dir: PathBuf,
	/// The playback speed multiplier (1.0 = normal), applied to the loaded source and to
	/// whatever loads next. Reset to 1.0 only by creating a new `AudioPlayer`.
	///
	/// Speeding up plays the source through a WSOLA time-stretcher (`rodio_wsola::Wsola`)
	/// rather than through rodio's own `Player::set_speed`, which just plays samples faster
	/// (raising their pitch, the "chipmunk effect") rather than actually stretching time.
	/// `shared_speed` is the live channel into that already-running stretcher: `load_source`
	/// wraps each freshly loaded decoder in a `Wsola` that polls it every `SPEED_POLL_PERIOD`
	/// (`Wsola::set_speed` takes `&mut self`, and the wrapped source lives on rodio's own
	/// mixer thread once appended, out of this struct's reach otherwise).
	speed: f32,
	shared_speed: Arc<Mutex<f32>>,
}

impl AudioPlayer {
	pub fn new(timeline: AudioTimeline) -> Result<Self> {
		let cache_dir = env::temp_dir().join("paperback-audio-cache");
		fs::create_dir_all(&cache_dir).context("failed to create audio cache directory")?;
		Ok(Self {
			device: shared_device()?,
			player: None,
			timeline,
			current_source: None,
			current_length_ms: None,
			position_base_ms: 0,
			playing: false,
			pending_target_ms: None,
			last_seek_target: None,
			cache_dir,
			speed: 1.0,
			shared_speed: Arc::new(Mutex::new(1.0)),
		})
	}

	#[must_use]
	pub const fn timeline(&self) -> &AudioTimeline {
		&self.timeline
	}

	/// Stops playback and releases the decoder, ahead of this player being torn down.
	pub fn stop(&mut self) {
		self.playing = false;
		self.player = None;
		self.current_source = None;
		self.current_length_ms = None;
		self.position_base_ms = 0;
		self.last_seek_target = None;
	}

	#[must_use]
	pub fn is_playing(&self) -> bool {
		self.playing
	}

	pub fn play(&mut self) {
		self.playing = true;
		if let Some(target) = self.pending_target_ms.take() {
			self.seek_to_ms(target);
			return;
		}
		if let Some(player) = self.player.as_ref()
			&& !player.empty()
		{
			player.play();
			return;
		}
		// Nothing loaded yet, or the last source played out: pick up from wherever the
		// document was left.
		let elapsed = self.elapsed_ms().unwrap_or(0);
		self.seek_to_ms(elapsed);
	}

	pub fn pause(&mut self) {
		self.playing = false;
		if let Some(player) = self.player.as_ref() {
			player.pause();
		}
	}

	pub fn toggle(&mut self) {
		if self.is_playing() {
			self.pause();
		} else {
			self.play();
		}
	}

	/// The current playback speed multiplier (1.0 = normal).
	#[must_use]
	pub const fn speed(&self) -> f32 {
		self.speed
	}

	/// Sets the playback speed multiplier, clamped to `MIN_SPEED..=MAX_SPEED`, applying it to
	/// the currently loaded source (if any) as well as whatever loads next. Returns the speed
	/// actually applied, after clamping.
	///
	/// `position_ms` (and everything built on it: `elapsed_ms`, seeking, "is this the same
	/// spot" checks) reads the loaded source's position as `position_base_ms` plus however much
	/// wall-clock time the player reports played back, scaled by `speed` — see its own doc
	/// comment. That scaling is only correct for time played *at the current speed*, so a speed
	/// change re-seeks the live source to where it already was, which re-bases
	/// `position_base_ms`/`position_ms` against the new speed before anything is scaled by it.
	pub fn set_speed(&mut self, speed: f32) -> f32 {
		let new_speed = round_speed(speed.clamp(MIN_SPEED, MAX_SPEED));
		if (new_speed - self.speed).abs() < f32::EPSILON {
			return self.speed;
		}
		let resume_content_ms = self.position_ms();
		self.speed = new_speed;
		*self.shared_speed.lock().unwrap() = new_speed;
		if let (Some(player), Some(content_ms)) = (self.player.as_ref(), resume_content_ms) {
			let apparent = Duration::from_millis(content_ms).div_f32(new_speed);
			if player.try_seek(apparent).is_ok() {
				self.position_base_ms = 0;
			} else {
				tracing::debug!(content_ms, new_speed, "audio: re-seek after a speed change was refused");
			}
		}
		self.speed
	}

	/// Steps the playback speed up by `SPEED_STEP`, clamped at `MAX_SPEED`. Returns the new
	/// speed, which callers compare against the speed before the call to notice a clamp.
	pub fn increase_speed(&mut self) -> f32 {
		self.set_speed(self.speed + SPEED_STEP)
	}

	/// Steps the playback speed down by `SPEED_STEP`, clamped at `MIN_SPEED`.
	pub fn decrease_speed(&mut self) -> f32 {
		self.set_speed(self.speed - SPEED_STEP)
	}

	/// Moves on to the next source once the current one has played out, and notices the end of
	/// the last one. rodio has no "finished" callback, so this is driven by the same recurring
	/// tick that syncs the caret.
	pub fn pump(&mut self) {
		if !self.playing {
			return;
		}
		let Some(player) = self.player.as_ref() else {
			return;
		};
		if !player.empty() {
			return;
		}
		let Some(current) = self.current_source else {
			return;
		};
		let next = self.timeline.next_source_after(current);
		tracing::debug!(current, ?next, "audio: source finished");
		match next {
			Some(next) => {
				self.load_source(next, 0);
			}
			None => self.playing = false,
		}
	}

	/// Seeks playback to the point covering `position` in the text, if the timeline
	/// narrates it. Leaves the transport running or paused as it already was.
	pub fn seek_to_position(&mut self, position: usize) -> bool {
		let target = self.timeline.point_for_position(position);
		tracing::debug!(position, ?target, "audio: seek_to_position");
		target.is_some_and(|point| self.seek_to_ms(point.time_ms))
	}

	/// Seeks playback to `elapsed_ms` into the overall document timeline. While paused this
	/// only records the target (see `pending_target_ms`), applying it lazily on resume.
	pub fn seek_to_ms(&mut self, elapsed_ms: u64) -> bool {
		if !self.playing {
			tracing::debug!(elapsed_ms, "audio: seek_to_ms while paused, deferring");
			self.pending_target_ms = Some(elapsed_ms);
			return true;
		}
		let Some(cursor) = self.timeline.resolve(TimelinePoint::new(0, elapsed_ms)) else {
			tracing::debug!(elapsed_ms, "audio: seek_to_ms found no cursor for this elapsed time");
			return false;
		};
		let Some(clip) = self.timeline.clip(cursor.clip) else {
			tracing::debug!(elapsed_ms, clip_index = cursor.clip, "audio: seek_to_ms cursor names a missing clip");
			return false;
		};
		let (source_index, seek_ms) = (clip.source, cursor.seek_ms);
		tracing::debug!(elapsed_ms, source_index, seek_ms, current = ?self.current_source, "audio: seek_to_ms");
		// Already playing (or headed to) the right spot: don't restart it.
		if self.last_seek_target == Some((source_index, seek_ms)) {
			return true;
		}
		if self.current_source == Some(source_index) && self.seek_loaded_source(seek_ms) {
			self.last_seek_target = Some((source_index, seek_ms));
			return true;
		}
		self.load_source(source_index, seek_ms)
	}

	/// The current playback position in the overall document timeline, if a source is
	/// loaded and its position maps onto a known clip.
	#[must_use]
	pub fn elapsed_ms(&self) -> Option<u64> {
		let source = self.current_source?;
		self.timeline.elapsed_for_source_position(source, self.position_ms()?)
	}

	/// Where playback would resume right now, for saving as this document's audio position.
	/// Falls back to a seek that was deferred because it arrived while paused (see
	/// `pending_target_ms`), so browsing a paused book still records where it would pick up.
	/// `None` means nothing has established a position yet, and callers must not treat that
	/// as "the start", since it would wipe a perfectly good stored position.
	#[must_use]
	pub fn resume_point_ms(&self) -> Option<u64> {
		self.pending_target_ms.or_else(|| self.elapsed_ms())
	}

	/// The decoder's current position within the currently loaded source's own file, and that
	/// file's real (decoder-reported) length. `None` with no source loaded yet, or when the
	/// format declares no duration. Distinct from the document's own declared clip duration
	/// for that source, which a plain-audio-zip bundle's placeholder (see
	/// `build_plain_audio_zip_document`) can put far past the file's real end, so callers that
	/// need to know when a seek is about to run off the real end of the file (e.g. "continue
	/// into the next file" seeking) can't get this from the timeline alone.
	#[must_use]
	pub fn current_file_position_and_length_ms(&self) -> Option<(usize, u64, u64)> {
		let source = self.current_source?;
		let length_ms = self.current_length_ms?;
		let raw_ms = self.position_ms()?;
		(length_ms > 0).then_some((source, raw_ms, length_ms))
	}

	/// Where the loaded source is playing, within its own file.
	///
	/// `player.get_pos()` reports how much wall-clock time has actually played since the last
	/// seek (through the WSOLA-wrapped source, at whatever `speed` is now), not how far that
	/// covers into the source's own content — at 2x speed, one played-back second covers two
	/// content seconds. Scaling it by `speed` converts back to a content position, which is
	/// only correct as long as `speed` hasn't changed since the last seek; `set_speed` re-seeks
	/// specifically to keep that true.
	fn position_ms(&self) -> Option<u64> {
		let player = self.player.as_ref()?;
		let played_ms = duration_ms(player.get_pos());
		let content_elapsed_ms = (played_ms as f64 * f64::from(self.speed)) as u64;
		Some(self.position_base_ms.saturating_add(content_elapsed_ms))
	}

	/// Seeks the source that is already loaded, reporting whether that worked. A source that
	/// has played out is past seeking, and a decoder can refuse a seek outright, both of which
	/// leave the caller to load the source afresh instead.
	fn seek_loaded_source(&mut self, seek_ms: u64) -> bool {
		let Some(player) = self.player.as_ref() else {
			return false;
		};
		if player.empty() {
			return false;
		}
		// `seek_ms` is a content position; the live source expects wall-clock ("apparent")
		// position, which is content position divided by speed (see `position_ms`).
		let apparent = Duration::from_millis(seek_ms).div_f32(self.speed);
		if let Err(err) = player.try_seek(apparent) {
			tracing::debug!(seek_ms, error = %err, "audio: in-place seek refused, reloading the source");
			return false;
		}
		// rodio counts from the seek target now, so its position needs no head start added.
		self.position_base_ms = 0;
		if self.playing {
			player.play();
		}
		true
	}

	/// Loads `source_index` and starts it at `seek_ms` into its file.
	fn load_source(&mut self, source_index: usize, seek_ms: u64) -> bool {
		let Some(source) = self.timeline.source(source_index) else {
			return false;
		};
		let location = source.location.clone();
		let path = match resolve_source_path(&location, &self.cache_dir) {
			Ok(path) => path,
			Err(err) => {
				tracing::warn!(source_index, ?location, error = %err, "failed to prepare audio source");
				return false;
			}
		};
		let mut decoder = match open_decoder(&path) {
			Ok(decoder) => decoder,
			Err(err) => {
				tracing::warn!(source_index, path = %path.display(), error = %err, "failed to decode audio source");
				return false;
			}
		};
		let length_ms = decoder.total_duration().map(duration_ms);
		// A target past the real end of the file is a document's declared duration overrunning
		// its own audio (see `current_file_position_and_length_ms`). Land just short of the end
		// instead, so playback moves on to the next source rather than replaying this one.
		let target_ms = match length_ms {
			Some(length_ms) if seek_ms >= length_ms => length_ms.saturating_sub(END_MARGIN_MS),
			_ => seek_ms,
		};
		// Seeking the decoder before it reaches the player keeps the seek off the audio
		// thread, which `Player::try_seek` waits on.
		let applied_seek_ms = match decoder.try_seek(Duration::from_millis(target_ms)) {
			Ok(()) => target_ms,
			Err(err) => {
				tracing::warn!(source_index, seek_ms, target_ms, error = %err, "audio: seek refused, starting from the top");
				0
			}
		};
		tracing::debug!(source_index, path = %path.display(), seek_ms, applied_seek_ms, ?length_ms, "audio: load_source");
		let player = Player::connect_new(self.device.mixer());
		player.append(wsola_speed_source(decoder, self.speed, &self.shared_speed));
		if self.playing {
			player.play();
		} else {
			player.pause();
		}
		self.player = Some(player);
		self.current_source = Some(source_index);
		self.current_length_ms = length_ms;
		self.position_base_ms = applied_seek_ms;
		self.last_seek_target = Some((source_index, seek_ms));
		true
	}
}

/// The output device, opened on demand and shared by every player. Held here by weak
/// reference so the device closes once the last document with narration is gone.
fn shared_device() -> Result<Rc<MixerDeviceSink>> {
	AUDIO_DEVICE.with_borrow_mut(|slot| {
		if let Some(device) = slot.upgrade() {
			return Ok(device);
		}
		let device =
			DeviceSinkBuilder::open_default_sink().context("failed to open the default audio output device")?;
		let device = Rc::new(device);
		*slot = Rc::downgrade(&device);
		Ok(device)
	})
}

fn open_decoder(path: &Path) -> Result<FileDecoder> {
	let file = fs::File::open(path).with_context(|| format!("failed to open '{}'", path.display()))?;
	let byte_len = file.metadata().with_context(|| format!("failed to measure '{}'", path.display()))?.len();
	let mut builder = Decoder::builder().with_data(BufReader::new(file)).with_byte_len(byte_len).with_seekable(true);
	if let Some(extension) = path.extension().and_then(|extension| extension.to_str()) {
		builder = builder.with_hint(extension);
	}
	builder.build().with_context(|| format!("failed to decode '{}'", path.display()))
}

fn duration_ms(duration: Duration) -> u64 {
	u64::try_from(duration.as_millis()).unwrap_or(u64::MAX)
}

/// Wraps `decoder` in a WSOLA time-stretcher started at `initial_speed`, which keeps polling
/// `shared_speed` for the rest of its life so a later `AudioPlayer::set_speed` (changing that
/// shared value) reaches it — `Wsola::set_speed` takes `&mut self`, and once this is handed to
/// `Player::append` it runs on rodio's own mixer thread, out of `AudioPlayer`'s direct reach.
fn wsola_speed_source(
	decoder: FileDecoder,
	initial_speed: f32,
	shared_speed: &Arc<Mutex<f32>>,
) -> impl Source<Item = f32> + Send + 'static {
	let shared_speed = Arc::clone(shared_speed);
	let mut applied_speed = initial_speed;
	decoder.wsola(initial_speed).periodic_access(SPEED_POLL_PERIOD, move |wsola| {
		let target_speed = *shared_speed.lock().unwrap();
		if (target_speed - applied_speed).abs() > f32::EPSILON {
			wsola.set_speed(target_speed);
			applied_speed = target_speed;
		}
	})
}

/// Rounds a speed multiplier to the nearest hundredth, so repeated `increase_speed`/
/// `decrease_speed` steps land on clean values (`1.0`, `1.1`, ...) instead of drifting from
/// `f32` addition error.
fn round_speed(speed: f32) -> f32 {
	(speed * 100.0).round() / 100.0
}

/// Resolves an `AudioLocation` to a real file path the decoder can open. Zip-embedded
/// sources are extracted to `cache_dir` once and reused on subsequent calls.
fn resolve_source_path(location: &AudioLocation, cache_dir: &Path) -> Result<PathBuf> {
	match location {
		AudioLocation::File(path) => Ok(PathBuf::from(path)),
		AudioLocation::ZipEntry { archive, entry, password } => {
			let output_path = cache_dir.join(cache_file_name(archive, entry));
			if !output_path.exists() {
				let file = fs::File::open(archive).with_context(|| format!("failed to open archive '{archive}'"))?;
				let mut zip = ZipArchive::new(BufReader::new(file))
					.with_context(|| format!("failed to read archive '{archive}'"))?;
				extract_zip_entry_to_file_with_password(&mut zip, entry, &output_path, password.as_deref())
					.with_context(|| format!("failed to extract '{entry}' from '{archive}'"))?;
			}
			Ok(output_path)
		}
	}
}

/// A stable, filesystem-safe cache file name for an archive+entry pair, keeping the entry's
/// own extension so the decoder can sniff its format from the file name.
fn cache_file_name(archive: &str, entry: &str) -> String {
	let mut hasher = DefaultHasher::new();
	archive.hash(&mut hasher);
	entry.hash(&mut hasher);
	let ext = Path::new(entry).extension().and_then(|e| e.to_str()).unwrap_or("mp3");
	format!("{:016x}.{ext}", hasher.finish())
}

#[cfg(test)]
mod tests {
	use std::{
		io::{Cursor, Write},
		thread::sleep,
		time::Instant,
	};

	use paperback_core::audio::AudioTimelineBuilder;
	use zip::{ZipWriter, write::FileOptions};

	use super::*;

	fn write_zip(name: &str, data: &[u8]) -> Vec<u8> {
		let mut buf = Vec::new();
		{
			let cursor = Cursor::new(&mut buf);
			let mut writer = ZipWriter::new(cursor);
			writer.start_file(name, FileOptions::<()>::default()).unwrap();
			writer.write_all(data).unwrap();
			writer.finish().unwrap();
		}
		buf
	}

	/// Writes `millis` of silent 16-bit mono PCM, which is enough for the decoder to report a
	/// length, seek within, and play out.
	fn write_wav(path: &Path, millis: u64) {
		const SAMPLE_RATE: u32 = 44100;
		let samples = u32::try_from(u64::from(SAMPLE_RATE) * millis / 1000).unwrap();
		let data_len = samples * 2;
		let mut out = Vec::with_capacity(44 + data_len as usize);
		out.extend_from_slice(b"RIFF");
		out.extend_from_slice(&(36 + data_len).to_le_bytes());
		out.extend_from_slice(b"WAVEfmt ");
		out.extend_from_slice(&16u32.to_le_bytes());
		out.extend_from_slice(&1u16.to_le_bytes());
		out.extend_from_slice(&1u16.to_le_bytes());
		out.extend_from_slice(&SAMPLE_RATE.to_le_bytes());
		out.extend_from_slice(&(SAMPLE_RATE * 2).to_le_bytes());
		out.extend_from_slice(&2u16.to_le_bytes());
		out.extend_from_slice(&16u16.to_le_bytes());
		out.extend_from_slice(b"data");
		out.extend_from_slice(&data_len.to_le_bytes());
		out.resize(44 + data_len as usize, 0);
		fs::write(path, out).unwrap();
	}

	/// Waits up to `timeout_ms` for `check` to pass, pumping the player as the app's timer
	/// does. Reports whether it passed, so a test can say what it was waiting for.
	fn wait_for(player: &mut AudioPlayer, timeout_ms: u64, check: impl Fn(&AudioPlayer) -> bool) -> bool {
		let deadline = Instant::now() + Duration::from_millis(timeout_ms);
		loop {
			player.pump();
			if check(player) {
				return true;
			}
			if Instant::now() >= deadline {
				return false;
			}
			sleep(Duration::from_millis(25));
		}
	}

	/// Plays real audio through the real output device. Skipped where there isn't one, which
	/// is most CI runners.
	#[test]
	fn plays_a_timeline_through_to_the_next_source() {
		if shared_device().is_err() {
			eprintln!("no audio output device: skipping");
			return;
		}
		let dir = env::temp_dir().join("paperback-audio-playback-test");
		fs::create_dir_all(&dir).unwrap();
		let first = dir.join("one.wav");
		let second = dir.join("two.wav");
		write_wav(&first, 1000);
		write_wav(&second, 1000);
		let mut builder = AudioTimelineBuilder::new();
		let one = builder.add_source(AudioLocation::File(first.to_string_lossy().to_string()), Some(1000));
		let two = builder.add_source(AudioLocation::File(second.to_string_lossy().to_string()), Some(1000));
		builder.add_clip(one, 0, 1000, 0, 1);
		builder.add_clip(two, 0, 1000, 1, 2);
		let mut player = AudioPlayer::new(builder.build()).unwrap();

		player.play();
		assert_eq!(player.current_source, Some(one), "playing from a standing start loads the first source");
		assert!(
			wait_for(&mut player, 2000, |player| player.elapsed_ms().is_some_and(|ms| ms >= 100)),
			"position advances while playing"
		);

		player.seek_to_ms(700);
		assert_eq!(player.current_source, Some(one), "a seek within the loaded source doesn't reload it");
		let elapsed = player.elapsed_ms().unwrap();
		assert!((700..1000).contains(&elapsed), "seeking lands where it was asked to, got {elapsed}");

		assert!(
			wait_for(&mut player, 3000, |player| player.current_source == Some(two)),
			"the first source playing out moves on to the second"
		);
		assert!(player.is_playing(), "moving on to the next source keeps playing");

		assert!(
			wait_for(&mut player, 3000, |player| !player.is_playing()),
			"the last source playing out stops playback"
		);
	}

	/// A speed change mid-playback must not make `elapsed_ms` jump: `position_ms` scales
	/// however much wall-clock time the player reports by the *current* speed (see its doc
	/// comment), which is only correct for time played at that speed, so `set_speed` has to
	/// re-seek to re-base it. Written after that re-seek arithmetic came out backwards once
	/// already in development (position jumped instead of staying put).
	#[test]
	fn changing_speed_mid_playback_does_not_jump_the_position() {
		if shared_device().is_err() {
			eprintln!("no audio output device: skipping");
			return;
		}
		let dir = env::temp_dir().join("paperback-audio-speed-test");
		fs::create_dir_all(&dir).unwrap();
		let path = dir.join("long.wav");
		write_wav(&path, 5000);
		let mut builder = AudioTimelineBuilder::new();
		let source = builder.add_source(AudioLocation::File(path.to_string_lossy().to_string()), Some(5000));
		builder.add_clip(source, 0, 5000, 0, 1);
		let mut player = AudioPlayer::new(builder.build()).unwrap();

		player.play();
		assert!(
			wait_for(&mut player, 2000, |player| player.elapsed_ms().is_some_and(|ms| ms >= 300)),
			"position advances while playing at normal speed"
		);
		let before = player.elapsed_ms().unwrap();

		assert!((player.increase_speed() - 1.1).abs() < 1e-6, "steps up to 1.1x");
		let after = player.elapsed_ms().unwrap();
		assert!(
			after.abs_diff(before) < 100,
			"a speed change alone should not move the position, went from {before}ms to {after}ms"
		);

		// At 1.1x, real playback time should now cover more content than it would have at 1.0x.
		assert!(
			wait_for(&mut player, 3000, |player| player.elapsed_ms().is_some_and(|ms| ms >= after + 500)),
			"position keeps advancing after the speed change"
		);
	}

	/// The decoder reports a real length for a file, which is what "continue into the next
	/// file" seeking needs and what a document's own declared duration can't be trusted for.
	#[test]
	fn reports_the_loaded_files_own_length() {
		if shared_device().is_err() {
			eprintln!("no audio output device: skipping");
			return;
		}
		let dir = env::temp_dir().join("paperback-audio-length-test");
		fs::create_dir_all(&dir).unwrap();
		let path = dir.join("one.wav");
		write_wav(&path, 1500);
		let mut builder = AudioTimelineBuilder::new();
		// A placeholder duration hours longer than the real file, as a plain audio zip has.
		let source = builder.add_source(AudioLocation::File(path.to_string_lossy().to_string()), Some(86_400_000));
		builder.add_clip(source, 0, 86_400_000, 0, 1);
		let mut player = AudioPlayer::new(builder.build()).unwrap();
		player.play();
		let (reported_source, _, length_ms) = player.current_file_position_and_length_ms().unwrap();
		assert_eq!(reported_source, source);
		assert!((1400..1600).contains(&length_ms), "reported length should be the file's own, got {length_ms}");
		player.stop();
	}

	#[test]
	fn speed_steps_and_clamps_at_both_ends() {
		if shared_device().is_err() {
			eprintln!("no audio output device: skipping");
			return;
		}
		let mut player = AudioPlayer::new(AudioTimelineBuilder::new().build()).unwrap();
		assert!((player.speed() - 1.0).abs() < f32::EPSILON, "starts at normal speed");

		assert!((player.increase_speed() - 1.1).abs() < 1e-6, "one step up from 1.0 is 1.1");
		assert!((player.decrease_speed() - 1.0).abs() < 1e-6, "one step back down is 1.0 again");

		player.set_speed(MAX_SPEED - 0.05);
		let at_max = player.increase_speed();
		assert!((at_max - MAX_SPEED).abs() < f32::EPSILON, "clamps at the maximum, got {at_max}");
		assert!((player.increase_speed() - MAX_SPEED).abs() < f32::EPSILON, "stays at the maximum");

		player.set_speed(MIN_SPEED + 0.05);
		let at_min = player.decrease_speed();
		assert!((at_min - MIN_SPEED).abs() < f32::EPSILON, "clamps at the minimum, got {at_min}");
		assert!((player.decrease_speed() - MIN_SPEED).abs() < f32::EPSILON, "stays at the minimum");
	}

	#[test]
	fn resolves_a_plain_file_location_directly() {
		let dir = env::temp_dir().join("paperback-audio-player-test");
		fs::create_dir_all(&dir).unwrap();
		let path = dir.join("clip.mp3");
		fs::write(&path, b"fake-mp3-bytes").unwrap();
		let cache_dir = dir.join("cache");
		let resolved =
			resolve_source_path(&AudioLocation::File(path.to_string_lossy().to_string()), &cache_dir).unwrap();
		assert_eq!(resolved, path);
	}

	#[test]
	fn extracts_and_caches_a_zip_entry() {
		let dir = env::temp_dir().join("paperback-audio-player-test");
		fs::create_dir_all(&dir).unwrap();
		let zip_path = dir.join("book.zip");
		fs::write(&zip_path, write_zip("chapter1.mp3", b"chapter-one-bytes")).unwrap();
		let cache_dir = dir.join("cache2");
		let location = AudioLocation::ZipEntry {
			archive: zip_path.to_string_lossy().to_string(),
			entry: "chapter1.mp3".to_string(),
			password: None,
		};
		let resolved = resolve_source_path(&location, &cache_dir).unwrap();
		assert_eq!(fs::read(&resolved).unwrap(), b"chapter-one-bytes");
		// A second resolve reuses the cached file rather than re-extracting.
		let resolved_again = resolve_source_path(&location, &cache_dir).unwrap();
		assert_eq!(resolved, resolved_again);
	}

	#[test]
	fn reports_a_missing_zip_entry() {
		let dir = env::temp_dir().join("paperback-audio-player-test");
		fs::create_dir_all(&dir).unwrap();
		let zip_path = dir.join("book_missing.zip");
		fs::write(&zip_path, write_zip("chapter1.mp3", b"data")).unwrap();
		let cache_dir = dir.join("cache3");
		let location = AudioLocation::ZipEntry {
			archive: zip_path.to_string_lossy().to_string(),
			entry: "missing.mp3".to_string(),
			password: None,
		};
		assert!(resolve_source_path(&location, &cache_dir).is_err());
	}
}
