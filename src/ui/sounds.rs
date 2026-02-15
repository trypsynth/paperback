use std::{
	env,
	path::{Path, PathBuf},
};

use wxdragon::sound::{Sound, SoundFlags};

fn sounds_directory() -> PathBuf {
	env::current_exe()
		.ok()
		.and_then(|p| p.parent().map(Path::to_path_buf))
		.unwrap_or_else(|| PathBuf::from("."))
		.join("sounds")
}

pub fn play_sound(filename: &str) {
	let path = sounds_directory().join(filename);
	if path.exists() {
		Sound::play_file(&path.to_string_lossy(), SoundFlags::Async);
	}
}

pub fn play_bookmark_sound(has_note: bool) {
	if has_note {
		play_sound("note.wav");
	} else {
		play_sound("bookmark.wav");
	}
}
