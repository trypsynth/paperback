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

#[cfg(feature = "uniffi")]
uniffi::include_scaffolding!("paperback");

pub fn set_pdfium_library_path(path: String) {
	pdfium::set_library_location(&path);
}

/// Translates library-internal strings (e.g. document content labels, parser error messages).
///
/// `patois`'s "ui" feature (which pulls in wxdragon) is never enabled here, so this stays free
/// of desktop UI dependencies for the CLI and mobile FFI consumers of this crate.
pub(crate) fn t(s: &str) -> String {
	patois::t(s)
}
