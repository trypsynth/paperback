use std::collections::HashMap;

use rtf_parser::lexer::Lexer;

use super::*;
use crate::parser::rtf::escapes::normalize_escapes;

#[test]
fn extract_content_maps_quote_unknown_control_words_to_typographic_quotes() {
	let tokens = vec![
		Token::ControlSymbol((ControlWord::Pard, Property::None)),
		Token::ControlSymbol((ControlWord::Unknown(r"\ldblquote"), Property::None)),
		Token::PlainText("ship"),
		Token::ControlSymbol((ControlWord::Unknown(r"\rquote"), Property::None)),
		Token::PlainText("s"),
		Token::ControlSymbol((ControlWord::Unknown(r"\rdblquote"), Property::None)),
		Token::PlainText(" and "),
		Token::ControlSymbol((ControlWord::Unknown(r"\lquote"), Property::None)),
		Token::PlainText("captain"),
		Token::ControlSymbol((ControlWord::Unknown(r"\rquote"), Property::None)),
	];
	let buffer = extract_content_from_tokens(&tokens);
	assert_eq!(buffer.content, "\u{201C}ship\u{2019}s\u{201D} and \u{2018}captain\u{2019}");
}

#[test]
fn extract_content_preserves_line_and_tab_unknown_controls() {
	let rtf = r"{\rtf1\ansi\pard delay.\line \tab next}";
	let normalized = normalize_escapes(rtf, encoding_rs::WINDOWS_1252, &HashMap::new()).replace('\r', "");
	let tokens = Lexer::scan(&normalized).expect("RTF tokenization should succeed");
	let buffer = extract_content_from_tokens(&tokens);
	assert_eq!(buffer.content, "delay.\n\tnext");
}

#[test]
fn extract_content_keeps_text_around_word_style_unicode_escapes() {
	// Word writes curly quotes as \uN followed by a bare `?` fallback. Because the
	// lexer ends a control word at the first space, every character between the
	// digits and that space used to be swallowed along with the escape.
	let rtf = "{\\rtf1\\ansi\\pard\t\n\\u8216?Yes!\\u8217? shrieked Salem Rews, quartermaster of his August \
	           Majesty\\u8217?s First Regiment. \\u8216?Give \\u8217?em hell!\\u8217?\\par}";
	let normalized = normalize_escapes(rtf, encoding_rs::WINDOWS_1252, &HashMap::new()).replace('\r', "");
	let tokens = Lexer::scan(&normalized).expect("RTF tokenization should succeed");
	let buffer = extract_content_from_tokens(&tokens);
	assert_eq!(
		buffer.content,
		"\u{2018}Yes!\u{2019} shrieked Salem Rews, quartermaster of his August Majesty\u{2019}s First Regiment. \
		 \u{2018}Give \u{2019}em hell!\u{2019}"
	);
}

#[test]
fn extract_content_keeps_literal_tab_indentation() {
	// This writer opens each paragraph with a literal tab rather than \tab.
	let rtf = "{\\rtf1\\ansi\\pard first\\par}{\t\nFor Mum and Dad, Couldn\\u8217?t have done it.\\par}";
	let normalized = normalize_escapes(rtf, encoding_rs::WINDOWS_1252, &HashMap::new()).replace('\r', "");
	let tokens = Lexer::scan(&normalized).expect("RTF tokenization should succeed");
	let buffer = extract_content_from_tokens(&tokens);
	assert_eq!(buffer.content, "first\n\tFor Mum and Dad, Couldn\u{2019}t have done it.");
}

#[test]
fn extract_content_maps_page_control_to_marker_and_separator() {
	let rtf = r"{\rtf1\ansi\pard chapter one\page chapter two}";
	let normalized = normalize_escapes(rtf, encoding_rs::WINDOWS_1252, &HashMap::new()).replace('\r', "");
	let tokens = Lexer::scan(&normalized).expect("RTF tokenization should succeed");
	let buffer = extract_content_from_tokens(&tokens);
	assert_eq!(buffer.content, "chapter one chapter two");
	let page_markers: Vec<_> = buffer.markers.iter().filter(|m| m.mtype == MarkerType::PageBreak).collect();
	assert_eq!(page_markers.len(), 2);
	assert_eq!(page_markers[0].position, "chapter one ".chars().count());
	assert_eq!(page_markers[1].position, 0);
}

#[test]
fn extract_content_skips_pict_groups() {
	// A bare {\pict...} group (no leading \*) still carries binary image data as
	// hex text, which must not leak into the document body.
	let rtf = r"{\rtf1\ansi\pard before{\pict\wmetafile8\picw100\pich100 010009000003}after}";
	let normalized = normalize_escapes(rtf, encoding_rs::WINDOWS_1252, &HashMap::new()).replace('\r', "");
	let tokens = Lexer::scan(&normalized).expect("RTF tokenization should succeed");
	let buffer = extract_content_from_tokens(&tokens);
	assert_eq!(buffer.content, "beforeafter");
	assert!(!buffer.content.contains("010009000003"));
}

