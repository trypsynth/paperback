use std::{
	cell::RefCell,
	collections::hash_map::DefaultHasher,
	fs,
	hash::{Hash, Hasher},
	io::BufReader,
	path::{Path, PathBuf},
	rc::Rc,
};

use anyhow::{Context, Result};
use paperback_core::{
	audio::{AudioLocation, AudioTimeline, TimelinePoint},
	util::zip::extract_zip_entry_to_file,
};
#[cfg(target_os = "windows")]
use wxdragon::accessible::AccRole;
use wxdragon::{
	prelude::*,
	widgets::media_ctrl::{MediaCtrlStyle, SeekMode},
};
use zip::ZipArchive;

struct PlayerState {
	timeline: AudioTimeline,
	current_source: Option<usize>,
	playing: bool,
	/// Where to seek once the in-flight `Load()` finishes; `Load()` is asynchronous, so the
	/// seek has to wait for `MediaCtrlEvent::Loaded`.
	pending_seek_ms: Option<u64>,
	/// Whether a `Load()` is in flight, waiting on its `Loaded` event. Overlapping `Load()`
	/// calls race the backend — the `Loaded` event can't be tied back to either call — so a
	/// new source request queues in `pending_load_target` instead.
	load_in_flight: bool,
	/// The most recent source request that arrived mid-load, acted on once the in-flight
	/// load resolves, so rapid-fire navigation converges on wherever the reader ended up.
	pending_load_target: Option<(usize, u64)>,
	/// A seek requested while paused, applied lazily on resume. Applying it eagerly would
	/// drive a full backend load per keystroke while navigating with audio nobody is hearing.
	pending_target_ms: Option<u64>,
	/// `(source, seek_ms)` of the most recent seek issued to (or in flight toward) the
	/// backend, so navigation resolving to the same pair doesn't restart correct audio.
	last_seek_target: Option<(usize, u64)>,
	/// Where extracted zip-embedded sources are cached, keyed by archive+entry.
	cache_dir: PathBuf,
}

/// Plays a DAISY audiobook's narration against its `AudioTimeline` via a hidden
/// `wxMediaCtrl`, so decoding and seeking go through the native OS media backend.
/// `MediaCtrl` only plays real file paths, so zip-embedded sources are extracted on use.
pub struct AudioPlayer {
	media: MediaCtrl,
	state: Rc<RefCell<PlayerState>>,
}

impl AudioPlayer {
	pub fn new(parent: &Panel, timeline: AudioTimeline) -> Result<Self> {
		let cache_dir = std::env::temp_dir().join("paperback-audio-cache");
		fs::create_dir_all(&cache_dir).context("failed to create audio cache directory")?;
		// Hidden and unfocusable so it never surfaces to a screen reader, but deliberately
		// *not* zero-sized: some Windows backends build an internal renderer window sized to
		// the control, and a 0x0 one can leave `Load()` in flight with no `Loaded` event.
		//
		// WMP10 is requested explicitly rather than letting wxMediaCtrl fall back to its
		// default "AM" backend, which never fires `MEDIA_LOADED` for audio-only files on
		// modern Windows and so wedges every load after the first.
		let media = MediaCtrl::builder(parent)
			.with_style(MediaCtrlStyle::NoAutoResize)
			.with_backend_name("wxWMP10MediaBackend")
			.build();
		media.hide();
		media.set_can_focus(false);
		#[cfg(target_os = "windows")]
		media.set_accessibility_role(AccRole::None);
		let state = Rc::new(RefCell::new(PlayerState {
			timeline,
			current_source: None,
			playing: false,
			pending_seek_ms: None,
			load_in_flight: false,
			pending_load_target: None,
			pending_target_ms: None,
			last_seek_target: None,
			cache_dir,
		}));

		let loaded_media = media;
		let loaded_state = Rc::clone(&state);
		media.on_loaded(move |_| {
			let (seek_ms, playing, next_target, settled_source) = {
				let mut state = loaded_state.borrow_mut();
				state.load_in_flight = false;
				(state.pending_seek_ms.take(), state.playing, state.pending_load_target.take(), state.current_source)
			};
			tracing::debug!(?seek_ms, playing, ?next_target, ?settled_source, "audio: on_loaded");
			// A newer request arrived while this load was in flight: go straight to it
			// instead of settling into the source that just happened to finish loading.
			if let Some((source_index, seek_ms)) = next_target {
				if Some(source_index) == settled_source {
					apply_seek(loaded_media, &loaded_state, source_index, seek_ms, playing);
				} else {
					start_load(loaded_media, &loaded_state, source_index, seek_ms);
				}
				return;
			}
			if let (Some(seek_ms), Some(source_index)) = (seek_ms, settled_source) {
				apply_seek(loaded_media, &loaded_state, source_index, seek_ms, playing);
			} else if playing {
				loaded_media.play();
			} else {
				loaded_media.pause();
			}
		});

		let finished_media = media;
		let finished_state = Rc::clone(&state);
		media.on_finished(move |_| {
			let next = {
				let state = finished_state.borrow();
				state.current_source.and_then(|current| state.timeline.next_source_after(current))
			};
			tracing::debug!(?next, "audio: on_finished");
			match next {
				Some(next) => {
					request_source(finished_media, &finished_state, next, 0);
				}
				None => finished_state.borrow_mut().playing = false,
			}
		});

		Ok(Self { media, state })
	}

