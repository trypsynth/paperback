use std::fs;

use anyhow::{Context, Result};
use math_core::{LatexToMathML, MathCoreConfig, MathDisplay};
use pulldown_cmark::{Event, Options, Parser as MarkdownParserImpl, Tag, TagEnd, html::push_html};

use crate::{
	document::{Document, DocumentBuffer, ParserContext},
	parser::{
		Parser, add_converter_markers,
		convert::html_to_text::{HtmlSourceMode, HtmlToText},
		util::{path::extract_title_from_path, toc::build_toc_from_headings},
	},
	t,
	util::encoding::convert_to_utf8,
};

// Rendering and source lookup must use the same grammar to keep block anchors in sync.
const MARKDOWN_OPTIONS: Options = Options::ENABLE_TABLES.union(Options::ENABLE_MATH);

/// Converts Markdown to HTML with an empty `<span id="pb-block-N"></span>` before each block.
///
/// The anchors produce no text but give every block a stable id, so a position
/// in the converted text can be mapped back to a `#fragment` when the document
/// is shown in a web view.
#[must_use]
pub fn markdown_to_html(markdown_text: &str) -> String {
	let parser = MarkdownParserImpl::new_ext(markdown_text, MARKDOWN_OPTIONS).into_offset_iter();
	// Created per document so that equations and similar environments are numbered from (1) in each one.
	let mut math = LatexToMathML::new(MathCoreConfig { xml_namespace: true, annotation: true, ..Default::default() })
		.expect("math conversion configuration is valid");
	let mut block_counter = 0usize;
	let mut image_depth = 0usize;
	let mut events = Vec::new();
	for (event, range) in parser {
		match &event {
			Event::Start(Tag::Paragraph | Tag::Heading { .. } | Tag::Item | Tag::BlockQuote(_) | Tag::CodeBlock(_)) => {
				block_counter += 1;
				events.push(Event::Html(format!("<span id=\"pb-block-{block_counter}\"></span>").into()));
			}
			Event::Start(Tag::Image { .. }) => image_depth += 1,
			Event::End(TagEnd::Image) => image_depth -= 1,
			_ => {}
		}
		let (latex, display, delimiter) = match &event {
			// Inside an image, emitted text goes into its plain-text alt attribute, so math stays as TeX.
			Event::InlineMath(latex) if image_depth == 0 => (latex, MathDisplay::Inline, "$"),
			Event::DisplayMath(latex) if image_depth == 0 => (latex, MathDisplay::Block, "$$"),
			_ => {
				events.push(event);
				continue;
			}
		};
		// math-core turns blank TeX into an empty `<mrow>`, which MathCAT reads as a backslash.
		if latex.trim().is_empty() {
			continue;
		}
		// Rebuilt from the TeX rather than sliced from the source, because the source range of a multi-line formula
		// includes container prefixes on its later lines, such as `> ` in a block quote.
		let source_text = Event::Text(format!("{delimiter}{latex}{delimiter}").into());
		// pulldown-cmark ends inline math at any `$` preceded by a non-space, so in "Tickets cost $10-$20 each." it
		// interprets "$10-$" as math. Following Pandoc, we don't emit MathML when the closing `$` is followed by a
		// digit. The span can't be re-parsed, so any Markdown formatting inside it (very unlikely) is shown literally.
		//
		// This still misreads a `$` that directly follows a non-space character and isn't followed by a digit, as in
		// the shell's `$PATH:$HOME` or PHP's `$a.$b`. Markdown is ambiguous here: without knowing whether the writer
		// uses dollars for math, there is no way to tell which reading is meant.
		let closes_before_digit =
			display == MathDisplay::Inline && markdown_text[range.end..].starts_with(|c: char| c.is_ascii_digit());
		if closes_before_digit {
			events.push(source_text);
			continue;
		}
		match math.convert_with_global_state(latex, display) {
			Ok(converted) => events.push(Event::InlineHtml(converted.mathml.into())),
			Err(error) => {
				tracing::debug!(%error, "could not convert Markdown formula; preserving its source");
				events.push(source_text);
			}
		}
	}
	let mut html_content = String::new();
	push_html(&mut html_content, events.into_iter());
	html_content
}

