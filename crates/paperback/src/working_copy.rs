use std::{
	io,
	path::{Path, PathBuf},
};

use tempfile::TempDir;

/// Dropping it deletes the folder, so the copy lasts exactly as long as whatever reads it; it's never reused as a cache.
pub struct WorkingCopy {
	_dir: TempDir,
	path: PathBuf,
}

impl WorkingCopy {
	pub fn new(file_name: &str) -> io::Result<Self> {
		let dir = tempfile::Builder::new().prefix("paperback-").tempdir()?;
		let path = dir.path().join(file_name);
		Ok(Self { _dir: dir, path })
	}

	pub fn path(&self) -> &Path {
		&self.path
	}

	pub fn file_name(&self) -> String {
		self.path.file_name().map(|name| name.to_string_lossy().into_owned()).unwrap_or_default()
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
		assert_eq!(first.file_name(), "book.epub");
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
