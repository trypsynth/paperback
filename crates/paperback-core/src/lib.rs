#![warn(clippy::all, clippy::nursery, clippy::pedantic)]

pub mod config;
pub mod document;
pub mod export;
pub mod ffi_config;
pub mod parser;
pub mod reader_core;
pub mod session;
pub mod types;
pub mod util;
pub mod version;

pub use crate::{
	ffi_config::ConfigManagerFfi,
	session::{
		DocumentError, DocumentSession, DocumentStatsFfi, HeadingTreeFfi, HeadingTreeItemFfi, LineMarker, LinkAction,
		LinkActivationResult, LinkListFfi, LinkListItemFfi, MarkerTypeFfi, SearchOptionsFfi, SearchResultFfi,
		SegmentDirectionFfi, SegmentTypeFfi, StatusInfo, TextSegmentFfi, TocEntry,
	},
};

uniffi::include_scaffolding!("paperback");

pub fn set_pdfium_library_path(path: String) {
	pdfium::set_library_location(&path);
}

/// Minimal translation stub for library-internal strings (e.g. document content labels).
/// The GUI binary sets up the real wxWidgets translation system independently; strings
/// returned by this function are English only and are intended for non-GUI consumers
/// (CLI, mobile bindings) or for embedding into document content.
pub(crate) fn t(s: &str) -> String {
	s.to_owned()
}
