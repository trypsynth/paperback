//! Builds a minimal RTF document from plain text plus a set of non-overlapping
//! bold/italic/underline spans, for the Windows RTF fast path in
//! `document_manager`. See `apply_formatting_markers_to_ctrl` for why this
//! exists: the native RichEdit control backing a `wxTE_RICH2` `TextCtrl`
//! special-cases `WM_SETTEXT` — text starting with `{\rtf` is parsed as RTF in
//! one shot instead of literal text, which is far cheaper than issuing one
//! `SetStyle` call per formatting span on documents with thousands of them.

use paperback_core::util::text::ch_width;

use super::document_manager::FormatSegment;

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

/// `segments` must be the already-merged, non-overlapping, start-sorted output of
/// `merge_formatting_markers` — this walks it with a single forward cursor, so
/// overlapping entries would silently shadow one another instead of combining.
#[must_use]
pub fn build_rtf(content: &str, segments: &[FormatSegment], font: &RtfFontInfo) -> String {
	let mut out = String::with_capacity(content.len() + segments.len() * 12 + 128);
	out.push_str("{\\rtf1\\ansi\\ansicpg1252\\deff0{\\fonttbl{\\f0 ");
	escape_font_name(&font.face_name, &mut out);
	out.push_str(";}}\n\\uc1\\pard\\f0\\fs");
	// RTF font size is in half-points.
	out.push_str(&(font.point_size.max(1) * 2).to_string());
	out.push_str("\\b0\\i0\\ulnone\n");

	let mut seg_idx = 0;
	let mut current = ActiveStyle::default();
	let mut position: i64 = 0;
	for ch in content.chars() {
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
				out.push_str(&format!("\\u{signed}\\'3f"));
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn font() -> RtfFontInfo {
		RtfFontInfo { face_name: "Segoe UI".to_string(), point_size: 10 }
	}

	fn seg(start: i64, end: i64, bold: bool, italic: bool, underline: bool) -> FormatSegment {
		FormatSegment { start, end, bold, italic, underline }
	}

	#[test]
	fn plain_text_has_no_style_toggles() {
		let rtf = build_rtf("hello", &[], &font());
		assert!(!rtf.contains("\\b "));
		assert!(!rtf.contains("\\i "));
		assert!(rtf.contains("hello"));
		assert!(rtf.starts_with("{\\rtf1"));
		assert!(rtf.ends_with('}'));
	}

	#[test]
	fn escapes_backslash_and_braces() {
		let rtf = build_rtf("a\\b{c}d", &[], &font());
		assert!(rtf.contains("a\\\\b\\{c\\}d"));
	}

	#[test]
	fn newline_becomes_par_and_tab_becomes_tab_control_word() {
		let rtf = build_rtf("a\nb\tc", &[], &font());
		assert!(rtf.contains("a\\par\nb\\tab c"));
	}

	#[test]
	fn crlf_emits_a_single_par() {
		let rtf = build_rtf("a\r\nb", &[], &font());
		assert!(rtf.contains("a\\par\nb"));
		assert!(!rtf.contains("\\par\n\\par"));
	}

	#[test]
	fn bmp_non_ascii_char_is_unicode_escaped() {
		let rtf = build_rtf("caf\u{e9}", &[], &font());
		// 'é' is U+00E9, fits in one UTF-16 unit, well under 0x8000.
		assert!(rtf.contains("\\u233\\'3f"));
	}

	#[test]
	fn astral_char_emits_two_surrogate_escapes() {
		// U+1F600 GRINNING FACE, outside the BMP: encodes as a UTF-16 surrogate pair.
		let rtf = build_rtf("\u{1f600}", &[], &font());
		assert!(rtf.contains("\\u-10179\\'3f\\u-8704\\'3f"));
	}

	#[test]
	fn single_bold_segment_toggles_on_and_off() {
		let rtf = build_rtf("plain bold plain", &[seg(6, 10, true, false, false)], &font());
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
		let rtf = build_rtf(text, &segments, &font());
		let bold_on = rtf.find("\\b ").unwrap();
		let italic_on = rtf.find("\\i ").unwrap();
		assert!(bold_on < italic_on);
		// Bold never closes because it's active through the end of the text.
		assert!(!rtf.contains("\\b0 "));
	}

	#[test]
	fn no_formatting_segments_means_no_style_changes_emitted() {
		let rtf = build_rtf("nothing special", &[], &font());
		assert_eq!(rtf.matches("\\b").count(), 1); // just the header's \b0
		assert_eq!(rtf.matches("\\i").count(), 1); // just the header's \i0
	}

	/// Independent black-box check: run generated RTF through paperback-core's
	/// own `RtfParser` (used for *reading* .rtf files) and confirm the plain
	/// text and bold/italic/underline spans it extracts match what went in.
	/// This doesn't exercise the real fast path (that's native RichEdit, not
	/// this crate's parser), but it does catch escaping bugs independently of
	/// hand-checking the raw RTF string.
	#[test]
	fn round_trips_through_paperback_cores_own_rtf_parser() {
		use paperback_core::{
			document::{MarkerType, ParserContext},
			parser::{Parser, rtf::RtfParser},
		};

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
		let rtf = build_rtf(text, &segments, &font());

		let path = std::env::temp_dir().join(format!("rtf_write_roundtrip_test_{}.rtf", std::process::id()));
		std::fs::write(&path, &rtf).expect("write temp RTF file");
		let context = ParserContext::new(path.to_string_lossy().into_owned());
		let parse_result = RtfParser.parse(&context);
		let _ = std::fs::remove_file(&path);
		let doc = parse_result.expect("generated RTF should parse back cleanly");

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
