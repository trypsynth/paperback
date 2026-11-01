//! Open from URL for the mobile apps, which run each step on a background thread of their own and poll for progress.

use std::{
	path::{Path, PathBuf},
	sync::{
		Mutex,
		atomic::{AtomicBool, AtomicU64, Ordering},
	},
};

use super::{
	DEFAULT_MAX_SIZE, FetchError, Remote, RemoteInfo, Verdict, not_a_link, open, refused_extension, warning_text,
};
use crate::parser::is_remote_url;

#[cfg_attr(feature = "uniffi", derive(uniffi::Error), uniffi(flat_error))]
#[derive(Debug, thiserror::Error)]
pub enum LinkDownloadError {
	/// Already in the reader's language, naming the link.
	#[error("{0}")]
	Failed(String),
	#[error("cancelled")]
	Cancelled,
}

#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkDownloadInfoFfi {
	pub file_name: String,
	/// -1 when the server doesn't say.
	pub size: i64,
	/// What to ask before downloading, when the server sends something other than the document the link names; empty when nothing needs asking.
	pub warning: String,
}

#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LinkProgressFfi {
	pub done: i64,
	/// -1 when the server doesn't say.
	pub total: i64,
}

/// One link: connected to on creation, saved by `save`, and cancellable from any thread meanwhile.
#[cfg_attr(feature = "uniffi", derive(uniffi::Object))]
pub struct LinkDownload {
	link: String,
	info: RemoteInfo,
	remote: Mutex<Option<Remote>>,
	cancel: AtomicBool,
	done: AtomicU64,
}

fn failed(link: &str, error: &FetchError) -> LinkDownloadError {
	LinkDownloadError::Failed(format!("{link}: {error}"))
}

fn to_i64(value: Option<u64>) -> i64 {
	value.and_then(|value| i64::try_from(value).ok()).unwrap_or(-1)
}

/// `name` in `folder`, numbered past any file already there, since the reader may download the same link twice.
fn free_path(folder: &Path, name: &str) -> PathBuf {
	let first = folder.join(name);
	if !first.exists() {
		return first;
	}
	let (stem, extension) = match name.rsplit_once('.') {
		Some((stem, extension)) if !stem.is_empty() => (stem, format!(".{extension}")),
		_ => (name, String::new()),
	};
	(2..10_000).map(|n| folder.join(format!("{stem} ({n}){extension}"))).find(|path| !path.exists()).unwrap_or(first)
}

impl LinkDownload {
	/// Blocks while the server is asked about the link, so callers run it off the main thread.
	pub fn connect(link: &str) -> Result<Self, LinkDownloadError> {
		let link = link.trim();
		if !is_remote_url(link) {
			return Err(LinkDownloadError::Failed(not_a_link(link)));
		}
		if let Some(extension) = refused_extension(link) {
			return Err(failed(link, &FetchError::Refused(extension)));
		}
		let remote = open(link, DEFAULT_MAX_SIZE).map_err(|error| failed(link, &error))?;
		let info = remote.info().clone();
		if let Verdict::Refuse(extension) = info.verdict(link) {
			return Err(failed(link, &FetchError::Refused(extension)));
		}
		Ok(Self {
			link: link.to_string(),
			info,
			remote: Mutex::new(Some(remote)),
			cancel: AtomicBool::new(false),
			done: AtomicU64::new(0),
		})
	}

	/// Blocks for the whole download, into a free name in `folder`, returning the saved file's path.
	pub fn save_into(&self, folder: &Path) -> Result<PathBuf, LinkDownloadError> {
		let Some(remote) = self.remote.lock().unwrap().take() else {
			return Err(LinkDownloadError::Cancelled);
		};
		std::fs::create_dir_all(folder).map_err(|error| failed(&self.link, &FetchError::from(error)))?;
		let path = free_path(folder, &self.info.file_name);
		match remote.save(&path, &self.cancel, |done, _| self.done.store(done, Ordering::Relaxed)) {
			Ok(_) => Ok(path),
			Err(FetchError::Cancelled) => Err(LinkDownloadError::Cancelled),
			Err(error) => Err(failed(&self.link, &error)),
		}
	}
}

#[cfg(feature = "uniffi")]
#[uniffi::export]
impl LinkDownload {
	// Owned `String` params (not `&str`) because that is the signature UniFFI generates for.
	#[uniffi::constructor]
	#[allow(clippy::needless_pass_by_value)]
	pub fn new(link: String) -> Result<std::sync::Arc<Self>, LinkDownloadError> {
		Self::connect(&link).map(std::sync::Arc::new)
	}
}

#[cfg_attr(feature = "uniffi", uniffi::export)]
impl LinkDownload {
	pub fn info(&self) -> LinkDownloadInfoFfi {
		let warning = match self.info.verdict(&self.link) {
			Verdict::Warn => warning_text(&self.link, self.info.content_type.as_deref()),
			Verdict::Pass | Verdict::Refuse(_) => String::new(),
		};
		LinkDownloadInfoFfi { file_name: self.info.file_name.clone(), size: to_i64(self.info.size), warning }
	}

	#[allow(clippy::needless_pass_by_value, reason = "UniFFI generates an owned String")]
	pub fn save(&self, folder: String) -> Result<String, LinkDownloadError> {
		self.save_into(Path::new(&folder)).map(|path| path.to_string_lossy().into_owned())
	}

	pub fn progress(&self) -> LinkProgressFfi {
		LinkProgressFfi { done: to_i64(Some(self.done.load(Ordering::Relaxed))), total: to_i64(self.info.size) }
	}

	pub fn cancel(&self) {
		self.cancel.store(true, Ordering::Relaxed);
	}
}

#[cfg(test)]
mod tests;
