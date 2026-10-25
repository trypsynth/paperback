//! One page: its size, its text layer, its tag tree, the objects drawn on it, and rasterizing it for OCR.

// The surface hands coordinates out as `f32`, as pdfium's does, while hayro works in `f64`.
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
	PdfDocument, PdfError, PdfTextPage, TagTree, decode_text_string, fonts,
	text::{FontCache, TextObject},
};

/// How much of the page one image can cover and still be a picture on the page rather than the page itself. A scanned book with an OCR text layer draws each sheet as one image under its invisible text, and announcing that put an "[Image]" with nothing behind it on every page (#1046).
pub const PAGE_IMAGE_COVERAGE: f64 = 0.9;

/// How far the parser follows form `XObject`s into each other. hayro already draws a form's contents as the page's own objects, so no object here is ever a form and the parser never gets that far.
pub const MAX_FORM_DEPTH: u32 = 8;

/// Shortest side, in PDF user units, an image must have to be worth a line of its own. A page laid out from HTML draws its spacers and its hairline rules as images a fraction of a point across.
const MIN_IMAGE_SIDE: f32 = 4.0;

/// A page, read in full when it is loaded and borrowed from the document it belongs to.
///
/// It owns what was read, the text layer included, so pages can be read on several threads at once and handed back to one.
pub struct PdfPage<'doc> {
	document: &'doc PdfDocument,
	index: usize,
	width: f32,
	height: f32,
	objects: Vec<ObjectData>,
	text: PdfTextPage,
}

/// The fonts and objects hayro has already read, kept across the pages one thread loads. A book sets every page in the same few fonts, and reading them again for each page was half of what loading a page cost.
pub struct PageCache<'doc>(InterpreterCache<'doc>);

impl PageCache<'_> {
	pub(super) fn new() -> Self {
		Self(InterpreterCache::new())
	}
}

impl<'doc> PdfPage<'doc> {
	pub(super) fn load(document: &'doc PdfDocument, index: i32, cache: &PageCache<'doc>) -> Result<Self, PdfError> {
		let index = usize::try_from(index).map_err(|_| PdfError::Failed)?;
		let page = document.pdf().pages().get(index).ok_or(PdfError::Failed)?;
		let (width, height) = page.render_dimensions();
		// The bounds only decide what gets drawn, and text off the page is still text to pdfium, so they are made too large to leave anything out.
		let mut context = Context::new(
			Affine::IDENTITY,
			Rect::new(-1e7, -1e7, 1e7, 1e7),
			&cache.0,
			document.pdf().xref(),
			fonts::settings(),
		);
		let mut collector = Collector::default();
		interpret_page(page, &mut context, &mut collector);
		let text = PdfTextPage::build(&collector.text_objects, &collector.objects, width, height, display_matrix(page));
		Ok(Self { document, index, width, height, objects: collector.objects, text })
	}

	pub const fn width(&self) -> f32 {
		self.width
	}

	pub const fn height(&self) -> f32 {
		self.height
	}

	/// The page's text layer. Always there; an `Option` because the parser is written for a page whose text cannot be read.
	#[allow(clippy::unnecessary_wraps)]
	pub const fn text(&self) -> Option<&PdfTextPage> {
		Some(&self.text)
	}

	/// The page's structure tree, or `None` when the document is not tagged.
	pub fn tags(&self) -> Option<TagTree<'_>> {
		Some(TagTree::of(self.document.structure()?, i32::try_from(self.index).ok()?))
	}

	pub fn object_count(&self) -> i32 {
		i32::try_from(self.objects.len()).unwrap_or(i32::MAX)
	}

	pub fn object(&self, index: i32) -> Option<PageObject<'_>> {
		let data = self.objects.get(usize::try_from(index).ok()?)?;
		Some(PageObject { data })
	}

	/// Every image the page draws that is worth a line of its own, nearest the top first.
	pub fn announced_images(&self) -> Vec<PageImage<'_>> {
		let page_area = f64::from(self.width) * f64::from(self.height);
		let mut images: Vec<PageImage<'_>> = self
			.images()
			.filter(|(_, data)| {
				data.bounds.is_some_and(|bounds| {
					is_announced_image(
						(bounds.right - bounds.left).abs(),
						(bounds.top - bounds.bottom).abs(),
						page_area,
					)
				})
			})
			.map(|(index, data)| PageImage::new(index, data))
			.collect();
		images.sort_by(|a, b| b.top.total_cmp(&a.top));
		images
	}

	/// The picture at `path`, encoded as PNG. `path` is the one an announced image carries in [`PageImage::path`].
	///
	/// The picture is taken at the resolution the file stores rather than as it is drawn on the page, so a photo placed small still saves at full size.
	pub fn image_png(&self, path: &str) -> Option<Vec<u8>> {
		let index: usize = path.parse().ok()?;
		let ordinal = self.objects.get(index)?.image?;
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

	/// The image path for each marked-content id on the page. A tagged PDF's `Figure` element points at its picture by marked-content id, and this is what turns that into a reference that can be saved.
	pub fn image_paths_by_mcid(&self) -> std::collections::HashMap<i32, String> {
		let mut paths = std::collections::HashMap::new();
		for (index, data) in self.images() {
			for mcid in data.marked_content_ids() {
				paths.entry(mcid).or_insert_with(|| index.to_string());
			}
		}
		paths
	}

	/// Rasterizes the page `width` pixels across, keeping its aspect ratio, as RGBA8 on white.
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

	/// Every link annotation on the page, with the box it covers and where it points.
	///
	/// A link may carry a URI action, a destination, or an action that itself carries a destination. All three are tried here so the parser only ever sees the answer.
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
		let rect = Bounds {
			left: rect.x0.min(rect.x1) as f32,
			bottom: rect.y0.min(rect.y1) as f32,
			right: rect.x0.max(rect.x1) as f32,
			top: rect.y0.max(rect.y1) as f32,
		};
		Some(AnnotationLink { rect, target })
	}

	fn images(&self) -> impl Iterator<Item = (usize, &ObjectData)> {
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
	/// Somewhere inside this document, by zero-based page index.
	Page(i32),
}

