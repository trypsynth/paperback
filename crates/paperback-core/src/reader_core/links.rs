//! Resolving an in-document link `href` ([`resolve_link`]) to a position, and finding the
//! nearest element id before a position for use as a web-view URL fragment.

use crate::{document::DocumentHandle, parser::is_external_url};

#[derive(Debug, Clone)]
pub struct LinkNavigation {
	pub found: bool,
	pub is_external: bool,
	pub offset: usize,
	pub url: String,
}

fn current_section_path(doc: &DocumentHandle, position: usize) -> Option<String> {
	let idx = doc.section_index(position)?;
	let idx = usize::try_from(idx).ok()?;
	let manifest_id = doc.document().spine_items.get(idx)?;
	doc.document().manifest_items.get(manifest_id).cloned()
}

fn find_fragment_offset(doc: &DocumentHandle, fragment: &str, scoped_path: Option<&str>) -> Option<usize> {
	let fragment = fragment.trim_start_matches('#');
	if fragment.is_empty() {
		return None;
	}
	if let Some(path) = scoped_path {
		let key = format!("{path}#{fragment}");
		if let Some(offset) = doc.document().id_positions.get(&key) {
			return Some(*offset);
		}
	}
	doc.document().id_positions.get(fragment).copied()
}

/// Percent-encodes an element id for use as a URL `#fragment`.
#[must_use]
pub fn encode_url_fragment(id: &str) -> String {
	const FRAGMENT_ENCODE_SET: &percent_encoding::AsciiSet =
		&percent_encoding::CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`').add(b'#').add(b'%');
	percent_encoding::utf8_percent_encode(id, FRAGMENT_ENCODE_SET).to_string()
}

/// Finds the id of the element closest at-or-before `position`, suitable as a URL
/// fragment when opening the current section in a web view.
///
/// For multi-section documents (epub) only ids scoped to the current section
/// (stored as `"{path}#{id}"` keys) are considered; for single-file documents
/// bare id keys are used instead.
#[must_use]
pub fn nearest_fragment_before(doc: &DocumentHandle, position: usize) -> Option<String> {
	let id_positions = &doc.document().id_positions;
	current_section_path(doc, position).map_or_else(
		|| {
			id_positions
				.iter()
				.filter(|&(key, &offset)| offset <= position && !key.contains('#'))
				.max_by_key(|&(_, &offset)| offset)
				.map(|(key, _)| key.clone())
		},
		|path| {
			let prefix = format!("{path}#");
			id_positions
				.iter()
				.filter(|&(key, &offset)| offset <= position && key.starts_with(&prefix))
				.max_by_key(|&(_, &offset)| offset)
				.map(|(key, _)| key[prefix.len()..].to_string())
		},
	)
}

fn find_manifest_id_for_path(doc: &DocumentHandle, path: &str) -> Option<String> {
	doc.document().manifest_items.iter().find_map(|(id, p)| if p == path { Some(id.clone()) } else { None })
}

fn spine_section_bounds(doc: &DocumentHandle, spine_index: usize) -> (usize, usize) {
	let start = i32::try_from(spine_index)
		.ok()
		.and_then(|idx| doc.get_marker_position_by_index(crate::document::MarkerType::SectionBreak, idx))
		.unwrap_or(0);
	let end = if spine_index + 1 < doc.document().spine_items.len() {
		i32::try_from(spine_index + 1)
			.ok()
			.and_then(|idx| doc.get_marker_position_by_index(crate::document::MarkerType::SectionBreak, idx))
			.unwrap_or_else(|| doc.document().buffer.content.len())
	} else {
		doc.document().buffer.content.len()
	};
	(start, end)
}

#[must_use]
pub fn resolve_link(doc: &DocumentHandle, href: &str, current_position: i64) -> LinkNavigation {
	let href_trimmed = href.trim();
	if href_trimmed.is_empty() {
		return LinkNavigation { found: false, is_external: false, offset: 0, url: String::new() };
	}
	if is_external_url(href_trimmed) {
		return LinkNavigation { found: true, is_external: true, offset: 0, url: href_trimmed.to_string() };
	}
	let current_section = current_section_path(doc, usize::try_from(current_position.max(0)).unwrap_or(0));
	if let Some(fragment) = href_trimmed.strip_prefix('#') {
		if let Some(offset) = find_fragment_offset(doc, fragment, current_section.as_deref()) {
			return LinkNavigation { found: true, is_external: false, offset, url: String::new() };
		}
		return LinkNavigation { found: false, is_external: false, offset: 0, url: String::new() };
	}
	let mut parts = href_trimmed.splitn(2, '#');
	let file_path = parts.next().unwrap_or_default();
	let fragment = parts.next().unwrap_or_default();
	if let Some(manifest_id) = find_manifest_id_for_path(doc, file_path)
		&& let Some(spine_index) = doc.document().spine_items.iter().position(|id| id == &manifest_id)
	{
		let (section_start, section_end) = spine_section_bounds(doc, spine_index);
		let mut offset = section_start;
		if !fragment.is_empty()
			&& let Some(found) = find_fragment_offset(doc, fragment, Some(file_path))
			&& found >= section_start
			&& found < section_end
		{
			offset = found;
		}
		return LinkNavigation { found: true, is_external: false, offset, url: String::new() };
	}
	// Fallback for formats like CHM that store id_positions with "{path}#{id}" keys.
	if !fragment.is_empty()
		&& let Some(offset) = find_fragment_offset(doc, fragment, Some(file_path))
	{
		return LinkNavigation { found: true, is_external: false, offset, url: String::new() };
	}
	// Direct file-path lookup (covers fragment-less CHM links and fallback for fragment misses).
	if let Some(&offset) = doc.document().id_positions.get(file_path) {
		return LinkNavigation { found: true, is_external: false, offset, url: String::new() };
	}
	if !fragment.is_empty()
		&& let Some(offset) = find_fragment_offset(doc, fragment, current_section.as_deref())
	{
		return LinkNavigation { found: true, is_external: false, offset, url: String::new() };
	}
	LinkNavigation { found: false, is_external: false, offset: 0, url: String::new() }
}

#[cfg(test)]
mod tests {
	use std::collections::HashMap;

	use super::*;
	use crate::document::{Document, DocumentBuffer, Marker, MarkerType};

	fn sample_link_doc_handle() -> DocumentHandle {
		let mut buffer = DocumentBuffer::with_content("x".repeat(220));
		buffer.add_marker(Marker::new(MarkerType::SectionBreak, 0));
		buffer.add_marker(Marker::new(MarkerType::SectionBreak, 100));
		let mut manifest_items = HashMap::new();
		manifest_items.insert("id1".to_string(), "chapter1.xhtml".to_string());
		manifest_items.insert("id2".to_string(), "chapter2.xhtml".to_string());
		let mut id_positions = HashMap::new();
		id_positions.insert("chapter1.xhtml#intro".to_string(), 10);
		id_positions.insert("chapter2.xhtml#target".to_string(), 120);
		id_positions.insert("global".to_string(), 180);
		let mut doc = Document::new();
		doc.set_buffer(buffer);
		doc.spine_items = vec!["id1".to_string(), "id2".to_string()];
		doc.manifest_items = manifest_items;
		doc.id_positions = id_positions;
		DocumentHandle::new(doc)
	}

	#[test]
	fn resolve_link_handles_empty_and_external_hrefs() {
		let doc = sample_link_doc_handle();
		let empty = resolve_link(&doc, "  ", 0);
		assert!(!empty.found);
		let ext = resolve_link(&doc, "https://example.com", 0);
		assert!(ext.found);
		assert!(ext.is_external);
		assert_eq!(ext.url, "https://example.com");
	}

	#[test]
	fn resolve_link_fragment_prefers_current_section_scoped_id() {
		let doc = sample_link_doc_handle();
		let result = resolve_link(&doc, "#target", 150);
		assert!(result.found);
		assert!(!result.is_external);
		assert_eq!(result.offset, 120);
	}

	#[test]
	fn resolve_link_fragment_falls_back_to_global_id() {
		let doc = sample_link_doc_handle();
		let result = resolve_link(&doc, "#global", 150);
		assert!(result.found);
		assert_eq!(result.offset, 180);
	}

	#[test]
	fn resolve_link_file_path_uses_section_start_when_no_fragment() {
		let doc = sample_link_doc_handle();
		let result = resolve_link(&doc, "chapter2.xhtml", 0);
		assert!(result.found);
		assert_eq!(result.offset, 100);
	}

	#[test]
	fn resolve_link_file_path_uses_fragment_within_section_bounds() {
		let doc = sample_link_doc_handle();
		let result = resolve_link(&doc, "chapter2.xhtml#target", 0);
		assert!(result.found);
		assert_eq!(result.offset, 120);
	}

	#[test]
	fn resolve_link_file_path_ignores_fragment_outside_section_bounds() {
		let doc = sample_link_doc_handle();
		let result = resolve_link(&doc, "chapter2.xhtml#intro", 0);
		assert!(result.found);
		assert_eq!(result.offset, 100);
	}

	#[test]
	fn resolve_link_returns_not_found_for_unknown_targets() {
		let doc = sample_link_doc_handle();
		let result = resolve_link(&doc, "missing.xhtml#none", 0);
		assert!(!result.found);
	}

	fn sample_reading_pos_doc_handle() -> DocumentHandle {
		let mut buffer = DocumentBuffer::with_content("x".repeat(220));
		buffer.add_marker(Marker::new(MarkerType::SectionBreak, 0));
		buffer.add_marker(Marker::new(MarkerType::SectionBreak, 100));
		let mut manifest_items = HashMap::new();
		manifest_items.insert("id1".to_string(), "chapter1.xhtml".to_string());
		manifest_items.insert("id2".to_string(), "chapter2.xhtml".to_string());
		let mut id_positions = HashMap::new();
		id_positions.insert("chapter1.xhtml#intro".to_string(), 10);
		id_positions.insert("chapter1.xhtml#mid".to_string(), 50);
		id_positions.insert("chapter2.xhtml#target".to_string(), 120);
		// Bare duplicate as inserted by the epub parser alongside the scoped key.
		id_positions.insert("intro".to_string(), 10);
		let mut doc = Document::new();
		doc.set_buffer(buffer);
		doc.spine_items = vec!["id1".to_string(), "id2".to_string()];
		doc.manifest_items = manifest_items;
		doc.id_positions = id_positions;
		DocumentHandle::new(doc)
	}

	fn sample_single_file_doc_handle() -> DocumentHandle {
		let mut id_positions = HashMap::new();
		id_positions.insert("top".to_string(), 10);
		id_positions.insert("middle".to_string(), 50);
		id_positions.insert("some.xhtml#scoped".to_string(), 40);
		let mut doc = Document::new();
		doc.set_buffer(DocumentBuffer::with_content("x".repeat(220)));
		doc.id_positions = id_positions;
		DocumentHandle::new(doc)
	}

	#[test]
	fn nearest_fragment_picks_latest_id_at_or_before_position() {
		let doc = sample_reading_pos_doc_handle();
		assert_eq!(nearest_fragment_before(&doc, 60), Some("mid".to_string()));
		assert_eq!(nearest_fragment_before(&doc, 30), Some("intro".to_string()));
		assert_eq!(nearest_fragment_before(&doc, 120), Some("target".to_string()));
	}

	#[test]
	fn nearest_fragment_returns_none_before_first_id_in_section() {
		let doc = sample_reading_pos_doc_handle();
		assert_eq!(nearest_fragment_before(&doc, 5), None);
	}

	#[test]
	fn nearest_fragment_ignores_ids_from_other_sections_and_bare_duplicates() {
		let doc = sample_reading_pos_doc_handle();
		// Position 110 is in chapter2 before its first id; chapter1 ids and the
		// bare "intro" duplicate must not match.
		assert_eq!(nearest_fragment_before(&doc, 110), None);
	}

	#[test]
	fn encode_url_fragment_escapes_unsafe_characters() {
		assert_eq!(encode_url_fragment("plain-id_1"), "plain-id_1");
		assert_eq!(encode_url_fragment("with space"), "with%20space");
		assert_eq!(encode_url_fragment("a#b%c"), "a%23b%25c");
	}

	#[test]
	fn nearest_fragment_uses_bare_ids_for_single_file_documents() {
		let doc = sample_single_file_doc_handle();
		assert_eq!(nearest_fragment_before(&doc, 60), Some("middle".to_string()));
		assert_eq!(nearest_fragment_before(&doc, 45), Some("top".to_string()));
		assert_eq!(nearest_fragment_before(&doc, 5), None);
	}
}
