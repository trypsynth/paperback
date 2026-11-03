use std::{collections::HashMap, ops::Range};

use anyhow::{Context, Result};
use hayro_interpret::kurbo::{Affine, Point};
use roxmltree::{Document as XmlDocument, Node};

use crate::parser::util::xml::{find_child_element, is_element};

fn parse_transform(value: &str) -> Option<Affine> {
	let numbers: Vec<f64> = value.split(',').map(|number| number.trim().parse().ok()).collect::<Option<_>>()?;
	Some(Affine::new(numbers.try_into().ok()?))
}

fn transform_of(element: Node) -> Affine {
	let property = format!("{}.RenderTransform", element.tag_name().name());
	let matrix = || find_child_element(find_child_element(element, &property)?, "MatrixTransform")?.attribute("Matrix");
	element.attribute("RenderTransform").or_else(matrix).and_then(parse_transform).unwrap_or(Affine::IDENTITY)
}

fn vertical_scale(transform: Affine) -> f64 {
	let [_, _, c, d, _, _] = transform.as_coeffs();
	c.hypot(d)
}

/// In hundredths of the size. No glyph is this wide, so it can only be the jump between two strings a printer driver merged into one run.
const GAP_ADVANCE: f64 = 150.0;

#[derive(Debug, Default)]
pub(super) struct Run {
	/// What a reader gets; `source` is what `indices` counts through.
	pub text: String,
	pub source: String,
	pub indices: String,
	pub font: String,
	pub x: f64,
	pub y: f64,
	pub size: f64,
	pub bidi_level: u32,
	pub simulated_bold: bool,
}

/// One entry of `Indices` and the UTF-16 code units of the text it draws.
#[derive(Debug, PartialEq)]
pub(super) struct Cluster {
	pub code_units: usize,
	pub glyph: Option<u32>,
	pub advance: Option<f64>,
}

pub(super) fn clusters(indices: &str) -> Vec<Cluster> {
	let mut clusters: Vec<Cluster> = Vec::new();
	let mut glyphs_left = 0usize;
	for entry in indices.split(';') {
		let (mapping, metrics) = entry.strip_prefix('(').and_then(|entry| entry.split_once(')')).unzip();
		let (code_units, glyphs) = mapping.map_or((1, 1), |mapping: &str| {
			let (code_units, glyphs) = mapping.split_once(':').unwrap_or((mapping, "1"));
			(code_units.trim().parse().unwrap_or(1), glyphs.trim().parse().unwrap_or(1))
		});
		let mut metrics = metrics.unwrap_or(entry).split(',');
		let glyph = metrics.next().and_then(|glyph| glyph.trim().parse().ok());
		let advance = metrics.next().and_then(|advance| advance.trim().parse::<f64>().ok());
		match clusters.last_mut() {
			Some(cluster) if glyphs_left > 0 => {
				glyphs_left -= 1;
				cluster.advance = cluster.advance.map_or(advance, |sum| Some(sum + advance.unwrap_or(0.0)));
			}
			_ => {
				glyphs_left = glyphs.max(1) - 1;
				clusters.push(Cluster { code_units, glyph, advance });
			}
		}
	}
	clusters
}

fn spaced_at_gaps(text: &str, indices: &str) -> String {
	let mut gaps = Vec::new();
	let mut code_units = 0;
	for cluster in clusters(indices) {
		code_units += cluster.code_units;
		if cluster.advance.is_some_and(|advance| advance > GAP_ADVANCE) {
			gaps.push(code_units);
		}
	}
	let mut spaced = String::with_capacity(text.len() + gaps.len());
	let mut position = 0;
	let mut characters = text.chars().peekable();
	while let Some(character) = characters.next() {
		spaced.push(character);
		position += character.len_utf16();
		let next_is_text = characters.peek().is_some_and(|next| !next.is_whitespace());
		if gaps.contains(&position) && next_is_text && !character.is_whitespace() {
			spaced.push(' ');
		}
	}
	spaced
}

/// Rectangles are written with fewer decimals than the runs they surround, so a run can start a hair outside its own.
const LINK_TOLERANCE: f64 = 0.5;

#[derive(Debug, PartialEq)]
pub(super) struct Area {
	pub left: f64,
	pub top: f64,
	pub right: f64,
	pub bottom: f64,
}

