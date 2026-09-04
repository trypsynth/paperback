use super::*;

#[test]
fn two_by_two_table_is_tab_and_newline_separated() {
	let html = "<table><tr><td>a</td><td>b</td></tr><tr><td>c</td><td>d</td></tr></table>";
	assert_eq!(html_table_to_tsv(html), "a\tb\nc\td");
}

#[test]
fn header_row_cells_are_included() {
	let html = "<table><tr><th>H1</th><th>H2</th></tr><tr><td>v1</td><td>v2</td></tr></table>";
	assert_eq!(html_table_to_tsv(html), "H1\tH2\nv1\tv2");
}

#[test]
fn empty_cells_produce_empty_strings_between_tabs() {
	let html = "<table><tr><td></td><td>b</td></tr></table>";
	assert_eq!(html_table_to_tsv(html), "\tb");
}

#[test]
fn nested_table_is_flattened_to_text() {
	let html = "<table><tr><td>outer<table><tr><td>inner</td></tr></table></td><td>x</td></tr></table>";
	assert_eq!(html_table_to_tsv(html), "outerinner\tx");
}

#[test]
fn embedded_tab_and_newline_collapse_to_single_space() {
	let html = "<table><tr><td>a\t\nb</td></tr></table>";
	assert_eq!(html_table_to_tsv(html), "a b");
}

#[test]
fn entities_decoded_and_br_becomes_space() {
	let html = "<table><tr><td>1 &lt; 2 &amp; 3<br/>line2</td></tr></table>";
	assert_eq!(html_table_to_tsv(html), "1 < 2 & 3 line2");
}

#[test]
fn empty_table_yields_empty_string() {
	assert_eq!(html_table_to_tsv("<table></table>"), "");
}

#[test]
fn html_table_to_display_inline_2x2_table() {
	let html = "<table><tr><td>a</td><td>b</td></tr><tr><td>c</td><td>d</td></tr></table>";
	assert_eq!(html_table_to_display(html, true), "a\tb\nc\td");
}

#[test]
fn html_table_to_display_placeholder_2x2_table() {
	let html = "<table><tr><td>a</td><td>b</td></tr><tr><td>c</td><td>d</td></tr></table>";
	assert_eq!(html_table_to_display(html, false), "[Table]: a b");
}

#[test]
fn html_table_to_display_placeholder_single_cell() {
	let html = "<table><tr><td>x</td></tr></table>";
	assert_eq!(html_table_to_display(html, false), "[Table]: x");
}

#[test]
fn html_table_to_display_placeholder_empty_table() {
	let html = "<table></table>";
	assert_eq!(html_table_to_display(html, false), "[Table]");
}

#[test]
fn html_table_to_display_inline_empty_table() {
	let html = "<table></table>";
	assert_eq!(html_table_to_display(html, true), "");
}

#[test]
fn html_table_to_display_placeholder_header_body_table() {
	let html = "<table><tr><th>H1</th><th>H2</th></tr><tr><td>v1</td><td>v2</td></tr></table>";
	assert_eq!(html_table_to_display(html, false), "[Table]: H1 H2");
}

#[test]
fn table_caption_prefers_caption_element() {
	let html = "<table><caption>Cap</caption><tr><td>row</td></tr></table>";
	assert_eq!(table_caption_from_html(html), Some("Cap".to_string()));
}

#[test]
fn table_caption_falls_back_to_first_row() {
	let html = "<table><tr><td>a</td><td>b</td></tr><tr><td>c</td><td>d</td></tr></table>";
	assert_eq!(table_caption_from_html(html), Some("a b".to_string()));
}

#[test]
fn table_caption_returns_none_for_empty_table() {
	assert_eq!(table_caption_from_html("<table></table>"), None);
}

#[test]
fn table_lines_preserve_empty_rows_and_report_display_length() {
	let (lines, display_length) = display_lines_and_length("a\tb\n\tc");
	assert_eq!(lines, vec!["a\tb".to_string(), "\tc".to_string()]);
	assert_eq!(display_length, 7);
}

#[test]
fn table_lines_handle_empty_display_text() {
	let (lines, display_length) = display_lines_and_length("");
	assert!(lines.is_empty());
	assert_eq!(display_length, 0);
}

#[test]
fn table_render_bundle_includes_caption_lines_and_length() {
	let bundle = table_render_bundle("<table><caption>Cap</caption><tr><td>a</td><td>b</td></tr></table>", true);
	assert_eq!(bundle.caption, "Cap");
	assert_eq!(bundle.lines, vec!["a\tb".to_string()]);
	assert_eq!(bundle.display_length, 4);
}

#[test]
fn table_render_bundle_falls_back_to_default_caption() {
	let bundle = table_render_bundle("<table></table>", true);
	assert_eq!(bundle.caption, "table");
	assert!(bundle.lines.is_empty());
	assert_eq!(bundle.display_length, 0);
}

#[test]
fn build_html_table_from_grid_2x2() {
	let rows = vec![vec!["a".to_string(), "b".to_string()], vec!["c".to_string(), "d".to_string()]];
	assert_eq!(
		build_html_table_from_grid(&rows),
		"<table border=\"1\"><tr><td>a</td><td>b</td></tr><tr><td>c</td><td>d</td></tr></table>"
	);
	// Round-trips through the shared TSV renderer.
	assert_eq!(html_table_to_tsv(&build_html_table_from_grid(&rows)), "a\tb\nc\td");
}

#[test]
fn build_html_table_from_grid_empty() {
	assert_eq!(build_html_table_from_grid(&[]), "<table border=\"1\"></table>");
}

#[test]
fn build_html_table_from_grid_empty_cell() {
	let rows = vec![vec![String::new(), "b".to_string()]];
	assert_eq!(build_html_table_from_grid(&rows), "<table border=\"1\"><tr><td></td><td>b</td></tr></table>");
}
