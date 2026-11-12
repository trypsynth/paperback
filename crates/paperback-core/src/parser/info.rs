//! GNU Info manuals, the format `info` and Emacs read, which `makeinfo` writes from Texinfo.
//!
//! A manual is nodes separated by `^_` (0x1F), each opening with a header line naming it and the nodes around it. A large manual is split: its main file lists the pieces in an `Indirect:` table and gives every node and anchor a byte offset in a `Tag Table:`, counted as if the pieces were one file without their preambles.

use std::{
	collections::{HashMap, HashSet},
	fs,
	io::Read,
	path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use flate2::read::GzDecoder;

use crate::{
	document::{DocPos, Document, DocumentBuffer, Marker, MarkerType, ParserContext, TocItem},
	parser::{
		Parser,
		util::{path::extract_title_from_path, toc::heading_level_to_marker_type},
	},
	util::{encoding::convert_to_utf8, text::normalize_line_endings},
};

#[cfg(test)]
mod tests;

const SEPARATOR: u8 = 0x1f;
/// Quotes a node name holding characters that would otherwise end it, such as a colon or a full stop.
const QUOTE: char = '\x7f';
/// Separates a node name from an index entry's line number in [`Document::id_positions`] keys, which no node name holds.
const LINE_KEY_SEPARATOR: char = '\u{1f}';

pub struct InfoParser;

impl Parser for InfoParser {
	fn parse(&self, context: &ParserContext) -> Result<Document> {
		tracing::debug!(path = %context.file_path, "parsing Info manual");
		let path = main_file(Path::new(&context.file_path));
		let manual = Manual::read(&path)?;
		if manual.nodes.is_empty() {
			anyhow::bail!("'{}' is not an Info manual", context.file_path);
		}
		let built = Builder::new(&manual).build();
		let title = built.title.clone().unwrap_or_else(|| info_title_from_path(&path));
		tracing::debug!(path = %context.file_path, nodes = manual.nodes.len(), "parsed Info manual");
		let mut document = Document::new().with_title(title);
		document.set_buffer(built.buffer);
		document.toc_items = built.toc;
		document.id_positions = built.id_positions;
		Ok(document)
	}
}

/// Whether `path` is named like an Info file or a piece of one, gzipped or not: `sed.info`, `find.info-2.gz`.
#[must_use]
pub fn is_info_name(path: &Path) -> bool {
	let name = path.file_name().and_then(|name| name.to_str()).unwrap_or_default().to_ascii_lowercase();
	let name = name.strip_suffix(".gz").unwrap_or(&name);
	if Path::new(name).extension().is_some_and(|extension| extension == "info") {
		return true;
	}
	name.rsplit_once(".info-")
		.is_some_and(|(stem, piece)| !stem.is_empty() && !piece.is_empty() && piece.bytes().all(|b| b.is_ascii_digit()))
}

/// A piece such as `find.info-2.gz` holds only some nodes, and only the main file knows the rest, so opening a piece opens its manual.
fn main_file(path: &Path) -> PathBuf {
	let Some(name) = path.file_name().and_then(|name| name.to_str()) else { return path.to_path_buf() };
	let (unzipped, gz) = name.strip_suffix(".gz").map_or((name, ""), |stem| (stem, ".gz"));
	let Some((stem, piece)) = unzipped.rsplit_once(".info-") else { return path.to_path_buf() };
	if piece.is_empty() || !piece.bytes().all(|b| b.is_ascii_digit()) {
		return path.to_path_buf();
	}
	let main = format!("{stem}.info");
	[format!("{main}{gz}"), main.clone(), format!("{main}.gz")]
		.into_iter()
		.map(|candidate| path.with_file_name(candidate))
		.find(|candidate| candidate.is_file())
		.unwrap_or_else(|| path.to_path_buf())
}

fn info_title_from_path(path: &Path) -> String {
	let title = extract_title_from_path(&path.to_string_lossy());
	let title = title.strip_suffix(".gz").unwrap_or(&title);
	title.strip_suffix(".info").unwrap_or(title).to_owned()
}

fn read_bytes(path: &Path) -> Result<Vec<u8>> {
	let bytes = fs::read(path).with_context(|| format!("Failed to open Info file '{}'", path.display()))?;
	if !bytes.starts_with(&[0x1f, 0x8b]) {
		return Ok(bytes);
	}
	let mut out = Vec::new();
	GzDecoder::new(bytes.as_slice())
		.read_to_end(&mut out)
		.with_context(|| format!("Failed to decompress '{}'", path.display()))?;
	Ok(out)
}

/// One node as it stands in the file.
struct RawNode {
	name: String,
	up: Option<String>,
	/// Where the node's separator is, in the tag table's count.
	offset: usize,
	/// From the separator on, so lines and anchor offsets count the same way the tag table does.
	bytes: Vec<u8>,
}

struct Manual {
	nodes: Vec<RawNode>,
	/// Anchors from the tag table, by name, at their offset in the tag table's count.
	anchors: Vec<(String, usize)>,
}

impl Manual {
	fn read(path: &Path) -> Result<Self> {
		let main = read_bytes(path)?;
		let main_sections = sections(&main);
		let mut pieces = Vec::new();
		let mut anchors = Vec::new();
		for &(start, end) in &main_sections {
			let body = section_body(&main[start..end]);
			if let Some(table) = body.strip_prefix(b"Indirect:") {
				pieces = indirect_table(table);
			} else if let Some(table) = body.strip_prefix(b"Tag Table:") {
				anchors = tag_table_anchors(table);
			}
		}
		let mut nodes = Vec::new();
		if pieces.is_empty() {
			collect_nodes(&main, &main_sections, 0, 0, &mut nodes);
		} else {
			for (name, start) in pieces {
				let piece_path = path.with_file_name(&name);
				let piece_path = if piece_path.is_file() {
					piece_path
				} else {
					let mut gzipped = piece_path.into_os_string();
					gzipped.push(".gz");
					PathBuf::from(gzipped)
				};
				let bytes = read_bytes(&piece_path)
					.with_context(|| format!("Info manual '{}' is missing its part '{name}'", path.display()))?;
				let piece_sections = sections(&bytes);
				let first = piece_sections.first().map_or(0, |&(start, _)| start);
				collect_nodes(&bytes, &piece_sections, start, first, &mut nodes);
			}
		}
		Ok(Self { nodes, anchors })
	}
}

/// The byte ranges of the `^_`-separated sections, each from its separator to the next, leaving out the preamble before the first.
fn sections(bytes: &[u8]) -> Vec<(usize, usize)> {
	let starts: Vec<usize> = bytes.iter().enumerate().filter(|&(_, &b)| b == SEPARATOR).map(|(i, _)| i).collect();
	starts.iter().enumerate().map(|(i, &start)| (start, starts.get(i + 1).copied().unwrap_or(bytes.len()))).collect()
}

/// A section's text after its separator, and the form feed and line break that may follow it.
fn section_body(section: &[u8]) -> &[u8] {
	let body = &section[1..];
	let body = body.strip_prefix(b"\x0c").unwrap_or(body);
	body.strip_prefix(b"\r\n").or_else(|| body.strip_prefix(b"\n")).unwrap_or(body)
}

/// `find.info-1: 1200` lines: each piece, and where its first separator falls in the tag table's count.
fn indirect_table(table: &[u8]) -> Vec<(String, usize)> {
	String::from_utf8_lossy(table)
		.lines()
		.filter_map(|line| {
			let (name, offset) = line.rsplit_once(':')?;
			Some((name.trim().to_owned(), offset.trim().parse().ok()?))
		})
		.collect()
}

/// `Ref: name\x7f123` lines, which `@anchor` writes; nodes are found from their own headers instead.
fn tag_table_anchors(table: &[u8]) -> Vec<(String, usize)> {
	String::from_utf8_lossy(table)
		.lines()
		.filter_map(|line| {
			let (name, offset) = line.strip_prefix("Ref: ")?.rsplit_once(QUOTE)?;
			Some((name.to_owned(), offset.trim().parse().ok()?))
		})
		.collect()
}

/// Adds the nodes among `sections` of `bytes`. A separator `first` bytes into the file is at `base` in the tag table's count.
fn collect_nodes(bytes: &[u8], sections: &[(usize, usize)], base: usize, first: usize, nodes: &mut Vec<RawNode>) {
	for &(start, end) in sections {
		let section = &bytes[start..end];
		let body = section_body(section);
		let header_end = body.iter().position(|&b| b == b'\n').unwrap_or(body.len());
		let header = String::from_utf8_lossy(&body[..header_end]);
		let Some(name) = header_field(&header, "Node") else { continue };
		nodes.push(RawNode {
			name,
			up: header_field(&header, "Up"),
			offset: base + (start - first),
			bytes: section.to_vec(),
		});
	}
}

/// A field of a node header such as `File: sed.info,  Node: Top,  Next: Introduction,  Up: (dir)`.
fn header_field(header: &str, field: &str) -> Option<String> {
	let key = format!("{field}:");
	let mut rest = header;
	loop {
		let at = rest.find(&key)?;
		let before = &rest[..at];
		rest = &rest[at + key.len()..];
		if before.is_empty() || before.ends_with(char::is_whitespace) || before.ends_with(',') {
			break;
		}
	}
	let rest = rest.trim_start();
	let value = if let Some(quoted) = rest.strip_prefix(QUOTE) {
		quoted.split(QUOTE).next().unwrap_or_default()
	} else {
		rest.split([',', '\t']).next().unwrap_or_default()
	};
	let value = value.trim();
	(!value.is_empty()).then(|| value.to_owned())
}

/// What a link in the manual points at.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Target {
	/// A node or anchor in this manual.
	Node(String),
	/// A line of a node, as an index entry gives it, counting the node's header as line 1.
	Line(String, usize),
	/// Somewhere in another manual, `(coreutils)ls invocation`, which can't be followed from here.
	Elsewhere,
}

