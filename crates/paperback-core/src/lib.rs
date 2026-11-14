pub mod audio;
pub mod config;
pub mod document;
pub mod export;
pub mod fetch;
pub mod ffi_config;
pub mod ocr;
pub mod parser;
mod pdf_engine;
pub mod reader_core;
pub mod reading;
pub mod session;
pub mod speech;
pub mod types;
pub mod util;
pub mod version;

pub use crate::{
	document::MarkerType,
	export::ExportFormat,
	ffi_config::ConfigManagerFfi,
	session::{
		AudioClipFfi, AudioCursorFfi, AudioPointFfi, ChapterProgressFfi, DocumentError, DocumentSession, DocumentStatsFfi, HeadingTreeFfi,
		HeadingTreeItemFfi, LineMarker, LinkAction, LinkActivationResult, LinkListFfi, LinkListItemFfi,
		SearchOptionsFfi, SearchResultFfi, SegmentDirectionFfi, SegmentTypeFfi, StatusInfo, TextSegmentFfi, TocEntry,
		WHOLE_DOCUMENT_LEN, WINDOW_LEN,
	},
};

#[cfg(feature = "uniffi")]
uniffi::setup_scaffolding!("paperback");

pub(crate) use patois::t;

/// For chasing down how a PDF's text layer was read; nothing in the app calls it.
#[doc(hidden)]
pub fn dump_pdf_page_chars(path: &str, password: Option<&str>, page: i32) -> Result<String, String> {
	use std::fmt::Write as _;
	let document = pdf_engine::Document::open(path, password).map_err(|error| error.to_string())?;
	let page = document.page(usize::try_from(page).unwrap_or(usize::MAX)).map_err(|error| error.to_string())?;
	let mut out = String::new();
	for (i, object) in page.objects().enumerate() {
		if object.kind() != pdf_engine::ObjectKind::Other {
			let _ = writeln!(out, "object {i} {:?} {:?}", object.kind(), object.bounds());
		}
	}
	for image in page.images() {
		let _ = writeln!(out, "image {} top {:.2}", image.id, image.bounds.top);
	}
	fn walk(tag: &pdf_engine::Tag<'_>, depth: usize, out: &mut String) {
		let mut line = format!("{}{}", "  ".repeat(depth), tag.kind().unwrap_or_default());
		for child in tag.children() {
			match child {
				pdf_engine::TagChild::Tag(child) => {
					out.push_str(&line);
					out.push('\n');
					line = "  ".repeat(depth) + "  ...";
					walk(&child, depth + 1, out);
				}
				pdf_engine::TagChild::Content(mcid) => {
					let _ = write!(line, " {mcid}");
				}
			}
		}
		out.push_str(&line);
		out.push('\n');
	}
	if let Some(tree) = page.tags() {
		for tag in tree.roots() {
			walk(&tag, 0, &mut out);
		}
	}
	let text = page.text();
	for (i, page_char) in text.chars().enumerate() {
		let ch = char::from_u32(page_char.unicode())
			.map_or_else(|| format!("U+{:04X}", page_char.unicode()), |ch| format!("{ch:?}"));
		let origin = page_char.origin();
		let (x, y) = (origin.x, origin.y);
		let b = page_char.bounds();
		let boxed = format!("{:.2} {:.2} {:.2} {:.2}", b.left, b.bottom, b.right, b.top);
		let font = page_char.font().map(|font| format!("{} {}", font.name, font.flags.bits())).unwrap_or_default();
		let generated = if page_char.is_generated() { "gen" } else { "   " };
		let marks = page_char
			.text_object()
			.map(|object| {
				let ids: Vec<String> = object
					.marks()
					.map(|mark| {
						format!(
							"{}:{}",
							mark.name().unwrap_or_default(),
							mark.mcid().map_or_else(String::new, |id| id.to_string())
						)
					})
					.collect();
				format!("mcid {:?} [{}]", object.marked_content_id(), ids.join(" "))
			})
			.unwrap_or_default();
		let _ = writeln!(
			out,
			"{i:>6} {ch:<10} {generated} ({x:.2}, {y:.2}) [{boxed}] size {:.2} {font} {marks}",
			page_char.font_size()
		);
	}
	Ok(out)
}
