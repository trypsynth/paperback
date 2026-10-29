use std::{
	env, fs,
	ops::Deref,
	path::{Path, PathBuf},
	process,
	sync::atomic::{AtomicU64, Ordering},
	time::{SystemTime, UNIX_EPOCH},
};

/// Deleted on drop, so `let _dir = TempDir::new(..)` takes the files with it immediately; bind it to a name.
pub struct TempDir {
	path: PathBuf,
}

impl TempDir {
	/// `label` is in the name so a directory left behind by a killed test can be traced to the test.
	pub fn new(label: &str) -> Self {
		static COUNTER: AtomicU64 = AtomicU64::new(0);
		let nanos = SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |elapsed| elapsed.as_nanos());
		let unique = COUNTER.fetch_add(1, Ordering::Relaxed);
		let path = env::temp_dir().join(format!("paperback_test_{label}_{}_{nanos}_{unique}", process::id()));
		fs::create_dir_all(&path).expect("create temp dir");
		Self { path }
	}

	#[must_use]
	pub fn path(&self) -> &Path {
		&self.path
	}

	pub fn write(&self, name: &str, contents: impl AsRef<[u8]>) -> PathBuf {
		let path = self.path.join(name);
		if let Some(parent) = path.parent() {
			fs::create_dir_all(parent).expect("create parent dir");
		}
		fs::write(&path, contents).expect("write temp file");
		path
	}

	pub fn write_str(&self, name: &str, contents: impl AsRef<[u8]>) -> String {
		self.write(name, contents).to_string_lossy().into_owned()
	}

	#[must_use]
	pub fn join_str(&self, name: &str) -> String {
		self.path.join(name).to_string_lossy().into_owned()
	}
}

impl Deref for TempDir {
	type Target = Path;

	fn deref(&self) -> &Path {
		&self.path
	}
}

impl AsRef<Path> for TempDir {
	fn as_ref(&self) -> &Path {
		&self.path
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
