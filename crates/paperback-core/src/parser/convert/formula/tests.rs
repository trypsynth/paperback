use std::collections::HashMap;

use rstest::rstest;
use scraper::{ElementRef, Html, Selector};

use crate::{
	document::DocumentBuffer,
	parser::convert::{
		html_to_text::{HtmlSourceMode, HtmlToText},
		xml_to_text::XmlToText,
	},
	types::{FormulaInfo, HeadingInfo, LinkInfo, ListItemInfo},
	util::text::display_len,
};

#[derive(Clone, Copy, Debug)]
enum Source {
	Html,
	Xml,
}

#[derive(Clone, Copy, Debug)]
enum Tables {
	Inline,
	Placeholder,
}

type Expression = (&'static str, &'static str);

const EQUATION: Expression = ("<mi>x</mi><mo>=</mo><mn>1</mn>", "x = 1");
const FRACTION: Expression = ("<mfrac><mi>a</mi><mi>b</mi></mfrac>", "a/b");
const ROOT: Expression = ("<msqrt><mi>x</mi></msqrt>", "sqrt(x)");
const POWER: Expression = ("<msup><mi>x</mi><mn>2</mn></msup>", "x^2");

const DEFAULT_NAMESPACE: &str = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><msqrt><mi>x</mi></msqrt></math>"#;
const INHERITED_DEFAULT_NAMESPACE: &str =
	r#"<span xmlns="http://www.w3.org/1998/Math/MathML"><math><msqrt><mi>x</mi></msqrt></math></span>"#;
const INHERITED_PREFIX: &str =
	r#"<span xmlns:m="http://www.w3.org/1998/Math/MathML"><m:math><m:msqrt><m:mi>x</m:mi></m:msqrt></m:math></span>"#;
// Those are the ones MathCat can't handle natively.
const MIXED_PREFIXES: &str = r#"<span xmlns:m="http://www.w3.org/1998/Math/MathML" xmlns:q="http://www.w3.org/1998/Math/MathML"><m:math><q:msqrt><mi xmlns="http://www.w3.org/1998/Math/MathML">x</mi></q:msqrt></m:math></span>"#;
const PREFIX_WITH_DIGIT: &str = r#"<span xmlns:m1="http://www.w3.org/1998/Math/MathML"><m1:math><m1:msqrt><m1:mi>x</m1:mi></m1:msqrt></m1:math></span>"#;

const FOREIGN_DEFAULT: &str = r#"<math xmlns="urn:other"><msup><mi>x</mi><mn>2</mn></msup></math>"#;
const INHERITED_FOREIGN: &str = r#"<span xmlns="urn:other"><math><msup><mi>x</mi><mn>2</mn></msup></math></span>"#;
const FOREIGN_PREFIX: &str =
	r#"<span xmlns:m="urn:other"><m:math><m:msup><m:mi>x</m:mi><m:mn>2</m:mn></m:msup></m:math></span>"#;

const SVG_MATH: &str =
	r#"<svg xmlns="http://www.w3.org/2000/svg"><math><msup><mi>x</mi><mn>2</mn></msup></math></svg>"#;
const SVG_MATHML: &str = r#"<svg xmlns="http://www.w3.org/2000/svg"><foreignObject><math xmlns="http://www.w3.org/1998/Math/MathML"><msup><mi>x</mi><mn>2</mn></msup></math></foreignObject></svg>"#;

struct Converted {
	text: String,
	formulas: Vec<FormulaInfo>,
	headings: Vec<HeadingInfo>,
	links: Vec<LinkInfo>,
	list_items: Vec<ListItemInfo>,
	ids: HashMap<String, usize>,
}

fn convert(body: &str, source: Source) -> Converted {
	convert_with_tables(body, source, Tables::Placeholder)
}

fn convert_with_tables(body: &str, source: Source, tables: Tables) -> Converted {
	convert_document(&format!("<html><body>{body}</body></html>"), source, tables)
}

