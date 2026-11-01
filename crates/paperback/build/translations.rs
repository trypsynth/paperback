use std::env;

// The .pot is deliberately not regenerated here: doing it every build fights `cargo xtask translate` and the auto-translate CI job, which suppress timestamp and wrapping churn.
pub fn build() {
	patois_build::compile_translations("../../po", "locale");
	// `patois::embed_wx_translations!()` finds wx's catalogs through `PROFILE`, which Cargo only sets for build scripts; without it stock wx strings stay English (#958).
	if let Ok(profile) = env::var("PROFILE") {
		println!("cargo:rustc-env=PROFILE={profile}");
	}
}
