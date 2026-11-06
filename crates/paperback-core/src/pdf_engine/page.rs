// What a page keeps is in `f32`, as pdfium's is, while hayro works in `f64`.
#![allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]

use std::sync::Arc;

use hayro::{PixmapSettings, RenderCache, RenderSettings, vello_cpu::color::palette::css::WHITE};
use hayro_interpret::{
	BlendMode, ClipPath, Context, Device, DrawMode, DrawProps, Image, ImageData, ImageDrawProps, InterpreterCache,
	MarkedContentProperties, SoftMask,
	font::GlyphRun,
	interpret_page,
	kurbo::{Affine, BezPath, Point, Rect},
};
use hayro_syntax::object::{Array, Dict, Name};

use super::{
	Document, Error, TagTree, TextPage, decode_text_string, fonts,
	text::{FontCache, TextObject},
};

/// One page of a document. It owns what was read, the text layer included, so pages can be read on several threads at once and handed back to one.
pub struct Page<'doc> {
	document: &'doc Document,
	index: usize,
	width: f32,
	height: f32,
	objects: Vec<ObjectData>,
	text: TextPage,
}

/// A book sets every page in the same few fonts, and reading them again for each page was half of what loading a page cost.
pub struct PageCache<'doc>(InterpreterCache<'doc>);

impl PageCache<'_> {
	pub(super) fn new() -> Self {
		Self(InterpreterCache::new())
	}
}

impl<'doc> Page<'doc> {
	pub(super) fn load(document: &'doc Document, index: usize, cache: &PageCache<'doc>) -> Result<Self, Error> {
		let page = document.pdf().pages().get(index).ok_or(Error::PageOutOfRange)?;
		let (width, height) = page.render_dimensions();
		// The bounds only decide what gets drawn, so they are made too large to leave anything out; the collector leaves out what's off the page.
		let mut context = Context::new(
			Affine::IDENTITY,
			Rect::new(-1e7, -1e7, 1e7, 1e7),
			&cache.0,
			document.pdf().xref(),
			fonts::settings(),
		);
		let crop = page.intersected_crop_box();
		let mut collector = Collector { page: Rect::new(crop.x0, crop.y0, crop.x1, crop.y1), ..Collector::default() };
		interpret_page(page, &mut context, &mut collector);
		let text = TextPage::build(&collector.text_objects, &collector.objects, width, height, display_matrix(page));
		Ok(Self { document, index, width, height, objects: collector.objects, text })
	}

	/// In points, after the page's rotation.
	pub fn width(&self) -> f64 {
		f64::from(self.width)
	}

	/// In points, after the page's rotation.
	pub fn height(&self) -> f64 {
		f64::from(self.height)
	}

	/// The page's characters.
	pub const fn text(&self) -> &TextPage {
		&self.text
	}