	#[must_use]
	pub fn timeline(&self) -> std::cell::Ref<'_, AudioTimeline> {
		std::cell::Ref::map(self.state.borrow(), |state| &state.timeline)
	}

	/// Stops playback and releases the native media session, ahead of this player (and the
	/// window it's parented to) being torn down.
	pub fn stop(&mut self) {
		self.state.borrow_mut().playing = false;
		self.media.stop();
	}

	#[must_use]
	pub fn is_playing(&self) -> bool {
		self.state.borrow().playing
	}

	pub fn play(&mut self) {
		let (pending_target, has_source) = {
			let mut state = self.state.borrow_mut();
			state.playing = true;
			(state.pending_target_ms.take(), state.current_source.is_some())
		};
		if let Some(target) = pending_target {
			self.seek_to_ms(target);
		} else if has_source {
			self.media.play();
		} else {
			self.seek_to_ms(0);
		}
	}

	pub fn pause(&mut self) {
		self.state.borrow_mut().playing = false;
		self.media.pause();
	}

	pub fn toggle(&mut self) {
		if self.is_playing() {
			self.pause();
		} else {
			self.play();
		}
	}

	/// Seeks playback to the point covering `position` in the text, if the timeline
	/// narrates it. Leaves the transport running or paused as it already was.
	pub fn seek_to_position(&mut self, position: usize) -> bool {
		let target = self.state.borrow().timeline.point_for_position(position);
		tracing::debug!(position, ?target, "audio: seek_to_position");
		target.is_some_and(|point| self.seek_to_ms(point.time_ms))
	}

	/// Seeks playback to `elapsed_ms` into the overall document timeline. While paused this
	/// only records the target (see `pending_target_ms`), applying it lazily on resume.
	pub fn seek_to_ms(&mut self, elapsed_ms: u64) -> bool {
		if !self.is_playing() {
			tracing::debug!(elapsed_ms, "audio: seek_to_ms while paused, deferring");
			self.state.borrow_mut().pending_target_ms = Some(elapsed_ms);
			return true;
		}
		let (source_index, seek_ms, current_source, load_in_flight, already_there) = {
			let state = self.state.borrow();
			let Some(cursor) = state.timeline.resolve(TimelinePoint::new(0, elapsed_ms)) else {
				tracing::debug!(elapsed_ms, "audio: seek_to_ms found no cursor for this elapsed time");
				return false;
			};
			let Some(clip) = state.timeline.clip(cursor.clip) else {
				tracing::debug!(elapsed_ms, clip_index = cursor.clip, "audio: seek_to_ms cursor names a missing clip");
				return false;
			};
			let already_there = state.last_seek_target == Some((clip.source, cursor.seek_ms));
			(clip.source, cursor.seek_ms, state.current_source, state.load_in_flight, already_there)
		};
		tracing::debug!(
			elapsed_ms,
			source_index,
			seek_ms,
			?current_source,
			load_in_flight,
			already_there,
			"audio: seek_to_ms while playing"
		);
		// Already playing (or headed to) the right spot: don't restart it.
		if already_there {
			return true;
		}
		// A load in flight means the control isn't ready for a `Seek()` even when the source
		// matches, so fall through to `request_source` and let the `Loaded` handler apply it.
		if current_source == Some(source_index) && !load_in_flight {
			apply_seek(self.media, &self.state, source_index, seek_ms, self.is_playing());
			true
		} else {
			request_source(self.media, &self.state, source_index, seek_ms)
		}
	}

	/// The current playback position in the overall document timeline, if a source is
	/// loaded and its position maps onto a known clip.
	#[must_use]
	pub fn elapsed_ms(&self) -> Option<u64> {
		let state = self.state.borrow();
		let source = state.current_source?;
		// Mid-load, `current_source` is already the new source but the control isn't, so
		// `tell()` still reports the previous source's stale position. Report where playback
		// is headed instead — this matters at chapter boundaries, which are source switches.
		let raw_ms = if state.load_in_flight {
			state.pending_seek_ms.unwrap_or(0)
		} else {
			u64::try_from(self.media.tell().max(0)).unwrap_or(0)
		};
		state.timeline.elapsed_for_source_position(source, raw_ms)
	}

	/// Where playback would resume right now, for saving as this document's audio position.
	/// Falls back to a seek that was deferred because it arrived while paused (see
	/// `pending_target_ms`), so browsing a paused book still records where it would pick up.
	/// `None` means nothing has established a position yet, and callers must not treat that
	/// as "the start" — it would wipe a perfectly good stored position.
	#[must_use]
	pub fn resume_point_ms(&self) -> Option<u64> {
		self.elapsed_ms().or_else(|| self.state.borrow().pending_target_ms)
	}
}

