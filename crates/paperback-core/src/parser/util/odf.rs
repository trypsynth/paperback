//! What ODT and ODP share in running text: the elements that stand for spacing, the parts of the markup that are not text at all, and how a picture reads.

use std::fmt::Write as _;

use roxmltree::Node;

use crate::parser::convert::image_text;

/// The text a spacing element stands for: `<text:s text:c="3"/>` for three spaces, a tab, or a line break. ODF writes runs of spaces this way because XML collapses them, so without it words run together.
#[must_use]
pub fn spacing(node: Node) -> Option<String> {
	if !node.is_element() {
		return None;
	}
	match node.tag_name().name() {
		"s" => Some(" ".repeat(node.attribute("c").and_then(|count| count.parse().ok()).unwrap_or(1))),
		"tab" => Some("\t".to_string()),
		"line-break" => Some("\n".to_string()),
		_ => None,
	}
}

/// Whether an element is markup rather than text: a picture's alt text, which its placeholder reads instead, or a comment, with its author and date.
#[must_use]
pub fn is_hidden(node: Node) -> bool {
	if !node.is_element() {
		return false;
	}
	let name = node.tag_name().name();
	let is_svg = node.tag_name().namespace().is_some_and(|namespace| namespace.contains("svg"));
	(is_svg && matches!(name, "title" | "desc")) || matches!(name, "annotation" | "annotation-end")
}

/// The number a footnote or endnote shows where it is cited.
#[must_use]
pub fn note_citation(note: Node) -> String {
	note.children()
		.find(|child| child.is_element() && child.tag_name().name() == "note-citation")
		.map(plain_text)
		.unwrap_or_default()
}

/// An element's text as it reads: spacing elements honoured, comments and alt text left out, a note reduced to its citation in brackets, and a picture to its placeholder.
#[must_use]
pub fn plain_text(node: Node) -> String {
	let mut text = String::new();
	collect(node, &mut text);
	text
}

fn collect(node: Node, text: &mut String) {
	for child in node.children() {
		if child.is_text() {
			text.push_str(child.text().unwrap_or_default());
		} else if let Some(spaces) = spacing(child) {
			text.push_str(&spaces);
		} else if is_hidden(child) {
		} else if child.tag_name().name() == "note" {
			let _ = write!(text, "[{}]", note_citation(child));
		} else if let Some((placeholder, _)) = picture(child) {
			text.push_str(&placeholder);
		} else {
			collect(child, text);
		}
	}
}

/// The placeholder a frame holding a picture shows, such as `[Image: a cat]`, and the description in it, read from the frame's title, the alternative text, or else its description. `None` for any other element, including a frame of text, which reads as its text.
#[must_use]
pub fn picture(frame: Node) -> Option<(String, String)> {
	if !frame.is_element() || frame.tag_name().name() != "frame" {
		return None;
	}
	frame.children().find(|child| child.is_element() && child.tag_name().name() == "image")?;
	let described = |name: &str| {
		frame
			.children()
			.find(|child| child.is_element() && child.tag_name().name() == name && is_hidden(*child))
			.map(|child| child.text().unwrap_or_default().trim().to_string())
			.filter(|text| !text.is_empty())
	};
	let description = described("title").or_else(|| described("desc")).unwrap_or_default();
	Some((image_text::placeholder(false, &description), description))
}

#[cfg(test)]
mod tests {
	use roxmltree::Document;

	use super::*;

	const NAMESPACES: &str = r#"xmlns:text="urn:oasis:names:tc:opendocument:xmlns:text:1.0" xmlns:draw="urn:oasis:names:tc:opendocument:xmlns:drawing:1.0" xmlns:svg="urn:oasis:names:tc:opendocument:xmlns:svg-compatible:1.0" xmlns:office="urn:oasis:names:tc:opendocument:xmlns:office:1.0" xmlns:dc="http://purl.org/dc/elements/1.1/""#;

	fn text_of(body: &str) -> String {
		let xml = format!("<text:p {NAMESPACES}>{body}</text:p>");
		let document = Document::parse(&xml).unwrap();
		plain_text(document.root_element())
	}

	#[test]
	fn spacing_elements_keep_words_apart() {
		assert_eq!(
			text_of("one<text:s/>two<text:s text:c=\"3\"/>three<text:tab/>four<text:line-break/>five"),
			"one two   three\tfour\nfive"
		);
	}

	#[test]
	fn comments_and_alt_text_are_not_text_and_a_note_is_its_citation() {
		let body = "Said<office:annotation><dc:creator>Ann</dc:creator><dc:date>2026</dc:date><text:p>a comment</text:p></office:annotation> so<text:note><text:note-citation>1</text:note-citation><text:note-body><text:p>The note.</text:p></text:note-body></text:note>.";
		assert_eq!(text_of(body), "Said so[1].");
	}

	#[test]
	fn a_picture_reads_as_its_placeholder_with_its_alternative_text() {
		let body = "See <draw:frame><draw:image/><svg:title>A map</svg:title><svg:desc>Longer words</svg:desc></draw:frame> here <draw:frame><draw:image/></draw:frame>";
		assert_eq!(text_of(body), "See [Image: A map] here [Image]");
	}

	/// `text:title` is a field showing the document's title, not alt text, so only SVG's title is hidden.
	#[test]
	fn a_title_field_is_text() {
		assert_eq!(text_of("By <text:title>My Report</text:title>"), "By My Report");
	}
}