#[test]
fn extract_content_skips_nested_ignorable_destination_without_leaking_hex() {
	// Word writes shapes as {\*\shppict{\pict{\*\picprop ...}\jpegblip <hex>}}: an
	// ignorable destination (\shppict) wrapping a \pict group that itself contains
	// another ignorable destination (\picprop). The inner \picprop group closing must
	// not end the skip early and let the outer \jpegblip hex data leak as text.
	let rtf = r"{\rtf1\ansi\pard before {\*\shppict{\pict{\*\picprop\shplid1{\sp{\sn shapeType}{\sv 75}}}\picscalex100\jpegblip abc123def456}} after}";
	let normalized = normalize_escapes(rtf, encoding_rs::WINDOWS_1252, &HashMap::new()).replace('\r', "");
	let tokens = Lexer::scan(&normalized).expect("RTF tokenization should succeed");
	let buffer = extract_content_from_tokens(&tokens);
	assert_eq!(buffer.content, "before  after");
	assert!(!buffer.content.contains("abc123def456"));
}

#[test]
fn extract_content_skips_ignorable_destination_groups() {
	let tokens = vec![
		Token::ControlSymbol((ControlWord::Pard, Property::None)),
		Token::PlainText("before"),
		Token::OpeningBracket,
		Token::IgnorableDestination,
		Token::PlainText("504b0304themedata"),
		Token::ClosingBracket,
		Token::PlainText("after"),
	];
	let buffer = extract_content_from_tokens(&tokens);
	assert_eq!(buffer.content, "beforeafter");
	assert!(!buffer.content.contains("504b0304"));
}

#[test]
fn extract_content_handles_libreoffice_unicode_fallback_and_nbsp_symbols() {
	let rtf = r"{\rtf1\ansi\pard AGRAVANTE:\~ Pedro da Silva\par O Ju\u237\'edzo da Vara, pela decis\u227\'e3o e execu\u231\'e7\u227\'e3o contra a 2\u170\'aa executada\par}";
	let normalized = normalize_escapes(rtf, encoding_rs::WINDOWS_1252, &HashMap::new()).replace('\r', "");
	let tokens = Lexer::scan(&normalized).expect("RTF tokenization should succeed");
	let buffer = extract_content_from_tokens(&tokens);
	assert!(buffer.content.contains("AGRAVANTE:"));
	assert!(buffer.content.contains("Pedro da Silva"));
	assert!(buffer.content.contains("Juízo"));
	assert!(buffer.content.contains("decisão"));
	assert!(buffer.content.contains("execução"));
	assert!(buffer.content.contains("2ª executada"));
}

#[test]
fn extract_content_maps_bold_toggle_to_marker() {
	// \b bold \b0 not-bold  → one Bold marker spanning exactly "bold ".
	let tokens = vec![
		Token::ControlSymbol((ControlWord::Pard, Property::None)),
		Token::ControlSymbol((ControlWord::Bold, Property::None)),
		Token::PlainText("bold "),
		Token::ControlSymbol((ControlWord::Bold, Property::Value(0))),
		Token::PlainText("not-bold"),
	];
	let buffer = extract_content_from_tokens(&tokens);
	assert_eq!(buffer.content, "bold not-bold");
	let bold: Vec<_> = buffer.markers.iter().filter(|m| m.mtype == MarkerType::Bold).collect();
	assert_eq!(bold.len(), 1);
	assert_eq!(bold[0].position, 0);
	assert_eq!(bold[0].length, "bold ".chars().count());
}

#[test]
fn extract_content_scopes_nested_group_formatting() {
	// \b bold {\i more} still-bold \b0
	// Bold spans the whole "bold more still-bold " (uninterrupted by the group);
	// Italic spans only "more" (opened and closed within the group).
	let tokens = vec![
		Token::ControlSymbol((ControlWord::Pard, Property::None)),
		Token::ControlSymbol((ControlWord::Bold, Property::None)),
		Token::PlainText("bold "),
		Token::OpeningBracket,
		Token::ControlSymbol((ControlWord::Italic, Property::None)),
		Token::PlainText("more"),
		Token::ClosingBracket,
		Token::PlainText(" still-bold "),
		Token::ControlSymbol((ControlWord::Bold, Property::Value(0))),
	];
	let buffer = extract_content_from_tokens(&tokens);
	assert_eq!(buffer.content, "bold more still-bold");
	let bold: Vec<_> = buffer.markers.iter().filter(|m| m.mtype == MarkerType::Bold).collect();
	assert_eq!(bold.len(), 1);
	assert_eq!(bold[0].position, 0);
	assert_eq!(bold[0].length, "bold more still-bold ".chars().count());
	let italic: Vec<_> = buffer.markers.iter().filter(|m| m.mtype == MarkerType::Italic).collect();
	assert_eq!(italic.len(), 1);
	assert_eq!(italic[0].position, "bold ".chars().count());
	assert_eq!(italic[0].length, "more".chars().count());
}

