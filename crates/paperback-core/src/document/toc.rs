//! [`TocItem`]: one entry in a document's table of contents, as built by each parser from
//! whatever structure it has (an EPUB nav document, a PDF's bookmark outline, headings detected
//! by font size, ...).

#[derive(Debug, Clone)]
pub struct TocItem {
	pub name: String,
	pub reference: String,
	pub offset: usize,
	pub children: Vec<Self>,
}

impl TocItem {
	#[must_use]
	pub const fn new(name: String, reference: String, offset: usize) -> Self {
		Self { name, reference, offset, children: Vec::new() }
	}
}