fn parse_target(text: &str) -> Target {
	let text = unquote(text.trim());
	if text.starts_with('(') {
		return Target::Elsewhere;
	}
	Target::Node(collapse_whitespace(&text))
}

fn unquote(text: &str) -> String {
	text.replace(QUOTE, "")
}

fn collapse_whitespace(text: &str) -> String {
	text.split_whitespace().collect::<Vec<_>>().join(" ")
}

struct Built {
	buffer: DocumentBuffer,
	toc: Vec<TocItem>,
	id_positions: HashMap<String, DocPos>,
	title: Option<String>,
}

struct Builder<'a> {
	manual: &'a Manual,
	buffer: DocumentBuffer,
	/// Each node's start in the text, by name.
	node_starts: HashMap<String, DocPos>,
	/// Every node and anchor name, to tell a link that can be followed.
	known: HashSet<String>,
	/// Names by their lowercase form, since `info` falls back to a case-blind match.
	names_by_lowercase: HashMap<String, String>,
	/// For each node, where each of its lines starts in the text, or the text after a line that wasn't kept.
	line_positions: Vec<Vec<DocPos>>,
	/// Index entries pointing at a node's line, to give `id_positions` keys once every node is placed.
	line_targets: Vec<(String, usize)>,
	/// The first heading of each node, for the table of contents.
	node_titles: Vec<Option<String>>,
}

