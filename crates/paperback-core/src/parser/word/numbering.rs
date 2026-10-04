//! Word's automatic numbering: the `1.`, `a)`, `iii.` and bullets that a `.docx` keeps out of
//! the text and generates from `word/numbering.xml`.
//!
//! A paragraph joins a list through `<w:numPr>` (a list instance, `numId`, and a level, `ilvl`),
//! either directly or through its paragraph style, which is how Word numbers headings. Each
//! instance points at an abstract list that defines, per level, the number format, the label
//! template (`%1.%2.`) and the starting value. Instances that share an abstract list continue
//! one count, unless an instance overrides a level's start, which is what Word's "Restart
//! numbering" writes.

use std::collections::{HashMap, HashSet};

use roxmltree::{Document as XmlDocument, Node, NodeType};

use crate::parser::util::xml::find_child_element;

const LEVELS: usize = 9;

#[derive(Clone, Debug, Default)]
pub(super) struct Level {
	/// `w:numFmt`: decimal, lowerLetter, upperRoman, bullet, none, ...
	pub(super) format: String,
	/// `w:lvlText`: the label template, `%N` standing for level N's current number.
	pub(super) text: String,
	pub(super) start: i32,
	/// `w:isLgl`: every level's number is shown in decimal, as legal documents number sections.
	pub(super) legal: bool,
}

#[derive(Debug, Default)]
pub(super) struct Instance {
	pub(super) abstract_id: String,
	pub(super) start_overrides: HashMap<usize, i32>,
	pub(super) level_overrides: HashMap<usize, Level>,
}

/// The numbering definitions of one document and the running counts through it.
#[derive(Debug, Default)]
pub(super) struct Numbering {
	abstracts: HashMap<String, Vec<Option<Level>>>,
	instances: HashMap<String, Instance>,
	/// Paragraph style id to the list instance and level it numbers its paragraphs with.
	styles: HashMap<String, (String, usize)>,
	counters: HashMap<String, [Option<i32>; LEVELS]>,
	started: HashSet<String>,
}

/// A paragraph's place in a list, as [`Numbering::label_for`] reports it.
#[derive(Debug, PartialEq, Eq)]
pub(super) struct ListLabel {
	/// What the paragraph is numbered or bulleted with: `1.`, `b)`, `•`.
	pub text: String,
	/// Its level, 1 for the outermost.
	pub level: i32,
}

impl Numbering {
	/// Reads the definitions from `word/numbering.xml` and the numbered paragraph styles from
	/// `word/styles.xml`. Either may be missing, which leaves nothing numbered.
	pub(super) fn load(numbering_xml: Option<&str>, styles_xml: Option<&str>) -> Self {
		let mut numbering = Self::default();
		if let Some(xml) = numbering_xml.and_then(|text| XmlDocument::parse(text).ok()) {
			for node in xml.root().descendants().filter(Node::is_element) {
				match node.tag_name().name() {
					"abstractNum" => {
						if let Some(id) = node.attribute("abstractNumId") {
							let mut levels = vec![None; LEVELS];
							for lvl in children(node, "lvl") {
								if let Some(index) = level_index(lvl) {
									levels[index] = Some(read_level(lvl));
								}
							}
							numbering.abstracts.insert(id.to_string(), levels);
						}
					}
					"num" => {
						let (Some(id), Some(abstract_id)) =
							(node.attribute("numId"), find_child_element(node, "abstractNumId").and_then(val))
						else {
							continue;
						};
						let mut instance = Instance { abstract_id: abstract_id.to_string(), ..Instance::default() };
						for over in children(node, "lvlOverride") {
							let Some(index) = level_index(over) else { continue };
							if let Some(start) = find_child_element(over, "startOverride").and_then(val_i32) {
								instance.start_overrides.insert(index, start);
							}
							if let Some(lvl) = find_child_element(over, "lvl") {
								instance.level_overrides.insert(index, read_level(lvl));
							}
						}
						numbering.instances.insert(id.to_string(), instance);
					}
					_ => {}
				}
			}
		}
		if let Some(xml) = styles_xml.and_then(|text| XmlDocument::parse(text).ok()) {
			numbering.styles = read_numbered_styles(&xml);
		}
		numbering
	}

	/// The label for a paragraph with these properties, advancing the count it belongs to.
	/// `None` for a paragraph that is not numbered, or whose level shows no label.
	pub(super) fn label_for(&mut self, paragraph_properties: Option<Node>) -> Option<ListLabel> {
		let (num_id, ilvl) = self.list_of(paragraph_properties?)?;
		self.label_of(&num_id, ilvl)
	}

