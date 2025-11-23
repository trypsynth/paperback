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

	pub struct FfiMarker {
		pub marker_type: i32,
		pub position: usize,
		pub text: String,
		pub reference: String,
		pub level: i32,
	}

	pub struct FfiTocItem {
		pub name: String,
		pub reference: String,
		pub offset: usize,
		pub depth: i32,
	}

	pub struct FfiDocumentStats {
		pub word_count: usize,
		pub line_count: usize,
		pub char_count: usize,
		pub char_count_no_whitespace: usize,
	}

	pub struct FfiIdPosition {
		pub id: String,
		pub offset: usize,
	}

	pub struct FfiManifestItem {
		pub id: String,
		pub path: String,
	}

	pub struct FfiHeadingInfo {
		pub offset: usize,
		pub level: i32,
		pub text: String,
	}

	pub struct FfiXmlConversion {
		pub text: String,
		pub headings: Vec<FfiHeadingInfo>,
		pub section_offsets: Vec<usize>,
		pub id_positions: Vec<FfiIdPosition>,
	}

	pub struct FfiDocument {
		pub title: String,
		pub author: String,
		pub content: String,
		pub markers: Vec<FfiMarker>,
		pub toc_items: Vec<FfiTocItem>,
		pub stats: FfiDocumentStats,
		pub id_positions: Vec<FfiIdPosition>,
		pub spine_items: Vec<String>,
		pub manifest_items: Vec<FfiManifestItem>,
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

	extern "Rust" {
		type ConfigManager;
		type DocumentHandle;

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
		fn config_manager_rebuild_recent_documents(manager: &mut ConfigManager);
		fn config_manager_add_opened_document(manager: &mut ConfigManager, path: &str);
		fn config_manager_remove_opened_document(manager: &mut ConfigManager, path: &str);
		fn config_manager_get_opened_documents(manager: &ConfigManager) -> Vec<String>;
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
		fn config_manager_get_closest_bookmark(
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
		fn parse_document_handle(file_path: &str, password: &str) -> Result<Box<DocumentHandle>>;
		fn document_title(doc: &DocumentHandle) -> String;
		fn document_author(doc: &DocumentHandle) -> String;
		fn document_content(doc: &DocumentHandle) -> String;
		fn document_length(doc: &DocumentHandle) -> usize;
		fn document_stats(doc: &DocumentHandle) -> FfiDocumentStats;
		fn document_toc_items(doc: &DocumentHandle) -> Vec<FfiTocItem>;
		fn document_markers(doc: &DocumentHandle) -> Vec<FfiMarker>;
		fn document_heading_markers(doc: &DocumentHandle, level: i32) -> Vec<FfiHeadingInfo>;
		fn document_find_closest_toc_offset(doc: &DocumentHandle, position: usize) -> usize;
		fn document_next_marker(doc: &DocumentHandle, position: i64, marker_type: i32) -> i32;
		fn document_previous_marker(doc: &DocumentHandle, position: i64, marker_type: i32) -> i32;
		fn document_current_marker(doc: &DocumentHandle, position: usize, marker_type: i32) -> i32;
		fn document_find_first_marker_after(doc: &DocumentHandle, position: i64, marker_type: i32) -> i32;
		fn document_marker_position(doc: &DocumentHandle, marker_index: i32) -> usize;
		fn document_marker_at(doc: &DocumentHandle, marker_index: i32) -> FfiMarker;
		fn document_count_markers(doc: &DocumentHandle, marker_type: i32) -> usize;
		fn document_marker_position_by_index(doc: &DocumentHandle, marker_type: i32, index: i32) -> usize;
		fn document_next_heading(doc: &DocumentHandle, position: i64, level: i32) -> i32;
		fn document_previous_heading(doc: &DocumentHandle, position: i64, level: i32) -> i32;
		fn document_heading_info(doc: &DocumentHandle, index: i32) -> FfiHeadingInfo;
		fn document_section_index(doc: &DocumentHandle, position: usize) -> i32;
		fn document_page_index(doc: &DocumentHandle, position: usize) -> i32;
		fn document_id_positions(doc: &DocumentHandle) -> Vec<FfiIdPosition>;
		fn document_spine_items(doc: &DocumentHandle) -> Vec<String>;
		fn document_manifest_items(doc: &DocumentHandle) -> Vec<FfiManifestItem>;
		fn check_for_updates(current_version: &str, is_installer: bool) -> UpdateResult;
		fn remove_soft_hyphens(input: &str) -> String;
		fn url_decode(encoded: &str) -> String;
		fn collapse_whitespace(input: &str) -> String;
		fn trim_string(input: &str) -> String;
		fn convert_to_utf8(input: &[u8]) -> String;
		fn read_zip_entry(zip_path: &str, entry_name: &str) -> Result<String>;
		fn find_zip_entry(zip_path: &str, entry_name: &str) -> Result<usize>;
		fn get_available_parsers() -> Vec<ParserInfo>;
		fn parse_document(file_path: &str, password: &str) -> Result<FfiDocument>;
		fn get_parser_for_extension(extension: &str) -> Result<String>;
		fn convert_xml_to_text(content: &str) -> Result<FfiXmlConversion>;
	}
}

use std::fs::File;

use self::ffi::UpdateStatus;
use crate::{
	config::{Bookmark, ConfigManager as RustConfigManager, NavigationHistory},
	document::{DocumentHandle, MarkerType, ParserContext, TocItem},
	parser, update as update_module,
	utils::{encoding, text, zip as zip_module},
	xml_to_text::XmlToText,
};

type ConfigManager = crate::config::ConfigManager;

fn config_manager_new() -> Box<RustConfigManager> {
	Box::new(RustConfigManager::new())
}

fn config_manager_initialize(manager: &mut RustConfigManager) -> bool {
	manager.initialize()
}

fn config_manager_flush(manager: &RustConfigManager) {
	manager.flush();
}

fn config_manager_shutdown(manager: &mut RustConfigManager) {
	manager.shutdown();
}

fn config_manager_get_string(manager: &RustConfigManager, key: &str, default_value: &str) -> String {
	manager.get_string(key, default_value)
}

fn config_manager_get_bool(manager: &RustConfigManager, key: &str, default_value: bool) -> bool {
	manager.get_bool(key, default_value)
}

fn config_manager_get_int(manager: &RustConfigManager, key: &str, default_value: i32) -> i32 {
	manager.get_int(key, default_value)
}

fn config_manager_set_string(manager: &mut RustConfigManager, key: &str, value: &str) {
	manager.set_string(key, value);
}

fn config_manager_set_bool(manager: &mut RustConfigManager, key: &str, value: bool) {
	manager.set_bool(key, value);
}

fn config_manager_set_int(manager: &mut RustConfigManager, key: &str, value: i32) {
	manager.set_int(key, value);
}

fn config_manager_get_app_string(manager: &RustConfigManager, key: &str, default_value: &str) -> String {
	manager.get_app_string(key, default_value)
}

fn config_manager_get_app_bool(manager: &RustConfigManager, key: &str, default_value: bool) -> bool {
	manager.get_app_bool(key, default_value)
}

fn config_manager_get_app_int(manager: &RustConfigManager, key: &str, default_value: i32) -> i32 {
	manager.get_app_int(key, default_value)
}

fn config_manager_set_app_string(manager: &mut RustConfigManager, key: &str, value: &str) {
	manager.set_app_string(key, value);
}

fn config_manager_set_app_bool(manager: &mut RustConfigManager, key: &str, value: bool) {
	manager.set_app_bool(key, value);
}

fn config_manager_set_app_int(manager: &mut RustConfigManager, key: &str, value: i32) {
	manager.set_app_int(key, value);
}

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

fn config_manager_add_recent_document(manager: &mut RustConfigManager, path: &str) {
	manager.add_recent_document(path);
}

fn config_manager_get_recent_documents(manager: &RustConfigManager) -> Vec<String> {
	manager.get_recent_documents()
}

fn config_manager_clear_recent_documents(manager: &mut RustConfigManager) {
	manager.clear_recent_documents();
}

fn config_manager_rebuild_recent_documents(manager: &mut RustConfigManager) {
	manager.rebuild_recent_documents();
}

fn config_manager_add_opened_document(manager: &mut RustConfigManager, path: &str) {
	manager.add_opened_document(path);
}

fn config_manager_remove_opened_document(manager: &mut RustConfigManager, path: &str) {
	manager.remove_opened_document(path);
}

fn config_manager_get_opened_documents(manager: &RustConfigManager) -> Vec<String> {
	manager.get_opened_documents()
}

fn config_manager_clear_opened_documents(manager: &mut RustConfigManager) {
	manager.clear_opened_documents();
}

fn config_manager_set_document_position(manager: &mut RustConfigManager, path: &str, position: i64) {
	manager.set_document_position(path, position);
}

fn config_manager_get_document_position(manager: &RustConfigManager, path: &str) -> i64 {
	manager.get_document_position(path)
}

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

fn config_manager_set_document_opened(manager: &mut RustConfigManager, path: &str, opened: bool) {
	manager.set_document_opened(path, opened);
}

fn config_manager_get_document_opened(manager: &RustConfigManager, path: &str) -> bool {
	manager.get_document_opened(path)
}

fn config_manager_remove_document_history(manager: &mut RustConfigManager, path: &str) {
	manager.remove_document_history(path);
}

fn config_manager_remove_navigation_history(manager: &mut RustConfigManager, path: &str) {
	manager.remove_navigation_history(path);
}

fn config_manager_get_all_opened_documents(manager: &RustConfigManager) -> Vec<String> {
	manager.get_all_opened_documents()
}

fn config_manager_get_all_documents(manager: &RustConfigManager) -> Vec<String> {
	manager.get_all_documents()
}

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
	manager.get_bookmarks(path).into_iter().map(to_ffi_bookmark).collect()
}

