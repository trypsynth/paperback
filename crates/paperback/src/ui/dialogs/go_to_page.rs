use patois::t;
use wxdragon::prelude::*;

use super::go_to::{NumberPrompt, show_number_dialog};

pub fn show_go_to_page_dialog(
	parent: &Frame,
	current_page: i32,
	max_page: i32,
	live_region_label: StaticText,
) -> Option<i32> {
	let max_page = max_page.max(1);
	// TRANSLATORS: Title of the Go to page dialog
	let title = t("Go to page");
	// TRANSLATORS: Label/prompt template for the page selection dialog. The %d placeholders represent current_page and max_page respectively.
	let label = t("Go to page (%d/%d):").replacen("%d", &current_page.clamp(1, max_page).to_string(), 1).replacen(
		"%d",
		&max_page.to_string(),
		1,
	);
	// TRANSLATORS: Announced when the entered page number is outside the document's range
	let out_of_range = t("Page out of range.");
	let prompt = NumberPrompt {
		title: &title,
		label: &label,
		out_of_range: &out_of_range,
		current: current_page,
		min: 1,
		max: max_page,
	};
	show_number_dialog(parent, &prompt, live_region_label)
}
