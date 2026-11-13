use paperback_core::config::DocumentSpeech;
use patois::t;

use super::DocumentManager;
use crate::{
	read_aloud::{Followed, RATE_STEP, Speaker},
	ui::navigation::set_caret_to_doc_offset,
};

impl DocumentManager {
	/// Starts reading the active document aloud from the caret, or pauses or resumes it.
	///
	/// # Errors
	///
	/// Returns a message to announce when read aloud can't run on this system.
	pub fn toggle_read_aloud(&mut self) -> Result<(), String> {
		let Some(index) = self.active_tab_index() else { return Ok(()) };
		let path = self.tabs[index].file_path.clone();
		if self.speaker_tab.as_ref() == Some(&path)
			&& let Some(speaker) = self.speaker.as_mut()
			&& speaker.is_reading()
		{
			// Resuming after a move reading couldn't start at yet goes on from the caret.
			if let Some(moved_to) = speaker.pending_move()
				&& speaker.is_paused()
			{
				let tab = &self.tabs[index];
				speaker.read_from_caret(
					&tab.session,
					moved_to,
					&self.config.lock().unwrap(),
					&path.to_string_lossy(),
					false,
				);
			} else if speaker.is_paused() {
				speaker.resume();
			} else {
				speaker.pause();
			}
			return Ok(());
		}
		self.speaker()?;
		let tab = &self.tabs[index];
		let position = tab.window.to_doc(tab.text_ctrl.get_insertion_point());
		let started = self.speaker.as_mut().is_some_and(|speaker| {
			speaker.start(&tab.session, position, &self.config.lock().unwrap(), &path.to_string_lossy())
		});
		if !started {
			// TRANSLATORS: Announced when read aloud is started with nothing left to read after the caret
			return Err(t("End of document"));
		}
		self.speaker_tab = Some(path);
		Ok(())
	}

	#[cfg(target_os = "windows")]
	/// Plays or pauses the active document's audio or reading as asked, for a headset button, which says which it means. A toggle would start a book that something else had already paused, or pause one the button meant to start.
	///
	/// # Errors
	///
	/// Returns a message to announce when read aloud can't run on this system.
	pub fn set_media_playing(&mut self, playing: bool) -> Result<(), String> {
		if let Some(player) = self.active_tab_mut().and_then(|tab| tab.audio_player.as_mut()) {
			if playing {
				player.play();
			} else {
				player.pause();
			}
			return Ok(());
		}
		if self.reading_tab_index().is_some() {
			if playing {
				self.resume_read_aloud();
			} else {
				self.pause_read_aloud();
			}
			return Ok(());
		}
		// Starting to read locks the config, which a dialog open may be holding.
		if !playing || self.config.try_lock().is_err() {
			return Ok(());
		}
		self.toggle_read_aloud()
	}

	/// What Windows is told is playing. Idle until the active document has played something, so that merely having a recording open doesn't take the headset buttons from the music player.
	#[cfg(target_os = "windows")]
	pub(super) fn media_playback_state(&self) -> crate::windows_audio::PlaybackState {
		use crate::windows_audio::PlaybackState;
		let Some(index) = self.active_tab_index() else { return PlaybackState::Idle };
		if let Some(player) = self.tabs[index].audio_player.as_ref() {
			return match (player.is_playing(), player.resume_point_ms()) {
				(true, _) => PlaybackState::Playing,
				(false, Some(_)) => PlaybackState::Paused,
				(false, None) => PlaybackState::Idle,
			};
		}
		match self.speaker.as_ref() {
			Some(speaker) if self.reading_tab_index().is_some() => {
				if speaker.is_paused() {
					PlaybackState::Paused
				} else {
					PlaybackState::Playing
				}
			}
			_ => PlaybackState::Idle,
		}
	}

	/// Moves reading to the paragraph before or after the one playing. `None` when the active document isn't being read; `Some(false)` at either end of it.
	pub fn skip_read_aloud(&mut self, forward: bool) -> Option<bool> {
		let index = self.reading_tab_index()?;
		let tab = &self.tabs[index];
		let speaker = self.speaker.as_mut()?;
		Some(speaker.skip(&tab.session, forward, &self.config.lock().unwrap(), &tab.file_path.to_string_lossy()))
	}

	/// Changes the speech rate: the book's own, if it has one, else the app's. Returns the new rate, and whether it was already at its limit, or a message when read aloud can't run here.
	pub fn change_read_aloud_rate(&mut self, increase: bool) -> Result<(i32, bool), String> {
		let Some(index) = self.active_tab_index() else { return Err(String::new()) };
		let rate_key = self.speaker()?.rate_key();
		let path = self.tabs[index].file_path.to_string_lossy().to_string();
		let step = if increase { RATE_STEP } else { -RATE_STEP };
		let (before, after) = {
			let config = self.config.lock().unwrap();
			let book = config.get_document_speech(&path);
			let before = book.rate_percent.unwrap_or_else(|| config.get_app_int(&rate_key, 50));
			let after = (before + step).clamp(0, 100);
			if book.rate_percent.is_some() {
				config.set_document_speech(&path, &DocumentSpeech { rate_percent: Some(after), ..book });
			} else {
				config.set_app_int(&rate_key, after);
			}
			(before, after)
		};
		if let Some(reading) = self.reading_tab_index() {
			let tab = &self.tabs[reading];
			if let Some(speaker) = self.speaker.as_mut() {
				speaker.restart(&tab.session, &self.config.lock().unwrap(), &tab.file_path.to_string_lossy());
			}
		}
		Ok((after, before == after))
	}

