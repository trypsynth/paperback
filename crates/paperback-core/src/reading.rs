//! The reading screen's view of a document: every paragraph, heading, image, list item and table row as a unit, and the units grouped into the blocks a screen reader moves between, a list or table being one block.
//!
//! Only where each unit sits is kept; its text, kind and links are read from the buffer when asked for, so a book of millions of lines costs one number a line.

use std::sync::Arc;

use crate::{
	document::{DocLen, DocPos, DocumentBuffer, MarkerType},
	parser::convert::table_text::{TableGrid, html_table_grid},
	session::DocumentSession,
};

#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadingUnitKind {
	Paragraph,
	Heading,
	Image,
	ListItem,
	Caption,
	TableRow,
}

#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadingBlockKind {
	Single,
	List,
	Table,
}

#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadingLinkFfi {
	/// What `activate_link` resolves.
	pub position: DocPos,
	/// In chars of the unit's text.
	pub start: i64,
	pub length: i64,
	pub text: String,
}

#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadingUnitFfi {
	pub kind: ReadingUnitKind,
	/// A heading's level, else 0.
	pub level: i32,
	pub text: String,
	/// An image's own description, empty when it has none.
	pub description: String,
	pub position: DocPos,
	pub block: i64,
	/// A caption's or row's table and a row's index in it, else -1.
	pub table: i64,
	pub row: i64,
	pub links: Vec<ReadingLinkFfi>,
}

#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadingBlockFfi {
	pub kind: ReadingBlockKind,
	pub first_unit: i64,
	pub unit_count: i64,
	pub table: i64,
}

#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadingCellFfi {
	pub text: String,
	pub is_header: bool,
	pub row: i64,
	pub column: i64,
	pub row_span: i64,
	pub column_span: i64,
}

#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadingTableFfi {
	pub caption: String,
	pub cells: Vec<ReadingCellFfi>,
	pub row_count: i64,
	pub column_count: i64,
	pub header_row_count: i64,
	/// Row r is unit `first_unit + r`.
	pub first_unit: i64,
	pub block: i64,
}

#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadingLinkStopFfi {
	pub found: bool,
	pub unit: i64,
	/// Its index among the unit's links.
	pub link: i64,
}

struct Table {
	start: DocPos,
	end: DocPos,
	first_unit: usize,
	has_caption: bool,
	/// Whether each row has a line of its own, as when tables are rendered inline, rather than sharing the placeholder's.
	rows_have_lines: bool,
	grid: TableGrid,
}

/// A run of units read as one block. Units outside every group are a block each.
struct Group {
	first_unit: usize,
	len: usize,
	table: Option<usize>,
	/// How many fewer blocks than units come before this group.
	collapsed_before: usize,
}

#[cfg_attr(feature = "uniffi", derive(uniffi::Object))]
pub struct ReadingLayout {
	session: Arc<DocumentSession>,
	/// Each unit's 0-based buffer line, in order.
	lines: Vec<usize>,
	tables: Vec<Table>,
	groups: Vec<Group>,
	block_count: usize,
	headings: Vec<(DocPos, i32)>,
	images: Vec<usize>,
	/// Link markers outside tables, whose rows carry no links.
	links: Vec<usize>,
	/// The outermost lists' spans, nested ones included in them.
	lists: Vec<(DocPos, DocPos)>,
	/// Every table marker's span, including those read as plain lines.
	table_spans: Vec<(DocPos, DocPos)>,
}

fn line_bounds(buf: &DocumentBuffer, line: usize) -> (DocPos, DocPos) {
	let newlines = buf.newline_positions();
	let start = if line == 0 { DocPos::ZERO } else { newlines[line - 1] + DocLen::ONE };
	(start, newlines.get(line).copied().unwrap_or_else(|| buf.end()))
}

fn line_of(buf: &DocumentBuffer, position: DocPos) -> usize {
	buf.newline_positions().partition_point(|&newline| newline < position)
}

