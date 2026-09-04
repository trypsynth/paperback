//! Plain-text extraction from a PDF page: reading pdfium's per-character stream into
//! visual lines (with RTL reordering per line), then joining those lines into paragraphs
//! and flagging large-font lines as headings by comparing against the page's median body
//! font size. Used as the fallback path for pages with no trustworthy structure tree (see
//! [`super::structure`]), and its [`sanitize_pdf_text`] helper is shared by metadata and TOC
//! extraction as well.

use std::{cmp::Ordering, mem};

use pdfium::PdfiumTextPage;

use crate::{
	parser::util::bidi,
	util::text::{collapse_whitespace, display_len, trim_string},
};

pub(super) fn sanitize_pdf_text(input: &str) -> String {
	input.chars().filter(|&ch| (!ch.is_control() || matches!(ch, '\n' | '\r' | '\t')) && ch != '\u{00AD}').collect()
}

fn is_cjk(c: char) -> bool {
	let u = c as u32;
	(0x4E00..=0x9FFF).contains(&u) || // CJK Unified Ideographs
	(0x3400..=0x4DBF).contains(&u) || // CJK Extension A
	(0x20000..=0x2A6DF).contains(&u) || // CJK Extension B
	(0x3040..=0x309F).contains(&u) || // Hiragana
	(0x30A0..=0x30FF).contains(&u) || // Katakana
	(0xAC00..=0xD7AF).contains(&u) // Hangul
}

fn char_x_origin(text_page: &PdfiumTextPage, i: i32) -> f32 {
	let (mut x, mut y) = (0.0, 0.0);
	let _ = text_page.get_char_origin(i, &mut x, &mut y);
	x as f32
}

/// Assemble one run of `(char, pdfium index)` pairs into text, reordering
/// visual→logical for RTL scripts. Fetches x origins (a per-char FFI call)
/// only when the run actually contains an RTL character, so pure-LTR runs —
/// the overwhelming majority — pay a single cheap classification scan instead.
pub(super) fn reorder_run(text_page: &PdfiumTextPage, chars: &[(char, i32)]) -> String {
	if !bidi::contains_rtl(chars.iter().map(|&(c, _)| c)) {
		return chars.iter().map(|&(c, _)| c).collect();
	}
	let with_origin: Vec<(char, f32)> = chars.iter().map(|&(c, i)| (c, char_x_origin(text_page, i))).collect();
	bidi::reorder_line(&with_origin)
}

pub(super) fn extract_text_lines(text_page: &PdfiumTextPage, page_index: i32) -> Vec<(String, f64)> {
	let Ok(char_count) = text_page.char_count() else {
		tracing::warn!(
			page_index,
			"page text char count unavailable, falling back to whole-page text blob, heading detection by font size will be degraded for this page"
		);
		let raw = sanitize_pdf_text(&text_page.full()).replace('\r', "");
		return raw.lines().map(|l| (l.to_string(), 0.0)).collect();
	};
	let mut result: Vec<(String, f64)> = Vec::new();
	// Chars of the current visual line with their pdfium index, so each line can be
	// reordered visual→logical (handles RTL scripts) before paragraph joining.
	let mut current_chars: Vec<(char, i32)> = Vec::new();
	// One `FPDFText_GetFontSize` FFI call per line rather than one per character: a real
	// document's line is rendered in one consistent size, so the first character with a
	// usable size stands in for the whole line. Per-character font-size calls (on top of the
	// per-character unicode call every line already pays) were the dominant cost of PDF
	// parsing on documents with many lines - see #747.
	let mut current_size = 0.0f64;
	for i in 0..char_count {
		let unicode = text_page.get_unicode(i);
		let Some(ch) = char::from_u32(unicode) else { continue };
		if ch == '\n' || ch == '\r' {
			result.push((reorder_run(text_page, &mem::take(&mut current_chars)), current_size));
			current_size = 0.0;
		} else if (ch.is_control() && !matches!(ch, '\t')) || ch == '\u{00AD}' {
			continue;
		} else {
			if current_size == 0.0 {
				current_size = text_page.get_font_size(i);
			}
			current_chars.push((ch, i));
		}
	}
	if !current_chars.is_empty() {
		result.push((reorder_run(text_page, &current_chars), current_size));
	}
	result
}

fn sorted_median(values: &mut [f64]) -> f64 {
	if values.is_empty() {
		return 0.0;
	}
	values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
	values[values.len() / 2]
}

pub(super) fn median_line_font_size(line_infos: &[(String, f64)]) -> f64 {
	let mut sizes: Vec<f64> = line_infos
		.iter()
		.filter(|(text, size)| !text.trim().is_empty() && *size > 0.0)
		.map(|(_, size)| *size)
		.collect();
	sorted_median(&mut sizes)
}