impl<'a> Builder<'a> {
	fn new(manual: &'a Manual) -> Self {
		let names: Vec<&String> =
			manual.nodes.iter().map(|node| &node.name).chain(manual.anchors.iter().map(|(name, _)| name)).collect();
		let known = names.iter().map(|&name| name.clone()).collect();
		// Reversed, so the first of two names that differ only in case wins, as `info` would take it.
		let names_by_lowercase = names.iter().rev().map(|&name| (name.to_lowercase(), name.clone())).collect();
		Self {
			manual,
			buffer: DocumentBuffer::new(),
			node_starts: HashMap::new(),
			known,
			names_by_lowercase,
			line_positions: Vec::new(),
			line_targets: Vec::new(),
			node_titles: Vec::new(),
		}
	}

	fn build(mut self) -> Built {
		for node in &self.manual.nodes {
			if !self.buffer.is_empty() {
				self.buffer.append("\n");
			}
			let start = self.buffer.end();
			self.node_starts.entry(node.name.clone()).or_insert(start);
			let text = normalize_line_endings(convert_to_utf8(section_body(&node.bytes)));
			let (lines, title) = self.write_node(&text);
			self.line_positions.push(lines);
			self.node_titles.push(title);
		}
		let mut id_positions: HashMap<String, DocPos> =
			self.node_starts.iter().map(|(name, &start)| (name.clone(), start)).collect();
		for (name, offset) in &self.manual.anchors {
			if let Some(position) = self.anchor_position(*offset) {
				id_positions.entry(name.clone()).or_insert(position);
			}
		}
		for (name, line) in std::mem::take(&mut self.line_targets) {
			if let Some(position) = self.line_position(&name, line) {
				id_positions.insert(line_key(&name, line), position);
			}
		}
		let toc = self.toc();
		let title = self
			.manual
			.nodes
			.iter()
			.position(|node| node.name.eq_ignore_ascii_case("Top"))
			.and_then(|index| self.node_titles[index].clone());
		Built { buffer: self.buffer, toc, id_positions, title }
	}