impl Area {
	/// Word pads a link's rectangle, so the full stop after a link can start inside it and only a run that also ends there belongs to the link.
	pub fn covers(&self, run: &Run, width: Option<f64>) -> bool {
		(self.left - LINK_TOLERANCE..=self.right + LINK_TOLERANCE).contains(&run.x)
			&& (self.top - LINK_TOLERANCE..=self.bottom + LINK_TOLERANCE).contains(&run.y)
			&& width.is_none_or(|width| run.x + width <= self.right + LINK_TOLERANCE)
	}
}

#[derive(Debug)]
pub(super) enum LinkPlace {
	/// A rectangle laid over the page, which says nothing about the text beneath it.
	Area(Area),
	Runs(Range<usize>),
}

#[derive(Debug)]
pub(super) struct PageLink {
	pub uri: String,
	pub place: LinkPlace,
}

#[derive(Debug, Default)]
pub(super) struct Page {
	pub height: f64,
	pub runs: Vec<Run>,
	/// The runs under each named element, which is how the document structure points at the page.
	pub groups: HashMap<String, Range<usize>>,
	pub links: Vec<PageLink>,
}

/// Link rectangles are drawn with straight lines alone, so anything else is not one and gives `None`.
fn outline_points(data: &str) -> Option<Vec<(f64, f64)>> {
	let data = data.trim_start();
	let data = data.strip_prefix(['F', 'f']).map_or(data, |rest| rest.get(1..).unwrap_or_default());
	let tokens = data.split(|c: char| c.is_whitespace() || matches!(c, ',' | 'M' | 'L' | 'Z' | 'z'));
	let numbers: Vec<f64> =
		tokens.filter(|token| !token.is_empty()).map(|token| token.parse().ok()).collect::<Option<_>>()?;
	Some(numbers.as_chunks().0.iter().map(|[x, y]| (*x, *y)).collect())
}

fn link_area(path: Node, transform: Affine) -> Option<Area> {
	let geometry = || path.descendants().find(|node| is_element(*node, "PathGeometry"))?.attribute("Figures");
	let points = outline_points(path.attribute("Data").or_else(geometry)?)?;
	let transform = transform * transform_of(path);
	let mut points = points.into_iter().map(|(x, y)| transform * Point::new(x, y));
	let first = points.next()?;
	Some(points.fold(Area { left: first.x, top: first.y, right: first.x, bottom: first.y }, |area, point| Area {
		left: area.left.min(point.x),
		top: area.top.min(point.y),
		right: area.right.max(point.x),
		bottom: area.bottom.max(point.y),
	}))
}

/// Printer drivers keep the ligature a font drew, which search and speech would then trip over.
fn expand_ligatures(text: &str) -> String {
	let mut expanded = String::with_capacity(text.len());
	for character in text.chars() {
		match character {
			'\u{FB00}' => expanded.push_str("ff"),
			'\u{FB01}' => expanded.push_str("fi"),
			'\u{FB02}' => expanded.push_str("fl"),
			'\u{FB03}' => expanded.push_str("ffi"),
			'\u{FB04}' => expanded.push_str("ffl"),
			'\u{FB05}' | '\u{FB06}' => expanded.push_str("st"),
			other => expanded.push(other),
		}
	}
	expanded
}

fn read_run(element: Node, transform: Affine) -> Option<Run> {
	let unicode = element.attribute("UnicodeString")?;
	let unicode = unicode.strip_prefix("{}").unwrap_or(unicode);
	if unicode.is_empty() {
		return None;
	}
	let number = |name: &str| element.attribute(name).and_then(|value| value.trim().parse::<f64>().ok());
	let transform = transform * transform_of(element);
	let origin = transform * Point::new(number("OriginX")?, number("OriginY")?);
	let size = number("FontRenderingEmSize")? * vertical_scale(transform);
	let indices = element.attribute("Indices").unwrap_or_default();
	Some(Run {
		text: expand_ligatures(&spaced_at_gaps(unicode, indices)),
		source: unicode.to_string(),
		indices: indices.to_string(),
		font: element.attribute("FontUri").unwrap_or_default().to_string(),
		x: origin.x,
		y: origin.y,
		size,
		bidi_level: element.attribute("BidiLevel").and_then(|level| level.parse().ok()).unwrap_or(0),
		simulated_bold: element.attribute("StyleSimulations").is_some_and(|style| style.contains("Bold")),
	})
}

