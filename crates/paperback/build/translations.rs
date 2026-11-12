// The .pot is deliberately not regenerated here: doing it every build fights `cargo xtask translate` and the auto-translate CI job, which suppress timestamp and wrapping churn.
pub fn build() {
	patois_build::compile_translations("../../po", "locale");
}
