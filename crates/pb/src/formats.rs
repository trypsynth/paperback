use std::fmt::Write as _;

use paperback_core::parser::ParserRegistry;

/// Grouped by format rather than as bare extensions, because `.1` and `.opf` say nothing on their own.
#[must_use]
pub fn listing() -> String {
	let mut formats: Vec<(&'static str, String)> = ParserRegistry::global()
		.all_parsers()
		.iter()
		.map(|parser| (parser.name(), parser.extensions().join(", ")))
		.collect();
	formats.sort_unstable();
	let widest = formats.iter().map(|(name, _)| name.len()).max().unwrap_or(0);
	let mut out = String::new();
	for (name, extensions) in formats {
		let _ = writeln!(out, "{name:widest$}  {extensions}");
	}
	out
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn every_registered_format_is_listed_with_its_extensions() {
		let listing = listing();
		for parser in ParserRegistry::global().all_parsers() {
			let line = listing
				.lines()
				.find(|line| line.starts_with(parser.name()))
				.unwrap_or_else(|| panic!("{} is missing from the listing", parser.name()));
			for extension in parser.extensions() {
				assert!(line.contains(extension), "{line} does not name .{extension}");
			}
		}
	}

	#[test]
	fn the_listing_is_one_sorted_line_per_format() {
		let listing = listing();
		let lines: Vec<&str> = listing.lines().collect();
		assert_eq!(lines.len(), ParserRegistry::global().all_parsers().len());
		let mut sorted = lines.clone();
		sorted.sort_unstable();
		assert_eq!(lines, sorted, "the formats are not in order");
	}

	#[test]
	fn the_manual_page_sections_are_listed_under_their_format() {
		let listing = listing();
		let line = listing.lines().find(|line| line.starts_with("Manual pages")).expect("manual pages are listed");
		assert!(line.contains("man"), "{line}");
		assert!(line.contains('1'), "{line}");
	}
}
