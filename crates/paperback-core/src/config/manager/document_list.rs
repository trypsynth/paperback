//! [`get_sorted_document_list`], the "All Documents" dialog's filtering/sorting/status logic
//! (recents first, then everything else alphabetically by file name), and
//! [`compute_document_hash`], the content-based fingerprint [`super::ConfigManager::get_doc_key`]
//! hashes a document path down to.

use std::{
	cmp::{self, Ordering},
	fs,
	path::Path,
};

use sha1::{Digest, Sha1};

use super::ConfigManager;
use crate::{
	parser::{is_remote_url, url_file_name},
	types::{DocumentListItem, DocumentListStatus},
};

pub fn get_sorted_document_list(
	config: &ConfigManager,
	open_paths: &[String],
	filter: &str,
	status_filter: Option<DocumentListStatus>,
) -> Vec<DocumentListItem> {
	let recent_docs = config.get_recent_documents();
	let all_docs = config.get_all_documents();
	let mut doc_paths: Vec<String> = Vec::new();
	for path in &recent_docs {
		if !doc_paths.contains(path) {
			doc_paths.push(path.clone());
		}
	}
	let mut rest: Vec<String> = all_docs.iter().filter(|path| !doc_paths.contains(path)).cloned().collect();
	rest.sort_by(|a, b| {
		let a_name = document_name(a).unwrap_or_else(|| a.clone());
		let b_name = document_name(b).unwrap_or_else(|| b.clone());
		let name_cmp = a_name.to_lowercase().cmp(&b_name.to_lowercase());
		if name_cmp != Ordering::Equal {
			return name_cmp;
		}
		a.to_lowercase().cmp(&b.to_lowercase())
	});
	doc_paths.extend(rest);
	let filter_lower = filter.to_lowercase();
	doc_paths
		.into_iter()
		.filter_map(|path| {
			let filename = document_name(&path).unwrap_or_default();
			if !filter.is_empty() && !filename.to_lowercase().contains(&filter_lower) {
				return None;
			}
			// A link has no file of its own to go missing: opening it downloads it again.
			let status = if !is_remote_url(&path) && !Path::new(&path).exists() {
				DocumentListStatus::Missing
			} else if open_paths.contains(&path) {
				DocumentListStatus::Open
			} else {
				DocumentListStatus::Closed
			};
			if status_filter.is_some_and(|wanted| wanted != status) {
				return None;
			}
			Some(DocumentListItem { path, filename, status })
		})
		.collect()
}

/// The name a document is listed, sorted and searched by: the file's own name, or for a link the
/// file name it ends in.
#[must_use]
pub fn document_name(path: &str) -> Option<String> {
	if is_remote_url(path) {
		return url_file_name(path);
	}
	Path::new(path).file_name().and_then(|name| name.to_str()).map(str::to_string)
}

#[must_use]
pub fn compute_document_hash(path: &str) -> [u8; 20] {
	let mut hasher = Sha1::new();
	if let Ok(mut file) = fs::File::open(path) {
		use std::io::{Read, Seek, SeekFrom};
		let mut buffer = [0; 65536];
		let mut total_read = 0;
		let max_read = 1024 * 1024;
		while total_read < max_read {
			let to_read = cmp::min(buffer.len(), max_read - total_read);
			if let Ok(n) = file.read(&mut buffer[..to_read]) {
				if n == 0 {
					break;
				}
				hasher.update(&buffer[..n]);
				total_read += n;
			} else {
				break;
			}
		}
		if let Ok(metadata) = file.metadata() {
			let file_size = metadata.len();
			hasher.update(file_size.to_le_bytes());
			if file_size > max_read as u64 {
				let seek_pos = cmp::max(file_size.saturating_sub(max_read as u64), max_read as u64);
				if file.seek(SeekFrom::Start(seek_pos)).is_ok() {
					let mut end_read = 0;
					let end_max = (file_size - seek_pos) as usize;
					while end_read < end_max {
						let to_read = cmp::min(buffer.len(), end_max - end_read);
						if let Ok(n) = file.read(&mut buffer[..to_read]) {
							if n == 0 {
								break;
							}
							hasher.update(&buffer[..n]);
							end_read += n;
						} else {
							break;
						}
					}
				}
			}
		}
	} else {
		hasher.update(path.as_bytes());
	}
	hasher.finalize().into()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn get_sorted_document_list_filters_by_status() {
		use crate::util::test_support::TempDir;
		let dir = TempDir::new("document-list-status-filter");
		let open_path = dir.write_str("open.txt", "content");
		let closed_path = dir.write_str("closed.txt", "content");
		let missing_path = dir.join_str("missing.txt");
		let mut config = ConfigManager::new();
		config.initialized = true;
		for path in [&open_path, &closed_path, &missing_path] {
			config.add_recent_document(path);
		}
		let open_paths = vec![open_path.clone()];
		let all = get_sorted_document_list(&config, &open_paths, "", None);
		assert_eq!(all.len(), 3);
		let open_only = get_sorted_document_list(&config, &open_paths, "", Some(DocumentListStatus::Open));
		assert_eq!(open_only.iter().map(|item| &item.path).collect::<Vec<_>>(), vec![&open_path]);
		let closed_only = get_sorted_document_list(&config, &open_paths, "", Some(DocumentListStatus::Closed));
		assert_eq!(closed_only.iter().map(|item| &item.path).collect::<Vec<_>>(), vec![&closed_path]);
		let missing_only = get_sorted_document_list(&config, &open_paths, "", Some(DocumentListStatus::Missing));
		assert_eq!(missing_only.iter().map(|item| &item.path).collect::<Vec<_>>(), vec![&missing_path]);
	}

	/// A document opened from a link has no file of its own to go missing; it is downloaded again
	/// when it is opened.
	#[test]
	fn a_link_is_open_or_closed_and_never_missing() {
		let config = ConfigManager::in_memory();
		config.add_recent_document("https://example.org/a.epub");
		config.add_recent_document("https://example.org/b.pdf");
		let open_paths = vec!["https://example.org/b.pdf".to_string()];
		let statuses: Vec<_> = get_sorted_document_list(&config, &open_paths, "", None)
			.into_iter()
			.map(|item| (item.path, item.status))
			.collect();
		assert_eq!(
			statuses,
			vec![
				("https://example.org/b.pdf".to_string(), DocumentListStatus::Open),
				("https://example.org/a.epub".to_string(), DocumentListStatus::Closed),
			]
		);
	}

	#[test]
	fn a_link_is_named_and_found_by_its_file_name() {
		let config = ConfigManager::in_memory();
		config.add_recent_document("https://example.org/books/Moby%20Dick.epub?from=aardvark");
		let found = get_sorted_document_list(&config, &[], "moby", None);
		assert_eq!(found.iter().map(|item| item.filename.as_str()).collect::<Vec<_>>(), vec!["Moby Dick.epub"]);
		assert!(get_sorted_document_list(&config, &[], "aardvark", None).is_empty());
	}

	#[test]
	fn a_link_is_sorted_by_its_file_name() {
		let config = ConfigManager::in_memory();
		config.set_document_position("b.epub-notes.txt", 1);
		config.set_document_position("https://example.org/b.epub?ref=zzz", 1);
		let order: Vec<_> =
			get_sorted_document_list(&config, &[], "", None).into_iter().map(|item| item.filename).collect();
		assert_eq!(order, vec!["b.epub", "b.epub-notes.txt"]);
	}
}
