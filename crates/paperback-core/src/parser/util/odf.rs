use std::fmt::Write as _;

use roxmltree::Node;

use crate::parser::{convert::image_text, odf_crypto::read_odf_content};

/// ODF writes runs of spaces as `<text:s text:c="3"/>` because XML collapses them, so without this words run together.
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

/// A picture's alt text is read through its placeholder instead.
#[must_use]
pub fn is_hidden(node: Node) -> bool {
	if !node.is_element() {
		return false;
	}
	let name = node.tag_name().name();
	let is_svg = node.tag_name().namespace().is_some_and(|namespace| namespace.contains("svg"));
	(is_svg && matches!(name, "title" | "desc")) || matches!(name, "annotation" | "annotation-end")
}

/// Looked for only inside `office:meta`, since a picture's alt text and a title field in the text are also named `title`.
#[must_use]
pub fn metadata(document: &roxmltree::Document) -> (Option<String>, Option<String>) {
	let Some(meta) = document.descendants().find(|node| {
		node.tag_name().name() == "meta"
			&& node.tag_name().namespace().is_some_and(|namespace| namespace.contains(":office:"))
	}) else {
		return (None, None);
	};
	let field = |name: &str| {
		meta.children()
			.find(|child| child.tag_name().name() == name)
			.and_then(|child| child.text())
			.map(str::trim)
			.filter(|text| !text.is_empty())
			.map(str::to_string)
	};
	(field("title"), field("initial-creator").or_else(|| field("creator")))
}

#[must_use]
pub fn packaged_metadata(path: &str, password: Option<&str>) -> (Option<String>, Option<String>) {
	read_odf_content(path, "meta.xml", password)
		.ok()
		.and_then(|xml| roxmltree::Document::parse(&xml).ok().map(|document| metadata(&document)))
		.unwrap_or_default()
}

#[must_use]
pub fn note_citation(note: Node) -> String {
	note.children()
		.find(|child| child.is_element() && child.tag_name().name() == "note-citation")
		.map(plain_text)
		.unwrap_or_default()
}

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

	#[test]
	fn metadata_gives_the_title_and_creator_and_ignores_titles_elsewhere() {
		let xml = format!(
			"<office:document {NAMESPACES} xmlns:meta=\"urn:oasis:names:tc:opendocument:xmlns:meta:1.0\"><office:meta><dc:title>Field Notes</dc:title><meta:initial-creator>Ann</meta:initial-creator><dc:creator>Bob</dc:creator></office:meta><office:body><svg:title>Not this</svg:title></office:body></office:document>"
		);
		let document = Document::parse(&xml).unwrap();
		assert_eq!(metadata(&document), (Some("Field Notes".into()), Some("Ann".into())));
		let bare = format!(
			"<office:document {NAMESPACES}><office:body><svg:title>Not this</svg:title></office:body></office:document>"
		);
		assert_eq!(metadata(&Document::parse(&bare).unwrap()), (None, None));
	}
}
