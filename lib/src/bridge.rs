#[cxx::bridge]
pub mod ffi {
	#[derive(Debug, Clone, Copy, PartialEq, Eq)]
	pub enum UpdateStatus {
		Available,
		UpToDate,
		HttpError,
		NetworkError,
		InvalidResponse,
		NoDownload,
		InvalidInput,
		InternalError,
	}

	pub struct UpdateResult {
		pub status: UpdateStatus,
		pub http_status: i32,
		pub latest_version: String,
		pub download_url: String,
		pub release_notes: String,
		pub error_message: String,
	}

	pub struct ParserInfo {
		pub name: String,
		pub extensions: Vec<String>,
		pub flags: u32,
	}

	#[derive(Debug, Clone, Copy, PartialEq, Eq)]
	#[repr(i32)]
	pub enum MarkerKind {
		Heading1 = 0,
		Heading2 = 1,
		Heading3 = 2,
		Heading4 = 3,
		Heading5 = 4,
		Heading6 = 5,
		PageBreak = 6,
		SectionBreak = 7,
		TocItem = 8,
		Link = 9,
		List = 10,
		ListItem = 11,
		Table = 12,
	}

	#[derive(Debug, Clone, Copy, PartialEq, Eq)]
	pub enum NavDirection {
		Next,
		Previous,
	}

	#[derive(Debug, Clone, Copy, PartialEq, Eq)]
	pub enum NavTarget {
		Section,
		Page,
		Heading,
		List,
		ListItem,
		Link,
		Table,
	}

	pub struct NavRequest {
		pub position: i64,
		pub wrap: bool,
		pub direction: NavDirection,
		pub target: NavTarget,
		pub level_filter: i32,
	}

	pub struct NavResult {
		pub found: bool,
		pub wrapped: bool,
		pub offset: usize,
		pub marker_level: i32,
		pub marker_text: String,
	}

	pub struct BookmarkNavResult {
		pub found: bool,
		pub start: i64,
		pub end: i64,
		pub note: String,
		pub index: i32,
		pub wrapped: bool,
	}

	pub struct FfiBookmarkNavDisplay {
		pub found: bool,
		pub wrapped: bool,
		pub start: i64,
		pub end: i64,
		pub note: String,
		pub snippet: String,
		pub index: i32,
	}

	pub struct FfiBookmarkDisplayAtPosition {
		pub found: bool,
		pub note: String,
		pub snippet: String,
	}

	#[derive(Debug, Clone, Copy, PartialEq, Eq)]
	pub enum ParserErrorKind {
		Generic,
		PasswordRequired,
	}

	pub struct ParserErrorInfo {
		pub kind: ParserErrorKind,
		pub detail: String,
	}

	pub struct FfiSearchResult {
		pub found: bool,
		pub wrapped: bool,
		pub position: i64,
	}

	#[derive(Debug, Clone, Copy, PartialEq, Eq)]
	pub enum BookmarkFilterType {
		All,
		BookmarksOnly,
		NotesOnly,
	}

	pub struct FfiBookmarkDisplayItem {
		pub start: i64,
		pub end: i64,
		pub note: String,
		pub is_whole_line: bool,
		pub index: usize,
	}

	pub struct FfiBookmarkDisplayEntry {
		pub start: i64,
		pub end: i64,
		pub note: String,
		pub snippet: String,
		pub is_whole_line: bool,
		pub index: usize,
	}

	pub struct FfiFilteredBookmarks {
		pub items: Vec<FfiBookmarkDisplayItem>,
		pub closest_index: i32,
	}

	pub struct FfiFilteredBookmarkDisplay {
		pub items: Vec<FfiBookmarkDisplayEntry>,
		pub closest_index: i32,
	}

	#[derive(Debug, Clone, Copy, PartialEq, Eq)]
	pub enum DocumentListStatus {
		Open,
		Closed,
		Missing,
	}

	pub struct FfiDocumentListItem {
		pub path: String,
		pub filename: String,
		pub status: DocumentListStatus,
	}

	pub struct FfiRecentDocument {
		pub path: String,
		pub filename: String,
	}

	pub struct FfiLinkListItem {
		pub offset: usize,
		pub text: String,
	}

	pub struct FfiLinkList {
		pub items: Vec<FfiLinkListItem>,
		pub closest_index: i32,
	}

	pub struct FfiTocItemWithParent {
		pub name: String,
		pub reference: String,
		pub offset: usize,
		pub depth: i32,
		pub parent_index: i32,
	}

	pub struct FfiDocumentStats {
		pub word_count: usize,
		pub line_count: usize,
		pub char_count: usize,
		pub char_count_no_whitespace: usize,
	}

	pub struct FfiHeadingTreeItem {
		pub offset: usize,
		pub level: i32,
		pub text: String,
		pub parent_index: i32,
	}

	pub struct FfiHeadingTree {
		pub items: Vec<FfiHeadingTreeItem>,
		pub closest_index: i32,
	}

	pub struct FfiBookmark {
		pub start: i64,
		pub end: i64,
		pub note: String,
	}

	pub struct FfiNavigationHistory {
		pub positions: Vec<i64>,
		pub index: usize,
	}

	pub struct FfiHistoryNavResult {
		pub found: bool,
		pub target: i64,
		pub positions: Vec<i64>,
		pub index: usize,
	}

	pub struct FfiLinkNavigation {
		pub found: bool,
		pub is_external: bool,
		pub offset: usize,
		pub url: String,
	}

	pub struct FfiSessionNavResult {
		pub found: bool,
		pub wrapped: bool,
		pub offset: i64,
		pub marker_text: String,
		pub marker_level: i32,
		pub marker_index: i32,
		pub not_supported: bool,
	}

	#[derive(Debug, Clone, Copy, PartialEq, Eq)]
	pub enum FfiLinkAction {
		Internal,
		External,
		NotFound,
	}

