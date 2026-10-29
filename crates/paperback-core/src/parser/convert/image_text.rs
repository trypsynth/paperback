//! Shared by the HTML and XML converters so the two describe pictures alike.

use crate::{t, util::text::collapse_whitespace};

/// An empty `alt` doesn't end the search, so `alt="" aria-label="Map"` still reads "Map".
pub(super) fn description(attribute: &dyn Fn(&str) -> Option<String>) -> String {
	["alt", "aria-label", "aria-description", "title"]
		.into_iter()
		.find_map(|name| {
			attribute(name)
				.map(|value| collapse_whitespace(&value).trim().to_string())
				.filter(|value| !value.is_empty())
		})
		.unwrap_or_default()
}

/// Any other undescribed picture is still shown so it can be found, saved and read with OCR. Only asked once [`description`] is empty, so a present `alt` was an empty one.
pub(super) fn is_decorative(attribute: &dyn Fn(&str) -> Option<String>) -> bool {
	let says =
		|name: &str, value: &str| attribute(name).is_some_and(|actual| actual.trim().eq_ignore_ascii_case(value));
	let tiny = |name: &str| {
		attribute(name)
			.and_then(|size| size.trim().trim_end_matches("px").parse::<f32>().ok())
			.is_some_and(|size| size <= 1.0)
	};
	attribute("alt").is_some()
		|| says("role", "presentation")
		|| says("role", "none")
		|| says("aria-hidden", "true")
		|| tiny("width")
		|| tiny("height")
}

pub fn placeholder(is_figure: bool, description: &str) -> String {
	// TRANSLATORS: Label inserted before a figure or image's description, e.g. "[Figure: a cat sleeping]", and on its own, "[Image]", for a picture with none
	let label = if is_figure { t("Figure") } else { t("Image") };
	if description.is_empty() { format!("[{label}]") } else { format!("[{label}: {description}]") }
}

#[cfg(test)]
mod tests {
	use super::*;

	fn attributes(pairs: &'static [(&'static str, &'static str)]) -> impl Fn(&str) -> Option<String> {
		move |name| pairs.iter().find(|(key, _)| *key == name).map(|(_, value)| (*value).to_string())
	}

	#[test]
	fn an_empty_alt_does_not_stop_the_search_for_a_description() {
		assert_eq!(description(&attributes(&[("alt", ""), ("aria-label", "Map")])), "Map");
		assert_eq!(description(&attributes(&[("alt", "  a   cat ")])), "a cat");
		assert_eq!(description(&attributes(&[("src", "x.png")])), "");
	}

	#[test]
	fn marked_decoration_and_tracking_pixels_are_decorative() {
		assert!(is_decorative(&attributes(&[("alt", "")])));
		assert!(is_decorative(&attributes(&[("role", "presentation")])));
		assert!(is_decorative(&attributes(&[("aria-hidden", "TRUE")])));
		assert!(is_decorative(&attributes(&[("width", "1"), ("height", "1px")])));
		assert!(!is_decorative(&attributes(&[("src", "chart.png"), ("width", "400")])));
	}

	#[test]
	fn a_picture_without_a_description_still_has_a_placeholder() {
		assert_eq!(placeholder(false, ""), "[Image]");
		assert_eq!(placeholder(true, "a cat"), "[Figure: a cat]");
	}
}
