#![warn(clippy::all, clippy::nursery, clippy::pedantic)]

use std::sync::OnceLock;

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

static TRANSLATE_FN: OnceLock<fn(&str) -> String> = OnceLock::new();

/// Registers the translation function used by [`t`]. This crate deliberately has no dependency
/// on `patois` itself (linking it here would drag `patois`'s "wx" feature — and with it desktop
/// wxWidgets — into every consumer of this crate via Cargo's feature unification, breaking the
/// Android and other non-desktop-GUI builds). Instead, the desktop GUI binary calls this once at
/// startup with `patois::t`. Non-GUI consumers (CLI, mobile bindings) that never call this keep
/// getting the English source string back unchanged.
pub fn set_translator(f: fn(&str) -> String) {
	let _ = TRANSLATE_FN.set(f);
}

/// Translates library-internal strings (e.g. document content labels, parser error messages).
/// Returns the English source string unchanged until [`set_translator`] has been called.
pub(crate) fn t(s: &str) -> String {
	TRANSLATE_FN.get().map_or_else(|| s.to_owned(), |f| f(s))
}
