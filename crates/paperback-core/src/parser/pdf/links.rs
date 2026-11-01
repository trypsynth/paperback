//! A link's location comes as a character range in pdfium's own indexing, so it is placed later by searching for its text, once paragraph joining and running-header removal have settled the page.

use super::text::sanitize_pdf_text;
use crate::{
	document::{DocLen, DocPos, DocumentBuffer, Marker, MarkerType},
	pdf_engine::{LinkTarget, PdfPage, PdfTextPage},
	util::text::{collapse_whitespace, trim_string},
};

pub(super) struct PendingLink {
	pub text: String,
	pub reference: String,
}

/// Searches from the previous link on, mirroring pdfium's order; a link whose text did not survive (a URL in a running header) places nothing.
pub(super) fn place_links(
	links: &[PendingLink],
	page_start_offset: DocPos,
	page_display_text: &str,
	buffer: &mut DocumentBuffer,
) {
	let mut last_search_pos = 0;
	for link in links {
		if let Some(pos) = page_display_text[last_search_pos..].find(&link.text) {
			let text_before = &page_display_text[last_search_pos..last_search_pos + pos];
			let marker_pos =
				page_start_offset + DocLen::of(&page_display_text[..last_search_pos]) + DocLen::of(text_before);
			let link_len = DocLen::of(&link.text);
			buffer.add_marker(
				Marker::new(MarkerType::Link, marker_pos)
					.with_text(link.text.clone())
					.with_reference(link.reference.clone())
					.with_length(link_len),
			);
			last_search_pos += pos + link.text.len();
		}
	}
}

pub(super) fn collect_web_links(text_page: &PdfTextPage) -> Vec<PendingLink> {
	let mut pending = Vec::new();
	for (start, char_count, url) in text_page.web_links() {
		let link_text = sanitize_pdf_text(&text_page.extract(start, char_count));
		let trimmed_link = trim_string(&collapse_whitespace(&link_text));
		if trimmed_link.is_empty() {
			continue;
		}
		pending.push(PendingLink { text: trimmed_link, reference: url });
	}
	pending
}

pub(super) fn collect_annotation_links(page: &PdfPage, text_page: &PdfTextPage) -> Vec<PendingLink> {
	let mut pending = Vec::new();
	for link in page.annotation_links() {
		let rect = link.rect;
		let text = sanitize_pdf_text(&text_page.bounded_text(
			f64::from(rect.left),
			f64::from(rect.top),
			f64::from(rect.right),
			f64::from(rect.bottom),
		));
		let trimmed_link = trim_string(&collapse_whitespace(&text));
		if trimmed_link.is_empty() {
			continue;
		}
		let reference = match link.target {
			LinkTarget::Url(url) => url,
			LinkTarget::Page(page_index) => format!("#page_{page_index}"),
		};
		pending.push(PendingLink { text: trimmed_link, reference });
	}
	pending
}
