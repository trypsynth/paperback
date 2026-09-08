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

use pdfium::{PdfiumDocument, PdfiumPage, PdfiumTextPage, lib};

use super::text::sanitize_pdf_text;
use crate::{
	document::{DocumentBuffer, Marker, MarkerType},
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

pub(super) fn collect_web_links(text_page: &PdfiumTextPage) -> Vec<PendingLink> {
	let mut pending = Vec::new();
	let Ok(links) = text_page.load_web_links() else { return pending };
	let count = lib().FPDFLink_CountWebLinks(&links);
	for i in 0..count {
		let mut start = 0;
		let mut char_count = 0;
		if lib().FPDFLink_GetTextRange(&links, i, &mut start, &mut char_count).is_ok() {
			let link_text = sanitize_pdf_text(&text_page.extract(start, char_count));
			let trimmed_link = trim_string(&collapse_whitespace(&link_text));
			if trimmed_link.is_empty() {
				continue;
			}
			let mut url_buffer = vec![0u16; 2048];
			let len = lib().FPDFLink_GetURL(&links, i, &mut url_buffer[0], 2048);
			if len > 0 {
				let url = String::from_utf16_lossy(&url_buffer[..(len as usize - 1)]);
				pending.push(PendingLink { text: trimmed_link, reference: url });
			}
		}
	}
	pending
}

pub(super) fn collect_annotation_links(
	page: &PdfiumPage,
	text_page: &PdfiumTextPage,
	document: &PdfiumDocument,
) -> Vec<PendingLink> {
	let mut pending = Vec::new();
	let annot_count = lib().FPDFPage_GetAnnotCount(page);
	for i in 0..annot_count {
		let annot_result = lib().FPDFPage_GetAnnot(page, i);
		if let Ok(annot) = annot_result
			&& lib().FPDFAnnot_GetSubtype(&annot) == pdfium::pdfium_constants::FPDF_ANNOT_LINK
		{
			let mut rect = pdfium::pdfium_types::FS_RECTF { left: 0.0, top: 0.0, right: 0.0, bottom: 0.0 };
			if lib().FPDFAnnot_GetRect(&annot, &mut rect).is_ok() {
				let mut text_buffer = vec![0u16; 2048];
				let len = lib().FPDFText_GetBoundedText(
					text_page,
					f64::from(rect.left),
					f64::from(rect.top),
					f64::from(rect.right),
					f64::from(rect.bottom),
					&mut text_buffer[0],
					2048,
				);
				if len > 0 {
					let text = sanitize_pdf_text(&String::from_utf16_lossy(&text_buffer[..(len as usize - 1)]));
					let trimmed_link = trim_string(&collapse_whitespace(&text));
					if trimmed_link.is_empty() {
						continue;
					}
					let mut url = String::new();
					let link_result = lib().FPDFAnnot_GetLink(&annot);
					if let Ok(link) = link_result {
						let action_result = lib().FPDFLink_GetAction(&link);
						if let Ok(action) = action_result {
							let action_type = lib().FPDFAction_GetType(&action);
							// PDFACTION_URI is 3
							if action_type == 3 {
								let mut uri_buffer = vec![0u8; 2048];
								let uri_len =
									lib().FPDFAction_GetURIPath(document, &action, Some(&mut uri_buffer), 2048);
								if uri_len > 0 {
									url = String::from_utf8_lossy(&uri_buffer[..(uri_len as usize - 1)]).to_string();
								}
							}
						}
						if url.is_empty() {
							let dest_result = lib().FPDFLink_GetDest(document, &link);
							let dest = dest_result.ok().or_else(|| {
								lib()
									.FPDFLink_GetAction(&link)
									.ok()
									.and_then(|action| lib().FPDFAction_GetDest(document, &action).ok())
							});
							if let Some(dest) = dest {
								let dest_page = lib().FPDFDest_GetDestPageIndex(document, &dest);
								if dest_page >= 0 {
									url = format!("#page_{dest_page}");
								}
							}
						}
					}
					if !url.is_empty() {
						pending.push(PendingLink { text: trimmed_link, reference: url });
					}
				}
			}
		}
	}
	pending
}
