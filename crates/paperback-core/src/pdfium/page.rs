//! One page: its size, its text layer, its tag tree, the objects drawn on it, and rasterizing
//! it for OCR.

use std::ffi::{c_ulong, c_void};

use pdfium_render::prelude::{
	FPDF_ANNOTATION, FPDF_DOCUMENT, FPDF_DWORD, FPDF_PAGE, FPDF_PAGEOBJ_FORM, FPDF_PAGEOBJ_IMAGE, FPDF_PAGEOBJECT,
	FPDF_PAGEOBJECTMARK, FS_RECTF,
};

use super::{PdfDocument, PdfError, PdfTextPage, TagTree, bindings, utf8_out_param};

/// `PDFACTION_URI` and `FPDF_ANNOT_LINK`, the two codes the crate does not re-export. Every
/// other code here is imported from it rather than written out, because a page-object type
/// written from memory once cost a silent loss of every image on a page.
const ACTION_URI: c_ulong = 3;
const ANNOT_SUBTYPE_LINK: i32 = 2;
/// `FPDFBitmap_BGRA`: four bytes per pixel, blue first.
const BITMAP_BGRA: i32 = 4;
/// White, as the background a page is drawn onto. OCR does far better on white than on the
/// transparent black an uninitialized bitmap would otherwise leave behind the glyphs.
///
/// Typed as `FPDF_DWORD` rather than as a fixed width on purpose: it is a C `unsigned long`,
/// which is 32 bits on Windows and 64 everywhere else, so writing either one here compiles on
/// one half of the platforms and fails on the other.
const WHITE: FPDF_DWORD = 0xFFFF_FFFF;

/// A page, borrowed from the document it belongs to.
pub struct PdfPage<'doc> {
	handle: FPDF_PAGE,
	/// Kept because link destinations are resolved against the document, not the page.
	document: FPDF_DOCUMENT,
	_doc: std::marker::PhantomData<&'doc PdfDocument>,
}

impl<'doc> PdfPage<'doc> {
	pub(super) fn load(document: &'doc PdfDocument, index: i32) -> Result<Self, PdfError> {
		let handle = bindings().FPDF_LoadPage(document.handle(), index);
		if handle.is_null() {
			return Err(PdfError::Failed);
		}
		Ok(Self { handle, document: document.handle(), _doc: std::marker::PhantomData })
	}

	pub fn width(&self) -> f32 {
		bindings().FPDF_GetPageWidthF(self.handle)
	}

	pub fn height(&self) -> f32 {
		bindings().FPDF_GetPageHeightF(self.handle)
	}

