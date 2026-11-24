//! QRead's books: an SQLite `shelf` of keys and values, with the text in `raw.text` and its pages, headings, links, tables and contents in `tree.json`.

use std::{
	collections::{HashMap, HashSet},
	fmt::Write as _,
};

use anyhow::{Context, Result};
use rusqlite::types::ValueRef;
use serde::Deserialize;
use serde_json::Value;

use crate::{
	document::{DocPos, Document, DocumentBuffer, Marker, MarkerType, ParserContext, TocItem},
	parser::{
		Parser,
		convert::layout_table::{TableElement, is_layout_table},
		util::path::extract_title_from_path,
	},
	util::{html::escape, sqlite::open_read_only, text::url_decode},
};

pub struct QrdParser;

#[derive(Deserialize, Default)]
struct Book {
	text_location: Option<String>,
	tree_location: Option<String>,
	metadata_location: Option<String>,
	#[serde(default)]
	original_path: String,
}

#[derive(Deserialize, Default)]
struct Metadata {
	#[serde(default)]
	title: String,
	#[serde(default)]
	authors: Vec<String>,
}

#[derive(Deserialize, Default)]
struct Tree {
	#[serde(default)]
	tree: Vec<Node>,
}

#[derive(Deserialize)]
struct Node {
	#[serde(rename = "type")]
	kind: String,
	id: Option<i64>,
	parent: Option<i64>,
	#[serde(default)]
	start: i64,
	end: Option<i64>,
	name: Option<String>,
	href: Option<String>,
	level: Option<Value>,
	pagenum: Option<i64>,
	tag: Option<String>,
	attrs: Option<serde_json::Map<String, Value>>,
}

/// QRead ran on Python 2, whose strings on Windows count UTF-16 units, so its offsets do too; Paperback's count characters, and drop the carriage returns of CRLF.
struct Offsets {
	/// Each unit no character starts at: the second half of a surrogate pair, or a dropped carriage return.
	lost: Vec<usize>,
	units: usize,
}

impl Offsets {
	fn position(&self, unit: i64) -> DocPos {
		let unit = usize::try_from(unit).unwrap_or(0).min(self.units);
		DocPos::from_raw(unit - self.lost.partition_point(|&lost| lost < unit))
	}
}

fn normalize(raw: &str) -> (String, Offsets) {
	let mut text = String::with_capacity(raw.len());
	let mut lost = Vec::new();
	let mut unit = 0;
	let mut chars = raw.chars().peekable();
	while let Some(ch) = chars.next() {
		match ch {
			'\r' if chars.peek() == Some(&'\n') => lost.push(unit),
			'\r' => text.push('\n'),
			_ => {
				text.push(ch);
				if ch.len_utf16() == 2 {
					lost.push(unit + 1);
				}
			}
		}
		unit += ch.len_utf16();
	}
	(text, Offsets { lost, units: unit })
}

fn read_shelf(path: &str) -> Result<HashMap<String, Vec<u8>>> {
	let connection = open_read_only(path).with_context(|| format!("Failed to open QRead book: {path}"))?;
	let mut statement =
		connection.prepare("SELECT key, value FROM shelf").with_context(|| format!("Not a QRead book: {path}"))?;
	let rows = statement.query_map([], |row| {
		let value = match row.get_ref(1)? {
			ValueRef::Text(bytes) | ValueRef::Blob(bytes) => bytes.to_vec(),
			_ => Vec::new(),
		};
		Ok((row.get::<_, String>(0)?, value))
	})?;
	rows.collect::<rusqlite::Result<_>>().with_context(|| format!("QRead book is unreadable: {path}"))
}

fn json<T: for<'a> Deserialize<'a> + Default>(shelf: &HashMap<String, Vec<u8>>, key: &str) -> T {
	shelf.get(key).map_or_else(T::default, |data| {
		serde_json::from_slice(data).unwrap_or_else(|err| {
			tracing::warn!(key, error = %err, "skipping unreadable qrd section");
			T::default()
		})
	})
}

