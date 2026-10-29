use std::{
	fmt,
	fs::{self, File},
	io::{self, Read, Write},
	path::{Path, PathBuf},
	sync::atomic::{AtomicBool, Ordering},
	time::Duration,
};

use patois::t;
use ureq::{Agent, Body, ResponseExt, config::Config, http::Response};

use super::{Verdict, disposition_file_name, file_name_for, verdict};
use crate::version::user_agent;

const CONNECT_TIMEOUT: Duration = Duration::from_secs(30);
const OVERALL_TIMEOUT: Duration = Duration::from_mins(10);
const CHUNK_SIZE: usize = 8192;
pub const DEFAULT_MAX_SIZE: u64 = 512 * 1024 * 1024;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoteInfo {
	pub final_url: String,
	pub disposition_name: Option<String>,
	pub content_type: Option<String>,
	pub size: Option<u64>,
	pub file_name: String,
}

impl RemoteInfo {
	#[must_use]
	pub fn verdict(&self, given_url: &str) -> Verdict {
		verdict(given_url, &self.final_url, self.disposition_name.as_deref(), self.content_type.as_deref())
	}
}

#[derive(Debug)]
pub enum FetchError {
	Http(u16),
	Network(String),
	Io(String),
	Cancelled,
	Refused(String),
	Insecure,
	/// The limit, in bytes.
	TooLarge(u64),
}

impl fmt::Display for FetchError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let message = match self {
			// TRANSLATORS: Error when downloading a document from a link fails; {} is the HTTP status code, such as 404
			Self::Http(code) => t("the server answered with status {}").replace("{}", &code.to_string()),
			// TRANSLATORS: Error when the server of a link cannot be reached; {} is the technical reason
			Self::Network(reason) => t("the server could not be reached: {}").replace("{}", reason),
			// TRANSLATORS: Error when a document downloaded from a link cannot be saved; {} is the technical reason
			Self::Io(reason) => t("the download could not be saved: {}").replace("{}", reason),
			// TRANSLATORS: Message when downloading a document from a link is cancelled
			Self::Cancelled => t("the download was cancelled"),
			Self::Refused(extension) => {
				// TRANSLATORS: Error when a link names a file type that is never downloaded; {} is the extension without the leading dot
				t(".{} files cannot be read, so the link was not downloaded").replace("{}", extension)
			}
			// TRANSLATORS: Error when a secure https link is redirected to an insecure http address
			Self::Insecure => t("the secure link was redirected to an insecure address, so it was not downloaded"),
			// TRANSLATORS: Error when a document behind a link is too large to download; {} is the limit in megabytes
			Self::TooLarge(limit) => t("the document is larger than {} MB, so it was not downloaded")
				.replace("{}", &(limit / (1024 * 1024)).to_string()),
		};
		f.write_str(&message)
	}
}

impl std::error::Error for FetchError {}

impl From<ureq::Error> for FetchError {
	fn from(error: ureq::Error) -> Self {
		match error {
			ureq::Error::StatusCode(code) => Self::Http(code),
			ureq::Error::RequireHttpsOnly(_) => Self::Insecure,
			other => Self::Network(other.to_string()),
		}
	}
}

impl From<io::Error> for FetchError {
	fn from(error: io::Error) -> Self {
		Self::Io(error.to_string())
	}
}

fn config_for(url: &str) -> Config {
	Config::builder()
		.timeout_connect(Some(CONNECT_TIMEOUT))
		.timeout_global(Some(OVERALL_TIMEOUT))
		.https_only(url.get(..8).is_some_and(|scheme| scheme.eq_ignore_ascii_case("https://")))
		.build()
}

pub struct Remote {
	given_url: String,
	info: RemoteInfo,
	body: Body,
	max_size: u64,
}

pub fn open(url: &str, max_size: u64) -> Result<Remote, FetchError> {
	let response = Agent::new_with_config(config_for(url)).get(url).header("User-Agent", &user_agent()).call()?;
	let info = info_from(&response, url);
	if info.size.is_some_and(|size| size > max_size) {
		return Err(FetchError::TooLarge(max_size));
	}
	Ok(Remote { given_url: url.to_string(), info, body: response.into_body(), max_size })
}

impl Remote {
	#[must_use]
	pub const fn info(&self) -> &RemoteInfo {
		&self.info
	}

	/// Streams into a `.part` file renamed over `dest` when complete, so `dest` is untouched on every error.
	pub fn save(
		self,
		dest: &Path,
		cancel: &AtomicBool,
		mut progress: impl FnMut(u64, Option<u64>),
	) -> Result<u64, FetchError> {
		if let Verdict::Refuse(extension) = self.info.verdict(&self.given_url) {
			return Err(FetchError::Refused(extension));
		}
		let mut part = PartFile::create(part_path(dest))?;
		let mut reader = self.body.into_reader();
		let mut buffer = [0u8; CHUNK_SIZE];
		let mut done = 0u64;
		loop {
			if cancel.load(Ordering::Relaxed) {
				return Err(FetchError::Cancelled);
			}
			let read = reader.read(&mut buffer).map_err(|error| FetchError::Network(error.to_string()))?;
			if read == 0 {
				break;
			}
			done += read as u64;
			if done > self.max_size {
				return Err(FetchError::TooLarge(self.max_size));
			}
			part.write_all(&buffer[..read])?;
			progress(done, self.info.size);
		}
		part.finish(dest)?;
		Ok(done)
	}
}

fn info_from(response: &Response<Body>, given_url: &str) -> RemoteInfo {
	let header = |name: &str| response.headers().get(name).and_then(|value| value.to_str().ok()).map(str::to_string);
	let final_url = response.get_uri().to_string();
	let disposition_name = header("content-disposition").as_deref().and_then(disposition_file_name);
	let content_type = header("content-type");
	let size = header("content-length").and_then(|value| value.parse().ok());
	let file_name = file_name_for(given_url, &final_url, disposition_name.as_deref(), content_type.as_deref());
	RemoteInfo { final_url, disposition_name, content_type, size, file_name }
}

fn part_path(dest: &Path) -> PathBuf {
	let mut name = dest.as_os_str().to_owned();
	name.push(".part");
	PathBuf::from(name)
}

/// Created only where no file exists yet, since dropping it removes the file.
struct PartFile {
	path: PathBuf,
	file: Option<File>,
}

impl PartFile {
	fn create(path: PathBuf) -> io::Result<Self> {
		let file = File::create_new(&path)?;
		Ok(Self { path, file: Some(file) })
	}

	fn write_all(&mut self, bytes: &[u8]) -> io::Result<()> {
		self.file.as_mut().map_or_else(
			|| Err(io::Error::other("the partial download is already closed")),
			|file| file.write_all(bytes),
		)
	}

	fn finish(mut self, dest: &Path) -> io::Result<()> {
		drop(self.file.take());
		fs::rename(&self.path, dest)
	}
}

impl Drop for PartFile {
	fn drop(&mut self) {
		drop(self.file.take());
		let _ = fs::remove_file(&self.path);
	}
}

#[cfg(test)]
mod tests;