	pub struct FfiLinkActivationResult {
		pub found: bool,
		pub action: FfiLinkAction,
		pub offset: i64,
		pub url: String,
	}

	pub struct FfiStatusInfo {
		pub line_number: i64,
		pub character_number: i64,
		pub percentage: i32,
		pub total_chars: i64,
	}

	extern "Rust" {
		type ConfigManager;
		type DocumentHandle;
		type DocumentSession;

		fn config_manager_new() -> Box<ConfigManager>;
		fn config_manager_initialize(manager: &mut ConfigManager) -> bool;
		fn config_manager_flush(manager: &ConfigManager);
		fn config_manager_shutdown(manager: &mut ConfigManager);
		fn config_manager_get_string(manager: &ConfigManager, key: &str, default_value: &str) -> String;
		fn config_manager_get_bool(manager: &ConfigManager, key: &str, default_value: bool) -> bool;
		fn config_manager_get_int(manager: &ConfigManager, key: &str, default_value: i32) -> i32;
		fn config_manager_set_string(manager: &mut ConfigManager, key: &str, value: &str);
		fn config_manager_set_bool(manager: &mut ConfigManager, key: &str, value: bool);
		fn config_manager_set_int(manager: &mut ConfigManager, key: &str, value: i32);
		fn config_manager_get_app_string(manager: &ConfigManager, key: &str, default_value: &str) -> String;
		fn config_manager_get_app_bool(manager: &ConfigManager, key: &str, default_value: bool) -> bool;
		fn config_manager_get_app_int(manager: &ConfigManager, key: &str, default_value: i32) -> i32;
		fn config_manager_set_app_string(manager: &mut ConfigManager, key: &str, value: &str);
		fn config_manager_set_app_bool(manager: &mut ConfigManager, key: &str, value: bool);
		fn config_manager_set_app_int(manager: &mut ConfigManager, key: &str, value: i32);
		fn config_manager_get_doc_string(manager: &ConfigManager, path: &str, key: &str, default_value: &str)
		-> String;
		fn config_manager_get_doc_bool(manager: &ConfigManager, path: &str, key: &str, default_value: bool) -> bool;
		fn config_manager_get_doc_int(manager: &ConfigManager, path: &str, key: &str, default_value: i64) -> i64;
		fn config_manager_set_doc_string(manager: &mut ConfigManager, path: &str, key: &str, value: &str);
		fn config_manager_set_doc_bool(manager: &mut ConfigManager, path: &str, key: &str, value: bool);
		fn config_manager_set_doc_int(manager: &mut ConfigManager, path: &str, key: &str, value: i64);
		fn config_manager_add_recent_document(manager: &mut ConfigManager, path: &str);
		fn config_manager_get_recent_documents(manager: &ConfigManager) -> Vec<String>;
		fn config_manager_clear_recent_documents(manager: &mut ConfigManager);
		fn config_manager_add_opened_document(manager: &mut ConfigManager, path: &str);
		fn config_manager_remove_opened_document(manager: &mut ConfigManager, path: &str);
		fn config_manager_clear_opened_documents(manager: &mut ConfigManager);
		fn config_manager_set_document_position(manager: &mut ConfigManager, path: &str, position: i64);
		fn config_manager_get_document_position(manager: &ConfigManager, path: &str) -> i64;
		fn config_manager_set_navigation_history(
			manager: &mut ConfigManager,
			path: &str,
			history: &[i64],
			history_index: usize,
		);
		fn config_manager_get_navigation_history(manager: &ConfigManager, path: &str) -> FfiNavigationHistory;
		fn config_manager_set_document_opened(manager: &mut ConfigManager, path: &str, opened: bool);
		fn config_manager_get_document_opened(manager: &ConfigManager, path: &str) -> bool;
		fn config_manager_remove_document_history(manager: &mut ConfigManager, path: &str);
		fn config_manager_remove_navigation_history(manager: &mut ConfigManager, path: &str);
		fn config_manager_get_all_opened_documents(manager: &ConfigManager) -> Vec<String>;
		fn config_manager_get_all_documents(manager: &ConfigManager) -> Vec<String>;
		fn config_manager_add_bookmark(manager: &mut ConfigManager, path: &str, start: i64, end: i64, note: &str);
		fn config_manager_remove_bookmark(manager: &mut ConfigManager, path: &str, start: i64, end: i64);
		fn config_manager_toggle_bookmark(manager: &mut ConfigManager, path: &str, start: i64, end: i64, note: &str);
		fn config_manager_update_bookmark_note(
			manager: &mut ConfigManager,
			path: &str,
			start: i64,
			end: i64,
			note: &str,
		);
		fn config_manager_get_bookmarks(manager: &ConfigManager, path: &str) -> Vec<FfiBookmark>;
		fn config_manager_clear_bookmarks(manager: &mut ConfigManager, path: &str);
		fn config_manager_get_next_bookmark(manager: &ConfigManager, path: &str, current_position: i64) -> FfiBookmark;
		fn config_manager_get_previous_bookmark(
			manager: &ConfigManager,
			path: &str,
			current_position: i64,
		) -> FfiBookmark;
		fn config_manager_set_document_format(manager: &mut ConfigManager, path: &str, format: &str);
		fn config_manager_get_document_format(manager: &ConfigManager, path: &str) -> String;
		fn config_manager_set_document_password(manager: &mut ConfigManager, path: &str, password: &str);
		fn config_manager_get_document_password(manager: &ConfigManager, path: &str) -> String;
		fn config_manager_needs_migration(manager: &ConfigManager) -> bool;
		fn config_manager_migrate_config(manager: &mut ConfigManager) -> bool;
		fn config_manager_export_document_settings(manager: &ConfigManager, doc_path: &str, export_path: &str);
		fn config_manager_import_document_settings(manager: &mut ConfigManager, path: &str);
		fn config_manager_import_settings_from_file(manager: &mut ConfigManager, doc_path: &str, import_path: &str);
		fn parse_document_handle(
			file_path: &str,
			password: &str,
			forced_extension: &str,
		) -> Result<Box<DocumentHandle>>;
		fn check_for_updates(current_version: &str, is_installer: bool) -> UpdateResult;
		fn remove_soft_hyphens(input: &str) -> String;
		fn url_decode(encoded: &str) -> String;
		fn collapse_whitespace(input: &str) -> String;
		fn trim_string(input: &str) -> String;
		fn convert_to_utf8(input: &[u8]) -> String;
		fn read_zip_entry(zip_path: &str, entry_name: &str) -> Result<String>;
		fn find_zip_entry(zip_path: &str, entry_name: &str) -> Result<usize>;
		fn get_available_parsers() -> Vec<ParserInfo>;
		fn parser_supported_wildcards() -> String;
		fn parser_supports_extension(extension: &str) -> bool;
		fn parser_error_info(message: &str) -> ParserErrorInfo;
		fn get_parser_for_extension(extension: &str) -> Result<String>;
		fn markdown_to_text(input: &str) -> String;
		fn reader_navigate(doc: &DocumentHandle, req: &NavRequest) -> NavResult;
		fn reader_search(
			req: &str,
			needle: &str,
			start: i64,
			forward: bool,
			match_case: bool,
			whole_word: bool,
			regex: bool,
		) -> i64;
		fn reader_search_with_wrap(
			req: &str,
			needle: &str,
			start: i64,
			forward: bool,
			match_case: bool,
			whole_word: bool,
			regex: bool,
		) -> FfiSearchResult;
		fn bookmark_navigate(
			manager: &ConfigManager,
			path: &str,
			position: i64,
			wrap: bool,
			next: bool,
			notes_only: bool,
		) -> BookmarkNavResult;
		fn bookmark_note_at_position(manager: &ConfigManager, path: &str, position: i64) -> String;
		fn get_filtered_bookmarks(
			manager: &ConfigManager,
			path: &str,
			current_pos: i64,
			filter: BookmarkFilterType,
		) -> FfiFilteredBookmarks;
		fn get_filtered_bookmark_display_items(
			session: &DocumentSession,
			manager: &ConfigManager,
			path: &str,
			current_pos: i64,
			filter: BookmarkFilterType,
		) -> FfiFilteredBookmarkDisplay;
		fn get_sorted_document_list(
			config: &ConfigManager,
			open_paths: &[String],
			filter: &str,
		) -> Vec<FfiDocumentListItem>;
		fn get_recent_documents_for_menu(config: &ConfigManager, limit: usize) -> Vec<FfiRecentDocument>;
		fn history_normalize(history: &[i64], history_index: usize) -> FfiNavigationHistory;
		fn history_record_position(
			history: &[i64],
			history_index: usize,
			current_pos: i64,
			max_len: usize,
		) -> FfiNavigationHistory;
		fn history_go_previous(
			history: &[i64],
			history_index: usize,
			current_pos: i64,
			max_len: usize,
		) -> FfiHistoryNavResult;
		fn history_go_next(
			history: &[i64],
			history_index: usize,
			current_pos: i64,
			max_len: usize,
		) -> FfiHistoryNavResult;
		fn resolve_link(doc: &DocumentHandle, href: &str, current_position: i64) -> FfiLinkNavigation;
		fn session_new(file_path: &str, password: &str, forced_extension: &str) -> Result<Box<DocumentSession>>;
		fn session_title(session: &DocumentSession) -> String;
		fn session_author(session: &DocumentSession) -> String;
		fn session_content(session: &DocumentSession) -> String;
		fn session_file_path(session: &DocumentSession) -> String;
		fn session_parser_flags(session: &DocumentSession) -> u32;
		fn session_get_history(session: &DocumentSession) -> FfiNavigationHistory;
		fn session_set_history(session: &mut DocumentSession, positions: &[i64], index: usize);
		fn session_record_position(session: &mut DocumentSession, position: i64);
		fn session_navigate_section(
			session: &DocumentSession,
			position: i64,
			wrap: bool,
			next: bool,
		) -> FfiSessionNavResult;
		fn session_navigate_heading(
			session: &DocumentSession,
			position: i64,
			wrap: bool,
			next: bool,
			level: i32,
		) -> FfiSessionNavResult;
		fn session_navigate_page(
			session: &DocumentSession,
			position: i64,
			wrap: bool,
			next: bool,
		) -> FfiSessionNavResult;
		fn session_navigate_link(
			session: &DocumentSession,
			position: i64,
			wrap: bool,
			next: bool,
		) -> FfiSessionNavResult;
		fn session_navigate_list(
			session: &DocumentSession,
			position: i64,
			wrap: bool,
			next: bool,
		) -> FfiSessionNavResult;
		fn session_navigate_list_item(
			session: &DocumentSession,
			position: i64,
			wrap: bool,
			next: bool,
		) -> FfiSessionNavResult;
		fn session_navigate_table(
			session: &DocumentSession,
			position: i64,
			wrap: bool,
			next: bool,
		) -> FfiSessionNavResult;
		fn session_navigate_bookmark(
			session: &DocumentSession,
			config: &ConfigManager,
			position: i64,
			wrap: bool,
			next: bool,
		) -> FfiSessionNavResult;
		fn session_navigate_bookmark_display(
			session: &DocumentSession,
			config: &ConfigManager,
			position: i64,
			wrap: bool,
			next: bool,
			notes_only: bool,
		) -> FfiBookmarkNavDisplay;
		fn session_bookmark_display_at_position(
			session: &DocumentSession,
			config: &ConfigManager,
			position: i64,
		) -> FfiBookmarkDisplayAtPosition;
		fn session_navigate_note(
			session: &DocumentSession,
			config: &ConfigManager,
			position: i64,
			wrap: bool,
			next: bool,
		) -> FfiSessionNavResult;
		fn session_link_list(session: &DocumentSession, position: i64) -> FfiLinkList;
		fn session_history_go_back(session: &mut DocumentSession, current_pos: i64) -> FfiSessionNavResult;
		fn session_history_go_forward(session: &mut DocumentSession, current_pos: i64) -> FfiSessionNavResult;
		fn session_activate_link(session: &mut DocumentSession, position: i64) -> FfiLinkActivationResult;
		fn session_get_table_at_position(session: &DocumentSession, position: i64) -> String;
		fn session_get_current_section_path(session: &DocumentSession, position: i64) -> String;
		fn session_extract_resource(session: &DocumentSession, resource_path: &str, output_path: &str) -> Result<bool>;
		fn session_get_status_info(session: &DocumentSession, position: i64) -> FfiStatusInfo;
		fn session_stats(session: &DocumentSession) -> FfiDocumentStats;
		fn session_page_count(session: &DocumentSession) -> usize;
		fn session_current_page(session: &DocumentSession, position: i64) -> i32;
		fn session_page_offset(session: &DocumentSession, page_index: i32) -> i64;
		fn session_export_content(session: &DocumentSession, output_path: &str) -> Result<()>;
		fn session_get_text_range(session: &DocumentSession, start: i64, end: i64) -> String;
		fn session_get_line_text(session: &DocumentSession, position: i64) -> String;
		fn session_handle(session: &DocumentSession) -> &DocumentHandle;
		fn session_toc_items_with_parents(session: &DocumentSession) -> Vec<FfiTocItemWithParent>;
		fn session_find_closest_toc_offset(session: &DocumentSession, position: usize) -> usize;
		fn session_heading_tree(session: &DocumentSession, position: i64) -> FfiHeadingTree;
		fn is_heading_marker_type(marker_type: i32) -> bool;
	}
}

