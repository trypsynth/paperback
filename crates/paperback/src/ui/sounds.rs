#[cfg(target_os = "macos")]
use std::{env, fs, io, path::PathBuf};

use wxdragon::sound::{Sound, SoundFlags};

static BOOKMARK_WAV: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../sounds/bookmark.wav"));
static NOTE_WAV: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../sounds/note.wav"));

pub fn play_bookmark_sound(has_note: bool) {
	let (bytes, name) = if has_note { (NOTE_WAV, "note.wav") } else { (BOOKMARK_WAV, "bookmark.wav") };
	if !play(bytes, name) {
		tracing::debug!(has_note, "failed to play embedded sound");
	}
}

#[cfg(not(target_os = "macos"))]
fn play(bytes: &[u8], _name: &str) -> bool {
	Sound::from_data(bytes).play(SoundFlags::Async)
}

/// wxOSX cannot build a sound from memory at all: `wxSound::Create(size, data)` is a
/// `wxFAIL_MSG("not implemented")`, and asserts are compiled into the wxWidgets we link, so
/// passing the embedded bytes straight to it aborts rather than just failing. The sound API
/// there wraps `AudioServices`, which needs a file URL, so the bytes have to land on disk first.
#[cfg(target_os = "macos")]
fn play(bytes: &[u8], name: &str) -> bool {
	match cached_wav(bytes, name) {
		Ok(path) => Sound::play_file(&path.to_string_lossy(), SoundFlags::Async),
		Err(e) => {
			tracing::debug!(error = %e, name, "could not write sound to the cache directory");
			false
		}
	}
}

/// Writes an embedded sound to a stable temp path, once. Rewritten when the file is missing or
/// its length no longer matches, so a build with different sounds is not stuck with old ones.
#[cfg(target_os = "macos")]
fn cached_wav(bytes: &[u8], name: &str) -> io::Result<PathBuf> {
	let dir = env::temp_dir().join("paperback-sounds");
	fs::create_dir_all(&dir)?;
	let path = dir.join(name);
	if !fs::metadata(&path).is_ok_and(|m| m.len() == bytes.len() as u64) {
		fs::write(&path, bytes)?;
	}
	Ok(path)
}
