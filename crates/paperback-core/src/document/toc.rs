use super::position::DocPos;

#[derive(Debug, Clone)]
pub struct TocItem {
	pub name: String,
	pub reference: String,
	pub offset: DocPos,
	pub children: Vec<Self>,
}

impl TocItem {
	#[must_use]
	pub const fn new(name: String, reference: String, offset: DocPos) -> Self {
		Self { name, reference, offset, children: Vec::new() }
	}
}