/// Returns the byte offset in `markdown_text` where the 1-based `block_index`
/// block begins, using the same block numbering as [`markdown_to_html`].
///
/// This maps a `pb-block-N` anchor (recorded in `id_positions`) back to its
/// location in the original Markdown source.
#[must_use]
pub fn block_source_offset(markdown_text: &str, block_index: usize) -> Option<usize> {
	let parser = MarkdownParserImpl::new_ext(markdown_text, MARKDOWN_OPTIONS).into_offset_iter();
	let mut block_counter = 0usize;
	for (event, range) in parser {
		if matches!(
			event,
			Event::Start(Tag::Paragraph | Tag::Heading { .. } | Tag::Item | Tag::BlockQuote(_) | Tag::CodeBlock(_))
		) {
			block_counter += 1;
			if block_counter == block_index {
				return Some(range.start);
			}
		}
	}
	None
}

pub struct MarkdownParser;

impl Parser for MarkdownParser {
	fn parse(&self, context: &ParserContext) -> Result<Document> {
		tracing::debug!(path = %context.file_path, "parsing markdown file");
		let bytes = fs::read(&context.file_path)
			.with_context(|| format!("Failed to open Markdown file '{}'", context.file_path))?;
		let markdown_content = convert_to_utf8(&bytes);
		let html_content = markdown_to_html(&markdown_content);
		let mut converter = HtmlToText::with_render_tables_inline(context.render_tables_inline);
		if !converter.convert(&html_content, HtmlSourceMode::Markdown) {
			// currently unreachable, HtmlToText::convert never returns false today
			tracing::warn!(path = %context.file_path, "failed to convert markdown to text");
			// TRANSLATORS: Error shown when a Markdown file fails to convert to plain text; {} is the file path
			anyhow::bail!(t("Failed to convert Markdown to text: {}").replace("{}", &context.file_path));
		}
		let title = extract_title_from_path(&context.file_path);
		let text = converter.get_text();
		let mut buffer = DocumentBuffer::with_content(text);
		let id_positions = converter.get_id_positions().clone();
		add_converter_markers(&mut buffer, &converter, 0);
		let toc_items = build_toc_from_headings(converter.get_headings());
		let mut doc = Document::new().with_title(title);
		doc.set_buffer(buffer);
		doc.toc_items = toc_items;
		doc.id_positions = id_positions;
		tracing::debug!(path = %context.file_path, chars = doc.buffer.content.chars().count(), "parsed markdown file");
		Ok(doc)
	}
}

#[cfg(test)]
mod tests {
	use rstest::rstest;
	use scraper::{Html, Selector};

	use super::*;
	use crate::{
		document::MarkerType,
		util::{test_support::TempDir, text::display_len},
	};

	fn parse_markdown(source: &str) -> Document {
		let dir = TempDir::new("markdown-parser");
		let path = dir.write_str("formulas.md", source);
		MarkdownParser.parse(&ParserContext::new(path)).expect("parse Markdown document")
	}

