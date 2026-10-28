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
	let document = pdf_engine::PdfDocument::open(path, password).map_err(|error| error.to_string())?;
	let page = document.page(page).map_err(|error| error.to_string())?;
	let mut out = String::new();
	for i in 0..page.object_count() {
		if let Some(object) = page.object(i)
			&& object.kind() != pdf_engine::ObjectKind::Other
		{
			let _ = writeln!(out, "object {i} {:?} {:?}", object.kind(), object.bounds());
		}
	}
	for image in page.announced_images() {
		let _ = writeln!(out, "announced image {} top {:.2}", image.path, image.top);
	}
	fn walk(tag: &pdf_engine::Tag<'_>, depth: usize, out: &mut String) {
		let mut line = format!("{}{}", "  ".repeat(depth), tag.kind().unwrap_or_default());
		for i in 0..tag.child_count() {
			if let Some(child) = tag.child(i) {
				out.push_str(&line);
				out.push('\n');
				line = "  ".repeat(depth) + "  ...";
				walk(&child, depth + 1, out);
			} else if let Some(mcid) = tag.child_mcid(i) {
				let _ = write!(line, " {mcid}");
			} else {
				line.push_str(" -");
			}
		}
		out.push_str(&line);
		out.push('\n');
	}
	if let Some(tree) = page.tags() {
		for i in 0..tree.child_count() {
			match tree.child(i) {
				Some(tag) => walk(&tag, 0, &mut out),
				None => out.push_str("(root child not on page)\n"),
			}
		}
	}
	let Some(text) = page.text() else { return Ok(out) };
	for i in 0..text.char_count().unwrap_or(0) {
		let ch = char::from_u32(text.unicode_at(i))
			.map_or_else(|| format!("U+{:04X}", text.unicode_at(i)), |ch| format!("{ch:?}"));
		let (x, y) = text.char_origin(i).unwrap_or_default();
		let boxed = text
			.char_box(i)
			.map(|b| format!("{:.2} {:.2} {:.2} {:.2}", b.left, b.bottom, b.right, b.top))
			.unwrap_or_default();
		let font = text.font(i).map(|font| format!("{} {}", font.name, font.flags)).unwrap_or_default();
		let generated = if text.is_generated(i) { "gen" } else { "   " };
		let marks = text
			.text_object(i)
			.map(|object| {
				let ids: Vec<String> = (0..object.mark_count())
					.filter_map(|m| object.mark(m))
					.map(|mark| {
						format!(
							"{}:{}",
							mark.name().unwrap_or_default(),
							mark.param_int("MCID").map_or_else(String::new, |id| id.to_string())
						)
					})
					.collect();
				format!("mcid {:?} [{}]", object.marked_content_id(), ids.join(" "))
			})
			.unwrap_or_default();
		let _ = writeln!(
			out,
			"{i:>6} {ch:<10} {generated} ({x:.2}, {y:.2}) [{boxed}] size {:.2} {font} {marks}",
			text.font_size(i)
		);
	}
	Ok(out)
}
