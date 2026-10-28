use std::{
	error::Error,
	path::{Path, PathBuf},
};

use patois_build::lint::{check_catalogs, check_sources};

use crate::workspace::{crate_version, project_root, translatable_crate_src_dirs};

const SOURCE_DIRS: [&str; 3] = ["crates", "android/app/src/main/kotlin", "ios/Paperback"];

pub(crate) fn gen_pot() -> Result<(), Box<dyn Error>> {
	let root = project_root();
	let po_dir = root.join("po");
	let pot_file = po_dir.join("paperback.pot");
	let translatable_dirs = translatable_crate_src_dirs(&root)?;
	if translatable_dirs.is_empty() {
		return Err("no translatable crates found, check [package.metadata.patois] translatable = true".into());
	}
	// A concatenated call would land in the pot as a fragment no lookup can ever match.
	let source_dirs = SOURCE_DIRS.map(|dir| root.join(dir));
	check_sources(&source_dirs.each_ref().map(PathBuf::as_path))?;
	// A translation that kept an old menu string's shortcut is spoken with it, and pot regeneration is when such copies appear.
	check_catalogs(&po_dir)?;
	let version = crate_version(&root, "paperback")?;
	patois_build::gen_pot_from_dirs(&translatable_dirs, &po_dir, "paperback", &version)?;
	let ios_dir = root.join("ios/Paperback");
	// Starts below the sibling `kotlin/uniffi` bindings, which have no translatable strings.
	let android_dir = root.join("android/app/src/main/kotlin/dev/paperback/android");
	extend_pot_from(&pot_file, &ios_dir, "swift")?;
	extend_pot_from(&pot_file, &android_dir, "kt")?;
	for (tag, dir) in [("swift", &ios_dir), ("kt", &android_dir)] {
		let removed = patois_build::prune_pot_from_source_dirs(&[dir], tag, &pot_file)?;
		if !removed.is_empty() {
			println!("gen-pot: pruned {} dead {tag} entry/entries:", removed.len());
			for msgid in &removed {
				println!("  {msgid}");
			}
		}
	}
	Ok(())
}

/// Fails on a missing directory: a package rename once left the Kotlin path pointing at nothing, and Kotlin strings silently stopped being extracted.
fn extend_pot_from(pot_file: &Path, src: &Path, extension: &str) -> Result<(), Box<dyn Error>> {
	if !src.is_dir() {
		return Err(format!("{extension} sources not found at {}", src.display()).into());
	}
	patois_build::extend_pot_from_source_dirs(&[src], extension, pot_file)?;
	Ok(())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn no_source_file_concatenates_a_translation_call() {
		let root = project_root();
		let source_dirs = SOURCE_DIRS.map(|dir| root.join(dir));
		if let Err(report) = check_sources(&source_dirs.each_ref().map(PathBuf::as_path)) {
			panic!("{report}");
		}
	}

	#[test]
	fn no_catalog_carries_a_stale_shortcut() {
		if let Err(report) = check_catalogs(&project_root().join("po")) {
			panic!("{report}");
		}
	}
}
