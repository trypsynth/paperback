//! pdfium reaches a page's elements through `/ParentTree` and so loses the whole tree of a PDF exported without one, as Apple Pages exports are, so here the tree is read from `/StructTreeRoot` down while each page still sees what pdfium would show it.

use std::collections::HashMap;

use hayro_syntax::object::{Dict, Name, Object, ObjectIdentifier};

use super::{PdfDocument, decode_text_string};

/// A tree this deep is a malformed file rather than a document's structure.
const MAX_DEPTH: u32 = 64;

/// Flattened into owned nodes so that pages can share it.
pub(super) struct Structure {
	roots: Vec<Kid>,
	nodes: Vec<Node>,
}

struct Node {
	kind: Option<String>,
	actual_text: Option<String>,
	alt_text: Option<String>,
	kids: Vec<Kid>,
	/// Sorted, since pages are looked up by binary search.
	pages: Vec<usize>,
}

enum Kid {
	Element(usize),
	/// pdfium reports it only on the page it names: the reference's own `/Pg`, else the element's own, never an ancestor's.
	Content {
		mcid: i32,
		page: Option<usize>,
	},
	Other,
}

impl Structure {
	/// `None` unless `/MarkInfo` says the document is tagged, which is pdfium's test too.
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

fn gather_pages(index: usize, nodes: &mut [Node], direct: &[Vec<usize>], state: &mut [Visit]) {
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
	direct_content: Vec<Vec<usize>>,
}

impl<'a> Reader<'_, 'a> {
	/// The reported page (`own_page`) follows pdfium; the nearest one (`page`) only decides which pages an element is on.
	fn kid(
		&mut self,
		kid: Object<'a>,
		own_page: Option<usize>,
		page: Option<usize>,
		depth: u32,
	) -> (Kid, Option<usize>) {
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
		own_page: Option<usize>,
		page: Option<usize>,
		depth: u32,
	) -> (Kid, Option<usize>) {
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

	fn element(&mut self, dict: &Dict<'a>, inherited_page: Option<usize>, depth: u32) -> Option<usize> {
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
		let read: Vec<(Kid, Option<usize>)> = match dict.get::<Object<'a>>(b"K") {
			Some(Object::Array(kids)) => {
				kids.iter::<Object<'a>>().map(|kid| self.kid(kid, own_page, page, depth + 1)).collect()
			}
			Some(kid) => vec![self.kid(kid, own_page, page, depth + 1)],
			None => Vec::new(),
		};
		let (kids, pages): (Vec<Kid>, Vec<Option<usize>>) = read.into_iter().unzip();
		self.direct_content[index] = pages.into_iter().flatten().collect();
		self.nodes[index].kids = kids;
		Some(index)
	}

	/// Through the role map once, as pdfium resolves it.
	fn kind(&self, dict: &Dict<'a>) -> Option<String> {
		let name = dict.get::<Name<'_>>(b"S")?;
		let mapped = self.role_map.as_ref().and_then(|map| map.get::<Name<'_>>(name.as_ref()));
		Some(mapped.unwrap_or(name).as_str().to_string())
	}

	fn page_of(&self, dict: &Dict<'a>) -> Option<usize> {
		self.document.page_number(dict.get_ref(b"Pg")?.into())
	}
}

fn text_field(dict: &Dict<'_>, key: &[u8]) -> Option<String> {
	dict.get::<hayro_syntax::object::String<'_>>(key)
		.map(|text| decode_text_string(text.as_bytes()))
		.filter(|text| !text.is_empty())
}

/// A page's view of the document's structure tree.
pub struct TagTree<'page> {
	structure: &'page Structure,
	page: usize,
}

impl<'page> TagTree<'page> {
	pub(super) const fn of(structure: &'page Structure, page: usize) -> Self {
		Self { structure, page }
	}

	/// Whether the document's tree has no top-level elements at all, on any page.
	pub const fn is_empty(&self) -> bool {
		self.structure.roots.is_empty()
	}

	/// The top-level elements with something on this page.
	pub fn roots(&self) -> impl Iterator<Item = Tag<'page>> + use<'page> {
		let (structure, page) = (self.structure, self.page);
		structure.roots.iter().filter_map(move |kid| element_on_page(structure, kid, page))
	}
}

fn element_on_page<'page>(structure: &'page Structure, kid: &Kid, page: usize) -> Option<Tag<'page>> {
	let Kid::Element(index) = kid else { return None };
	let node = &structure.nodes[*index];
	node.pages.binary_search(&page).is_ok().then_some(Tag { structure, node, page })
}

/// One element of the structure tree, as seen from a page.
pub struct Tag<'tree> {
	structure: &'tree Structure,
	node: &'tree Node,
	page: usize,
}

/// What a structure element holds on the page being read.
pub enum TagChild<'tree> {
	/// A nested element.
	Tag(Tag<'tree>),
	/// The id of a marked content sequence, which the page's objects carry in their marks.
	Content(i32),
}

impl<'tree> Tag<'tree> {
	/// The element's structure type, such as `P`, `H1`, or `Figure`.
	pub fn kind(&self) -> Option<&'tree str> {
		self.node.kind.as_deref()
	}

	pub fn actual_text(&self) -> Option<&'tree str> {
		self.node.actual_text.as_deref()
	}

	pub fn alt_text(&self) -> Option<&'tree str> {
		self.node.alt_text.as_deref()
	}

	/// How many children the element has across the whole document, which a list's size needs even when it runs over several pages.
	pub const fn child_count(&self) -> usize {
		self.node.kids.len()
	}

	/// The children on this page, in order.
	pub fn children(&self) -> impl Iterator<Item = TagChild<'tree>> + use<'tree> {
		let (structure, kids, page) = (self.structure, &self.node.kids, self.page);
		kids.iter().filter_map(move |kid| match kid {
			Kid::Element(_) => element_on_page(structure, kid, page).map(TagChild::Tag),
			Kid::Content { mcid, page: on } => (*on == Some(page) && *mcid >= 0).then_some(TagChild::Content(*mcid)),
			Kid::Other => None,
		})
	}
}
