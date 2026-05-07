#![warn(clippy::all, clippy::nursery, clippy::pedantic)]

pub mod config;
pub mod document;
pub mod export;
pub mod ipc;
pub mod parser;
pub mod reader_core;
pub mod session;
pub mod types;
pub mod update;
pub mod util;
pub mod version;

/// Minimal translation stub for library-internal strings (e.g. document content labels).
/// The GUI binary sets up the real wxWidgets translation system independently; strings
/// returned by this function are English only and are intended for non-GUI consumers
/// (CLI, mobile bindings) or for embedding into document content.
pub(crate) fn t(s: &str) -> String {
	s.to_owned()
}