	/// Writes one node without its header line, and returns where each of its lines landed and its first heading. A line that isn't written, such as an underline or a blank, is given where the text after it starts.
	fn write_node(&mut self, text: &str) -> (Vec<DocPos>, Option<String>) {
		let lines: Vec<&str> = text.split('\n').collect();
		let mut positions = vec![DocPos::ZERO; lines.len()];
		// Lines before this one have their place in `positions`.
		let mut placed = 0;
		let mut title = None;
		let mut index = 1;
		let mut list_start: Option<DocPos> = None;
		let mut in_menu = false;
		// Blank lines are written only between text, never at the start or the end of a node.
		let mut pending_blank = false;
		let mut wrote_text = false;
		while index < lines.len() {
			let line = lines[index];
			if line.trim().is_empty() {
				pending_blank = wrote_text;
				index += 1;
				continue;
			}
			if line.trim_start().starts_with("\0\u{8}[index\0\u{8}]") {
				index += 1;
				continue;
			}
			if !in_menu && is_menu_start(line) {
				in_menu = true;
				index += 1;
				continue;
			}
			let entry = in_menu.then(|| menu_entry(&lines, index)).flatten();
			if entry.is_none()
				&& let Some(start) = list_start.take()
			{
				self.end_list(start);
			}
			if pending_blank {
				self.buffer.append("\n");
				pending_blank = false;
			}
			let position = self.buffer.end();
			let used = if let Some((entry, used)) = entry {
				if list_start.is_none() {
					list_start = Some(position);
				}
				self.write_menu_entry(&entry);
				used
			} else if let Some((level, used)) = heading_at(&lines, index) {
				let heading = clean_inline(line.trim());
				title.get_or_insert_with(|| heading.clone());
				self.buffer.add_marker(
					Marker::new(heading_level_to_marker_type(level), position)
						.with_level(level)
						.with_text(heading.clone()),
				);
				self.buffer.append(&heading);
				self.buffer.append("\n");
				used
			} else {
				self.write_text_line(&lines, index)
			};
			index = (index + used).min(lines.len());
			positions[placed..index].fill(position);
			placed = index;
			wrote_text = true;
		}
		if let Some(start) = list_start {
			self.end_list(start);
		}
		let end = self.buffer.end();
		positions[placed..].fill(end);
		(positions, title)
	}

	fn end_list(&mut self, start: DocPos) {
		let length = self.buffer.end().since(start);
		if !length.is_zero() {
			self.buffer.add_marker(Marker::new(MarkerType::List, start).with_length(length));
		}
	}

	fn write_menu_entry(&mut self, entry: &MenuEntry) {
		let start = self.buffer.end();
		self.buffer.add_marker(Marker::new(MarkerType::ListItem, start).with_text(entry.label.clone()));
		self.write_link(&entry.label, &entry.target);
		if !entry.description.is_empty() {
			self.buffer.append(": ");
			self.buffer.append(&entry.description);
		} else if let Target::Line(name, _) = &entry.target
			&& name != &entry.label
		{
			self.buffer.append(": ");
			self.buffer.append(name);
		}
		self.buffer.append("\n");
	}

