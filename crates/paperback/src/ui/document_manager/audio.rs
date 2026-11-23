#[cfg(target_os = "windows")]
use std::rc::Rc;

#[cfg(target_os = "windows")]
use paperback_core::config::ConfigManager;
use paperback_core::document::DocPos;
#[cfg(target_os = "windows")]
use wxdragon::prelude::*;

use super::{DocumentManager, DocumentTab};
#[cfg(target_os = "windows")]
use crate::ui::reader_units::visual_line_at;
use crate::ui::{
	bookmark_sounds::{BookmarkSoundMode, MoveUnit, Unit, remembered_unit, sound_for_move},
	navigation::doc_caret,
};

/// The word, visual line or paragraph at the caret.
#[cfg(target_os = "windows")]
fn unit_at_caret(tab: &DocumentTab, unit: MoveUnit) -> Option<Unit> {
	let caret = tab.text_ctrl.get_insertion_point();
	let (start, end) = match unit {
		MoveUnit::Character => return None,
		MoveUnit::Word => (caret, crate::ui::reader_units::word_end(tab.text_ctrl, caret)),
		MoveUnit::Line => visual_line_at(tab.text_ctrl, caret)?,
		MoveUnit::Paragraph => {
			let (start, end) = tab.session.line_bounds_at(tab.window.to_doc(caret))?;
			return Some(Unit { start, end });
		}
	};
	Some(Unit { start: tab.window.to_doc(start), end: tab.window.to_doc(end) })
}

/// Only Windows measures the unit a key moves by.
#[cfg(not(target_os = "windows"))]
const fn unit_at_caret(_tab: &DocumentTab, _unit: MoveUnit) -> Option<Unit> {
	None
}

impl DocumentManager {
	pub fn stop_all_audio(&mut self) {
		self.stop_read_aloud();
		for tab in &mut self.tabs {
			if let Some(player) = tab.audio_player.as_mut() {
				player.stop();
			}
		}
	}

	#[cfg(target_os = "windows")]
	/// Moves everything playing, recorded or spoken, onto the system's current default output after Windows switched it, such as to or from Bluetooth headphones. Each stream stays on the output it was opened on, and goes silent if that goes away.
	pub fn follow_default_audio_device(&mut self, config: &ConfigManager) {
		let device = match crate::audio_player::reopen_shared_device() {
			Ok(device) => device,
			Err(error) => {
				tracing::warn!(%error, "audio: couldn't open the new default output device");
				return;
			}
		};
		for tab in &mut self.tabs {
			if let Some(player) = tab.audio_player.as_mut() {
				player.move_to_device(Rc::clone(&device));
			}
		}
		let Some(speaker) = self.speaker.as_mut() else { return };
		let reading_tab =
			self.speaker_tab.as_ref().and_then(|path| self.tabs.iter().find(|tab| &tab.file_path == path));
		let book = reading_tab.map(|tab| (&tab.session, config, tab.file_path.to_string_lossy().into_owned()));
		let moved = speaker
			.move_to_device(device, book.as_ref().map(|(session, config, path)| (*session, *config, path.as_str())));
		if let Err(error) = moved {
			tracing::warn!(%error, "read aloud: couldn't move to the new default output device");
		}
	}

	/// Moves what is playing to the new output once Windows has switched it. An Options dialog open holds the config for as long as it is, and locking it from here would freeze the window, so the move waits until the config is free.
	#[cfg(target_os = "windows")]
	fn follow_output_change(&mut self) {
		if !crate::windows_audio::take_output_change() {
			return;
		}
		let config = Rc::clone(&self.config);
		let Ok(config) = config.try_lock() else {
			crate::windows_audio::note_output_change();
			return;
		};
		self.follow_default_audio_device(&config);
	}

	/// Called after a jump, so the next step isn't measured from the old position and sounded as crossing a bookmark it never crossed.
	fn forget_bookmark_sound_position(&self) {
		let Some(tab) = self.active_tab() else {
			return;
		};
		let caret = doc_caret(tab);
		self.last_sound_unit.set(Some(remembered_unit(self.last_sound_unit.get(), None, caret)));
		self.last_sound_position.set(Some(caret));
	}

