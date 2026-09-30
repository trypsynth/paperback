use paperback_core::version;
use patois::t;
use wx_utils::AboutBoxBuilder;
use wxdragon::prelude::*;

/// Everyone named in a `po` catalog's `Last-Translator` header, gathered at build time.
mod translators {
	include!(concat!(env!("OUT_DIR"), "/translators.rs"));
}

/// Paperback's own license, read from the file at build time so the dialog cannot drift from it.
static LICENSE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../LICENSE.md"));

/// Where the libraries Paperback is built on are credited.
///
/// A link rather than a copy: the credits run to hundreds of licenses and are generated from the
/// dependency tree when the site is built, so anything shipped inside the application would be
/// out of date by the next release.
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

/// Adds every translator to the dialog, so the people who made Paperback readable in their own
/// language are credited in the application rather than only in the catalogs.
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

	/// The dialog has to name both licenses: Paperback's own, and where to find everyone else's.
	/// Losing either one is the kind of thing nobody notices until it matters.
	#[test]
	fn the_licence_text_carries_the_license_and_the_credits_link() {
		let text = licence_text();
		assert!(text.contains("MIT License"), "the project's own license");
		assert!(text.contains("Permission is hereby granted"), "the license body, not just its name");
		assert!(text.contains(CREDITS_URL), "somewhere to find the third-party credits");
	}
}