impl Parser for QrdParser {
	fn parse(&self, context: &ParserContext) -> Result<Document> {
		let mut shelf = read_shelf(&context.file_path)?;
		let book: Book = shelf
			.get("book.json")
			.and_then(|data| serde_json::from_slice(data).ok())
			.with_context(|| format!("Not a QRead book, as it has no book.json: {}", context.file_path))?;
		let metadata: Metadata = json(&shelf, book.metadata_location.as_deref().unwrap_or("metadata.json"));
		let nodes = json::<Tree>(&shelf, book.tree_location.as_deref().unwrap_or("tree.json")).tree;
		let raw = shelf.remove(book.text_location.as_deref().unwrap_or("raw.text")).unwrap_or_default();
		let (text, offsets) = normalize(&String::from_utf8_lossy(&raw));
		let mut buffer = DocumentBuffer::with_content(text);
		let span = |node: &Node| {
			let start = offsets.position(node.start);
			(start, node.end.map_or(start, |end| offsets.position(end).max(start)))
		};
		let mut id_positions = HashMap::new();
		let mut ids = HashMap::new();
		let mut pages = Vec::new();
		for node in nodes.iter().filter(|node| node.kind == "id") {
			let Some(name) = node.name.as_deref().filter(|name| !name.is_empty()) else { continue };
			let (position, _) = span(node);
			id_positions.entry(name.to_string()).or_insert(position);
			ids.entry(name.to_lowercase()).or_insert_with(|| name.to_string());
			if !name.contains('#') {
				pages.push((position, name.to_string()));
			}
		}
		pages.sort_by_key(|&(position, _)| position);
		for (index, (position, name)) in pages.iter().enumerate() {
			buffer.add_marker(
				Marker::new(MarkerType::SectionBreak, *position)
					.with_text(format!("Section {}", index + 1))
					.with_reference(name.clone()),
			);
		}
		let tables = Tables::new(&nodes);
		let mut page_count = 0;
		for node in &nodes {
			let (start, end) = span(node);
			let spanned = buffer.text(start..end).trim().to_string();
			match node.kind.as_str() {
				"page" => {
					page_count += 1;
					let number = node.pagenum.unwrap_or(page_count);
					buffer.add_marker(Marker::new(MarkerType::PageBreak, start).with_text(format!("Page {number}")));
				}
				"heading" => {
					let text = node
						.name
						.as_deref()
						.map(str::trim)
						.filter(|name| !name.is_empty())
						.map_or(spanned, str::to_string);
					let level = heading_level(node.level.as_ref());
					buffer.add_marker(Marker::new(heading_marker(level), start).with_text(text).with_level(level));
				}
				"link" => {
					let page = pages[..pages.partition_point(|&(position, _)| position <= start)]
						.last()
						.map_or("", |(_, name)| name);
					let text = if spanned.is_empty() { node.name.clone().unwrap_or_default() } else { spanned };
					buffer.add_marker(
						Marker::new(MarkerType::Link, start)
							.with_text(text)
							.with_reference(resolve_link(node.href.as_deref().unwrap_or_default(), page, &ids))
							.with_length(end.since(start)),
					);
				}
				"table" if tables.is_data(node) => {
					let html = tables.html(node, &buffer, &span);
					buffer.add_marker(
						Marker::new(MarkerType::Table, start).with_reference(html).with_length(end.since(start)),
					);
				}
				_ => {}
			}
		}
		let title = book_title(&metadata.title, &book.original_path)
			.unwrap_or_else(|| extract_title_from_path(&context.file_path));
		let mut document = Document::new().with_title(title);
		if !metadata.authors.is_empty() {
			document = document.with_author(metadata.authors.join(", "));
		}
		document.toc_items = toc_items(&nodes, &|node| span(node).0);
		document.set_buffer(buffer);
		document.id_positions = id_positions;
		Ok(document)
	}
}

