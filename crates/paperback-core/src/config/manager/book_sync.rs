use std::{
	collections::HashSet,
	path::{Path, PathBuf},
};

use super::ConfigManager;
use crate::{
	book_sync::{LocalBook, ReadingData, ResumePosition, SyncBaseline, SyncReport, UploadApproval, standalone_file},
	config::DocumentConfig,
};

impl ConfigManager {
	pub fn remember_document_local_source(&self, locator: &str, local_path: &str) {
		self.data.borrow_mut().document_sources.insert(locator.to_owned(), local_path.to_owned());
		self.dirty.set(true);
	}

	pub fn drive_book_snapshot(&self) -> Vec<LocalBook> {
		let data = self.data.borrow();
		let mut books: Vec<_> = data
			.documents
			.iter()
			.map(|(key, document)| {
				let source = data.document_sources.get(&document.path).map_or(document.path.as_str(), String::as_str);
				let path = if let Some(encoded) = source.strip_prefix("file://") {
					PathBuf::from(percent_encoding::percent_decode_str(encoded).decode_utf8_lossy().into_owned())
				} else {
					PathBuf::from(source)
				};
				let title = document
					.download_name
					.clone()
					.or_else(|| path.file_name().map(|name| name.to_string_lossy().into_owned()))
					.unwrap_or_default();
				LocalBook {
					key: key.clone(),
					locator: document.path.clone(),
					path,
					title,
					reading: reading_data(document),
					baseline: document.drive_sync_baseline.clone(),
					upload_approval: document.drive_upload_approval.clone(),
				}
			})
			.collect();
		books.sort_by(|first, second| first.key.cmp(&second.key));
		books
	}

	pub fn request_drive_book_file(&self, path: &str, account: &str, confirmed_size: u64) -> bool {
		let key = self.get_doc_key(path);
		let Some(book) = self.drive_book_snapshot().into_iter().find(|book| book.key == key) else { return false };
		let Some(approval) = UploadApproval::new(account, &book.path, confirmed_size) else { return false };
		let mut data = self.data.borrow_mut();
		let Some(document) = data.documents.get_mut(&key) else { return false };
		document.drive_upload_approval = Some(approval);
		self.dirty.set(true);
		drop(data);
		self.flush();
		true
	}

	pub fn drive_book_file_size(&self, path: &str) -> Option<u64> {
		let key = self.get_doc_key(path);
		let book = self.drive_book_snapshot().into_iter().find(|book| book.key == key)?;
		let size = book.path.metadata().ok()?.len();
		(standalone_file(&book.path) && book.path.is_file() && size > 0).then_some(size)
	}

	pub fn drive_book_file_requested(&self, path: &str) -> bool {
		let key = self.get_doc_key(path);
		self.data.borrow().documents.get(&key).is_some_and(|doc| doc.drive_upload_approval.is_some())
	}

	pub fn apply_drive_book_report(&self, report: &SyncReport, protected: &[String]) -> Vec<String> {
		let protected: HashSet<_> = protected.iter().map(|path| self.get_doc_key(path)).collect();
		let mut data = self.data.borrow_mut();
		let mut finished = Vec::new();
		for (key, approval) in &report.finished_uploads {
			if let Some(doc) = data.documents.get_mut(key)
				&& doc.drive_upload_approval.as_ref() == Some(approval)
			{
				doc.drive_upload_approval = None;
				finished.push(doc.path.clone());
				self.dirty.set(true);
			}
		}
		for reading in &report.readings {
			let Some(doc) = data.documents.get_mut(&reading.local_key) else { continue };
			if protected.contains(&reading.local_key) || reading_data(doc) != reading.expected {
				continue;
			}
			if doc.drive_sync_baseline.as_ref().is_some_and(|old| {
				old.account == reading.baseline.account
					&& old.sha256 == reading.baseline.sha256
					&& old.applied == reading.baseline.applied
					&& old.applied == reading.expected
			}) {
				continue;
			}
			apply_reading(doc, reading.baseline.clone());
			self.dirty.set(true);
		}
		drop(data);
		for book in &report.books {
			if book.local_locator.is_some() {
				continue;
			}
			if let Some(path) = book.offline_path.as_deref().and_then(Path::to_str) {
				if protected.contains(&self.get_doc_key(path)) {
					continue;
				}
				if let Some(bytes) = book.cloud_bytes
					&& self.get_document_download_name(path).is_none()
				{
					self.set_document_download(path, &book.title, bytes);
				}
				self.apply_drive_book_for_open(report, &book.id, path);
			}
		}
		self.flush();
		finished
	}

