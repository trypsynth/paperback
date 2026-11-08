use crate::document::{DocLen, DocPos, DocumentBuffer, Marker, MarkerType};

pub(in crate::parser) struct PendingLink {
	pub text: String,
	pub reference: String,
}

/// Searches from the previous link on, mirroring pdfium's order; a link whose text did not survive (a URL in a running header) places nothing.
pub(in crate::parser) fn place_links(
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
