//! One page: its size, its text layer, its tag tree, the objects drawn on it, and rasterizing
//! it for OCR.

use std::ffi::{c_ulong, c_void};

use pdfium_render::prelude::{
	FPDF_ANNOTATION, FPDF_DOCUMENT, FPDF_DWORD, FPDF_PAGE, FPDF_PAGEOBJ_FORM, FPDF_PAGEOBJ_IMAGE, FPDF_PAGEOBJECT,
	FPDF_PAGEOBJECTMARK, FS_RECTF,
};

use super::{PdfDocument, PdfError, PdfTextPage, TagTree, bindings, utf8_out_param, utf16_out_param};

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
		let handle = unsafe { bindings().FPDF_LoadPage(document.handle(), index) };
		if handle.is_null() {
			return Err(PdfError::Failed);
		}
		Ok(Self { handle, document: document.handle(), _doc: std::marker::PhantomData })
	}

	pub fn width(&self) -> f32 {
		unsafe { bindings().FPDF_GetPageWidthF(self.handle) }
	}

	pub fn height(&self) -> f32 {
		unsafe { bindings().FPDF_GetPageHeightF(self.handle) }
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
		unsafe { bindings().FPDFPage_CountObjects(self.handle) }
	}

	pub fn object(&self, index: i32) -> Option<PageObject<'_>> {
		PageObject::of(unsafe { bindings().FPDFPage_GetObject(self.handle, index) })
	}

	/// Every image the page draws that is worth a line of its own, nearest the top first.
	///
	/// This is the one list the parser places image lines from, so a page's images are announced in a fixed order.
	pub fn announced_images(&self) -> Vec<PageImage<'_>> {
		let page_area = f64::from(self.width()) * f64::from(self.height());
		let mut images = Vec::new();
		for i in 0..self.object_count() {
			if let Some(object) = self.object(i) {
				collect_images(&object, 0, page_area, true, &i.to_string(), &mut images);
			}
		}
		images.sort_by(|a: &PageImage<'_>, b| b.top.total_cmp(&a.top));
		images
	}

	/// The picture at `path`, encoded as PNG. `path` is the one an announced image carries in [`PageImage::path`], as it was written into an image marker's reference.
	pub fn image_png(&self, path: &str) -> Option<Vec<u8>> {
		let mut parts = path.split('/');
		let first = i32::try_from(parts.next()?.parse::<usize>().ok()?).ok()?;
		let object = self.object(first)?;
		let handle = descend_object(&object, parts)?;
		encode_image_png(handle)
	}

	/// The image path for each marked-content id on the page. A tagged PDF's `Figure` element points at its picture by marked-content id, and this is what turns that into a reference that can be saved, whether or not the picture is one Paperback announces.
	pub fn image_paths_by_mcid(&self) -> std::collections::HashMap<i32, String> {
		let page_area = f64::from(self.width()) * f64::from(self.height());
		let mut all = Vec::new();
		for i in 0..self.object_count() {
			if let Some(object) = self.object(i) {
				collect_images(&object, 0, page_area, false, &i.to_string(), &mut all);
			}
		}
		let mut paths = std::collections::HashMap::new();
		for image in all {
			for mcid in image.marked_content_ids() {
				paths.entry(mcid).or_insert_with(|| image.path.clone());
			}
		}
		paths
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
		let bitmap = unsafe { bindings().FPDFBitmap_CreateEx(width, height, BITMAP_BGRA, std::ptr::null_mut(), 0) };
		if bitmap.is_null() {
			return None;
		}
		unsafe { bindings().FPDFBitmap_FillRect(bitmap, 0, 0, width, height, WHITE) };
		unsafe { bindings().FPDF_RenderPageBitmap(bitmap, self.handle, 0, 0, width, height, 0, 0) };
		let stride = unsafe { bindings().FPDFBitmap_GetStride(bitmap) };
		let buffer = unsafe { bindings().FPDFBitmap_GetBuffer(bitmap) };
		let rgba = bitmap_to_rgba(buffer, width, height, stride, BITMAP_BGRA);
		unsafe { bindings().FPDFBitmap_Destroy(bitmap) };
		let (width, height, rgba) = rgba?;
		Some(PageBitmap { width, height, rgba })
	}

	/// Every link annotation on the page, with the box it covers and where it points.
	///
	/// A PDF states a link twice over and not always consistently: the annotation may carry an
	/// action with a URI, or a destination naming a page, or an action that itself carries a
	/// destination. All three are tried here so the parser only ever sees the answer.
	pub fn annotation_links(&self) -> Vec<AnnotationLink> {
		let bindings = bindings();
		let mut links = Vec::new();
		for index in 0..unsafe { bindings.FPDFPage_GetAnnotCount(self.handle) } {
			let annotation = unsafe { bindings.FPDFPage_GetAnnot(self.handle, index) };
			if annotation.is_null() {
				continue;
			}
			if unsafe { bindings.FPDFAnnot_GetSubtype(annotation) } == ANNOT_SUBTYPE_LINK
				&& let Some(link) = self.link_of(annotation)
			{
				links.push(link);
			}
			unsafe { bindings.FPDFPage_CloseAnnot(annotation) };
		}
		links
	}

	fn link_of(&self, annotation: FPDF_ANNOTATION) -> Option<AnnotationLink> {
		let bindings = bindings();
		let mut rect = FS_RECTF { left: 0.0, top: 0.0, right: 0.0, bottom: 0.0 };
		if unsafe { bindings.FPDFAnnot_GetRect(annotation, &raw mut rect) } == 0 {
			return None;
		}
		let link = unsafe { bindings.FPDFAnnot_GetLink(annotation) };
		if link.is_null() {
			return None;
		}
		let action = unsafe { bindings.FPDFLink_GetAction(link) };
		let target = if !action.is_null() && unsafe { bindings.FPDFAction_GetType(action) } == ACTION_URI {
			utf8_out_param(|buffer, len| unsafe {
				bindings.FPDFAction_GetURIPath(self.handle_document(), action, buffer, len)
			})
			.map(LinkTarget::Url)
		} else {
			None
		};
		let target = target.or_else(|| {
			let mut destination = unsafe { bindings.FPDFLink_GetDest(self.handle_document(), link) };
			if destination.is_null() && !action.is_null() {
				destination = unsafe { bindings.FPDFAction_GetDest(self.handle_document(), action) };
			}
			if destination.is_null() {
				return None;
			}
			let page = unsafe { bindings.FPDFDest_GetDestPageIndex(self.handle_document(), destination) };
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
		unsafe { bindings().FPDF_ClosePage(self.handle) };
	}
}

/// Shortest side, in PDF user units, an image must have to be worth a line of its own. A page laid out from HTML draws its spacers and its hairline rules as images a fraction of a point across; announcing one of those as an image gives the reader a stop with nothing at it.
const MIN_IMAGE_SIDE: f32 = 4.0;

/// How much of the page one image can cover and still be a picture on the page rather than the page itself. A scanned book with an OCR text layer draws each sheet as one image under its invisible text, and announcing that put an "[Image]" with nothing behind it on every page (#1046).
pub const PAGE_IMAGE_COVERAGE: f64 = 0.9;

/// How far to follow form `XObject`s into each other looking for the images they draw. A form holding a form is ordinary enough (a placed logo, an annotation's appearance stream), while a chain of them this deep is a malformed file rather than a page anyone meant to draw.
pub const MAX_FORM_DEPTH: u32 = 8;

/// `FPDFBitmap_BGRA`, `FPDFBitmap_BGRx`, `FPDFBitmap_BGR` and `FPDFBitmap_Gray`, the four bitmap layouts pdfium hands back.
const BITMAP_GRAY: i32 = 1;
const BITMAP_BGR: i32 = 2;
const BITMAP_BGRX: i32 = 3;

/// One image a page draws, as Paperback announces it.
///
/// Borrows the page it came from. Nothing is closed here; the page owns the object.
pub struct PageImage<'page> {
	/// The image's top edge, in PDF user units. Y grows up the page, so the nearest the top has the largest one.
	pub top: f64,
	/// Where the image object sits in the page's content stream, as slash-separated indices. This is what an image marker's reference carries, so [`PdfPage::image_png`] can find the picture again without depending on the order images were listed in.
	pub path: String,
	handle: FPDF_PAGEOBJECT,
	_page: std::marker::PhantomData<&'page ()>,
}

