pub mod audio;
pub mod config;
pub mod document;
pub mod export;
pub mod ffi_config;
pub mod ocr;
pub mod parser;
mod pdfium;
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
		AudioClipFfi, AudioCursorFfi, AudioPointFfi, DocumentError, DocumentSession, DocumentStatsFfi, HeadingTreeFfi,
		HeadingTreeItemFfi, LineMarker, LinkAction, LinkActivationResult, LinkListFfi, LinkListItemFfi,
		SearchOptionsFfi, SearchResultFfi, SegmentDirectionFfi, SegmentTypeFfi, StatusInfo, TextSegmentFfi, TocEntry,
		WHOLE_DOCUMENT_DISPLAY_LEN, WINDOW_DISPLAY_LEN,
	},
};

#[cfg(feature = "uniffi")]
uniffi::setup_scaffolding!("paperback");

// `path: String` (not `&str`) because that is the signature UniFFI generates for.
#[cfg_attr(feature = "uniffi", uniffi::export)]
#[allow(clippy::needless_pass_by_value)]
pub fn set_pdfium_library_path(path: String) {
	pdfium::set_library_path(&path);
}

pub(crate) use patois::t;