#[test]
fn extract_content_maps_underline_off_via_ulnone() {
	// \ul under \ulnone plain → one Underline marker spanning "under ".
	let tokens = vec![
		Token::ControlSymbol((ControlWord::Pard, Property::None)),
		Token::ControlSymbol((ControlWord::Underline, Property::None)),
		Token::PlainText("under "),
		Token::ControlSymbol((ControlWord::UnderlineNone, Property::None)),
		Token::PlainText("plain"),
	];
	let buffer = extract_content_from_tokens(&tokens);
	assert_eq!(buffer.content, "under plain");
	let underline: Vec<_> = buffer.markers.iter().filter(|m| m.mtype == MarkerType::Underline).collect();
	assert_eq!(underline.len(), 1);
	assert_eq!(underline[0].position, 0);
	assert_eq!(underline[0].length, "under ".chars().count());
}

#[test]
fn extract_content_reverts_bold_on_group_close() {
	// \b before {\b0 middle} after \b0
	// Two separate Bold markers ("before " and " after "), "middle" in neither.
	let tokens = vec![
		Token::ControlSymbol((ControlWord::Pard, Property::None)),
		Token::ControlSymbol((ControlWord::Bold, Property::None)),
		Token::PlainText("before "),
		Token::OpeningBracket,
		Token::ControlSymbol((ControlWord::Bold, Property::Value(0))),
		Token::PlainText("middle"),
		Token::ClosingBracket,
		Token::PlainText(" after "),
		Token::ControlSymbol((ControlWord::Bold, Property::Value(0))),
	];
	let buffer = extract_content_from_tokens(&tokens);
	assert_eq!(buffer.content, "before middle after");
	let bold: Vec<_> = buffer.markers.iter().filter(|m| m.mtype == MarkerType::Bold).collect();
	assert_eq!(bold.len(), 2, "expected two Bold spans, got {bold:?}");
	// "before "
	assert_eq!(bold[0].position, 0);
	assert_eq!(bold[0].length, "before ".chars().count());
	// " after " starts right after "before middle"
	assert_eq!(bold[1].position, "before middle".chars().count());
	assert_eq!(bold[1].length, " after ".chars().count());
	// "middle" is covered by neither span.
	let middle_start = "before ".chars().count();
	let middle_end = "before middle".chars().count();
	for m in &bold {
		let span_end = m.position + m.length;
		assert!(m.position >= middle_end || span_end <= middle_start, "Bold span {m:?} should not overlap \"middle\"");
	}
}

#[test]
fn extract_content_renders_bold_from_real_rtf_string() {
	// Round-trip through the real lexer to confirm \b / \b0 map to Bold markers.
	let rtf = r"{\rtf1\ansi\pard normal \b bold\b0  normal}";
	let normalized = normalize_escapes(rtf, encoding_rs::WINDOWS_1252, &HashMap::new()).replace('\r', "");
	let tokens = Lexer::scan(&normalized).expect("RTF tokenization should succeed");
	let buffer = extract_content_from_tokens(&tokens);
	assert_eq!(buffer.content, "normal bold normal");
	let bold: Vec<_> = buffer.markers.iter().filter(|m| m.mtype == MarkerType::Bold).collect();
	assert_eq!(bold.len(), 1);
	assert_eq!(bold[0].position, "normal ".chars().count());
	assert_eq!(bold[0].length, "bold".chars().count());
}

/// Round-trip through the real lexer (unlike
/// `extract_content_maps_quote_unknown_control_words_to_typographic_quotes` above, which
/// hand-builds `ControlWord::Unknown` tokens and so can't catch this): rtf_parser 0.4.3
/// promoted `\rquote`/`\lquote`/`\rdblquote`/`\ldblquote`/`\emdash`/`\endash`/`\bullet` from
/// `Unknown(name)` to their own dedicated `ControlWord` variants, which silently fell
/// through the `_ => {}` catch-all until dedicated match arms were added for them.
#[test]
fn extract_content_maps_special_char_control_words_from_real_rtf_string() {
	let rtf = r"{\rtf1\ansi\pard Earth\rquote s \lquote quoted\rquote \ldblquote double\rdblquote em\emdash dash en\endash dash \bullet bullet}";
	let normalized = normalize_escapes(rtf, encoding_rs::WINDOWS_1252, &HashMap::new()).replace('\r', "");
	let tokens = Lexer::scan(&normalized).expect("RTF tokenization should succeed");
	let buffer = extract_content_from_tokens(&tokens);
	assert_eq!(
		buffer.content,
		"Earth\u{2019}s \u{2018}quoted\u{2019}\u{201C}double\u{201D}em\u{2014}dash en\u{2013}dash \u{2022}bullet"
	);
}
