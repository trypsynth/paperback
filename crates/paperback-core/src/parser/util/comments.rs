//! Read where they sit, comments interrupt the text, and dropped they lose what reviewers said, so they go after the text, each linking back to its place.

use crate::{
	document::{DocPos, DocumentBuffer, Marker, MarkerType},
	t,
	util::text::collapse_whitespace,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Comment {
	pub key: String,
	pub author: String,
	pub text: String,
	pub start: DocPos,
	/// `None` for a comment on a point rather than on words.
	pub end: Option<DocPos>,
}

pub fn write_comments(buffer: &mut DocumentBuffer, comments: Vec<Comment>) {
	if comments.is_empty() {
		return;
	}
	if !buffer.content.is_empty() && !buffer.content.ends_with('\n') {
		buffer.append("\n");
	}
	// TRANSLATORS: Heading over the comments reviewers left on a document, written after its text
	let heading = t("Comments");
	let heading_offset = buffer.end();
	buffer.append(&heading);
	buffer.append("\n");
	buffer.add_marker(Marker::new(MarkerType::Heading1, heading_offset).with_text(heading).with_level(1));
	for comment in comments {
		// TRANSLATORS: Names a comment's author when the document does not say who wrote it
		let author = if comment.author.is_empty() { t("Comment") } else { comment.author };
		let quote = comment.end.filter(|end| *end > comment.start).map(|end| quoted(buffer, comment.start, end));
		let entry = match quote.filter(|quote| !quote.is_empty()) {
			// TRANSLATORS: One comment, after the document's text: the first {} is its author, the second the words it was made on, and the third the comment
			Some(quote) => t("{} on \u{201c}{}\u{201d}: {}")
				.replacen("{}", &author, 1)
				.replacen("{}", &quote, 1)
				.replacen("{}", &comment.text, 1),
			// TRANSLATORS: One comment made on a point rather than on words, after the document's text: the first {} is its author, the second the comment
			None => t("{}: {}").replacen("{}", &author, 1).replacen("{}", &comment.text, 1),
		};
		let offset = buffer.end();
		buffer.append(&entry);
		buffer.append("\n");
		buffer.add_marker(
			Marker::new(MarkerType::Link, offset).with_text(entry).with_reference(format!("#{}", comment.key)),
		);
	}
}

/// Cut short when long, since the entry only has to say which words.
fn quoted(buffer: &DocumentBuffer, start: DocPos, end: DocPos) -> String {
	const LONGEST: usize = 60;
	let words = buffer.slice(buffer.byte_at(start)..buffer.byte_at(end));
	let words = collapse_whitespace(words).trim().to_string();
	if words.chars().count() <= LONGEST {
		return words;
	}
	let mut short: String = words.chars().take(LONGEST).collect();
	short.truncate(short.trim_end().len());
	short.push('\u{2026}');
	short
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::document::DocLen;

	#[test]
	fn a_long_quote_is_cut_short() {
		let mut buffer = DocumentBuffer::new();
		let words = "word ".repeat(30);
		buffer.append(&words);
		let comment = Comment {
			key: "c".into(),
			author: "Ann".into(),
			text: "Too long.".into(),
			start: 0.into(),
			end: Some(DocLen::of(&words).as_pos()),
		};
		write_comments(&mut buffer, vec![comment]);
		let entry = buffer.content.lines().last().unwrap();
		assert!(entry.starts_with("Ann on \u{201c}word word"), "{entry}");
		assert!(entry.ends_with("word\u{2026}\u{201d}: Too long."), "{entry}");
	}
}