fn convert_document(input: &str, source: Source, tables: Tables) -> Converted {
	let inline = matches!(tables, Tables::Inline);
	match source {
		Source::Xml => {
			let mut converter = XmlToText::with_render_tables_inline(inline);
			assert!(converter.convert(input));
			Converted {
				text: converter.get_text(),
				formulas: converter.get_formulas().to_vec(),
				headings: converter.get_headings().to_vec(),
				links: converter.get_links().to_vec(),
				list_items: converter.get_list_items().to_vec(),
				ids: converter.get_id_positions().clone(),
			}
		}
		Source::Html => {
			let mut converter = HtmlToText::with_render_tables_inline(inline);
			assert!(converter.convert(input, HtmlSourceMode::NativeHtml));
			Converted {
				text: converter.get_text(),
				formulas: converter.get_formulas().to_vec(),
				headings: converter.get_headings().to_vec(),
				links: converter.get_links().to_vec(),
				list_items: converter.get_list_items().to_vec(),
				ids: converter.get_id_positions().clone(),
			}
		}
	}
}

fn wrap(tag: &str, content: &str) -> String {
	match tag {
		"a" => format!(r##"<a href="#target">{content}</a>"##),
		"li" => format!("<ul><li>{content}</li></ul>"),
		"td" => format!("<table><tr><td>{content}</td></tr></table>"),
		_ => format!("<{tag}>{content}</{tag}>"),
	}
}

fn formula_view(formula: &FormulaInfo) -> Html {
	// Formula View loads an HTML document, so XML reparsing would test the wrong consumer.
	Html::parse_document(&format!("<html><body>{}</body></html>", formula.mathml))
}

#[rstest]
#[case::paragraph("p")]
#[case::span("span")]
#[case::emphasis("em")]
#[case::strong("strong")]
#[case::heading_1("h1")]
#[case::heading_2("h2")]
#[case::heading_6("h6")]
#[case::list_item("li")]
#[ignore = "deferred: links need shared rendering for nested content and its markers"]
#[case::deferred_link("a")]
#[ignore = "deferred: table cells flatten nested content to raw text"]
#[case::deferred_table("td")]
fn embedded_formulas_preserve_text_spans_and_anchors(
	#[values(Source::Html, Source::Xml)] source: Source,
	#[case] tag: &str,
) {
	let (expression, expected) = POWER;
	let body =
		wrap(tag, &format!(r#"😀 Before <math id="formula"><mrow id="token">{expression}</mrow></math> after."#));
	let converted = convert_with_tables(&body, source, Tables::Inline);
	let bullet = if tag == "li" { "• " } else { "" };
	assert_eq!(converted.text, format!("{bullet}😀 Before {expected} after."));
	assert_eq!(converted.formulas.len(), 1);
	let formula = &converted.formulas[0];
	assert_eq!(formula.text, expected);
	assert_eq!(formula.offset, display_len(&format!("{bullet}😀 Before ")));
	assert_eq!(formula.length, display_len(expected));
	assert_eq!(converted.ids["formula"], formula.offset);
	assert_eq!(converted.ids["token"], formula.offset);
}

// TODO: merge with embedded_formulas_preserve_text_spans_and_anchors when labels use the rendered content.
#[rstest]
#[ignore = "deferred: heading and list labels concatenate raw descendant text"]
fn formula_labels_match_the_reading_buffer(
	#[values(Source::Html, Source::Xml)] source: Source,
	#[values("h1", "h2", "h6", "li")] tag: &str,
) {
	let converted = convert(&wrap(tag, &format!("Before <math>{}</math> after.", POWER.0)), source);
	let labels: Vec<_> = converted
		.headings
		.iter()
		.map(|heading| heading.text.as_str())
		.chain(converted.list_items.iter().map(|item| item.text.as_str()))
		.collect();
	assert_eq!(labels, ["Before x^2 after."]);
}

#[rstest]
fn formula_expressions_reach_mathcat(
	#[values(EQUATION, FRACTION, ROOT, POWER)] expression: Expression,
	#[values(Source::Html, Source::Xml)] source: Source,
) {
	let (markup, expected) = expression;
	let converted = convert(&format!("<p><math>{markup}</math></p>"), source);
	assert_eq!(converted.text, expected);
	assert_eq!(converted.formulas.len(), 1);
	assert_eq!(converted.formulas[0].text, expected);
}

#[rstest]
#[case::default(None, " after.", "Before x^2 after.", "Before ")]
#[case::inline(Some("inline"), " after.", "Before x^2 after.", "Before ")]
#[case::block(Some("block"), " after.", "Before\nx^2\nafter.", "Before\n")]
#[case::block_at_end(Some("block"), "", "Before\nx^2", "Before\n")]
fn formula_display_controls_line_breaks(
	#[case] display: Option<&str>,
	#[case] suffix: &str,
	#[case] expected: &str,
	#[case] prefix: &str,
	#[values(Source::Html, Source::Xml)] source: Source,
	#[values("p", "span", "h1")] tag: &str,
) {
	let attribute = display.map_or(String::new(), |value| format!(r#" display="{value}""#));
	let body = wrap(tag, &format!(r#"Before <math{attribute} id="formula">{}</math>{suffix}"#, POWER.0));
	let converted = convert(&body, source);
	assert_eq!(converted.text, expected);
	assert_eq!(converted.formulas.len(), 1);
	let formula = &converted.formulas[0];
	assert_eq!(formula.text, "x^2");
	assert_eq!(formula.offset, display_len(prefix));
	assert_eq!(formula.length, display_len("x^2"));
	assert_eq!(converted.ids["formula"], formula.offset);
	let view = formula_view(formula);
	let selector = Selector::parse("body > math").unwrap();
	let math = view.select(&selector).next().unwrap();
	assert_eq!(math.attr("display"), display);
	let power = math.child_elements().next().unwrap();
	assert_eq!(power.value().name(), "msup");
	assert_eq!(power.value().name.ns.as_ref(), "http://www.w3.org/1998/Math/MathML");
	assert_eq!(power.text().collect::<String>(), "x2");
}

#[rstest]
#[case::alternative(r#" alttext="  alternative&#10; text ""#, "<mfrac><mi>raw</mi></mfrac>", Some("alternative text"))]
#[case::text(r#" alttext=" ""#, "<mfrac><mi> raw\n text </mi></mfrac>", Some("raw text"))]
#[case::empty("", "<mfrac/>", None)]
fn invalid_formulas_use_fallbacks_without_breaking_later_formulas(
	#[case] attributes: &str,
	#[case] expression: &str,
	#[case] fallback: Option<&str>,
	#[values(Source::Html, Source::Xml)] source: Source,
) {
	let converted = convert(
		&format!("<p><math{attributes}>{expression}</math></p><p><math><msqrt><mi>y</mi></msqrt></math></p>"),
		source,
	);
	let expected: Vec<_> = fallback.into_iter().chain(["sqrt(y)"]).collect();
	assert_eq!(converted.text, expected.join("\n"));
	assert_eq!(converted.formulas.iter().map(|formula| formula.text.as_str()).collect::<Vec<_>>(), expected);
}

#[rstest]
#[case::default_namespace("", DEFAULT_NAMESPACE)]
#[case::inherited_default("", INHERITED_DEFAULT_NAMESPACE)]
#[case::inherited_prefix("", INHERITED_PREFIX)]
#[case::mixed_prefixes("", MIXED_PREFIXES)]
#[case::prefix_with_digit("", PREFIX_WITH_DIGIT)]
#[case::document_entity(r#"<!DOCTYPE html [<!ENTITY bookvar "x">]>"#, "<math><msqrt><mi>&bookvar;</mi></msqrt></math>")]
fn xml_formula_fragments_work_for_mathcat_and_html(#[case] doctype: &str, #[case] markup: &str) {
	let converted = convert_document(
		&format!("{doctype}<html><body><p>Before {markup} after.</p></body></html>"),
		Source::Xml,
		Tables::Placeholder,
	);
	assert_eq!(converted.text, "Before sqrt(x) after.");
	assert_eq!(converted.formulas.len(), 1);
	let view = formula_view(&converted.formulas[0]);
	let selector = Selector::parse("body > math > msqrt > mi").unwrap();
	let token = view.select(&selector).next().unwrap();
	assert_eq!(token.text().collect::<String>(), "x");
	assert!(
		std::iter::once(token)
			.chain(token.ancestors().take(2).filter_map(ElementRef::wrap))
			.all(|element| { element.value().name.ns.as_ref() == "http://www.w3.org/1998/Math/MathML" })
	);
}

#[rstest]
fn formula_entities_survive_conversion_and_html_view(#[values(Source::Html, Source::Xml)] source: Source) {
	let space = match source {
		Source::Html => "&nbsp;",
		Source::Xml => "&#160;",
	};
	let converted = convert(
		&format!(r#"<math alttext="a{space}&lt;{space}&quot;b&quot;" xml:lang="en"><mtext>a{space}b</mtext></math>"#),
		source,
	);
	assert_eq!(converted.text, r#""a\b""#);
	assert_eq!(converted.formulas.len(), 1);
	let view = formula_view(&converted.formulas[0]);
	let selector = Selector::parse("body > math").unwrap();
	let math = view.select(&selector).next().unwrap();
	assert_eq!(math.attr("alttext"), Some("a\u{00a0}<\u{00a0}\"b\""));
	assert_eq!(math.text().collect::<String>(), "a\u{00a0}b");
	assert!(math.value().attrs.iter().any(|(name, value)| {
		name.ns.as_ref() == "http://www.w3.org/XML/1998/namespace" && name.local.as_ref() == "lang" && value == "en"
	}));
}

// TODO: merge with xml_formula_fragments_work_for_mathcat_and_html when foreign annotations can reach the HTML view.
#[rstest]
#[case::xhtml(
	r#"<annotation-xml encoding="application/xhtml+xml"><h:div xmlns:h="http://www.w3.org/1999/xhtml">annotation</h:div></annotation-xml>"#,
	"div",
	"http://www.w3.org/1999/xhtml",
)]
#[case::svg(
	r#"<annotation-xml encoding="image/svg+xml"><s:svg xmlns:s="http://www.w3.org/2000/svg"><s:text>annotation</s:text></s:svg></annotation-xml>"#,
	"svg",
	"http://www.w3.org/2000/svg",
)]
#[ignore = "deferred: the XML formula adapter omits foreign subtrees"]
fn xml_formula_view_retains_foreign_annotations(#[case] annotation: &str, #[case] tag: &str, #[case] namespace: &str) {
	let converted = convert(&format!("<math><semantics>{}{annotation}</semantics></math>", POWER.0), Source::Xml);
	assert_eq!(converted.text, "x^2");
	assert_eq!(converted.formulas.len(), 1);
	let view = formula_view(&converted.formulas[0]);
	let selector = Selector::parse(&format!("math > semantics > annotation-xml > {tag}")).unwrap();
	let annotation = view.select(&selector).next().expect("foreign annotation retained for the view");
	assert_eq!(annotation.value().name.ns.as_ref(), namespace);
	assert_eq!(annotation.text().collect::<String>(), "annotation");
}

// TODO: merge with xml_formula_fragments_work_for_mathcat_and_html when links and tables use shared XML rendering.
#[rstest]
#[ignore = "deferred: links flatten XML text, and tables reparse source slices as HTML"]
fn xml_prefixes_survive_link_and_table_rendering(#[values("a", "td")] tag: &str) {
	let converted = convert_with_tables(&wrap(tag, INHERITED_PREFIX), Source::Xml, Tables::Inline);
	assert_eq!(converted.text, "sqrt(x)");
	assert_eq!(converted.formulas.len(), 1);
}

#[rstest]
fn xml_elements_in_other_namespaces_are_not_formulas(
	#[values(FOREIGN_DEFAULT, INHERITED_FOREIGN, FOREIGN_PREFIX)] markup: &str,
	#[values("p", "h1")] tag: &str,
) {
	let converted = convert(&wrap(tag, &format!("Before {markup} after.")), Source::Xml);
	assert_eq!(converted.text, "Before x2 after.");
	assert!(converted.formulas.is_empty());
}

#[test]
fn native_html_uses_the_parsed_namespace_instead_of_xmlns_attributes() {
	let converted = convert(FOREIGN_DEFAULT, Source::Html);
	assert_eq!(converted.text, "x^2");
	assert_eq!(converted.formulas.len(), 1);
}

#[rstest]
#[case::svg_namespace(SVG_MATH, "x2", 0)]
#[case::mathml_namespace(SVG_MATHML, "x^2", 1)]
fn svg_formulas_use_their_parsed_namespace(
	#[case] markup: &str,
	#[case] expected: &str,
	#[case] count: usize,
	#[values(Source::Html, Source::Xml)] source: Source,
) {
	let converted = convert(&wrap("p", &format!("Before {markup} after.")), source);
	assert_eq!(converted.text, format!("Before {expected} after."));
	assert_eq!(converted.formulas.len(), count);
}

#[rstest]
#[ignore = "deferred: table cells flatten nested content to raw text"]
fn table_formulas_preserve_text_and_spans(
	#[values(Source::Html, Source::Xml)] source: Source,
	#[values(Tables::Inline, Tables::Placeholder)] tables: Tables,
) {
	let converted = convert_with_tables(
		"<p>Start</p><table><tr><td>  a <math><msup><mi>x</mi><mn>2</mn></msup></math></td><td> <math><msqrt><mi>y</mi></msqrt></math></td></tr><tr><td><math><mfrac><mi>a</mi><mi>b</mi></mfrac></math></td></tr></table><p>After</p>",
		source,
		tables,
	);
	let (expected, count) = match tables {
		Tables::Inline => ("Start\na x^2\tsqrt(y)\na/b\nAfter", 3),
		Tables::Placeholder => ("Start\n[Table]: a x^2 sqrt(y)\nAfter", 2),
	};
	assert_eq!(converted.text, expected);
	assert_eq!(converted.formulas.len(), count);
	let buffer = DocumentBuffer::with_content(converted.text);
	for formula in converted.formulas {
		let start = buffer.byte_index_for_display(formula.offset);
		let end = buffer.byte_index_for_display(formula.offset + formula.length);
		assert_eq!(&buffer.content[start..end], formula.text);
	}
}

// TODO: merge with the link case in embedded_formulas_preserve_text_spans_and_anchors when links retain markers.
#[test]
fn html_link_text_uses_formula_conversion() {
	let converted = convert(&wrap("a", &format!("Before <math>{}</math> after.", POWER.0)), Source::Html);
	assert_eq!(converted.text, "Before x^2 after.");
	assert_eq!(converted.links.len(), 1);
	assert_eq!(converted.links[0].text, converted.text);
	assert_eq!(converted.links[0].reference, "#target");
}

#[rstest]
#[case::at_start("")]
#[case::after_text("Before ")]
#[ignore = "deferred: links need shared rendering for nested content and its markers"]
fn link_formula_spans_count_boundary_whitespace_once(
	#[case] prefix: &str,
	#[values(Source::Html, Source::Xml)] source: Source,
) {
	let converted = convert(&format!(r##"<p>{prefix}<a href="#x">  <math>{}</math></a></p>"##, POWER.0), source);
	assert_eq!(converted.formulas.len(), 1);
	assert_eq!(converted.formulas[0].offset, display_len(prefix));
}

#[rstest]
fn reusing_a_converter_clears_formula_markers(#[values(Source::Html, Source::Xml)] source: Source) {
	let with_formulas = "<html><body><math><mi>x</mi></math></body></html>";
	let plain = "<html><body>Plain</body></html>";
	match source {
		Source::Xml => {
			let mut converter = XmlToText::new();
			assert!(converter.convert(with_formulas));
			assert_eq!(converter.get_formulas().len(), 1);
			assert!(converter.convert(plain));
			assert!(converter.get_formulas().is_empty());
		}
		Source::Html => {
			let mut converter = HtmlToText::new();
			assert!(converter.convert(with_formulas, HtmlSourceMode::NativeHtml));
			assert_eq!(converter.get_formulas().len(), 1);
			assert!(converter.convert(plain, HtmlSourceMode::NativeHtml));
			assert!(converter.get_formulas().is_empty());
		}
	}
}
