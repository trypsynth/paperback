//! One-time "set up file associations" flow shown the first time Paperback runs from an
//! AppImage. AppImages have no install step to hang an installer-style Tasks page off of, so
//! this mimics one at first launch instead: the same checkbox list the Windows installer
//! generates from `paperback_formats::ALL` (see `build/installer.rs`), applied via `xdg-mime`
//! and a `.desktop` file in the user's local applications directory, since Linux associates by
//! MIME type rather than by extension.
//!
//! A no-op for a regular (non-AppImage) Linux install: there's no single executable path to
//! point a `.desktop` file's `Exec=` line at, and anyone packaging Paperback as a `.deb`/`.rpm`
//! is expected to ship its own desktop file and associations as part of that packaging.

use std::{
	env, fs,
	io::{self, Write},
	path::{Path, PathBuf},
	process::Command,
	sync::Mutex,
};

use paperback_core::config::ConfigManager;
use paperback_formats::FormatMeta;
use patois::t;
use wxdragon::prelude::Frame;

use crate::ui::{AssociationChoice, show_linux_setup_dialog};

/// Config key marking that the user has already been through the dialog (or dismissed it),
/// so it never shows more than once.
const SETUP_DONE_KEY: &str = "linux_file_associations_setup_done";

const DESKTOP_FILE_ID: &str = "paperback.desktop";

/// `.opf` has no freedesktop.org-registered media type, so Paperback supplies its own via a
/// small supplementary shared-mime-info package installed alongside the desktop file. Extra
/// `<mime-type>` packages like this are additive by spec — this can't conflict with anything
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
/// or a read-only home directory, just means the OS-level association doesn't stick — it
/// doesn't stop the app from starting.
pub fn maybe_run_first_run_setup(parent: &Frame, config: &Mutex<ConfigManager>) {
	let Ok(appimage_path) = env::var("APPIMAGE") else { return };
	if config.lock().unwrap().get_app_bool(SETUP_DONE_KEY, false) {
		return;
	}
	let choices = association_choices();
	if let Some(selections) = show_linux_setup_dialog(parent, &choices)
		&& let Err(err) = apply_selections(&appimage_path, &choices, &selections)
	{
		tracing::warn!(error = %err, "failed to set up Linux file associations");
	}
	config.lock().unwrap().set_app_bool(SETUP_DONE_KEY, true);
}

/// Builds the checkbox list from `paperback_formats::ALL`, plus a standalone ZIP entry shared
/// by DAISY and Word-in-zip — mirrors `format_tasks_block()` in `build/installer.rs`, whose
/// per-format loop also skips `zip` in favor of one shared, unchecked, non-default task.
fn association_choices() -> Vec<AssociationChoice> {
	let mut choices: Vec<AssociationChoice> = paperback_formats::ALL
		.iter()
		.map(|format| AssociationChoice {
			label: format_label(format),
			mime_types: format.mime_types,
			default_checked: format.installer.default_checked,
			default_handler: format.installer.default_handler,
		})
		.collect();
	choices.push(AssociationChoice {
		// TRANSLATORS: Checkbox label in the Linux file-association setup dialog. Shared by
		// multiple parsers (DAISY, Word-in-zip), so it's kept as its own opt-in entry rather
		// than tied to one format, same as the Windows installer's "assoc_zip" task.
		label: t("ZIP Archives (.zip)"),
		mime_types: &["application/zip"],
		default_checked: false,
		default_handler: false,
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
	let mut default_handler_mime_types: Vec<&str> = Vec::new();
	for (choice, &checked) in choices.iter().zip(selections) {
		if !checked {
			continue;
		}
		mime_types.extend(choice.mime_types.iter().copied());
		if choice.default_handler {
			default_handler_mime_types.extend(choice.mime_types.iter().copied());
		}
	}
	write_desktop_file(&apps_dir, appimage_path, &icon_name, &mime_types)?;
	install_daisy_opf_mime_info();
	run_best_effort("update-desktop-database", &[&apps_dir.to_string_lossy()]);
	for mime in &default_handler_mime_types {
		run_best_effort("xdg-mime", &["default", DESKTOP_FILE_ID, mime]);
	}
	Ok(())
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
