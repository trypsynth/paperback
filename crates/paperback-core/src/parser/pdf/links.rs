//! A link's location comes as a character range in pdfium's own indexing, so it is placed later by searching for its text, once paragraph joining and running-header removal have settled the page.

use super::text::sanitize_pdf_text;
use crate::{
	parser::layout::PendingLink,
	pdf_engine::{LinkTarget, Page, TextPage},
	util::text::{collapse_whitespace, trim_string},
};

pub(super) fn collect_web_links(text_page: &TextPage) -> Vec<PendingLink> {
	let mut pending = Vec::new();
	for link in text_page.web_links() {
		let link_text = sanitize_pdf_text(&text_page.text(link.start..link.start + link.char_count));
		let trimmed_link = trim_string(&collapse_whitespace(&link_text));
		if trimmed_link.is_empty() {
			continue;
		}
		pending.push(PendingLink { text: trimmed_link, reference: link.url });
	}
	pending
}

pub(super) fn collect_annotation_links(page: &Page, text_page: &TextPage) -> Vec<PendingLink> {
	let mut pending = Vec::new();
	for link in page.annotation_links() {
		let text = sanitize_pdf_text(&text_page.text_in(link.rect));
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
