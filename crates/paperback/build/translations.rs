//! Compiling the committed `po/*.po` catalogs into the `.mo` files the app loads at runtime.

use std::env;

// Regenerating paperback.pot is deliberately NOT done here: a regen on every `cargo build`
// fights with `cargo xtask translate` and the auto-translate CI job (which regenerate it
// carefully, suppressing pure timestamp/wrapping churn) and touches the tracked .pot file
// mid-build. Run `cargo xtask translate` (or `--dry-run` to preview) to regenerate it.
pub fn build() {
	patois_build::compile_translations("../../po", "locale");
	// `patois::embed_wx_translations!()` finds wxWidgets' own catalogs through `PROFILE`, which
	// Cargo sets for build scripts but not for the compile the macro runs in. Without this it
	// finds nothing, embeds nothing, and every stock wx string (OK, Cancel, the About box's
	// License, Developers and Translators) stays English in every language
	// (<https://github.com/trypsynth/paperback/issues/958>).
	if let Ok(profile) = env::var("PROFILE") {
		println!("cargo:rustc-env=PROFILE={profile}");
	}
}
