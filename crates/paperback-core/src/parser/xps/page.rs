use std::{collections::HashMap, ops::Range};

use anyhow::{Context, Result};
use roxmltree::{Document as XmlDocument, Node};

use super::package::is_element;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Transform {
	a: f64,
	b: f64,
	c: f64,
	d: f64,
	e: f64,
	f: f64,
}

impl Transform {
	const IDENTITY: Self = Self { a: 1.0, b: 0.0, c: 0.0, d: 1.0, e: 0.0, f: 0.0 };

	fn parse(value: &str) -> Option<Self> {
		let numbers: Vec<f64> = value.split(',').map(|number| number.trim().parse().ok()).collect::<Option<_>>()?;
		let [a, b, c, d, e, f] = numbers[..] else { return None };
		Some(Self { a, b, c, d, e, f })
	}

	fn of(element: Node) -> Self {
		let property = format!("{}.RenderTransform", element.tag_name().name());
		let matrix = || {
			let holder = element.children().find(|child| is_element(*child, &property))?;
			holder.children().find(|child| is_element(*child, "MatrixTransform"))?.attribute("Matrix")
		};
		element.attribute("RenderTransform").or_else(matrix).and_then(Self::parse).unwrap_or(Self::IDENTITY)
	}

	fn then(self, inner: Self) -> Self {
		Self {
			a: self.a * inner.a + self.c * inner.b,
			b: self.b * inner.a + self.d * inner.b,
			c: self.a * inner.c + self.c * inner.d,
			d: self.b * inner.c + self.d * inner.d,
			e: self.a * inner.e + self.c * inner.f + self.e,
			f: self.b * inner.e + self.d * inner.f + self.f,
		}
	}

	fn apply(self, x: f64, y: f64) -> (f64, f64) {
		(self.a * x + self.c * y + self.e, self.b * x + self.d * y + self.f)
	}

	fn vertical_scale(self) -> f64 {
		self.c.hypot(self.d)
	}
}

#[derive(Debug)]
pub(super) struct Run {
	pub text: String,
	pub x: f64,
	pub y: f64,
	pub size: f64,
}

#[derive(Debug, Default)]
pub(super) struct Page {
	pub height: f64,
	pub runs: Vec<Run>,
	/// The runs under each named element, which is how the document structure points at the page.
	pub groups: HashMap<String, Range<usize>>,
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

fn read_run(element: Node, transform: Transform) -> Option<Run> {
	let unicode = element.attribute("UnicodeString")?;
	let unicode = unicode.strip_prefix("{}").unwrap_or(unicode);
	if unicode.is_empty() {
		return None;
	}
	let number = |name: &str| element.attribute(name).and_then(|value| value.trim().parse::<f64>().ok());
	let transform = transform.then(Transform::of(element));
	let (x, y) = transform.apply(number("OriginX")?, number("OriginY")?);
	let size = number("FontRenderingEmSize")? * transform.vertical_scale();
	Some(Run { text: expand_ligatures(unicode), x, y, size })
}

/// The fallback is the branch every reader is promised to understand.
fn alternate_branch<'a, 'input>(content: Node<'a, 'input>) -> Option<Node<'a, 'input>> {
	let branch = |name: &str| content.children().find(|child| is_element(*child, name));
	branch("Fallback").or_else(|| branch("Choice"))
}

fn walk(parent: Node, transform: Transform, page: &mut Page) {
	for element in parent.children().filter(Node::is_element) {
		let first_run = page.runs.len();
		match element.tag_name().name() {
			"Glyphs" => page.runs.extend(read_run(element, transform)),
			"Canvas" => walk(element, transform.then(Transform::of(element)), page),
			"AlternateContent" => {
				if let Some(branch) = alternate_branch(element) {
					walk(branch, transform, page);
				}
			}
			_ => {}
		}
		if let Some(name) = element.attribute("Name") {
			page.groups.insert(name.to_string(), first_run..page.runs.len());
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
	walk(root, Transform::IDENTITY, &mut page);
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

	#[test]
	fn a_part_that_is_not_xml_is_an_error() {
		assert!(read_page("<FixedPage").is_err());
	}
}
