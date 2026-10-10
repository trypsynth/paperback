//! The temporary file a document opened from a link is read from.

use std::{
	io,
	path::{Path, PathBuf},
};

use tempfile::TempDir;

/// A downloaded document's file, in a folder of its own in the system's temporary folder.
///
/// Dropping it deletes the folder, so the copy lasts exactly as long as whatever reads it. It is
/// never reused as a cache: opening the link again downloads it again.
pub struct WorkingCopy {
	_dir: TempDir,
	path: PathBuf,
}

impl WorkingCopy {
	/// Makes a new, empty folder for a file called `file_name`. The file itself is written by the
	/// download.
	pub fn new(file_name: &str) -> io::Result<Self> {
		let dir = tempfile::Builder::new().prefix("paperback-").tempdir()?;
		let path = dir.path().join(file_name);
		Ok(Self { _dir: dir, path })
	}

	pub fn path(&self) -> &Path {
		&self.path
	}
}

#[cfg(test)]
mod tests {
	use std::fs;

	use super::*;

	#[test]
	fn each_copy_gets_a_new_folder_of_its_own() {
		let first = WorkingCopy::new("book.epub").unwrap();
		let second = WorkingCopy::new("book.epub").unwrap();
		assert_eq!(first.path().file_name().unwrap(), "book.epub");
		assert_eq!(second.path().file_name().unwrap(), "book.epub");
		assert!(first.path().parent().unwrap().is_dir());
		assert_ne!(first.path().parent(), second.path().parent());
	}

	#[test]
	fn dropping_a_copy_removes_its_folder() {
		let copy = WorkingCopy::new("book.epub").unwrap();
		fs::write(copy.path(), "downloaded").unwrap();
		let folder = copy.path().parent().unwrap().to_path_buf();
		drop(copy);
		assert!(!folder.exists());
	}
}
