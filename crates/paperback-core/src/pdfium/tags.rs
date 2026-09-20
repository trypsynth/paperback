//! A tagged PDF's structure tree: the author's own account of what the page means.
//!
//! An untagged PDF is a bag of glyphs at coordinates, and reading it back as prose means
//! guessing where paragraphs and headings were from spacing alone. A tagged one carries a tree
//! saying "this is a heading, this is a list item, this is a table cell", and each leaf of that
//! tree points at a marked-content id that the page's glyphs are stamped with. Following that
//! tree is the difference between reading an author's document and reading a reconstruction of
//! it, so this is the most load-bearing part of the PDF parser for a screen reader user.
//!
//! The one shape worth knowing: a tag's children are a mixed list. Some are tags again, and
//! some are references to marked content on the page. Pdfium reports both through the same
//! index, so a child is fetched as a tag first and read as a content id only when that comes
//! back empty. [`Tag::child`] and [`Tag::child_mcid`] are that pair, and they have to be asked
//! in that order.

use std::ffi::{c_ulong, c_void};

use pdfium_render::prelude::{FPDF_STRUCTELEMENT, FPDF_STRUCTTREE};

use super::{PdfPage, bindings, utf16_out_param};

/// A page's structure tree, borrowed from the page so it cannot outlive it.
pub struct TagTree<'page> {
	handle: FPDF_STRUCTTREE,
	_page: std::marker::PhantomData<&'page PdfPage<'page>>,
}

impl<'page> TagTree<'page> {
	/// The structure tree for `page`, or `None` when the document is not tagged.
	pub(super) fn of(page: &'page PdfPage<'page>) -> Option<Self> {
		let handle = bindings().FPDF_StructTree_GetForPage(page.handle());
		if handle.is_null() {
			return None;
		}
		Some(Self { handle, _page: std::marker::PhantomData })
	}

	pub fn child_count(&self) -> i32 {
		bindings().FPDF_StructTree_CountChildren(self.handle)
	}

	/// The top-level tag at `index`, or `None` when there is no tag there.
	///
	/// A tree whose every root comes back empty is how a PDF that claims to be tagged but is not
	/// really shows up, which the parser checks for before trusting any of this.
	pub fn child(&self, index: i32) -> Option<Tag<'_>> {
		let handle = bindings().FPDF_StructTree_GetChildAtIndex(self.handle, index);
		Tag::of(handle)
	}
}

impl Drop for TagTree<'_> {
	fn drop(&mut self) {
		bindings().FPDF_StructTree_Close(self.handle);
	}
}

/// One element of the structure tree: a heading, a paragraph, a list item, a table cell.
///
/// Borrowed from the tree, which owns every element in it. Nothing is closed here; closing the
/// tree frees them all.
pub struct Tag<'tree> {
	handle: FPDF_STRUCTELEMENT,
	_tree: std::marker::PhantomData<&'tree TagTree<'tree>>,
}

impl Tag<'_> {
	fn of(handle: FPDF_STRUCTELEMENT) -> Option<Self> {
		if handle.is_null() {
			return None;
		}
		Some(Self { handle, _tree: std::marker::PhantomData })
	}

	/// The structure type, as the PDF spells it: `H1`, `P`, `LI`, `Figure`, `TD` and so on.
	pub fn kind(&self) -> Option<String> {
		self.string_field(|bindings, handle, buffer, len| bindings.FPDF_StructElement_GetType(handle, buffer, len))
	}

	/// Text the author supplied to be read in place of this element's glyphs, which is how a
	/// ligature or a stylised word says what it really is.
	pub fn actual_text(&self) -> Option<String> {
		self.string_field(|bindings, handle, buffer, len| {
			bindings.FPDF_StructElement_GetActualText(handle, buffer, len)
		})
	}

	/// The alternative description, which is what a tagged figure carries instead of text.
	pub fn alt_text(&self) -> Option<String> {
		self.string_field(|bindings, handle, buffer, len| bindings.FPDF_StructElement_GetAltText(handle, buffer, len))
	}

	pub fn child_count(&self) -> i32 {
		bindings().FPDF_StructElement_CountChildren(self.handle)
	}

	/// The child tag at `index`, or `None` when that child is marked content rather than a tag.
	///
	/// Ask this first; [`Tag::child_mcid`] answers for the same index when this returns `None`.
	pub fn child(&self, index: i32) -> Option<Tag<'_>> {
		Tag::of(bindings().FPDF_StructElement_GetChildAtIndex(self.handle, index))
	}

	/// The marked-content id of the child at `index`, which is how a leaf of the tree names the
	/// glyphs on the page that belong to it. `None` when that child is a tag, or carries no id.
	pub fn child_mcid(&self, index: i32) -> Option<i32> {
		let id = bindings().FPDF_StructElement_GetChildMarkedContentID(self.handle, index);
		(id >= 0).then_some(id)
	}

	fn string_field(
		&self,
		read: impl Fn(
			&'static dyn pdfium_render::prelude::PdfiumLibraryBindings,
			FPDF_STRUCTELEMENT,
			*mut c_void,
			c_ulong,
		) -> c_ulong,
	) -> Option<String> {
		utf16_out_param(|buffer, len| read(bindings(), self.handle, buffer, len))
	}
}