	/// Writes a line of text, with any cross-reference that starts on it, and returns how many lines it took: a reference can run onto the next.
	fn write_text_line(&mut self, lines: &[&str], index: usize) -> usize {
		let mut text = lines[index].to_owned();
		let mut used = 1;
		// A reference broken across lines is completed from the lines after it, as `info` reads it.
		while has_unfinished_reference(&text) && index + used < lines.len() && !lines[index + used].trim().is_empty() {
			text.push(' ');
			text.push_str(lines[index + used].trim_start());
			used += 1;
		}
		let text = text.trim_end();
		let mut rest = text;
		while let Some((before, reference, after)) = next_reference(rest) {
			self.buffer.append(&clean_inline(before));
			self.buffer.append(if reference.capital { "See " } else { "see " });
			self.write_link(&reference.label, &reference.target);
			self.buffer.append(&reference.trailing);
			rest = after;
		}
		self.buffer.append(&clean_inline(rest));
		self.buffer.append("\n");
		used
	}

	fn write_link(&mut self, label: &str, target: &Target) {
		let position = self.buffer.end();
		self.buffer.append(label);
		let reference = match target {
			Target::Node(name) => self.resolve(name).map(|name| format!("#{name}")),
			Target::Line(name, line) => self.resolve(name).map(|name| {
				self.line_targets.push((name.clone(), *line));
				format!("#{}", line_key(&name, *line))
			}),
			Target::Elsewhere => None,
		};
		if let Some(reference) = reference {
			self.buffer.add_marker(
				Marker::new(MarkerType::Link, position).with_text(label.to_owned()).with_reference(reference),
			);
		}
	}

	/// The node or anchor `name` means, matching exactly first and then ignoring case, as `info` does.
	fn resolve(&self, name: &str) -> Option<String> {
		if self.known.contains(name) {
			return Some(name.to_owned());
		}
		self.names_by_lowercase.get(&name.to_lowercase()).cloned()
	}

	fn anchor_position(&self, offset: usize) -> Option<DocPos> {
		let index = self.manual.nodes.iter().rposition(|node| node.offset <= offset)?;
		let node = &self.manual.nodes[index];
		let within = (offset - node.offset).min(node.bytes.len());
		// The separator's own line comes first, then the header, which is line 0 of `line_positions`.
		let line = memchr::memchr_iter(b'\n', &node.bytes[..within]).count().saturating_sub(1);
		let positions = &self.line_positions[index];
		positions.get(line).or_else(|| positions.last()).copied()
	}

	fn line_position(&self, name: &str, line: usize) -> Option<DocPos> {
		let index = self.manual.nodes.iter().position(|node| node.name == name)?;
		let positions = &self.line_positions[index];
		positions.get(line.saturating_sub(1)).or_else(|| positions.last()).copied()
	}

	/// Nodes nest under the node their `Up:` names, in the order they come in the manual.
	fn toc(&self) -> Vec<TocItem> {
		let nodes = &self.manual.nodes;
		let index_of: HashMap<&str, usize> =
			nodes.iter().enumerate().map(|(i, node)| (node.name.as_str(), i)).rev().collect();
		let parent: Vec<Option<usize>> = nodes
			.iter()
			.enumerate()
			.map(|(i, node)| {
				let up = index_of.get(node.up.as_deref()?).copied()?;
				// A node above itself, or below one that comes later, would make the tree loop.
				(up < i).then_some(up)
			})
			.collect();
		let mut children: Vec<Vec<usize>> = vec![Vec::new(); nodes.len()];
		let mut roots = Vec::new();
		for (i, up) in parent.iter().enumerate() {
			match up {
				Some(up) => children[*up].push(i),
				None => roots.push(i),
			}
		}
		// The Top node is the manual itself, so its children are the top of the contents.
		if let [only] = *roots.as_slice()
			&& nodes[only].name.eq_ignore_ascii_case("Top")
		{
			roots = std::mem::take(&mut children[only]);
			roots.insert(0, only);
		}
		roots.iter().map(|&i| self.toc_item(i, &children)).collect()
	}

	fn toc_item(&self, index: usize, children: &[Vec<usize>]) -> TocItem {
		let node = &self.manual.nodes[index];
		let name = self.node_titles[index].clone().unwrap_or_else(|| node.name.clone());
		let offset = self.node_starts.get(&node.name).copied().unwrap_or(DocPos::ZERO);
		let mut item = TocItem::new(name, format!("#{}", node.name), offset);
		if !node.name.eq_ignore_ascii_case("Top") {
			item.children = children[index].iter().map(|&child| self.toc_item(child, children)).collect();
		}
		item
	}
}

