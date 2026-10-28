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
	document::DocPos,
	util::zip::extract_zip_entry_to_file_with_password,
};
use rodio::{Decoder, DeviceSinkBuilder, MixerDeviceSink, Player, Source};
use rodio_wsola::WsolaSourceExt;
use zip::ZipArchive;

/// Leaves enough of the file to play out, so a seek past the end moves on to the next source instead of replaying this one.
const END_MARGIN_MS: u64 = 100;

const MIN_SPEED: f32 = 0.5;
const MAX_SPEED: f32 = 3.0;
const SPEED_STEP: f32 = 0.1;

const MIN_VOLUME: f32 = 0.1;
/// Above 1.0 amplifies, for recordings made too quietly.
const MAX_VOLUME: f32 = 2.0;
const VOLUME_STEP: f32 = 0.1;

const SPEED_POLL_PERIOD: Duration = Duration::from_millis(20);

type FileDecoder = Decoder<BufReader<fs::File>>;

thread_local! {
	static AUDIO_DEVICE: RefCell<Weak<MixerDeviceSink>> = const { RefCell::new(Weak::new()) };
}

/// Switching source builds a fresh `Player` and drops the old one, since `clear()` and `stop()` both block the UI thread.
pub struct AudioPlayer {
	/// Dropping the last handle closes the output device.
	device: Rc<MixerDeviceSink>,
	player: Option<Player>,
	timeline: AudioTimeline,
	current_source: Option<usize>,
	current_length_ms: Option<u64>,
	/// rodio counts from zero after a decoder seek made before loading, so this holds that head start; a seek through the player makes rodio's count absolute and zeroes it.
	position_base_ms: u64,
	playing: bool,
	/// Applied on resume, since applying it eagerly would decode a fresh source per keystroke while navigating with audio nobody hears.
	pending_target_ms: Option<u64>,
	/// So navigation resolving to the same spot doesn't restart audio that's already right.
	last_seek_target: Option<(usize, u64)>,
	cache_dir: PathBuf,
	/// Played through WSOLA rather than `Player::set_speed`, which raises the pitch instead of stretching time.
	speed: f32,
	/// `Wsola::set_speed` takes `&mut self` and the source lives on rodio's mixer thread once appended, so it polls this instead.
	shared_speed: Arc<Mutex<f32>>,
	volume: f32,
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
			volume: 1.0,
		})
	}

	#[must_use]
	pub const fn timeline(&self) -> &AudioTimeline {
		&self.timeline
	}

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

	#[must_use]
	pub const fn speed(&self) -> f32 {
		self.speed
	}

	/// Re-seeks the live source to where it already was, because `position_ms` scaling by `speed` is only right for time played at the current speed.
	pub fn set_speed(&mut self, speed: f32) -> f32 {
		let new_speed = round_to_hundredth(speed.clamp(MIN_SPEED, MAX_SPEED));
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

	pub fn increase_speed(&mut self) -> f32 {
		self.set_speed(self.speed + SPEED_STEP)
	}

	pub fn decrease_speed(&mut self) -> f32 {
		self.set_speed(self.speed - SPEED_STEP)
	}

	#[must_use]
	pub const fn volume(&self) -> f32 {
		self.volume
	}

	pub fn set_volume(&mut self, volume: f32) -> f32 {
		self.volume = round_to_hundredth(volume.clamp(MIN_VOLUME, MAX_VOLUME));
		if let Some(player) = self.player.as_ref() {
			player.set_volume(self.volume);
		}
		self.volume
	}

	pub fn increase_volume(&mut self) -> f32 {
		self.set_volume(self.volume + VOLUME_STEP)
	}

	pub fn decrease_volume(&mut self) -> f32 {
		self.set_volume(self.volume - VOLUME_STEP)
	}

	/// rodio has no "finished" callback, so the caret-sync timer drives this.
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

	pub fn seek_to_position(&mut self, position: DocPos) -> bool {
		let target = self.timeline.point_for_position(position);
		tracing::debug!(%position, ?target, "audio: seek_to_position");
		target.is_some_and(|point| self.seek_to_ms(point.time_ms))
	}

	pub fn seek_to_ms(&mut self, elapsed_ms: u64) -> bool {
		if !self.playing {
			tracing::debug!(elapsed_ms, "audio: seek_to_ms while paused, deferring");
			self.pending_target_ms = Some(elapsed_ms);
			return true;
		}
		let Some(cursor) = self.timeline.resolve(TimelinePoint::new(DocPos::ZERO, elapsed_ms)) else {
			tracing::debug!(elapsed_ms, "audio: seek_to_ms found no cursor for this elapsed time");
			return false;
		};
		let Some(clip) = self.timeline.clip(cursor.clip) else {
			tracing::debug!(elapsed_ms, clip_index = cursor.clip, "audio: seek_to_ms cursor names a missing clip");
			return false;
		};
		let (source_index, seek_ms) = (clip.source, cursor.seek_ms);
		tracing::debug!(elapsed_ms, source_index, seek_ms, current = ?self.current_source, "audio: seek_to_ms");
		if self.last_seek_target == Some((source_index, seek_ms)) {
			return true;
		}
		if self.current_source == Some(source_index) && self.seek_loaded_source(seek_ms) {
			self.last_seek_target = Some((source_index, seek_ms));
			return true;
		}
		self.load_source(source_index, seek_ms)
	}

	#[must_use]
	pub fn elapsed_ms(&self) -> Option<u64> {
		let source = self.current_source?;
		self.timeline.elapsed_for_source_position(source, self.position_ms()?)
	}

	/// `None` means no position is established yet; callers must not treat it as the start, or a good stored position gets wiped.
	#[must_use]
	pub fn resume_point_ms(&self) -> Option<u64> {
		self.pending_target_ms.or_else(|| self.elapsed_ms())
	}

	/// The decoder's real file length, since a plain audio zip's declared clip duration is a placeholder far past the file's end.
	#[must_use]
	pub fn current_file_position_and_length_ms(&self) -> Option<(usize, u64, u64)> {
		let source = self.current_source?;
		let length_ms = self.current_length_ms?;
		let raw_ms = self.position_ms()?;
		(length_ms > 0).then_some((source, raw_ms, length_ms))
	}

	/// `get_pos()` is wall-clock time played since the last seek, so it's scaled by `speed`; only valid while `speed` is unchanged since that seek.
	fn position_ms(&self) -> Option<u64> {
		let player = self.player.as_ref()?;
		let played_ms = duration_ms(player.get_pos());
		#[allow(
			clippy::cast_possible_truncation,
			clippy::cast_sign_loss,
			reason = "a play position in milliseconds, never negative, and a float cast saturates"
		)]
		let content_elapsed_ms = (played_ms as f64 * f64::from(self.speed)) as u64;
		Some(self.position_base_ms.saturating_add(content_elapsed_ms))
	}

	fn seek_loaded_source(&mut self, seek_ms: u64) -> bool {
		let Some(player) = self.player.as_ref() else {
			return false;
		};
		if player.empty() {
			return false;
		}
		// The live source counts wall-clock time, which is content time divided by speed.
		let apparent = Duration::from_millis(seek_ms).div_f32(self.speed);
		if let Err(err) = player.try_seek(apparent) {
			tracing::debug!(seek_ms, error = %err, "audio: in-place seek refused, reloading the source");
			return false;
		}
		self.position_base_ms = 0;
		if self.playing {
			player.play();
		}
		true
	}

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
		// A declared duration can overrun the file's real audio.
		let target_ms = match length_ms {
			Some(length_ms) if seek_ms >= length_ms => length_ms.saturating_sub(END_MARGIN_MS),
			_ => seek_ms,
		};
		// Seeking the decoder before the player gets it avoids `Player::try_seek` waiting on the audio thread.
		let applied_seek_ms = match decoder.try_seek(Duration::from_millis(target_ms)) {
			Ok(()) => target_ms,
			Err(err) => {
				tracing::warn!(source_index, seek_ms, target_ms, error = %err, "audio: seek refused, starting from the top");
				0
			}
		};
		tracing::debug!(source_index, path = %path.display(), seek_ms, applied_seek_ms, ?length_ms, "audio: load_source");
		let player = Player::connect_new(self.device.mixer());
		player.set_volume(self.volume);
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