use std::{fs::File, path::Path};

use self::ffi::UpdateStatus;
use crate::{
	config::{Bookmark, ConfigManager as RustConfigManager, NavigationHistory},
	document::{DocumentHandle, ParserContext, TocItem},
	parser, update as update_module,
	utils::{encoding, text, zip as zip_module},
};

type ConfigManager = crate::config::ConfigManager;

macro_rules! ffi_wrapper {
	(mut $fn_name:ident, $method:ident) => {
		fn $fn_name(manager: &mut RustConfigManager) { manager.$method(); }
	};
	($fn_name:ident, $method:ident) => {
		fn $fn_name(manager: &RustConfigManager) { manager.$method(); }
	};
	(mut $fn_name:ident, $method:ident -> $ret:ty) => {
		fn $fn_name(manager: &mut RustConfigManager) -> $ret { manager.$method() }
	};
	($fn_name:ident, $method:ident -> $ret:ty) => {
		fn $fn_name(manager: &RustConfigManager) -> $ret { manager.$method() }
	};
	(mut $fn_name:ident, $method:ident($($arg:ident: $ty:ty),*)) => {
		fn $fn_name(manager: &mut RustConfigManager, $($arg: $ty),*) { manager.$method($($arg),*); }
	};
	($fn_name:ident, $method:ident($($arg:ident: $ty:ty),*)) => {
		fn $fn_name(manager: &RustConfigManager, $($arg: $ty),*) { manager.$method($($arg),*); }
	};
	(mut $fn_name:ident, $method:ident($($arg:ident: $ty:ty),*) -> $ret:ty) => {
		fn $fn_name(manager: &mut RustConfigManager, $($arg: $ty),*) -> $ret { manager.$method($($arg),*) }
	};
	($fn_name:ident, $method:ident($($arg:ident: $ty:ty),*) -> $ret:ty) => {
		fn $fn_name(manager: &RustConfigManager, $($arg: $ty),*) -> $ret { manager.$method($($arg),*) }
	};
}

