//! Downloading the links among the inputs, each into its own folder under one folder per run.

use std::{
	env,
	fs::{self, File},
	io::{BufRead, BufReader},
	path::PathBuf,
	sync::atomic::AtomicBool,
};

use anyhow::{Context, Result, anyhow, bail};
use paperback_core::fetch::{self, RemoteInfo, Verdict};
use tempfile::TempDir;

/// A question put to the reader, answered yes (`true`) or no.
type Prompt = Box<dyn FnMut(&str) -> bool>;

/// The folder one run downloads into, created on first use under a name no other run shares.
pub struct Downloads {
	parent: PathBuf,
	root: Option<TempDir>,
	count: usize,
	/// Asked whether to download a link that got a security warning; without it, such a link is
	/// refused.
	ask: Option<Prompt>,
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
		Self::in_folder(env::temp_dir())
	}
}

impl Downloads {
	/// Downloads into a new `pb-` folder inside `parent`.
	#[must_use]
	pub const fn in_folder(parent: PathBuf) -> Self {
		Self { parent, root: None, count: 0, ask: None }
	}

	/// Asks `ask` with the warning whether to download a link that got one.
	#[must_use]
	pub fn with_prompt(mut self, ask: impl FnMut(&str) -> bool + 'static) -> Self {
		self.ask = Some(Box::new(ask));
		self
	}

	/// Checks `url` against the safety rule and downloads it.
	///
	/// # Errors
	///
	/// Returns an error naming `url` when the link is refused, when the server gives no sign of a
	/// format pb reads, or when the request or the download fails.
	pub fn fetch(&mut self, url: &str) -> Result<Downloaded> {
		if let Some(extension) = fetch::refused_extension(url) {
			return Err(not_downloaded(url, &extension));
		}
		let remote = fetch::open(url).map_err(|error| anyhow!("{url}: {error}"))?;
		self.allowed(url, remote.info())?;
		let folder = self.next_folder()?;
		let downloaded = Downloaded { path: folder.join(&remote.info().file_name) };
		remote.save(&downloaded.path, &AtomicBool::new(false), |_, _| {}).map_err(|error| anyhow!("{url}: {error}"))?;
		Ok(downloaded)
	}

	/// Whether the link may be downloaded, by its [`Verdict`]: a refusal is final, and a warning
	/// is put to the prompt, or refused when there is none.
	fn allowed(&mut self, url: &str, info: &RemoteInfo) -> Result<()> {
		match info.verdict(url) {
			Verdict::Pass => Ok(()),
			Verdict::Refuse(extension) => Err(not_downloaded(url, &extension)),
			Verdict::Warn => {
				let warning = warning(url, info);
				if self.ask.as_mut().is_some_and(|ask| ask(&warning)) {
					Ok(())
				} else {
					bail!("{warning}, so it was not downloaded")
				}
			}
		}
	}

	/// A new, empty folder for one download inside the run's folder, which is created first if
	/// this is the run's first download.
	fn next_folder(&mut self) -> Result<PathBuf> {
		let root = if let Some(root) = &self.root {
			root.path().to_path_buf()
		} else {
			let root = tempfile::Builder::new()
				.prefix("pb-")
				.tempdir_in(&self.parent)
				.with_context(|| format!("failed to create a download folder in {}", self.parent.display()))?;
			let path = root.path().to_path_buf();
			self.root = Some(root);
			path
		};
		self.count += 1;
		let folder = root.join(self.count.to_string());
		fs::create_dir(&folder).with_context(|| format!("failed to create the folder {}", folder.display()))?;
		Ok(folder)
	}

	/// Removes the run's download folder.
	pub fn remove(&mut self) {
		drop(self.root.take());
	}
}

fn not_downloaded(url: &str, extension: &str) -> anyhow::Error {
	anyhow!(
		"{url}: pb does not read .{extension} files, so it was not downloaded\nIf it is a document, download it yourself and pass the file instead"
	)
}

/// The security warning for a link, naming what the server sends.
fn warning(url: &str, info: &RemoteInfo) -> String {
	let content_type = info.content_type.as_deref().unwrap_or("no content type");
	if fetch::is_web_page(content_type) {
		format!(
			"{url}: security warning: the server sends a web page ({content_type}) rather than the document the link names"
		)
	} else {
		format!("{url}: security warning: the server sends {content_type}, which is not a format pb reads")
	}
}

/// Asks on the console whether to download a link that got `warning`; no when there is no
/// console to ask on.
pub fn ask_console(warning: &str) -> bool {
	#[cfg(windows)]
	const CONSOLE: &str = "CONIN$";
	#[cfg(not(windows))]
	const CONSOLE: &str = "/dev/tty";
	let Ok(console) = File::open(CONSOLE) else { return false };
	eprint!("pb: {warning}\nDownload it anyway? [y/N] ");
	let mut answer = String::new();
	BufReader::new(console).read_line(&mut answer).is_ok() && is_yes(&answer)
}

