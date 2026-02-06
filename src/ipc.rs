use std::{env, path::{Path, PathBuf}};

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
	let payload = payload.trim_end_matches('\0');
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
