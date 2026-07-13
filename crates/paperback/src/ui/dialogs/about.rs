use paperback_core::version;
use patois::t;
use wx_utils::AboutBoxBuilder;
use wxdragon::prelude::*;

pub fn show_about_dialog(parent: &Frame) {
	AboutBoxBuilder::new(parent)
		.name("Paperback")
		.version(version::display_version())
		// TRANSLATORS: Description/tagline of the application shown in the About dialog
		.description(t("An accessible, lightweight, fast ebook and document reader"))
		.copyright("Copyright (C) 2025-2026 Quin Gillespie")
		.website("https://paperback.dev")
		.add_developer("Quin Gillespie")
		.show();
}
