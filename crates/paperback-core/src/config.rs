mod manager;
mod settings;
mod shortcuts;

pub use manager::{ConfigManager, compute_document_hash, document_missing, document_name, get_sorted_document_list};
pub use settings::{
	AppSettings, Bookmark, ConfigData, DocumentConfig, DocumentSpeech, FindSettings, NavigationHistory,
	ReadabilityFont, StoredBookmark,
};
pub use shortcuts::{ActionId, KeyChord, ShortcutCategory, ShortcutsConfig, default_window_hotkey};