fn config_manager_new() -> Box<RustConfigManager> {
	Box::new(RustConfigManager::new())
}

ffi_wrapper!(mut config_manager_initialize, initialize -> bool);
ffi_wrapper!(config_manager_flush, flush);
ffi_wrapper!(mut config_manager_shutdown, shutdown);
ffi_wrapper!(config_manager_get_string, get_string(key: &str, default_value: &str) -> String);
ffi_wrapper!(config_manager_get_bool, get_bool(key: &str, default_value: bool) -> bool);
ffi_wrapper!(config_manager_get_int, get_int(key: &str, default_value: i32) -> i32);
ffi_wrapper!(mut config_manager_set_string, set_string(key: &str, value: &str));
ffi_wrapper!(mut config_manager_set_bool, set_bool(key: &str, value: bool));
ffi_wrapper!(mut config_manager_set_int, set_int(key: &str, value: i32));
ffi_wrapper!(config_manager_get_app_string, get_app_string(key: &str, default_value: &str) -> String);
ffi_wrapper!(config_manager_get_app_bool, get_app_bool(key: &str, default_value: bool) -> bool);
ffi_wrapper!(config_manager_get_app_int, get_app_int(key: &str, default_value: i32) -> i32);
ffi_wrapper!(mut config_manager_set_app_string, set_app_string(key: &str, value: &str));
ffi_wrapper!(mut config_manager_set_app_bool, set_app_bool(key: &str, value: bool));
ffi_wrapper!(mut config_manager_set_app_int, set_app_int(key: &str, value: i32));

fn config_manager_get_doc_string(manager: &RustConfigManager, path: &str, key: &str, default_value: &str) -> String {
	manager.get_document_string(path, key, default_value)
}

fn config_manager_get_doc_bool(manager: &RustConfigManager, path: &str, key: &str, default_value: bool) -> bool {
	manager.get_document_bool(path, key, default_value)
}

