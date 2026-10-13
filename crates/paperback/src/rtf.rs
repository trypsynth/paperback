//! Builds a minimal RTF document from plain text plus a set of non-overlapping
//! bold/italic/underline spans and the heading and list paragraphs a screen reader
//! reports, for the Windows RTF fast path in
//! `document_manager`. See `apply_formatting_markers_to_ctrl_from_segments` for why this
//! exists: the native `RichEdit` control backing a `wxTE_RICH2` `TextCtrl`
//! special-cases `WM_SETTEXT`. Text starting with `{\rtf` is parsed as RTF in
//! one shot instead of literal text, which is far cheaper than issuing one
//! `SetStyle` call per formatting span on documents with thousands of them.

use std::{borrow::Cow, fmt::Write as _};

use paperback_core::{session::ParagraphFormat, util::text::ch_width};

use crate::text_format::FormatSegment;

/// Replaces every character `RichEdit` silently discards instead of storing with a space, so
/// its buffer holds exactly one display unit per display unit of `content`. Borrows unchanged
/// when there is nothing to replace, which is the overwhelmingly common case.
///
/// Every position the app hands the control is an offset into that buffer - the caret, a
/// bookmark, a heading jump, the document-absolute translation in `text_window` - so a
/// character that vanishes on the way in shifts everything after it by one and quietly breaks
/// all of them. Sweeping the entire BMP through [`build_rtf`] + `EM_STREAMIN` and comparing
/// `RichEdit`'s own reported buffer length found exactly one discarded range, the tail of the
/// Unicode Specials block: the interlinear annotation marks, the object replacement character,
/// the replacement character, and the two noncharacters. Nothing else in the BMP is dropped,
/// and no astral character is.
///
/// A space is what `wxTextCtrl::SetValue` - the non-RTF path the caller falls back to - already
/// substitutes for these, so both paths leave identical text in the control.
///
/// U+FFFD is the one that turns up in practice: it is what every decoder emits for malformed
/// input, so any document that discusses or demonstrates broken encodings is littered with
/// them. Before this, one such character anywhere in a window made the round-trip check fail on
/// every single load of that window and fall back to issuing one `SetStyle` call per formatting
/// span - seconds rather than milliseconds.
#[must_use]
pub fn sanitize_for_rich_edit(content: &str) -> Cow<'_, str> {
	if content.chars().any(is_discarded_by_rich_edit) {
		Cow::Owned(content.replace(is_discarded_by_rich_edit, " "))
	} else {
		Cow::Borrowed(content)
	}
}

/// U+FFF9..=U+FFFF - see [`sanitize_for_rich_edit`] for how this range was established.
const fn is_discarded_by_rich_edit(ch: char) -> bool {
	matches!(ch, '\u{FFF9}'..='\u{FFFF}')
}

pub struct RtfFontInfo {
	pub face_name: String,
	/// Same units as `wxdragon::widgets::font::Font::get_point_size()`.
	pub point_size: i32,
}

#[derive(Default, Clone, Copy, PartialEq)]
struct ActiveStyle {
	bold: bool,
	italic: bool,
	underline: bool,
}

/// `RichEdit` maps these names, and only these, to its built-in heading styles, which is what a
/// screen reader reading the control through the Text Object Model sees. Declaring a style
/// changes nothing on screen: the stylesheet entries carry no formatting.
const STYLESHEET: &str = "{\\stylesheet{\\s0 Normal;}{\\s1 heading 1;}{\\s2 heading 2;}{\\s3 heading 3;}{\\s4 heading 4;}{\\s5 heading 5;}{\\s6 heading 6;}}\n";

/// A list item for which `RichEdit` draws no bullet or number, as its own RTF writer encodes one:
/// Paperback's bullets and numbers are already part of the text.
const LIST_ITEM_WITHOUT_MARKER: &str = "{\\*\\pn\\pnlvlcont\\pnindent0{\\pntxtb\\'B7}}";