const fn heading_level(mtype: MarkerType) -> Option<i32> {
	match mtype {
		MarkerType::Heading1 => Some(1),
		MarkerType::Heading2 => Some(2),
		MarkerType::Heading3 => Some(3),
		MarkerType::Heading4 => Some(4),
		MarkerType::Heading5 => Some(5),
		MarkerType::Heading6 => Some(6),
		_ => None,
	}
}

fn to_i64(value: usize) -> i64 {
	i64::try_from(value).unwrap_or(i64::MAX)
}

fn to_index(value: i64) -> usize {
	usize::try_from(value).unwrap_or(0)
}

/// A table of one cell reads better as the paragraph it is.
fn is_real_table(grid: &TableGrid) -> bool {
	grid.row_count > 1 || grid.column_count > 1
}

impl ReadingLayout {
	#[must_use]
	pub fn build(session: Arc<DocumentSession>) -> Self {
		let buf = &session.handle().document().buffer;
		let mut headings = Vec::new();
		let mut images = Vec::new();
		let mut links = Vec::new();
		let mut lists: Vec<(DocPos, DocPos)> = Vec::new();
		let mut table_markers: Vec<(DocPos, DocPos, usize)> = Vec::new();
		for (index, marker) in buf.markers.iter().enumerate() {
			let end = marker.position + marker.length;
			if let Some(level) = heading_level(marker.mtype) {
				headings.push((marker.position, level));
				continue;
			}
			match marker.mtype {
				MarkerType::Image | MarkerType::Figure => images.push(index),
				MarkerType::Link => links.push(index),
				MarkerType::List => match lists.last_mut() {
					Some(last) if marker.position < last.1 => last.1 = last.1.max(end),
					_ => lists.push((marker.position, end)),
				},
				MarkerType::Table if table_markers.last().is_none_or(|last| marker.position >= last.1) => {
					table_markers.push((marker.position, end, index));
				}
				_ => {}
			}
		}
		links.retain(|&index| {
			let position = buf.markers[index].position;
			let after = table_markers.partition_point(|span| span.0 <= position);
			after == 0 || position >= table_markers[after - 1].1
		});
		let line_count = buf.newline_positions().len() + 1;
		let mut lines = Vec::new();
		let mut tables = Vec::new();
		let mut groups: Vec<Group> = Vec::new();
		// The list being read and its group, so a list right after another one starts a group of its own.
		let mut open_list: Option<(usize, usize)> = None;
		let mut table_cursor = 0;
		let mut list_cursor = 0;
		let mut line = 0;
		while line < line_count {
			let (start, end) = line_bounds(buf, line);
			while table_markers.get(table_cursor).is_some_and(|span| span.0 < start && span.1 <= start) {
				table_cursor += 1;
			}
			if let Some(&(table_start, table_end, marker)) = table_markers.get(table_cursor)
				&& table_start <= end
			{
				table_cursor += 1;
				let last_line =
					line_of(buf, if table_end > table_start { table_end - DocLen::ONE } else { table_start }).max(line);
				if let Some(grid) = html_table_grid(&buf.markers[marker].reference).filter(is_real_table) {
					open_list = None;
					let has_caption = !grid.caption.is_empty();
					if has_caption {
						lines.push(line);
					}
					let first_unit = lines.len();
					let rows_have_lines = last_line - line + 1 >= grid.row_count;
					lines.extend((0..grid.row_count).map(|row| if rows_have_lines { line + row } else { line }));
					groups.push(Group {
						first_unit,
						len: grid.row_count,
						table: Some(tables.len()),
						collapsed_before: 0,
					});
					tables.push(Table {
						start: table_start,
						end: table_end,
						first_unit,
						has_caption,
						rows_have_lines,
						grid,
					});
					line = last_line + 1;
					continue;
				}
			}
			if buf.text(start..end).trim().is_empty() {
				line += 1;
				continue;
			}
			while lists.get(list_cursor).is_some_and(|span| span.1 <= start) {
				list_cursor += 1;
			}
			let unit = lines.len();
			lines.push(line);
			if lists.get(list_cursor).is_some_and(|span| span.0 < end) {
				match open_list {
					Some((list, group))
						if list == list_cursor && groups[group].first_unit + groups[group].len == unit =>
					{
						groups[group].len += 1;
					}
					_ => {
						groups.push(Group { first_unit: unit, len: 1, table: None, collapsed_before: 0 });
						open_list = Some((list_cursor, groups.len() - 1));
					}
				}
			}
			line += 1;
		}
		let mut collapsed = 0;
		for group in &mut groups {
			group.collapsed_before = collapsed;
			collapsed += group.len - 1;
		}
		let block_count = lines.len() - collapsed;
		let table_spans = table_markers.iter().map(|&(start, end, _)| (start, end)).collect();
		Self { session, lines, tables, groups, block_count, headings, images, links, lists, table_spans }
	}

