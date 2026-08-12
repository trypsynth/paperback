pub mod audio;
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
	document::MarkerType,
	export::ExportFormat,
	ffi_config::ConfigManagerFfi,
	session::{
		DocumentError, DocumentSession, DocumentStatsFfi, HeadingTreeFfi, HeadingTreeItemFfi, LineMarker, LinkAction,
		LinkActivationResult, LinkListFfi, LinkListItemFfi, SearchOptionsFfi, SearchResultFfi, SegmentDirectionFfi,
		SegmentTypeFfi, StatusInfo, TextSegmentFfi, TocEntry,
	},
};

#[cfg(feature = "uniffi")]
uniffi::include_scaffolding!("paperback");

// `path: String` (not `&str`) because paperback.udl dictates this signature for UniFFI scaffolding.
#[allow(clippy::needless_pass_by_value)]
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
