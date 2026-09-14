use rstest::{fixture, rstest};

use super::*;
use crate::util::{test_support::TempDir, text::display_len};

const MATHML: &str = "<math><mi>x</mi><mo>=</mo><mn>1</mn></math>";

const MARKDOWN_SOURCE: &str = "Before $x^2$.\n\n$$\n\\frac{a}{b}\n$$\n\nAfter.";

/// A session over [`MARKDOWN_SOURCE`], with the directory that holds the file and receives
/// web and source views.
struct MarkdownSession {
	dir: TempDir,
	session: DocumentSession,
}

impl MarkdownSession {
	fn temp_path(&self) -> &str {
		self.dir.path().to_str().unwrap()
	}
}

#[fixture]
fn markdown() -> MarkdownSession {
	let dir = TempDir::new("markdown-formula-session");
	let path = dir.write_str("formulas.md", MARKDOWN_SOURCE);
	let session = DocumentSession::new(&path, "", "", ParseSettings::default()).expect("open Markdown");
	MarkdownSession { dir, session }
}

#[rstest]
fn markdown_formulas_are_navigable(markdown: MarkdownSession) {
	let first = markdown.session.navigate_formula(0, false, true);
	let second = markdown.session.navigate_formula(first.offset, false, true);
	assert_eq!((first.found, first.marker_text.as_str()), (true, "x^2"));
	assert_eq!((second.found, second.marker_text.as_str()), (true, "a/b"));
}

#[rstest]
fn markdown_formula_view_shows_mathml(markdown: MarkdownSession) {
	let first = markdown.session.navigate_formula(0, false, true);
	let formula = markdown.session.get_formula_at_position(first.offset).expect("formula view content");
	assert!(formula.contains("<msup>"), "{formula}");
}

#[rstest]
fn markdown_web_view_renders_formulas_as_mathml(markdown: MarkdownSession) {
	let first = markdown.session.navigate_formula(0, false, true);
	let formula = markdown.session.get_formula_at_position(first.offset).expect("formula view content");
	let target = markdown.session.webview_target_path(first.offset, markdown.temp_path()).expect("Markdown web view");
	let html = scraper::Html::parse_document(&std::fs::read_to_string(target.path).unwrap());
	let math = scraper::Selector::parse("math").unwrap();
	let formulas: Vec<_> = html.select(&math).collect();
	assert_eq!(formulas.len(), 2);
	assert_eq!(formulas[0].html(), formula);
	assert_eq!(formulas[1].attr("display"), Some("block"));
}

#[rstest]
fn markdown_source_view_maps_positions_past_formulas(markdown: MarkdownSession) {
	let content = markdown.session.content();
	let after = content.find("After.").unwrap();
	let position = i64::try_from(display_len(&content[..after])).unwrap();
	let view = markdown.session.view_source(position, markdown.temp_path()).expect("Markdown source view");
	assert_eq!(std::fs::read_to_string(view.path).unwrap(), MARKDOWN_SOURCE);
	assert_eq!(view.caret, i64::try_from(MARKDOWN_SOURCE.find("After.").unwrap()).unwrap());
}

fn formula_session() -> DocumentSession {
	let mut buffer = DocumentBuffer::with_content("before\nx = 1 after".to_string());
	buffer.add_marker(
		Marker::new(MarkerType::Formula, 7)
			.with_length(5)
			.with_text("x = 1".to_string())
			.with_reference(MATHML.to_string()),
	);
	session_from_buffer(buffer)
}

#[test]
fn reference_uses_half_open_display_extent() {
	let session = formula_session();
	assert_eq!(session.get_formula_at_position(7).as_deref(), Some(MATHML));
	assert_eq!(session.get_formula_at_position(11).as_deref(), Some(MATHML));
	assert!(session.get_formula_at_position(12).is_none());
	assert!(session.get_formula_at_position(2).is_none());
}

#[test]
fn missing_reference_does_not_activate() {
	let mut buffer = DocumentBuffer::with_content("x".to_string());
	buffer.add_marker(Marker::new(MarkerType::Formula, 0).with_length(1));
	assert!(session_from_buffer(buffer).get_formula_at_position(0).is_none());
}

#[test]
fn navigation_finds_formulas_and_wraps() {
	let session = formula_session();
	let result = session.navigate_formula(0, false, true);
	assert!(result.found);
	assert_eq!(result.offset, 7);
	assert_eq!(result.marker_text, "x = 1");
	assert!(!session.navigate_formula(0, false, false).found);
	assert_eq!(session.navigate_formula(12, false, false).offset, 7);
	let wrapped = session.navigate_formula(12, true, true);
	assert!(wrapped.found && wrapped.wrapped);
	assert_eq!(wrapped.offset, 7);
}

#[test]
fn supported_only_when_formulas_are_present() {
	assert!(formula_session().get_supported_segment_types_ffi().iter().any(|t| matches!(t, SegmentTypeFfi::Formula)));
	let without_formulas = sample_session(ParserFlags::NONE);
	assert!(without_formulas.navigate_formula(0, false, true).not_supported);
	assert!(!without_formulas.get_supported_segment_types_ffi().iter().any(|t| matches!(t, SegmentTypeFfi::Formula)));
}

#[test]
fn text_segment_navigates_formulas() {
	let segment = formula_session().get_text_segment(0, SegmentTypeFfi::Formula, SegmentDirectionFfi::Next);
	assert!(segment.found);
	assert_eq!(segment.start_pos, 7);
	assert_eq!(segment.end_pos, 12);
	assert_eq!(segment.text, "x = 1");
}
