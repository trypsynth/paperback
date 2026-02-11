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
const WINDOWS_INSTALLER_ASSETS: &[&str] = &["paperback_setup.exe"];
const WINDOWS_PORTABLE_ASSETS: &[&str] = &["paperback_windows.zip", "paperback.zip"];
const MACOS_ASSETS: &[&str] = &["paperback_mac.zip", "paperback_macos.zip"];
const LINUX_ASSETS: &[&str] = &[
	"paperback_linux.zip",
	"paperback-linux.zip",
	"paperback_linux.tar.gz",
	"paperback-linux.tar.gz",
	"paperback.appimage",
];
const GENERIC_ASSETS: &[&str] = &["paperback.zip"];

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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum UpdateTarget {
	Windows,
	MacOs,
	Linux,
	Other,
}

impl UpdateTarget {
	const fn current() -> Self {
		if cfg!(target_os = "windows") {
			Self::Windows
		} else if cfg!(target_os = "macos") {
			Self::MacOs
		} else if cfg!(target_os = "linux") {
			Self::Linux
		} else {
			Self::Other
		}
	}
}

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
	let is_zip = Path::new(fname).extension().is_some_and(|ext| ext.eq_ignore_ascii_case("zip"));
	let mut dest_path = if matches!(UpdateTarget::current(), UpdateTarget::Windows) && is_zip {
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

fn preferred_assets(target: UpdateTarget, is_installer: bool) -> &'static [&'static str] {
	match (target, is_installer) {
		(UpdateTarget::Windows, true) => WINDOWS_INSTALLER_ASSETS,
		(UpdateTarget::Windows, false) => WINDOWS_PORTABLE_ASSETS,
		(UpdateTarget::MacOs, _) => MACOS_ASSETS,
		(UpdateTarget::Linux, _) => LINUX_ASSETS,
		(UpdateTarget::Other, _) => GENERIC_ASSETS,
	}
}

fn is_tar_gz(name: &str) -> bool {
	name.ends_with(".tar.gz")
}

fn is_archive(name: &str) -> bool {
	name.ends_with(".zip") || is_tar_gz(name)
}

fn is_linux_package(name: &str) -> bool {
	is_archive(name) || name.ends_with(".appimage") || name.ends_with(".deb") || name.ends_with(".rpm")
}

fn pick_fallback_asset(target: UpdateTarget, is_installer: bool, assets: &[ReleaseAsset]) -> Option<String> {
	match target {
		UpdateTarget::Windows if is_installer => {
			for asset in assets {
				let name = asset.name.to_ascii_lowercase();
				if name.contains("setup") && name.ends_with(".exe") {
					return Some(asset.browser_download_url.clone());
				}
			}
			for asset in assets {
				let name = asset.name.to_ascii_lowercase();
				if name.ends_with(".exe") {
					return Some(asset.browser_download_url.clone());
				}
			}
		}
		UpdateTarget::Windows => {
			for asset in assets {
				let name = asset.name.to_ascii_lowercase();
				if name.contains("windows") && is_archive(&name) {
					return Some(asset.browser_download_url.clone());
				}
			}
			for asset in assets {
				let name = asset.name.to_ascii_lowercase();
				if name == "paperback.zip" {
					return Some(asset.browser_download_url.clone());
				}
			}
			for asset in assets {
				let name = asset.name.to_ascii_lowercase();
				if is_archive(&name) {
					return Some(asset.browser_download_url.clone());
				}
			}
		}
		UpdateTarget::MacOs => {
			for asset in assets {
				let name = asset.name.to_ascii_lowercase();
				if name.contains("mac") && (is_archive(&name) || name.ends_with(".dmg")) {
					return Some(asset.browser_download_url.clone());
				}
			}
			for asset in assets {
				let name = asset.name.to_ascii_lowercase();
				if name.ends_with(".dmg") || is_archive(&name) {
					return Some(asset.browser_download_url.clone());
				}
			}
		}
		UpdateTarget::Linux => {
			for asset in assets {
				let name = asset.name.to_ascii_lowercase();
				if name.contains("linux") && is_linux_package(&name) {
					return Some(asset.browser_download_url.clone());
				}
			}
			for asset in assets {
				let name = asset.name.to_ascii_lowercase();
				if is_linux_package(&name) {
					return Some(asset.browser_download_url.clone());
				}
			}
		}
		UpdateTarget::Other => {
			for asset in assets {
				let name = asset.name.to_ascii_lowercase();
				if name.ends_with(".zip") {
					return Some(asset.browser_download_url.clone());
				}
			}
		}
	}
	None
}