	/// Numbering already parsed by a reader of its own format (the binary `.doc` one): abstract
	/// lists by id, each with its levels, and the instances that point at them, by id.
	pub(super) fn from_definitions(
		abstracts: HashMap<String, Vec<Option<Level>>>,
		instances: HashMap<String, Instance>,
	) -> Self {
		Self { abstracts, instances, ..Self::default() }
	}

	/// The label for the next item of list instance `num_id` at level `ilvl` (0-based),
	/// advancing its count. `None` when the instance or level is unknown, or shows no label.
	pub(super) fn label_of(&mut self, num_id: &str, ilvl: usize) -> Option<ListLabel> {
		let ilvl = ilvl.min(LEVELS - 1);
		let num_id = num_id.to_string();
		let instance = self.instances.get(&num_id)?;
		let abstract_id = instance.abstract_id.clone();
		let levels: Vec<Option<Level>> = (0..LEVELS)
			.map(|i| {
				instance
					.level_overrides
					.get(&i)
					.cloned()
					.or_else(|| self.abstracts.get(&abstract_id).and_then(|l| l.get(i).cloned().flatten()))
			})
			.collect();
		let start_overrides = instance.start_overrides.clone();
		let level = levels.get(ilvl)?.clone()?;
		let counters = self.counters.entry(abstract_id).or_insert([None; LEVELS]);
		// The first paragraph of an instance that restarts a level begins it again.
		if self.started.insert(num_id) {
			for (index, start) in start_overrides {
				if index < LEVELS {
					counters[index] = Some(start - 1);
					counters[index + 1..].fill(None);
				}
			}
		}
		let value = counters[ilvl].map_or(level.start, |current| current + 1);
		counters[ilvl] = Some(value);
		counters[ilvl + 1..].fill(None);
		let text = render_label(&level, &levels, counters);
		(!text.is_empty()).then(|| ListLabel { text, level: i32::try_from(ilvl).unwrap_or(0) + 1 })
	}

	/// The list instance and level a paragraph belongs to, from its own `<w:numPr>` or, failing
	/// that, its paragraph style's. A `numId` of 0 takes the paragraph out of its style's list.
	fn list_of(&self, ppr: Node) -> Option<(String, usize)> {
		let direct = find_child_element(ppr, "numPr");
		let style = find_child_element(ppr, "pStyle").and_then(val).and_then(|id| self.styles.get(id));
		let num_id = direct
			.and_then(|n| find_child_element(n, "numId"))
			.and_then(val)
			.map(str::to_string)
			.or_else(|| style.map(|(id, _)| id.clone()))?;
		if num_id == "0" {
			return None;
		}
		let ilvl = direct
			.and_then(|n| find_child_element(n, "ilvl"))
			.and_then(val_i32)
			.and_then(|v| usize::try_from(v).ok())
			.or_else(|| style.map(|&(_, level)| level))
			.unwrap_or(0)
			.min(LEVELS - 1);
		Some((num_id, ilvl))
	}
}

fn children<'a, 'input>(node: Node<'a, 'input>, name: &'static str) -> impl Iterator<Item = Node<'a, 'input>> {
	node.children().filter(move |c| c.node_type() == NodeType::Element && c.tag_name().name() == name)
}

fn val<'a>(node: Node<'a, '_>) -> Option<&'a str> {
	node.attribute("val")
}

fn val_i32(node: Node) -> Option<i32> {
	val(node).and_then(|v| v.parse().ok())
}

fn level_index(node: Node) -> Option<usize> {
	node.attribute("ilvl").and_then(|v| v.parse::<usize>().ok()).filter(|&i| i < LEVELS)
}

fn read_level(lvl: Node) -> Level {
	Level {
		format: find_child_element(lvl, "numFmt").and_then(val).unwrap_or("decimal").to_string(),
		text: find_child_element(lvl, "lvlText").and_then(val).unwrap_or("").to_string(),
		start: find_child_element(lvl, "start").and_then(val_i32).unwrap_or(1),
		legal: find_child_element(lvl, "isLgl").is_some_and(|n| val(n).is_none_or(|v| !matches!(v, "0" | "false"))),
	}
}