	pub fn apply_drive_book_for_open(&self, report: &SyncReport, id: &str, path: &str) -> bool {
		let Some(book) = report.books.iter().find(|book| book.id == id) else { return false };
		if book.local_locator.as_deref() != Some(path)
			&& book.offline_path.as_deref().and_then(Path::to_str) != Some(path)
		{
			return false;
		}
		let key = self.get_doc_key(path);
		let mut data = self.data.borrow_mut();
		let doc = Self::doc_entry_mut(&mut data, key.clone(), path);
		let current = reading_data(doc);
		let expected = report.readings.iter().find(|read| read.local_key == key).map(|read| &read.expected);
		let prior = doc
			.drive_sync_baseline
			.as_ref()
			.filter(|prior| prior.account == report.account && prior.sha256 == id)
			.map(|prior| &prior.applied);
		if current != book.reading && expected.or(prior).is_some_and(|known| &current != known) {
			return false;
		}
		if doc.drive_sync_baseline.as_ref().is_some_and(|prior| {
			prior.account == report.account
				&& prior.sha256 == id
				&& prior.applied == book.reading
				&& current == book.reading
		}) {
			return true;
		}
		let revision = doc.drive_sync_baseline.as_ref().map_or(1, |baseline| baseline.revision.saturating_add(1));
		apply_reading(
			doc,
			SyncBaseline {
				account: report.account.clone(),
				sha256: id.into(),
				applied: book.reading.clone(),
				revision,
			},
		);
		self.dirty.set(true);
		true
	}
}

fn reading_data(doc: &DocumentConfig) -> ReadingData {
	ReadingData {
		resume: ResumePosition { text: doc.last_position, audio_ms: doc.audio_time_ms },
		bookmarks: doc.bookmarks.clone(),
		temporary_bookmark: doc.temporary_bookmark,
	}
}

fn apply_reading(doc: &mut DocumentConfig, baseline: SyncBaseline) {
	doc.last_position = baseline.applied.resume.text;
	doc.audio_time_ms = baseline.applied.resume.audio_ms;
	doc.bookmarks.clone_from(&baseline.applied.bookmarks);
	doc.temporary_bookmark = baseline.applied.temporary_bookmark;
	doc.drive_sync_baseline = Some(baseline);
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{book_sync::SyncedReading, document::DocPos, util::test_support::TempDir};

	fn update(key: String) -> SyncReport {
		SyncReport {
			readings: vec![SyncedReading {
				local_key: key,
				expected: ReadingData::default(),
				baseline: SyncBaseline {
					account: "account".into(),
					sha256: "book".into(),
					applied: ReadingData {
						resume: ResumePosition { text: DocPos::from_raw(20), audio_ms: Some(8_321) },
						..ReadingData::default()
					},
					revision: 1,
				},
			}],
			..SyncReport::default()
		}
	}

	#[test]
	fn an_update_does_not_change_an_open_book_even_through_another_locator() {
		let dir = TempDir::new("drive-position-protected");
		let path = dir.write_str("book.txt", "the book");
		let uri = "content://local/book";
		let config = ConfigManager::in_memory();
		config.add_recent_document(&path);
		config.associate_uri_with_local_file(uri, &path);
		let report = update(config.get_doc_key(&path));
		config.apply_drive_book_report(&report, &[uri.into()]);
		assert_eq!(config.get_document_position(&path), DocPos::ZERO);
		config.apply_drive_book_report(&report, &[]);
		assert_eq!(config.get_document_position(&path), DocPos::from_raw(20));
		assert_eq!(config.get_document_audio_time(&path), Some(8_321));
	}

	#[test]
	fn a_reader_move_while_sync_was_in_flight_is_preserved() {
		let config = ConfigManager::in_memory();
		let path = "book.txt";
		config.add_recent_document(path);
		let report = update(config.get_doc_key(path));
		config.set_document_position(path, DocPos::from_raw(31));
		config.apply_drive_book_report(&report, &[]);
		assert_eq!(config.get_document_position(path), DocPos::from_raw(31));
		assert_eq!(config.get_document_audio_time(path), None);
	}

	#[test]
	fn opening_from_the_first_cloud_report_does_not_overwrite_a_newer_local_position() {
		let config = ConfigManager::in_memory();
		let path = "book.txt";
		config.add_recent_document(path);
		let mut report = update(config.get_doc_key(path));
		report.books.push(crate::book_sync::CloudBook {
			id: "book".into(),
			title: "Book".into(),
			local_locator: Some(path.into()),
			offline_path: None,
			cloud_bytes: None,
			resume: report.readings[0].baseline.applied.resume,
			choices: Vec::new(),
			reading: report.readings[0].baseline.applied.clone(),
		});
		config.set_document_position(path, DocPos::from_raw(31));
		assert!(!config.apply_drive_book_for_open(&report, "book", path));
		assert_eq!(config.get_document_position(path), DocPos::from_raw(31));
	}
}