/// QRead names a book with no title of its own by the file it came from, extension and all.
fn book_title(title: &str, original_path: &str) -> Option<String> {
	let title = title.trim();
	let original_name = original_path.rsplit(['\\', '/']).next().unwrap_or_default();
	let title = if !original_name.is_empty() && title == original_name {
		original_name.rsplit_once('.').map_or(original_name, |(stem, _)| stem)
	} else {
		title
	};
	(!title.is_empty()).then(|| title.to_string())
}

fn heading_level(level: Option<&Value>) -> i32 {
	let level = match level {
		Some(Value::Number(number)) => number.as_i64(),
		Some(Value::String(text)) => text.trim().parse().ok(),
		_ => None,
	};
	level.and_then(|level| i32::try_from(level).ok()).unwrap_or(1).clamp(1, 6)
}

const fn heading_marker(level: i32) -> MarkerType {
	match level {
		1 => MarkerType::Heading1,
		2 => MarkerType::Heading2,
		3 => MarkerType::Heading3,
		4 => MarkerType::Heading4,
		5 => MarkerType::Heading5,
		_ => MarkerType::Heading6,
	}
}

/// QRead joined some links wrongly: a page's own `#fragment` onto its folder, and `mailto:` onto the folder too. A link to an anchor the book lacks goes to its page.
fn resolve_link(href: &str, page: &str, ids: &HashMap<String, String>) -> String {
	let href = url_decode(href.trim());
	if let Some(url) = external_url(&href) {
		return url.to_string();
	}
	let (path, fragment) =
		href.split_once('#').map_or((href.as_str(), None), |(path, fragment)| (path, Some(fragment)));
	let path = if path.is_empty()
		|| (fragment.is_some() && path.ends_with('/') && page.to_lowercase().starts_with(&path.to_lowercase()))
	{
		page
	} else {
		path
	};
	let key = fragment.map_or_else(|| path.to_string(), |fragment| format!("{path}#{fragment}"));
	ids.get(&key.to_lowercase()).or_else(|| ids.get(&path.to_lowercase())).cloned().unwrap_or(key)
}

fn external_url(href: &str) -> Option<&str> {
	if let Some(at) = href.find("mailto:") {
		return Some(&href[at..]);
	}
	let colon = href.find("://")?;
	let start = href[..colon]
		.rfind(|c: char| !(c.is_ascii_alphanumeric() || matches!(c, '+' | '-' | '.')))
		.map_or(0, |at| at + 1);
	(start < colon).then(|| &href[start..])
}

/// QRead kept each row and cell where it falls in the text, so the table view gets the table back.
struct Tables<'a> {
	by_id: HashMap<i64, &'a Node>,
	children: HashMap<i64, Vec<&'a Node>>,
	/// By where they start, since QRead parents a heading to the one before it, not to the cell it is in.
	headings: Vec<&'a Node>,
}

impl<'a> Tables<'a> {
	fn new(nodes: &'a [Node]) -> Self {
		let by_id = nodes.iter().filter_map(|node| Some((node.id?, node))).collect();
		let mut children: HashMap<i64, Vec<&Node>> = HashMap::new();
		for node in nodes {
			if let Some(parent) = node.parent {
				children.entry(parent).or_default().push(node);
			}
		}
		let mut headings: Vec<&Node> = nodes.iter().filter(|node| node.kind == "heading").collect();
		headings.sort_by_key(|node| node.start);
		Self { by_id, children, headings }
	}

	fn children(&self, node: &Node) -> &[&'a Node] {
		node.id.and_then(|id| self.children.get(&id)).map_or(&[], Vec::as_slice)
	}

	/// A layout table is read as the text in its cells, as the HTML converter reads one, and a table inside a data table is part of that table.
	fn is_data(&self, table: &Node) -> bool {
		if is_layout_table(Element { node: table, tables: self }) {
			return false;
		}
		let mut parent = table.parent;
		for _ in 0..self.by_id.len() {
			let Some(ancestor) = parent.and_then(|id| self.by_id.get(&id)) else { break };
			if ancestor.kind == "table" {
				return !self.is_data(ancestor);
			}
			parent = ancestor.parent;
		}
		true
	}

