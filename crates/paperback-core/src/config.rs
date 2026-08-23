//! Persisted app/document configuration, split into three concerns that together back
//! the flat `paperback_core::config::*` surface every consumer already imports from:
//! [`shortcuts`] (the keyboard shortcut system), [`settings`] (the serializable data
//! model and its defaults), and [`manager`] ([`ConfigManager`], the stateful store built
//! on top of them).

mod manager;
mod settings;
mod shortcuts;

pub use manager::{ConfigManager, compute_document_hash, get_sorted_document_list};
pub use settings::{
	AppSettings, Bookmark, ConfigData, DocumentConfig, FindSettings, NavigationHistory, ReadabilityFont, StoredBookmark,
};
pub use shortcuts::{ActionId, HotkeyConfig, KeyChord, ShortcutCategory, ShortcutsConfig};