	#[rstest]
	#[case::inline("$x^2$", "😀 Before ", "x^2", " after.")]
	#[case::display("$$x^2$$", "😀 Before\n", "x^2", "\nafter.")]
	#[case::fraction(r"$\frac{a}{b}$", "😀 Before ", "a/b", " after.")]
	fn math_reaches_the_reading_buffer_and_formula_markers(
		#[case] source: &str,
		#[case] prefix: &str,
		#[case] formula_text: &str,
		#[case] suffix: &str,
	) {
		let doc = parse_markdown(&format!("😀 Before {source} after."));
		assert_eq!(doc.buffer.content, format!("{prefix}{formula_text}{suffix}"));
		let formulas: Vec<_> = doc.buffer.markers.iter().filter(|marker| marker.mtype == MarkerType::Formula).collect();
		assert_eq!(formulas.len(), 1);
		let formula = formulas[0];
		assert_eq!(formula.text, formula_text);
		assert_eq!(formula.position, display_len(prefix));
		assert_eq!(formula.length, display_len(formula_text));
		let mathml = roxmltree::Document::parse(&formula.reference).expect("valid MathML reference");
		assert_eq!(mathml.root_element().tag_name().namespace(), Some("http://www.w3.org/1998/Math/MathML"));
	}

	#[test]
	fn preserves_tex_in_mathml_annotations() {
		let html = Html::parse_fragment(&markdown_to_html(r"$x < y$ and $\sqrt{x}$"));
		let selector = Selector::parse("math annotation[encoding='application/x-tex']").unwrap();
		let annotations: Vec<_> = html.select(&selector).map(|node| node.text().collect::<String>()).collect();
		assert_eq!(annotations, ["x < y", r"\sqrt{x}"]);
	}

	#[test]
	fn numbered_environments_share_state_only_within_a_document() {
		let source = "$$\\begin{equation}x=1\\end{equation}$$\n\n$$\\begin{align}y&=2\\end{align}$$";
		let selector = Selector::parse("math mtext").unwrap();
		for _ in 0..2 {
			let html = Html::parse_fragment(&markdown_to_html(source));
			let numbers: Vec<_> = html.select(&selector).map(|node| node.text().collect::<String>()).collect();
			assert_eq!(numbers, ["(1)", "(2)"]);
		}
	}

	#[rstest]
	#[case::inline_code("`$x^2$`", "$x^2$")]
	#[case::fenced_code("```tex\n$x^2$\n$$y^2$$\n```", "$x^2$\n$$y^2$$")]
	#[case::escaped(r"\$x^2\$", "$x^2$")]
	#[case::currency("Prices: $5.00 and $10.00.", "Prices: $5.00 and $10.00.")]
	#[case::unclosed("Unclosed $x^2", "Unclosed $x^2")]
	#[case::price_range("Tickets cost $10-$20 each.", "Tickets cost $10-$20 each.")]
	#[case::price_alternatives("Prices: $5/$10 per unit.", "Prices: $5/$10 per unit.")]
	#[case::image_alt("![Plot of $x^2$ curve](a.png)", "[Image: Plot of $x^2$ curve]")]
	fn literal_dollars_remain_text(#[case] source: &str, #[case] expected: &str) {
		let doc = parse_markdown(source);
		assert_eq!(doc.buffer.content, expected);
		assert!(!doc.buffer.markers.iter().any(|marker| marker.mtype == MarkerType::Formula));
	}

	#[rstest]
	#[case::unsupported(r"$\unknown{x}$")]
	#[case::malformed(r"$\frac{x}$")]
	#[case::html_in_unsupported_formula(r"$\unknown{<img src=x>}$")]
	fn failed_formulas_preserve_source_and_later_formulas(#[case] source: &str) {
		let markdown = format!("Before {source} then $x^2$.");
		let doc = parse_markdown(&markdown);
		assert_eq!(doc.buffer.content, format!("Before {source} then x^2."));
		let formulas: Vec<_> = doc.buffer.markers.iter().filter(|marker| marker.mtype == MarkerType::Formula).collect();
		assert_eq!(formulas.len(), 1);
		assert_eq!(formulas[0].text, "x^2");
		assert_eq!(formulas[0].position, display_len(&format!("Before {source} then ")));
		let html = markdown_to_html(&markdown);
		assert!(!html.contains("<img"));
	}