	fn buffer(&self) -> &DocumentBuffer {
		&self.session.handle().document().buffer
	}

	fn table_of_unit(&self, unit: usize) -> Option<usize> {
		let after = self.tables.partition_point(|table| table.first_unit <= unit);
		let candidate =
			if after < self.tables.len() && self.tables[after].has_caption && self.tables[after].first_unit == unit + 1
			{
				after
			} else {
				after.checked_sub(1)?
			};
		let table = &self.tables[candidate];
		let first = table.first_unit - usize::from(table.has_caption);
		(first <= unit && unit < table.first_unit + table.grid.row_count).then_some(candidate)
	}

	fn block_index_of_unit(&self, unit: usize) -> usize {
		let after = self.groups.partition_point(|group| group.first_unit <= unit);
		let Some(group) = after.checked_sub(1).map(|index| &self.groups[index]) else { return unit };
		if unit < group.first_unit + group.len {
			group.first_unit - group.collapsed_before
		} else {
			unit - group.collapsed_before - (group.len - 1)
		}
	}

	fn position_of_unit(&self, unit: usize) -> DocPos {
		let buf = self.buffer();
		let Some(&line) = self.lines.get(unit) else { return buf.end() };
		let Some(index) = self.table_of_unit(unit) else { return line_bounds(buf, line).0 };
		let table = &self.tables[index];
		if unit < table.first_unit {
			return table.start;
		}
		if table.rows_have_lines {
			return line_bounds(buf, line).0;
		}
		// The rows share the placeholder's one line, so each takes its own char of it, the last ones sharing its last.
		let offset = (unit - table.first_unit + usize::from(table.has_caption))
			.min(table.end.since(table.start).get().saturating_sub(1));
		table.start + DocLen::from_raw(offset)
	}

	fn unit_index_at(&self, position: DocPos) -> usize {
		let buf = self.buffer();
		let line = line_of(buf, position.min(buf.end()));
		let after = self.lines.partition_point(|&unit_line| unit_line < line);
		if self.lines.get(after) != Some(&line) {
			return after.saturating_sub(1).min(self.lines.len().saturating_sub(1));
		}
		let mut unit = after;
		while self.lines.get(unit + 1) == Some(&line) && self.position_of_unit(unit + 1) <= position {
			unit += 1;
		}
		unit
	}

