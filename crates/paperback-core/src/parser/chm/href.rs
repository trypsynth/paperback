use crate::parser::{is_external_url, util::path::resolve_relative_path};

pub(super) fn normalize_path(path: &str) -> String {
	let mut result = path.replace('\\', "/").to_lowercase();
	if !result.starts_with('/') {
		result.insert(0, '/');
	}
	result
}

pub(super) fn resolve_chm_href(current_file: &str, href: &str) -> String {
	if is_external_url(href) {
		return href.to_string();
	}
	let (path_part, fragment) = href.split_once('#').map_or((href, None), |(p, f)| (p, Some(f)));
	let resolved_path = if path_part.is_empty() {
		normalize_path(current_file)
	} else {
		let current_normalized = normalize_path(current_file);
		let current_dir = current_normalized.rfind('/').map_or("", |i| &current_normalized[..i]);
		let path_normalized = path_part.replace('\\', "/");
		format!("/{}", resolve_relative_path(current_dir, &path_normalized)).to_lowercase()
	};
	match fragment {
		Some(frag) if !frag.is_empty() => format!("{resolved_path}#{frag}"),
		_ => resolved_path,
	}
}

#[cfg(test)]
mod tests;