	/// The page's text layer, or `None` for a page that carries no text at all.
	pub fn text(&self) -> Option<PdfTextPage<'_>> {
		PdfTextPage::of(self)
	}

	/// The page's structure tree, or `None` when the document is not tagged.
	pub fn tags(&self) -> Option<TagTree<'_>> {
		TagTree::of(self)
	}

	pub fn object_count(&self) -> i32 {
		bindings().FPDFPage_CountObjects(self.handle)
	}

	pub fn object(&self, index: i32) -> Option<PageObject<'_>> {
		PageObject::of(bindings().FPDFPage_GetObject(self.handle, index))
	}

	/// Rasterizes the page `width` pixels across, keeping its aspect ratio, as RGBA8.
	///
	/// Pdfium draws BGRA, so the channels are swapped on the way out rather than leaving every
	/// OCR engine to discover that for itself.
	pub fn render(&self, width: u32) -> Option<PageBitmap> {
		let page_width = f64::from(self.width());
		let page_height = f64::from(self.height());
		if page_width <= 0.0 || page_height <= 0.0 || width == 0 {
			return None;
		}
		let height = ((f64::from(width) * page_height) / page_width).round().max(1.0);
		let (width, height) = (i32::try_from(width).ok()?, height as i32);

		let bitmap = bindings().FPDFBitmap_CreateEx(width, height, BITMAP_BGRA, std::ptr::null_mut(), 0);
		if bitmap.is_null() {
			return None;
		}
		bindings().FPDFBitmap_FillRect(bitmap, 0, 0, width, height, WHITE);
		bindings().FPDF_RenderPageBitmap(bitmap, self.handle, 0, 0, width, height, 0, 0);
		let stride = bindings().FPDFBitmap_GetStride(bitmap);
		let buffer = bindings().FPDFBitmap_GetBuffer(bitmap);
		let rgba = bgra_to_rgba(buffer, width, height, stride);
		bindings().FPDFBitmap_Destroy(bitmap);
		let rgba = rgba?;
		Some(PageBitmap { width: u32::try_from(width).ok()?, height: u32::try_from(height).ok()?, rgba })
	}

	/// Every link annotation on the page, with the box it covers and where it points.
	///
	/// A PDF states a link twice over and not always consistently: the annotation may carry an
	/// action with a URI, or a destination naming a page, or an action that itself carries a
	/// destination. All three are tried here so the parser only ever sees the answer.
	pub fn annotation_links(&self) -> Vec<AnnotationLink> {
		let bindings = bindings();
		let mut links = Vec::new();
		for index in 0..bindings.FPDFPage_GetAnnotCount(self.handle) {
			let annotation = bindings.FPDFPage_GetAnnot(self.handle, index);
			if annotation.is_null() {
				continue;
			}
			if bindings.FPDFAnnot_GetSubtype(annotation) == ANNOT_SUBTYPE_LINK
				&& let Some(link) = self.link_of(annotation)
			{
				links.push(link);
			}
			bindings.FPDFPage_CloseAnnot(annotation);
		}
		links
	}

	fn link_of(&self, annotation: FPDF_ANNOTATION) -> Option<AnnotationLink> {
		let bindings = bindings();
		let mut rect = FS_RECTF { left: 0.0, top: 0.0, right: 0.0, bottom: 0.0 };
		if bindings.FPDFAnnot_GetRect(annotation, &raw mut rect) == 0 {
			return None;
		}
		let link = bindings.FPDFAnnot_GetLink(annotation);
		if link.is_null() {
			return None;
		}
		let action = bindings.FPDFLink_GetAction(link);
		let target = if !action.is_null() && bindings.FPDFAction_GetType(action) == ACTION_URI {
			utf8_out_param(|buffer, len| bindings.FPDFAction_GetURIPath(self.handle_document(), action, buffer, len))
				.map(LinkTarget::Url)
		} else {
			None
		};
		let target = target.or_else(|| {
			let mut destination = bindings.FPDFLink_GetDest(self.handle_document(), link);
			if destination.is_null() && !action.is_null() {
				destination = bindings.FPDFAction_GetDest(self.handle_document(), action);
			}
			if destination.is_null() {
				return None;
			}
			let page = bindings.FPDFDest_GetDestPageIndex(self.handle_document(), destination);
			(page >= 0).then_some(LinkTarget::Page(page))
		})?;
		Some(AnnotationLink {
			rect: Bounds { left: rect.left, bottom: rect.bottom, right: rect.right, top: rect.top },
			target,
		})
	}

	const fn handle_document(&self) -> FPDF_DOCUMENT {
		self.document
	}

	pub(super) fn handle(&self) -> FPDF_PAGE {
		self.handle
	}
}

/// Where a link goes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LinkTarget {
	Url(String),
	/// Somewhere inside this document, by zero-based page index.
	Page(i32),
}

/// One link annotation: the box on the page a reader would activate, and its destination.
#[derive(Debug, Clone, PartialEq)]
pub struct AnnotationLink {
	pub rect: Bounds,
	pub target: LinkTarget,
}

impl Drop for PdfPage<'_> {
	fn drop(&mut self) {
		bindings().FPDF_ClosePage(self.handle);
	}
}