impl PageImage<'_> {
	/// The marked-content ids this image carries, which is how a tagged `Figure` names it.
	fn marked_content_ids(&self) -> Vec<i32> {
		let bindings = bindings();
		let mut ids = Vec::new();
		if let Some(id) = marked_content_id(self.handle) {
			ids.push(id);
		}
		for index in 0..unsafe { bindings.FPDFPageObj_CountMarks(self.handle) } {
			let mark = unsafe { bindings.FPDFPageObj_GetMark(self.handle, c_ulong::try_from(index).unwrap_or(0)) };
			if mark.is_null() {
				continue;
			}
			let mut value = 0;
			if unsafe { bindings.FPDFPageObjMark_GetParamIntValue(mark, "MCID", &raw mut value) } != 0 {
				ids.push(value);
			}
		}
		ids.sort_unstable();
		ids.dedup();
		ids
	}
}

fn marked_content_id(handle: FPDF_PAGEOBJECT) -> Option<i32> {
	let id = unsafe { bindings().FPDFPageObj_GetMarkedContentID(handle) };
	(id >= 0).then_some(id)
}

/// Follows the rest of a content-stream path into a form's children.
fn descend_object<'p, 'a>(object: &PageObject<'p>, mut path: impl Iterator<Item = &'a str>) -> Option<FPDF_PAGEOBJECT> {
	let Some(component) = path.next() else { return Some(object.handle()) };
	let index = i32::try_from(component.parse::<usize>().ok()?).ok()?;
	let child = object.form_object(index)?;
	descend_object(&child, path)
}

