use anyhow::{Context, Result};
use roxmltree::{Document as XmlDocument, Node};

use super::package::is_element;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum FragmentKind {
	Header,
	Content,
	Footer,
}

#[derive(Debug)]
pub(super) enum Block {
	Paragraph(Vec<String>),
	/// Each item is the blocks inside it.
	List(Vec<Vec<Block>>),
	/// Rows of cells, each cell the blocks inside it.
	Table(Vec<Vec<Vec<Block>>>),
}

#[derive(Debug)]
pub(super) struct Fragment {
	pub kind: FragmentKind,
	pub blocks: Vec<Block>,
}

fn blocks_in(parent: Node) -> Vec<Block> {
	let mut blocks = Vec::new();
	read_blocks(parent, &mut blocks);
	blocks
}

fn blocks_of_each(parent: Node, element: &str) -> Vec<Vec<Block>> {
	parent.children().filter(|child| is_element(*child, element)).map(blocks_in).collect()
}

fn read_rows(parent: Node, rows: &mut Vec<Vec<Vec<Block>>>) {
	for element in parent.children().filter(Node::is_element) {
		match element.tag_name().name() {
			"TableRowStructure" => rows.push(blocks_of_each(element, "TableCellStructure")),
			"TableRowGroupStructure" => read_rows(element, rows),
			_ => {}
		}
	}
}

fn read_blocks(parent: Node, blocks: &mut Vec<Block>) {
	for element in parent.children().filter(Node::is_element) {
		match element.tag_name().name() {
			"ParagraphStructure" => {
				let elements = element.descendants().filter(|node| is_element(*node, "NamedElement"));
				let names = elements.filter_map(|named| named.attribute("NameReference")).map(String::from).collect();
				blocks.push(Block::Paragraph(names));
			}
			"NamedElement" => {
				blocks.extend(element.attribute("NameReference").map(|name| Block::Paragraph(vec![name.to_string()])));
			}
			"ListStructure" => blocks.push(Block::List(blocks_of_each(element, "ListItemStructure"))),
			"TableStructure" => {
				let mut rows = Vec::new();
				read_rows(element, &mut rows);
				blocks.push(Block::Table(rows));
			}
			_ => read_blocks(element, blocks),
		}
	}
}

pub(super) fn read_fragments(xml: &str) -> Result<Vec<Fragment>> {
	let parsed = XmlDocument::parse(xml).context("Failed to parse XPS story fragments")?;
	let elements = parsed.descendants().filter(|node| is_element(*node, "StoryFragment"));
	Ok(elements
		.map(|fragment| {
			let kind = match fragment.attribute("FragmentType") {
				Some("Header") => FragmentKind::Header,
				Some("Footer") => FragmentKind::Footer,
				_ => FragmentKind::Content,
			};
			let mut blocks = Vec::new();
			read_blocks(fragment, &mut blocks);
			Fragment { kind, blocks }
		})
		.collect())
}

#[cfg(test)]
mod tests {
	use super::*;

	fn names(block: &Block) -> &[String] {
		match block {
			Block::Paragraph(names) => names,
			other => panic!("expected a paragraph, found {other:?}"),
		}
	}

	fn only_block(xml: &str) -> Block {
		let mut fragments =
			read_fragments(&format!("<StoryFragments><StoryFragment>{xml}</StoryFragment></StoryFragments>"))
				.expect("fragments");
		fragments.remove(0).blocks.remove(0)
	}

	#[test]
	fn a_list_keeps_its_items_and_their_paragraphs() {
		let block = only_block(
			r#"<ListStructure>
				<ListItemStructure>
					<ParagraphStructure><NamedElement NameReference="bullet"/></ParagraphStructure>
					<ParagraphStructure><NamedElement NameReference="text"/></ParagraphStructure>
				</ListItemStructure>
				<ListItemStructure><ParagraphStructure><NamedElement NameReference="other"/></ParagraphStructure></ListItemStructure>
			</ListStructure>"#,
		);
		let Block::List(items) = block else { panic!("expected a list") };
		assert_eq!(items.len(), 2);
		assert_eq!(names(&items[0][1]), ["text"]);
		assert_eq!(names(&items[1][0]), ["other"]);
	}

