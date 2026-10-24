//! A tagged PDF's structure tree: the author's own account of what the page means.
//!
//! pdfium reaches a page's elements through the tree root's `/ParentTree`, so it loses the whole tree of a PDF exported without one, as Apple Pages exports are. Here the whole tree is read once from `/StructTreeRoot` down, and each element records which pages its content lands on, so the parent tree is never consulted. What a page sees is still what pdfium shows it: the root's children by position, an element only where its content is on the page, and a marked-content reference only where it names the page.

use std::collections::HashMap;

use hayro_syntax::object::{Dict, Name, Object, ObjectIdentifier};

use super::{PdfDocument, decode_text_string};

/// How deep the tree is followed. A tree this deep is a malformed file rather than a document's structure.
const MAX_DEPTH: u32 = 64;

/// The document's structure tree, flattened into owned nodes so that pages can share it.
pub(super) struct Structure {
	roots: Vec<Kid>,
	nodes: Vec<Node>,
}

struct Node {
	kind: Option<String>,
	actual_text: Option<String>,
	alt_text: Option<String>,
	kids: Vec<Kid>,
	/// Every page some content under this element lands on, sorted.
	pages: Vec<i32>,
}

enum Kid {
	Element(usize),
	/// A marked-content id, bare or in a reference dictionary. pdfium reports it only on the page it names: the reference's own `/Pg`, else the element's own, never an ancestor's.
	Content {
		mcid: i32,
		page: Option<i32>,
	},
	Other,
}

impl Structure {
	/// Reads the structure tree, or `None` when the document does not say it is tagged, which is pdfium's test too.
	pub(super) fn read(document: &PdfDocument, catalog: &Dict<'_>) -> Option<Self> {
		let marked = catalog.get::<Dict<'_>>(b"MarkInfo").is_some_and(|info| {
			info.get::<bool>(b"Marked").unwrap_or(false) || info.get::<i32>(b"Marked").is_some_and(|value| value != 0)
		});
		if !marked {
			return None;
		}
		let mut reader =
			Reader { document, role_map: None, nodes: Vec::new(), by_id: HashMap::new(), direct_content: Vec::new() };
		let Some(root) = catalog.get::<Dict<'_>>(b"StructTreeRoot") else {
			return Some(Self { roots: Vec::new(), nodes: Vec::new() });
		};
		reader.role_map = root.get::<Dict<'_>>(b"RoleMap");
		let roots = match root.get::<Object<'_>>(b"K") {
			Some(Object::Array(kids)) => {
				kids.iter::<Object<'_>>().map(|kid| reader.kid(kid, None, None, 0).0).collect()
			}
			Some(kid) => vec![reader.kid(kid, None, None, 0).0],
			None => Vec::new(),
		};
		let mut nodes = std::mem::take(&mut reader.nodes);
		let direct = std::mem::take(&mut reader.direct_content);
		let mut state = vec![Visit::Unvisited; nodes.len()];
		for index in 0..nodes.len() {
			gather_pages(index, &mut nodes, &direct, &mut state);
		}
		Some(Self { roots, nodes })
	}
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Visit {
	Unvisited,
	InProgress,
	Done,
}

/// Fills in each element's pages from its own content and its children's, guarding against a tree that loops.
fn gather_pages(index: usize, nodes: &mut [Node], direct: &[Vec<i32>], state: &mut [Visit]) {
	if state[index] != Visit::Unvisited {
		return;
	}
	state[index] = Visit::InProgress;
	let mut pages = direct[index].clone();
	let children: Vec<usize> = nodes[index]
		.kids
		.iter()
		.filter_map(|kid| if let Kid::Element(child) = kid { Some(*child) } else { None })
		.collect();
	for child in children {
		gather_pages(child, nodes, direct, state);
		pages.extend_from_slice(&nodes[child].pages);
	}
	pages.sort_unstable();
	pages.dedup();
	nodes[index].pages = pages;
	state[index] = Visit::Done;
}

struct Reader<'d, 'a> {
	document: &'d PdfDocument,
	role_map: Option<Dict<'a>>,
	nodes: Vec<Node>,
	by_id: HashMap<ObjectIdentifier, usize>,
	/// The pages each element's own marked content lands on, before its children's are added.
	direct_content: Vec<Vec<i32>>,
}