	/// Sounds a bookmark after a key moved the caret by `unit`. Other keys only note the caret's position.
	pub(crate) fn sound_key_move(&self, unit: Option<MoveUnit>) {
		let Some(unit) = unit else {
			self.forget_bookmark_sound_position();
			return;
		};
		let Some(tab) = self.active_tab() else {
			return;
		};
		let caret = doc_caret(tab);
		// A line or paragraph move sounds only by its unit.
		let before = match unit {
			MoveUnit::Character | MoveUnit::Word => self.last_sound_position.get().unwrap_or(caret),
			MoveUnit::Line | MoveUnit::Paragraph => caret,
		};
		self.sound_move(before, caret, |tab| unit_at_caret(tab, unit));
	}

	pub(crate) fn check_bookmark_sounds(&self) {
		let Some(tab) = self.active_tab() else {
			return;
		};
		let caret = doc_caret(tab);
		self.sound_move(self.last_sound_position.get().unwrap_or(caret), caret, |_| None);
	}

	/// Sounds the caret's move from `before` to `after` by the setting. Under the unit setting, `unit_of` measures the unit moved to; without a unit, the move sounds the bookmarks it entered.
	pub(super) fn sound_move(&self, before: DocPos, after: DocPos, unit_of: impl FnOnce(&DocumentTab) -> Option<Unit>) {
		let Some(tab) = self.active_tab() else {
			return;
		};
		let Ok(config) = self.config.try_lock() else {
			return;
		};
		let mode = BookmarkSoundMode::load(&config);
		self.last_sound_position.set(Some(after));
		if mode == BookmarkSoundMode::Never {
			self.last_sound_unit.set(None);
			return;
		}
		let unit = if mode == BookmarkSoundMode::InUnit { unit_of(tab) } else { None };
		let bookmarks = config.get_bookmarks(&tab.file_path.to_string_lossy());
		drop(config);
		let (sound, remembered) = sound_for_move(&bookmarks, self.last_sound_unit.get(), before, after, unit);
		self.last_sound_unit.set(Some(remembered));
		if let Some(sound) = sound {
			sound.play();
		}
	}

	pub fn pump_audio(&mut self) {
		#[cfg(target_os = "windows")]
		{
			crate::windows_audio::publish_playback_state(self.frame.get_handle(), self.media_playback_state());
			self.follow_output_change();
		}
		// `try_lock` because a modal dialog like Options holds the config lock across `show_modal` while pumping messages, so a blocking lock on this tick would deadlock the UI thread.
		let sync_enabled = self.config.try_lock().ok().map(|config| config.get_app_bool("sync_caret_to_audio", true));
		let Some(tab) = self.active_tab_mut() else {
			return;
		};
		let Some(player) = tab.audio_player.as_mut() else {
			return;
		};
		// rodio has no "finished" callback, so this tick also advances to the next file, whether or not sync is on or the lock was free.
		player.pump();
		if sync_enabled != Some(true) || !player.is_playing() {
			return;
		}
		let Some(elapsed) = player.elapsed_ms() else {
			tracing::warn!("sync caret to audio: playing but no elapsed position available");
			return;
		};
		let Some(cursor) = player.timeline().cursor_at_elapsed(elapsed) else {
			tracing::warn!(elapsed, "sync caret to audio: no clip covers the current elapsed time");
			return;
		};
		let Some(clip) = player.timeline().clip(cursor.clip).map(|clip| Unit { start: clip.start, end: clip.end })
		else {
			tracing::warn!(clip_index = cursor.clip, "sync caret to audio: cursor names a clip that doesn't exist");
			return;
		};
		let position = clip.start;
		// TODO(windowing): sets the caret directly rather than through a window-aware jump, so a target outside the loaded window is clamped instead of reloading.
		let current = tab.window.to_doc(tab.text_ctrl.get_insertion_point());
		if position != current {
			let local = tab.window.to_local(position);
			tab.text_ctrl.set_insertion_point(local);
			tab.text_ctrl.show_position(local);
			self.sound_move(current, position, |_| Some(clip));
		}
	}

	pub fn pause_inactive_audio(&mut self) {
		let active = self.active_tab_index();
		if self.speaker_tab.is_some() && active.map(|index| &self.tabs[index].file_path) != self.speaker_tab.as_ref() {
			self.stop_read_aloud();
		}
		for (index, tab) in self.tabs.iter_mut().enumerate() {
			if Some(index) != active
				&& let Some(player) = tab.audio_player.as_mut()
				&& player.is_playing()
			{
				player.pause();
			}
		}
	}

	pub fn reset_sound_line(&self) {
		self.last_sound_position.set(None);
		self.last_sound_unit.set(None);
		self.last_audio_seek_position.set(None);
	}
}
