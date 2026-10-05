//! Downloading the links among the inputs, each into its own folder under one folder per run.

use std::{env, fs, path::PathBuf, process, sync::atomic::AtomicBool};

use anyhow::{Context, Result, anyhow, bail};
use paperback_core::fetch::{self, Verdict};

/// The folder one run downloads into.
pub struct Downloads {
	root: PathBuf,
	count: usize,
}

/// One downloaded document; the folder it was downloaded into is removed when this is dropped.
pub struct Downloaded {
	pub path: PathBuf,
}

impl Drop for Downloaded {
	fn drop(&mut self) {
		if let Some(folder) = self.path.parent() {
			let _ = fs::remove_dir_all(folder);
		}
	}
}

impl Default for Downloads {
	fn default() -> Self {
		Self::in_folder(env::temp_dir().join(format!("pb-{}", process::id())))
	}
}

impl Downloads {
	#[must_use]
	pub const fn in_folder(root: PathBuf) -> Self {
		Self { root, count: 0 }
	}

	/// Checks `url` against the safety rule and downloads it.
	///
	/// # Errors
	///
	/// Returns an error naming `url` when the link is refused, when the server gives no sign of a
	/// format pb reads, or when the probe or the download fails.
	pub fn fetch(&mut self, url: &str) -> Result<Downloaded> {
		if let Some(extension) = fetch::refused_extension(url) {
			return Err(not_downloaded(url, &extension));
		}
		let info = fetch::probe(url).map_err(|error| anyhow!("{url}: {error}"))?;
		match info.verdict(url) {
			Verdict::Pass => {}
			Verdict::Refuse(extension) => return Err(not_downloaded(url, &extension)),
			Verdict::Warn => bail!(
				"{url}: security warning: the server sends {}, which is not a format pb reads, so it was not downloaded",
				info.content_type.as_deref().unwrap_or("no content type")
			),
		}
		self.count += 1;
		let folder = self.root.join(self.count.to_string());
		fs::create_dir_all(&folder).with_context(|| format!("failed to create the folder {}", folder.display()))?;
		let downloaded = Downloaded { path: folder.join(&info.file_name) };
		fetch::download(url, &downloaded.path, &AtomicBool::new(false), |_, _| {})
			.map_err(|error| anyhow!("{url}: {error}"))?;
		Ok(downloaded)
	}

	/// Removes the run's download folder.
	pub fn remove(&self) {
		let _ = fs::remove_dir_all(&self.root);
	}
}

fn not_downloaded(url: &str, extension: &str) -> anyhow::Error {
	anyhow!("{url}: pb does not read .{extension} files, so it was not downloaded")
}

#[cfg(test)]
mod tests {
	use std::{
		env, fs,
		path::PathBuf,
		process,
		sync::atomic::{AtomicU64, Ordering},
	};

	use super::*;

	struct TempDir {
		path: PathBuf,
	}

	impl TempDir {
		fn new(label: &str) -> Self {
			static COUNTER: AtomicU64 = AtomicU64::new(0);
			let unique = COUNTER.fetch_add(1, Ordering::Relaxed);
			let path = env::temp_dir().join(format!("pb_remote_test_{label}_{}_{unique}", process::id()));
			fs::create_dir_all(&path).expect("create the temp dir");
			Self { path }
		}

		fn join(&self, name: &str) -> PathBuf {
			self.path.join(name)
		}
	}

	impl Drop for TempDir {
		fn drop(&mut self) {
			let _ = fs::remove_dir_all(&self.path);
		}
	}

	#[test]
	fn a_link_to_a_type_pb_does_not_read_is_refused_without_a_download() {
		let dir = TempDir::new("refused");
		let mut downloads = Downloads::in_folder(dir.join("run"));
		let error = downloads.fetch("https://example.invalid/setup.exe").err().expect("refused").to_string();
		assert!(error.contains("https://example.invalid/setup.exe"), "{error}");
		assert!(error.contains("not downloaded"), "{error}");
		assert!(!dir.join("run").exists());
	}

	#[test]
	fn removing_the_downloads_removes_the_run_folder() {
		let dir = TempDir::new("remove");
		let root = dir.join("run");
		fs::create_dir_all(root.join("1")).expect("mkdir");
		Downloads::in_folder(root.clone()).remove();
		assert!(!root.exists());
	}

	#[test]
	fn a_downloaded_document_removes_its_folder_when_dropped() {
		let dir = TempDir::new("drop");
		let folder = dir.join("1");
		fs::create_dir_all(&folder).expect("mkdir");
		let path = folder.join("a.epub");
		fs::write(&path, b"x").expect("write");
		drop(Downloaded { path });
		assert!(!folder.exists());
	}
}
