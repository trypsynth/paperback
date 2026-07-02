#![warn(clippy::all, clippy::cargo, clippy::nursery, clippy::pedantic)]

use std::{
	env,
	error::Error,
	fmt::{Display, Formatter, Result as FmtResult},
	fs::{self, File},
	io::{Read, Write},
	path::{Path, PathBuf},
	str,
	sync::atomic::{AtomicBool, Ordering},
	time::Duration,
};

use minisign_verify::{PublicKey, Signature};
use serde::Deserialize;
use ureq::{Agent, config::Config};

/// Configuration for the updater. Construct once and pass to all ship-shape functions.
#[derive(Clone)]
pub struct UpdaterConfig {
	/// GitHub repository in `"owner/repo"` format.
	pub github_repo: String,
	/// App name used to derive asset file names.
	/// A zip asset is expected to be `{app_name}.zip` and an installer `{app_name}_setup.exe`.
	pub app_name: String,
	/// Human-readable app name used in dialog titles and messages (e.g. `"Paperback"`).
	pub app_display_name: String,
	/// Base64-encoded minisign public key used to verify downloaded files.
	pub minisign_public_key: String,
	/// Value sent as the `User-Agent` header for all HTTP requests.
	pub user_agent: String,
}

impl UpdaterConfig {
	pub fn new(
		github_repo: impl Into<String>,
		app_name: impl Into<String>,
		app_display_name: impl Into<String>,
		minisign_public_key: impl Into<String>,
		user_agent: impl Into<String>,
	) -> Self {
		Self {
			github_repo: github_repo.into(),
			app_name: app_name.into(),
			app_display_name: app_display_name.into(),
			minisign_public_key: minisign_public_key.into(),
			user_agent: user_agent.into(),
		}
	}
}

pub mod ui;

/// Which release stream to check against.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UpdateChannel {
	/// Compare semver tags against the latest GitHub release.
	#[default]
	Stable,
	/// Compare short commit hashes against a rolling `"latest"` pre-release tag.
	Dev,
}

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
	pub signature_url: String,
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
	VerificationError(String),
	/// The in-progress download was cancelled by the caller via the `cancelled` flag
	/// passed to [`download_update_file`].
	Cancelled,
}

impl Display for UpdateError {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		match self {
			Self::InvalidVersion(msg) => write!(f, "Invalid version: {msg}"),
			Self::HttpError(code) => write!(f, "HTTP error: {code}"),
			Self::NetworkError(msg) => write!(f, "Network error: {msg}"),
			Self::InvalidResponse(msg) => write!(f, "Invalid response: {msg}"),
			Self::NoDownload(msg) => write!(f, "No download: {msg}"),
			Self::VerificationError(msg) => write!(f, "Verification error: {msg}"),
			Self::Cancelled => write!(f, "Download cancelled"),
		}
	}
}

impl Error for UpdateError {}

