use std::{
	env,
	error::Error,
	fmt::{Display, Formatter, Result as FmtResult},
	fs::File,
	io::{Read, Write},
	path::{Path, PathBuf},
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

pub fn download_update_file(url: &str, mut progress_callback: impl FnMut(u64, u64)) -> Result<PathBuf, UpdateError> {
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
	let is_exe = Path::new(fname).extension().is_some_and(|ext| ext.eq_ignore_ascii_case("exe"));
	let is_zip = Path::new(fname).extension().is_some_and(|ext| ext.eq_ignore_ascii_case("zip"));
	let mut dest_path = if is_exe {
		env::temp_dir()
	} else if is_zip {
		env::current_exe()
			.map_err(|e| UpdateError::NoDownload(format!("Failed to determine exe path: {e}")))?
			.parent()
			.ok_or_else(|| UpdateError::NoDownload("Failed to get exe directory".to_string()))?
			.to_path_buf()
	} else {
		env::temp_dir()
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
		Write::write_all(&mut file, &buffer[..n])
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

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parse_semver_accepts_prefixes_and_suffixes() {
		assert_eq!(parse_semver_value("v1.2.3"), Some((1, 2, 3)));
		assert_eq!(parse_semver_value("V4.5.6"), Some((4, 5, 6)));
		assert_eq!(parse_semver_value("1.2.3-beta.1"), Some((1, 2, 3)));
	}

	#[test]
	fn parse_semver_defaults_missing_parts() {
		assert_eq!(parse_semver_value("1"), Some((1, 0, 0)));
		assert_eq!(parse_semver_value("1.2"), Some((1, 2, 0)));
	}

	#[test]
	fn parse_semver_rejects_empty_or_invalid() {
		assert_eq!(parse_semver_value(""), None);
		assert_eq!(parse_semver_value("not-a-version"), None);
		assert_eq!(parse_semver_value("v"), None);
	}

	#[test]
	fn parse_semver_trims_whitespace() {
		assert_eq!(parse_semver_value("  v2.3.4  "), Some((2, 3, 4)));
	}

	#[test]
	fn parse_semver_ignores_extra_segments_after_patch() {
		assert_eq!(parse_semver_value("1.2.3.99"), Some((1, 2, 3)));
	}

	#[test]
	fn parse_semver_rejects_missing_major_component() {
		assert_eq!(parse_semver_value(".2.3"), None);
	}

	#[test]
	fn pick_download_url_prefers_installer() {
		let assets = vec![
			ReleaseAsset {
				name: "paperback.zip".to_string(),
				browser_download_url: "https://example.com/paperback.zip".to_string(),
			},
			ReleaseAsset {
				name: "paperback_setup.exe".to_string(),
				browser_download_url: "https://example.com/paperback_setup.exe".to_string(),
			},
		];
		let url = pick_download_url(true, &assets);
		assert_eq!(url.as_deref(), Some("https://example.com/paperback_setup.exe"));
	}

	#[test]
	fn pick_download_url_accepts_case_insensitive_matches() {
		let assets = vec![ReleaseAsset {
			name: "PAPERBACK.ZIP".to_string(),
			browser_download_url: "https://example.com/PAPERBACK.ZIP".to_string(),
		}];
		let url = pick_download_url(false, &assets);
		assert_eq!(url.as_deref(), Some("https://example.com/PAPERBACK.ZIP"));
	}

	#[test]
	fn pick_download_url_returns_none_when_preferred_asset_missing() {
		let assets = vec![ReleaseAsset {
			name: "notes.txt".to_string(),
			browser_download_url: "https://example.com/notes.txt".to_string(),
		}];
		assert!(pick_download_url(true, &assets).is_none());
		assert!(pick_download_url(false, &assets).is_none());
	}

	#[test]
	fn pick_download_url_uses_flag_to_choose_between_exe_and_zip() {
		let assets = vec![
			ReleaseAsset {
				name: "paperback.zip".to_string(),
				browser_download_url: "https://example.com/paperback.zip".to_string(),
			},
			ReleaseAsset {
				name: "paperback_setup.exe".to_string(),
				browser_download_url: "https://example.com/paperback_setup.exe".to_string(),
			},
		];
		assert_eq!(pick_download_url(false, &assets).as_deref(), Some("https://example.com/paperback.zip"));
		assert_eq!(pick_download_url(true, &assets).as_deref(), Some("https://example.com/paperback_setup.exe"));
	}
}
