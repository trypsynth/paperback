use std::{
	env,
	path::{Path, PathBuf},
};

use paperback_core::parser::is_remote_url;

#[cfg(any(target_os = "linux", target_os = "windows", test))]
pub const IPC_COMMAND_ACTIVATE: &str = "ACTIVATE";
#[cfg(any(target_os = "linux", target_os = "windows", test))]
pub const IPC_COMMAND_TOGGLE_VISIBILITY: &str = "TOGGLE";
#[cfg(any(target_os = "linux", target_os = "windows"))]
pub const SINGLE_INSTANCE_NAME: &str = "paperback_running";

#[cfg(any(target_os = "linux", target_os = "windows", test))]
#[derive(Debug, Clone)]
pub enum IpcCommand {
	Activate,
	#[cfg(any(target_os = "linux", target_os = "windows", test))]
	ToggleVisibility,
	OpenFile(PathBuf),
	OpenLink(String),
}

/// What a command-line argument asks Paperback to open.
#[derive(Debug, PartialEq, Eq)]
pub enum Target {
	/// A file, by its full path.
	File(PathBuf),
	/// A web link, as given.
	Link(String),
}

pub fn target_for_argument(argument: &str) -> Target {
	if is_remote_url(argument) {
		Target::Link(argument.to_string())
	} else {
		Target::File(normalize_cli_path(Path::new(argument)))
	}
}

#[cfg(any(target_os = "linux", target_os = "windows", test))]
pub fn decode_execute_payload(data: &[u8]) -> Option<IpcCommand> {
	if data.is_empty() {
		return None;
	}
	let payload = String::from_utf8_lossy(data);
	let payload = payload.replace('\0', "");
	let payload = payload.trim();
	if payload.is_empty() {
		return None;
	}
	if payload == IPC_COMMAND_ACTIVATE {
		return Some(IpcCommand::Activate);
	}
	if payload == IPC_COMMAND_TOGGLE_VISIBILITY {
		return Some(IpcCommand::ToggleVisibility);
	}
	if is_remote_url(payload) {
		return Some(IpcCommand::OpenLink(payload.to_string()));
	}
	Some(IpcCommand::OpenFile(PathBuf::from(payload)))
}

pub fn normalize_cli_path(path: &Path) -> PathBuf {
	if let Ok(normalized) = dunce::canonicalize(path) {
		return normalized;
	}
	if path.is_absolute() {
		return path.to_path_buf();
	}
	env::current_dir().map_or_else(|_| path.to_path_buf(), |cwd| cwd.join(path))
}

/// Named pipe path scoped to the current user, preventing cross-user connections.
/// The default pipe security descriptor further enforces same-user access.
#[cfg(windows)]
pub fn named_pipe_path() -> String {
	let user = env::var("USERNAME").unwrap_or_else(|_| "user".to_string());
	format!(r"\\.\pipe\paperback_{user}")
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn decode_execute_payload_handles_empty_and_nulls() {
		assert!(decode_execute_payload(b"").is_none());
		assert!(decode_execute_payload(b"\0\0").is_none());
		assert!(decode_execute_payload(b" \0").is_none());
	}

	#[test]
	fn decode_execute_payload_handles_activate() {
		let cmd = decode_execute_payload(b"ACTIVATE\0").expect("expected command");
		match cmd {
			IpcCommand::Activate => {}
			_ => panic!("expected Activate"),
		}
	}

	#[test]
	fn decode_execute_payload_handles_open_file() {
		let cmd = decode_execute_payload(b"C:\\test\\file.txt\0").expect("expected command");
		match cmd {
			IpcCommand::OpenFile(path) => {
				assert_eq!(path, PathBuf::from("C:\\test\\file.txt"));
			}
			_ => panic!("expected OpenFile"),
		}
	}

	#[test]
	fn normalize_cli_path_handles_absolute_and_relative() {
		#[cfg(windows)]
		let abs = Path::new("C:\\nonexistent_abs_path");
		#[cfg(not(windows))]
		let abs = Path::new("/nonexistent_abs_path");
		assert_eq!(normalize_cli_path(abs), abs.to_path_buf());
		let rel = Path::new("nonexistent_rel_path");
		let expected = env::current_dir().unwrap().join(rel);
		assert_eq!(normalize_cli_path(rel), expected);
	}

	#[test]
	fn decode_execute_payload_trims_whitespace_for_activate() {
		let cmd = decode_execute_payload(b"  ACTIVATE  ").expect("expected command");
		match cmd {
			IpcCommand::Activate => {}
			_ => panic!("expected Activate"),
		}
	}

	#[test]
	fn decode_execute_payload_allows_spaced_open_file_paths() {
		let cmd = decode_execute_payload(b"  C:\\My Docs\\book.txt  ").expect("expected command");
		match cmd {
			IpcCommand::OpenFile(path) => assert_eq!(path, PathBuf::from("C:\\My Docs\\book.txt")),
			_ => panic!("expected OpenFile"),
		}
	}

	#[test]
	fn decode_execute_payload_handles_non_utf8_bytes_lossy() {
		let cmd = decode_execute_payload(&[0xFF, 0xFE, b'a']).expect("expected command");
		match cmd {
			IpcCommand::OpenFile(path) => assert!(path.to_string_lossy().contains('a')),
			_ => panic!("expected OpenFile"),
		}
	}

	#[test]
	fn decode_execute_payload_strips_embedded_nulls() {
		let cmd = decode_execute_payload(b"C:\\Books\\novel.epub\0\0").expect("expected command");
		match cmd {
			IpcCommand::OpenFile(path) => assert_eq!(path, PathBuf::from("C:\\Books\\novel.epub")),
			_ => panic!("expected OpenFile"),
		}
	}

	#[test]
	fn decode_execute_payload_hands_a_link_on_as_written() {
		match decode_execute_payload(b"https://example.org/Book%20One.epub\0").expect("expected command") {
			IpcCommand::OpenLink(link) => assert_eq!(link, "https://example.org/Book%20One.epub"),
			other => panic!("expected OpenLink, got {other:?}"),
		}
	}

	#[test]
	fn a_link_argument_is_opened_as_written() {
		assert_eq!(
			target_for_argument("https://example.org/Book%20One.epub"),
			Target::Link("https://example.org/Book%20One.epub".to_string())
		);
	}

	#[test]
	fn a_file_argument_is_opened_by_its_full_path() {
		let expected = env::current_dir().unwrap().join("nonexistent_book.epub");
		assert_eq!(target_for_argument("nonexistent_book.epub"), Target::File(expected));
	}

	#[test]
	fn normalize_cli_path_canonicalizes_existing_path() {
		let cwd = env::current_dir().expect("cwd");
		let cwd = dunce::canonicalize(cwd).expect("canonical cwd");
		let normalized = normalize_cli_path(Path::new("."));
		assert_eq!(normalized, cwd);
	}

	#[test]
	fn normalize_cli_path_preserves_existing_absolute_files() {
		let abs = env::current_exe().expect("current exe");
		let abs = dunce::canonicalize(abs).expect("canonical exe");
		let normalized = normalize_cli_path(&abs);
		assert_eq!(normalized, abs);
	}
}
