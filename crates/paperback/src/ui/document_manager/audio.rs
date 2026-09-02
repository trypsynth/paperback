//! Playing a document's recorded narration, and the sounds that mark a bookmark or a note.

use wxdragon::prelude::*;

use super::DocumentManager;

impl DocumentManager {
	/// Stops every tab's audio ahead of the app closing, winding the native media sessions
	/// down deliberately rather than as a side effect of the frame being destroyed.
	pub fn stop_all_audio(&mut self) {
		for tab in &mut self.tabs {
			if let Some(player) = tab.audio_player.as_mut() {
				player.stop();
			}
		}
	}

	pub(crate) fn check_bookmark_sounds(&self) {
		let config = self.config.lock().unwrap();
		if !config.get_app_bool("bookmark_sounds", true) {
			return;
		}
		let Some(tab) = self.active_tab() else {
			return;
		};
		let position = tab.window.to_doc(tab.text_ctrl.get_insertion_point());
		let prev = self.last_sound_position.get().unwrap_or(position);
		self.last_sound_position.set(Some(position));
		if prev == position {
			return;
		}
		let path_str = tab.file_path.to_string_lossy().to_string();
		let bookmarks = config.get_bookmarks(&path_str);
		drop(config);
		let mut has_note = false;
		let mut has_bookmark = false;
		for bm in &bookmarks {
			let was_inside = if bm.start == bm.end { prev == bm.start } else { prev >= bm.start && prev < bm.end };
			let is_inside =
				if bm.start == bm.end { position == bm.start } else { position >= bm.start && position < bm.end };
			let triggered = is_inside && !was_inside;
			if triggered {
				if bm.note.is_empty() {
					has_bookmark = true;
				} else {
					has_note = true;
				}
			}
		}
		if has_note || has_bookmark {
			crate::ui::sounds::play_bookmark_sound(has_note);
		}
	}

	/// When "sync caret to audio" is on, moves the caret to follow playback. Called from a
	/// recurring timer; a no-op for documents with no audio.
	///
	/// Uses `try_lock` on `config` rather than `lock`: this runs on the main thread on every
	/// timer tick, and a modal dialog (e.g. Options) pumps the OS message loop while it holds
	/// that same lock across `show_modal`. A blocking `lock` here would deadlock the UI thread
	/// against itself the moment a tick landed mid-dialog; skipping the tick is harmless since
	/// it just retries in 250ms.
	pub fn pump_audio(&mut self) {
		let Ok(config) = self.config.try_lock() else {
			return;
		};
		let sync_enabled = config.get_app_bool("sync_caret_to_audio", true);
		drop(config);
		let Some(tab) = self.active_tab_mut() else {
			return;
		};
		let Some(player) = tab.audio_player.as_ref() else {
			return;
		};
		if !sync_enabled || !player.is_playing() {
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
		let Some(position) = player.timeline().clip(cursor.clip).map(|clip| clip.start) else {
			tracing::warn!(clip_index = cursor.clip, "sync caret to audio: cursor names a clip that doesn't exist");
			return;
		};
		// TODO(windowing, phase 3): still sets the caret directly rather than through a
		// window-aware jump, see C:\Users\Quin\.claude\plans\fluffy-hugging-crystal.md - a
		// scrub target outside the loaded window is silently clamped instead of reloading.
		let current = tab.window.to_doc(tab.text_ctrl.get_insertion_point());
		if i64::try_from(position).ok() != Some(current) {
			let target = i64::try_from(position).unwrap_or(current);
			let local = tab.window.to_local(target);
			tab.text_ctrl.set_insertion_point(local);
			tab.text_ctrl.show_position(local);
		}
	}

	/// Pauses audio on every tab except the active one, so switching tabs can't leave two
	/// documents narrating at once (the active tab may have audio of its own still playing,
	/// which this leaves untouched).
	pub fn pause_inactive_audio(&mut self) {
		let active = self.active_tab_index();
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
		self.last_audio_seek_position.set(None);
	}
}