	/// The document's structure tree as this page sees it, or `None` for a document without one.
	pub fn tags(&self) -> Option<TagTree<'_>> {
		Some(TagTree::of(self.document.structure()?, self.index))
	}

	/// The page's text runs and images, in the order they are drawn.
	pub fn objects(&self) -> impl ExactSizeIterator<Item = PageObject<'_>> {
		self.objects.iter().map(|data| PageObject { data })
	}

	/// Every image the page draws, in drawing order.
	pub fn images(&self) -> impl Iterator<Item = PageImage> {
		self.image_objects().filter_map(|(index, data)| {
			let bounds = data.bounds?;
			Some(PageImage { id: ImageId(index), bounds: rect_of(bounds) })
		})
	}

	/// Taken at the resolution the file stores rather than as drawn, so a photo placed small still saves at full size.
	pub fn image_png(&self, id: ImageId) -> Option<Vec<u8>> {
		let ordinal = self.objects.get(id.0)?.image?;
		let page = self.document.pdf().pages().get(self.index)?;
		let cache = InterpreterCache::new();
		let mut context = Context::new(
			Affine::IDENTITY,
			Rect::new(-1e7, -1e7, 1e7, 1e7),
			&cache,
			self.document.pdf().xref(),
			fonts::settings(),
		);
		let mut extractor = ImageExtractor { wanted: ordinal, seen: 0, png: None };
		interpret_page(page, &mut context, &mut extractor);
		extractor.png
	}

	/// Which image each marked content id draws, for a structure tree that tags its figures.
	pub fn images_by_mcid(&self) -> std::collections::HashMap<i32, ImageId> {
		let mut images = std::collections::HashMap::new();
		for (index, data) in self.image_objects() {
			for mcid in data.marked_content_ids() {
				images.entry(mcid).or_insert(ImageId(index));
			}
		}
		images
	}

	/// Draws the page on white at the given width in pixels, keeping its proportions. `None` when the page or the width is empty.
	pub fn render(&self, width: u32) -> Option<PageBitmap> {
		if self.width <= 0.0 || self.height <= 0.0 || width == 0 {
			return None;
		}
		let page = self.document.pdf().pages().get(self.index)?;
		let scale = width as f32 / self.width;
		let settings = PixmapSettings { x_scale: scale, y_scale: scale, bg_color: WHITE };
		let pixmap =
			hayro::render(page, &RenderCache::new(), &fonts::settings(), &RenderSettings::default(), &settings);
		let (width, height) = (u32::from(pixmap.width()), u32::from(pixmap.height()));
		if width == 0 || height == 0 {
			return None;
		}
		// Drawn onto opaque white, so the premultiplied pixels hayro hands back are already the straight ones.
		Some(PageBitmap { width, height, rgba: pixmap.data_as_u8_slice().to_vec() })
	}

	/// The page's link annotations that lead somewhere. A web address that is only written out in the text is in [`TextPage::web_links`] instead.
	pub fn annotation_links(&self) -> Vec<AnnotationLink> {
		let Some(page) = self.document.pdf().pages().get(self.index) else { return Vec::new() };
		let Some(annotations) = page.raw().get::<Array<'_>>(b"Annots") else { return Vec::new() };
		annotations.iter::<Dict<'_>>().filter_map(|annotation| self.link_of(&annotation)).collect()
	}

	fn link_of(&self, annotation: &Dict<'_>) -> Option<AnnotationLink> {
		if annotation.get::<Name<'_>>(b"Subtype").as_deref() != Some(b"Link".as_slice()) {
			return None;
		}
		let rect = annotation.get::<hayro_syntax::object::Rect>(b"Rect")?;
		let uri = annotation
			.get::<Dict<'_>>(b"A")
			.filter(|action| action.get::<Name<'_>>(b"S").as_deref() == Some(b"URI".as_slice()))
			.and_then(|action| action.get::<hayro_syntax::object::String<'_>>(b"URI"))
			.map(|uri| String::from_utf8_lossy(uri.as_bytes()).into_owned())
			.filter(|uri| !uri.is_empty());
		let target = match uri {
			Some(uri) => LinkTarget::Url(uri),
			None => LinkTarget::Page(self.document.destination_page(annotation)?),
		};
		let rect = super::Rect {
			left: rect.x0.min(rect.x1),
			bottom: rect.y0.min(rect.y1),
			right: rect.x0.max(rect.x1),
			top: rect.y0.max(rect.y1),
		};
		Some(AnnotationLink { rect, target })
	}

	fn image_objects(&self) -> impl Iterator<Item = (usize, &ObjectData)> {
		self.objects.iter().enumerate().filter(|(_, data)| data.kind == ObjectKind::Image)
	}
}

/// pdfium's display matrix: page space to a device whose y grows downward, turned by the page's /Rotate.
fn display_matrix(page: &hayro_syntax::page::Page<'_>) -> [f32; 6] {
	let crop = page.intersected_crop_box();
	let (x0, y0, x1, y1) = (crop.x0 as f32, crop.y0 as f32, crop.x1 as f32, crop.y1 as f32);
	match page.rotation() {
		hayro_syntax::page::Rotation::Horizontal => [0.0, 1.0, 1.0, 0.0, -y0, -x0],
		hayro_syntax::page::Rotation::Flipped => [-1.0, 0.0, 0.0, 1.0, x1, -y0],
		hayro_syntax::page::Rotation::FlippedHorizontal => [0.0, -1.0, -1.0, 0.0, y1, x1],
		hayro_syntax::page::Rotation::None => [1.0, 0.0, 0.0, -1.0, -x0, y1],
	}
}