/// Download `url`, verify its minisign signature, and return the local path of the verified file.
///
/// The signature is fetched from `signature_url`. `progress_callback(downloaded, total)` is called
/// after each chunk; `total` may be 0 if the server does not send `Content-Length`.
///
/// `cancelled` is polled between network operations and after every chunk read; as soon as it is
/// set to `true` the download stops, any partially written file is removed, and
/// [`UpdateError::Cancelled`] is returned. This keeps cancellation responsive instead of letting a
/// slow or stalled transfer run to completion (or time out) in the background after the user has
/// already dismissed the progress dialog.
///
/// `.exe` files land in the system temp directory; `.zip` files land next to the current
/// executable so that the extraction script can overwrite in-place.
///
/// # Errors
///
/// Returns [`UpdateError`] on network failure, HTTP error, I/O error, cancellation, or signature
/// verification failure.
pub fn download_update_file(
	config: &UpdaterConfig,
	url: &str,
	signature_url: &str,
	cancelled: &AtomicBool,
	mut progress_callback: impl FnMut(u64, u64),
) -> Result<PathBuf, UpdateError> {
	let http = build_agent(Some(Duration::from_mins(10)));
	let sig_resp =
		http.get(signature_url).header("User-Agent", &config.user_agent).call().map_err(|e| map_http_err(&e))?;
	let mut sig_bytes = Vec::new();
	sig_resp
		.into_body()
		.as_reader()
		.read_to_end(&mut sig_bytes)
		.map_err(|e| UpdateError::NetworkError(format!("Failed to read signature: {e}")))?;
	if cancelled.load(Ordering::Relaxed) {
		return Err(UpdateError::Cancelled);
	}
	let resp = http.get(url).header("User-Agent", &config.user_agent).call().map_err(|e| map_http_err(&e))?;
	let total_size = resp
		.headers()
		.get("Content-Length")
		.and_then(|v| v.to_str().ok())
		.and_then(|v| v.parse::<u64>().ok())
		.unwrap_or(0);
	let fname = url.rsplit('/').next().unwrap_or("update.bin");
	let ext = Path::new(fname).extension().map(std::ffi::OsStr::to_ascii_lowercase);
	let is_exe = ext.as_deref() == Some(std::ffi::OsStr::new("exe"));
	let is_zip = ext.as_deref() == Some(std::ffi::OsStr::new("zip"));
	let mut dest_dir = if is_exe {
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
	let tmp_fname = format!("{fname}.tmp");
	dest_dir.push(&tmp_fname);
	let tmp_path = dest_dir;
	let mut file =
		File::create(&tmp_path).map_err(|e| UpdateError::NoDownload(format!("Failed to create file: {e}")))?;
	let mut downloaded: u64 = 0;
	let mut buffer = [0u8; 8192];
	let mut body = resp.into_body();
	let mut reader = body.as_reader();
	loop {
		if cancelled.load(Ordering::Relaxed) {
			drop(file);
			let _ = fs::remove_file(&tmp_path);
			return Err(UpdateError::Cancelled);
		}
		let n = match reader.read(&mut buffer) {
			Ok(n) => n,
			Err(e) => {
				drop(file);
				let _ = fs::remove_file(&tmp_path);
				return Err(UpdateError::NetworkError(e.to_string()));
			}
		};
		if n == 0 {
			break;
		}
		if let Err(e) = Write::write_all(&mut file, &buffer[..n]) {
			drop(file);
			let _ = fs::remove_file(&tmp_path);
			return Err(UpdateError::NoDownload(format!("Failed to write file: {e}")));
		}
		downloaded += n as u64;
		progress_callback(downloaded, total_size);
	}
	drop(file);
	let data = match fs::read(&tmp_path) {
		Ok(data) => data,
		Err(e) => {
			let _ = fs::remove_file(&tmp_path);
			return Err(UpdateError::NoDownload(format!("Failed to read file for verification: {e}")));
		}
	};
	let pk = PublicKey::from_base64(&config.minisign_public_key)
		.map_err(|e| UpdateError::VerificationError(format!("Invalid public key: {e}")))?;
	let sig_str = str::from_utf8(&sig_bytes)
		.map_err(|e| UpdateError::VerificationError(format!("Signature is not valid UTF-8: {e}")))?;
	let sig =
		Signature::decode(sig_str).map_err(|e| UpdateError::VerificationError(format!("Invalid signature: {e}")))?;
	if let Err(e) = pk.verify(&data, &sig, true) {
		let _ = fs::remove_file(&tmp_path);
		return Err(UpdateError::VerificationError(format!("Signature verification failed: {e}")));
	}
	let mut final_path = tmp_path.clone();
	final_path.set_file_name(fname);
	if let Err(e) = fs::rename(&tmp_path, &final_path) {
		let _ = fs::remove_file(&tmp_path);
		return Err(UpdateError::NoDownload(format!("Failed to rename verified file: {e}")));
	}
	Ok(final_path)
}

/// Check whether an update is available.
///
/// - `current_version`: semver string used for [`UpdateChannel::Stable`] comparisons.
/// - `current_commit`: short or full git commit hash used for [`UpdateChannel::Dev`] comparisons.
/// - `is_installer` — selects between `{app_name}_setup.exe` and `{app_name}.zip` as the
///   preferred asset.
///
/// # Errors
///
/// Returns [`UpdateError`] on network failure, HTTP error, invalid version strings, or missing
/// release assets.
pub fn check_for_updates(
	config: &UpdaterConfig,
	current_version: &str,
	current_commit: &str,
	is_installer: bool,
	channel: UpdateChannel,
) -> Result<UpdateCheckOutcome, UpdateError> {
	match channel {
		UpdateChannel::Stable => check_stable(config, current_version, is_installer),
		UpdateChannel::Dev => check_dev(config, current_commit, is_installer),
	}
}

fn check_stable(
	config: &UpdaterConfig,
	current_version: &str,
	is_installer: bool,
) -> Result<UpdateCheckOutcome, UpdateError> {
	let current = parse_semver(current_version)
		.ok_or_else(|| UpdateError::InvalidVersion("Current version is not a valid semver.".to_string()))?;
	let release = fetch_latest_release(config)?;
	let latest = parse_semver(&release.tag_name)
		.ok_or_else(|| UpdateError::InvalidResponse("Latest release tag is not a valid semver.".to_string()))?;
	if current >= latest {
		return Ok(UpdateCheckOutcome::UpToDate(release.tag_name));
	}
	let (download_url, signature_url) = require_asset_pair(config, is_installer, &release)?;
	Ok(UpdateCheckOutcome::UpdateAvailable(UpdateAvailableResult {
		latest_version: release.tag_name,
		download_url,
		signature_url,
		release_notes: release.body.unwrap_or_default(),
	}))
}

fn check_dev(
	config: &UpdaterConfig,
	current_commit: &str,
	is_installer: bool,
) -> Result<UpdateCheckOutcome, UpdateError> {
	let release = fetch_release_by_tag(config, "latest")?;
	let raw_notes = release.body.clone().unwrap_or_default();
	let commit_lines: Vec<&str> = raw_notes.lines().filter(|l| l.trim().starts_with("- ")).collect();
	let short_current = if current_commit.len() > 7 { &current_commit[..7] } else { current_commit };
	if commit_lines.is_empty() {
		return Ok(UpdateCheckOutcome::UpToDate(format!("dev-{short_current}")));
	}
	let latest_hash = commit_lines.first().and_then(|l| l.split_whitespace().nth(1)).unwrap_or("latest");
	if short_current == latest_hash {
		return Ok(UpdateCheckOutcome::UpToDate(format!("dev-{latest_hash}")));
	}
	let position = commit_lines.iter().position(|l| l.contains(short_current));
	match position {
		Some(0) => Ok(UpdateCheckOutcome::UpToDate(format!("dev-{latest_hash}"))),
		Some(pos) => {
			let new_notes = commit_lines[..pos].join("\n");
			let (download_url, signature_url) = require_asset_pair(config, is_installer, &release)?;
			Ok(UpdateCheckOutcome::UpdateAvailable(UpdateAvailableResult {
				latest_version: format!("dev-{latest_hash}"),
				download_url,
				signature_url,
				release_notes: new_notes,
			}))
		}
		None => {
			// Commit not found in recent history; assume it's old and offer full update.
			let (download_url, signature_url) = require_asset_pair(config, is_installer, &release)?;
			Ok(UpdateCheckOutcome::UpdateAvailable(UpdateAvailableResult {
				latest_version: format!("dev-{latest_hash}"),
				download_url,
				signature_url,
				release_notes: raw_notes,
			}))
		}
	}
}

fn fetch_latest_release(config: &UpdaterConfig) -> Result<GithubRelease, UpdateError> {
	let url = format!("https://api.github.com/repos/{}/releases/latest", config.github_repo);
	fetch_release_url(config, &url)
}

fn fetch_release_by_tag(config: &UpdaterConfig, tag: &str) -> Result<GithubRelease, UpdateError> {
	let url = format!("https://api.github.com/repos/{}/releases/tags/{tag}", config.github_repo);
	fetch_release_url(config, &url)
}

fn fetch_release_url(config: &UpdaterConfig, url: &str) -> Result<GithubRelease, UpdateError> {
	let http = build_agent(Some(Duration::from_secs(15)));
	let mut resp = http
		.get(url)
		.header("User-Agent", &config.user_agent)
		.header("Accept", "application/vnd.github+json")
		.call()
		.map_err(|e| map_http_err(&e))?;
	resp.body_mut()
		.read_json::<GithubRelease>()
		.map_err(|e| UpdateError::InvalidResponse(format!("Failed to parse release JSON: {e}")))
}

fn build_agent(global_timeout: Option<Duration>) -> Agent {
	let mut builder = Config::builder().timeout_connect(Some(Duration::from_secs(30)));
	if let Some(t) = global_timeout {
		builder = builder.timeout_global(Some(t));
	}
	Agent::new_with_config(builder.build())
}

fn map_http_err(err: &ureq::Error) -> UpdateError {
	match err {
		ureq::Error::StatusCode(code) => UpdateError::HttpError(i32::from(*code)),
		_ => UpdateError::NetworkError(err.to_string()),
	}
}

fn require_asset_pair(
	config: &UpdaterConfig,
	is_installer: bool,
	release: &GithubRelease,
) -> Result<(String, String), UpdateError> {
	match release.assets.as_ref() {
		Some(list) if !list.is_empty() => pick_asset_pair(&config.app_name, is_installer, list).ok_or_else(|| {
			UpdateError::NoDownload(
				"Update is available but no matching download asset or signature was found.".to_string(),
			)
		}),
		_ => Err(UpdateError::NoDownload("Latest release does not include downloadable assets.".to_string())),
	}
}

fn pick_asset_pair(app_name: &str, is_installer: bool, assets: &[ReleaseAsset]) -> Option<(String, String)> {
	let base = if is_installer { format!("{app_name}_setup.exe") } else { format!("{app_name}.zip") };
	let sig_name = format!("{base}.minisig");
	let mut download_url = None;
	let mut sig_url = None;
	for asset in assets {
		if asset.name.eq_ignore_ascii_case(&base) {
			download_url = Some(asset.browser_download_url.clone());
		} else if asset.name.eq_ignore_ascii_case(&sig_name) {
			sig_url = Some(asset.browser_download_url.clone());
		}
	}
	match (download_url, sig_url) {
		(Some(d), Some(s)) => Some((d, s)),
		_ => None,
	}
}

fn parse_semver(value: &str) -> Option<(u64, u64, u64)> {
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

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn semver_accepts_prefixes_and_prerelease_suffix() {
		assert_eq!(parse_semver("v1.2.3"), Some((1, 2, 3)));
		assert_eq!(parse_semver("V4.5.6"), Some((4, 5, 6)));
		assert_eq!(parse_semver("1.2.3-beta.1"), Some((1, 2, 3)));
	}

	#[test]
	fn semver_defaults_missing_parts() {
		assert_eq!(parse_semver("1"), Some((1, 0, 0)));
		assert_eq!(parse_semver("1.2"), Some((1, 2, 0)));
	}

	#[test]
	fn semver_rejects_empty_or_invalid() {
		assert_eq!(parse_semver(""), None);
		assert_eq!(parse_semver("not-a-version"), None);
		assert_eq!(parse_semver("v"), None);
		assert_eq!(parse_semver(".2.3"), None);
	}

	#[test]
	fn semver_trims_whitespace() {
		assert_eq!(parse_semver("  v2.3.4  "), Some((2, 3, 4)));
	}

	#[test]
	fn semver_ignores_extra_segments() {
		assert_eq!(parse_semver("1.2.3.99"), Some((1, 2, 3)));
	}

	fn make_assets(entries: &[(&str, &str)]) -> Vec<ReleaseAsset> {
		entries
			.iter()
			.map(|(name, url)| ReleaseAsset { name: name.to_string(), browser_download_url: url.to_string() })
			.collect()
	}

	#[test]
	fn pick_asset_pair_installer() {
		let assets = make_assets(&[
			("myapp.zip", "https://example.com/myapp.zip"),
			("myapp.zip.minisig", "https://example.com/myapp.zip.minisig"),
			("myapp_setup.exe", "https://example.com/myapp_setup.exe"),
			("myapp_setup.exe.minisig", "https://example.com/myapp_setup.exe.minisig"),
		]);
		let (url, sig) = pick_asset_pair("myapp", true, &assets).unwrap();
		assert_eq!(url, "https://example.com/myapp_setup.exe");
		assert_eq!(sig, "https://example.com/myapp_setup.exe.minisig");
	}

	#[test]
	fn pick_asset_pair_zip() {
		let assets = make_assets(&[
			("myapp.zip", "https://example.com/myapp.zip"),
			("myapp.zip.minisig", "https://example.com/myapp.zip.minisig"),
			("myapp_setup.exe", "https://example.com/myapp_setup.exe"),
			("myapp_setup.exe.minisig", "https://example.com/myapp_setup.exe.minisig"),
		]);
		let (url, sig) = pick_asset_pair("myapp", false, &assets).unwrap();
		assert_eq!(url, "https://example.com/myapp.zip");
		assert_eq!(sig, "https://example.com/myapp.zip.minisig");
	}

	#[test]
	fn pick_asset_pair_case_insensitive() {
		let assets = make_assets(&[
			("MYAPP.ZIP", "https://example.com/MYAPP.ZIP"),
			("MYAPP.ZIP.MINISIG", "https://example.com/MYAPP.ZIP.MINISIG"),
		]);
		let (url, sig) = pick_asset_pair("myapp", false, &assets).unwrap();
		assert_eq!(url, "https://example.com/MYAPP.ZIP");
		assert_eq!(sig, "https://example.com/MYAPP.ZIP.MINISIG");
	}

	#[test]
	fn pick_asset_pair_returns_none_when_missing() {
		let assets = make_assets(&[("notes.txt", "https://example.com/notes.txt")]);
		assert!(pick_asset_pair("myapp", true, &assets).is_none());
		assert!(pick_asset_pair("myapp", false, &assets).is_none());
	}

	#[test]
	fn pick_asset_pair_returns_none_when_sig_missing() {
		let assets = make_assets(&[("myapp.zip", "https://example.com/myapp.zip")]);
		assert!(pick_asset_pair("myapp", false, &assets).is_none());
	}
}