fn apply_seek(media: MediaCtrl, state: &Rc<RefCell<PlayerState>>, source_index: usize, seek_ms: u64, playing: bool) {
	state.borrow_mut().last_seek_target = Some((source_index, seek_ms));
	let native_seek_ms = native_seek_target_ms(&media, seek_ms);
	let seek_result = media.seek(i64::try_from(native_seek_ms).unwrap_or(0), SeekMode::FromStart);
	tracing::debug!(seek_ms, native_seek_ms, playing, seek_result, "audio: apply_seek");
	if playing {
		media.play()
	} else {
		media.pause()
	};
}

/// `wxWMP10MediaBackend::SetPosition` — the only Windows backend that reliably plays
/// audio-only DAISY sources, see `AudioPlayer::new` — subtracts a full video frame's
/// worth of time (`1000 / playback_rate` ms) from every seek target before applying it.
/// It's a workaround upstream added so video controls redraw the correct frame after a
/// seek (`src/msw/mediactrl_wmp10.cpp`, `SetPosition`), fired unconditionally even for
/// audio-only media. Left uncompensated, every jump lands about a second before the
/// intended clip — audible as the tail of the *previous* line instead of the one just
/// navigated to (e.g. hearing "...District Twelve" instead of "End of Book Two"). Adding
/// the same amount back before handing the target to the backend cancels it out. Other
/// platforms' backends don't carry this bug, so the compensation is Windows-only.
#[cfg(target_os = "windows")]
fn native_seek_target_ms(media: &MediaCtrl, seek_ms: u64) -> u64 {
	let rate = media.get_playback_rate();
	let bias_ms = if rate > 0.0 { (1000.0 / rate).round() as u64 } else { 1000 };
	let compensated = seek_ms.saturating_add(bias_ms);
	let length_ms = u64::try_from(media.length().max(0)).unwrap_or(0);
	if length_ms > 0 { compensated.min(length_ms) } else { compensated }
}

#[cfg(not(target_os = "windows"))]
fn native_seek_target_ms(_media: &MediaCtrl, seek_ms: u64) -> u64 {
	seek_ms
}

/// Entry point for switching to `source_index`: starts loading it immediately, unless a
/// load is already in flight, in which case this becomes the target picked up once that
/// one's `Loaded` event fires — see `PlayerState::pending_load_target`.
fn request_source(media: MediaCtrl, state: &Rc<RefCell<PlayerState>>, source_index: usize, seek_ms: u64) -> bool {
	if state.borrow().load_in_flight {
		tracing::debug!(source_index, seek_ms, "audio: request_source queued behind in-flight load");
		state.borrow_mut().pending_load_target = Some((source_index, seek_ms));
		return true;
	}
	start_load(media, state, source_index, seek_ms)
}