fn config_manager_get_doc_int(manager: &RustConfigManager, path: &str, key: &str, default_value: i64) -> i64 {
	manager.get_document_int(path, key, default_value)
}

fn config_manager_set_doc_string(manager: &mut RustConfigManager, path: &str, key: &str, value: &str) {
	manager.set_document_string(path, key, value);
}

fn config_manager_set_doc_bool(manager: &mut RustConfigManager, path: &str, key: &str, value: bool) {
	manager.set_document_bool(path, key, value);
}

fn config_manager_set_doc_int(manager: &mut RustConfigManager, path: &str, key: &str, value: i64) {
	manager.set_document_int(path, key, value);
}

ffi_wrapper!(mut config_manager_add_recent_document, add_recent_document(path: &str));
ffi_wrapper!(config_manager_get_recent_documents, get_recent_documents -> Vec<String>);
ffi_wrapper!(mut config_manager_clear_recent_documents, clear_recent_documents);
ffi_wrapper!(mut config_manager_add_opened_document, add_opened_document(path: &str));
ffi_wrapper!(mut config_manager_remove_opened_document, remove_opened_document(path: &str));
ffi_wrapper!(mut config_manager_clear_opened_documents, clear_opened_documents);
ffi_wrapper!(mut config_manager_set_document_position, set_document_position(path: &str, position: i64));
ffi_wrapper!(config_manager_get_document_position, get_document_position(path: &str) -> i64);

fn config_manager_set_navigation_history(
	manager: &mut RustConfigManager,
	path: &str,
	history: &[i64],
	history_index: usize,
) {
	manager.set_navigation_history(path, history, history_index);
}

fn config_manager_get_navigation_history(manager: &RustConfigManager, path: &str) -> ffi::FfiNavigationHistory {
	let history: NavigationHistory = manager.get_navigation_history(path);
	ffi::FfiNavigationHistory { positions: history.positions, index: history.index }
}

ffi_wrapper!(mut config_manager_set_document_opened, set_document_opened(path: &str, opened: bool));
ffi_wrapper!(config_manager_get_document_opened, get_document_opened(path: &str) -> bool);
ffi_wrapper!(mut config_manager_remove_document_history, remove_document_history(path: &str));
ffi_wrapper!(mut config_manager_remove_navigation_history, remove_navigation_history(path: &str));
ffi_wrapper!(config_manager_get_all_opened_documents, get_all_opened_documents -> Vec<String>);
ffi_wrapper!(config_manager_get_all_documents, get_all_documents -> Vec<String>);

fn config_manager_add_bookmark(manager: &mut RustConfigManager, path: &str, start: i64, end: i64, note: &str) {
	manager.add_bookmark(path, start, end, note);
}

fn config_manager_remove_bookmark(manager: &mut RustConfigManager, path: &str, start: i64, end: i64) {
	manager.remove_bookmark(path, start, end);
}

fn config_manager_toggle_bookmark(manager: &mut RustConfigManager, path: &str, start: i64, end: i64, note: &str) {
	manager.toggle_bookmark(path, start, end, note);
}

fn config_manager_update_bookmark_note(manager: &mut RustConfigManager, path: &str, start: i64, end: i64, note: &str) {
	manager.update_bookmark_note(path, start, end, note);
}

fn config_manager_get_bookmarks(manager: &RustConfigManager, path: &str) -> Vec<ffi::FfiBookmark> {
	manager.get_bookmarks(path).into_iter().map(Into::into).collect()
}

ffi_wrapper!(mut config_manager_clear_bookmarks, clear_bookmarks(path: &str));

fn config_manager_get_next_bookmark(
	manager: &RustConfigManager,
	path: &str,
	current_position: i64,
) -> ffi::FfiBookmark {
	manager.get_next_bookmark(path, current_position).into()
}

fn config_manager_get_previous_bookmark(
	manager: &RustConfigManager,
	path: &str,
	current_position: i64,
) -> ffi::FfiBookmark {
	manager.get_previous_bookmark(path, current_position).into()
}

ffi_wrapper!(mut config_manager_set_document_format, set_document_format(path: &str, format: &str));
ffi_wrapper!(config_manager_get_document_format, get_document_format(path: &str) -> String);
ffi_wrapper!(mut config_manager_set_document_password, set_document_password(path: &str, password: &str));
ffi_wrapper!(config_manager_get_document_password, get_document_password(path: &str) -> String);
ffi_wrapper!(config_manager_needs_migration, needs_migration -> bool);
ffi_wrapper!(mut config_manager_migrate_config, migrate_config -> bool);
ffi_wrapper!(config_manager_export_document_settings, export_document_settings(doc_path: &str, export_path: &str));
ffi_wrapper!(mut config_manager_import_document_settings, import_document_settings(path: &str));
ffi_wrapper!(mut config_manager_import_settings_from_file, import_settings_from_file(doc_path: &str, import_path: &str));

impl From<Bookmark> for ffi::FfiBookmark {
	fn from(bookmark: Bookmark) -> Self {
		Self { start: bookmark.start, end: bookmark.end, note: bookmark.note }
	}
}