/// `segments` must be the already-merged, non-overlapping, start-sorted output of
/// `merge_formatting_markers`, and `paragraphs` likewise sorted and non-overlapping. Both are
/// walked with a single forward cursor, so overlapping entries would silently shadow one another
/// instead of combining.
#[must_use]
pub fn build_rtf(
	content: &str,
	segments: &[FormatSegment],
	paragraphs: &[ParagraphFormat],
	font: &RtfFontInfo,
) -> String {
	let mut out = String::with_capacity(content.len() + segments.len() * 12 + 256);
	out.push_str("{\\rtf1\\ansi\\ansicpg1252\\deff0{\\fonttbl{\\f0 ");
	escape_font_name(&font.face_name, &mut out);
	out.push_str(";}}\n");
	out.push_str(STYLESHEET);
	out.push_str("\\uc1\\pard\\f0\\fs");
	// RTF font size is in half-points.
	out.push_str(&(font.point_size.max(1) * 2).to_string());
	out.push_str("\\b0\\i0\\ulnone\n");
	let mut seg_idx = 0;
	let mut current = ActiveStyle::default();
	let mut paragraph_idx = 0;
	// Paragraph properties carry over each `\par` until the next `\pard`, so they are only
	// written where they change.
	let mut current_paragraph = (None, false);
	let mut at_paragraph_start = true;
	let mut position: i64 = 0;
	for ch in content.chars() {
		if at_paragraph_start {
			while paragraph_idx < paragraphs.len() && paragraphs[paragraph_idx].end <= position {
				paragraph_idx += 1;
			}
			let paragraph = paragraphs
				.get(paragraph_idx)
				.filter(|p| p.start <= position)
				.map_or((None, false), |p| (p.heading_level, p.list_item));
			if paragraph != current_paragraph {
				let (heading_level, list_item) = paragraph;
				out.push_str("\\pard");
				if let Some(level) = heading_level {
					let _ = write!(out, "\\s{level}");
				}
				if list_item {
					out.push_str(LIST_ITEM_WITHOUT_MARKER);
				} else {
					out.push(' ');
				}
				current_paragraph = paragraph;
			}
		}
		at_paragraph_start = ch == '\n';
		while seg_idx < segments.len() && segments[seg_idx].end <= position {
			seg_idx += 1;
		}
		let active = segments.get(seg_idx).filter(|s| s.start <= position && position < s.end);
		let style = active.map_or(ActiveStyle::default(), |s| ActiveStyle {
			bold: s.bold,
			italic: s.italic,
			underline: s.underline,
		});
		if style != current {
			if style.bold != current.bold {
				out.push_str(if style.bold { "\\b" } else { "\\b0" });
			}
			if style.italic != current.italic {
				out.push_str(if style.italic { "\\i" } else { "\\i0" });
			}
			if style.underline != current.underline {
				out.push_str(if style.underline { "\\ul" } else { "\\ulnone" });
			}
			out.push(' ');
			current = style;
		}
		append_escaped_char(ch, &mut out);
		position += i64::try_from(ch_width(ch)).unwrap_or(1);
	}
	out.push('}');
	out
}

/// [`build_rtf`] for an RTF stream that replaces everything the control holds, as
/// `stream_rtf_into_ctrl` does; text appended after existing content uses [`build_rtf`] itself.
///
/// `RichEdit` takes the last paragraph mark of a replacing stream as the end of its story and
/// stores nothing for it. A window ending in one would then load a display unit short, and text
/// appended to it later would run into its last paragraph. One more paragraph mark keeps it.
#[must_use]
pub fn build_rtf_for_whole_control(
	content: &str,
	segments: &[FormatSegment],
	paragraphs: &[ParagraphFormat],
	font: &RtfFontInfo,
) -> String {
	let mut rtf = build_rtf(content, segments, paragraphs, font);
	if content.ends_with('\n') {
		// Before the closing brace.
		rtf.insert_str(rtf.len() - 1, "\\par");
	}
	rtf
}

fn escape_font_name(name: &str, out: &mut String) {
	for ch in name.chars() {
		match ch {
			'\\' => out.push_str("\\\\"),
			'{' => out.push_str("\\{"),
			'}' => out.push_str("\\}"),
			';' => out.push(','),
			c => out.push(c),
		}
	}
}

