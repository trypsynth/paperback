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

/// Translates library-internal strings (e.g. document content labels, parser error messages).
/// Delegates to `patois`'s process-global registry: when the GUI binary calls `patois::init`
/// at startup, these strings come back translated automatically. Non-GUI consumers (CLI,
/// mobile bindings) that never call `patois::init` get the English source string back unchanged.
pub(crate) fn t(s: &str) -> String {
	patois::t(s)
}
