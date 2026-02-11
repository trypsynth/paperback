use std::{
	env,
	path::{Path, PathBuf},
};

pub const IPC_SERVICE: &str = "4242";
pub const IPC_TOPIC_OPEN_FILE: &str = "open_file";
pub const IPC_COMMAND_ACTIVATE: &str = "ACTIVATE";
pub const IPC_HOST_LOCALHOST: &str = "localhost";
pub const SINGLE_INSTANCE_NAME: &str = "paperback_running";

#[derive(Debug, Clone)]
pub enum IpcCommand {
	Activate,
	OpenFile(PathBuf),
}

pub fn decode_execute_payload(data: &[u8]) -> Option<IpcCommand> {
	if data.is_empty() {
		return None;
	}
	let payload = String::from_utf8_lossy(data);
	let payload = payload.trim_end_matches('\0').trim();
	if payload.is_empty() {
		return None;
	}
	if payload == IPC_COMMAND_ACTIVATE {
		return Some(IpcCommand::Activate);
	}
	Some(IpcCommand::OpenFile(PathBuf::from(payload)))
}

pub fn normalize_cli_path(path: &Path) -> PathBuf {
	if let Ok(normalized) = path.canonicalize() {
		return normalized;
	}
	if path.is_absolute() {
		return path.to_path_buf();
	}
	env::current_dir().map_or_else(|_| path.to_path_buf(), |cwd| cwd.join(path))
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
		assert_eq!(normalize_cli_path(abs), PathBuf::from(abs));
		let rel = Path::new("nonexistent_rel_path");
		let expected = env::current_dir().unwrap().join(rel);
		assert_eq!(normalize_cli_path(rel), expected);
	}
}
