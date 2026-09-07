//! Compiling the committed `po/*.po` catalogs into the `.mo` files the app loads at runtime.

// Regenerating paperback.pot is deliberately NOT done here: a regen on every `cargo build`
// fights with `cargo xtask translate` and the auto-translate CI job (which regenerate it
// carefully, suppressing pure timestamp/wrapping churn) and touches the tracked .pot file
// mid-build. Run `cargo xtask translate` (or `--dry-run` to preview) to regenerate it.
pub fn build() {
	patois_build::compile_translations("../../po", "locale");
}