	#[test]
	fn headings_name_formulas_as_the_reading_buffer_does() {
		let doc = parse_markdown("## Proof of $x^2$\n\nBody.");
		let heading = doc.buffer.markers.iter().find(|marker| marker.mtype == MarkerType::Heading2).unwrap();
		assert_eq!(doc.buffer.content, "Proof of x^2\nBody.");
		assert_eq!((heading.text.as_str(), doc.toc_items[0].name.as_str()), ("Proof of x^2", "Proof of x^2"));
	}

	#[test]
	fn table_cells_read_formulas_once() {
		let doc = parse_markdown("| Proof of $x^2$ |\n| --- |");
		assert_eq!(doc.buffer.content, "Proof of x^2");
	}

	#[test]
	fn failed_formulas_in_block_quotes_omit_quote_markers() {
		let doc = parse_markdown("> $$\n> \\frac{x}\n> $$");
		assert_eq!(doc.buffer.content, "$$ \\frac{x} $$");
	}

	#[test]
	fn blank_display_math_produces_nothing() {
		let doc = parse_markdown("Before\n\n$$\n$$\n\nAfter.");
		assert_eq!(doc.buffer.content, "Before\nAfter.");
		assert!(!doc.buffer.markers.iter().any(|marker| marker.mtype == MarkerType::Formula));
	}

	#[test]
	fn source_anchors_point_past_multiline_math() {
		let source = "$x^2$\n\n$$\n\\begin{aligned}\nx &= 1\\\\\ny &= 2\n\\end{aligned}\n$$\n\nAfter.";
		let doc = parse_markdown(source);
		assert_eq!(block_source_offset(source, 3), source.find("After."));
		assert_eq!(doc.id_positions["pb-block-3"], doc.buffer.content.find("After.").unwrap());
		assert_eq!(block_source_offset(source, 4), None);
	}

	#[test]
	fn markdown_to_html_injects_block_anchors() {
		let html = markdown_to_html("# Title\n\nFirst paragraph.\n\nSecond paragraph.\n");
		let anchor_2 = html.find(r#"<span id="pb-block-2"></span>"#).expect("second anchor present");
		let anchor_3 = html.find(r#"<span id="pb-block-3"></span>"#).expect("third anchor present");
		let first_para = html.find("<p>First paragraph.</p>").expect("first paragraph present");
		let second_para = html.find("<p>Second paragraph.</p>").expect("second paragraph present");
		assert!(html.contains(r#"<span id="pb-block-1"></span>"#), "got: {html}");
		assert!(anchor_2 < first_para && first_para < anchor_3 && anchor_3 < second_para, "got: {html}");
	}

	#[test]
	fn block_source_offset_points_at_block_start_in_source() {
		let source = "# Title\n\nFirst paragraph.\n\nSecond paragraph.\n";
		// Block 1 = heading, 2 = first paragraph, 3 = second paragraph.
		assert_eq!(block_source_offset(source, 1), Some(source.find("# Title").unwrap()));
		assert_eq!(block_source_offset(source, 2), Some(source.find("First paragraph.").unwrap()));
		assert_eq!(block_source_offset(source, 3), Some(source.find("Second paragraph.").unwrap()));
		assert_eq!(block_source_offset(source, 99), None);
	}

	#[test]
	fn markdown_block_anchors_reach_id_positions_without_changing_text() {
		let source = "# Title\n\nFirst paragraph.\n\nSecond paragraph.\n";
		let html = markdown_to_html(source);
		let mut converter = HtmlToText::new();
		assert!(converter.convert(&html, HtmlSourceMode::Markdown));
		let text = converter.get_text();
		let ids = converter.get_id_positions();
		assert_eq!(ids.get("pb-block-1"), Some(&text.find("Title").unwrap()), "ids: {ids:?} text: {text:?}");
		assert_eq!(ids.get("pb-block-3"), Some(&text.find("Second").unwrap()), "ids: {ids:?} text: {text:?}");
		assert!(!text.contains("pb-block"), "anchors must not leak into text: {text:?}");
	}
}