fn start_load(media: MediaCtrl, state: &Rc<RefCell<PlayerState>>, source_index: usize, seek_ms: u64) -> bool {
	let (location, cache_dir) = {
		let state = state.borrow();
		let Some(source) = state.timeline.source(source_index) else { return false };
		(source.location.clone(), state.cache_dir.clone())
	};
	let path = match resolve_source_path(&location, &cache_dir) {
		Ok(path) => path,
		Err(err) => {
			tracing::warn!(source_index, location = ?location, error = %err, "failed to prepare audio source");
			return false;
		}
	};
	// Some backends (Media Foundation especially) won't reliably accept a new `Load()`, and
	// may never fire `Loaded`, while a previous source is still playing.
	media.stop();
	tracing::debug!(source_index, path = %path.display(), seek_ms, "audio: start_load calling media.load()");
	if !media.load(&path.to_string_lossy()) {
		tracing::warn!(source_index, path = %path.display(), "media control refused to load audio source");
		return false;
	}
	let mut state = state.borrow_mut();
	state.current_source = Some(source_index);
	state.pending_seek_ms = Some(seek_ms);
	state.load_in_flight = true;
	state.last_seek_target = Some((source_index, seek_ms));
	true
}

/// Resolves an `AudioLocation` to a real file path `MediaCtrl` can load. Zip-embedded
/// sources are extracted to `cache_dir` once and reused on subsequent calls.
fn resolve_source_path(location: &AudioLocation, cache_dir: &Path) -> Result<PathBuf> {
	match location {
		AudioLocation::File(path) => Ok(PathBuf::from(path)),
		AudioLocation::ZipEntry { archive, entry } => {
			let output_path = cache_dir.join(cache_file_name(archive, entry));
			if !output_path.exists() {
				let file = fs::File::open(archive).with_context(|| format!("failed to open archive '{archive}'"))?;
				let mut zip = ZipArchive::new(BufReader::new(file))
					.with_context(|| format!("failed to read archive '{archive}'"))?;
				extract_zip_entry_to_file(&mut zip, entry, &output_path)
					.with_context(|| format!("failed to extract '{entry}' from '{archive}'"))?;
			}
			Ok(output_path)
		}
	}
}

/// A stable, filesystem-safe cache file name for an archive+entry pair, keeping the entry's
/// own extension so the media backend can sniff its format from the file name.
fn cache_file_name(archive: &str, entry: &str) -> String {
	let mut hasher = DefaultHasher::new();
	archive.hash(&mut hasher);
	entry.hash(&mut hasher);
	let ext = Path::new(entry).extension().and_then(|e| e.to_str()).unwrap_or("mp3");
	format!("{:016x}.{ext}", hasher.finish())
}

#[cfg(test)]
mod tests {
	use std::io::Write;

	use super::*;

	fn write_zip(name: &str, data: &[u8]) -> Vec<u8> {
		use zip::{ZipWriter, write::FileOptions};

		let mut buf = Vec::new();
		{
			let cursor = std::io::Cursor::new(&mut buf);
			let mut writer = ZipWriter::new(cursor);
			writer.start_file(name, FileOptions::<()>::default()).unwrap();
			writer.write_all(data).unwrap();
			writer.finish().unwrap();
		}
		buf
	}

	#[test]
	fn resolves_a_plain_file_location_directly() {
		let dir = std::env::temp_dir().join("paperback-audio-player-test");
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
		let dir = std::env::temp_dir().join("paperback-audio-player-test");
		fs::create_dir_all(&dir).unwrap();
		let zip_path = dir.join("book.zip");
		fs::write(&zip_path, write_zip("chapter1.mp3", b"chapter-one-bytes")).unwrap();
		let cache_dir = dir.join("cache2");
		let location = AudioLocation::ZipEntry {
			archive: zip_path.to_string_lossy().to_string(),
			entry: "chapter1.mp3".to_string(),
		};
		let resolved = resolve_source_path(&location, &cache_dir).unwrap();
		assert_eq!(fs::read(&resolved).unwrap(), b"chapter-one-bytes");
		// A second resolve reuses the cached file rather than re-extracting.
		let resolved_again = resolve_source_path(&location, &cache_dir).unwrap();
		assert_eq!(resolved, resolved_again);
	}

	#[test]
	fn reports_a_missing_zip_entry() {
		let dir = std::env::temp_dir().join("paperback-audio-player-test");
		fs::create_dir_all(&dir).unwrap();
		let zip_path = dir.join("book_missing.zip");
		fs::write(&zip_path, write_zip("chapter1.mp3", b"data")).unwrap();
		let cache_dir = dir.join("cache3");
		let location = AudioLocation::ZipEntry {
			archive: zip_path.to_string_lossy().to_string(),
			entry: "missing.mp3".to_string(),
		};
		assert!(resolve_source_path(&location, &cache_dir).is_err());
	}
}
