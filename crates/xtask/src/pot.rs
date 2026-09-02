//! Generating `po/paperback.pot`: the Rust scan, the Kotlin and Swift scans layered on top,
//! and dropping entries whose call site is gone.

use std::{error::Error, path::Path};

use crate::{
	pot_lint,
	workspace::{crate_version, project_root, translatable_crate_src_dirs},
};

pub(crate) fn gen_pot() -> Result<(), Box<dyn Error>> {
	let root = project_root();
	let po_dir = root.join("po");
	let pot_file = po_dir.join("paperback.pot");
	// Step 1: generate from Rust crates tagged with translatable = true. patois-build
	// sanitizes the sources for `xgettext --language=C` itself, so the real source dirs go
	// straight in.
	let translatable_dirs = translatable_crate_src_dirs(&root)?;
	if translatable_dirs.is_empty() {
		return Err("no translatable crates found — check [package.metadata.patois] translatable = true".into());
	}
	// Before anything is written: a concatenated call would otherwise land in the pot as a
	// fragment that no lookup can ever match.
	pot_lint::check_sources(&root)?;
	let version = crate_version(&root, "paperback")?;
	patois_build::gen_pot_from_dirs(&translatable_dirs, &po_dir, "paperback", &version)?;
	// Steps 2 and 3: layer the mobile front-ends' own strings on top. Both directories are
	// required rather than skipped when absent: the Kotlin path spent a package rename
	// (dev.paperback.mobile -> dev.paperback.android) pointing at nothing, and because a
	// missing directory was silently fine, the only symptom was that Kotlin strings stopped
	// being extracted, which stayed invisible for as long as the pot carried the last good
	// copy of them forward.
	let ios_dir = root.join("ios/Paperback");
	// The uniffi-generated bindings live in the sibling `kotlin/uniffi` tree and have no
	// translatable strings, so the scan starts below them.
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

/// Scan one mobile front-end's sources and add whatever they turn up to the pot, failing loudly
/// if the directory isn't where it's expected to be.
fn extend_pot_from(pot_file: &Path, src: &Path, extension: &str) -> Result<(), Box<dyn Error>> {
	if !src.is_dir() {
		return Err(format!("{extension} sources not found at {}", src.display()).into());
	}
	patois_build::extend_pot_from_source_dirs(&[src], extension, pot_file)?;
	Ok(())
}