/// Whether a typed answer is yes: `y` or `yes` in any case.
fn is_yes(answer: &str) -> bool {
	matches!(answer.trim().to_ascii_lowercase().as_str(), "y" | "yes")
}

#[cfg(test)]
mod tests {
	use std::{
		cell::RefCell,
		env, fs,
		path::{Path, PathBuf},
		process,
		rc::Rc,
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

	fn entries(path: &Path) -> usize {
		fs::read_dir(path).expect("read the folder").count()
	}

	#[test]
	fn a_link_to_a_type_pb_does_not_read_is_refused_without_a_download() {
		let dir = TempDir::new("refused");
		let mut downloads = Downloads::in_folder(dir.path.clone());
		let error = downloads.fetch("https://example.invalid/setup.exe").err().expect("refused").to_string();
		assert!(error.contains("https://example.invalid/setup.exe"), "{error}");
		assert!(error.contains("not downloaded"), "{error}");
		assert!(error.contains("download it yourself"), "{error}");
		assert_eq!(entries(&dir.path), 0, "a refused link left a folder behind");
	}

	#[test]
	fn each_run_downloads_into_a_new_folder_of_its_own() {
		let dir = TempDir::new("own-folder");
		let mut one = Downloads::in_folder(dir.path.clone());
		let mut two = Downloads::in_folder(dir.path.clone());
		let (first, second) = (one.next_folder().expect("a folder"), two.next_folder().expect("a folder"));
		assert_ne!(first.parent(), second.parent());
		for folder in [&first, &second] {
			assert!(folder.starts_with(&dir.path), "{}", folder.display());
			let run = folder.parent().and_then(Path::file_name).expect("a run folder").to_string_lossy();
			assert!(run.starts_with("pb-"), "{run}");
		}
	}

	#[test]
	fn removing_the_downloads_removes_the_run_folder() {
		let dir = TempDir::new("remove");
		let mut downloads = Downloads::in_folder(dir.path.clone());
		let folder = downloads.next_folder().expect("a folder");
		assert!(folder.exists());
		downloads.remove();
		assert_eq!(entries(&dir.path), 0);
	}

	const LINK: &str = "https://example.org/download?id=3";

	fn info(final_url: &str, content_type: &str) -> RemoteInfo {
		RemoteInfo {
			final_url: final_url.to_string(),
			disposition_name: None,
			content_type: Some(content_type.to_string()),
			size: None,
			file_name: "download".to_string(),
		}
	}

	#[test]
	fn a_warned_link_is_downloaded_when_the_reader_says_yes() {
		let mut downloads = Downloads::default().with_prompt(|_| true);
		downloads.allowed(LINK, &info(LINK, "application/octet-stream")).expect("the reader said yes");
	}

	#[test]
	fn a_warned_link_is_refused_when_the_reader_says_no() {
		let mut downloads = Downloads::default().with_prompt(|_| false);
		let error = downloads.allowed(LINK, &info(LINK, "application/octet-stream")).expect_err("no").to_string();
		assert!(error.contains("security warning") && error.contains("not downloaded"), "{error}");
	}

	#[test]
	fn a_warned_link_is_refused_without_a_question_when_prompting_is_off() {
		let error = Downloads::default().allowed(LINK, &info(LINK, "application/octet-stream")).expect_err("refused");
		assert!(error.to_string().contains("not downloaded"), "{error}");
	}

	#[test]
	fn the_question_names_the_link_and_what_the_server_sends() {
		let asked = Rc::new(RefCell::new(String::new()));
		let question = Rc::clone(&asked);
		let mut downloads = Downloads::default().with_prompt(move |warning| {
			*question.borrow_mut() = warning.to_string();
			false
		});
		let _ = downloads.allowed(LINK, &info(LINK, "application/octet-stream"));
		let asked = asked.borrow();
		assert!(asked.contains(LINK) && asked.contains("application/octet-stream"), "{asked}");
	}

	#[test]
	fn a_web_page_is_called_one_in_the_warning() {
		let page = "https://github.com/o/r/blob/main/book.pdf";
		let error = Downloads::default().allowed(page, &info(page, "text/html")).expect_err("a web page").to_string();
		assert!(error.contains("web page"), "{error}");
	}

	#[test]
	fn a_refused_link_is_never_asked_about() {
		let mut downloads = Downloads::default().with_prompt(|_| panic!("a refused link was asked about"));
		let exe = "https://example.org/setup.exe";
		downloads.allowed(exe, &info(exe, "application/octet-stream")).expect_err("refused");
	}

	#[test]
	fn yes_is_y_or_yes_in_any_case() {
		for answer in ["y", "Y\r\n", " yes\n", "YES"] {
			assert!(is_yes(answer), "{answer:?}");
		}
		for answer in ["", "\n", "n", "no", "yep"] {
			assert!(!is_yes(answer), "{answer:?}");
		}
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