/// One link annotation: the box on the page a reader would activate, and its destination.
#[derive(Debug, Clone, PartialEq)]
pub struct AnnotationLink {
	pub rect: Bounds,
	pub target: LinkTarget,
}

/// One image a page draws, as Paperback announces it.
pub struct PageImage<'page> {
	/// The image's top edge, in PDF user units. Y grows up the page, so the nearest the top has the largest one.
	pub top: f64,
	/// Where the image sits among the page's objects. This is what an image marker's reference carries, so [`PdfPage::image_png`] can find the picture again.
	pub path: String,
	_page: std::marker::PhantomData<&'page ()>,
}

impl PageImage<'_> {
	fn new(index: usize, data: &ObjectData) -> Self {
		let top = data.bounds.map_or(0.0, |bounds| f64::from(bounds.top));
		Self { top, path: index.to_string(), _page: std::marker::PhantomData }
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
	/// A form `XObject`. hayro draws a form's contents in its place, so no object is ever one, but the parser's walk through forms matches on it.
	#[allow(dead_code)]
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

/// One marked-content sequence an object sits inside.
#[derive(Debug, Clone)]
pub(super) struct Mark {
	pub(super) name: String,
	pub(super) mcid: Option<i32>,
	pub(super) actual_text: Option<String>,
	/// Which `BDC` opened it, so two objects can tell whether they share one sequence the way pdfium tells by comparing property dictionaries.
	pub(super) sequence: usize,
}

/// One object the page draws: a text run or an image.
#[derive(Clone)]
pub(super) struct ObjectData {
	pub(super) kind: ObjectKind,
	pub(super) bounds: Option<Bounds>,
	/// Outermost first, the order pdfium lists an object's marks in.
	pub(super) marks: Arc<[Mark]>,
	/// For an image, how many images the page drew before it, which is how [`PdfPage::image_png`] finds it again.
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

/// One object on a page, borrowed from the page that collected it.
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

	pub const fn bounds(&self) -> Option<Bounds> {
		self.data.bounds
	}

	/// Always zero: hayro draws a form's contents as the page's own objects.
	#[allow(clippy::unused_self)]
	pub const fn form_object_count(&self) -> i32 {
		0
	}

	#[allow(clippy::unused_self)]
	pub const fn form_object(&self, _index: i32) -> Option<Self> {
		None
	}

	/// The outermost mark carrying an id, which is the one pdfium reports.
	pub fn marked_content_id(&self) -> Option<i32> {
		self.data.marked_content_id()
	}

	pub fn mark_count(&self) -> i32 {
		i32::try_from(self.data.marks.len()).unwrap_or(i32::MAX)
	}

	pub fn mark(&self, index: i32) -> Option<ObjectMark<'page>> {
		let mark = self.data.marks.get(usize::try_from(index).ok()?)?;
		Some(ObjectMark { mark })
	}
}

/// One marked-content mark on an object.
pub struct ObjectMark<'object> {
	mark: &'object Mark,
}

impl ObjectMark<'_> {
	/// The mark's tag as the content stream spells it: `P`, `Span`, `Artifact` and so on.
	pub fn name(&self) -> Option<String> {
		Some(self.mark.name.clone()).filter(|name| !name.is_empty())
	}

	/// An integer parameter on this mark. Only `MCID` is ever asked for, so it is the only one kept.
	pub fn param_int(&self, key: &str) -> Option<i32> {
		if key == "MCID" { self.mark.mcid } else { None }
	}
}

/// Collects what a page draws into [`PageContent`].
#[derive(Default)]
struct Collector {
	objects: Vec<ObjectData>,
	text_objects: Vec<TextObject>,
	marks: Vec<Mark>,
	/// The open marks as one shared list, rebuilt only when a mark opens or closes, since a page's objects mostly sit inside the same few.
	current_marks: Option<Arc<[Mark]>>,
	sequences: usize,
	images: usize,
	fonts: FontCache,
	last_run: Option<(usize, usize, Option<[f64; 6]>)>,
}

impl Collector {
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
		self.objects.push(ObjectData { kind: ObjectKind::Other, bounds: Some(bounds), marks, image: None });
		self.text_objects.push(text_object);
	}
	fn draw_image(&mut self, image: Image<'a, '_>, props: ImageDrawProps<'a>) {
		let marks = self.marks();
		let bounds = image_bounds(props.transform, f64::from(image.width()), f64::from(image.height()));
		self.objects.push(ObjectData {
			kind: ObjectKind::Image,
			bounds: Some(bounds),
			marks,
			image: Some(self.images),
		});
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

/// The box an image fills. hayro's transform places the image's pixel grid rather than the unit square PDF images are defined on, so the corners are the image's own size.
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

/// Decodes the `wanted`-th image a page draws into a PNG.
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
