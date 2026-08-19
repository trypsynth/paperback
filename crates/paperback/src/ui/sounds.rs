use wxdragon::sound::{Sound, SoundFlags};

static BOOKMARK_WAV: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../sounds/bookmark.wav"));
static NOTE_WAV: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../sounds/note.wav"));

pub fn play_bookmark_sound(has_note: bool) {
	let bytes = if has_note { NOTE_WAV } else { BOOKMARK_WAV };
	if !Sound::from_data(bytes).play(SoundFlags::Async) {
		tracing::debug!(has_note, "failed to play embedded sound");
	}
}
