use std::fs;

use anyhow::{Context, Result};
use rtf_parser::lexer::Lexer;

use crate::{
	document::{Document, ParserContext},
	parser::{Parser, text::TextParser, util::path::extract_title_from_path},
	t,
};

mod encoding;
mod escapes;
mod tokens;

use encoding::{extract_codepage, extract_font_table};
use escapes::{normalize_escapes, normalize_wrapped_space_lines};
use tokens::extract_content_from_tokens;

/// Whether the bytes are Rich Text Format: `{\rtf` at the start, past a byte order mark or
/// leading whitespace.
///
/// Worth checking before reading a file the extension already calls RTF. Plain text saved under
/// an `.rtf` name is common in scanned and scene-released book collections, and the lexer's
/// complaint about it ("Invalid last char, should be '}'") describes a brace the file never had,
/// which reads as a corrupt document rather than a misnamed one. Routing to the text parser
/// instead turns a book that will not open into a book that does.
pub(super) fn looks_like_rtf(bytes: &[u8]) -> bool {
	let bytes = bytes.strip_prefix(&[0xEF, 0xBB, 0xBF]).unwrap_or(bytes);
	let start = bytes.iter().position(|b| !b.is_ascii_whitespace()).unwrap_or(bytes.len());
	bytes[start..].starts_with(br"{\rtf")
}

pub struct RtfParser;

impl Parser for RtfParser {
	fn parse(&self, context: &ParserContext) -> Result<Document> {
		tracing::debug!(path = %context.file_path, "parsing rtf document");
		let bytes =
			fs::read(&context.file_path).with_context(|| format!("Failed to open RTF file '{}'", context.file_path))?;
		if !looks_like_rtf(&bytes) {
			tracing::debug!(path = %context.file_path, "rtf file has no rtf header, routing to the text parser");
			return TextParser.parse(context);
		}
		let content_str = String::from_utf8_lossy(&bytes);
		// Some RTF files have garbage at the end
		let content_str = content_str.trim_end_matches(|c: char| c == '\0' || c.is_whitespace());
		let content_str = normalize_wrapped_space_lines(content_str);
		let encoding = extract_codepage(&content_str);
		tracing::debug!(path = %context.file_path, encoding = %encoding.name(), "resolved rtf document encoding");
		let font_table = extract_font_table(&content_str, encoding);
		let content_str = normalize_escapes(&content_str, encoding, &font_table);
		// Strip \r so that \r\n line endings don't leave stray carriage returns in text tokens
		let content_str = content_str.replace('\r', "");
		let tokens = Lexer::scan(&content_str).map_err(|e| {
			tracing::warn!(path = %context.file_path, error = %e, "failed to scan rtf document tokens");
			// TRANSLATORS: Error shown when an RTF document's tokens fail to parse; {} is the underlying lexer error
			anyhow::anyhow!(t("Failed to parse RTF document: {}").replace("{}", &e.to_string()))
		})?;
		let buffer = extract_content_from_tokens(&tokens);
		let title = extract_title_from_path(&context.file_path);
		let mut doc = Document::new().with_title(title);
		doc.set_buffer(buffer);
		tracing::debug!(path = %context.file_path, "parsed rtf document successfully");
		Ok(doc)
	}
}

#[cfg(test)]
mod tests {
	use super::looks_like_rtf;

	#[test]
	fn rtf_is_recognised_past_a_bom_or_whitespace() {
		assert!(looks_like_rtf(br"{\rtf1\ansi hello}"));
		assert!(looks_like_rtf(b"\xEF\xBB\xBF{\\rtf1 x}"), "a byte order mark before the brace");
		assert!(looks_like_rtf(b"  \r\n{\\rtf1 x}"), "leading whitespace");
	}

	/// Plain text under an `.rtf` name, which is what sent this check into the parser.
	#[test]
	fn a_file_with_no_rtf_header_is_not_rtf() {
		assert!(!looks_like_rtf(b"JOB: A Comedy of Justice\r\nRobert A. Heinlein\r\n"));
		assert!(!looks_like_rtf(b""));
		assert!(!looks_like_rtf(b"   \r\n\t  "), "whitespace all the way to the end");
	}

	/// A brace alone is not enough: the control word has to follow it, or anything else that
	/// opens with one would be read as RTF.
	#[test]
	fn a_leading_brace_without_the_control_word_is_not_rtf() {
		assert!(!looks_like_rtf(br#"{"title": "not rtf"}"#));
		assert!(!looks_like_rtf(b"{"));
	}

	/// The Windows Write binary magic, which `wri` routes on before falling back to text.
	#[test]
	fn the_write_binary_magic_is_not_rtf() {
		assert!(!looks_like_rtf(b"\x31\xBE\x00\x00"));
	}
}