fn config_manager_clear_bookmarks(manager: &mut RustConfigManager, path: &str) {
	manager.clear_bookmarks(path);
}

fn config_manager_get_next_bookmark(
	manager: &RustConfigManager,
	path: &str,
	current_position: i64,
) -> ffi::FfiBookmark {
	to_ffi_bookmark(manager.get_next_bookmark(path, current_position))
}

fn config_manager_get_previous_bookmark(
	manager: &RustConfigManager,
	path: &str,
	current_position: i64,
) -> ffi::FfiBookmark {
	to_ffi_bookmark(manager.get_previous_bookmark(path, current_position))
}

fn config_manager_get_closest_bookmark(
	manager: &RustConfigManager,
	path: &str,
	current_position: i64,
) -> ffi::FfiBookmark {
	to_ffi_bookmark(manager.get_closest_bookmark(path, current_position))
}

fn config_manager_set_document_format(manager: &mut RustConfigManager, path: &str, format: &str) {
	manager.set_document_format(path, format);
}

fn config_manager_get_document_format(manager: &RustConfigManager, path: &str) -> String {
	manager.get_document_format(path)
}

fn config_manager_set_document_password(manager: &mut RustConfigManager, path: &str, password: &str) {
	manager.set_document_password(path, password);
}

fn config_manager_get_document_password(manager: &RustConfigManager, path: &str) -> String {
	manager.get_document_password(path)
}

