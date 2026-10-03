//! The auto-updater flow: checking GitHub for a newer release, and the Windows-only foreground
//! hand-off that lets the relaunched instance take the foreground when it replaces this process.

use std::{env, sync::Arc};

use paperback_core::version;
use ship_shape::{InstallKind, UpdateChannel as ShipChannel, UpdaterConfig, ui::CheckTrigger};
use wxdragon::prelude::*;

use crate::config_ext::UpdateChannel;

const PAPERBACK_GITHUB_REPO: &str = "trypsynth/paperback";
const PAPERBACK_MINISIGN_KEY: &str = "RWQasnbWXwK2dhno9ThUm8HONEIo85iiDBZvw3jlNs574QJHEkoRiGX7";

// Matches the `-x64`/`-arm64` suffixes the release workflow appends to Windows and Linux asset
// names (see .github/workflows/build.yml) so the updater requests the build for this machine's
// architecture instead of a name that no longer exists in the release. macOS publishes a single
// unsuffixed asset.
#[cfg(all(any(target_os = "windows", target_os = "linux"), target_arch = "x86_64"))]
const UPDATE_ASSET_SUFFIX: &str = "-x64";
#[cfg(all(any(target_os = "windows", target_os = "linux"), target_arch = "aarch64"))]
const UPDATE_ASSET_SUFFIX: &str = "-arm64";
#[cfg(not(any(target_os = "windows", target_os = "linux")))]
const UPDATE_ASSET_SUFFIX: &str = "";

/// Keep the right to take the foreground re-issued for as long as an update is in flight, so the
/// process that replaces this one can spend it.
///
/// The updater relaunches the app from a hidden PowerShell that outlives this process
/// (`ship-shape`'s `ui/install/windows.rs`), so the instance coming up is a grandchild of a
/// windowless background process and satisfies none of the conditions Windows requires before it
/// will let a window take the foreground - in particular it did not take the last input and was not
/// started by the foreground process. It does not call anything to claim the foreground either: the
/// activation it gets is `wxTopLevelWindowMSW::Show`'s `SW_SHOW`, which is subject to the same
/// rules. This process *is* the foreground process - the update started with a click on its own
/// dialog - and `AllowSetForegroundWindow` passes that right on. `ASFW_ANY` rather than a pid,
/// because the right is spent by the grandchild rather than by the PowerShell this process starts.
/// It is the same hand-off `send_ipc_command` does to activate an already-running instance.
///
/// One call is not enough, and where it goes matters more than that it happens. The grant is
/// revoked by the next user input, and the download in between runs for seconds to minutes with the
/// reader free to click anything, so a grant issued before it has been spent long before the
/// replacement exists. Being live at the moment that replacement shows its window is the whole
/// requirement, and re-issuing it is the only way to be sure of that. This is where a fix that
/// grants once, however carefully placed, stops working.
///
/// The beat is scoped to the update flow rather than the session. It grants nothing until one of
/// the updater's own dialogs has been on screen, so an app that is up to date - the silent startup
/// check, which is the common case - never hands the foreground out at all, and it stops once those
/// dialogs have been gone for `QUIET_BEATS` seconds, so declining an update does not leave a
/// foreground permission open afterwards.
#[cfg(target_os = "windows")]
fn beat_foreground_grant() {
	use std::time::{Duration, Instant};

	use windows::Win32::UI::WindowsAndMessaging::AllowSetForegroundWindow;

	const BEAT: Duration = Duration::from_secs(1);
	/// Long enough to bridge the gap between the changelog dialog closing and the progress dialog
	/// opening, which is the only moment in a real update where no dialog of ours is on screen.
	const QUIET_BEATS: u32 = 3;
	/// A check that finds nothing never puts a dialog up, so there is nothing to wait for; this only
	/// bounds the thread's life in that case. It has to clear the update check's own network round
	/// trip with room to spare, because stopping before the changelog appears would mean no grant at
	/// all - the one way this can fail that would look like the mechanism not working.
	const START_TIMEOUT: Duration = Duration::from_mins(3);

	std::thread::spawn(move || {
		let started = Instant::now();
		let mut in_update = false;
		let mut quiet = 0u32;
		loop {
			std::thread::sleep(BEAT);
			if crate::ui::own_dialog_is_up() || crate::ui::frame_is_disabled() {
				in_update = true;
				quiet = 0;
			} else if in_update {
				quiet += 1;
				if quiet >= QUIET_BEATS {
					tracing::info!("update dialogs are gone, stopped re-issuing the foreground hand-off");
					return;
				}
			} else if started.elapsed() >= START_TIMEOUT {
				tracing::info!("no update dialog appeared, stopped watching to re-issue the foreground hand-off");
				return;
			}
			if in_update {
				let granted = unsafe { AllowSetForegroundWindow(u32::MAX) }.is_ok();
				tracing::info!(granted, "re-issued foreground hand-off to the relaunched instance");
			}
		}
	});
}

#[cfg(not(target_os = "windows"))]
fn beat_foreground_grant() {}

pub fn run_update_check(parent: &Frame, silent: bool, channel: UpdateChannel) {
	tracing::info!(channel = %channel, silent, "checking for updates");
	let install_kind = if is_installer_distribution() { InstallKind::Installer } else { InstallKind::Portable };
	let config = Arc::new(
		UpdaterConfig::new(
			PAPERBACK_GITHUB_REPO,
			"paperback",
			"Paperback",
			PAPERBACK_MINISIGN_KEY,
			env!("CARGO_PKG_VERSION"),
		)
		.with_commit(version::COMMIT_HASH)
		.with_install_kind(install_kind)
		.with_user_agent(version::user_agent())
		.with_asset_suffix(UPDATE_ASSET_SUFFIX),
	);
	beat_foreground_grant();
	let ship_channel = match channel {
		UpdateChannel::Stable => ShipChannel::Stable,
		UpdateChannel::Dev => ShipChannel::Dev,
	};
	let trigger = if silent { CheckTrigger::Automatic } else { CheckTrigger::Manual };
	ship_shape::ui::run_update_check(config, parent, ship_channel, trigger);
}

pub fn is_installer_distribution() -> bool {
	// An AppImage is what the updater treats as the installer on Linux: it replaces the
	// `.AppImage` file itself, where the portable tar.gz would be extracted next to an executable
	// that lives in the AppImage's read-only mount. `$APPIMAGE` is the same check `config_dir`
	// and `linux_integration.rs` use.
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