fn check_for_updates(current_version: &str, is_installer: bool) -> ffi::UpdateResult {
	match update_module::check_for_updates(current_version, is_installer) {
		Ok(outcome) => match outcome {
			update_module::UpdateCheckOutcome::UpdateAvailable(result) => ffi::UpdateResult {
				status: UpdateStatus::Available,
				http_status: result.http_status,
				latest_version: result.latest_version,
				download_url: result.download_url,
				release_notes: result.release_notes,
				error_message: String::new(),
			},
			update_module::UpdateCheckOutcome::UpToDate(latest_version) => ffi::UpdateResult {
				status: UpdateStatus::UpToDate,
				http_status: 0,
				latest_version,
				download_url: String::new(),
				release_notes: String::new(),
				error_message: String::new(),
			},
		},
		Err(err) => {
			let (status, http_status) = match &err {
				update_module::UpdateError::InvalidVersion(_) => (UpdateStatus::InvalidInput, 0),
				update_module::UpdateError::HttpError(code) => (UpdateStatus::HttpError, *code),
				update_module::UpdateError::NetworkError(_) => (UpdateStatus::NetworkError, 0),
				update_module::UpdateError::InvalidResponse(_) => (UpdateStatus::InvalidResponse, 0),
				update_module::UpdateError::NoDownload(_) => (UpdateStatus::NoDownload, 0),
			};
			ffi::UpdateResult {
				status,
				http_status,
				latest_version: String::new(),
				download_url: String::new(),
				release_notes: String::new(),
				error_message: err.to_string(),
			}
		}
	}
}

fn remove_soft_hyphens(input: &str) -> String {
	text::remove_soft_hyphens(input)
}

fn url_decode(encoded: &str) -> String {
	text::url_decode(encoded)
}

fn collapse_whitespace(input: &str) -> String {
	text::collapse_whitespace(input)
}

fn trim_string(input: &str) -> String {
	text::trim_string(input)
}

fn markdown_to_text(input: &str) -> String {
	text::markdown_to_text(input)
}

fn convert_to_utf8(input: &[u8]) -> String {
	encoding::convert_to_utf8(input)
}

fn read_zip_entry(zip_path: &str, entry_name: &str) -> Result<String, String> {
	let file = File::open(zip_path).map_err(|e| format!("Failed to open ZIP file: {e}"))?;
	let mut archive = zip::ZipArchive::new(file).map_err(|e| format!("Failed to read ZIP archive: {e}"))?;
	zip_module::read_zip_entry_by_name(&mut archive, entry_name).map_err(|e| e.to_string())
}

fn find_zip_entry(zip_path: &str, entry_name: &str) -> Result<usize, String> {
	let file = File::open(zip_path).map_err(|e| format!("Failed to open ZIP file: {e}"))?;
	let mut archive = zip::ZipArchive::new(file).map_err(|e| format!("Failed to read ZIP archive: {e}"))?;
	zip_module::find_zip_entry(&mut archive, entry_name)
		.ok_or_else(|| format!("Entry '{entry_name}' not found in ZIP archive"))
}

fn get_available_parsers() -> Vec<ffi::ParserInfo> {
	let parsers = parser::get_all_parsers();
	parsers
		.into_iter()
		.map(|p| ffi::ParserInfo { name: p.name, extensions: p.extensions, flags: p.flags.bits() })
		.collect()
}

fn parser_supported_wildcards() -> String {
	parser::build_file_filter_string()
}

fn parser_supports_extension(extension: &str) -> bool {
	parser::parser_supports_extension(extension)
}

fn parser_error_info(message: &str) -> ffi::ParserErrorInfo {
	let prefix = parser::PASSWORD_REQUIRED_ERROR_PREFIX;
	if let Some(rest) = message.strip_prefix(prefix) {
		return ffi::ParserErrorInfo { kind: ffi::ParserErrorKind::PasswordRequired, detail: rest.to_string() };
	}
	ffi::ParserErrorInfo { kind: ffi::ParserErrorKind::Generic, detail: message.to_string() }
}

fn get_parser_for_extension(extension: &str) -> Result<String, String> {
	parser::get_parser_name_for_extension(extension)
		.ok_or_else(|| format!("No parser found for extension: .{extension}"))
}

fn flatten_recursive_with_parents(
	items: &[TocItem],
	depth: i32,
	parent_index: i32,
	result: &mut Vec<ffi::FfiTocItemWithParent>,
) {
	for item in items {
		let current_index = i32::try_from(result.len()).unwrap_or(-1);
		result.push(ffi::FfiTocItemWithParent {
			name: item.name.clone(),
			reference: item.reference.clone(),
			offset: item.offset,
			depth,
			parent_index,
		});
		flatten_recursive_with_parents(&item.children, depth + 1, current_index, result);
	}
}

fn flatten_toc_items_with_parents(items: &[TocItem]) -> Vec<ffi::FfiTocItemWithParent> {
	let mut result = Vec::new();
	flatten_recursive_with_parents(items, 0, -1, &mut result);
	result
}

const fn document_stats_to_ffi(stats: &crate::document::DocumentStats) -> ffi::FfiDocumentStats {
	ffi::FfiDocumentStats {
		word_count: stats.word_count,
		line_count: stats.line_count,
		char_count: stats.char_count,
		char_count_no_whitespace: stats.char_count_no_whitespace,
	}
}

fn parse_document_handle(
	file_path: &str,
	password: &str,
	forced_extension: &str,
) -> Result<Box<DocumentHandle>, String> {
	let mut context = ParserContext::new(file_path.to_string());
	if !password.is_empty() {
		context = context.with_password(password.to_string());
	}
	if !forced_extension.is_empty() {
		context = context.with_forced_extension(forced_extension.to_string());
	}
	let mut doc = parser::parse_document(&context).map_err(|e| e.to_string())?;
	doc.compute_stats();
	Ok(Box::new(DocumentHandle::new(doc)))
}

fn document_heading_tree(doc: &DocumentHandle, position: i64) -> ffi::FfiHeadingTree {
	let pos = usize::try_from(position.max(0)).unwrap_or(0);
	let mut last_indices = [-1; 7];
	let mut closest_index = -1;
	let mut items = Vec::new();
	for marker in
		doc.document().buffer.markers.iter().filter(|marker| crate::document::is_heading_marker(marker.marker_type))
	{
		let level = marker.level;
		if !(1..=6).contains(&level) {
			continue;
		}
		let parent_index = last_indices[(level - 1) as usize];
		let current_index = i32::try_from(items.len()).unwrap_or(-1);
		if marker.position <= pos {
			closest_index = current_index;
		}
		items.push(ffi::FfiHeadingTreeItem { offset: marker.position, level, text: marker.text.clone(), parent_index });
		for idx in level..=6 {
			last_indices[idx as usize] = current_index;
		}
	}
	ffi::FfiHeadingTree { items, closest_index }
}

