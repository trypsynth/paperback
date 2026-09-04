//! Resolving EPUB-internal hrefs (nav/NCX/spine link targets) against the path they were
//! found in, and splitting a href into its path and `#fragment` parts.

use crate::{
	parser::{is_external_url, util::path::resolve_relative_path},
	util::text::url_decode,
};

pub(super) fn resolve_href(current_path: &str, target: &str) -> String {
	if is_external_url(target) {
		return target.to_string();
	}
	if target.starts_with('#') {
		return target.to_string();
	}
	let (path_part, fragment) = split_href(target);
	let resolved = if path_part.is_empty() {
		current_path.to_string()
	} else {
		let current_dir = current_path.rfind('/').map_or("", |i| &current_path[..i]);
		resolve_relative_path(current_dir, &path_part)
	};
	if let Some(frag) = fragment {
		if frag.is_empty() { resolved } else { format!("{resolved}#{frag}") }
	} else {
		resolved
	}
}

pub(super) fn split_href(input: &str) -> (String, Option<String>) {
	let decoded = url_decode(input);
	let trimmed = decoded.strip_prefix("epub://").unwrap_or(&decoded);
	if let Some((path, frag)) = trimmed.split_once('#') {
		(path.to_string(), Some(frag.to_string()))
	} else {
		(trimmed.to_string(), None)
	}
}
