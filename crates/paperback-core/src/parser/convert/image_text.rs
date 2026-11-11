//! Shared by the HTML and XML converters so the two describe pictures alike.

use std::{collections::HashSet, io::Cursor};

use scraper::{Html, Selector};

use crate::{t, util::text::collapse_whitespace};

/// The sources of a page's pictures that have an empty `alt` and no other description, which a format can measure to find the ones that are content.
#[must_use]
pub fn empty_alt_sources(markup: &str) -> Vec<String> {
	if !markup.contains("alt=\"\"") && !markup.contains("alt=''") {
		return Vec::new();
	}
	let Ok(selector) = Selector::parse("img[alt]") else {
		return Vec::new();
	};
	Html::parse_document(markup)
		.select(&selector)
		.filter(|element| description(&|name| element.attr(name).map(str::to_string)).is_empty())
		.filter_map(|element| element.attr("src").map(|source| source.trim().to_string()))
		.filter(|source| !source.is_empty())
		.collect()
}

/// Whether the picture in `bytes` is at least [`CONTENT_IMAGE_SIDE`] on its shortest side, which its header says without decoding it.
#[must_use]
pub fn is_content_sized(bytes: &[u8]) -> bool {
	image::ImageReader::new(Cursor::new(bytes))
		.with_guessed_format()
		.ok()
		.and_then(|reader| reader.into_dimensions().ok())
		.is_some_and(|(width, height)| width.min(height) >= CONTENT_IMAGE_SIDE)
}

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

/// A picture at least this many pixels on its shortest side is big enough to be content, a scanned letter or a map, rather than an ornament or a divider.
pub const CONTENT_IMAGE_SIDE: u32 = 200;

/// Any other undescribed picture is still shown so it can be found, saved and read with OCR. Only asked once [`description`] is empty, so a present `alt` was an empty one.
///
/// An empty `alt` marks decoration on the web, but ebooks give it to every picture, scanned letters included, so such a picture is shown anyway when `content_images` holds its `source`: the pictures the format could measure and found at least [`CONTENT_IMAGE_SIDE`] on each side.
pub(super) fn is_decorative(
	attribute: &dyn Fn(&str) -> Option<String>,
	source: &str,
	content_images: &HashSet<String>,
) -> bool {
	let says =
		|name: &str, value: &str| attribute(name).is_some_and(|actual| actual.trim().eq_ignore_ascii_case(value));
	let tiny = |name: &str| {
		attribute(name)
			.and_then(|size| size.trim().trim_end_matches("px").parse::<f32>().ok())
			.is_some_and(|size| size <= 1.0)
	};
	(attribute("alt").is_some() && !content_images.contains(source))
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
		let none = HashSet::new();
		assert!(is_decorative(&attributes(&[("alt", "")]), "", &none));
		assert!(is_decorative(&attributes(&[("role", "presentation")]), "", &none));
		assert!(is_decorative(&attributes(&[("aria-hidden", "TRUE")]), "", &none));
		assert!(is_decorative(&attributes(&[("width", "1"), ("height", "1px")]), "", &none));
		assert!(!is_decorative(&attributes(&[("src", "chart.png"), ("width", "400")]), "chart.png", &none));
	}

	/// Ebooks give every picture an empty `alt`, the scanned letters of Rex Stout's The Silent Speaker included.
	#[test]
	fn a_picture_with_an_empty_alt_is_shown_once_measured_as_content() {
		let letters = HashSet::from(["images/letter.jpg".to_string()]);
		assert!(!is_decorative(&attributes(&[("alt", "")]), "images/letter.jpg", &letters));
		assert!(is_decorative(&attributes(&[("alt", "")]), "images/rule.png", &letters));
		assert!(is_decorative(&attributes(&[("alt", ""), ("role", "presentation")]), "images/letter.jpg", &letters));
	}

	#[test]
	fn a_picture_without_a_description_still_has_a_placeholder() {
		assert_eq!(placeholder(false, ""), "[Image]");
		assert_eq!(placeholder(true, "a cat"), "[Figure: a cat]");
	}
}
