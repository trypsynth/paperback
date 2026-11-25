//! Whether a `<table>` lays out a page rather than holding data. Old sites put the whole page in a table, the site menu in one cell and the text in another, and read as a data table that's one line with no headings, as #1310 found. A layout table is read as the blocks in its cells instead, as browsers and screen readers read one.
//!
//! The signs are the ones screen readers go by. NVDA, for one, takes a table for data when it has a caption, header cells, head or foot rows, column groups, a `summary`, or cells naming their headers. Paperback stays a step more careful, since a table that is read as text loses its table view: without those signs a table is data unless it shows it is layout, by a role saying so, a table or a heading, quotation or form in a cell, or a single row or column. A list in a cell is no sign: data tables hold them, such as a changelog's list of fixes for each version.

/// What the check needs of an element, from either document tree the converters read.
pub(crate) trait TableElement: Copy {
	/// The element's local name, `None` for anything that isn't an element.
	fn tag(self) -> Option<String>;
	fn attr(self, name: &str) -> Option<String>;
	fn children(self) -> Vec<Self>;
}

#[must_use]
pub(crate) fn is_layout_table<E: TableElement>(table: E) -> bool {
	match table.attr("role").map(|role| role.trim().to_ascii_lowercase()).as_deref() {
		Some("presentation" | "none") => return true,
		Some("table" | "grid" | "treegrid") => return false,
		_ => {}
	}
	if table.attr("summary").is_some_and(|summary| !summary.trim().is_empty()) {
		return false;
	}
	let mut data = false;
	let mut layout = false;
	let mut rows = 0;
	let mut columns = 0;
	// The table's own content: a table inside it is a sign of layout, but what that table holds is its own.
	let mut pending = table.children();
	while let Some(element) = pending.pop() {
		let Some(tag) = element.tag() else { continue };
		match tag.as_str() {
			"table" => {
				layout = true;
				continue;
			}
			"caption" | "thead" | "tfoot" | "th" | "colgroup" => data = true,
			"td" if element.attr("headers").is_some() || element.attr("scope").is_some() => data = true,
			"tr" => {
				rows += 1;
				let cells = element
					.children()
					.into_iter()
					.filter(|cell| matches!(cell.tag().as_deref(), Some("td" | "th")))
					.count();
				columns = columns.max(cells);
			}
			"h1" | "h2" | "h3" | "h4" | "h5" | "h6" | "blockquote" | "form" => layout = true,
			_ => {}
		}
		pending.extend(element.children());
	}
	!data && (layout || rows <= 1 || columns <= 1)
}

impl TableElement for ego_tree::NodeRef<'_, scraper::Node> {
	fn tag(self) -> Option<String> {
		self.value().as_element().map(|element| element.name().to_ascii_lowercase())
	}

	fn attr(self, name: &str) -> Option<String> {
		self.value().as_element()?.attr(name).map(str::to_string)
	}

	fn children(self) -> Vec<Self> {
		ego_tree::NodeRef::children(&self).collect()
	}
}

impl TableElement for roxmltree::Node<'_, '_> {
	fn tag(self) -> Option<String> {
		self.is_element().then(|| self.tag_name().name().to_ascii_lowercase())
	}

	fn attr(self, name: &str) -> Option<String> {
		self.attribute(name).map(str::to_string)
	}

	fn children(self) -> Vec<Self> {
		roxmltree::Node::children(&self).collect()
	}
}

#[cfg(test)]
mod tests {
	use scraper::{Html, Selector};

	use super::*;

	fn layout(html: &str) -> bool {
		let document = Html::parse_document(html);
		let table = document.select(&Selector::parse("table").expect("a selector")).next().expect("a table");
		is_layout_table(*table)
	}

	#[test]
	fn a_page_laid_out_in_a_table_is_layout() {
		assert!(layout(
			"<table><tr><td><ul><li><a href=a>Home</a></li></ul></td><td><h1>Chapter One</h1><p>Text.</p></td></tr></table>"
		));
	}

	#[test]
	fn a_grid_of_values_is_data() {
		assert!(!layout("<table><tr><td>1</td><td>2</td></tr><tr><td>3</td><td>4</td></tr></table>"));
	}

	#[test]
	fn paragraphs_in_cells_alone_keep_a_table_data() {
		assert!(!layout(
			"<table><tr><td><p>a</p></td><td><p>b</p></td></tr><tr><td><p>c</p></td><td><p>d</p></td></tr></table>"
		));
	}

	#[test]
	fn lists_in_cells_keep_a_table_data() {
		assert!(!layout(
			"<table><tr><td>1.0</td><td><ol><li>a fix</li></ol></td></tr><tr><td>1.1</td><td><ol><li>another</li></ol></td></tr></table>"
		));
	}

	#[test]
	fn header_cells_make_a_table_data_whatever_its_cells_hold() {
		assert!(!layout("<table><tr><th>Name</th></tr><tr><td><h2>a</h2></td></tr></table>"));
		assert!(!layout("<table summary='Prices'><tr><td>1</td></tr></table>"));
	}

	#[test]
	fn a_role_decides() {
		assert!(layout("<table role=presentation><tr><td>1</td><td>2</td></tr><tr><td>3</td><td>4</td></tr></table>"));
		assert!(!layout("<table role=table><tr><td><h2>a</h2></td></tr></table>"));
	}

	#[test]
	fn a_single_row_or_column_is_layout() {
		assert!(layout("<table><tr><td>Menu</td><td>Text</td></tr></table>"));
		assert!(layout("<table><tr><td>One</td></tr><tr><td>Two</td></tr></table>"));
	}

	#[test]
	fn a_data_table_inside_doesnt_make_the_outer_one_data() {
		assert!(layout("<table><tr><td>Menu</td><td><table><tr><th>A</th></tr></table></td></tr></table>"));
	}
}