	/// In document order; rows in a table nested in one of its cells belong to that table.
	fn collect_rows(&self, node: &Node, rows: &mut Vec<&'a Node>) {
		for child in self.children(node) {
			match child.kind.as_str() {
				"tr" => rows.push(child),
				"thead" | "tbody" | "tfoot" => self.collect_rows(child, rows),
				_ => {}
			}
		}
	}

	fn html(&self, table: &'a Node, buffer: &DocumentBuffer, span: &dyn Fn(&Node) -> (DocPos, DocPos)) -> String {
		let mut rows = Vec::new();
		self.collect_rows(table, &mut rows);
		if rows.is_empty() {
			return String::new();
		}
		let mut html = String::from("<table>");
		for row in rows {
			html.push_str("<tr>");
			for cell in self.children(row).iter().filter(|cell| matches!(cell.kind.as_str(), "td" | "th")) {
				let (start, end) = span(cell);
				let tag = cell.kind.as_str();
				let _ = write!(html, "<{tag}>{}</{tag}>", escape(buffer.text(start..end).trim()));
			}
			html.push_str("</tr>");
		}
		html.push_str("</table>");
		html
	}
}

/// A table node as the layout check sees an element.
#[derive(Clone, Copy)]
struct Element<'a> {
	node: &'a Node,
	tables: &'a Tables<'a>,
}

impl TableElement for Element<'_> {
	fn tag(self) -> Option<String> {
		Some(if self.node.kind == "heading" {
			self.node.tag.clone().unwrap_or_else(|| "h1".to_string())
		} else {
			self.node.kind.clone()
		})
	}

	fn attr(self, name: &str) -> Option<String> {
		let value = self.node.attrs.as_ref()?.get(name)?;
		Some(value.as_str().map_or_else(|| value.to_string(), str::to_string))
	}

	fn children(self) -> Vec<Self> {
		if self.node.kind == "heading" {
			return Vec::new();
		}
		let mut children = self.tables.children(self.node).to_vec();
		if matches!(self.node.kind.as_str(), "td" | "th") {
			let end = self.node.end.unwrap_or(self.node.start);
			let headings = &self.tables.headings;
			let first = headings.partition_point(|heading| heading.start < self.node.start);
			children.extend(headings[first..].iter().take_while(|heading| heading.start < end));
		}
		children.into_iter().map(|node| Self { node, tables: self.tables }).collect()
	}
}

fn toc_items(nodes: &[Node], position: &dyn Fn(&Node) -> DocPos) -> Vec<TocItem> {
	let toc_ids: HashSet<i64> = nodes.iter().filter(|node| node.kind == "toc").filter_map(|node| node.id).collect();
	let mut children: HashMap<Option<i64>, Vec<&Node>> = HashMap::new();
	for entry in nodes.iter().filter(|node| node.kind == "toc") {
		children.entry(entry.parent.filter(|parent| toc_ids.contains(parent))).or_default().push(entry);
	}
	toc_children(None, &children, position)
}

/// An entry only its own descendants point at is never reached from a root, so a cycle in the file can't recurse forever.
fn toc_children(
	parent: Option<i64>,
	children: &HashMap<Option<i64>, Vec<&Node>>,
	position: &dyn Fn(&Node) -> DocPos,
) -> Vec<TocItem> {
	children.get(&parent).map_or_else(Vec::new, |entries| {
		entries
			.iter()
			.map(|entry| {
				let name = entry.name.as_deref().unwrap_or_default().trim().to_string();
				let mut item = TocItem::new(name, String::new(), position(entry));
				if entry.id.is_some() {
					item.children = toc_children(entry.id, children, position);
				}
				item
			})
			.collect()
	})
}

#[cfg(test)]
mod tests;
