//! Extracting a PDF page's links: implicit "web links" (bare URLs pdfium's own scanner
//! recognizes in the text) and explicit link annotations (internal page-destination links as
//! well as `/URI` actions).
//!
//! Reading a link out of pdfium and placing it in the assembled text are two separate steps.
//! pdfium reports a link's location as a character range in its own indexing rather than as a
//! display offset, so a link is found in the page's rendered text by searching for the text it
//! covers - and that text is not settled until the page has been through paragraph joining and
//! had its running headers removed, which happens after every page has been read (see
//! [`super::running`]). So [`collect_web_links`] and [`collect_annotation_links`] gather what
//! pdfium knows, and [`place_links`] turns it into markers once there is text to place them in.

use super::text::sanitize_pdf_text;
use crate::{
	document::{DocumentBuffer, Marker, MarkerType},
	pdfium::{LinkTarget, PdfPage, PdfTextPage},
	util::text::{collapse_whitespace, display_len, trim_string},
};

/// A link pdfium reported, waiting for the page text that will say where it sits.
pub(super) struct PendingLink {
	/// The text the link covers, as it will read in the assembled document.
	pub text: String,
	/// Where the link goes: a URL, or `#page_<n>` for a link within the document.
	pub reference: String,
}

/// Place each link at the first occurrence of its text at or after the previous link's, mirroring
/// the order pdfium reports them in. A link whose text did not survive into the page - a URL
/// inside a running header, say - simply places nothing.
pub(super) fn place_links(
	links: &[PendingLink],
	page_start_offset: usize,
	page_display_text: &str,
	buffer: &mut DocumentBuffer,
) {
	let mut last_search_pos = 0;
	for link in links {
		if let Some(pos) = page_display_text[last_search_pos..].find(&link.text) {
			let text_before = &page_display_text[last_search_pos..last_search_pos + pos];
			let marker_pos =
				page_start_offset + display_len(&page_display_text[..last_search_pos]) + display_len(text_before);
			let link_len = display_len(&link.text);
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