fn append_escaped_char(ch: char, out: &mut String) {
	match ch {
		'\\' => out.push_str("\\\\"),
		'{' => out.push_str("\\{"),
		'}' => out.push_str("\\}"),
		// A lone \r (or the \r of a \r\n pair) contributes no visible glyph;
		// the \n that follows (or stands alone) emits the paragraph break.
		'\r' => {}
		'\n' => out.push_str("\\par\n"),
		'\t' => out.push_str("\\tab "),
		c if (0x20..=0x7e).contains(&(c as u32)) => out.push(c),
		c => {
			let mut buf = [0u16; 2];
			for unit in c.encode_utf16(&mut buf) {
				// RTF \u takes a signed 16-bit value; surrogate halves (and any
				// unit >= 0x8000) must be encoded as negative.
				let signed = if *unit > 0x7fff { i32::from(*unit) - 0x1_0000 } else { i32::from(*unit) };
				// Every \u escape must be followed by exactly one fallback
				// character for readers that don't understand \u (\uc1 in the
				// header declares that count as 1). Writing it as \'3f (hex for
				// '?') rather than a literal '?' keeps it unambiguously delimited
				// from adjacent plain text for any hex-escape-aware RTF reader.
				let _ = write!(out, "\\u{signed}\\'3f");
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use std::{env, fs, process};

	use paperback_core::{
		document::{Document, ParserContext},
		parser::{Parser, rtf::RtfParser},
		util::text::display_len,
	};

	use super::*;

	fn font() -> RtfFontInfo {
		RtfFontInfo { face_name: "Segoe UI".to_string(), point_size: 10 }
	}

	/// Parses `rtf` with paperback-core's own `RtfParser`, through a temporary file named after `test`.
	fn parse_back(rtf: &str, test: &str) -> Document {
		let path = env::temp_dir().join(format!("{test}_{}.rtf", process::id()));
		fs::write(&path, rtf).expect("write temp RTF file");
		let parse_result = RtfParser.parse(&ParserContext::new(path.to_string_lossy().into_owned()));
		let _ = fs::remove_file(&path);
		parse_result.expect("generated RTF should parse back cleanly")
	}

	fn seg(start: i64, end: i64, bold: bool, italic: bool, underline: bool) -> FormatSegment {
		FormatSegment { start, end, bold, italic, underline }
	}

	#[test]
	fn sanitize_borrows_when_there_is_nothing_to_replace() {
		let content = "ordinary text with é, ←, 😀 and a BOM \u{FEFF}";
		assert!(matches!(sanitize_for_rich_edit(content), Cow::Borrowed(_)));
	}

	#[test]
	fn sanitize_replaces_the_characters_rich_edit_discards() {
		// U+FFFD is the one that matters in practice - see `sanitize_for_rich_edit`.
		let content = "Before: r\u{FFFD}sum\u{FFFD}";
		assert_eq!(sanitize_for_rich_edit(content), "Before: r sum ");
	}

	#[test]
	fn sanitize_is_width_preserving_across_the_whole_discarded_range() {
		for cp in 0xFFF9..=0xFFFFu32 {
			let ch = char::from_u32(cp).expect("valid scalar value");
			let content = format!("a{ch}b");
			let sanitized = sanitize_for_rich_edit(&content);
			assert_eq!(sanitized, "a b", "U+{cp:04X} should become a space");
			assert_eq!(display_len(&sanitized), display_len(&content), "U+{cp:04X} changed width");
		}
	}

	#[test]
	fn sanitize_leaves_the_neighbouring_specials_alone() {
		// The sweep that established the range found these stored faithfully; replacing them
		// would be a silent, pointless downgrade of the text.
		let content = "a\u{FFF8}b\u{FFF0}c";
		assert!(matches!(sanitize_for_rich_edit(content), Cow::Borrowed(_)));
	}

	#[test]
	fn plain_text_has_no_style_toggles() {
		let rtf = build_rtf("hello", &[], &[], &font());
		assert!(!rtf.contains("\\b "));
		assert!(!rtf.contains("\\i "));
		assert!(rtf.contains("hello"));
		assert!(rtf.starts_with("{\\rtf1"));
		assert!(rtf.ends_with('}'));
	}

	#[test]
	fn escapes_backslash_and_braces() {
		let rtf = build_rtf("a\\b{c}d", &[], &[], &font());
		assert!(rtf.contains("a\\\\b\\{c\\}d"));
	}

	#[test]
	fn newline_becomes_par_and_tab_becomes_tab_control_word() {
		let rtf = build_rtf("a\nb\tc", &[], &[], &font());
		assert!(rtf.contains("a\\par\nb\\tab c"));
	}

	#[test]
	fn crlf_emits_a_single_par() {
		let rtf = build_rtf("a\r\nb", &[], &[], &font());
		assert!(rtf.contains("a\\par\nb"));
		assert!(!rtf.contains("\\par\n\\par"));
	}

	#[test]
	fn bmp_non_ascii_char_is_unicode_escaped() {
		let rtf = build_rtf("caf\u{e9}", &[], &[], &font());
		// 'é' is U+00E9, fits in one UTF-16 unit, well under 0x8000.
		assert!(rtf.contains("\\u233\\'3f"));
	}

	#[test]
	fn astral_char_emits_two_surrogate_escapes() {
		// U+1F600 GRINNING FACE, outside the BMP: encodes as a UTF-16 surrogate pair.
		let rtf = build_rtf("\u{1f600}", &[], &[], &font());
		assert!(rtf.contains("\\u-10179\\'3f\\u-8704\\'3f"));
	}

	#[test]
	fn single_bold_segment_toggles_on_and_off() {
		let rtf = build_rtf("plain bold plain", &[seg(6, 10, true, false, false)], &[], &font());
		let bold_on = rtf.find("\\b ").unwrap();
		let bold_off = rtf.find("\\b0 ").unwrap();
		assert!(bold_on < bold_off);
		assert!(rtf.contains("plain "));
		assert!(rtf.contains("bold"));
	}

	#[test]
	fn adjacent_segments_with_different_combined_styles_both_apply() {
		// Mirrors what merge_formatting_markers would emit for a bold span
		// overlapped by an italic span from position 5 onward: two adjacent,
		// non-overlapping runs, the second carrying both styles. Bold only
		// needs to toggle on once since it stays active across both runs;
		// only italic toggles at the second boundary.
		let text = "bold italic both";
		let segments = [seg(0, 5, true, false, false), seg(5, 17, true, true, false)];
		let rtf = build_rtf(text, &segments, &[], &font());
		let bold_on = rtf.find("\\b ").unwrap();
		let italic_on = rtf.find("\\i ").unwrap();
		assert!(bold_on < italic_on);
		// Bold never closes because it's active through the end of the text.
		assert!(!rtf.contains("\\b0 "));
	}

	#[test]
	fn rtf_for_the_whole_control_writes_one_more_paragraph_mark_after_a_trailing_one() {
		let rtf = build_rtf_for_whole_control("Intro\n", &[], &[], &font());
		assert!(rtf.ends_with("Intro\\par\n\\par}"));
	}

	#[test]
	fn rtf_for_the_whole_control_adds_nothing_to_text_without_a_trailing_paragraph_mark() {
		assert_eq!(build_rtf_for_whole_control("Intro", &[], &[], &font()), build_rtf("Intro", &[], &[], &font()));
	}

	#[test]
	fn no_formatting_segments_means_no_style_changes_emitted() {
		let rtf = build_rtf("nothing special", &[], &[], &font());
		assert_eq!(rtf.matches("\\b").count(), 1); // just the header's \b0
		assert_eq!(rtf.matches("\\i").count(), 1); // just the header's \i0
	}

	fn paragraph(start: i64, end: i64, heading_level: Option<u8>, list_item: bool) -> ParagraphFormat {
		ParagraphFormat { start, end, heading_level, list_item }
	}

	/// Only the built-in style names make `RichEdit` report a paragraph as a heading.
	#[test]
	fn stylesheet_names_the_built_in_heading_styles() {
		let rtf = build_rtf("x", &[], &[], &font());
		assert!(rtf.contains("{\\stylesheet{\\s0 Normal;}"));
		for level in 1..=6 {
			assert!(rtf.contains(&format!("{{\\s{level} heading {level};}}")), "heading {level}");
		}
	}

	#[test]
	fn heading_paragraph_gets_its_style_and_the_next_paragraph_resets() {
		let rtf = build_rtf("Intro\nTitle\nBody", &[], &[paragraph(6, 12, Some(2), false)], &font());
		assert!(rtf.contains("Intro\\par\n\\pard\\s2 Title\\par\n\\pard Body"));
	}

	#[test]
	fn list_run_emits_the_list_group_once_and_resets_after() {
		let rtf = build_rtf("Intro\n• a\n• b\nAfter", &[], &[paragraph(6, 14, None, true)], &font());
		assert!(rtf.contains(
			"Intro\\par\n\\pard{\\*\\pn\\pnlvlcont\\pnindent0{\\pntxtb\\'B7}}\\u8226\\'3f a\\par\n\\u8226\\'3f b\\par\n\\pard After"
		));
		assert_eq!(rtf.matches("\\pnlvlcont").count(), 1);
	}

	#[test]
	fn paragraph_formats_and_character_toggles_combine() {
		let rtf = build_rtf(
			"Bold title\nBody",
			&[seg(0, 4, true, false, false)],
			&[paragraph(0, 11, Some(1), false)],
			&font(),
		);
		assert!(rtf.contains("\\pard\\s1 \\b Bold\\b0  title\\par\n\\pard Body"));
	}

	/// Paragraph formatting must never add or drop text: every offset into the control depends on it.
	#[test]
	fn paragraph_formats_round_trip_through_paperback_cores_own_rtf_parser() {
		let text = "Title\n• a\n• b\nBody";
		let paragraphs = [paragraph(0, 6, Some(1), false), paragraph(6, 14, None, true)];
		let rtf = build_rtf(text, &[], &paragraphs, &font());
		assert_eq!(parse_back(&rtf, "rtf_paragraph_roundtrip_test").buffer.content, text);
	}

	/// Independent black-box check: run generated RTF through paperback-core's
	/// own `RtfParser` (used for *reading* .rtf files) and confirm the plain
	/// text and bold/italic/underline spans it extracts match what went in.
	/// This doesn't exercise the real fast path (that's native `RichEdit`, not
	/// this crate's parser), but it does catch escaping bugs independently of
	/// hand-checking the raw RTF string.
	#[test]
	fn round_trips_through_paperback_cores_own_rtf_parser() {
		use paperback_core::document::MarkerType;
		let text = "plain bold plain italic plain underline plain both plain";
		let idx = |needle: &str| i64::try_from(text.find(needle).unwrap()).unwrap();
		let seg_for = |needle: &str, bold: bool, italic: bool, underline: bool| {
			let start = idx(needle);
			let end = start + i64::try_from(needle.chars().count()).unwrap();
			seg(start, end, bold, italic, underline)
		};
		let segments = [
			seg_for("bold", true, false, false),
			seg_for("italic", false, true, false),
			seg_for("underline", false, false, true),
			seg_for("both", true, true, false),
		];
		let doc = parse_back(&build_rtf(text, &segments, &[], &font()), "rtf_write_roundtrip_test");
		assert_eq!(doc.buffer.content, text);
		let span_text_for = |mtype: MarkerType| {
			doc.buffer
				.markers
				.iter()
				.filter(|m| m.mtype == mtype)
				.map(|m| doc.buffer.content.chars().skip(m.position).take(m.length).collect::<String>())
				.collect::<Vec<_>>()
		};
		assert_eq!(span_text_for(MarkerType::Bold), vec!["bold".to_string(), "both".to_string()]);
		assert_eq!(span_text_for(MarkerType::Italic), vec!["italic".to_string(), "both".to_string()]);
		assert_eq!(span_text_for(MarkerType::Underline), vec!["underline".to_string()]);
	}
}