	#[test]
	fn a_list_can_sit_inside_a_list_item() {
		let block = only_block(
			r#"<ListStructure><ListItemStructure>
				<ParagraphStructure><NamedElement NameReference="outer"/></ParagraphStructure>
				<ListStructure><ListItemStructure><ParagraphStructure><NamedElement NameReference="inner"/></ParagraphStructure></ListItemStructure></ListStructure>
			</ListItemStructure></ListStructure>"#,
		);
		let Block::List(items) = block else { panic!("expected a list") };
		assert!(matches!(items[0][1], Block::List(_)));
	}

	#[test]
	fn a_table_keeps_its_rows_and_cells() {
		let cell = |name: &str| {
			format!(
				r#"<TableCellStructure><ParagraphStructure><NamedElement NameReference="{name}"/></ParagraphStructure></TableCellStructure>"#
			)
		};
		let block = only_block(&format!(
			"<TableStructure><TableRowGroupStructure><TableRowStructure>{}{}</TableRowStructure><TableRowStructure>{}</TableRowStructure></TableRowGroupStructure></TableStructure>",
			cell("a"),
			cell("b"),
			cell("c")
		));
		let Block::Table(rows) = block else { panic!("expected a table") };
		assert_eq!(rows.iter().map(Vec::len).collect::<Vec<_>>(), [2, 1]);
		assert_eq!(names(&rows[0][1][0]), ["b"]);
		assert_eq!(names(&rows[1][0][0]), ["c"]);
	}

	#[test]
	fn a_paragraph_lists_its_named_elements_in_order() {
		let xml = r#"<StoryFragments xmlns="http://schemas.microsoft.com/xps/2005/06/documentstructure">
			<StoryFragment FragmentType="Content">
				<ParagraphStructure><NamedElement NameReference="a"/><NamedElement NameReference="b"/></ParagraphStructure>
				<ParagraphStructure><NamedElement NameReference="c"/></ParagraphStructure>
			</StoryFragment>
		</StoryFragments>"#;
		let fragments = read_fragments(xml).expect("fragments");
		assert_eq!(fragments.len(), 1);
		assert_eq!(fragments[0].kind, FragmentKind::Content);
		assert_eq!(names(&fragments[0].blocks[0]), ["a", "b"]);
		assert_eq!(names(&fragments[0].blocks[1]), ["c"]);
	}

	#[test]
	fn the_fragment_type_is_kept() {
		let xml = r#"<StoryFragments>
			<StoryFragment FragmentType="Header"/>
			<StoryFragment FragmentType="Content"/>
			<StoryFragment FragmentType="Footer"/>
			<StoryFragment/>
		</StoryFragments>"#;
		let kinds: Vec<_> = read_fragments(xml).expect("fragments").into_iter().map(|fragment| fragment.kind).collect();
		assert_eq!(kinds, [FragmentKind::Header, FragmentKind::Content, FragmentKind::Footer, FragmentKind::Content]);
	}

	#[test]
	fn a_story_break_is_not_content() {
		let xml = r#"<StoryFragments><StoryFragment>
			<StoryBreak/>
			<ParagraphStructure><NamedElement NameReference="a"/></ParagraphStructure>
			<StoryBreak/>
		</StoryFragment></StoryFragments>"#;
		assert_eq!(read_fragments(xml).expect("fragments")[0].blocks.len(), 1);
	}

	#[test]
	fn containers_are_walked_for_their_contents() {
		let xml = r#"<StoryFragments><StoryFragment>
			<SectionStructure>
				<FigureStructure><ParagraphStructure><NamedElement NameReference="a"/></ParagraphStructure></FigureStructure>
				<NamedElement NameReference="b"/>
			</SectionStructure>
		</StoryFragment></StoryFragments>"#;
		let fragments = read_fragments(xml).expect("fragments");
		assert_eq!(names(&fragments[0].blocks[0]), ["a"]);
		assert_eq!(names(&fragments[0].blocks[1]), ["b"]);
	}

	#[test]
	fn a_part_that_is_not_xml_is_an_error() {
		assert!(read_fragments("<StoryFragments").is_err());
	}
}
