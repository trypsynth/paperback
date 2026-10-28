use super::*;
use crate::{document::ParseSettings, util::test_support::TempDir};

fn layout(name: &str, html: &str) -> ReadingLayout {
	let dir = TempDir::new("reading-layout");
	let path = dir.write_str(name, html);
	ReadingLayout::build(Arc::new(
		DocumentSession::new(&path, "", "", ParseSettings::default()).expect("parse test document"),
	))
}

fn html(body: &str) -> ReadingLayout {
	layout("book.html", &format!("<html><head><title>Book</title></head><body>{body}</body></html>"))
}

fn all_units(layout: &ReadingLayout) -> Vec<ReadingUnitFfi> {
	layout.units(0, layout.unit_count())
}

fn kinds_and_texts(layout: &ReadingLayout) -> Vec<(ReadingUnitKind, String)> {
	all_units(layout).into_iter().map(|unit| (unit.kind, unit.text)).collect()
}

#[test]
fn each_paragraph_and_heading_is_a_unit_and_blank_lines_are_none() {
	let layout = html("<h1>Title</h1><p>One.</p><p>Two.</p><h2>Part</h2><p>Three.</p>");
	assert_eq!(
		kinds_and_texts(&layout),
		vec![
			(ReadingUnitKind::Heading, "Title".to_string()),
			(ReadingUnitKind::Paragraph, "One.".to_string()),
			(ReadingUnitKind::Paragraph, "Two.".to_string()),
			(ReadingUnitKind::Heading, "Part".to_string()),
			(ReadingUnitKind::Paragraph, "Three.".to_string()),
		]
	);
	let units = all_units(&layout);
	assert_eq!((units[0].level, units[3].level), (1, 2));
	assert_eq!(layout.block_count(), 5);
	assert_eq!(layout.heading_units(), vec![0, 3]);
}

#[test]
fn a_list_is_one_block_of_its_items() {
	let layout = html("<p>Before</p><ul><li>One</li><li>Two</li><li>Three</li></ul><p>After</p>");
	let kinds: Vec<ReadingUnitKind> = all_units(&layout).into_iter().map(|unit| unit.kind).collect();
	assert_eq!(
		kinds,
		vec![
			ReadingUnitKind::Paragraph,
			ReadingUnitKind::ListItem,
			ReadingUnitKind::ListItem,
			ReadingUnitKind::ListItem,
			ReadingUnitKind::Paragraph
		]
	);
	assert_eq!(layout.block_count(), 3);
	assert_eq!(
		layout.block(1),
		ReadingBlockFfi { kind: ReadingBlockKind::List, first_unit: 1, unit_count: 3, table: -1 }
	);
	assert_eq!(layout.block(2).first_unit, 4);
	assert_eq!((1..4).map(|unit| layout.block_of_unit(unit)).collect::<Vec<_>>(), vec![1, 1, 1]);
	assert_eq!(layout.block_of_unit(4), 2);
}

#[test]
fn two_lists_in_a_row_stay_two_blocks() {
	let layout = html("<ul><li>A</li><li>B</li></ul><ol><li>C</li></ol>");
	assert_eq!(layout.block_count(), 2);
	assert_eq!(layout.block(0).unit_count, 2);
	assert_eq!(
		layout.block(1),
		ReadingBlockFfi { kind: ReadingBlockKind::List, first_unit: 2, unit_count: 1, table: -1 }
	);
}

#[test]
fn a_table_is_a_caption_then_one_block_of_rows_with_its_grid() {
	let layout = html(
		"<p>Before</p><table><caption>Prices</caption><tr><th>Item</th><th>Cost</th></tr><tr><td>Tea</td><td>3</td></tr><tr><td colspan=\"2\">Total 3</td></tr></table><p>After</p>",
	);
	assert_eq!(
		kinds_and_texts(&layout),
		vec![
			(ReadingUnitKind::Paragraph, "Before".to_string()),
			(ReadingUnitKind::Caption, "Prices".to_string()),
			(ReadingUnitKind::TableRow, "Item Cost".to_string()),
			(ReadingUnitKind::TableRow, "Tea 3".to_string()),
			(ReadingUnitKind::TableRow, "Total 3".to_string()),
			(ReadingUnitKind::Paragraph, "After".to_string()),
		]
	);
	assert_eq!(layout.block_count(), 4);
	assert_eq!(
		layout.block(2),
		ReadingBlockFfi { kind: ReadingBlockKind::Table, first_unit: 2, unit_count: 3, table: 0 }
	);
	assert_eq!(layout.block(3).first_unit, 5);
	assert_eq!(layout.table_units(), vec![2]);
	let table = layout.table(0);
	assert_eq!(
		(table.row_count, table.column_count, table.header_row_count, table.first_unit, table.block),
		(3, 2, 1, 2, 2)
	);
	let total = table.cells.iter().find(|cell| cell.text == "Total 3").unwrap();
	assert_eq!((total.row, total.column, total.column_span), (2, 0, 2));
	assert!(table.cells.iter().filter(|cell| cell.row == 0).all(|cell| cell.is_header));
}