/// The image's own pixels, encoded as PNG.
///
/// The picture is taken at the resolution the file stores rather than as it is drawn on the page, so a photo placed small still saves at full size. An image mask is not applied: the PDF keeps the mask as a separate object, and the picture on its own is what was asked for.
fn encode_image_png(handle: FPDF_PAGEOBJECT) -> Option<Vec<u8>> {
	let bindings = bindings();
	let bitmap = unsafe { bindings.FPDFImageObj_GetBitmap(handle) };
	if bitmap.is_null() {
		return None;
	}
	let width = unsafe { bindings.FPDFBitmap_GetWidth(bitmap) };
	let height = unsafe { bindings.FPDFBitmap_GetHeight(bitmap) };
	let stride = unsafe { bindings.FPDFBitmap_GetStride(bitmap) };
	let format = unsafe { bindings.FPDFBitmap_GetFormat(bitmap) };
	let buffer = unsafe { bindings.FPDFBitmap_GetBuffer(bitmap) };
	let rgba = bitmap_to_rgba(buffer, width, height, stride, format);
	unsafe { bindings.FPDFBitmap_Destroy(bitmap) };
	let (width, height, rgba) = rgba?;
	let mut out = Vec::new();
	image::RgbaImage::from_raw(width, height, rgba)?
		.write_to(&mut std::io::Cursor::new(&mut out), image::ImageFormat::Png)
		.ok()?;
	Some(out)
}

