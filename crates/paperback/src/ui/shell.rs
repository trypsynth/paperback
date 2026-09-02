//! Handing documents to the desktop shell's own recent-items list.
//!
//! Paperback already keeps a recent list of its own in the File menu, but that one is only
//! reachable once the app is running. A Windows user reaching for the book they had open
//! yesterday goes to the taskbar first: right-clicking a pinned or running app is expected to
//! drop down a jump list of what it last opened, and an app that never tells the shell anything
//! shows an empty one. Registering each opened file here is what fills that in, and the Start
//! menu's recent list along with it, at the cost of one call per open.

use std::path::Path;

/// Adds `path` to the shell's recent documents, for the taskbar jump list and the Start menu.
///
/// Best-effort by nature: the shell silently ignores paths it can't resolve, and the user may
/// have recent-item tracking switched off entirely in Settings, so there is no outcome to
/// report or act on.
#[cfg(target_os = "windows")]
pub fn add_recent_document(path: &Path) {
	use std::{ffi::OsStr, iter, os::windows::ffi::OsStrExt as _};

	use windows::Win32::UI::Shell::{SHARD_PATHW, SHAddToRecentDocs};

	// SHAddToRecentDocs takes a bare pointer whose meaning is set by the flag; SHARD_PATHW says
	// it is a null-terminated wide path. The vector has to outlive the call, so it is bound
	// rather than built inline in the argument.
	let wide: Vec<u16> = OsStr::new(path).encode_wide().chain(iter::once(0)).collect();
	unsafe {
		SHAddToRecentDocs(SHARD_PATHW.0 as u32, Some(wide.as_ptr().cast()));
	}
}

/// No-op away from Windows. GTK and macOS both have their own recent-document registries, but
/// neither is reachable through wx, and neither is what this change was for.
#[cfg(not(target_os = "windows"))]
pub const fn add_recent_document(_path: &Path) {}