fn line_key(name: &str, line: usize) -> String {
	format!("{name}{LINE_KEY_SEPARATOR}{line}")
}

fn is_menu_start(line: &str) -> bool {
	line.trim_end().eq_ignore_ascii_case("* Menu:")
}

/// A title is a line with an underline of `*`, `=`, `-` or `.` as long as it below it. Returns its level and how many lines it takes.
fn heading_at(lines: &[&str], index: usize) -> Option<(i32, usize)> {
	let title = lines[index].trim_end();
	let underline = lines.get(index + 1)?.trim_end();
	let mut chars = underline.chars();
	let mark = chars.next()?;
	let level = match mark {
		'*' => 1,
		'=' => 2,
		'-' => 3,
		'.' => 4,
		_ => return None,
	};
	if !chars.all(|c| c == mark) || title.starts_with(char::is_whitespace) {
		return None;
	}
	let width = title.chars().count();
	let underline_len = underline.chars().count();
	// makeinfo underlines to the title's display width, which wide characters make longer than its count.
	(underline_len >= 3 && underline_len >= width && underline_len <= width * 2).then_some((level, 2))
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct MenuEntry {
	label: String,
	target: Target,
	description: String,
}

/// The entry starting at `index`, `* Label: Node.  Description` or `* Node::  Description`, with how many lines it takes: its description can run onto lines indented under it, and an index entry's `(line N)` can sit on the next.
fn menu_entry(lines: &[&str], index: usize) -> Option<(MenuEntry, usize)> {
	let rest = lines[index].strip_prefix("* ")?;
	let (label, after) = split_label(rest)?;
	let (target, mut description) = if let Some(after) = after.strip_prefix(':') {
		(parse_target(&label), after.trim().to_owned())
	} else {
		let (node, description) = split_node(after.trim_start());
		(parse_target(&node), description.trim().to_owned())
	};
	let mut used = 1;
	while let Some(next) = lines.get(index + used)
		&& next.starts_with(char::is_whitespace)
		&& !next.trim().is_empty()
	{
		if !description.is_empty() {
			description.push(' ');
		}
		description.push_str(next.trim());
		used += 1;
	}
	let (target, description) = match (target, line_number(&description)) {
		(Target::Node(name), Some((line, without))) => (Target::Line(name, line), without),
		(target, _) => (target, description),
	};
	let label = clean_inline(&collapse_whitespace(&unquote(&label)));
	Some((MenuEntry { label, target, description: clean_inline(&description) }, used))
}

/// The label of a menu entry or reference, up to the colon that ends it, and what follows that colon.
fn split_label(text: &str) -> Option<(String, &str)> {
	if let Some(quoted) = text.strip_prefix(QUOTE) {
		let end = quoted.find(QUOTE)?;
		let after = quoted[end + QUOTE.len_utf8()..].strip_prefix(':')?;
		return Some((quoted[..end].to_owned(), after));
	}
	let colon = text.find(':')?;
	Some((text[..colon].to_owned(), &text[colon + 1..]))
}

/// A node name after a label, ending at a full stop, comma or tab, and the rest of the line after that.
fn split_node(text: &str) -> (String, &str) {
	if let Some(quoted) = text.strip_prefix(QUOTE)
		&& let Some(end) = quoted.find(QUOTE)
	{
		let after = &quoted[end + QUOTE.len_utf8()..];
		return (quoted[..end].to_owned(), after.strip_prefix(['.', ',']).unwrap_or(after));
	}
	// A full stop ends the name only when a space, the end of the line or a closing parenthesis follows, so `sed.info` stays one name.
	let bytes = text.as_bytes();
	let end = (0..bytes.len())
		.find(|&i| match bytes[i] {
			b',' | b'\t' => true,
			b'.' => bytes.get(i + 1).is_none_or(|&next| next == b' ' || next == b')' || next == b'\n'),
			_ => false,
		})
		.unwrap_or(bytes.len());
	let after = text.get(end + 1..).unwrap_or_default();
	(text[..end].to_owned(), after)
}

/// The `(line N)` an index entry ends with, and its description without it.
fn line_number(description: &str) -> Option<(usize, String)> {
	let trimmed = description.trim_end();
	let open = trimmed.rfind("(line ")?;
	let number = trimmed[open + "(line ".len()..].strip_suffix(')')?.trim().parse().ok()?;
	Some((number, trimmed[..open].trim_end().to_owned()))
}

struct Reference {
	capital: bool,
	label: String,
	target: Target,
	/// The comma or full stop that ended the node name, which the sentence goes on with.
	trailing: String,
}

const REFERENCE_STARTS: [&str; 2] = ["*Note", "*note"];

/// The first cross-reference in `text`: the text before it, the reference, and the text after it.
fn next_reference(text: &str) -> Option<(&str, Reference, &str)> {
	let (start, capital) = REFERENCE_STARTS
		.iter()
		.filter_map(|word| text.find(word).map(|at| (at, word.starts_with("*N"))))
		.filter(|&(at, _)| text[at + 5..].starts_with(char::is_whitespace))
		.min()?;
	let body = text[start + 5..].trim_start();
	let (label, after_label) = split_label(body)?;
	let (target, trailing, rest) = if let Some(rest) = after_label.strip_prefix(':') {
		(parse_target(&label), String::new(), rest)
	} else {
		let (node, rest) = split_node(after_label.trim_start());
		let consumed = after_label.trim_start().len() - rest.len();
		let terminator = after_label.trim_start()[..consumed].chars().last().filter(|c| matches!(c, '.' | ','));
		// Inside parentheses the full stop only ends the reference, so the sentence doesn't stop there.
		let trailing = match terminator {
			Some('.') if rest.starts_with(')') => String::new(),
			Some(c) => c.to_string(),
			None => String::new(),
		};
		(parse_target(&node), trailing, rest)
	};
	let label = clean_inline(&collapse_whitespace(&unquote(&label)));
	Some((&text[..start], Reference { capital, label, target, trailing }, rest))
}

/// Whether a reference starts in `text` without its name ending in it, so it goes on on the next line.
fn has_unfinished_reference(text: &str) -> bool {
	// Only the keyword followed by a space, or ending the line, so emphasis such as `*Note* that` isn't one.
	let Some(start) = REFERENCE_STARTS
		.iter()
		.filter_map(|word| text.rfind(word))
		.filter(|&at| text[at + 5..].is_empty() || text[at + 5..].starts_with(char::is_whitespace))
		.max()
	else {
		return false;
	};
	let body = &text[start + 5..];
	let Some(colon) = body.find(':') else { return true };
	let after = &body[colon + 1..];
	if after.starts_with(':') {
		return false;
	}
	!after.contains(['.', ','])
}

/// Text without the `\0\b[...\0\b]` markup makeinfo leaves for images and indexes: an image reads as its description, or as nothing.
fn clean_inline(text: &str) -> String {
	const OPEN: &str = "\0\u{8}[";
	const CLOSE: &str = "\0\u{8}]";
	let mut out = String::with_capacity(text.len());
	let mut rest = text;
	while let Some(start) = rest.find(OPEN) {
		out.push_str(&rest[..start]);
		let inner_start = start + OPEN.len();
		let Some(len) = rest[inner_start..].find(CLOSE) else {
			rest = &rest[inner_start..];
			continue;
		};
		let inner = &rest[inner_start..inner_start + len];
		if let Some(image) = inner.strip_prefix("image") {
			let alt = attribute(image, "alt").or_else(|| attribute(image, "text"));
			if let Some(alt) = alt.filter(|alt| !alt.trim().is_empty()) {
				out.push_str(alt.trim());
			}
		}
		rest = &rest[inner_start + len + CLOSE.len()..];
	}
	out.push_str(rest);
	out
}

/// The value of `name="..."` in an image's markup, where a backslash escapes a quote.
fn attribute(markup: &str, name: &str) -> Option<String> {
	let start = markup.find(&format!("{name}=\""))? + name.len() + 2;
	let mut value = String::new();
	let mut chars = markup[start..].chars();
	while let Some(c) = chars.next() {
		match c {
			'\\' => value.push(chars.next()?),
			'"' => return Some(value),
			_ => value.push(c),
		}
	}
	None
}
