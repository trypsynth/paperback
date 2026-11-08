//! Read aloud: a document's paragraphs spoken through a speech engine, with the user's pronunciation rules, voice and pauses.
//!
//! An engine only turns one paragraph of text into audio. Everything else, from choosing the voice to following the reader's place, happens here, so every platform's read aloud behaves the same.

mod dictionary;
mod engine;
mod text_map;
mod voice;

pub use dictionary::{MatchType, RuleScope, SpeechRule, VoiceFilter, apply_rules};
pub use engine::{AudioChunk, SpeechEngine, SpeechRequest, Synthesis, Voice, WordBoundary, utf16_to_char_offset};
pub use text_map::TextMap;
pub use voice::choose_voice;
