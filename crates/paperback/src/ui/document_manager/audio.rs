use super::DocumentManager;

impl DocumentManager {
	pub fn stop_all_audio(&mut self) {
		for tab in &mut self.tabs {
			if let Some(player) = tab.audio_player.as_mut() {
				player.stop();
			}
		}
	}

	/// Called after a jump, so the next step isn't measured from the old position and sounded as crossing a bookmark it never crossed.
	pub(crate) fn forget_bookmark_sound_position(&self) {
		let Some(tab) = self.active_tab() else {
			return;
		};
		self.last_sound_position.set(Some(tab.window.to_doc(tab.text_ctrl.get_insertion_point())));
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

	pub fn pump_audio(&mut self) {
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
		let Some(position) = player.timeline().clip(cursor.clip).map(|clip| clip.start) else {
			tracing::warn!(clip_index = cursor.clip, "sync caret to audio: cursor names a clip that doesn't exist");
			return;
		};
		// TODO(windowing): sets the caret directly rather than through a window-aware jump, so a target outside the loaded window is clamped instead of reloading.
		let current = tab.window.to_doc(tab.text_ctrl.get_insertion_point());
		if position != current {
			let local = tab.window.to_local(position);
			tab.text_ctrl.set_insertion_point(local);
			tab.text_ctrl.show_position(local);
		}
	}

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
