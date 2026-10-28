use std::{env, sync::Arc};

use paperback_core::version;
use ship_shape::{InstallKind, UpdateChannel as ShipChannel, UpdaterConfig, ui::CheckTrigger};
use wxdragon::prelude::*;

use crate::config_ext::UpdateChannel;

const PAPERBACK_GITHUB_REPO: &str = "trypsynth/paperback";
const PAPERBACK_MINISIGN_KEY: &str = "RWQasnbWXwK2dhno9ThUm8HONEIo85iiDBZvw3jlNs574QJHEkoRiGX7";

// Must match the suffixes `.github/workflows/build.yml` gives Windows and Linux assets; macOS publishes one unsuffixed asset.
#[cfg(all(any(target_os = "windows", target_os = "linux"), target_arch = "x86_64"))]
const UPDATE_ASSET_SUFFIX: &str = "-x64";
#[cfg(all(any(target_os = "windows", target_os = "linux"), target_arch = "aarch64"))]
const UPDATE_ASSET_SUFFIX: &str = "-arm64";
#[cfg(not(any(target_os = "windows", target_os = "linux")))]
const UPDATE_ASSET_SUFFIX: &str = "";

pub fn run_update_check(parent: &Frame, silent: bool, channel: UpdateChannel) {
	tracing::info!(channel = %channel, silent, "checking for updates");
	let install_kind = if is_installer_distribution() { InstallKind::Installer } else { InstallKind::Portable };
	let config = UpdaterConfig::new(
		PAPERBACK_GITHUB_REPO,
		"paperback",
		"Paperback",
		PAPERBACK_MINISIGN_KEY,
		env!("CARGO_PKG_VERSION"),
	)
	.with_commit(version::COMMIT_HASH)
	.with_install_kind(install_kind)
	.with_user_agent(version::user_agent())
	.with_asset_suffix(UPDATE_ASSET_SUFFIX);
	#[cfg(target_os = "macos")]
	let config = if let Some(directory) = env::var_os("PAPERBACK_CONFIG_DIR") {
		config.with_macos_relaunch_env("PAPERBACK_CONFIG_DIR", directory)
	} else {
		config
	};
	let config = Arc::new(config);
	let ship_channel = match channel {
		UpdateChannel::Stable => ShipChannel::Stable,
		UpdateChannel::Dev => ShipChannel::Dev,
	};
	let trigger = if silent { CheckTrigger::Automatic } else { CheckTrigger::Manual };
	ship_shape::ui::run_update_check_with_exit_handler(
		config,
		parent,
		ship_channel,
		trigger,
		crate::ui::quit_for_update,
	);
}

pub fn is_installer_distribution() -> bool {
	// The portable tar.gz would be extracted beside an executable in the AppImage's read-only mount, so the AppImage file itself is replaced.
	#[cfg(target_os = "linux")]
	if env::var_os("APPIMAGE").is_some() {
		return true;
	}
	let Ok(exe_path) = env::current_exe() else {
		return false;
	};
	let Some(exe_dir) = exe_path.parent() else {
		return false;
	};
	exe_dir.join("unins000.exe").exists()
}