/// The fallback is the branch every reader is promised to understand.
fn alternate_branch<'a, 'input>(content: Node<'a, 'input>) -> Option<Node<'a, 'input>> {
	let branch = |name: &str| find_child_element(content, name);
	branch("Fallback").or_else(|| branch("Choice"))
}

fn walk(parent: Node, transform: Affine, page: &mut Page) {
	for element in parent.children().filter(Node::is_element) {
		let first_run = page.runs.len();
		let uri = element.attribute("FixedPage.NavigateUri").map(String::from);
		match element.tag_name().name() {
			"Glyphs" => page.runs.extend(read_run(element, transform)),
			"Canvas" => walk(element, transform * transform_of(element), page),
			"Path" => {
				if let (Some(uri), Some(area)) = (uri.clone(), link_area(element, transform)) {
					page.links.push(PageLink { uri, place: LinkPlace::Area(area) });
				}
			}
			"AlternateContent" => {
				if let Some(branch) = alternate_branch(element) {
					walk(branch, transform, page);
				}
			}
			_ => {}
		}
		let runs = first_run..page.runs.len();
		if let Some(uri) = uri.filter(|_| !runs.is_empty()) {
			page.links.push(PageLink { uri, place: LinkPlace::Runs(runs.clone()) });
		}
		if let Some(name) = element.attribute("Name") {
			page.groups.insert(name.to_string(), runs);
		}
	}
}

pub(super) fn read_page(xml: &str) -> Result<Page> {
	let parsed = XmlDocument::parse(xml).context("Failed to parse XPS page")?;
	let root = parsed.root_element();
	let mut page = Page {
		height: root.attribute("Height").and_then(|value| value.parse().ok()).unwrap_or(0.0),
		..Page::default()
	};
	walk(root, Affine::IDENTITY, &mut page);
	Ok(page)
}

#[cfg(test)]
mod tests {
	use super::*;

	fn page(body: &str) -> Page {
		read_page(&format!(
			r#"<FixedPage xmlns="http://schemas.microsoft.com/xps/2005/06" Width="816" Height="1056">{body}</FixedPage>"#
		))
		.expect("page")
	}

