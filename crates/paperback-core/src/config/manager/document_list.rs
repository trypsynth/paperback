use std::{cmp, collections::HashSet, fs, path::Path};

use sha1::{Digest, Sha1};

use super::ConfigManager;
use crate::{
	parser::{is_remote_url, url_file_name},
	types::{DocumentListItem, DocumentListStatus},
};

/// Every file is checked once here, since a missing one on a network drive can take seconds to answer; [`filter_document_list`] narrows the result as the reader types.
#[must_use]
pub fn get_sorted_document_list(config: &ConfigManager, open_paths: &[String]) -> Vec<DocumentListItem> {
	let item = |path: String| {
		let status = if document_missing(&path) {
			DocumentListStatus::Missing
		} else if open_paths.contains(&path) {
			DocumentListStatus::Open
		} else {
			DocumentListStatus::Closed
		};
		DocumentListItem { filename: document_name(config, &path), path, status }
	};
	let mut seen = HashSet::new();
	let recent: Vec<String> =
		config.get_recent_documents().into_iter().filter(|path| seen.insert(path.clone())).collect();
	let mut rest: Vec<DocumentListItem> =
		config.get_all_documents().into_iter().filter(|path| !seen.contains(path)).map(item).collect();
	rest.sort_by_cached_key(|item| (item.filename.to_lowercase(), item.path.to_lowercase()));
	let mut items: Vec<DocumentListItem> = recent.into_iter().map(item).collect();
	items.extend(rest);
	items
}

#[must_use]
pub fn filter_document_list<'a>(
	items: &'a [DocumentListItem],
	filter: &str,
	status_filter: Option<DocumentListStatus>,
) -> Vec<&'a DocumentListItem> {
	let filter = filter.to_lowercase();
	items
		.iter()
		.filter(|item| filter.is_empty() || item.filename.to_lowercase().contains(&filter))
		.filter(|item| status_filter.is_none_or(|wanted| wanted == item.status))
		.collect()
}

/// A link's download name wins, since its last segment can be a script like `download` or nothing at all.
#[must_use]
pub fn document_name(config: &ConfigManager, path: &str) -> String {
	let name = if is_remote_url(path) {
		config.get_document_download_name(path).or_else(|| url_file_name(path))
	} else {
		Path::new(path).file_name().and_then(|name| name.to_str()).map(str::to_string)
	};
	name.unwrap_or_else(|| path.to_string())
}

/// A link is never missing, since opening it downloads it again.
#[must_use]
pub fn document_missing(path: &str) -> bool {
	!is_remote_url(path) && !Path::new(path).exists()
}

#[must_use]
pub fn compute_document_hash(path: &str) -> [u8; 20] {
	let mut hasher = Sha1::new();
	if let Ok(mut file) = fs::File::open(path) {
		use std::io::{Read, Seek, SeekFrom};
		let mut buffer = vec![0; 65536];
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
					let end_max = usize::try_from(file_size - seek_pos).unwrap_or(usize::MAX);
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
		let all = get_sorted_document_list(&config, &open_paths);
		assert_eq!(all.len(), 3);
		let open_only = filter_document_list(&all, "", Some(DocumentListStatus::Open));
		assert_eq!(open_only.iter().map(|item| &item.path).collect::<Vec<_>>(), vec![&open_path]);
		let closed_only = filter_document_list(&all, "", Some(DocumentListStatus::Closed));
		assert_eq!(closed_only.iter().map(|item| &item.path).collect::<Vec<_>>(), vec![&closed_path]);
		let missing_only = filter_document_list(&all, "", Some(DocumentListStatus::Missing));
		assert_eq!(missing_only.iter().map(|item| &item.path).collect::<Vec<_>>(), vec![&missing_path]);
	}

	#[test]
	fn a_link_is_open_or_closed_and_never_missing() {
		let config = ConfigManager::in_memory();
		config.add_recent_document("https://example.org/a.epub");
		config.add_recent_document("https://example.org/b.pdf");
		let open_paths = vec!["https://example.org/b.pdf".to_string()];
		let statuses: Vec<_> =
			get_sorted_document_list(&config, &open_paths).into_iter().map(|item| (item.path, item.status)).collect();
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
		let all = get_sorted_document_list(&config, &[]);
		let found = filter_document_list(&all, "moby", None);
		assert_eq!(found.iter().map(|item| item.filename.as_str()).collect::<Vec<_>>(), vec!["Moby Dick.epub"]);
		assert!(filter_document_list(&all, "aardvark", None).is_empty());
	}

	#[test]
	fn a_link_is_named_and_found_by_the_name_it_was_downloaded_under() {
		let config = ConfigManager::in_memory();
		config.add_recent_document("https://example.org/download?id=7");
		config.set_document_download("https://example.org/download?id=7", "Moby Dick.epub", 10);
		let all = get_sorted_document_list(&config, &[]);
		let found = filter_document_list(&all, "moby", None);
		assert_eq!(found.iter().map(|item| item.filename.as_str()).collect::<Vec<_>>(), vec!["Moby Dick.epub"]);
	}

	#[test]
	fn a_link_ending_in_no_file_name_is_named_by_the_link() {
		let config = ConfigManager::in_memory();
		assert_eq!(document_name(&config, "https://example.org/"), "https://example.org/");
	}

	#[test]
	fn a_link_is_sorted_by_its_file_name() {
		let config = ConfigManager::in_memory();
		config.set_document_position("b.epub-notes.txt", 1.into());
		config.set_document_position("https://example.org/b.epub?ref=zzz", 1.into());
		let order: Vec<_> = get_sorted_document_list(&config, &[]).into_iter().map(|item| item.filename).collect();
		assert_eq!(order, vec!["b.epub", "b.epub-notes.txt"]);
	}
}