	fn unit_ffi(&self, unit: usize) -> ReadingUnitFfi {
		let block = to_i64(self.block_index_of_unit(unit));
		let position = self.position_of_unit(unit);
		if let Some(index) = self.table_of_unit(unit) {
			let table = &self.tables[index];
			let (kind, text, row) = if unit < table.first_unit {
				(ReadingUnitKind::Caption, table.grid.caption.clone(), -1)
			} else {
				let row = unit - table.first_unit;
				(ReadingUnitKind::TableRow, table.grid.row_text(row), to_i64(row))
			};
			let table = if kind == ReadingUnitKind::Caption { -1 } else { to_i64(index) };
			return ReadingUnitFfi {
				kind,
				level: 0,
				text,
				description: String::new(),
				position,
				block,
				table,
				row,
				links: Vec::new(),
			};
		}
		let buf = self.buffer();
		let (start, end) = line_bounds(buf, self.lines[unit]);
		let raw = buf.text(start..end);
		let mut text = raw.trim();
		let leading = DocLen::of(&raw[..raw.len() - raw.trim_start().len()]);
		let content_start = start + leading;
		let in_table = {
			let after = self.table_spans.partition_point(|span| span.0 <= end);
			after > 0 && self.table_spans[after - 1].1 > start
		};
		if in_table {
			text = text.strip_prefix("[Table]:").map_or(text, str::trim_start);
		}
		let mut kind = ReadingUnitKind::Paragraph;
		let mut level = 0;
		let mut description = String::new();
		let heading = self.headings.partition_point(|heading| heading.0 < start);
		if let Some(&(heading_position, heading_level)) = self.headings.get(heading)
			&& heading_position < end
		{
			kind = ReadingUnitKind::Heading;
			level = heading_level;
		} else if let Some(image) = self.image_filling(content_start, end, DocLen::of(text)) {
			kind = ReadingUnitKind::Image;
			description.clone_from(&buf.markers[image].text);
		} else {
			let list = self.lists.partition_point(|span| span.1 <= start);
			if self.lists.get(list).is_some_and(|span| span.0 < end) {
				kind = ReadingUnitKind::ListItem;
			}
		}
		let text_len = DocLen::of(text).get();
		let first_link = self.links.partition_point(|&index| buf.markers[index].position < start);
		let links = self.links[first_link..]
			.iter()
			.map(|&index| &buf.markers[index])
			.take_while(|marker| marker.position < end)
			.filter_map(|marker| {
				let offset = marker.position.get().checked_sub(content_start.get())?;
				let length = if marker.length.is_zero() { DocLen::of(&marker.text).get() } else { marker.length.get() };
				(offset < text_len && length > 0).then(|| ReadingLinkFfi {
					position: marker.position,
					start: to_i64(offset),
					length: to_i64(length.min(text_len - offset)),
					text: marker.text.clone(),
				})
			})
			.collect();
		ReadingUnitFfi { kind, level, text: text.to_string(), description, position, block, table: -1, row: -1, links }
	}

	/// An image marker that is the whole of its line, since the buffer also writes a picture into running text where it stands.
	fn image_filling(&self, content_start: DocPos, end: DocPos, text_len: DocLen) -> Option<usize> {
		let buf = self.buffer();
		let first = self.images.partition_point(|&index| buf.markers[index].position < content_start);
		self.images[first..]
			.iter()
			.copied()
			.take_while(|&index| buf.markers[index].position < end)
			.find(|&index| buf.markers[index].position == content_start && buf.markers[index].length >= text_len)
	}

	fn links_of_unit(&self, unit: usize) -> std::ops::Range<usize> {
		let buf = self.buffer();
		if self.table_of_unit(unit).is_some() {
			return 0..0;
		}
		let (start, end) = line_bounds(buf, self.lines[unit]);
		let first = self.links.partition_point(|&index| buf.markers[index].position < start);
		let last = self.links.partition_point(|&index| buf.markers[index].position < end);
		first..last
	}
}

#[cfg(feature = "uniffi")]
#[uniffi::export]
impl ReadingLayout {
	#[uniffi::constructor]
	pub fn new(session: Arc<DocumentSession>) -> Self {
		Self::build(session)
	}
}

#[cfg_attr(feature = "uniffi", uniffi::export)]
impl ReadingLayout {
	pub fn unit_count(&self) -> i64 {
		to_i64(self.lines.len())
	}

	pub fn block_count(&self) -> i64 {
		to_i64(self.block_count)
	}

	/// Up to `count` units from `start`, fewer past the end.
	pub fn units(&self, start: i64, count: i64) -> Vec<ReadingUnitFfi> {
		let start = to_index(start).min(self.lines.len());
		let end = start.saturating_add(to_index(count)).min(self.lines.len());
		(start..end).map(|unit| self.unit_ffi(unit)).collect()
	}

