use paperback_core::version;
use patois::t;
use wx_utils::AboutBoxBuilder;
use wxdragon::prelude::*;

mod translators {
	include!(concat!(env!("OUT_DIR"), "/translators.rs"));
}

static LICENSE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../LICENSE.md"));

/// A link rather than a copy: the credits are generated from the dependency tree when the site is built, so a shipped copy would go stale.
const CREDITS_URL: &str = "https://paperback.dev/licenses";

pub fn show_about_dialog(parent: &Frame) {
	let builder = AboutBoxBuilder::new(parent)
		.name("Paperback")
		.version(version::display_version())
		// TRANSLATORS: Description/tagline of the application shown in the About dialog
		.description(t("An accessible, lightweight, fast ebook and document reader"))
		.copyright("Copyright (C) 2025-2026 Quin Gillespie")
		.website("https://paperback.dev")
		.licence(licence_text())
		.add_developer("Quin Gillespie");
	credit_translators(builder).show();
}

fn credit_translators(mut builder: AboutBoxBuilder<'_>) -> AboutBoxBuilder<'_> {
	for translator in translators::TRANSLATORS {
		builder = builder.add_translator(*translator);
	}
	builder
}

fn licence_text() -> String {
	// TRANSLATORS: Shown in the About dialog above the license text, pointing at the web page that credits the libraries Paperback is built on. {} is a web address.
	let credits =
		t("Paperback is built on other people's work. The libraries it uses, and their licenses, are credited at {}.")
			.replace("{}", CREDITS_URL);
	format!("{}\n\n{credits}", LICENSE.trim_end())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn the_licence_text_carries_the_license_and_the_credits_link() {
		let text = licence_text();
		assert!(text.contains("MIT License"), "the project's own license");
		assert!(text.contains("Permission is hereby granted"), "the license body, not just its name");
		assert!(text.contains(CREDITS_URL), "somewhere to find the third-party credits");
	}
}
