use patois::t;
use wxdragon::prelude::*;

use super::go_to::{NumberPrompt, show_number_dialog};

pub fn show_go_to_line_dialog(
	parent: &Frame,
	current_line: i32,
	max_lines: i32,
	live_region_label: StaticText,
) -> Option<i32> {
	let max_lines = max_lines.max(1);
	// TRANSLATORS: Title of the Go to Line dialog
	let title = t("Go to Line");
	// TRANSLATORS: Label/prompt template for the line selection dialog. The %d placeholders represent current_line and max_lines respectively.
	let label = t("Go to line (%d/%d):").replacen("%d", &current_line.clamp(1, max_lines).to_string(), 1).replacen(
		"%d",
		&max_lines.to_string(),
		1,
	);
	// TRANSLATORS: Announced when the entered line number is outside the document's range
	let out_of_range = t("Line out of range.");
	let prompt = NumberPrompt {
		title: &title,
		label: &label,
		out_of_range: &out_of_range,
		current: current_line,
		min: 1,
		max: max_lines,
	};
	show_number_dialog(parent, &prompt, live_region_label)
}
