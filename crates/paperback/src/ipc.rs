use std::{
	env,
	path::{Path, PathBuf},
};

use paperback_core::parser::is_remote_url;

/// The name a second copy finds the running one by. Not on macOS, which keeps a bundled app to one copy by itself and sends it the files the user opens.
#[cfg(any(target_os = "linux", target_os = "windows"))]
pub const APP_NAME: &str = "paperback";

#[cfg(any(target_os = "linux", target_os = "windows", test))]
#[derive(Debug, Clone)]
pub enum IpcCommand {
	Activate,
	/// From the global hotkey, which only Windows has.
	#[cfg(target_os = "windows")]
	ToggleVisibility,
	Open(Target),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Target {
	File(PathBuf),
	Link(String),
}

pub fn target_for_argument(argument: &str) -> Target {
	if is_remote_url(argument) {
		Target::Link(argument.to_string())
	} else {
		Target::File(normalize_cli_path(Path::new(argument)))
	}
}

/// What a second copy sends the running one: each file by its full path, since the running copy may be in another folder, and each link as written. An empty message brings the window to the front.
#[cfg(any(target_os = "linux", target_os = "windows", test))]
pub fn message_from_args(args: impl IntoIterator<Item = String>) -> Vec<String> {
	args.into_iter()
		.map(|argument| match target_for_argument(&argument) {
			Target::File(path) => path.to_string_lossy().into_owned(),
			Target::Link(link) => link,
		})
		.collect()
}

/// What the running copy does with a message from a second copy.
#[cfg(any(target_os = "linux", target_os = "windows", test))]
pub fn commands_from_message(message: &[String]) -> Vec<IpcCommand> {
	let commands: Vec<IpcCommand> = message
		.iter()
		.map(|part| part.trim())
		.filter(|part| !part.is_empty())
		.map(|part| {
			// The second copy already gave a file its full path.
			let target =
				if is_remote_url(part) { Target::Link(part.to_string()) } else { Target::File(PathBuf::from(part)) };
			IpcCommand::Open(target)
		})
		.collect();
	if commands.is_empty() { vec![IpcCommand::Activate] } else { commands }
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

#[cfg(test)]
mod tests {
	use super::*;

	fn opened(command: &IpcCommand) -> &Target {
		match command {
			IpcCommand::Open(target) => target,
			other => panic!("expected something to open, got {other:?}"),
		}
	}

	#[test]
	fn an_empty_message_brings_the_window_to_the_front() {
		assert!(matches!(commands_from_message(&[]).as_slice(), [IpcCommand::Activate]));
		assert!(matches!(commands_from_message(&[" ".into(), String::new()]).as_slice(), [IpcCommand::Activate]));
	}

	#[test]
	fn each_file_and_link_in_a_message_is_opened() {
		let commands = commands_from_message(&[
			"C:\\My Docs\\book.txt".into(),
			"https://example.org/Book%20One.epub".into(),
			"  C:\\Books\\novel.epub  ".into(),
		]);
		assert_eq!(commands.len(), 3);
		assert_eq!(opened(&commands[0]), &Target::File(PathBuf::from("C:\\My Docs\\book.txt")));
		assert_eq!(opened(&commands[1]), &Target::Link("https://example.org/Book%20One.epub".to_string()));
		assert_eq!(opened(&commands[2]), &Target::File(PathBuf::from("C:\\Books\\novel.epub")));
	}

	#[test]
	fn a_second_copy_sends_files_by_their_full_path_and_links_as_written() {
		let expected = env::current_dir().unwrap().join("nonexistent_book.epub");
		assert_eq!(
			message_from_args(["nonexistent_book.epub".to_string(), "https://example.org/Book%20One.epub".to_string()]),
			vec![expected.to_string_lossy().into_owned(), "https://example.org/Book%20One.epub".to_string()]
		);
		assert!(message_from_args(Vec::new()).is_empty());
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
