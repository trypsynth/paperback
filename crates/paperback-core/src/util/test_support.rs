//! Filesystem helpers shared by unit tests.
//!
//! Parsers take a path rather than bytes, so testing one means putting a real file on disk.
//! Several test modules had grown their own `unique_temp_path` for that; this is the one
//! implementation, and unlike those it removes what it created when the test ends.

use std::{
	env, fs,
	path::{Path, PathBuf},
	sync::atomic::{AtomicU64, Ordering},
	time::{SystemTime, UNIX_EPOCH},
};

/// A uniquely named directory under the system temp dir, deleted when the guard drops.
///
/// Bind it to a variable for the lifetime of the test — `let _dir = TempDir::new(..)` drops
/// immediately and takes the files with it.
pub struct TempDir {
	path: PathBuf,
}

impl TempDir {
	/// Creates the directory. `label` appears in the name so a directory left behind by a
	/// killed test process can be traced back to the test that made it.
	pub fn new(label: &str) -> Self {
		static COUNTER: AtomicU64 = AtomicU64::new(0);
		let nanos = SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |elapsed| elapsed.as_nanos());
		let unique = COUNTER.fetch_add(1, Ordering::Relaxed);
		let path = env::temp_dir().join(format!("paperback_test_{label}_{}_{nanos}_{unique}", std::process::id()));
		fs::create_dir_all(&path).expect("create temp dir");
		Self { path }
	}

	#[must_use]
	pub fn path(&self) -> &Path {
		&self.path
	}

	/// Writes `contents` to `name` inside the directory, creating parent directories as needed,
	/// and returns the full path.
	pub fn write(&self, name: &str, contents: impl AsRef<[u8]>) -> PathBuf {
		let path = self.path.join(name);
		if let Some(parent) = path.parent() {
			fs::create_dir_all(parent).expect("create parent dir");
		}
		fs::write(&path, contents).expect("write temp file");
		path
	}

	/// Like [`TempDir::write`], for the parser APIs that take a path as a `String`.
	pub fn write_str(&self, name: &str, contents: impl AsRef<[u8]>) -> String {
		self.write(name, contents).to_string_lossy().into_owned()
	}

	/// The path a file would have inside this directory, without creating it.
	#[must_use]
	pub fn join_str(&self, name: &str) -> String {
		self.path.join(name).to_string_lossy().into_owned()
	}
}

impl Drop for TempDir {
	fn drop(&mut self) {
		let _ = fs::remove_dir_all(&self.path);
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn write_creates_the_file_and_returns_its_path() {
		let dir = TempDir::new("selftest");
		let path = dir.write("nested/book.txt", "hello");
		assert!(path.exists());
		assert_eq!(fs::read_to_string(&path).expect("read back"), "hello");
	}

	#[test]
	fn directory_is_removed_on_drop() {
		let path = {
			let dir = TempDir::new("selftest");
			dir.write("book.txt", "hello");
			dir.path().to_path_buf()
		};
		assert!(!path.exists(), "temp dir must not outlive the guard");
	}

	#[test]
	fn each_instance_gets_its_own_directory() {
		let first = TempDir::new("selftest");
		let second = TempDir::new("selftest");
		assert_ne!(first.path(), second.path());
	}
}