/// Copies an image's bitmap into RGBA8, whatever layout pdfium handed back.
fn bitmap_to_rgba(
	buffer: *const c_void,
	width: i32,
	height: i32,
	stride: i32,
	format: i32,
) -> Option<(u32, u32, Vec<u8>)> {
	if buffer.is_null() || width <= 0 || height <= 0 {
		return None;
	}
	let (w, h) = (usize::try_from(width).ok()?, usize::try_from(height).ok()?);
	let stride = usize::try_from(stride).ok()?;
	let bytes_per_pixel = match format {
		BITMAP_GRAY => 1,
		BITMAP_BGR => 3,
		BITMAP_BGRX | BITMAP_BGRA => 4,
		_ => return None,
	};
	if stride < w * bytes_per_pixel {
		return None;
	}
	let source = unsafe { std::slice::from_raw_parts(buffer.cast::<u8>(), stride * h) };
	let mut rgba = vec![255u8; w * h * 4];
	for row in 0..h {
		for column in 0..w {
			let from = row * stride + column * bytes_per_pixel;
			let to = (row * w + column) * 4;
			if format == BITMAP_GRAY {
				let gray = source[from];
				rgba[to] = gray;
				rgba[to + 1] = gray;
				rgba[to + 2] = gray;
			} else {
				rgba[to] = source[from + 2];
				rgba[to + 1] = source[from + 1];
				rgba[to + 2] = source[from];
				if format == BITMAP_BGRA {
					rgba[to + 3] = source[from + 3];
				}
			}
		}
	}
	Some((u32::try_from(width).ok()?, u32::try_from(height).ok()?, rgba))
}

/// Collects the image objects under `object`, walking into form `XObject`s and recording where each one sits in the content stream. `only_announced` leaves out the images that are too small to be worth a line or that cover the page themselves.
fn collect_images(
	object: &PageObject<'_>,
	depth: u32,
	page_area: f64,
	only_announced: bool,
	path: &str,
	images: &mut Vec<PageImage<'_>>,
) {
	match object.kind() {
		ObjectKind::Image => {
			let announced = object.bounds().is_some_and(|bounds| {
				is_announced_image((bounds.right - bounds.left).abs(), (bounds.top - bounds.bottom).abs(), page_area)
			});
			if !only_announced || announced {
				let top = object.bounds().map_or(0.0, |bounds| f64::from(bounds.top));
				images.push(PageImage {
					top,
					path: path.to_string(),
					handle: object.handle(),
					_page: std::marker::PhantomData,
				});
			}
		}
		ObjectKind::Form if depth < MAX_FORM_DEPTH => {
			for i in 0..object.form_object_count() {
				if let Some(child) = object.form_object(i) {
					collect_images(&child, depth + 1, page_area, only_announced, &format!("{path}/{i}"), images);
				}
			}
		}
		_ => {}
	}
}

fn is_announced_image(width: f32, height: f32, page_area: f64) -> bool {
	let covers_page = page_area > 0.0 && f64::from(width) * f64::from(height) / page_area >= PAGE_IMAGE_COVERAGE;
	width >= MIN_IMAGE_SIDE && height >= MIN_IMAGE_SIDE && !covers_page
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
		let kind = unsafe { bindings().FPDFPageObj_GetType(self.handle) };
		if kind == FPDF_PAGEOBJ_IMAGE as i32 {
			ObjectKind::Image
		} else if kind == FPDF_PAGEOBJ_FORM as i32 {
			ObjectKind::Form
		} else {
			ObjectKind::Other
		}
	}

	pub const fn handle(&self) -> FPDF_PAGEOBJECT {
		self.handle
	}

	pub fn bounds(&self) -> Option<Bounds> {
		let (mut left, mut bottom, mut right, mut top) = (0.0, 0.0, 0.0, 0.0);
		let ok = unsafe { bindings().FPDFPageObj_GetBounds(self.handle, &mut left, &mut bottom, &mut right, &mut top) };
		(ok != 0).then_some(Bounds { left, bottom, right, top })
	}

	/// How many objects a form XObject groups. Zero for anything else.
	pub fn form_object_count(&self) -> i32 {
		unsafe { bindings().FPDFFormObj_CountObjects(self.handle) }
	}

	pub fn form_object(&self, index: i32) -> Option<PageObject<'_>> {
		PageObject::of(unsafe { bindings().FPDFFormObj_GetObject(self.handle, c_ulong::try_from(index).ok()?) })
	}

	/// The marked-content id pdfium reports for this object, which is the outermost mark that
	/// carries one.
	pub fn marked_content_id(&self) -> Option<i32> {
		let id = unsafe { bindings().FPDFPageObj_GetMarkedContentID(self.handle) };
		(id >= 0).then_some(id)
	}

	pub fn mark_count(&self) -> i32 {
		unsafe { bindings().FPDFPageObj_CountMarks(self.handle) }
	}

	pub fn mark(&self, index: i32) -> Option<ObjectMark<'_>> {
		let handle = unsafe { bindings().FPDFPageObj_GetMark(self.handle, c_ulong::try_from(index).ok()?) };
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
	/// The mark's tag as the content stream spells it: `P`, `Span`, `Artifact` and so on.
	pub fn name(&self) -> Option<String> {
		utf16_out_param(|buffer, len| {
			let mut needed: c_ulong = 0;
			let ok = unsafe { bindings().FPDFPageObjMark_GetName(self.handle, buffer.cast(), len, &mut needed) };
			if ok != 0 { needed } else { 0 }
		})
	}

	/// An integer parameter on this mark, such as `MCID`.
	pub fn param_int(&self, key: &str) -> Option<i32> {
		let mut value = 0;
		let ok = unsafe { bindings().FPDFPageObjMark_GetParamIntValue(self.handle, key, &mut value) };
		(ok != 0).then_some(value)
	}
}