fn config_manager_needs_migration(manager: &RustConfigManager) -> bool {
	manager.needs_migration()
}

fn config_manager_migrate_config(manager: &mut RustConfigManager) -> bool {
	manager.migrate_config()
}

fn config_manager_export_document_settings(manager: &RustConfigManager, doc_path: &str, export_path: &str) {
	manager.export_document_settings(doc_path, export_path);
}

fn config_manager_import_document_settings(manager: &mut RustConfigManager, path: &str) {
	manager.import_document_settings(path);
}

fn config_manager_import_settings_from_file(manager: &mut RustConfigManager, doc_path: &str, import_path: &str) {
	manager.import_settings_from_file(doc_path, import_path);
}

fn to_ffi_bookmark(bookmark: Bookmark) -> ffi::FfiBookmark {
	ffi::FfiBookmark { start: bookmark.start, end: bookmark.end, note: bookmark.note }
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

fn parse_document(file_path: &str, password: &str) -> Result<ffi::FfiDocument, String> {
	let mut context = ParserContext::new(file_path.to_string());
	if !password.is_empty() {
		context = context.with_password(password.to_string());
	}
	let mut doc = parser::parse_document(&context).map_err(|e| e.to_string())?;
	doc.compute_stats();
	// Convert TOC items to flat list (cxx doesn't support recursive types easily)
	let toc_items = flatten_toc_items(&doc.toc_items);
	Ok(ffi::FfiDocument {
		title: doc.title,
		author: doc.author,
		content: doc.buffer.content,
		markers: doc
			.buffer
			.markers
			.into_iter()
			.map(|m| ffi::FfiMarker {
				marker_type: m.marker_type.to_int(),
				position: m.position,
				text: m.text,
				reference: m.reference,
				level: m.level,
			})
			.collect(),
		toc_items,
		stats: ffi::FfiDocumentStats {
			word_count: doc.stats.word_count,
			line_count: doc.stats.line_count,
			char_count: doc.stats.char_count,
			char_count_no_whitespace: doc.stats.char_count_no_whitespace,
		},
		id_positions: doc
			.id_positions
			.iter()
			.map(|(id, offset)| ffi::FfiIdPosition { id: id.clone(), offset: *offset })
			.collect(),
		spine_items: doc.spine_items.clone(),
		manifest_items: doc
			.manifest_items
			.iter()
			.map(|(id, path)| ffi::FfiManifestItem { id: id.clone(), path: path.clone() })
			.collect(),
	})
}

fn get_parser_for_extension(extension: &str) -> Result<String, String> {
	parser::get_parser_name_for_extension(extension)
		.ok_or_else(|| format!("No parser found for extension: .{extension}"))
}

fn flatten_recursive(items: &[TocItem], depth: i32, result: &mut Vec<ffi::FfiTocItem>) {
	for item in items {
		result.push(ffi::FfiTocItem {
			name: item.name.clone(),
			reference: item.reference.clone(),
			offset: item.offset,
			depth,
		});
		flatten_recursive(&item.children, depth + 1, result);
	}
}

fn flatten_toc_items(items: &[TocItem]) -> Vec<ffi::FfiTocItem> {
	let mut result = Vec::new();
	flatten_recursive(items, 0, &mut result);
	result
}

fn convert_xml_to_text(content: &str) -> Result<ffi::FfiXmlConversion, String> {
	let mut converter = XmlToText::new();
	if !converter.convert(content) {
		return Err("Failed to parse XML content".to_string());
	}
	let headings = converter
		.get_headings()
		.iter()
		.map(|heading| ffi::FfiHeadingInfo { offset: heading.offset, level: heading.level, text: heading.text.clone() })
		.collect();
	let id_positions = converter
		.get_id_positions()
		.iter()
		.map(|(id, offset)| ffi::FfiIdPosition { id: id.clone(), offset: *offset })
		.collect();
	Ok(ffi::FfiXmlConversion {
		text: converter.get_text(),
		headings,
		section_offsets: converter.get_section_offsets().to_vec(),
		id_positions,
	})
}

const fn marker_type_from_i32(value: i32) -> Option<MarkerType> {
	MarkerType::from_int(value)
}

fn document_marker_to_ffi(marker: &crate::document::Marker) -> ffi::FfiMarker {
	ffi::FfiMarker {
		marker_type: marker.marker_type.to_int(),
		position: marker.position,
		text: marker.text.clone(),
		reference: marker.reference.clone(),
		level: marker.level,
	}
}

const fn document_stats_to_ffi(stats: &crate::document::DocumentStats) -> ffi::FfiDocumentStats {
	ffi::FfiDocumentStats {
		word_count: stats.word_count,
		line_count: stats.line_count,
		char_count: stats.char_count,
		char_count_no_whitespace: stats.char_count_no_whitespace,
	}
}

fn opt_usize_to_i32(value: Option<usize>) -> i32 {
	value.and_then(|v| i32::try_from(v).ok()).unwrap_or(-1)
}

fn parse_document_handle(file_path: &str, password: &str) -> Result<Box<DocumentHandle>, String> {
	let mut context = ParserContext::new(file_path.to_string());
	if !password.is_empty() {
		context = context.with_password(password.to_string());
	}
	let mut doc = parser::parse_document(&context).map_err(|e| e.to_string())?;
	doc.compute_stats();
	Ok(Box::new(DocumentHandle::new(doc)))
}

fn document_title(doc: &DocumentHandle) -> String {
	doc.document().title.clone()
}

fn document_author(doc: &DocumentHandle) -> String {
	doc.document().author.clone()
}

fn document_content(doc: &DocumentHandle) -> String {
	doc.document().buffer.content.clone()
}

fn document_length(doc: &DocumentHandle) -> usize {
	doc.document().buffer.content.chars().count()
}

const fn document_stats(doc: &DocumentHandle) -> ffi::FfiDocumentStats {
	document_stats_to_ffi(&doc.document().stats)
}

fn document_toc_items(doc: &DocumentHandle) -> Vec<ffi::FfiTocItem> {
	flatten_toc_items(&doc.document().toc_items)
}

fn document_markers(doc: &DocumentHandle) -> Vec<ffi::FfiMarker> {
	doc.document().buffer.markers.iter().map(document_marker_to_ffi).collect()
}

fn document_heading_markers(doc: &DocumentHandle, level: i32) -> Vec<ffi::FfiHeadingInfo> {
	let level_filter = if level > 0 { Some(level) } else { None };
	doc.document()
		.buffer
		.markers
		.iter()
		.filter(|m| {
			matches!(
				m.marker_type,
				MarkerType::Heading1
					| MarkerType::Heading2
					| MarkerType::Heading3
					| MarkerType::Heading4
					| MarkerType::Heading5
					| MarkerType::Heading6
			)
		})
		.filter(|m| level_filter.is_none_or(|lvl| m.level == lvl))
		.map(|m| ffi::FfiHeadingInfo { offset: m.position, level: m.level, text: m.text.clone() })
		.collect()
}

fn document_find_closest_toc_offset(doc: &DocumentHandle, position: usize) -> usize {
	doc.find_closest_toc_offset(position)
}

fn document_next_marker(doc: &DocumentHandle, position: i64, marker_type: i32) -> i32 {
	let Some(marker_type) = marker_type_from_i32(marker_type) else { return -1 };
	opt_usize_to_i32(doc.next_marker_index(position, marker_type))
}

fn document_previous_marker(doc: &DocumentHandle, position: i64, marker_type: i32) -> i32 {
	let Some(marker_type) = marker_type_from_i32(marker_type) else { return -1 };
	opt_usize_to_i32(doc.previous_marker_index(position, marker_type))
}

fn document_current_marker(doc: &DocumentHandle, position: usize, marker_type: i32) -> i32 {
	let Some(marker_type) = marker_type_from_i32(marker_type) else { return -1 };
	opt_usize_to_i32(doc.current_marker_index(position, marker_type))
}

fn document_find_first_marker_after(doc: &DocumentHandle, position: i64, marker_type: i32) -> i32 {
	let Some(marker_type) = marker_type_from_i32(marker_type) else { return -1 };
	opt_usize_to_i32(doc.find_first_marker_after(position, marker_type))
}

fn document_marker_position(doc: &DocumentHandle, marker_index: i32) -> usize {
	doc.marker_position(marker_index).unwrap_or(0)
}

fn document_marker_at(doc: &DocumentHandle, marker_index: i32) -> ffi::FfiMarker {
	doc.document().buffer.markers.get(usize::try_from(marker_index).unwrap_or(usize::MAX)).map_or(
		ffi::FfiMarker { marker_type: -1, position: 0, text: String::new(), reference: String::new(), level: 0 },
		document_marker_to_ffi,
	)
}

fn document_count_markers(doc: &DocumentHandle, marker_type: i32) -> usize {
	let Some(marker_type) = marker_type_from_i32(marker_type) else { return 0 };
	doc.count_markers_by_type(marker_type)
}

fn document_marker_position_by_index(doc: &DocumentHandle, marker_type: i32, index: i32) -> usize {
	let Some(marker_type) = marker_type_from_i32(marker_type) else { return 0 };
	doc.get_marker_position_by_index(marker_type, index).unwrap_or(0)
}

fn document_next_heading(doc: &DocumentHandle, position: i64, level: i32) -> i32 {
	let level_filter = if level > 0 { Some(level) } else { None };
	doc.next_heading_index(position, level_filter).unwrap_or(-1)
}

fn document_previous_heading(doc: &DocumentHandle, position: i64, level: i32) -> i32 {
	let level_filter = if level > 0 { Some(level) } else { None };
	doc.previous_heading_index(position, level_filter).unwrap_or(-1)
}

fn document_heading_info(doc: &DocumentHandle, index: i32) -> ffi::FfiHeadingInfo {
	doc.heading_info(index).map_or(ffi::FfiHeadingInfo { offset: 0, level: 0, text: String::new() }, |info| {
		ffi::FfiHeadingInfo { offset: info.offset, level: info.level, text: info.text }
	})
}

fn document_section_index(doc: &DocumentHandle, position: usize) -> i32 {
	doc.section_index(position).unwrap_or(-1)
}

fn document_page_index(doc: &DocumentHandle, position: usize) -> i32 {
	doc.page_index(position).unwrap_or(-1)
}

fn document_id_positions(doc: &DocumentHandle) -> Vec<ffi::FfiIdPosition> {
	doc.document()
		.id_positions
		.iter()
		.map(|(id, offset)| ffi::FfiIdPosition { id: id.clone(), offset: *offset })
		.collect()
}

fn document_spine_items(doc: &DocumentHandle) -> Vec<String> {
	doc.document().spine_items.clone()
}

fn document_manifest_items(doc: &DocumentHandle) -> Vec<ffi::FfiManifestItem> {
	doc.document()
		.manifest_items
		.iter()
		.map(|(id, path)| ffi::FfiManifestItem { id: id.clone(), path: path.clone() })
		.collect()
}