fn pick_download_url(target: UpdateTarget, is_installer: bool, assets: &[ReleaseAsset]) -> Option<String> {
	let preferred_names = preferred_assets(target, is_installer);
	for asset in assets {
		if preferred_names.iter().any(|name| asset.name.eq_ignore_ascii_case(name)) {
			return Some(asset.browser_download_url.clone());
		}
	}
	pick_fallback_asset(target, is_installer, assets)
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
	let target = UpdateTarget::current();
	let release = fetch_latest_release()?;
	let latest_semver = parse_semver_value(&release.tag_name).ok_or_else(|| {
		UpdateError::InvalidResponse("Latest release tag does not contain a valid semantic version.".to_string())
	})?;
	if current >= latest_semver {
		return Ok(UpdateCheckOutcome::UpToDate(release.tag_name));
	}
	let download_url = match release.assets.as_ref() {
		Some(list) if !list.is_empty() => pick_download_url(target, is_installer, list).ok_or_else(|| {
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
		let url = pick_download_url(UpdateTarget::Windows, true, &assets);
		assert_eq!(url.as_deref(), Some("https://example.com/paperback_setup.exe"));
	}

	#[test]
	fn pick_download_url_accepts_case_insensitive_matches() {
		let assets = vec![ReleaseAsset {
			name: "PAPERBACK.ZIP".to_string(),
			browser_download_url: "https://example.com/PAPERBACK.ZIP".to_string(),
		}];
		let url = pick_download_url(UpdateTarget::Windows, false, &assets);
		assert_eq!(url.as_deref(), Some("https://example.com/PAPERBACK.ZIP"));
	}

	#[test]
	fn pick_download_url_prefers_linux_assets() {
		let assets = vec![
			ReleaseAsset {
				name: "paperback.zip".to_string(),
				browser_download_url: "https://example.com/paperback.zip".to_string(),
			},
			ReleaseAsset {
				name: "paperback_linux.zip".to_string(),
				browser_download_url: "https://example.com/paperback_linux.zip".to_string(),
			},
		];
		let url = pick_download_url(UpdateTarget::Linux, false, &assets);
		assert_eq!(url.as_deref(), Some("https://example.com/paperback_linux.zip"));
	}

	#[test]
	fn pick_download_url_prefers_macos_assets() {
		let assets = vec![
			ReleaseAsset {
				name: "paperback_windows.zip".to_string(),
				browser_download_url: "https://example.com/paperback_windows.zip".to_string(),
			},
			ReleaseAsset {
				name: "paperback_mac.zip".to_string(),
				browser_download_url: "https://example.com/paperback_mac.zip".to_string(),
			},
		];
		let url = pick_download_url(UpdateTarget::MacOs, false, &assets);
		assert_eq!(url.as_deref(), Some("https://example.com/paperback_mac.zip"));
	}

	#[test]
	fn pick_download_url_linux_falls_back_to_generic_zip() {
		let assets = vec![ReleaseAsset {
			name: "paperback.zip".to_string(),
			browser_download_url: "https://example.com/paperback.zip".to_string(),
		}];
		let url = pick_download_url(UpdateTarget::Linux, false, &assets);
		assert_eq!(url.as_deref(), Some("https://example.com/paperback.zip"));
	}
}