fn reader_navigate(doc: &DocumentHandle, req: &ffi::NavRequest) -> ffi::NavResult {
	crate::reader_core::reader_navigate(doc, req)
}

fn reader_search(
	req: &str,
	needle: &str,
	start: i64,
	forward: bool,
	match_case: bool,
	whole_word: bool,
	regex: bool,
) -> i64 {
	crate::reader_core::reader_search(req, needle, start, forward, match_case, whole_word, regex)
}

fn reader_search_with_wrap(
	req: &str,
	needle: &str,
	start: i64,
	forward: bool,
	match_case: bool,
	whole_word: bool,
	regex: bool,
) -> ffi::FfiSearchResult {
	crate::reader_core::reader_search_with_wrap(req, needle, start, forward, match_case, whole_word, regex)
}

fn bookmark_navigate(
	manager: &ConfigManager,
	path: &str,
	position: i64,
	wrap: bool,
	next: bool,
	notes_only: bool,
) -> ffi::BookmarkNavResult {
	crate::reader_core::bookmark_navigate(manager, path, position, wrap, next, notes_only)
}

fn bookmark_note_at_position(manager: &ConfigManager, path: &str, position: i64) -> String {
	crate::reader_core::bookmark_note_at_position(manager, path, position)
}

fn get_filtered_bookmarks(
	manager: &ConfigManager,
	path: &str,
	current_pos: i64,
	filter: ffi::BookmarkFilterType,
) -> ffi::FfiFilteredBookmarks {
	crate::reader_core::get_filtered_bookmarks(manager, path, current_pos, filter)
}

fn get_filtered_bookmark_display_items(
	session: &DocumentSession,
	manager: &ConfigManager,
	path: &str,
	current_pos: i64,
	filter: ffi::BookmarkFilterType,
) -> ffi::FfiFilteredBookmarkDisplay {
	session.get_filtered_bookmark_display_items(manager, path, current_pos, filter)
}

fn get_sorted_document_list(
	config: &ConfigManager,
	open_paths: &[String],
	filter: &str,
) -> Vec<ffi::FfiDocumentListItem> {
	crate::config::get_sorted_document_list(config, open_paths, filter)
}

fn get_recent_documents_for_menu(config: &ConfigManager, limit: usize) -> Vec<ffi::FfiRecentDocument> {
	let docs = config.get_recent_documents();
	docs.into_iter()
		.take(limit)
		.map(|path| {
			let filename = Path::new(&path).file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
			ffi::FfiRecentDocument { path, filename }
		})
		.collect()
}

fn history_normalize(history: &[i64], history_index: usize) -> ffi::FfiNavigationHistory {
	crate::reader_core::history_normalize(history, history_index)
}

fn history_record_position(
	history: &[i64],
	history_index: usize,
	current_pos: i64,
	max_len: usize,
) -> ffi::FfiNavigationHistory {
	crate::reader_core::history_record_position(history, history_index, current_pos, max_len)
}

fn history_go_previous(
	history: &[i64],
	history_index: usize,
	current_pos: i64,
	max_len: usize,
) -> ffi::FfiHistoryNavResult {
	crate::reader_core::history_go_previous(history, history_index, current_pos, max_len)
}

fn history_go_next(
	history: &[i64],
	history_index: usize,
	current_pos: i64,
	max_len: usize,
) -> ffi::FfiHistoryNavResult {
	crate::reader_core::history_go_next(history, history_index, current_pos, max_len)
}

fn resolve_link(doc: &DocumentHandle, href: &str, current_position: i64) -> ffi::FfiLinkNavigation {
	crate::reader_core::resolve_link(doc, href, current_position)
}

use crate::session::{DocumentSession, LinkAction, NavigationResult};

fn nav_result_to_ffi(result: NavigationResult) -> ffi::FfiSessionNavResult {
	ffi::FfiSessionNavResult {
		found: result.found,
		wrapped: result.wrapped,
		offset: result.offset,
		marker_text: result.marker_text,
		marker_level: result.marker_level,
		marker_index: result.marker_index,
		not_supported: result.not_supported,
	}
}

fn session_new(file_path: &str, password: &str, forced_extension: &str) -> Result<Box<DocumentSession>, String> {
	DocumentSession::new(file_path, password, forced_extension).map(Box::new)
}

fn session_title(session: &DocumentSession) -> String {
	session.title()
}

fn session_author(session: &DocumentSession) -> String {
	session.author()
}

fn session_content(session: &DocumentSession) -> String {
	session.content()
}

fn session_file_path(session: &DocumentSession) -> String {
	session.file_path().to_string()
}

const fn session_parser_flags(session: &DocumentSession) -> u32 {
	session.parser_flags().bits()
}

fn session_get_history(session: &DocumentSession) -> ffi::FfiNavigationHistory {
	let (positions, index) = session.get_history();
	ffi::FfiNavigationHistory { positions: positions.to_vec(), index }
}

fn session_set_history(session: &mut DocumentSession, positions: &[i64], index: usize) {
	session.set_history(positions, index);
}

fn session_record_position(session: &mut DocumentSession, position: i64) {
	session.record_position(position);
}

fn session_navigate_section(
	session: &DocumentSession,
	position: i64,
	wrap: bool,
	next: bool,
) -> ffi::FfiSessionNavResult {
	nav_result_to_ffi(session.navigate_section(position, wrap, next))
}

fn session_navigate_heading(
	session: &DocumentSession,
	position: i64,
	wrap: bool,
	next: bool,
	level: i32,
) -> ffi::FfiSessionNavResult {
	nav_result_to_ffi(session.navigate_heading(position, wrap, next, level))
}

