//! A guard against translations that still carry a menu label's keyboard shortcut.
//!
//! Menu items used to be one string holding the label, its accelerator and its shortcut, as in
//! `Reopen &Last Closed\tCtrl+Shift+T`. They are separate now, and the shortcut dialog's action
//! names are plain text. When the change went through, `msgmerge` matched the new bare msgids
//! against the old combined ones and copied their translations across, so a catalogue ended up
//! saying `Rouvrir & Dernière fermeture    Ctrl+Maj+T` where the source says `Reopen Last
//! Closed`. The entries are fuzzy, which would normally mean unused, but Paperback compiles its
//! catalogues with `--use-fuzzy`, so readers heard the stale shortcut
//! (<https://github.com/trypsynth/paperback/issues/932>, found while reviewing #912).
//!
//! [`stale_shortcuts`] finds them, and the test below runs it over every catalogue in the
//! project, so a merge cannot quietly ship one again.

use std::{fs, path::Path};

/// A translation that has a shortcut its source does not.
#[derive(Debug, PartialEq, Eq)]
pub struct StaleShortcut {
	pub msgid: String,
	pub msgstr: String,
}

/// The separator between a label and its shortcut: a tab, or the run of spaces a machine
/// translator turned one into.
fn shortcut_tail(text: &str) -> Option<&str> {
	let unescaped_tab = text.find("\\t").map(|at| &text[at + 2..]);
	let padded = text.split("   ").nth(1);
	unescaped_tab.or(padded).map(str::trim)
}

/// Every entry in `catalogue` whose translation ends in a shortcut the msgid does not have.
///
/// Both sides are compared the same way, so a msgid that legitimately carries a shortcut (the
/// old-style menu strings still sitting in a stale catalogue) does not count as damaged.
#[must_use]
pub fn stale_shortcuts(catalogue: &str) -> Vec<StaleShortcut> {
	let mut found = Vec::new();
	for entry in catalogue.split("\n\n") {
		// Obsolete entries are commented out with `#~` and are never compiled in.
		if entry.starts_with("#~") {
			continue;
		}
		let (Some(msgid), Some(msgstr)) = (field(entry, "msgid"), field(entry, "msgstr")) else {
			continue;
		};
		if msgid.is_empty() || msgstr.is_empty() {
			continue;
		}
		if shortcut_tail(&msgstr).is_some() && shortcut_tail(&msgid).is_none() {
			found.push(StaleShortcut { msgid, msgstr });
		}
	}
	found
}

/// One `msgid`/`msgstr` of an entry, with its continuation lines joined.
fn field(entry: &str, name: &str) -> Option<String> {
	let mut lines = entry.lines().skip_while(|line| !line.starts_with(&format!("{name} \"")));
	let first = lines.next()?;
	let mut out = quoted(first)?.to_string();
	for line in lines {
		let Some(more) = line.strip_prefix('"').and_then(|rest| rest.strip_suffix('"')) else {
			break;
		};
		out.push_str(more);
	}
	Some(out)
}

fn quoted(line: &str) -> Option<&str> {
	let start = line.find('"')? + 1;
	line.get(start..line.len().checked_sub(1)?)
}

/// Checks every catalogue in `root`'s `po` directory, reporting all damage at once.
pub fn check_catalogues(root: &Path) -> Result<(), String> {
	let mut report = String::new();
	let Ok(entries) = fs::read_dir(root.join("po")) else {
		return Ok(());
	};
	for entry in entries.flatten() {
		let path = entry.path();
		if path.extension().and_then(|e| e.to_str()) != Some("po") {
			continue;
		}
		let Ok(text) = fs::read_to_string(&path) else {
			continue;
		};
		for stale in stale_shortcuts(&text) {
			report.push_str(&format!("{}: {:?} -> {:?}\n", path.display(), stale.msgid, stale.msgstr));
		}
	}
	if report.is_empty() {
		Ok(())
	} else {
		Err(format!("translations carrying a shortcut their source does not:\n{report}"))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn a_copied_menu_shortcut_is_reported() {
		let po = "msgid \"Reopen Last Closed\"\nmsgstr \"Rouvrir & Dernière fermeture    Ctrl+Maj+T\"\n";
		assert_eq!(
			stale_shortcuts(po),
			vec![StaleShortcut {
				msgid: "Reopen Last Closed".to_string(),
				msgstr: "Rouvrir & Dernière fermeture    Ctrl+Maj+T".to_string(),
			}]
		);
	}

	#[test]
	fn a_plain_translation_is_fine() {
		assert!(stale_shortcuts("msgid \"Close All\"\nmsgstr \"Tout fermer\"\n").is_empty());
	}

	/// The old-style msgids carry a shortcut of their own, so a translation that has one too is
	/// what it should be rather than a copy from somewhere else.
	#[test]
	fn a_shortcut_on_both_sides_is_fine() {
		let po = "msgid \"Close &All\\tCtrl+Shift+F4\"\nmsgstr \"Tout &fermer\\tCtrl+Shift+F4\"\n";
		assert!(stale_shortcuts(po).is_empty());
	}

	#[test]
	fn an_obsolete_entry_is_left_alone() {
		let po = "#~ msgid \"Go &Back\\tCtrl+[\"\n#~ msgstr \"Aller & Retour    Ctrl+[\"\n";
		assert!(stale_shortcuts(po).is_empty());
	}

	/// Every catalogue in the project, so a merge cannot ship a stale shortcut again.
	#[test]
	fn no_catalogue_carries_a_stale_shortcut() {
		if let Err(report) = check_catalogues(&crate::workspace::project_root()) {
			panic!("{report}");
		}
	}
}
