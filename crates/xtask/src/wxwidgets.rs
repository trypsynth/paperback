//! Fetching the wxWidgets commit Paperback builds against, which `.cargo/config.toml` names in `WXWIDGETS_DIR`.

use std::{
	error::Error,
	fs,
	path::{Path, PathBuf},
	process::Command,
};

use crate::workspace::project_root;

const REPO: &str = "https://github.com/wxWidgets/wxWidgets.git";

fn pinned(root: &Path) -> Result<(PathBuf, String), Box<dyn Error>> {
	let config = fs::read_to_string(root.join(".cargo/config.toml"))?;
	let line = config.lines().find(|l| l.trim_start().starts_with("WXWIDGETS_DIR")).ok_or("no WXWIDGETS_DIR")?;
	let value = line.split('"').nth(1).ok_or("WXWIDGETS_DIR has no quoted value")?;
	let commit = value.rsplit('-').next().filter(|c| c.len() == 40 && c.bytes().all(|b| b.is_ascii_hexdigit()));
	let commit = commit.ok_or("WXWIDGETS_DIR must end in -<the 40-character commit>")?;
	Ok((root.join(value), commit.to_string()))
}

fn git(dir: &Path, args: &[&str]) -> Result<(), Box<dyn Error>> {
	if !Command::new("git").current_dir(dir).args(args).status()?.success() {
		return Err(format!("git {} failed in {}", args.join(" "), dir.display()).into());
	}
	Ok(())
}

fn checked_out_commit(dir: &Path) -> Option<String> {
	let output = Command::new("git").current_dir(dir).args(["rev-parse", "HEAD"]).output().ok()?;
	output.status.success().then(|| String::from_utf8_lossy(&output.stdout).trim().to_string())
}

pub fn ensure() -> Result<PathBuf, Box<dyn Error>> {
	let root = project_root();
	let (dir, commit) = pinned(&root)?;
	// Checked for its own .git first: without one, rev-parse would answer for Paperback's repository. A plain `cargo build` run before this also leaves wxdragon-sys's release download in the folder, which must not pass for the pinned commit.
	if dir.join(".git").exists() && checked_out_commit(&dir).as_deref() == Some(commit.as_str()) {
		return Ok(dir);
	}
	println!("Fetching wxWidgets {commit}...");
	let _ = fs::remove_dir_all(&dir);
	fs::create_dir_all(&dir)?;
	git(&dir, &["init", "-q"])?;
	git(&dir, &["remote", "add", "origin", REPO])?;
	git(&dir, &["fetch", "-q", "--depth", "1", "origin", &commit])?;
	git(&dir, &["checkout", "-q", "FETCH_HEAD"])?;
	git(&dir, &["submodule", "update", "-q", "--init", "--recursive", "--depth", "1"])?;
	// Each checkout is several hundred megabytes, and only the pinned one is ever used again.
	if let Some(parent) = dir.parent() {
		for entry in fs::read_dir(parent)?.flatten() {
			let name = entry.file_name().to_string_lossy().into_owned();
			if name.starts_with("wxWidgets-") && entry.path() != dir {
				let _ = fs::remove_dir_all(entry.path());
			}
		}
	}
	Ok(dir)
}