fn session_navigate_page(session: &DocumentSession, position: i64, wrap: bool, next: bool) -> ffi::FfiSessionNavResult {
	nav_result_to_ffi(session.navigate_page(position, wrap, next))
}

fn session_navigate_link(session: &DocumentSession, position: i64, wrap: bool, next: bool) -> ffi::FfiSessionNavResult {
	nav_result_to_ffi(session.navigate_link(position, wrap, next))
}

fn session_navigate_list(session: &DocumentSession, position: i64, wrap: bool, next: bool) -> ffi::FfiSessionNavResult {
	nav_result_to_ffi(session.navigate_list(position, wrap, next))
}

fn session_navigate_list_item(
	session: &DocumentSession,
	position: i64,
	wrap: bool,
	next: bool,
) -> ffi::FfiSessionNavResult {
	nav_result_to_ffi(session.navigate_list_item(position, wrap, next))
}

fn session_navigate_table(
	session: &DocumentSession,
	position: i64,
	wrap: bool,
	next: bool,
) -> ffi::FfiSessionNavResult {
	nav_result_to_ffi(session.navigate_table(position, wrap, next))
}

fn session_navigate_bookmark(
	session: &DocumentSession,
	config: &ConfigManager,
	position: i64,
	wrap: bool,
	next: bool,
) -> ffi::FfiSessionNavResult {
	nav_result_to_ffi(session.navigate_bookmark(config, position, wrap, next))
}

fn session_navigate_bookmark_display(
	session: &DocumentSession,
	config: &ConfigManager,
	position: i64,
	wrap: bool,
	next: bool,
	notes_only: bool,
) -> ffi::FfiBookmarkNavDisplay {
	session.navigate_bookmark_display(config, position, wrap, next, notes_only)
}

fn session_bookmark_display_at_position(
	session: &DocumentSession,
	config: &ConfigManager,
	position: i64,
) -> ffi::FfiBookmarkDisplayAtPosition {
	session.bookmark_display_at_position(config, position)
}

fn session_navigate_note(
	session: &DocumentSession,
	config: &ConfigManager,
	position: i64,
	wrap: bool,
	next: bool,
) -> ffi::FfiSessionNavResult {
	nav_result_to_ffi(session.navigate_note(config, position, wrap, next))
}

fn session_link_list(session: &DocumentSession, position: i64) -> ffi::FfiLinkList {
	session.link_list(position)
}

fn session_history_go_back(session: &mut DocumentSession, current_pos: i64) -> ffi::FfiSessionNavResult {
	nav_result_to_ffi(session.history_go_back(current_pos))
}

fn session_history_go_forward(session: &mut DocumentSession, current_pos: i64) -> ffi::FfiSessionNavResult {
	nav_result_to_ffi(session.history_go_forward(current_pos))
}

fn session_activate_link(session: &mut DocumentSession, position: i64) -> ffi::FfiLinkActivationResult {
	let result = session.activate_link(position);
	ffi::FfiLinkActivationResult {
		found: result.found,
		action: match result.action {
			LinkAction::Internal => ffi::FfiLinkAction::Internal,
			LinkAction::External => ffi::FfiLinkAction::External,
			LinkAction::NotFound => ffi::FfiLinkAction::NotFound,
		},
		offset: result.offset,
		url: result.url,
	}
}

fn session_get_table_at_position(session: &DocumentSession, position: i64) -> String {
	session.get_table_at_position(position).unwrap_or_default()
}

fn session_get_current_section_path(session: &DocumentSession, position: i64) -> String {
	session.get_current_section_path(position).unwrap_or_default()
}

fn session_extract_resource(session: &DocumentSession, resource_path: &str, output_path: &str) -> Result<bool, String> {
	session.extract_resource(resource_path, output_path).map_err(|e| e.to_string())
}

const fn session_handle(session: &DocumentSession) -> &DocumentHandle {
	session.handle()
}

fn session_get_status_info(session: &DocumentSession, position: i64) -> ffi::FfiStatusInfo {
	let info = session.get_status_info(position);
	ffi::FfiStatusInfo {
		line_number: info.line_number,
		character_number: info.character_number,
		percentage: info.percentage,
		total_chars: info.total_chars,
	}
}

fn session_stats(session: &DocumentSession) -> ffi::FfiDocumentStats {
	document_stats_to_ffi(session.stats())
}

fn session_page_count(session: &DocumentSession) -> usize {
	session.page_count()
}

fn session_current_page(session: &DocumentSession, position: i64) -> i32 {
	session.current_page(position)
}

fn session_page_offset(session: &DocumentSession, page_index: i32) -> i64 {
	session.page_offset(page_index)
}

fn session_export_content(session: &DocumentSession, output_path: &str) -> Result<(), String> {
	session.export_content(output_path).map_err(|e| e.to_string())
}

fn session_get_text_range(session: &DocumentSession, start: i64, end: i64) -> String {
	session.get_text_range(start, end)
}

fn session_get_line_text(session: &DocumentSession, position: i64) -> String {
	session.get_line_text(position)
}

fn session_toc_items_with_parents(session: &DocumentSession) -> Vec<ffi::FfiTocItemWithParent> {
	flatten_toc_items_with_parents(&session.handle().document().toc_items)
}

fn session_find_closest_toc_offset(session: &DocumentSession, position: usize) -> usize {
	session.handle().find_closest_toc_offset(position)
}

fn session_heading_tree(session: &DocumentSession, position: i64) -> ffi::FfiHeadingTree {
	document_heading_tree(session.handle(), position)
}

fn is_heading_marker_type(marker_type: i32) -> bool {
	crate::document::MarkerType::try_from(marker_type).map(crate::document::is_heading_marker).unwrap_or(false)
}