/// Where a link goes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LinkTarget {
	Url(String),
	/// The index of a page in the same document.
	Page(usize),
}

/// A link annotation on a page.
#[derive(Debug, Clone, PartialEq)]
pub struct AnnotationLink {
	pub rect: super::Rect,
	pub target: LinkTarget,
}

/// Names an image on its page. It stays valid as long as the document is unchanged, and its text form can be stored and parsed back.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ImageId(usize);

impl std::fmt::Display for ImageId {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.0.fmt(f)
	}
}

impl std::str::FromStr for ImageId {
	type Err = std::num::ParseIntError;

	fn from_str(text: &str) -> Result<Self, Self::Err> {
		text.parse().map(Self)
	}
}

/// An image the page draws.
pub struct PageImage {
	pub id: ImageId,
	pub bounds: super::Rect,
}

fn rect_of(bounds: Bounds) -> super::Rect {
	super::Rect {
		left: f64::from(bounds.left),
		bottom: f64::from(bounds.bottom),
		right: f64::from(bounds.right),
		top: f64::from(bounds.top),
	}
}

/// A page drawn to pixels.
pub struct PageBitmap {
	pub width: u32,
	pub height: u32,
	/// Four bytes a pixel, row by row from the top.
	pub rgba: Vec<u8>,
}

/// What a [`PageObject`] is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectKind {
	Image,
	/// A run of text, the only other thing a page keeps.
	Other,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct Bounds {
	pub(super) left: f32,
	pub(super) bottom: f32,
	pub(super) right: f32,
	pub(super) top: f32,
}

#[derive(Debug, Clone)]
pub(super) struct Mark {
	pub(super) name: String,
	pub(super) mcid: Option<i32>,
	pub(super) actual_text: Option<String>,
	/// Which `BDC` opened it, so two objects can tell whether they share one sequence the way pdfium tells by comparing property dictionaries.
	pub(super) sequence: usize,
}

#[derive(Clone)]
pub(super) struct ObjectData {
	pub(super) kind: ObjectKind,
	pub(super) bounds: Option<Bounds>,
	/// Outermost first, the order pdfium lists an object's marks in.
	pub(super) marks: Arc<[Mark]>,
	image: Option<usize>,
}

impl ObjectData {
	fn marked_content_id(&self) -> Option<i32> {
		self.marks.iter().find_map(|mark| mark.mcid)
	}

	fn marked_content_ids(&self) -> Vec<i32> {
		let mut ids: Vec<i32> = self.marks.iter().filter_map(|mark| mark.mcid).collect();
		ids.sort_unstable();
		ids.dedup();
		ids
	}
}

/// A run of text or an image drawn on a page.
pub struct PageObject<'page> {
	data: &'page ObjectData,
}

impl<'page> PageObject<'page> {
	pub(super) const fn new(data: &'page ObjectData) -> Self {
		Self { data }
	}

	pub const fn kind(&self) -> ObjectKind {
		self.data.kind
	}

	/// The area the object covers, if it has one.
	pub fn bounds(&self) -> Option<super::Rect> {
		self.data.bounds.map(rect_of)
	}

	/// The outermost id, which is the one pdfium reports.
	pub fn marked_content_id(&self) -> Option<i32> {
		self.data.marked_content_id()
	}

	/// The marked content sequences the object sits in, outermost first.
	pub fn marks(&self) -> impl ExactSizeIterator<Item = ObjectMark<'page>> + use<'page> {
		self.data.marks.iter().map(|mark| ObjectMark { mark })
	}
}

/// A marked content sequence an object sits in.
pub struct ObjectMark<'object> {
	mark: &'object Mark,
}