/// Copies pdfium's BGRA rows out into a tight RGBA buffer.
///
/// The rows are padded to a stride, so this cannot be one flat copy; each row is taken at its
/// own offset and the padding left behind.
fn bgra_to_rgba(buffer: *const c_void, width: i32, height: i32, stride: i32) -> Option<Vec<u8>> {
	if buffer.is_null() || stride < width * 4 {
		return None;
	}
	let (w, h, stride) = (width as usize, height as usize, stride as usize);
	let source = unsafe { std::slice::from_raw_parts(buffer.cast::<u8>(), stride * h) };
	let mut rgba = vec![0u8; w * h * 4];
	for row in 0..h {
		for column in 0..w {
			let from = row * stride + column * 4;
			let to = (row * w + column) * 4;
			rgba[to] = source[from + 2];
			rgba[to + 1] = source[from + 1];
			rgba[to + 2] = source[from];
			rgba[to + 3] = source[from + 3];
		}
	}
	Some(rgba)
}

/// A rasterized page.
pub struct PageBitmap {
	pub width: u32,
	pub height: u32,
	/// Four bytes per pixel, row-major, `width * height * 4` long.
	pub rgba: Vec<u8>,
}

/// What a page object draws.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectKind {
	Image,
	/// A form XObject, which is a group of other objects and has to be descended into.
	Form,
	Other,
}

/// The box an object occupies, in PDF points.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bounds {
	pub left: f32,
	pub bottom: f32,
	pub right: f32,
	pub top: f32,
}

/// One object in a page's content stream.
///
/// Borrowed from whatever produced it, which owns it. Nothing is closed here.
pub struct PageObject<'page> {
	handle: FPDF_PAGEOBJECT,
	_page: std::marker::PhantomData<&'page ()>,
}

impl PageObject<'_> {
	pub(super) fn of(handle: FPDF_PAGEOBJECT) -> Option<Self> {
		if handle.is_null() {
			return None;
		}
		Some(Self { handle, _page: std::marker::PhantomData })
	}

	pub fn kind(&self) -> ObjectKind {
		let kind = bindings().FPDFPageObj_GetType(self.handle);
		if kind == FPDF_PAGEOBJ_IMAGE as i32 {
			ObjectKind::Image
		} else if kind == FPDF_PAGEOBJ_FORM as i32 {
			ObjectKind::Form
		} else {
			ObjectKind::Other
		}
	}

	pub fn bounds(&self) -> Option<Bounds> {
		let (mut left, mut bottom, mut right, mut top) = (0.0, 0.0, 0.0, 0.0);
		let ok = bindings().FPDFPageObj_GetBounds(self.handle, &mut left, &mut bottom, &mut right, &mut top);
		(ok != 0).then_some(Bounds { left, bottom, right, top })
	}

	/// How many objects a form XObject groups. Zero for anything else.
	pub fn form_object_count(&self) -> i32 {
		bindings().FPDFFormObj_CountObjects(self.handle)
	}

	pub fn form_object(&self, index: i32) -> Option<PageObject<'_>> {
		PageObject::of(bindings().FPDFFormObj_GetObject(self.handle, c_ulong::try_from(index).ok()?))
	}

	/// The marked-content id pdfium reports for this object, which is the outermost mark that
	/// carries one.
	pub fn marked_content_id(&self) -> Option<i32> {
		let id = bindings().FPDFPageObj_GetMarkedContentID(self.handle);
		(id >= 0).then_some(id)
	}

	pub fn mark_count(&self) -> i32 {
		bindings().FPDFPageObj_CountMarks(self.handle)
	}

	pub fn mark(&self, index: i32) -> Option<ObjectMark<'_>> {
		let handle = bindings().FPDFPageObj_GetMark(self.handle, c_ulong::try_from(index).ok()?);
		if handle.is_null() {
			return None;
		}
		Some(ObjectMark { handle, _object: std::marker::PhantomData })
	}
}

/// One marked-content mark on an object. A tagged page wraps its text in these, and the `MCID`
/// parameter on them is what the tag tree points at.
pub struct ObjectMark<'object> {
	handle: FPDF_PAGEOBJECTMARK,
	_object: std::marker::PhantomData<&'object ()>,
}

impl ObjectMark<'_> {
	/// An integer parameter on this mark, such as `MCID`.
	pub fn param_int(&self, key: &str) -> Option<i32> {
		let mut value = 0;
		let ok = bindings().FPDFPageObjMark_GetParamIntValue(self.handle, key, &mut value);
		(ok != 0).then_some(value)
	}
}
