use std::{
	error::Error,
	fmt::{Display, Formatter, Result as FmtResult},
	fs::File,
	io::Read,
	time::Duration,
};

use serde::Deserialize;
use ureq::{Agent, config::Config};

use crate::version;

const RELEASE_URL: &str = "https://api.github.com/repos/trypsynth/paperback/releases/latest";

#[derive(Debug, Deserialize)]
struct ReleaseAsset {
	name: String,
	browser_download_url: String,
}

#[derive(Debug, Deserialize)]
struct GithubRelease {
	tag_name: String,
	body: Option<String>,
	assets: Option<Vec<ReleaseAsset>>,
}

#[derive(Debug)]
pub struct UpdateAvailableResult {
	pub latest_version: String,
	pub download_url: String,
	pub release_notes: String,
}

#[derive(Debug)]
pub enum UpdateCheckOutcome {
	UpdateAvailable(UpdateAvailableResult),
	UpToDate(String),
}

#[derive(Debug)]
pub enum UpdateError {
	InvalidVersion(String),
	HttpError(i32),
	NetworkError(String),
	InvalidResponse(String),
	NoDownload(String),
}

impl Display for UpdateError {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		match self {
			Self::InvalidVersion(msg) => write!(f, "Invalid version: {msg}"),
			Self::HttpError(code) => write!(f, "HTTP error: {code}"),
			Self::NetworkError(msg) => write!(f, "Network error: {msg}"),
			Self::InvalidResponse(msg) => write!(f, "Invalid response: {msg}"),
			Self::NoDownload(msg) => write!(f, "No download: {msg}"),
		}
	}
}

impl Error for UpdateError {}

pub fn download_update_file(
	url: &str,
	mut progress_callback: impl FnMut(u64, u64),
) -> Result<std::path::PathBuf, UpdateError> {
	let user_agent = version::user_agent();
	let config = Config::builder()
		.timeout_connect(Some(Duration::from_secs(30)))
		.timeout_global(Some(Duration::from_secs(600)))
		.build();
	let agent = Agent::new_with_config(config);
	let resp = agent.get(url).header("User-Agent", &user_agent).call().map_err(|err| match err {
		ureq::Error::StatusCode(code) => UpdateError::HttpError(i32::from(code)),
		_ => UpdateError::NetworkError(format!("Network error: {err}")),
	})?;

	let total_size = resp
		.headers()
		.get("Content-Length")
		.and_then(|v| v.to_str().ok())
		.and_then(|v| v.parse::<u64>().ok())
		.unwrap_or(0);
	let fname = url.rsplit('/').next().unwrap_or("update.bin");
	let mut dest_path = if fname.ends_with(".exe") {
		std::env::temp_dir()
	} else if fname.ends_with(".zip") {
		std::env::current_exe()
			.map_err(|e| UpdateError::NoDownload(format!("Failed to determine exe path: {e}")))?
			.parent()
			.ok_or_else(|| UpdateError::NoDownload("Failed to get exe directory".to_string()))?
			.to_path_buf()
	} else {
		std::env::temp_dir()
	};

	dest_path.push(fname);

	let mut file =
		File::create(&dest_path).map_err(|e| UpdateError::NoDownload(format!("Failed to create file: {e}")))?;

	let mut downloaded: u64 = 0;
	let mut buffer = [0; 8192];
	let mut body = resp.into_body();
	let mut reader = body.as_reader();

	loop {
		let n = reader.read(&mut buffer).map_err(|e| UpdateError::NetworkError(e.to_string()))?;
		if n == 0 {
			break;
		}
		std::io::Write::write_all(&mut file, &buffer[..n])
			.map_err(|e| UpdateError::NoDownload(format!("Failed to write to file: {e}")))?;
		downloaded += n as u64;
		progress_callback(downloaded, total_size);
	}

	Ok(dest_path)
}

fn parse_semver_value(value: &str) -> Option<(u64, u64, u64)> {
	let trimmed = value.trim();
	if trimmed.is_empty() {
		return None;
	}
	let normalized = trimmed.trim_start_matches(['v', 'V']);
	let mut parts = normalized.split('.').map(|p| p.split_once('-').map_or(p, |(v, _)| v));
	let major = parts.next()?.parse().ok()?;
	let minor = parts.next().unwrap_or("0").parse().ok()?;
	let patch = parts.next().unwrap_or("0").parse().ok()?;
	Some((major, minor, patch))
}

fn pick_download_url(is_installer: bool, assets: &[ReleaseAsset]) -> Option<String> {
	let preferred_name = if is_installer { "paperback_setup.exe" } else { "paperback.zip" };
	for asset in assets {
		if asset.name.eq_ignore_ascii_case(preferred_name) {
			return Some(asset.browser_download_url.clone());
		}
	}
	None
}

fn fetch_latest_release() -> Result<GithubRelease, UpdateError> {
	let user_agent = version::user_agent();
	let config = Config::builder().timeout_global(Some(Duration::from_secs(15))).build();
	let agent = Agent::new_with_config(config);
	let mut resp = agent
		.get(RELEASE_URL)
		.header("User-Agent", &user_agent)
		.header("Accept", "application/vnd.github+json")
		.call()
		.map_err(|err| match err {
			ureq::Error::StatusCode(code) => UpdateError::HttpError(i32::from(code)),
			_ => UpdateError::NetworkError(format!("Network error: {err}")),
		})?;
	resp.body_mut()
		.read_json::<GithubRelease>()
		.map_err(|err| UpdateError::InvalidResponse(format!("Failed to parse release JSON: {err}")))
}

pub fn check_for_updates(current_version: &str, is_installer: bool) -> Result<UpdateCheckOutcome, UpdateError> {
	let current = parse_semver_value(current_version)
		.ok_or_else(|| UpdateError::InvalidVersion("Current version was not a valid semantic version.".to_string()))?;
	let release = fetch_latest_release()?;
	let latest_semver = parse_semver_value(&release.tag_name).ok_or_else(|| {
		UpdateError::InvalidResponse("Latest release tag does not contain a valid semantic version.".to_string())
	})?;
	if current >= latest_semver {
		return Ok(UpdateCheckOutcome::UpToDate(release.tag_name));
	}
	let download_url = match release.assets.as_ref() {
		Some(list) if !list.is_empty() => pick_download_url(is_installer, list).ok_or_else(|| {
			UpdateError::NoDownload("Update is available but no matching download asset was found.".to_string())
		})?,
		_ => return Err(UpdateError::NoDownload("Latest release does not include downloadable assets.".to_string())),
	};
	Ok(UpdateCheckOutcome::UpdateAvailable(UpdateAvailableResult {
		latest_version: release.tag_name,
		download_url,
		release_notes: release.body.unwrap_or_default(),
	}))
}
