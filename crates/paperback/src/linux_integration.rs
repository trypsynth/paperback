//! One-time "set up file associations" flow shown the first time Paperback runs from an
//! AppImage. AppImages have no install step to hang an installer-style Tasks page off of, so
//! this mimics one at first launch instead: the same checkbox list the Windows installer
//! generates from `paperback_formats::ALL` (see `build/installer.rs`), applied via `xdg-mime`
//! and a `.desktop` file in the user's local applications directory, since Linux associates by
//! MIME type rather than by extension. There's also one extra checkbox with no Windows-installer
//! equivalent, for symlinking the `pb` CLI onto `PATH`.
//!
//! A no-op for a regular (non-AppImage) Linux install: there's no single executable path to
//! point a `.desktop` file's `Exec=` line at, and anyone packaging Paperback as a `.deb`/`.rpm`
//! is expected to ship its own desktop file and associations as part of that packaging.

use std::{
	env, fs,
	io::{self, Write},
	path::{Path, PathBuf},
	process,
	process::Command,
	sync::Mutex,
};

use paperback_core::config::ConfigManager;
use paperback_formats::FormatMeta;
use patois::t;
use wxdragon::prelude::Frame;

use crate::ui::{AssociationChoice, ChoiceAction, show_linux_setup_dialog};

/// Config key marking that the user has confirmed the dialog, so it never shows more than once.
/// Left unset on Cancel/Escape, so declining shows it again next launch rather than silently
/// skipping setup for good.
const SETUP_DONE_KEY: &str = "linux_file_associations_setup_done";

const DESKTOP_FILE_ID: &str = "paperback.desktop";

/// `.opf` has no freedesktop.org-registered media type, so Paperback supplies its own via a
/// small supplementary shared-mime-info package installed alongside the desktop file. Extra
/// `<mime-type>` packages like this are additive by spec, so this can't conflict with anything
/// else on the system that happens to also claim `application/x-daisy-opf`.
const DAISY_OPF_MIME_INFO: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<mime-info xmlns="http://www.freedesktop.org/standards/shared-mime-info">
	<mime-type type="application/x-daisy-opf">
		<comment>DAISY Open Packaging Format</comment>
		<glob pattern="*.opf"/>
	</mime-type>
</mime-info>
"#;

/// Shows the setup dialog if Paperback is running from an AppImage and the user hasn't been
/// through it before. Best-effort throughout: a missing `xdg-mime`/`update-desktop-database`,
/// or a read-only home directory, just means the OS-level association doesn't stick. It
/// doesn't stop the app from starting.
///
/// Cancel/Escape quits Paperback outright rather than dropping the user into a main window
/// they never asked to open, and leaves `SETUP_DONE_KEY` unset so the dialog is offered again
/// next launch.
pub fn maybe_run_first_run_setup(parent: &Frame, config: &Mutex<ConfigManager>) {
	let Ok(appimage_path) = env::var("APPIMAGE") else { return };
	if config.lock().unwrap().get_app_bool(SETUP_DONE_KEY, false) {
		return;
	}
	let choices = association_choices();
	let Some(selections) = show_linux_setup_dialog(parent, &choices) else {
		process::exit(0);
	};
	if let Err(err) = apply_selections(&appimage_path, &choices, &selections) {
		tracing::warn!(error = %err, "failed to set up Linux file associations");
	}
	config.lock().unwrap().set_app_bool(SETUP_DONE_KEY, true);
}

/// Builds the checkbox list from `paperback_formats::ALL`, plus a standalone ZIP entry shared
/// by DAISY and Word-in-zip. This mirrors which boxes `format_tasks_block()` in `build/installer.rs`
/// offers and pre-checks, whose per-format loop also skips `zip` in favor of one shared,
/// unchecked task. Unlike Windows, this dialog doesn't distinguish "Open with" from "default
/// handler" (`format.installer.default_handler` is ignored here): it's an explicit, one-time
/// action the user walks through on purpose rather than a task list ticked mid-install, and its
/// own copy says "should open in Paperback", so every checked box becomes the default handler
/// for its MIME types.
fn association_choices() -> Vec<AssociationChoice> {
	let mut choices = vec![AssociationChoice {
		// TRANSLATORS: Checkbox label in the Linux setup dialog, offering to make the `pb`
		// command-line tool (Paperback's headless document-to-text/HTML converter) runnable
		// from a shell. Mirrors the Windows installer's "Add Paperback directory to PATH" task.
		label: t("Add the pb command-line tool to PATH"),
		action: ChoiceAction::AddPbToPath,
		default_checked: false,
	}];
	choices.extend(paperback_formats::ALL.iter().map(|format| AssociationChoice {
		label: format_label(format),
		action: ChoiceAction::Associate(format.mime_types),
		default_checked: format.installer.default_checked,
	}));
	choices.push(AssociationChoice {
		// TRANSLATORS: Checkbox label in the Linux file-association setup dialog. Shared by
		// multiple parsers (DAISY, Word-in-zip), so it's kept as its own opt-in entry rather
		// than tied to one format, same as the Windows installer's "assoc_zip" task.
		label: t("ZIP Archives (.zip)"),
		action: ChoiceAction::Associate(&["application/zip"]),
		default_checked: false,
	});
	choices
}

fn format_label(format: &FormatMeta) -> String {
	let extensions = format
		.extensions
		.iter()
		.filter(|ext| **ext != "zip")
		.map(|ext| format!(".{ext}"))
		.collect::<Vec<_>>()
		.join(", ");
	format!("{} ({extensions})", format.name)
}