/// Held weakly so the device closes once the last document with narration is gone.
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
	// So a constant-bitrate MP3 with no duration header still reports `total_duration()`.
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

/// So repeated steps land on clean values instead of drifting from `f32` addition error.
fn round_to_hundredth(value: f32) -> f32 {
	(value * 100.0).round() / 100.0
}

/// `Decoder` needs a seekable file, so zip-embedded sources are extracted to the cache.
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

/// Keeps the entry's extension so the decoder can sniff the format from the name.
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

	/// Asked once, because on Windows with no sound hardware (every CI runner) opening a device again after a failure crashes the test process.
	fn has_audio_device() -> bool {
		static AVAILABLE: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
		*AVAILABLE.get_or_init(|| shared_device().is_ok())
	}

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

	#[test]
	fn plays_a_timeline_through_to_the_next_source() {
		if !has_audio_device() {
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
		builder.add_clip(one, 0, 1000, DocPos::from_raw(0), DocPos::from_raw(1));
		builder.add_clip(two, 0, 1000, DocPos::from_raw(1), DocPos::from_raw(2));
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

	/// The re-seek arithmetic in `set_speed` once came out backwards and made the position jump.
	#[test]
	fn changing_speed_mid_playback_does_not_jump_the_position() {
		if !has_audio_device() {
			eprintln!("no audio output device: skipping");
			return;
		}
		let dir = env::temp_dir().join("paperback-audio-speed-test");
		fs::create_dir_all(&dir).unwrap();
		let path = dir.join("long.wav");
		write_wav(&path, 5000);
		let mut builder = AudioTimelineBuilder::new();
		let source = builder.add_source(AudioLocation::File(path.to_string_lossy().to_string()), Some(5000));
		builder.add_clip(source, 0, 5000, DocPos::from_raw(0), DocPos::from_raw(1));
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

		assert!(
			wait_for(&mut player, 3000, |player| player.elapsed_ms().is_some_and(|ms| ms >= after + 500)),
			"position keeps advancing after the speed change"
		);
	}

	#[test]
	fn reports_the_loaded_files_own_length() {
		if !has_audio_device() {
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
		builder.add_clip(source, 0, 86_400_000, DocPos::from_raw(0), DocPos::from_raw(1));
		let mut player = AudioPlayer::new(builder.build()).unwrap();
		player.play();
		let (reported_source, _, length_ms) = player.current_file_position_and_length_ms().unwrap();
		assert_eq!(reported_source, source);
		assert!((1400..1600).contains(&length_ms), "reported length should be the file's own, got {length_ms}");
		player.stop();
	}

	#[test]
	fn speed_steps_and_clamps_at_both_ends() {
		if !has_audio_device() {
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
	fn volume_steps_and_clamps_at_both_ends() {
		if !has_audio_device() {
			eprintln!("no audio output device: skipping");
			return;
		}
		let mut player = AudioPlayer::new(AudioTimelineBuilder::new().build()).unwrap();
		assert!((player.volume() - 1.0).abs() < f32::EPSILON, "starts as recorded");
		assert!((player.increase_volume() - 1.1).abs() < 1e-6, "one step up from 1.0 is 1.1");
		assert!((player.decrease_volume() - 1.0).abs() < 1e-6, "one step back down is 1.0 again");
		player.set_volume(MAX_VOLUME - 0.05);
		assert!((player.increase_volume() - MAX_VOLUME).abs() < f32::EPSILON, "clamps at the maximum");
		assert!((player.increase_volume() - MAX_VOLUME).abs() < f32::EPSILON, "stays at the maximum");
		player.set_volume(MIN_VOLUME + 0.05);
		assert!((player.decrease_volume() - MIN_VOLUME).abs() < f32::EPSILON, "clamps at the minimum");
		assert!((player.decrease_volume() - MIN_VOLUME).abs() < f32::EPSILON, "stays at the minimum");
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