	pub fn block(&self, index: i64) -> ReadingBlockFfi {
		let index = to_index(index).min(self.block_count.saturating_sub(1));
		let after = self.groups.partition_point(|group| group.first_unit - group.collapsed_before <= index);
		let single = |unit: usize| ReadingBlockFfi {
			kind: ReadingBlockKind::Single,
			first_unit: to_i64(unit),
			unit_count: 1,
			table: -1,
		};
		let Some(group) = after.checked_sub(1).map(|group| &self.groups[group]) else { return single(index) };
		let group_block = group.first_unit - group.collapsed_before;
		if index != group_block {
			return single(index + group.collapsed_before + group.len - 1);
		}
		ReadingBlockFfi {
			kind: if group.table.is_some() { ReadingBlockKind::Table } else { ReadingBlockKind::List },
			first_unit: to_i64(group.first_unit),
			unit_count: to_i64(group.len),
			table: group.table.map_or(-1, to_i64),
		}
	}

	pub fn block_of_unit(&self, unit: i64) -> i64 {
		to_i64(self.block_index_of_unit(to_index(unit)))
	}

	pub fn table(&self, index: i64) -> ReadingTableFfi {
		let Some(table) = self.tables.get(to_index(index)) else {
			return ReadingTableFfi {
				caption: String::new(),
				cells: Vec::new(),
				row_count: 0,
				column_count: 0,
				header_row_count: 0,
				first_unit: -1,
				block: -1,
			};
		};
		let grid = &table.grid;
		let cells = grid
			.cells
			.iter()
			.map(|cell| ReadingCellFfi {
				text: cell.text.clone(),
				is_header: cell.is_header,
				row: to_i64(cell.row),
				column: to_i64(cell.column),
				row_span: to_i64(cell.row_span),
				column_span: to_i64(cell.column_span),
			})
			.collect();
		ReadingTableFfi {
			caption: grid.caption.clone(),
			cells,
			row_count: to_i64(grid.row_count),
			column_count: to_i64(grid.column_count),
			header_row_count: to_i64(grid.header_row_count),
			first_unit: to_i64(table.first_unit),
			block: to_i64(self.block_index_of_unit(table.first_unit)),
		}
	}

	/// The unit holding a position, or the one before it when the position is on a blank line.
	pub fn unit_at_position(&self, position: DocPos) -> i64 {
		to_i64(self.unit_index_at(position))
	}

	pub fn unit_position(&self, unit: i64) -> DocPos {
		self.position_of_unit(to_index(unit))
	}

	pub fn heading_units(&self) -> Vec<i64> {
		self.units_at(self.headings.iter().map(|heading| heading.0))
	}

	/// Each table's first row.
	pub fn table_units(&self) -> Vec<i64> {
		self.tables.iter().map(|table| to_i64(table.first_unit)).collect()
	}

	/// Where each table of contents entry starts.
	pub fn section_units(&self) -> Vec<i64> {
		self.units_at(self.session.get_toc().into_iter().map(|entry| entry.position))
	}

	/// The link after or before link `link` of `unit`, or, with `link` -1, after or before the unit's own text, so its first link counts as next.
	pub fn link_stop(&self, unit: i64, link: i64, forward: bool) -> ReadingLinkStopFfi {
		let not_found = ReadingLinkStopFfi { found: false, unit: -1, link: -1 };
		let unit = to_index(unit);
		if unit >= self.lines.len() {
			return not_found;
		}
		let own = self.links_of_unit(unit);
		let from = match usize::try_from(link) {
			Ok(link) if own.start + link < own.end => Some(own.start + link),
			_ => None,
		};
		let target = match (from, forward) {
			(Some(current), true) => Some(current + 1),
			(Some(current), false) => current.checked_sub(1),
			(None, true) => Some(own.start),
			(None, false) => own.start.checked_sub(1),
		};
		let Some((target, &marker)) = target.and_then(|target| Some((target, self.links.get(target)?))) else {
			return not_found;
		};
		let found = self.unit_index_at(self.buffer().markers[marker].position);
		let index = target - self.links_of_unit(found).start;
		ReadingLinkStopFfi { found: true, unit: to_i64(found), link: to_i64(index) }
	}
}

impl ReadingLayout {
	fn units_at(&self, positions: impl Iterator<Item = DocPos>) -> Vec<i64> {
		let mut units: Vec<i64> = positions.map(|position| to_i64(self.unit_index_at(position))).collect();
		units.sort_unstable();
		units.dedup();
		units
	}
}

#[cfg(test)]
mod tests;