	fn glyphs(text: &str) -> String {
		format!(r#"<Glyphs OriginX="10" OriginY="20" FontRenderingEmSize="12" UnicodeString="{text}"/>"#)
	}

	fn texts(page: &Page) -> Vec<&str> {
		page.runs.iter().map(|run| run.text.as_str()).collect()
	}

	#[test]
	fn a_run_keeps_its_text_position_and_size() {
		let page = page(&glyphs("Hello"));
		assert_eq!(page.height, 1056.0);
		assert_eq!(texts(&page), ["Hello"]);
		assert_eq!((page.runs[0].x, page.runs[0].y, page.runs[0].size), (10.0, 20.0, 12.0));
	}

	#[test]
	fn runs_stay_in_markup_order() {
		assert_eq!(texts(&page(&format!("{}{}", glyphs("Second"), glyphs("First")))), ["Second", "First"]);
	}

	#[test]
	fn a_canvas_attribute_transform_moves_the_runs_inside_it() {
		let page = page(&format!(r#"<Canvas RenderTransform="1,0,0,1,72,146">{}</Canvas>"#, glyphs("Moved")));
		assert_eq!((page.runs[0].x, page.runs[0].y), (82.0, 166.0));
	}

	#[test]
	fn a_matrix_child_scales_positions_and_size() {
		let page = page(&format!(
			r#"<Canvas><Canvas.RenderTransform><MatrixTransform Matrix="2,0,0,2,0,0"/></Canvas.RenderTransform>{}</Canvas>"#,
			glyphs("Scaled")
		));
		assert_eq!((page.runs[0].x, page.runs[0].y, page.runs[0].size), (20.0, 40.0, 24.0));
	}

	#[test]
	fn nested_transforms_apply_from_the_outside_in() {
		let page = page(&format!(
			r#"<Canvas RenderTransform="2,0,0,2,0,0"><Canvas RenderTransform="1,0,0,1,5,5">{}</Canvas></Canvas>"#,
			glyphs("Nested")
		));
		assert_eq!((page.runs[0].x, page.runs[0].y), (30.0, 50.0));
	}

	#[test]
	fn a_transform_on_the_run_itself_counts() {
		let body = r#"<Glyphs RenderTransform="1,0,0,1,100,0" OriginX="10" OriginY="20" FontRenderingEmSize="12" UnicodeString="Own"/>"#;
		assert_eq!(page(body).runs[0].x, 110.0);
	}

	#[test]
	fn the_brace_escape_is_not_part_of_the_text() {
		assert_eq!(texts(&page(&glyphs("{}{braces}"))), ["{braces}"]);
	}

	#[test]
	fn ligature_characters_become_their_letters() {
		assert_eq!(
			texts(&page(&glyphs("\u{FB01}rst \u{FB00}ect \u{FB03}ce \u{FB04}e \u{FB02}ag"))),
			["first ffect ffice ffle flag"]
		);
	}

	fn with_indices(text: &str, indices: &str) -> String {
		format!(
			r#"<Glyphs OriginX="10" OriginY="20" FontRenderingEmSize="16" Indices="{indices}" UnicodeString="{text}"/>"#
		)
	}

	#[test]
	fn a_wide_advance_inside_a_run_is_a_gap_between_two_strings() {
		let page = page(&with_indices("Cell oneCell two", "38;72;79;79;3;82;81;72,428;38;72,45;79;79;3;87;90;82"));
		assert_eq!(texts(&page), ["Cell one Cell two"]);
	}

	#[test]
	fn an_ordinary_advance_is_no_gap() {
		assert_eq!(texts(&page(&with_indices("Cell", "38,60;72,45;79;79"))), ["Cell"]);
	}

	#[test]
	fn a_wide_advance_beside_a_space_adds_none() {
		assert_eq!(texts(&page(&with_indices("one two", ";;;,300;;;"))), ["one two"]);
	}

	#[test]
	fn a_wide_advance_at_the_end_of_a_run_adds_nothing() {
		assert_eq!(texts(&page(&with_indices("end", ";;,400"))), ["end"]);
	}

	#[test]
	fn cluster_maps_keep_the_advance_on_the_right_character() {
		assert_eq!(texts(&page(&with_indices("office next", ";(2:1)10,40;;;,300;;;;;"))), ["office next"]);
		assert_eq!(texts(&page(&with_indices("ffen", "(2:1)10,300;;"))), ["ff en"]);
	}

	#[test]
	fn a_cluster_of_several_glyphs_adds_their_advances_up() {
		assert_eq!(texts(&page(&with_indices("ab", "(1:2)10,100;11,100;12"))), ["a b"]);
	}

	#[test]
	fn characters_outside_the_basic_plane_count_as_two_code_units() {
		assert_eq!(texts(&page(&with_indices("\u{1D400}b", "(2:1)10,300;11"))), ["\u{1D400} b"]);
	}

	#[test]
	fn a_run_keeps_its_bidi_level() {
		let body = r#"<Glyphs BidiLevel="1" OriginX="10" OriginY="20" FontRenderingEmSize="12" UnicodeString="x"/>"#;
		assert_eq!(page(body).runs[0].bidi_level, 1);
		assert_eq!(page(&glyphs("x")).runs[0].bidi_level, 0);
	}

	#[test]
	fn a_run_without_text_is_left_out() {
		let body = r#"<Glyphs OriginX="10" OriginY="20" FontRenderingEmSize="12" Indices="53;88"/>"#;
		assert!(page(body).runs.is_empty());
		assert!(page(&glyphs("")).runs.is_empty());
		assert!(page(&glyphs("{}")).runs.is_empty());
	}

	#[test]
	fn alternate_content_is_read_once_from_its_fallback() {
		let body = format!(
			r#"<mc:AlternateContent xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006">
				<mc:Choice Requires="x">{}</mc:Choice>
				<mc:Fallback>{}</mc:Fallback>
			</mc:AlternateContent>"#,
			glyphs("Choice"),
			glyphs("Fallback")
		);
		assert_eq!(texts(&page(&body)), ["Fallback"]);
	}

	#[test]
	fn alternate_content_with_only_a_choice_still_counts() {
		let body = format!(
			r#"<mc:AlternateContent xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006">
				<mc:Choice Requires="x">{}</mc:Choice>
			</mc:AlternateContent>"#,
			glyphs("Choice")
		);
		assert_eq!(texts(&page(&body)), ["Choice"]);
	}

	#[test]
	fn a_name_leads_to_its_run_or_to_every_run_under_a_canvas() {
		let body = format!(
			r#"<Glyphs Name="one" OriginX="1" OriginY="2" FontRenderingEmSize="12" UnicodeString="One"/>
			<Canvas Name="many">{}<Canvas>{}</Canvas></Canvas>"#,
			glyphs("Two"),
			glyphs("Three")
		);
		let page = page(&body);
		assert_eq!(page.groups["one"], 0..1);
		assert_eq!(page.groups["many"], 1..3);
	}

	fn link_path(data: &str) -> String {
		format!(r#"<Path FixedPage.NavigateUri="https://example.com/" Data="{data}"/>"#)
	}

	fn area_of(page: &Page) -> &Area {
		match &page.links[0].place {
			LinkPlace::Area(area) => area,
			LinkPlace::Runs(runs) => panic!("expected an area, found {runs:?}"),
		}
	}

	#[test]
	fn a_link_on_a_path_is_the_rectangle_it_draws() {
		let page = page(&link_path("M 10,20 L 110,20 L 110,50 L 10,50 Z"));
		assert_eq!(page.links[0].uri, "https://example.com/");
		assert_eq!(area_of(&page), &Area { left: 10.0, top: 20.0, right: 110.0, bottom: 50.0 });
	}

	#[test]
	fn a_link_path_can_open_with_a_fill_rule_and_drop_its_commands() {
		let page = page(&link_path("F1M111.43,183.73L188.09,183.73 188.09,202.35 111.43,202.35Z"));
		assert_eq!(area_of(&page), &Area { left: 111.43, top: 183.73, right: 188.09, bottom: 202.35 });
	}

	#[test]
	fn a_link_rectangle_moves_with_the_canvas_around_it() {
		let page =
			page(&format!(r#"<Canvas RenderTransform="2,0,0,2,5,5">{}</Canvas>"#, link_path("M 0,0 L 10,0 L 10,10 Z")));
		assert_eq!(area_of(&page), &Area { left: 5.0, top: 5.0, right: 25.0, bottom: 25.0 });
	}

	#[test]
	fn a_link_covers_a_run_that_fits_inside_it_and_not_one_that_only_starts_there() {
		let area = Area { left: 272.94, top: 172.0, right: 338.97, bottom: 197.0 };
		let at = |x: f64| Run { x, y: 183.38, ..Run::default() };
		assert!(area.covers(&at(275.09), Some(61.58)));
		assert!(!area.covers(&at(336.67), Some(3.4)));
		assert!(area.covers(&at(336.67), None));
		assert!(!area.covers(&at(100.0), None));
	}

	#[test]
	fn a_link_path_can_keep_its_shape_in_a_geometry() {
		let body =
			r#"<Path FixedPage.NavigateUri="u"><Path.Data><PathGeometry Figures="M 1,2 L 3,4 Z"/></Path.Data></Path>"#;
		assert_eq!(area_of(&page(body)), &Area { left: 1.0, top: 2.0, right: 3.0, bottom: 4.0 });
	}

	#[test]
	fn a_path_with_no_link_or_a_curved_outline_gives_none() {
		assert!(page(r#"<Path Data="M 0,0 L 1,1 Z"/>"#).links.is_empty());
		assert!(page(&link_path("M 0,0 C 1,1 2,2 3,3 Z")).links.is_empty());
	}

	#[test]
	fn a_link_on_text_or_a_canvas_covers_its_runs() {
		let body = format!(
			r#"{}<Glyphs FixedPage.NavigateUri="one" OriginX="1" OriginY="2" FontRenderingEmSize="12" UnicodeString="Linked"/>
			<Canvas FixedPage.NavigateUri="two">{}{}</Canvas>"#,
			glyphs("Plain"),
			glyphs("Also"),
			glyphs("linked")
		);
		let page = page(&body);
		assert!(matches!(&page.links[0].place, LinkPlace::Runs(runs) if *runs == (1..2)));
		assert!(matches!(&page.links[1].place, LinkPlace::Runs(runs) if *runs == (2..4)));
		assert_eq!(page.links[1].uri, "two");
	}

	#[test]
	fn a_part_that_is_not_xml_is_an_error() {
		assert!(read_page("<FixedPage").is_err());
	}
}