impl<'a> Reader<'_, 'a> {
	/// One kid of an element whose own `/Pg` is `own_page` and whose nearest page is `page`. The reported page follows pdfium; the nearest one only decides which pages an element is on.
	fn kid(&mut self, kid: Object<'a>, own_page: Option<i32>, page: Option<i32>, depth: u32) -> (Kid, Option<i32>) {
		match kid {
			Object::Number(number) => {
				(Kid::Content { mcid: number.as_i64().try_into().unwrap_or(-1), page: own_page }, page)
			}
			Object::Dict(dict) => self.dict_kid(&dict, own_page, page, depth),
			_ => (Kid::Other, None),
		}
	}

	fn dict_kid(
		&mut self,
		dict: &Dict<'a>,
		own_page: Option<i32>,
		page: Option<i32>,
		depth: u32,
	) -> (Kid, Option<i32>) {
		let named = self.page_of(dict);
		match dict.get::<Name<'_>>(b"Type").as_deref() {
			Some(b"MCR") => {
				let mcid = dict.get::<i32>(b"MCID").unwrap_or(-1);
				(Kid::Content { mcid, page: named.or(own_page) }, named.or(page))
			}
			Some(b"OBJR") => (Kid::Other, None),
			_ => (self.element(dict, named.or(page), depth).map_or(Kid::Other, Kid::Element), None),
		}
	}

	fn element(&mut self, dict: &Dict<'a>, inherited_page: Option<i32>, depth: u32) -> Option<usize> {
		if depth >= MAX_DEPTH {
			return None;
		}
		if let Some(id) = dict.obj_id()
			&& let Some(&index) = self.by_id.get(&id)
		{
			return Some(index);
		}
		let index = self.nodes.len();
		if let Some(id) = dict.obj_id() {
			self.by_id.insert(id, index);
		}
		self.nodes.push(Node {
			kind: self.kind(dict),
			actual_text: text_field(dict, b"ActualText"),
			alt_text: text_field(dict, b"Alt"),
			kids: Vec::new(),
			pages: Vec::new(),
		});
		self.direct_content.push(Vec::new());
		let own_page = self.page_of(dict);
		let page = own_page.or(inherited_page);
		let read: Vec<(Kid, Option<i32>)> = match dict.get::<Object<'a>>(b"K") {
			Some(Object::Array(kids)) => {
				kids.iter::<Object<'a>>().map(|kid| self.kid(kid, own_page, page, depth + 1)).collect()
			}
			Some(kid) => vec![self.kid(kid, own_page, page, depth + 1)],
			None => Vec::new(),
		};
		let (kids, pages): (Vec<Kid>, Vec<Option<i32>>) = read.into_iter().unzip();
		self.direct_content[index] = pages.into_iter().flatten().collect();
		self.nodes[index].kids = kids;
		Some(index)
	}

	/// The structure type, through the role map once, as pdfium resolves it.
	fn kind(&self, dict: &Dict<'a>) -> Option<String> {
		let name = dict.get::<Name<'_>>(b"S")?;
		let mapped = self.role_map.as_ref().and_then(|map| map.get::<Name<'_>>(name.as_ref()));
		Some(mapped.unwrap_or(name).as_str().to_string())
	}

	fn page_of(&self, dict: &Dict<'a>) -> Option<i32> {
		self.document.page_number(dict.get_ref(b"Pg")?.into())
	}
}

fn text_field(dict: &Dict<'_>, key: &[u8]) -> Option<String> {
	dict.get::<hayro_syntax::object::String<'_>>(key)
		.map(|text| decode_text_string(text.as_bytes()))
		.filter(|text| !text.is_empty())
}

/// A page's view of the structure tree.
pub struct TagTree<'page> {
	structure: &'page Structure,
	page: i32,
}

impl<'page> TagTree<'page> {
	pub(super) const fn of(structure: &'page Structure, page: i32) -> Self {
		Self { structure, page }
	}

	pub fn child_count(&self) -> i32 {
		i32::try_from(self.structure.roots.len()).unwrap_or(i32::MAX)
	}

	/// The top-level tag at `index`, or `None` when there is no tag there or none of its content is on this page.
	pub fn child(&self, index: i32) -> Option<Tag<'page>> {
		element_on_page(self.structure, self.structure.roots.get(usize::try_from(index).ok()?)?, self.page)
	}
}

fn element_on_page<'page>(structure: &'page Structure, kid: &Kid, page: i32) -> Option<Tag<'page>> {
	let Kid::Element(index) = kid else { return None };
	let node = &structure.nodes[*index];
	node.pages.binary_search(&page).is_ok().then_some(Tag { structure, node, page })
}

/// One element of the structure tree, as one page sees it.
pub struct Tag<'tree> {
	structure: &'tree Structure,
	node: &'tree Node,
	page: i32,
}

impl Tag<'_> {
	/// The structure type, as the PDF spells it after its role map: `H1`, `P`, `LI`, `Figure`, `TD` and so on.
	pub fn kind(&self) -> Option<String> {
		self.node.kind.clone()
	}

	/// Text the author supplied to be read in place of this element's glyphs.
	pub fn actual_text(&self) -> Option<String> {
		self.node.actual_text.clone()
	}

	/// The alternative description, which is what a tagged figure carries instead of text.
	pub fn alt_text(&self) -> Option<String> {
		self.node.alt_text.clone()
	}

	pub fn child_count(&self) -> i32 {
		i32::try_from(self.node.kids.len()).unwrap_or(i32::MAX)
	}

	/// The child tag at `index`, or `None` when that child is marked content rather than a tag, or a tag with nothing on this page.
	pub fn child(&self, index: i32) -> Option<Tag<'_>> {
		element_on_page(self.structure, self.node.kids.get(usize::try_from(index).ok()?)?, self.page)
	}

	/// The marked-content id of the child at `index`. `None` when that child is a tag, carries no id, or is a reference to another page.
	pub fn child_mcid(&self, index: i32) -> Option<i32> {
		let Kid::Content { mcid, page } = self.node.kids.get(usize::try_from(index).ok()?)? else { return None };
		let id = if *page == Some(self.page) { *mcid } else { return None };
		(id >= 0).then_some(id)
	}
}