#[cfg(test)]
mod tests {
	use super::{BITMAP_BGR, BITMAP_BGRA, BITMAP_GRAY, bitmap_to_rgba, is_announced_image};

	/// The page from #1046: 323 by 518 points, under a 323 by 496 point scan.
	const PAGE_AREA: f64 = 323.0 * 518.0;

	#[test]
	fn a_scan_under_the_whole_page_is_not_announced() {
		assert!(!is_announced_image(323.0, 496.0, PAGE_AREA));
	}

	#[test]
	fn a_figure_on_the_page_is_announced() {
		assert!(is_announced_image(250.0, 300.0, PAGE_AREA));
	}

	#[test]
	fn a_hairline_rule_is_not_announced() {
		assert!(!is_announced_image(300.0, 0.5, PAGE_AREA));
	}

	/// Pdfium hands BGRA back, and a PNG wants RGBA.
	#[test]
	fn a_bgra_bitmap_becomes_rgba() {
		// One pixel: blue, green, red, alpha.
		let mut pixels = [0u8, 0, 255, 128];
		let (width, height, rgba) = bitmap_to_rgba(pixels.as_mut_ptr().cast(), 1, 1, 4, BITMAP_BGRA).unwrap();
		assert_eq!((width, height), (1, 1));
		assert_eq!(rgba, vec![255, 0, 0, 128]);
	}

	/// Rows are padded to a stride, so the padding must not be read as pixels.
	#[test]
	fn a_padded_row_does_not_shift_the_next_one() {
		// Two rows of two BGRA pixels with four bytes of padding between them.
		let mut pixels = [1u8, 2, 3, 4, 5, 6, 7, 8, 0, 0, 0, 0, 9, 10, 11, 12, 13, 14, 15, 16];
		let (_, _, rgba) = bitmap_to_rgba(pixels.as_mut_ptr().cast(), 2, 2, 12, BITMAP_BGRA).unwrap();
		assert_eq!(&rgba[0..4], &[3, 2, 1, 4]);
		assert_eq!(&rgba[8..12], &[11, 10, 9, 12], "the first pixel of the second row");
	}

	/// A grayscale bitmap, which an image PDF can hold, widens to RGBA.
	#[test]
	fn a_gray_bitmap_widens_to_rgba() {
		let mut pixels = [17u8, 34];
		let (_, _, rgba) = bitmap_to_rgba(pixels.as_mut_ptr().cast(), 2, 1, 2, BITMAP_GRAY).unwrap();
		assert_eq!(rgba, vec![17, 17, 17, 255, 34, 34, 34, 255]);
	}

	/// Three channels and no alpha, which pdfium can return for an image without transparency.
	#[test]
	fn a_bgr_bitmap_is_opaque() {
		let mut pixels = [1u8, 2, 3];
		let (_, _, rgba) = bitmap_to_rgba(pixels.as_mut_ptr().cast(), 1, 1, 3, BITMAP_BGR).unwrap();
		assert_eq!(rgba, vec![3, 2, 1, 255]);
	}
}