/// Paragraph styles that number their paragraphs, following `basedOn` so a style inherits the
/// numbering of the one it is built on.
fn read_numbered_styles(xml: &XmlDocument) -> HashMap<String, (String, usize)> {
	let mut direct: HashMap<String, (Option<String>, Option<usize>)> = HashMap::new();
	let mut based_on: HashMap<String, String> = HashMap::new();
	for style in xml.root().descendants().filter(|n| n.is_element() && n.tag_name().name() == "style") {
		let Some(id) = style.attribute("styleId") else { continue };
		if let Some(parent) = find_child_element(style, "basedOn").and_then(val) {
			based_on.insert(id.to_string(), parent.to_string());
		}
		if let Some(num_pr) = find_child_element(style, "pPr").and_then(|p| find_child_element(p, "numPr")) {
			let num_id = find_child_element(num_pr, "numId").and_then(val).map(str::to_string);
			let ilvl = find_child_element(num_pr, "ilvl").and_then(val_i32).and_then(|v| usize::try_from(v).ok());
			direct.insert(id.to_string(), (num_id, ilvl));
		}
	}
	let mut resolved = HashMap::new();
	for id in direct.keys().chain(based_on.keys()) {
		let (mut num_id, mut ilvl) = (None, None);
		let mut current = Some(id.as_str());
		// Bounded walk: a style chain that loops back on itself numbers nothing.
		for _ in 0..16 {
			let Some(style) = current else { break };
			if let Some((n, l)) = direct.get(style) {
				num_id = num_id.or_else(|| n.clone());
				ilvl = ilvl.or(*l);
			}
			current = based_on.get(style).map(String::as_str);
		}
		if let Some(num_id) = num_id {
			resolved.insert(id.clone(), (num_id, ilvl.unwrap_or(0).min(LEVELS - 1)));
		}
	}
	resolved
}

/// Fills a level's label template with the current numbers of the levels it names.
fn render_label(level: &Level, levels: &[Option<Level>], counters: &[Option<i32>; LEVELS]) -> String {
	if level.format == "bullet" {
		return bullet(&level.text);
	}
	if level.format == "none" && !level.text.contains('%') {
		return level.text.trim().to_string();
	}
	let mut out = String::new();
	let mut chars = level.text.chars().peekable();
	while let Some(ch) = chars.next() {
		if ch == '%'
			&& let Some(digit) = chars.peek().and_then(|d| d.to_digit(10))
			&& (1..=9).contains(&digit)
		{
			chars.next();
			let index = digit as usize - 1;
			let referenced = levels.get(index).cloned().flatten().unwrap_or_default();
			let value = counters[index].unwrap_or(referenced.start);
			let format = if level.legal { "decimal" } else { referenced.format.as_str() };
			out.push_str(&format_number(value, format));
		} else {
			out.push(ch);
		}
	}
	out.trim().to_string()
}

/// A bullet's own character, unless it is a symbol-font glyph with no meaning outside that
/// font (Word's default bullets are private-use characters from Symbol and Wingdings).
fn bullet(text: &str) -> String {
	let text = text.trim();
	match text.chars().next() {
		None => "\u{2022}".to_string(),
		Some(c) if ('\u{E000}'..='\u{F8FF}').contains(&c) => "\u{2022}".to_string(),
		Some('o') if text.len() == 1 => "\u{25E6}".to_string(),
		Some(_) => text.to_string(),
	}
}

pub(super) fn format_number(value: i32, format: &str) -> String {
	match format {
		"none" => String::new(),
		"decimalZero" if (0..10).contains(&value) => format!("0{value}"),
		"lowerLetter" => letters(value).to_lowercase(),
		"upperLetter" => letters(value),
		"lowerRoman" => roman(value).to_lowercase(),
		"upperRoman" => roman(value),
		_ => value.to_string(),
	}
}

/// A, B, ... Z, AA, BB, ...: Word repeats the letter rather than counting on like a column name.
fn letters(value: i32) -> String {
	let Ok(n) = u32::try_from(value) else { return value.to_string() };
	if n == 0 {
		return value.to_string();
	}
	let letter = char::from_u32(u32::from(b'A') + (n - 1) % 26).unwrap_or('A');
	letter.to_string().repeat(((n - 1) / 26 + 1) as usize)
}

fn roman(value: i32) -> String {
	const NUMERALS: [(i32, &str); 13] = [
		(1000, "M"),
		(900, "CM"),
		(500, "D"),
		(400, "CD"),
		(100, "C"),
		(90, "XC"),
		(50, "L"),
		(40, "XL"),
		(10, "X"),
		(9, "IX"),
		(5, "V"),
		(4, "IV"),
		(1, "I"),
	];
	if !(1..4000).contains(&value) {
		return value.to_string();
	}
	let mut rest = value;
	let mut out = String::new();
	for (n, numeral) in NUMERALS {
		while rest >= n {
			out.push_str(numeral);
			rest -= n;
		}
	}
	out
}

#[cfg(test)]
mod tests;