	/// Returns the new volume and whether it was already at its limit, or `None` when nothing is being read.
	pub fn change_read_aloud_volume(&mut self, increase: bool) -> Option<(f32, bool)> {
		self.reading_tab_index()?;
		self.speaker.as_mut().map(|speaker| speaker.change_volume(increase))
	}

	/// Switches read aloud to the engine called `id`, stopping any reading.
	pub fn set_speech_engine(&mut self, id: &str) {
		if let Ok(speaker) = self.speaker()
			&& let Err(error) = speaker.set_engine(id)
		{
			tracing::warn!(%error, engine = id, "read aloud: couldn't switch engine");
		}
	}

	/// Pauses reading if it's playing, and returns whether it was, so it can be resumed after.
	pub fn pause_read_aloud(&mut self) -> bool {
		if self.reading_tab_index().is_none() {
			return false;
		}
		let Some(speaker) = self.speaker.as_mut().filter(|speaker| !speaker.is_paused()) else { return false };
		speaker.pause();
		true
	}

	pub fn resume_read_aloud(&mut self) {
		if self.reading_tab_index().is_some()
			&& let Some(speaker) = self.speaker.as_mut()
		{
			speaker.resume();
		}
	}

	/// Speaks Settings' sample with the engine, voice, rate and pitch it shows. `configured_engine` is the one in the config, which a speaker made here starts with: Settings holds the config while it's open, so this can't read it.
	pub fn play_speech_sample(
		&mut self,
		configured_engine: &str,
		engine: &str,
		voice: Option<String>,
		rate: i32,
		pitch: i32,
	) {
		let percent = |value: i32| u8::try_from(value.clamp(0, 100)).unwrap_or(50);
		if self.speaker.is_none() {
			match Speaker::new(configured_engine) {
				Ok(speaker) => self.speaker = Some(speaker),
				Err(error) => {
					tracing::warn!(%error, "read aloud isn't available");
					return;
				}
			}
		}
		let Some(speaker) = self.speaker.as_mut() else { return };
		// TRANSLATORS: Spoken by the Play Sample button on the Audio tab of Settings
		let text = t("This is a sample of the current voice and speed settings.");
		if let Err(error) = speaker.play_sample(engine, &text, voice, percent(rate), percent(pitch)) {
			tracing::warn!(%error, "read aloud: couldn't play the sample");
		}
	}

	pub fn stop_speech_sample(&mut self) {
		if let Some(speaker) = self.speaker.as_mut() {
			speaker.stop_sample();
		}
	}

	/// Starts the paragraph being read again, so changed settings are heard from there. Paused reading stays paused.
	pub fn restart_read_aloud(&mut self) {
		let Some(index) = self.reading_tab_index() else { return };
		let tab = &self.tabs[index];
		if let Some(speaker) = self.speaker.as_mut() {
			speaker.restart(&tab.session, &self.config.lock().unwrap(), &tab.file_path.to_string_lossy());
		}
	}

	/// Moves the caret to each word as it's spoken, and past the last paragraph once it's read, so starting again doesn't repeat it. When the user moves the caret while reading, reading goes on from there. Called on its own short timer.
	pub fn pump_read_aloud(&mut self) {
		// Not `reading_tab_index`, since that stops counting the document once reading has finished, and finishing is what this has to notice.
		let Some(index) =
			self.active_tab_index().filter(|&index| self.speaker_tab.as_ref() == Some(&self.tabs[index].file_path))
		else {
			return;
		};
		let tab = &mut self.tabs[index];
		let Some(speaker) = self.speaker.as_mut() else { return };
		let caret = tab.window.to_doc(tab.text_ctrl.get_insertion_point());
		// `try_lock` because Settings holds the config while it's open; the move is still there to read from on a later tick.
		if let Some(moved_to) = speaker.caret_moved_to(caret)
			&& let Ok(config) = self.config.try_lock()
		{
			let paused = speaker.is_paused();
			speaker.read_from_caret(&tab.session, moved_to, &config, &tab.file_path.to_string_lossy(), paused);
		}
		match speaker.pump(&tab.session) {
			Some(Followed::Word(position)) => {
				if position != caret {
					let local = tab.window.to_local(position);
					tab.text_ctrl.set_insertion_point(local);
					tab.text_ctrl.show_position(local);
					speaker.caret_followed(tab.window.to_doc(tab.text_ctrl.get_insertion_point()));
				}
			}
			Some(Followed::End) => set_caret_to_doc_offset(tab, tab.session.document_len().as_pos()),
			None => {}
		}
	}

	/// Stops reading, such as when its document is closed or another becomes active.
	pub(super) fn stop_read_aloud(&mut self) {
		if let Some(speaker) = self.speaker.as_mut() {
			speaker.stop();
		}
		self.speaker_tab = None;
	}

	/// The active tab, if it's the one being read.
	fn reading_tab_index(&self) -> Option<usize> {
		let index = self.active_tab_index()?;
		(self.speaker_tab.as_ref() == Some(&self.tabs[index].file_path)
			&& self.speaker.as_ref().is_some_and(Speaker::is_reading))
		.then_some(index)
	}

	fn speaker(&mut self) -> Result<&mut Speaker, String> {
		if self.speaker.is_none() {
			let engine = self.config.lock().unwrap().get_app_string("tts_engine", "");
			let speaker = Speaker::new(&engine).map_err(|error| {
				tracing::warn!(%error, "read aloud isn't available");
				// TRANSLATORS: Announced when read aloud is asked for on a system Paperback has no speech engine for yet
				t("Read aloud isn't available on this system yet.")
			})?;
			self.speaker = Some(speaker);
		}
		Ok(self.speaker.as_mut().expect("set above"))
	}
}