impl<'object> ObjectMark<'object> {
	/// The mark's tag, such as `Artifact` or `P`.
	pub fn name(&self) -> Option<&'object str> {
		Some(self.mark.name.as_str()).filter(|name| !name.is_empty())
	}

	/// The id the structure tree refers to the sequence by.
	pub const fn mcid(&self) -> Option<i32> {
		self.mark.mcid
	}
}

#[derive(Default)]
struct Collector {
	page: Rect,
	objects: Vec<ObjectData>,
	text_objects: Vec<TextObject>,
	marks: Vec<Mark>,
	/// Rebuilt only when a mark opens or closes, since a page's objects mostly sit inside the same few.
	current_marks: Option<Arc<[Mark]>>,
	sequences: usize,
	images: usize,
	fonts: FontCache,
	last_run: Option<(usize, usize, Option<[f64; 6]>)>,
}

impl Collector {
	/// Some PDFs draw a whole long document on every page and shift it so one window shows, and reading what's outside the page would repeat the document once per page.
	fn off_page(&self, bounds: &Bounds) -> bool {
		let page = self.page;
		page.width() > 0.0
			&& page.height() > 0.0
			&& (f64::from(bounds.right) < page.x0
				|| f64::from(bounds.left) > page.x1
				|| f64::from(bounds.top) < page.y0
				|| f64::from(bounds.bottom) > page.y1)
	}

	fn marks(&mut self) -> Arc<[Mark]> {
		self.current_marks.get_or_insert_with(|| self.marks.as_slice().into()).clone()
	}
}