pub(super) fn join_paragraphs(raw_lines: &[(String, f64)], body_font_size: f64) -> Vec<(String, bool)> {
	const HEADING_FONT_RATIO: f64 = 1.2;
	const HEADING_MAX_LEN: usize = 150;
	let heading_threshold = if body_font_size > 0.0 { body_font_size * HEADING_FONT_RATIO } else { f64::INFINITY };
	let lines: Vec<(String, bool)> = raw_lines
		.iter()
		.map(|(text, size)| {
			let trimmed = trim_string(&collapse_whitespace(text));
			let len = display_len(&trimmed);
			let is_heading_line = *size >= heading_threshold && len > 0 && len <= HEADING_MAX_LEN;
			(trimmed, is_heading_line)
		})
		.collect();
	let mut max_len = 0usize;
	for (line, _) in &lines {
		let len = display_len(line);
		if len > max_len {
			max_len = len;
		}
	}
	let short_line_threshold = (max_len as f32 * 0.75) as usize;
	let mut paragraphs: Vec<(String, bool)> = Vec::new();
	let mut current_paragraph = String::new();
	let mut current_is_heading = false;
	let mut last_line_len = 0usize;
	let mut last_line_ends_with_punctuation = false;
	for (line, is_heading_line) in &lines {
		if line.is_empty() {
			if !current_paragraph.is_empty() {
				paragraphs.push((mem::take(&mut current_paragraph), current_is_heading));
				current_is_heading = false;
			}
			last_line_len = 0;
			last_line_ends_with_punctuation = false;
			continue;
		}
		let is_list_item = line.starts_with("- ") || line.starts_with("* ") || line.starts_with("• ");
		let first_char = line.chars().next();
		let starts_with_uppercase = first_char.is_some_and(char::is_uppercase);
		let starts_with_alpha = first_char.is_some_and(char::is_alphabetic);
		let len = display_len(line);
		if current_paragraph.is_empty() {
			current_paragraph = line.clone();
			current_is_heading = *is_heading_line;
		} else {
			let mut is_numbered = false;
			let mut chars = line.chars();
			if let Some(first) = chars.next()
				&& first.is_ascii_digit()
			{
				let mut found_space = false;
				for c in chars {
					if c.is_ascii_digit() || c == '.' || c == ')' {
						continue;
					} else if c.is_whitespace() {
						found_space = true;
						break;
					}
					break;
				}
				is_numbered = found_space;
			}
			// A heading (either side of the boundary) or a list item always starts a new paragraph;
			// otherwise a short previous line does, whether it ended on punctuation or the new line
			// looks like a fresh sentence.
			let break_paragraph = *is_heading_line
				|| current_is_heading
				|| is_list_item
				|| is_numbered
				|| (last_line_len < short_line_threshold
					&& (last_line_ends_with_punctuation || starts_with_uppercase || !starts_with_alpha));
			if break_paragraph {
				paragraphs.push((mem::take(&mut current_paragraph), current_is_heading));
				current_paragraph = line.clone();
				current_is_heading = *is_heading_line;
			} else {
				let last_char = current_paragraph.chars().last().unwrap_or(' ');
				if current_paragraph.ends_with('-') {
					current_paragraph.pop();
					current_paragraph.push_str(line);
				} else if is_cjk(last_char) && line.chars().next().is_some_and(is_cjk) {
					current_paragraph.push_str(line);
				} else {
					current_paragraph.push(' ');
					current_paragraph.push_str(line);
				}
			}
		}
		last_line_len = len;
		last_line_ends_with_punctuation = line.ends_with('.')
			|| line.ends_with('?')
			|| line.ends_with('!')
			|| line.ends_with(':')
			|| line.ends_with('"')
			|| line.ends_with('\u{201D}')
			|| line.ends_with('。')
			|| line.ends_with('？')
			|| line.ends_with('！')
			|| line.ends_with('：');
	}
	if !current_paragraph.is_empty() {
		paragraphs.push((current_paragraph, current_is_heading));
	}
	paragraphs
}

#[cfg(test)]
mod tests {
	use super::{join_paragraphs, sanitize_pdf_text};

	#[test]
	fn sanitize_pdf_text_strips_control_chars_and_soft_hyphens() {
		assert_eq!(sanitize_pdf_text("sugges\u{0002}tion\tline\r\nnext"), "suggestion\tline\r\nnext");
		assert_eq!(sanitize_pdf_text("hy\u{00AD}phen"), "hyphen");
	}

	#[test]
	fn join_paragraphs_merges_continuation_lines() {
		let lines = vec![("The suggestion appears here.".to_string(), 12.0), ("And here.".to_string(), 12.0)];
		let result = join_paragraphs(&lines, 12.0);
		assert_eq!(result.len(), 1);
		assert_eq!(result[0].0, "The suggestion appears here. And here.");
		assert!(!result[0].1);
	}

	#[test]
	fn join_paragraphs_flags_large_font_lines_as_headings() {
		let lines =
			vec![("Chapter One".to_string(), 18.0), ("This is the body text of the document.".to_string(), 12.0)];
		let result = join_paragraphs(&lines, 12.0);
		assert_eq!(result.len(), 2);
		assert_eq!(result[0].0, "Chapter One");
		assert!(result[0].1);
		assert_eq!(result[1].0, "This is the body text of the document.");
		assert!(!result[1].1);
	}
}
