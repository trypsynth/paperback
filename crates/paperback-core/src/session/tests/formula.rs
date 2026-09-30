use super::*;

const MATHML: &str = "<math><mi>x</mi><mo>=</mo><mn>1</mn></math>";

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