impl<'a> Device<'a> for Collector {
	fn draw_path(&mut self, _: &BezPath, _: DrawProps<'a>, _: &DrawMode) {}
	fn push_clip_path(&mut self, _: &ClipPath) {}
	fn push_transparency_group(&mut self, _: f32, _: Option<SoftMask<'a>>, _: BlendMode) {}
	fn draw_glyph_run(&mut self, run: &GlyphRun<'_, 'a>, props: DrawProps<'a>, draw_mode: &DrawMode) {
		// A fill-and-stroke run arrives twice in a row from the same buffer, once per paint, and its text is only on the page once.
		let identity = (
			run.glyphs().as_ptr() as usize,
			run.glyphs().len(),
			run.glyphs().first().map(|glyph| glyph.transform().as_coeffs()),
		);
		let repeated = matches!(draw_mode, DrawMode::Stroke(_)) && self.last_run == Some(identity);
		self.last_run = Some(identity);
		// pdfium widens a stroked text object's box by half the line width, which is what keeps a stroked space, as a faked bold sets one, from reading as an empty object and being dropped.
		let stroke = match draw_mode {
			DrawMode::Stroke(stroke) | DrawMode::FillAndStroke(_, stroke) => Some(stroke.line_width / 2.0),
			_ => None,
		};
		if repeated {
			if let (Some(half), Some(text_object)) = (stroke, self.text_objects.last_mut()) {
				text_object.inflate(half);
				let rect = text_object.rect();
				if let Some(object) = self.objects.get_mut(text_object.object) {
					object.bounds =
						Some(Bounds { left: rect.left, bottom: rect.bottom, right: rect.right, top: rect.top });
				}
			}
			return;
		}
		let marks = self.marks();
		let object = self.objects.len();
		let Some(mut text_object) = TextObject::from_run(run, props.transform, marks.clone(), object, &mut self.fonts)
		else {
			return;
		};
		if let Some(half) = stroke {
			text_object.inflate(half);
		}
		let rect = text_object.rect();
		let bounds = Bounds { left: rect.left, bottom: rect.bottom, right: rect.right, top: rect.top };
		if self.off_page(&bounds) {
			// Its stroke pass must not widen the object before it.
			self.last_run = None;
			return;
		}
		self.objects.push(ObjectData { kind: ObjectKind::Other, bounds: Some(bounds), marks, image: None });
		self.text_objects.push(text_object);
	}
	fn draw_image(&mut self, image: Image<'a, '_>, props: ImageDrawProps<'a>) {
		let marks = self.marks();
		let bounds = image_bounds(props.transform, f64::from(image.width()), f64::from(image.height()));
		if !self.off_page(&bounds) {
			self.objects.push(ObjectData {
				kind: ObjectKind::Image,
				bounds: Some(bounds),
				marks,
				image: Some(self.images),
			});
		}
		self.images += 1;
	}
	fn pop_clip(&mut self) {}
	fn pop_transparency_group(&mut self) {}
	fn begin_marked_content(&mut self, tag: &[u8], properties: &MarkedContentProperties<'_>) {
		self.sequences += 1;
		self.marks.push(Mark {
			name: String::from_utf8_lossy(tag).into_owned(),
			mcid: properties.mcid(),
			actual_text: properties.actual_text().map(|text| decode_text_string(text.as_bytes())),
			sequence: self.sequences,
		});
		self.current_marks = None;
	}
	fn end_marked_content(&mut self) {
		self.marks.pop();
		self.current_marks = None;
	}
}

/// hayro's transform places the image's pixel grid rather than the unit square PDF images are defined on, so the corners are the image's own size.
fn image_bounds(transform: Affine, width: f64, height: f64) -> Bounds {
	let corners = [Point::new(0.0, 0.0), Point::new(width, 0.0), Point::new(0.0, height), Point::new(width, height)]
		.map(|p| transform * p);
	let xs = corners.map(|p| p.x as f32);
	let ys = corners.map(|p| p.y as f32);
	Bounds {
		left: xs.iter().copied().fold(f32::INFINITY, f32::min),
		bottom: ys.iter().copied().fold(f32::INFINITY, f32::min),
		right: xs.iter().copied().fold(f32::NEG_INFINITY, f32::max),
		top: ys.iter().copied().fold(f32::NEG_INFINITY, f32::max),
	}
}

struct ImageExtractor {
	wanted: usize,
	seen: usize,
	png: Option<Vec<u8>>,
}

impl<'a> Device<'a> for ImageExtractor {
	fn draw_path(&mut self, _: &BezPath, _: DrawProps<'a>, _: &DrawMode) {}
	fn push_clip_path(&mut self, _: &ClipPath) {}
	fn push_transparency_group(&mut self, _: f32, _: Option<SoftMask<'a>>, _: BlendMode) {}
	fn draw_glyph_run(&mut self, _: &GlyphRun<'_, 'a>, _: DrawProps<'a>, _: &DrawMode) {}
	fn draw_image(&mut self, image: Image<'a, '_>, _: ImageDrawProps<'a>) {
		let ordinal = self.seen;
		self.seen += 1;
		if ordinal != self.wanted || self.png.is_some() {
			return;
		}
		// The mask is not applied: the picture on its own is what was asked for, as with pdfium.
		let rgba = match image {
			Image::Raster(raster) => {
				let mut rgba = None;
				raster.with_rgba(
					|data, _| {
						rgba = Some(match data {
							ImageData::Rgb(rgb) => {
								let pixels =
									rgb.data.as_chunks::<3>().0.iter().flat_map(|&[r, g, b]| [r, g, b, 255]).collect();
								(rgb.width, rgb.height, pixels)
							}
							ImageData::Luma(luma) => {
								let pixels = luma.data.iter().flat_map(|&g| [g, g, g, 255]).collect();
								(luma.width, luma.height, pixels)
							}
						});
					},
					None,
				);
				rgba
			}
			Image::Stencil(stencil) => {
				let mut rgba = None;
				stencil.with_stencil(
					|luma, _| {
						let pixels = luma.data.iter().flat_map(|&g| [g, g, g, 255]).collect();
						rgba = Some((luma.width, luma.height, pixels));
					},
					None,
				);
				rgba
			}
		};
		let Some((width, height, pixels)) = rgba else { return };
		let mut out = Vec::new();
		if image::RgbaImage::from_raw(width, height, pixels)
			.and_then(|image| image.write_to(&mut std::io::Cursor::new(&mut out), image::ImageFormat::Png).ok())
			.is_some()
		{
			self.png = Some(out);
		}
	}
	fn pop_clip(&mut self) {}
	fn pop_transparency_group(&mut self) {}
}
