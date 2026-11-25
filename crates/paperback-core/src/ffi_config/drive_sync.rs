use std::sync::Arc;

use super::ConfigManagerFfi;
use crate::book_sync::{BookTransfer, CloudBookKey, Command, DriveSync, ResumeChoice, SyncReport};

#[derive(uniffi::Record)]
pub struct DriveBookItem {
	pub key: CloudBookKey,
	pub title: String,
	pub local_locator: Option<String>,
	pub offline_path: Option<String>,
	pub cloud_bytes: Option<u64>,
	pub choices: Vec<ResumeChoice>,
}

#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum DriveSyncError {
	#[error("Google Drive synchronization failed")]
	Failed,
}

#[uniffi::export]
impl SyncReport {
	pub fn drive_books(&self) -> Vec<DriveBookItem> {
		self.books
			.iter()
			.map(|book| DriveBookItem {
				key: CloudBookKey { id: book.id.clone(), account: self.account.clone() },
				title: book.title.clone(),
				local_locator: book.local_locator.clone(),
				offline_path: book.offline_path.as_ref().and_then(|path| path.to_str()).map(str::to_owned),
				cloud_bytes: book.cloud_bytes,
				choices: book.choices.clone(),
			})
			.collect()
	}

	pub fn drive_account(&self) -> String {
		self.account_name.clone()
	}

	pub fn drive_account_key(&self) -> String {
		self.account.clone()
	}

	pub fn drive_files_waiting(&self) -> bool {
		self.files_waiting
	}

	pub fn drive_file_error(&self) -> bool {
		self.file_error
	}
}

#[uniffi::export]
#[allow(clippy::needless_pass_by_value, reason = "owned arguments are part of the UniFFI interface")]
impl ConfigManagerFfi {
	pub fn sync_google_drive(
		&self,
		root: String,
		token: String,
		command: Command,
		transfer: Box<dyn BookTransfer>,
	) -> Result<Arc<SyncReport>, DriveSyncError> {
		let books = self.inner.lock().unwrap().drive_book_snapshot();
		DriveSync::new(root.into(), "Android".into()).sync(&token, books, &command, &*transfer).map(Arc::new).map_err(
			|error| {
				tracing::warn!(%error, "Google Drive synchronization failed");
				DriveSyncError::Failed
			},
		)
	}

	pub fn cached_google_drive(
		&self,
		root: String,
		transfer: Box<dyn BookTransfer>,
	) -> Result<Arc<SyncReport>, DriveSyncError> {
		let books = self.inner.lock().unwrap().drive_book_snapshot();
		DriveSync::new(root.into(), "Android".into())
			.offline_report(books, &*transfer)
			.map(Arc::new)
			.map_err(|_| DriveSyncError::Failed)
	}

	pub fn apply_google_drive_report(&self, report: Arc<SyncReport>, protected: Vec<String>) -> Vec<String> {
		self.inner.lock().unwrap().apply_drive_book_report(&report, &protected)
	}

	pub fn apply_google_drive_book(&self, report: Arc<SyncReport>, id: String, path: String) -> bool {
		self.inner.lock().unwrap().apply_drive_book_for_open(&report, &id, &path)
	}

	pub fn google_drive_book_size(&self, path: String) -> Option<u64> {
		self.inner.lock().unwrap().drive_book_file_size(&path)
	}

	pub fn request_google_drive_book_file(&self, path: String, account: String, confirmed_size: u64) -> bool {
		self.inner.lock().unwrap().request_drive_book_file(&path, &account, confirmed_size)
	}

	pub fn google_drive_book_file_requested(&self, path: String) -> bool {
		self.inner.lock().unwrap().drive_book_file_requested(&path)
	}

	pub fn google_drive_book_title(&self, path: String) -> Option<String> {
		self.inner.lock().unwrap().get_document_download_name(&path)
	}
}
