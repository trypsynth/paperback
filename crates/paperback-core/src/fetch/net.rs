//! Asking a server what a link holds, and downloading it.

use std::{
	fs::{self, File},
	io::{self, Read, Write},
	path::{Path, PathBuf},
	sync::atomic::{AtomicBool, Ordering},
	time::Duration,
};

use ureq::{Agent, Body, ResponseExt, config::Config, http::Response};

use super::{Verdict, disposition_file_name, file_name_for, verdict};
use crate::version::user_agent;

const CONNECT_TIMEOUT: Duration = Duration::from_secs(30);
const OVERALL_TIMEOUT: Duration = Duration::from_mins(10);
const CHUNK_SIZE: usize = 8192;

/// What a server says about a link before its body is read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoteInfo {
	/// The link after redirects.
	pub final_url: String,
	/// The file name from `Content-Disposition`, made safe to save under.
	pub disposition_name: Option<String>,
	pub content_type: Option<String>,
	/// `Content-Length`, when the server sends one.
	pub size: Option<u64>,
	/// The name the working copy is saved under (see [`file_name_for`]).
	pub file_name: String,
}

impl RemoteInfo {
	/// The [`Verdict`] for this link, given as `given_url`.
	#[must_use]
	pub fn verdict(&self, given_url: &str) -> Verdict {
		verdict(given_url, &self.final_url, self.disposition_name.as_deref(), self.content_type.as_deref())
	}
}

#[derive(Debug, thiserror::Error)]
pub enum FetchError {
	#[error("the server answered with status {0}")]
	Http(u16),
	#[error("the server could not be reached: {0}")]
	Network(String),
	#[error("the download could not be saved: {0}")]
	Io(String),
	#[error("the download was cancelled")]
	Cancelled,
	#[error(".{0} files cannot be read, so the link was not downloaded")]
	Refused(String),
}

impl From<ureq::Error> for FetchError {
	fn from(error: ureq::Error) -> Self {
		match error {
			ureq::Error::StatusCode(code) => Self::Http(code),
			other => Self::Network(other.to_string()),
		}
	}
}

impl From<io::Error> for FetchError {
	fn from(error: io::Error) -> Self {
		Self::Io(error.to_string())
	}
}

fn agent() -> Agent {
	let config = Config::builder().timeout_connect(Some(CONNECT_TIMEOUT)).timeout_global(Some(OVERALL_TIMEOUT)).build();
	Agent::new_with_config(config)
}

/// Asks the server what `url` holds with a HEAD request. When HEAD gets an error status, a GET is
/// sent instead and only its headers are read.
///
/// # Errors
///
/// Returns [`FetchError::Http`] for an error status on the GET, [`FetchError::Network`] when the
/// server cannot be reached.
pub fn probe(url: &str) -> Result<RemoteInfo, FetchError> {
	let agent = agent();
	let response = match agent.head(url).header("User-Agent", &user_agent()).call() {
		Err(ureq::Error::StatusCode(_)) => agent.get(url).header("User-Agent", &user_agent()).call()?,
		other => other?,
	};
	Ok(info_from(&response))
}

/// Downloads `url` into `dest` + `.part` and renames it over `dest` once complete; `progress`
/// gets the bytes so far and the expected total. `cancel` is checked before each chunk.
///
/// # Errors
///
/// Returns [`FetchError::Refused`] when the link ends at a name whose extension no parser reads,
/// [`FetchError::Cancelled`] when `cancel` is set, and the other variants for network, status and
/// disk failures. `dest` is left as it was in every error case.
pub fn download(
	url: &str,
	dest: &Path,
	cancel: &AtomicBool,
	mut progress: impl FnMut(u64, Option<u64>),
) -> Result<u64, FetchError> {
	let response = agent().get(url).header("User-Agent", &user_agent()).call()?;
	let info = info_from(&response);
	if let Verdict::Refuse(extension) = info.verdict(url) {
		return Err(FetchError::Refused(extension));
	}
	let part = PartFile(part_path(dest));
	let mut file = File::create(&part.0)?;
	let mut reader = response.into_body().into_reader();
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
		file.write_all(&buffer[..read])?;
		done += read as u64;
		progress(done, info.size);
	}
	drop(file);
	fs::rename(&part.0, dest)?;
	Ok(done)
}

fn info_from(response: &Response<Body>) -> RemoteInfo {
	let header = |name: &str| response.headers().get(name).and_then(|value| value.to_str().ok()).map(str::to_string);
	let final_url = response.get_uri().to_string();
	let disposition_name = header("content-disposition").as_deref().and_then(disposition_file_name);
	let content_type = header("content-type");
	let size = header("content-length").and_then(|value| value.parse().ok());
	let file_name = file_name_for(&final_url, disposition_name.as_deref(), content_type.as_deref());
	RemoteInfo { final_url, disposition_name, content_type, size, file_name }
}

fn part_path(dest: &Path) -> PathBuf {
	let mut name = dest.as_os_str().to_owned();
	name.push(".part");
	PathBuf::from(name)
}

/// Removes the partial file when dropped; after the rename there is nothing left to remove.
struct PartFile(PathBuf);

impl Drop for PartFile {
	fn drop(&mut self) {
		let _ = fs::remove_file(&self.0);
	}
}

#[cfg(test)]
mod tests;