fn apply_selections(appimage_path: &str, choices: &[AssociationChoice], selections: &[bool]) -> io::Result<()> {
	let apps_dir = local_share_dir("applications")?;
	fs::create_dir_all(&apps_dir)?;
	let icon_name = install_icon().unwrap_or_else(|_| "paperback".to_string());
	let mut mime_types: Vec<&str> = Vec::new();
	let mut add_pb_to_path = false;
	for (choice, &checked) in choices.iter().zip(selections) {
		if !checked {
			continue;
		}
		match choice.action {
			ChoiceAction::Associate(mimes) => mime_types.extend(mimes.iter().copied()),
			ChoiceAction::AddPbToPath => add_pb_to_path = true,
		}
	}
	write_desktop_file(&apps_dir, appimage_path, &icon_name, &mime_types)?;
	install_daisy_opf_mime_info();
	run_best_effort("update-desktop-database", &[&apps_dir.to_string_lossy()]);
	for mime in &mime_types {
		run_best_effort("xdg-mime", &["default", DESKTOP_FILE_ID, mime]);
	}
	if add_pb_to_path {
		add_pb_to_local_bin(appimage_path);
	}
	Ok(())
}

/// Symlinks `pb` into `~/.local/bin`, the conventional per-user bin directory most desktop
/// distros already put on `PATH` (e.g. via `~/.profile`). This is the closest Linux equivalent of
/// the Windows installer's "Add Paperback directory to PATH" task, without editing shell startup
/// files or `PATH` itself by hand.
///
/// Points at the AppImage file (`$APPIMAGE`, stable across runs) rather than the `usr/bin/pb`
/// inside it, which lives on a fresh read-only mount that disappears once the app exits. AppRun
/// (see `xtask::release::build_appimage`) dispatches on `$ARGV0`'s basename, so invoking this
/// symlink as `pb` re-execs the AppImage straight into `usr/bin/pb` instead of the GUI.
fn add_pb_to_local_bin(appimage_path: &str) {
	let Ok(bin_dir) = local_bin_dir() else { return };
	if fs::create_dir_all(&bin_dir).is_err() {
		return;
	}
	let link_path = bin_dir.join("pb");
	let _ = fs::remove_file(&link_path);
	if let Err(err) = std::os::unix::fs::symlink(appimage_path, &link_path) {
		tracing::debug!(error = %err, "failed to symlink pb into ~/.local/bin");
	}
}

fn local_bin_dir() -> io::Result<PathBuf> {
	let home = env::var_os("HOME").ok_or_else(|| io::Error::other("HOME is not set"))?;
	Ok(PathBuf::from(home).join(".local/bin"))
}

fn write_desktop_file(apps_dir: &Path, appimage_path: &str, icon_name: &str, mime_types: &[&str]) -> io::Result<()> {
	let mime_line = if mime_types.is_empty() { String::new() } else { format!("MimeType={};\n", mime_types.join(";")) };
	let contents = format!(
		"[Desktop Entry]\n\
		 Type=Application\n\
		 Name=Paperback\n\
		 GenericName=Document Reader\n\
		 Comment=Lightweight, fast, and accessible ebook and document reader\n\
		 Exec=\"{appimage_path}\" %f\n\
		 Icon={icon_name}\n\
		 Terminal=false\n\
		 Categories=Office;Viewer;GTK;\n\
		 StartupNotify=true\n\
		 {mime_line}"
	);
	fs::File::create(apps_dir.join(DESKTOP_FILE_ID))?.write_all(contents.as_bytes())
}

/// Copies the icon out of the mounted AppImage (`$APPDIR`, set by the AppImage runtime
/// alongside `$APPIMAGE` while it's running) into the user's icon theme directory, so the
/// desktop entry has something to show once the AppImage itself is no longer mounted.
fn install_icon() -> io::Result<String> {
	let icons_dir = local_share_dir("icons/hicolor/512x512/apps")?;
	fs::create_dir_all(&icons_dir)?;
	if let Ok(appdir) = env::var("APPDIR") {
		let src = PathBuf::from(appdir).join("paperback.png");
		if src.is_file() {
			fs::copy(&src, icons_dir.join("paperback.png"))?;
		}
	}
	Ok("paperback".to_string())
}

fn install_daisy_opf_mime_info() {
	let Ok(mime_packages_dir) = local_share_dir("mime/packages") else { return };
	if fs::create_dir_all(&mime_packages_dir).is_err() {
		return;
	}
	if fs::write(mime_packages_dir.join("paperback-daisy.xml"), DAISY_OPF_MIME_INFO).is_err() {
		return;
	}
	if let Ok(mime_home) = local_share_dir("mime") {
		run_best_effort("update-mime-database", &[&mime_home.to_string_lossy()]);
	}
}

fn local_share_dir(sub: &str) -> io::Result<PathBuf> {
	if let Some(data_home) = env::var_os("XDG_DATA_HOME") {
		return Ok(PathBuf::from(data_home).join(sub));
	}
	let home = env::var_os("HOME").ok_or_else(|| io::Error::other("HOME is not set"))?;
	Ok(PathBuf::from(home).join(".local/share").join(sub))
}

fn run_best_effort(program: &str, args: &[&str]) {
	if let Err(err) = Command::new(program).args(args).status() {
		tracing::debug!(program, error = %err, "optional Linux desktop-integration command failed to run");
	}
}
