use std::path::Path;

/// Fills the taskbar jump list, which stays empty for an app that never tells the shell what it opened.
#[cfg(target_os = "windows")]
pub fn add_recent_document(path: &Path) {
	use std::{ffi::OsStr, iter, os::windows::ffi::OsStrExt as _};

	use windows::Win32::UI::Shell::{SHARD_PATHW, SHAddToRecentDocs};
	// Bound rather than built inline, since the vector has to outlive the call.
	let wide: Vec<u16> = OsStr::new(path).encode_wide().chain(iter::once(0)).collect();
	unsafe {
		SHAddToRecentDocs(SHARD_PATHW.0 as u32, Some(wide.as_ptr().cast()));
	}
}

/// GTK's and macOS's recent-document registries aren't reachable through wx.
#[cfg(not(target_os = "windows"))]
pub const fn add_recent_document(_path: &Path) {}
