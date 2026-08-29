//! Reading-position logic shared across all document formats: marker-based navigation, search,
//! bookmark lookups, back/forward history, and link resolution. Kept format-agnostic and free of
//! FFI/session state so it can be unit tested directly.

mod bookmarks;
mod history;
mod links;
mod navigation;
mod search;

pub use bookmarks::{bookmark_navigate, bookmark_note_at_position, get_filtered_bookmarks};
pub use history::{HistoryNavResult, history_go_next, history_go_previous, record_history_position};
pub use links::{LinkNavigation, encode_url_fragment, nearest_fragment_before, resolve_link};
pub use navigation::{reader_container_navigate, reader_navigate};
pub use search::{SearchOptions, reader_search, reader_search_with_wrap};
