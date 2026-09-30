use std::collections::HashMap;

use rstest::rstest;

use super::*;

fn sections() -> Vec<SectionMeta> {
	vec![
		SectionMeta { path: "OEBPS/chapter1.xhtml".to_string(), start: 0, end: 100 },
		SectionMeta { path: "OEBPS/chapter2.xhtml".to_string(), start: 100, end: 250 },
	]
}

#[rstest]
fn returns_the_matching_sections_start_when_reference_has_no_fragment() {
	let offset = compute_nav_offset("OEBPS/chapter2.xhtml", &sections(), &HashMap::new());
	assert_eq!(offset, 100);
}

#[rstest]
fn resolves_a_fragment_within_its_section_via_qualified_id() {
	let mut id_positions = HashMap::new();
	id_positions.insert("OEBPS/chapter2.xhtml#section1".to_string(), 150);
	let offset = compute_nav_offset("OEBPS/chapter2.xhtml#section1", &sections(), &id_positions);
	assert_eq!(offset, 150);
}

#[rstest]
fn falls_back_to_section_start_when_fragment_id_is_outside_the_section_bounds() {
	// An id that happens to collide with another section's offset must not be trusted for this
	// section's fragment: it lies outside [start, end), so this should fall back to section start.
	let mut id_positions = HashMap::new();
	id_positions.insert("OEBPS/chapter2.xhtml#section1".to_string(), 5);
	let offset = compute_nav_offset("OEBPS/chapter2.xhtml#section1", &sections(), &id_positions);
	assert_eq!(offset, 100);
}

#[rstest]
fn falls_back_to_section_start_when_fragment_id_is_missing() {
	let offset = compute_nav_offset("OEBPS/chapter2.xhtml#missing", &sections(), &HashMap::new());
	assert_eq!(offset, 100);
}

#[rstest]
fn resolves_a_bare_fragment_when_the_path_does_not_match_any_section() {
	let mut id_positions = HashMap::new();
	id_positions.insert("section1".to_string(), 42);
	let offset = compute_nav_offset("unknown.xhtml#section1", &sections(), &id_positions);
	assert_eq!(offset, 42);
}

#[rstest]
fn falls_back_to_matching_by_file_name_case_insensitively_when_the_full_path_does_not_match() {
	let offset = compute_nav_offset("Text/Chapter2.XHTML", &sections(), &HashMap::new());
	assert_eq!(offset, 100);
}

#[rstest]
fn returns_zero_when_nothing_matches() {
	let offset = compute_nav_offset("nowhere/missing.xhtml", &sections(), &HashMap::new());
	assert_eq!(offset, 0);
}
