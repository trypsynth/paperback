//! Compiling the committed `po/*.po` catalogs into the `.mo` files the app loads at runtime.

// Only compiles the already-committed po/*.po files into .mo files for runtime loading.
// Regenerating paperback.pot itself is deliberately NOT done here: it used to run on every
// `cargo build`, fighting with `cargo xtask translate`/the auto-translate CI job (which
// regenerate it carefully, suppressing pure timestamp/wrapping churn) and touching the
// tracked .pot file mid-build. Run `cargo xtask translate` (or `--dry-run` to preview)
// to regenerate it instead.
pub fn build() {
	patois_build::compile_translations("../../po", "locale");
}