#[test]
fn each_row_of_a_table_has_its_own_position_and_finds_its_way_back() {
	let layout = html(
		"<p>Before</p><table><tr><td>A</td><td>B</td></tr><tr><td>C</td><td>D</td></tr><tr><td>E</td><td>F</td></tr></table><p>After</p>",
	);
	let positions: Vec<DocPos> = (0..layout.unit_count()).map(|unit| layout.unit_position(unit)).collect();
	assert!(positions.windows(2).all(|pair| pair[0] < pair[1]), "{positions:?}");
	for unit in 0..layout.unit_count() {
		assert_eq!(layout.unit_at_position(layout.unit_position(unit)), unit);
	}
}

#[test]
fn a_one_cell_table_reads_as_a_paragraph_without_the_placeholder() {
	let layout = html("<table><tr><td>Just this</td></tr></table>");
	assert_eq!(kinds_and_texts(&layout), vec![(ReadingUnitKind::Paragraph, "Just this".to_string())]);
	assert!(layout.table_units().is_empty());
}

#[test]
fn an_image_on_its_own_line_is_an_image_with_its_description() {
	let layout = html(
		"<p><img src=\"a.png\" alt=\"A cat\"></p><p>Text <img src=\"b.png\" alt=\"inline\"> more</p><p><img src=\"c.png\"></p>",
	);
	let units = all_units(&layout);
	assert_eq!((units[0].kind, units[0].description.as_str()), (ReadingUnitKind::Image, "A cat"));
	assert_eq!(units[1].kind, ReadingUnitKind::Paragraph);
	assert_eq!((units[2].kind, units[2].description.as_str()), (ReadingUnitKind::Image, ""));
}

#[test]
fn links_sit_where_their_text_is_in_the_unit() {
	let layout =
		html("<p>Go <a href=\"#end\">there</a> or <a href=\"https://example.com\">out</a>.</p><p id=\"end\">End</p>");
	let unit = &all_units(&layout)[0];
	assert_eq!(unit.text, "Go there or out.");
	let spans: Vec<(i64, i64, &str)> =
		unit.links.iter().map(|link| (link.start, link.length, link.text.as_str())).collect();
	assert_eq!(spans, vec![(3, 5, "there"), (12, 3, "out")]);
	assert_eq!(layout.session.activate_link(unit.links[0].position).offset, layout.unit_position(1));
}

#[test]
fn link_stops_walk_every_link_in_order_and_back() {
	let layout = html(
		"<p><a href=\"#a\">one</a> and <a href=\"#b\">two</a></p><p>none</p><p id=\"a\"><a href=\"#b\">three</a></p><p id=\"b\">end</p>",
	);
	let mut stops = Vec::new();
	let (mut unit, mut link) = (0, -1);
	loop {
		let stop = layout.link_stop(unit, link, true);
		if !stop.found {
			break;
		}
		stops.push((stop.unit, stop.link));
		(unit, link) = (stop.unit, stop.link);
	}
	assert_eq!(stops, vec![(0, 0), (0, 1), (2, 0)]);
	assert_eq!(layout.link_stop(2, 0, false), ReadingLinkStopFfi { found: true, unit: 0, link: 1 });
	assert_eq!(layout.link_stop(1, -1, false), ReadingLinkStopFfi { found: true, unit: 0, link: 1 });
	assert!(!layout.link_stop(0, 0, false).found);
}

#[test]
fn a_position_on_a_blank_line_belongs_to_the_unit_before() {
	let layout = html("<p>One</p><p>Two</p>");
	let content = layout.session.content();
	let blank = content.find("\n\n").map(|byte| DocLen::of(&content[..=byte]).as_pos());
	if let Some(blank) = blank {
		assert_eq!(layout.unit_at_position(blank), 0);
	}
	assert_eq!(layout.unit_at_position(layout.buffer().end()), 1);
}

#[test]
fn sections_are_the_table_of_contents_entries_units() {
	let layout = html("<h1>A</h1><p>x</p><h2>B</h2><p>y</p>");
	let sections = layout.section_units();
	assert!(sections.iter().all(|unit| layout.heading_units().contains(unit)), "{sections:?}");
}

#[test]
fn units_past_the_end_are_left_out() {
	let layout = html("<p>One</p><p>Two</p>");
	assert_eq!(layout.units(1, 10).len(), 1);
	assert!(layout.units(5, 10).is_empty());
}

#[test]
fn an_empty_document_has_no_units() {
	let layout = layout("empty.txt", "");
	assert_eq!((layout.unit_count(), layout.block_count()), (0, 0));
	assert_eq!(layout.unit_at_position(DocPos::ZERO), 0);
}
