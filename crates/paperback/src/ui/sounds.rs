use std::{env, fs, path::PathBuf, sync::OnceLock};

use wxdragon::sound::{Sound, SoundFlags};

static BOOKMARK_WAV: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../sounds/bookmark.wav"));
static NOTE_WAV: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../sounds/note.wav"));

/// wxWidgets' `wxSound` (via `wxdragon::sound::Sound`) only plays from a filesystem path —
/// there's no in-memory-buffer constructor exposed — so an embedded sound still needs
/// writing to disk once before it can be played. Extracted lazily on first use and cached
/// for the rest of the process; later calls skip the write entirely.
fn extracted_path(bytes: &'static [u8], filename: &str, cache: &'static OnceLock<Option<PathBuf>>) -> Option<PathBuf> {
	cache
		.get_or_init(|| {
			let dir = env::temp_dir().join("paperback-sounds");
			fs::create_dir_all(&dir).ok()?;
			let path = dir.join(filename);
			fs::write(&path, bytes).ok()?;
			Some(path)
		})
		.clone()
}

fn play_embedded(bytes: &'static [u8], filename: &str, cache: &'static OnceLock<Option<PathBuf>>) {
	if let Some(path) = extracted_path(bytes, filename, cache) {
		Sound::play_file(&path.to_string_lossy(), SoundFlags::Async);
	} else {
		tracing::debug!(filename, "failed to extract embedded sound, skipping");
	}
}

pub fn play_bookmark_sound(has_note: bool) {
	static BOOKMARK_PATH: OnceLock<Option<PathBuf>> = OnceLock::new();
	static NOTE_PATH: OnceLock<Option<PathBuf>> = OnceLock::new();
	if has_note {
		play_embedded(NOTE_WAV, "note.wav", &NOTE_PATH);
	} else {
		play_embedded(BOOKMARK_WAV, "bookmark.wav", &BOOKMARK_PATH);
	}
}
