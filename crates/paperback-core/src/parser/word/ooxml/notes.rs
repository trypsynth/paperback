//! Footnotes and endnotes. Word keeps their text in parts of their own, `footnotes.xml` and `endnotes.xml`, and cites each in the document by an id, numbering them in the order they are cited.

use roxmltree::{Document as XmlDocument, Node};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(super) enum NoteKind {
	Footnote,
	Endnote,
}

impl NoteKind {
	/// The kind of note a run cites, and the reference element that cites it.
	pub(super) fn cited_by<'a, 'input>(run: Node<'a, 'input>) -> Option<(Self, Node<'a, 'input>)> {
		run.children().filter(Node::is_element).find_map(|child| match child.tag_name().name() {
			"footnoteReference" => Some((Self::Footnote, child)),
			"endnoteReference" => Some((Self::Endnote, child)),
			_ => None,
		})
	}

	/// The part that holds this kind of note, its relationships, and the element each note is.
	pub(super) const fn part(self) -> (&'static str, &'static str, &'static str) {
		match self {
			Self::Footnote => ("word/footnotes.xml", "word/_rels/footnotes.xml.rels", "footnote"),
			Self::Endnote => ("word/endnotes.xml", "word/_rels/endnotes.xml.rels", "endnote"),
		}
	}
}

/// One citation: which note it is, the label it shows, and the id its link lands on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct Citation {
	pub kind: NoteKind,
	pub id: String,
	pub label: String,
	pub key: String,
}

/// The notes cited so far, in the order their citations appear, which is also how Word numbers them.
#[derive(Debug, Default)]
pub(super) struct Citations {
	footnote_format: String,
	endnote_format: String,
	footnotes: usize,
	endnotes: usize,
	/// Tells one document's notes from another's when a zip holds several, since each numbers its own from one.
	document_start: usize,
	pub cited: Vec<Citation>,
}

impl Citations {
	/// Numbered as `word/settings.xml` says, or as Word does by default: footnotes 1, 2, 3 and endnotes i, ii, iii.
	pub(super) fn new(settings_xml: Option<&str>, document_start: usize) -> Self {
		let settings = settings_xml.and_then(|xml| XmlDocument::parse(xml).ok());
		let format = |properties: &str, default: &str| {
			settings
				.as_ref()
				.and_then(|settings| settings.descendants().find(|node| node.tag_name().name() == properties))
				.and_then(|properties| properties.children().find(|child| child.tag_name().name() == "numFmt"))
				.and_then(|format| format.attribute("val"))
				.unwrap_or(default)
				.to_string()
		};
		Self {
			footnote_format: format("footnotePr", "decimal"),
			endnote_format: format("endnotePr", "lowerRoman"),
			document_start,
			..Self::default()
		}
	}

	/// Records a citation of note `id` and returns it. A custom mark, such as an asterisk the author typed in place of a number, is the label as it stands.
	pub(super) fn cite(&mut self, kind: NoteKind, id: &str, custom_mark: Option<String>) -> Citation {
		let (count, format) = match kind {
			NoteKind::Footnote => (&mut self.footnotes, &self.footnote_format),
			NoteKind::Endnote => (&mut self.endnotes, &self.endnote_format),
		};
		*count += 1;
		let label = custom_mark.filter(|mark| !mark.is_empty()).unwrap_or_else(|| format_number(*count, format));
		let prefix = match kind {
			NoteKind::Footnote => "footnote",
			NoteKind::Endnote => "endnote",
		};
		let citation =
			Citation { kind, id: id.to_string(), label, key: format!("{prefix}-{}-{id}", self.document_start) };
		self.cited.push(citation.clone());
		citation
	}
}

/// `number` the way a Word number format writes it, decimal for any format this does not know.
fn format_number(number: usize, format: &str) -> String {
	match format {
		"lowerRoman" => roman(number).to_lowercase(),
		"upperRoman" => roman(number),
		"lowerLetter" => letters(number).to_lowercase(),
		"upperLetter" => letters(number),
		_ => number.to_string(),
	}
}

fn roman(mut number: usize) -> String {
	const NUMERALS: [(usize, &str); 13] = [
		(1000, "M"),
		(900, "CM"),
		(500, "D"),
		(400, "CD"),
		(100, "C"),
		(90, "XC"),
		(50, "L"),
		(40, "XL"),
		(10, "X"),
		(9, "IX"),
		(5, "V"),
		(4, "IV"),
		(1, "I"),
	];
	let mut text = String::new();
	for (value, numeral) in NUMERALS {
		while number >= value {
			text.push_str(numeral);
			number -= value;
		}
	}
	text
}

/// Word's letters go A to Z, then AA to ZZ, then AAA, repeating the letter rather than counting in base 26.
fn letters(number: usize) -> String {
	let Some(index) = number.checked_sub(1) else { return String::new() };
	let letter = char::from(b'A' + u8::try_from(index % 26).unwrap_or(0));
	letter.to_string().repeat(index / 26 + 1)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn notes_number_as_word_does_by_default() {
		let mut citations = Citations::new(None, 0);
		assert_eq!(citations.cite(NoteKind::Footnote, "2", None).label, "1");
		assert_eq!(citations.cite(NoteKind::Endnote, "1", None).label, "i");
		assert_eq!(citations.cite(NoteKind::Footnote, "3", None).label, "2");
		assert_eq!(citations.cite(NoteKind::Endnote, "2", None).label, "ii");
	}

	#[test]
	fn the_documents_settings_choose_the_numbering() {
		let settings = r#"<settings><footnotePr><numFmt val="upperLetter"/></footnotePr><endnotePr><numFmt val="decimal"/></endnotePr></settings>"#;
		let mut citations = Citations::new(Some(settings), 0);
		assert_eq!(citations.cite(NoteKind::Footnote, "1", None).label, "A");
		assert_eq!(citations.cite(NoteKind::Endnote, "1", None).label, "1");
	}

	#[test]
	fn a_custom_mark_is_the_label_and_keys_tell_documents_apart() {
		let mut first = Citations::new(None, 0);
		let mut second = Citations::new(None, 500);
		assert_eq!(first.cite(NoteKind::Footnote, "1", Some("*".to_string())).label, "*");
		assert_ne!(first.cite(NoteKind::Footnote, "2", None).key, second.cite(NoteKind::Footnote, "2", None).key);
	}

	#[test]
	fn number_formats() {
		assert_eq!(format_number(14, "lowerRoman"), "xiv");
		assert_eq!(format_number(1994, "upperRoman"), "MCMXCIV");
		assert_eq!(format_number(27, "lowerLetter"), "aa");
		assert_eq!(format_number(3, "cardinalText"), "3");
	}
}
