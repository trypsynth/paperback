//! A page's text layer, built the way pdfium builds it.
//!
//! This is a port of pdfium's `CPDF_TextPage` (`core/fpdftext/cpdf_textpage.cpp`). The parser's heuristics were tuned against what that class produces: the order it puts characters in, the spaces and line breaks it generates and flags as generated, the hyphen it marks with 0x02, the ligatures it splits back into letters. A text layer designed afresh would read differently in all of those places, so the class is followed closely, down to its thresholds and its `f32` arithmetic, and departs from it only where hayro offers its information in a different shape.
//!
//! Portions are derived from `PDFium`, used under its BSD-style license:
//! Copyright 2014 The `PDFium` Authors. Original code copyright 2014 Foxit Software Inc. <http://www.foxitsoftware.com>

// The arithmetic follows pdfium's own float and integer conversions, and fusing its multiply-adds would round differently from pdfium.
#![allow(
	clippy::cast_possible_truncation,
	clippy::cast_precision_loss,
	clippy::cast_sign_loss,
	clippy::float_cmp,
	clippy::suboptimal_flops
)]

use std::{
	cell::{Cell, RefCell},
	collections::HashMap,
	rc::Rc,
};

use hayro_interpret::{
	font::{FontProperties, Glyph, GlyphRun, PositionedGlyph},
	hayro_cmap::{BfString, WritingMode},
	kurbo::{Affine, Point, Shape},
};
use hayro_syntax::object::{Array, Name};

use super::{
	bidi::{self, Direction},
	links,
	page::{Mark, PageObject, PdfPage},
};

/// pdfium's `CPDF_Font::kInvalidCharCode`: the code a generated character carries.
pub(super) const INVALID_CODE: u32 = u32::MAX;
const DEFAULT_FONT_SIZE: f32 = 1.0;
const SIZE_EPSILON: f32 = 0.01;
const FONT_FLAG_SYMBOLIC: i32 = 1 << 2;
const FONT_FLAG_NONSYMBOLIC: i32 = 1 << 5;
const FONT_FLAG_ITALIC: i32 = 1 << 6;

/// The box one glyph occupies on the page, in PDF points.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CharBox {
	pub left: f64,
	pub right: f64,
	pub bottom: f64,
	pub top: f64,
}

/// The text matrix in force for one glyph, which is what turns a nominal font size into the size the glyph is actually drawn at.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TextMatrix {
	pub a: f32,
	pub b: f32,
	pub c: f32,
	pub d: f32,
	pub e: f32,
	pub f: f32,
}

/// A glyph's font, as far as deciding whether it is monospaced needs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FontInfo {
	pub name: String,
	/// The PDF font descriptor flags, whose fixed-pitch bit is more reliable than the name.
	pub flags: i32,
}

/// pdfium's `CFX_FloatRect`.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub(super) struct FRect {
	pub(super) left: f32,
	pub(super) bottom: f32,
	pub(super) right: f32,
	pub(super) top: f32,
}

impl FRect {
	const fn new(left: f32, bottom: f32, right: f32, top: f32) -> Self {
		Self { left, bottom, right, top }
	}

	fn width(&self) -> f32 {
		self.right - self.left
	}

	fn height(&self) -> f32 {
		self.top - self.bottom
	}

	fn is_empty(&self) -> bool {
		self.left >= self.right || self.bottom >= self.top
	}

	const fn normalized(self) -> Self {
		Self::new(
			self.left.min(self.right),
			self.bottom.min(self.top),
			self.left.max(self.right),
			self.bottom.max(self.top),
		)
	}

	const fn union(&mut self, other: Self) {
		let (a, b) = (self.normalized(), other.normalized());
		*self = Self::new(a.left.min(b.left), a.bottom.min(b.bottom), a.right.max(b.right), a.top.max(b.top));
	}

	fn intersect(&mut self, other: Self) {
		let (a, b) = (self.normalized(), other.normalized());
		let result = Self::new(a.left.max(b.left), a.bottom.max(b.bottom), a.right.min(b.right), a.top.min(b.top));
		*self = if result.left > result.right || result.bottom > result.top { Self::default() } else { result };
	}

	fn contains(&self, (x, y): (f32, f32)) -> bool {
		let n = self.normalized();
		x <= n.right && x >= n.left && y <= n.top && y >= n.bottom
	}
}

/// pdfium's `CFX_Matrix`: `x' = a*x + c*y + e`, `y' = b*x + d*y + f`.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Matrix {
	a: f32,
	b: f32,
	c: f32,
	d: f32,
	e: f32,
	f: f32,
}

impl Matrix {
	const IDENTITY: Self = Self { a: 1.0, b: 0.0, c: 0.0, d: 1.0, e: 0.0, f: 0.0 };

	#[allow(clippy::many_single_char_names)]
	const fn from_array([a, b, c, d, e, f]: [f32; 6]) -> Self {
		Self { a, b, c, d, e, f }
	}

	fn transform(&self, (x, y): (f32, f32)) -> (f32, f32) {
		(self.a * x + self.c * y + self.e, self.b * x + self.d * y + self.f)
	}

	fn transform_rect(&self, rect: FRect) -> FRect {
		let corners =
			[(rect.left, rect.top), (rect.left, rect.bottom), (rect.right, rect.top), (rect.right, rect.bottom)]
				.map(|point| self.transform(point));
		let mut out = FRect::new(corners[0].0, corners[0].1, corners[0].0, corners[0].1);
		for (x, y) in corners {
			out.left = out.left.min(x);
			out.right = out.right.max(x);
			out.bottom = out.bottom.min(y);
			out.top = out.top.max(y);
		}
		out
	}

	fn x_unit(&self) -> f32 {
		if self.b == 0.0 {
			self.a.abs()
		} else if self.a == 0.0 {
			self.b.abs()
		} else {
			self.a.hypot(self.b)
		}
	}

	fn y_unit(&self) -> f32 {
		if self.c == 0.0 {
			self.d.abs()
		} else if self.d == 0.0 {
			self.c.abs()
		} else {
			self.c.hypot(self.d)
		}
	}

	fn transform_distance(&self, distance: f32) -> f32 {
		distance * (self.x_unit() + self.y_unit()) / 2.0
	}

	fn transform_x_distance(&self, dx: f32) -> f32 {
		(self.a * dx).hypot(self.b * dx)
	}

	/// The identity when the matrix cannot be inverted, as pdfium does.
	fn inverse(&self) -> Self {
		let det = self.a * self.d - self.b * self.c;
		if det == 0.0 {
			return Self::IDENTITY;
		}
		let (a, b, c, d) = (self.d / det, -self.b / det, -self.c / det, self.a / det);
		Self { a, b, c, d, e: -(a * self.e + c * self.f), f: -(b * self.e + d * self.f) }
	}
}

/// pdfium's `FX_RECT` of a glyph's bounds, in thousandths of the font size.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct GlyphBox {
	left: i32,
	bottom: i32,
	right: i32,
	top: i32,
}

/// What pdfium's `CPDF_Font` would answer about one font, gathered from its dictionary and from the glyphs drawn in it.
pub(super) struct FontData {
	name: String,
	flags: i32,
	stem_v: i32,
	simple: bool,
	first_char: u32,
	widths: Vec<f32>,
	missing_width: f32,
	/// Widths and boxes of the codes drawn so far, which is all a composite or standard font can say without its font program.
	observed: RefCell<HashMap<u32, (f32, GlyphBox)>>,
	space_code: Cell<Option<u32>>,
	/// The descriptor's ascent and descent, in thousandths of the font size, for a glyph whose outline is missing.
	ascent: i32,
	descent: i32,
}

impl FontData {
	fn of(properties: Option<&FontProperties<'_>>) -> Self {
		let Some(properties) = properties else {
			return Self {
				name: String::new(),
				flags: FONT_FLAG_NONSYMBOLIC,
				stem_v: 0,
				simple: true,
				first_char: 0,
				widths: Vec::new(),
				missing_width: 0.0,
				observed: RefCell::new(HashMap::new()),
				space_code: Cell::new(None),
				ascent: 800,
				descent: -200,
			};
		};
		let dict = properties.dict();
		let name = properties.base_font().map(|name| name.as_str().to_string()).unwrap_or_default();
		let simple = dict.get::<Name<'_>>(b"Subtype").as_deref() != Some(b"Type0".as_slice());
		// pdfium fills in the flags of a standard font that has no descriptor, and marks italic any font that slants.
		let mut flags = properties.flags().map_or_else(
			|| {
				let bare = name.rsplit('+').next().unwrap_or(&name);
				if bare.starts_with("Symbol") || bare.starts_with("ZapfDingbats") {
					FONT_FLAG_SYMBOLIC
				} else {
					FONT_FLAG_NONSYMBOLIC
				}
			},
			|flags| flags.bits().cast_signed(),
		);
		if properties.italic_angle().is_some_and(|angle| angle != 0.0) {
			flags |= FONT_FLAG_ITALIC;
		}
		let stem_v = properties.stem_v().map_or(0, |stem| stem as i32);
		let widths = dict.get::<Array<'_>>(b"Widths").map(|widths| widths.iter::<f32>().collect()).unwrap_or_default();
		let missing_width =
			properties.descriptor().and_then(|descriptor| descriptor.get::<f32>(b"MissingWidth")).unwrap_or(0.0);
		Self {
			name,
			flags,
			stem_v,
			simple,
			first_char: dict.get::<u32>(b"FirstChar").unwrap_or(0),
			widths,
			missing_width,
			observed: RefCell::new(HashMap::new()),
			space_code: Cell::new(None),
			ascent: properties.ascent().filter(|ascent| *ascent > 0.0).map_or(800, |ascent| ascent.round() as i32),
			descent: properties
				.descent()
				.filter(|descent| *descent < 0.0)
				.map_or(-200, |descent| descent.round() as i32),
		}
	}

	/// pdfium's `CPDF_Font::GetCharWidth`, in thousandths of the font size.
	fn char_width(&self, code: u32) -> i32 {
		if code == INVALID_CODE {
			return 0;
		}
		if self.simple
			&& let Some(width) = code.checked_sub(self.first_char).and_then(|index| self.widths.get(index as usize))
		{
			return width.round() as i32;
		}
		if let Some((width, _)) = self.observed.borrow().get(&code) {
			return width.round() as i32;
		}
		self.missing_width.round() as i32
	}

	fn char_bbox(&self, code: u32) -> GlyphBox {
		self.observed.borrow().get(&code).map(|(_, bbox)| *bbox).unwrap_or_default()
	}

	/// pdfium's `CPDF_Font::CharCodeFromUnicode(' ')`.
	fn space_code(&self) -> Option<u32> {
		self.space_code.get().or_else(|| self.simple.then_some(32))
	}

	/// pdfium's `CPDF_Font::GetFontWeight`, which it works out from the stem width rather than reading `/FontWeight`.
	const fn weight(&self) -> i32 {
		if self.stem_v < 140 {
			self.stem_v.saturating_mul(5)
		} else {
			self.stem_v.saturating_mul(4).saturating_add(140)
		}
	}
}

/// The textpage's `GetCharWidth` helper: the width, failing that the width of the glyph's box.
fn char_width_or_box(code: u32, font: &FontData) -> i32 {
	if code == INVALID_CODE {
		return 0;
	}
	let width = font.char_width(code);
	if width > 0 {
		return width;
	}
	let bbox = font.char_bbox(code);
	if bbox.right < bbox.left || bbox.top < bbox.bottom {
		return 0;
	}
	(bbox.right - bbox.left).max(0)
}

/// The fonts a page's runs are set in, one [`FontData`] per font dictionary.
#[derive(Default)]
pub(super) struct FontCache {
	fonts: HashMap<usize, Rc<FontData>>,
	fallback: Option<Rc<FontData>>,
}

impl FontCache {
	fn font(&mut self, properties: Option<&FontProperties<'_>>) -> Rc<FontData> {
		let Some(properties) = properties else {
			return self.fallback.get_or_insert_with(|| Rc::new(FontData::of(None))).clone();
		};
		let key = properties.dict().data().as_ptr() as usize;
		self.fonts.entry(key).or_insert_with(|| Rc::new(FontData::of(Some(properties)))).clone()
	}
}

/// One character of a text object: pdfium's `CPDF_TextObject::Item`.
struct Item {
	code: u32,
	unicode: Vec<u32>,
	/// Where the glyph starts, in the text object's own space.
	origin: (f32, f32),
}

/// One `Tj` or `TJ`: pdfium's `CPDF_TextObject`.
pub(super) struct TextObject {
	/// Which of the page's objects this is.
	pub(super) object: usize,
	font: Rc<FontData>,
	font_size: f32,
	/// The font size as drawn horizontally, pdfium's `GetFontSizeH`.
	font_size_h: f32,
	char_space: f32,
	/// Text space to page space, translated to [`TextObject::pos`]; pdfium's `GetTextMatrix`.
	matrix: Matrix,
	pos: (f32, f32),
	items: Vec<Item>,
	/// The `TJ` adjustment after each item, in thousandths of text space; pdfium's `GetCharKernings`.
	kernings: Vec<f32>,
	rect: FRect,
	marks: Rc<[Mark]>,
}

impl TextObject {
	/// Reads one glyph run as a text object, or `None` for one that draws nothing a text layer could use.
	pub(super) fn from_run(
		run: &GlyphRun<'_, '_>,
		ctm: Affine,
		marks: Rc<[Mark]>,
		object: usize,
		fonts: &mut FontCache,
	) -> Option<Self> {
		let glyphs = run.glyphs();
		let first = glyphs.first()?;
		let state = run.text_properties();
		let (size, scale) = (f64::from(state.font_size), f64::from(state.horizontal_scaling));
		if size == 0.0 || scale == 0.0 {
			return None;
		}
		let font = fonts.font(run.font());
		// A glyph's transform is the text matrix times the font size and horizontal scaling, over the 1000 units of glyph space. Taking those back out of the first glyph's leaves pdfium's text matrix, which keeps the font size apart.
		let g = first.transform().as_coeffs();
		let text = [
			g[0] * 1000.0 / (size * scale),
			g[1] * 1000.0 / (size * scale),
			g[2] * 1000.0 / size,
			g[3] * 1000.0 / size,
		];
		let c = ctm.as_coeffs();
		let linear = [
			c[0] * text[0] + c[2] * text[1],
			c[1] * text[0] + c[3] * text[1],
			c[0] * text[2] + c[2] * text[3],
			c[1] * text[2] + c[3] * text[3],
		];
		let origin_of = |glyph: &PositionedGlyph<'_>| {
			let point = ctm * glyph.transform() * Point::ZERO;
			(point.x as f32, point.y as f32)
		};
		let pos = origin_of(first);
		let matrix = Matrix {
			a: linear[0] as f32,
			b: linear[1] as f32,
			c: linear[2] as f32,
			d: linear[3] as f32,
			e: pos.0,
			f: pos.1,
		};
		let inverse = matrix.inverse();
		let mut items = Vec::with_capacity(glyphs.len());
		let mut kernings = Vec::with_capacity(glyphs.len());
		for (index, glyph) in glyphs.iter().enumerate() {
			let code = glyph.char_code();
			let unicode: Vec<u32> = match glyph.as_unicode() {
				Some(BfString::Char(ch)) => vec![u32::from(ch)],
				Some(BfString::String(text)) => text.chars().map(u32::from).collect(),
				None => Vec::new(),
			};
			if unicode == [32] && font.space_code.get().is_none() {
				font.space_code.set(Some(code));
			}
			if !font.observed.borrow().contains_key(&code) {
				let (width, bbox) = match &**glyph {
					Glyph::Outline(outline) => {
						let bounds = outline.outline().bounding_box();
						let bbox = if bounds.width() > 0.0 || bounds.height() > 0.0 {
							GlyphBox {
								left: bounds.x0.round() as i32,
								bottom: bounds.y0.round() as i32,
								right: bounds.x1.round() as i32,
								top: bounds.y1.round() as i32,
							}
						} else {
							GlyphBox::default()
						};
						let width = outline.advance_width().unwrap_or(0.0);
						// A visible character with no outline is one the font hayro substituted cannot draw, such as Cyrillic in a missing Times New Roman. pdfium finds a system font with the glyph and so sees it, so it is given the box the glyph would have rather than being taken for an empty, invisible object.
						let missing = bbox == GlyphBox::default()
							&& width > 0.0
							&& !unicode.is_empty()
							&& unicode.iter().any(|&c| char::from_u32(c).is_some_and(|ch| !ch.is_whitespace()));
						let bbox = if missing {
							GlyphBox { left: 0, bottom: font.descent, right: width.round() as i32, top: font.ascent }
						} else {
							bbox
						};
						(width, bbox)
					}
					Glyph::Type3(_) => (0.0, GlyphBox::default()),
				};
				font.observed.borrow_mut().insert(code, (width, bbox));
			}
			let user = origin_of(glyph);
			let origin = inverse.transform(user);
			items.push(Item { code, unicode, origin });
			kernings.push(glyphs.get(index + 1).map_or(0.0, PositionedGlyph::adjustment));
		}
		let font_size = state.font_size;
		let font_size_h = matrix.a.hypot(matrix.c) * font_size;
		let rect = object_rect(&items, &font, font_size, &matrix, run.writing_mode() == WritingMode::Vertical);
		Some(Self {
			object,
			font,
			font_size,
			font_size_h,
			char_space: state.char_spacing,
			matrix,
			pos,
			items,
			kernings,
			rect,
			marks,
		})
	}

	pub(super) const fn rect(&self) -> FRect {
		self.rect
	}

	/// pdfium's `CFX_FloatRect::Inflate`, for a stroked object's box.
	pub(super) fn inflate(&mut self, by: f32) {
		let rect = self.rect.normalized();
		self.rect = FRect::new(rect.left - by, rect.bottom - by, rect.right + by, rect.top + by);
	}

	/// pdfium's `CPDF_TextObject::GetCharWidth`.
	fn char_width(&self, code: u32) -> f32 {
		self.font.char_width(code) as f32 * self.font_size / 1000.0
	}
}

/// The box a text object covers on the page: its glyphs' boxes, from the first glyph's start to the last's end, in page space. pdfium computes it the same way while laying the object out.
fn object_rect(items: &[Item], font: &FontData, font_size: f32, matrix: &Matrix, vertical: bool) -> FRect {
	let scale = font_size / 1000.0;
	let (mut min_x, mut min_y, mut max_x, mut max_y) = (10000.0_f32, 10000.0_f32, -10000.0_f32, -10000.0_f32);
	for item in items {
		let bbox = font.char_bbox(item.code);
		let (x, y) = item.origin;
		if vertical {
			min_x = min_x.min(bbox.left as f32 * scale + x).min(bbox.right as f32 * scale + x);
			max_x = max_x.max(bbox.left as f32 * scale + x).max(bbox.right as f32 * scale + x);
			min_y = min_y.min(y);
			max_y = max_y.max(y);
		} else {
			min_x = min_x.min(x + bbox.left as f32 * scale);
			max_x = max_x.max(x + bbox.right as f32 * scale);
			min_y = min_y.min(y + bbox.bottom as f32 * scale);
			max_y = max_y.max(y + bbox.top as f32 * scale);
		}
	}
	if items.is_empty() || min_x > max_x {
		return FRect::default();
	}
	matrix.transform_rect(FRect::new(min_x, min_y, max_x, max_y))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CharType {
	Normal,
	Generated,
	NotUnicode,
	Hyphen,
	Piece,
	ActualText,
}

#[derive(Debug, Clone)]
struct CharInfo {
	char_type: CharType,
	char_code: u32,
	unicode: u32,
	origin: (f32, f32),
	char_box: FRect,
	matrix: Matrix,
	text_object: Option<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Orientation {
	Unknown,
	Horizontal,
	Vertical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Generate {
	None,
	Space,
	LineBreak,
	Hyphen,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MarkedContent {
	Pass,
	Done,
	Delay,
}

fn normalize_threshold(threshold: f32, t1: i32, t2: i32, t3: i32) -> f32 {
	if threshold < t1 as f32 {
		threshold / 2.0
	} else if threshold < t2 as f32 {
		threshold / 4.0
	} else if threshold < t3 as f32 {
		threshold / 5.0
	} else {
		threshold / 6.0
	}
}

fn calculate_base_space(object: &TextObject, matrix: &Matrix) -> f32 {
	let char_space = object.char_space;
	if char_space == 0.0 || object.items.len() < 2 {
		return 0.0;
	}
	let mut has_kerning = false;
	let spacing = matrix.transform_distance(char_space);
	let mut base_space = spacing;
	for &kerning in &object.kernings {
		if kerning != 0.0 {
			base_space = base_space.min(-object.font_size_h * kerning / 1000.0 + spacing);
			has_kerning = true;
		}
	}
	if base_space < 0.0 || (object.items.len() == 2 && has_kerning) {
		return 0.0;
	}
	base_space
}

fn calculate_base_space_adjustment(object: &TextObject, matrix: &Matrix) -> f32 {
	if object.char_space > 0.001 {
		-matrix.transform_distance(object.char_space)
	} else if object.char_space < -0.001 {
		matrix.transform_distance(object.char_space.abs())
	} else {
		0.0
	}
}

fn calculate_space_threshold(font: &FontData, font_size_h: f32, code: u32) -> f32 {
	let mut threshold = font.space_code().map_or(0.0, |space| font_size_h * font.char_width(space) as f32 / 1000.0);
	if threshold > font_size_h / 3.0 {
		threshold = 0.0;
	} else {
		threshold /= 2.0;
	}
	if threshold == 0.0 {
		threshold = char_width_or_box(code, font) as f32;
		threshold = normalize_threshold(threshold, 300, 500, 700);
		threshold = font_size_h * threshold / 1000.0;
	}
	threshold
}

fn generate_space(pos: (f32, f32), last_pos: f32, this_width: f32, last_width: f32, threshold: f32) -> bool {
	if (last_pos + last_width - pos.0).abs() <= threshold {
		return false;
	}
	let threshold_pos = threshold + last_width;
	let pos_difference = pos.0 - last_pos;
	if pos_difference.abs() > threshold_pos {
		return true;
	}
	if pos.0 < 0.0 && -threshold_pos > pos_difference {
		return true;
	}
	pos_difference > this_width + last_width
}

fn end_horizontal_line(this_rect: FRect, prev_rect: FRect) -> bool {
	if this_rect.height() <= 4.5 || prev_rect.height() <= 4.5 {
		return false;
	}
	this_rect.bottom.max(prev_rect.bottom) >= this_rect.top.min(prev_rect.top)
}

fn end_vertical_line(this_rect: FRect, prev_rect: FRect, line: FRect, this_size: f32, prev_size: f32) -> bool {
	if this_rect.width() <= this_size * 0.1 || prev_rect.width() <= prev_size * 0.1 {
		return false;
	}
	this_rect.right.min(line.right) <= this_rect.left.max(line.left)
}

fn is_control_char(info: &CharInfo) -> bool {
	matches!(info.unicode, 0x2 | 0x3 | 0x93 | 0x94 | 0x96 | 0x97 | 0x98 | 0xFFFE) && info.char_type != CharType::Hyphen
}

const fn is_hyphen_code(c: u32) -> bool {
	c == 0x2D || c == 0xAD
}

fn is_normal_character(info: &CharInfo) -> bool {
	if info.unicode != 0 { !is_control_char(info) } else { info.char_code != 0 }
}

fn is_alpha(c: u32) -> bool {
	char::from_u32(c).is_some_and(char::is_alphabetic)
}

fn is_alphanumeric(c: u32) -> bool {
	char::from_u32(c).is_some_and(char::is_alphanumeric)
}

/// pdfium's `GetUnicodeNormalization`, for the characters it is ever asked about: right-to-left text and the Latin ligatures. Its table maps the presentation forms to the letters they present, composed (an Arabic waw-with-hamza form becomes U+0624, not a waw and a combining hamza), which is NFKC; anything outside those blocks it leaves as it is.
fn unicode_normalization(c: u32) -> Vec<u32> {
	let Some(ch) = char::from_u32(c) else { return vec![c] };
	if !matches!(c, 0xFB00..=0xFDFF | 0xFE70..=0xFEFF) {
		return vec![c];
	}
	let normalizer = icu_normalizer::ComposingNormalizerBorrowed::new_nfkc();
	let normalized: Vec<u32> = normalizer.normalize(ch.encode_utf8(&mut [0; 4])).chars().map(u32::from).collect();
	if normalized.is_empty() { vec![c] } else { normalized }
}

/// The parts of `CPDF_TextPage` that run while the page is laid out.
struct Builder<'p> {
	objects: &'p [TextObject],
	page_width: f32,
	page_height: f32,
	display: Matrix,
	char_list: Vec<CharInfo>,
	temp_char_list: Vec<CharInfo>,
	text_buf: Vec<u32>,
	temp_text_buf: Vec<u32>,
	prev_text_obj: Option<usize>,
	text_objects: Vec<usize>,
	textline_dir: Orientation,
	curline_rect: FRect,
}

impl<'p> Builder<'p> {
	fn new(objects: &'p [TextObject], page_width: f32, page_height: f32, display: Matrix) -> Self {
		Self {
			objects,
			page_width,
			page_height,
			display,
			char_list: Vec::new(),
			temp_char_list: Vec::new(),
			text_buf: Vec::new(),
			temp_text_buf: Vec::new(),
			prev_text_obj: None,
			text_objects: Vec::new(),
			textline_dir: Orientation::Unknown,
			curline_rect: FRect::default(),
		}
	}

	fn process_object(&mut self) {
		if self.objects.is_empty() {
			return;
		}
		self.textline_dir = self.find_textline_flow_orientation();
		for index in 0..self.objects.len() {
			self.process_text_object(index);
		}
		self.process_transformed_text_objects();
		self.text_objects.clear();
		self.close_temp_line();
	}

	fn find_textline_flow_orientation(&self) -> Orientation {
		let width = self.page_width as i32;
		let height = self.page_height as i32;
		if width <= 0 || height <= 0 {
			return Orientation::Unknown;
		}
		let mut horizontal = vec![false; width as usize];
		let mut vertical = vec![false; height as usize];
		let mut line_height = 0.0_f32;
		let (mut start_h, mut end_h, mut start_v, mut end_v) = (width, 0, height, 0);
		for object in self.objects {
			let rect = object.rect;
			if !(rect.left.is_finite() && rect.right.is_finite() && rect.bottom.is_finite() && rect.top.is_finite()) {
				continue;
			}
			let min_h = rect.left.clamp(0.0, width as f32) as i32;
			let max_h = rect.right.clamp(0.0, width as f32) as i32;
			let min_v = rect.bottom.clamp(0.0, height as f32) as i32;
			let max_v = rect.top.clamp(0.0, height as f32) as i32;
			if min_h >= max_h || min_v >= max_v {
				continue;
			}
			horizontal[min_h as usize..max_h as usize].fill(true);
			vertical[min_v as usize..max_v as usize].fill(true);
			start_h = start_h.min(min_h);
			end_h = end_h.max(max_h);
			start_v = start_v.min(min_v);
			end_v = end_v.max(max_v);
			if line_height <= 0.0 {
				line_height = rect.height();
			}
		}
		let double_line_height = (2.0 * line_height) as i32;
		if end_v - start_v < double_line_height {
			return Orientation::Horizontal;
		}
		if end_h - start_h < double_line_height {
			return Orientation::Vertical;
		}
		let filled = |mask: &[bool], start: i32, end: i32| {
			if start >= end {
				return 0.0;
			}
			mask[start as usize..end as usize].iter().filter(|&&set| set).count() as f32 / (end - start) as f32
		};
		let sum_h = filled(&horizontal, start_h, end_h);
		if sum_h > 0.8 {
			return Orientation::Horizontal;
		}
		let sum_v = filled(&vertical, start_v, end_v);
		if sum_h > sum_v {
			Orientation::Horizontal
		} else if sum_h < sum_v {
			Orientation::Vertical
		} else {
			Orientation::Unknown
		}
	}

	fn process_text_object(&mut self, index: usize) {
		let object = &self.objects[index];
		if object.rect.width().abs() < SIZE_EPSILON {
			return;
		}
		if self.text_objects.is_empty() {
			self.text_objects.push(index);
			return;
		}
		if self.is_same_as_pre_text_object(index) {
			return;
		}
		let prev = &self.objects[*self.text_objects.last().expect("checked to be non-empty")];
		let Some(last) = prev.items.last() else { return };
		let prev_width = char_width_or_box(last.code, &prev.font) as f32 * prev.font_size / 1000.0;
		let prev_width = prev.matrix.transform_distance(prev_width.abs());
		let Some(first) = object.items.first() else { return };
		let this_width = (char_width_or_box(first.code, &object.font) as f32 * object.font_size / 1000.0).abs();
		let this_width = object.matrix.transform_distance(this_width.abs());
		let threshold = prev_width.max(this_width) / 4.0;
		let prev_pos = self.display.transform(prev.pos);
		let this_pos = self.display.transform(object.pos);
		if (this_pos.1 - prev_pos.1).abs() > threshold * 2.0 {
			self.process_transformed_text_objects();
			self.text_objects.clear();
			self.text_objects.push(index);
			return;
		}
		for i in (1..=self.text_objects.len()).rev() {
			let new_prev_pos = self.display.transform(self.objects[self.text_objects[i - 1]].pos);
			if this_pos.0 >= new_prev_pos.0 {
				self.text_objects.insert(i, index);
				return;
			}
		}
		self.text_objects.insert(0, index);
	}

	fn is_same_as_pre_text_object(&self, index: usize) -> bool {
		(0..index).rev().take(5).any(|other| self.is_same_text_object(other, index))
	}

	fn is_same_text_object(&self, first: usize, second: usize) -> bool {
		let (one, two) = (&self.objects[first], &self.objects[second]);
		let mut prev_obj_rect = two.rect;
		let current_obj_rect = one.rect;
		if prev_obj_rect.is_empty() && current_obj_rect.is_empty() {
			let xdiff = (prev_obj_rect.left - current_obj_rect.left).abs();
			let count = self.char_list.len();
			if count >= 2 && xdiff > self.char_list[count - 2].char_box.width() {
				return false;
			}
		}
		if !prev_obj_rect.is_empty() || !current_obj_rect.is_empty() {
			prev_obj_rect.intersect(current_obj_rect);
			if prev_obj_rect.is_empty() {
				return false;
			}
			if (prev_obj_rect.width() - current_obj_rect.width()).abs() > current_obj_rect.width() / 2.0 {
				return false;
			}
			if two.font_size != one.font_size {
				return false;
			}
		}
		if two.items.len() != one.items.len() {
			return false;
		}
		let Some(last) = two.items.last() else { return true };
		if one.items.iter().zip(&two.items).any(|(a, b)| a.code != b.code) {
			return false;
		}
		let diff = (one.pos.0 - two.pos.0, one.pos.1 - two.pos.1);
		let font_size = two.font_size;
		let char_size = char_width_or_box(last.code, &two.font) as f32;
		let max_pre_size = prev_obj_rect.height().max(prev_obj_rect.width()).max(font_size);
		diff.0.abs() <= 0.9 * char_size * font_size / 1000.0 && diff.1.abs() <= max_pre_size / 8.0
	}

	fn process_transformed_text_objects(&mut self) {
		for index in self.text_objects.clone() {
			let object = &self.objects[index];
			if object.rect.width().abs() < SIZE_EPSILON {
				continue;
			}
			let marked = self.pre_marked_content(index);
			if marked == MarkedContent::Done {
				self.prev_text_obj = Some(index);
				continue;
			}
			if self.prev_text_obj.is_some() {
				let generate = self.process_insert_object(index);
				if generate == Generate::LineBreak {
					self.curline_rect = object.rect;
				} else {
					self.curline_rect.union(object.rect);
				}
				if !self.process_generate_character(generate, index) {
					continue;
				}
			} else {
				self.curline_rect = object.rect;
			}
			let (char_index, buf_index) = (self.temp_char_list.len(), self.temp_text_buf.len());
			if marked == MarkedContent::Delay {
				if self.process_marked_content(index) {
					self.reverse_temp_text_bufs(char_index, buf_index);
				}
				self.prev_text_obj = Some(index);
				continue;
			}
			self.prev_text_obj = Some(index);
			if self.process_text_object_items(index) {
				self.reverse_temp_text_bufs(char_index, buf_index);
			}
		}
	}

	/// The `/ActualText` of an object's marks and the sequence that carries it: the last mark with properties, which is the one pdfium ends up holding.
	fn actual_text(object: &TextObject) -> (bool, String, Option<usize>) {
		let mut exists = false;
		let mut text = String::new();
		let mut sequence = None;
		for mark in object.marks.iter() {
			if mark.mcid.is_none() && mark.actual_text.is_none() {
				continue;
			}
			sequence = Some(mark.sequence);
			text = mark.actual_text.clone().unwrap_or_default();
			exists |= mark.actual_text.is_some();
		}
		(exists, text, sequence)
	}

	fn pre_marked_content(&self, index: usize) -> MarkedContent {
		let object = &self.objects[index];
		if object.marks.is_empty() {
			return MarkedContent::Pass;
		}
		let (exists, actual_text, sequence) = Self::actual_text(object);
		if !exists {
			return MarkedContent::Pass;
		}
		if let Some(prev) = self.prev_text_obj {
			let prev_marks = &self.objects[prev].marks;
			if prev_marks.len() == object.marks.len() && prev_marks.last().map(|mark| mark.sequence) == sequence {
				return MarkedContent::Done;
			}
		}
		if actual_text.is_empty() {
			return MarkedContent::Pass;
		}
		let printable =
			actual_text.chars().map(u32::from).any(|wc| (wc > 0x80 && wc < 0xFFFD) || (0x20..0x7F).contains(&wc));
		if printable { MarkedContent::Delay } else { MarkedContent::Done }
	}

	fn process_marked_content(&mut self, index: usize) -> bool {
		let object = &self.objects[index];
		let (_, actual_text, _) = Self::actual_text(object);
		if actual_text.is_empty() {
			return false;
		}
		let chars: Vec<u32> = actual_text.chars().map(u32::from).collect();
		let is_rtl = self.is_right_to_left(object)
			|| (self.prev_text_obj.is_some_and(|prev| self.is_right_to_left(&self.objects[prev]))
				&& bidi::overall_direction(&chars) == Direction::Right);
		let mut rect = object.rect;
		let length = chars.len() as f32;
		let step = if is_rtl {
			rect.left = rect.right - rect.width() / length;
			-rect.width()
		} else {
			rect.right = rect.left + rect.width() / length;
			rect.width()
		};
		for (k, &wc) in chars.iter().enumerate() {
			let wc = if wc <= 0x80 && !(0x20..0x7F).contains(&wc) { 0x20 } else { wc };
			if wc >= 0xFFFD {
				continue;
			}
			let mut char_box = rect;
			char_box.left += k as f32 * step;
			char_box.right += k as f32 * step;
			self.temp_text_buf.push(wc);
			self.temp_char_list.push(CharInfo {
				char_type: CharType::ActualText,
				char_code: INVALID_CODE,
				unicode: wc,
				origin: object.pos,
				char_box,
				matrix: object.matrix,
				text_object: Some(index),
			});
		}
		is_rtl
	}

	fn is_right_to_left(&self, object: &TextObject) -> bool {
		let _ = self;
		let text: Vec<u32> = object
			.items
			.iter()
			.filter_map(|item| {
				let wc = item.unicode.first().copied().filter(|&c| c != 0).unwrap_or(item.code);
				(wc != 0).then_some(wc)
			})
			.collect();
		bidi::overall_direction(&text) == Direction::Right
	}

	fn reverse_temp_text_bufs(&mut self, char_index: usize, buf_index: usize) {
		if char_index < self.temp_char_list.len() {
			self.temp_char_list[char_index..].reverse();
		}
		if buf_index < self.temp_text_buf.len() {
			self.temp_text_buf[buf_index..].reverse();
		}
	}

	fn writing_mode(&self, object: &TextObject) -> Orientation {
		if object.items.len() <= 1 {
			return self.textline_dir;
		}
		let first = object.matrix.transform(object.items[0].origin);
		let last = object.matrix.transform(object.items[object.items.len() - 1].origin);
		let (dx, dy) = ((last.0 - first.0).abs(), (last.1 - first.1).abs());
		if dx <= 0.0001 && dy <= 0.0001 {
			return Orientation::Unknown;
		}
		let length = dx.hypot(dy);
		let (vx, vy) = (dx / length, dy / length);
		let under = vx <= 0.0872;
		if vy <= 0.0872 {
			if under { self.textline_dir } else { Orientation::Horizontal }
		} else if under {
			Orientation::Vertical
		} else {
			self.textline_dir
		}
	}

	fn is_hyphen(&self, current: u32) -> bool {
		let text = if self.temp_text_buf.is_empty() { &self.text_buf } else { &self.temp_text_buf };
		if text.is_empty() {
			return false;
		}
		let mut at = text.len() - 1;
		while at > 0 && text[at] == 0x20 {
			at -= 1;
		}
		if !is_hyphen_code(text[at]) {
			return false;
		}
		if at > 0 && is_alpha(text[at - 1]) && is_alphanumeric(current) {
			return true;
		}
		self.prev_char_info().is_some_and(|prev| {
			matches!(prev.char_type, CharType::Piece | CharType::ActualText) && is_hyphen_code(prev.unicode)
		})
	}

	fn prev_char_info(&self) -> Option<&CharInfo> {
		self.temp_char_list.last().or_else(|| self.char_list.last())
	}

	fn first_char(object: &TextObject) -> u32 {
		object.items.first().map_or(0, |item| item.unicode.first().copied().unwrap_or(item.code))
	}

	fn process_insert_object(&mut self, index: usize) -> Generate {
		if let Some(object) = self.prev_char_info().and_then(|prev| prev.text_object) {
			self.prev_text_obj = Some(object);
		}
		let object = &self.objects[index];
		let Some(prev_index) = self.prev_text_obj else { return Generate::None };
		let prev = &self.objects[prev_index];
		let mut writing_mode = self.writing_mode(object);
		if writing_mode == Orientation::Unknown {
			writing_mode = self.writing_mode(prev);
		}
		let (Some(prev_item), Some(item)) = (prev.items.last(), object.items.first()) else { return Generate::None };
		let (this_rect, prev_rect) = (object.rect, prev.rect);
		let current = Self::first_char(object);
		let line_ended = match writing_mode {
			Orientation::Horizontal => end_horizontal_line(this_rect, prev_rect),
			Orientation::Vertical => {
				end_vertical_line(this_rect, prev_rect, self.curline_rect, object.font_size, prev.font_size)
			}
			Orientation::Unknown => false,
		};
		if line_ended {
			return if self.is_hyphen(current) { Generate::Hyphen } else { Generate::LineBreak };
		}
		let last_pos = prev_item.origin.0;
		let last_width_units = char_width_or_box(prev_item.code, &prev.font);
		let last_width = (last_width_units as f32 * prev.font_size / 1000.0).abs();
		let this_width_units = char_width_or_box(item.code, &object.font);
		let this_width = (this_width_units as f32 * object.font_size / 1000.0).abs();
		let mut threshold = last_width.max(this_width) / 4.0;
		let prev_inverse = prev.matrix.inverse();
		let pos = prev_inverse.transform(object.pos);
		if last_width < this_width {
			threshold = prev_inverse.transform_distance(threshold);
		}
		let mut is_newline = false;
		if writing_mode == Orientation::Horizontal {
			let rect_height = prev.rect.height();
			let rect = prev.rect.normalized();
			if (rect.is_empty() && rect_height > 5.0)
				|| ((pos.1 > threshold * 2.0 || pos.1 < threshold * -3.0)
					&& (pos.1.abs() >= 1.0 || pos.1.abs() > pos.0.abs()))
			{
				is_newline = true;
				if prev.items.len() > 1 {
					let first_item = &prev.items[0];
					let (m, d) = (prev.matrix, self.display);
					if prev_item.origin.0 > first_item.origin.0
						&& d.a > 0.9
						&& d.b < 0.1
						&& d.c < 0.1
						&& d.d < -0.9
						&& m.b < 0.1
						&& m.c < 0.1
					{
						let prev_band = FRect::new(0.0, prev.rect.bottom, 1000.0, prev.rect.top);
						if prev_band.contains(object.pos)
							|| FRect::new(0.0, object.rect.bottom, 1000.0, object.rect.top).contains(prev.pos)
						{
							is_newline = false;
						}
					}
				}
			}
		}
		if is_newline {
			return if self.is_hyphen(current) { Generate::Hyphen } else { Generate::LineBreak };
		}
		if object.items.len() == 1 && is_hyphen_code(current) && self.is_hyphen(current) {
			return Generate::Hyphen;
		}
		if current == u32::from(' ') {
			return Generate::None;
		}
		if prev_item.unicode.last() == Some(&u32::from(' ')) {
			return Generate::None;
		}
		let mut threshold2 = last_width_units.max(this_width_units) as f32;
		threshold2 = normalize_threshold(threshold2, 400, 700, 800);
		if last_width_units >= this_width_units {
			threshold2 *= prev.font_size.abs();
		} else {
			threshold2 *= object.font_size.abs();
			threshold2 = object.matrix.transform_distance(threshold2);
			threshold2 = prev_inverse.transform_distance(threshold2);
		}
		threshold2 /= 1000.0;
		if (threshold2 < 1.4881 && threshold2 > 1.4879) || (threshold2 < 1.39001 && threshold2 > 1.38999) {
			threshold2 *= 1.5;
		}
		if generate_space(pos, last_pos, this_width, last_width, threshold2) { Generate::Space } else { Generate::None }
	}

	fn process_generate_character(&mut self, generate: Generate, index: usize) -> bool {
		match generate {
			Generate::None => true,
			Generate::Space => {
				self.append_generated_character(u32::from(' '), true);
				true
			}
			Generate::LineBreak => {
				self.close_temp_line();
				if !self.text_buf.is_empty() {
					self.append_generated_character(u32::from('\r'), false);
					self.append_generated_character(u32::from('\n'), false);
				}
				true
			}
			Generate::Hyphen => {
				let object = &self.objects[index];
				if object.items.len() == 1 && is_hyphen_code(Self::first_char(object)) {
					return false;
				}
				while self.temp_text_buf.last() == Some(&0x20) {
					self.temp_text_buf.pop();
					self.temp_char_list.pop();
				}
				let Some(info) = self.temp_char_list.last_mut() else { return true };
				self.temp_text_buf.pop();
				info.char_type = CharType::Hyphen;
				info.unicode = 0x2;
				self.temp_text_buf.push(0xFFFE);
				true
			}
		}
	}

	fn append_generated_character(&mut self, unicode: u32, use_temp: bool) {
		let Some(info) = self.generate_char_info(unicode) else { return };
		if use_temp {
			self.temp_text_buf.push(unicode);
			self.temp_char_list.push(info);
		} else {
			self.text_buf.push(unicode);
			self.char_list.push(info);
		}
	}

	fn generate_char_info(&self, unicode: u32) -> Option<CharInfo> {
		let prev = self.prev_char_info()?;
		let object = prev.text_object.map(|index| &self.objects[index]);
		let pre_width = match object {
			Some(object) if prev.char_code != INVALID_CODE => char_width_or_box(prev.char_code, &object.font),
			_ => 0,
		};
		let mut font_size = object.map_or_else(|| prev.char_box.height(), |object| object.font_size);
		if font_size == 0.0 {
			font_size = DEFAULT_FONT_SIZE;
		}
		let origin = (prev.origin.0 + pre_width as f32 * font_size / 1000.0, prev.origin.1);
		Some(CharInfo {
			char_type: CharType::Generated,
			char_code: INVALID_CODE,
			unicode,
			origin,
			char_box: FRect::new(origin.0, origin.1, origin.0, origin.1),
			matrix: Matrix::IDENTITY,
			text_object: None,
		})
	}

	fn process_text_object_items(&mut self, index: usize) -> bool {
		let object = &self.objects[index];
		let matrix = object.matrix;
		let base_space = calculate_base_space(object, &matrix) + calculate_base_space_adjustment(object, &matrix);
		let font = &object.font;
		let mut spacing = 0.0_f32;
		for (i, item) in object.items.iter().enumerate() {
			if i > 0 && object.kernings[i - 1] != 0.0 {
				let text = if self.temp_text_buf.is_empty() { &self.text_buf } else { &self.temp_text_buf };
				if text.last().is_some_and(|&last| last != u32::from(' ')) {
					spacing = -object.font_size_h * object.kernings[i - 1] / 1000.0;
				}
			}
			spacing -= base_space;
			if spacing != 0.0 && i > 0 {
				let threshold = calculate_space_threshold(font, object.font_size_h, item.code);
				if threshold != 0.0 && spacing >= threshold {
					self.temp_text_buf.push(u32::from(' '));
					let origin = matrix.transform(item.origin);
					self.temp_char_list.push(CharInfo {
						char_type: CharType::Generated,
						char_code: INVALID_CODE,
						unicode: u32::from(' '),
						origin,
						char_box: FRect::new(origin.0, origin.1, origin.0, origin.1),
						matrix: Matrix::IDENTITY,
						text_object: Some(index),
					});
				}
			}
			spacing = 0.0;
			let (unicode, char_type) = if item.unicode.is_empty() && item.code != 0 {
				(vec![item.code], CharType::NotUnicode)
			} else {
				(item.unicode.clone(), CharType::Normal)
			};
			let bbox = font.char_bbox(item.code);
			let size = object.font_size / 1000.0;
			let mut char_box = FRect::new(
				bbox.left as f32 * size + item.origin.0,
				bbox.bottom as f32 * size + item.origin.1,
				bbox.right as f32 * size + item.origin.0,
				bbox.top as f32 * size + item.origin.1,
			);
			if (char_box.top - char_box.bottom).abs() < SIZE_EPSILON {
				char_box.top = char_box.bottom + size;
			}
			if (char_box.right - char_box.left).abs() < SIZE_EPSILON {
				char_box.right = char_box.left + object.char_width(item.code);
			}
			let char_box = matrix.transform_rect(char_box);
			let mut info = CharInfo {
				char_type,
				char_code: item.code,
				unicode: 0,
				origin: matrix.transform(item.origin),
				char_box,
				matrix,
				text_object: Some(index),
			};
			if unicode.is_empty() {
				self.temp_char_list.push(info);
				self.temp_text_buf.push(0xFFFE);
				continue;
			}
			// A glyph overprinted at almost the same place in the same font is fake bold, and reads once.
			let threshold = info.matrix.transform_x_distance(0.07 * object.font_size);
			let count = self.temp_char_list.len().min(7);
			let overprinted = self.temp_char_list[self.temp_char_list.len() - count..].iter().rev().any(|candidate| {
				candidate.char_code == info.char_code
					&& candidate.text_object.is_some_and(|other| Rc::ptr_eq(&self.objects[other].font, font))
					&& (candidate.origin.0 - info.origin.0).abs() < threshold
					&& (candidate.origin.1 - info.origin.1).abs() < threshold
			});
			if !overprinted {
				for c in unicode {
					info.unicode = c;
					self.temp_text_buf.push(if c == 0 { 0xFFFE } else { c });
					self.temp_char_list.push(info.clone());
				}
			} else if i == 0 && self.temp_text_buf.last() == Some(&u32::from(' ')) {
				self.temp_text_buf.pop();
				self.temp_char_list.pop();
			}
		}
		self.is_right_to_left(object) && (matrix.a * matrix.d - matrix.b * matrix.c) < 0.0
	}

	fn close_temp_line(&mut self) {
		if self.temp_char_list.is_empty() {
			return;
		}
		let mut write = 0;
		let mut prev_is_space = false;
		for read in 0..self.temp_text_buf.len() {
			let ch = self.temp_text_buf[read];
			if ch == 0x20 && prev_is_space {
				continue;
			}
			prev_is_space = ch == 0x20;
			if write != read {
				self.temp_char_list[write] = self.temp_char_list[read].clone();
				self.temp_text_buf[write] = ch;
			}
			write += 1;
		}
		self.temp_char_list.truncate(write);
		self.temp_text_buf.truncate(write);
		let text = std::mem::take(&mut self.temp_text_buf);
		let chars = std::mem::take(&mut self.temp_char_list);
		let order = bidi::BidiString::new(&text);
		let mut current = order.overall_direction();
		for segment in order.segments() {
			let range = segment.start..segment.start + segment.count;
			if segment.direction == Direction::Right
				|| (segment.direction == Direction::Neutral && current == Direction::Right)
			{
				current = Direction::Right;
				if chars[range.clone()].first().is_some_and(|info| info.char_type == CharType::ActualText) {
					for i in range {
						self.add_char_info(text[i], chars[i].clone(), true);
					}
				} else {
					for i in range.rev() {
						self.add_char_info(text[i], chars[i].clone(), true);
					}
				}
			} else {
				if segment.direction != Direction::LeftWeak {
					current = Direction::Left;
				}
				for i in range {
					self.add_char_info(text[i], chars[i].clone(), false);
				}
			}
		}
	}

	fn add_char_info(&mut self, wc: u32, mut info: CharInfo, is_rtl: bool) {
		if !is_normal_character(&info) {
			self.char_list.push(info);
			return;
		}
		let wc = if is_rtl { bidi::mirror(wc) } else { wc };
		if !is_rtl && !(0xFB00..=0xFB06).contains(&wc) {
			self.text_buf.push(wc);
			self.char_list.push(info);
			return;
		}
		info.char_type = CharType::Piece;
		for c in unicode_normalization(wc) {
			info.unicode = c;
			self.text_buf.push(c);
			self.char_list.push(info.clone());
		}
	}
}

/// A page's text, laid out the way pdfium lays it out.
pub struct PdfTextPage<'page> {
	page: &'page PdfPage<'page>,
	chars: Vec<CharInfo>,
	text: Vec<u32>,
	/// pdfium's `char_indices_`: runs of characters that have a place in [`PdfTextPage::text`], as (first index, count).
	segments: Vec<(i32, i32)>,
}

impl<'page> PdfTextPage<'page> {
	pub(super) fn of(page: &'page PdfPage<'page>) -> Self {
		let display = Matrix::from_array(page.display_matrix());
		let mut builder = Builder::new(&page.content().text_objects, page.width(), page.height(), display);
		builder.process_object();
		let chars = builder.char_list;
		let text = builder.text_buf;
		let mut segments = Vec::new();
		if !chars.is_empty() {
			segments.push((0, 0));
		}
		let mut skipped = false;
		for (i, info) in chars.iter().enumerate() {
			let next = i32::try_from(i + 1).unwrap_or(i32::MAX);
			let last = segments.last_mut().expect("pushed for a non-empty page");
			if info.char_type == CharType::Generated || is_normal_character(info) {
				last.1 += 1;
				skipped = true;
			} else if skipped {
				segments.push((next, 0));
				skipped = false;
			} else {
				last.0 = next;
			}
		}
		Self { page, chars, text, segments }
	}

	fn info(&self, index: i32) -> Option<&CharInfo> {
		self.chars.get(usize::try_from(index).ok()?)
	}

	fn object_of(&self, index: i32) -> Option<&TextObject> {
		Some(&self.page.content().text_objects[self.info(index)?.text_object?])
	}

	/// How many characters the page holds. pdfium can fail to say; this cannot, but keeps its signature so the parser is written once for both.
	#[allow(clippy::unnecessary_wraps)]
	pub fn char_count(&self) -> Option<i32> {
		Some(i32::try_from(self.chars.len()).unwrap_or(i32::MAX))
	}

	/// The whole page as one string.
	pub fn text(&self) -> String {
		self.extract(0, self.char_count().unwrap_or(0))
	}

	fn text_index_from_char_index(&self, char_index: i32) -> i32 {
		let mut count = 0;
		for &(index, length) in &self.segments {
			let text_index = char_index - index;
			if text_index < length {
				return if text_index >= 0 { text_index + count } else { -1 };
			}
			count += length;
		}
		-1
	}

	/// The characters of the page's text from `start` for `count` characters: pdfium's `GetPageText`.
	pub(super) fn page_text(&self, mut start: i32, count: i32) -> Vec<u32> {
		let total = self.char_count().unwrap_or(0);
		if start < 0 || start >= total || count <= 0 || self.text.is_empty() {
			return Vec::new();
		}
		let mut text_start = self.text_index_from_char_index(start);
		while text_start < 0 {
			if start >= total {
				return Vec::new();
			}
			start += 1;
			text_start = self.text_index_from_char_index(start);
		}
		let count = count.min(total - start);
		let mut last = start + count - 1;
		let mut text_last = self.text_index_from_char_index(last);
		while text_last < 0 {
			if last < text_start {
				return Vec::new();
			}
			last -= 1;
			text_last = self.text_index_from_char_index(last);
		}
		if text_last < text_start {
			return Vec::new();
		}
		let (from, to) = (text_start as usize, (text_last as usize + 1).min(self.text.len()));
		self.text.get(from..to).map(<[u32]>::to_vec).unwrap_or_default()
	}

	/// `count` characters starting at `start`.
	pub fn extract(&self, start: i32, count: i32) -> String {
		self.page_text(start, count).into_iter().filter_map(char::from_u32).collect()
	}

	pub fn unicode_at(&self, index: i32) -> u32 {
		self.info(index).map_or(0, |info| info.unicode)
	}

	/// Where the glyph at `index` sits, as the point its baseline starts from.
	pub fn char_origin(&self, index: i32) -> Option<(f64, f64)> {
		self.info(index).map(|info| (f64::from(info.origin.0), f64::from(info.origin.1)))
	}

	pub fn char_box(&self, index: i32) -> Option<CharBox> {
		let r = self.info(index)?.char_box;
		Some(CharBox {
			left: f64::from(r.left),
			right: f64::from(r.right),
			bottom: f64::from(r.bottom),
			top: f64::from(r.top),
		})
	}

	/// The weight of a glyph's font, 400 for regular and 700 for bold, or `None` where it cannot be told.
	pub fn font_weight(&self, index: i32) -> Option<i32> {
		let weight = self.object_of(index)?.font.weight();
		(weight > 0).then_some(weight)
	}

	pub fn font_size(&self, index: i32) -> f64 {
		f64::from(self.object_of(index).map_or(DEFAULT_FONT_SIZE, |object| object.font_size))
	}

	pub fn text_matrix(&self, index: i32) -> Option<TextMatrix> {
		let m = self.info(index)?.matrix;
		Some(TextMatrix { a: m.a, b: m.b, c: m.c, d: m.d, e: m.e, f: m.f })
	}

	/// The font the glyph at `index` is set in. Like pdfium, a character with no font, such as a generated space, reports an empty name and no flags rather than nothing.
	#[allow(clippy::unnecessary_wraps)]
	pub fn font(&self, index: i32) -> Option<FontInfo> {
		let Some(object) = self.object_of(index) else { return Some(FontInfo { name: String::new(), flags: 0 }) };
		Some(FontInfo { name: object.font.name.clone(), flags: object.font.flags })
	}

	/// Whether the character was generated rather than found in the content stream, as the spaces inferred between words that are merely far apart are.
	pub fn is_generated(&self, index: i32) -> bool {
		self.info(index).is_some_and(|info| info.char_type == CharType::Generated)
	}

	/// The object that drew the glyph at `index`, which carries the marked-content id linking it to the tag tree.
	pub fn text_object(&self, index: i32) -> Option<PageObject<'_>> {
		let object = self.object_of(index)?.object;
		Some(PageObject::new(&self.page.content().objects[object]))
	}

	/// The text inside a rectangle on the page, which is how a link annotation's words are read: pdfium's `GetTextByRect`.
	pub fn bounded_text(&self, left: f64, top: f64, right: f64, bottom: f64) -> String {
		let rect = FRect::new(left as f32, bottom as f32, right as f32, top as f32);
		let mut posy = 0.0_f32;
		let mut contains_pre_char = false;
		let mut add_line_feed = false;
		let mut out = String::new();
		for info in &self.chars {
			let mut char_box = info.char_box;
			char_box.intersect(rect);
			if !char_box.is_empty() {
				if (posy - info.origin.1).abs() > 0.0 && !contains_pre_char && add_line_feed {
					posy = info.origin.1;
					if !out.is_empty() {
						out.push_str("\r\n");
					}
				}
				contains_pre_char = true;
				add_line_feed = false;
				if info.unicode != 0
					&& let Some(ch) = char::from_u32(info.unicode)
				{
					out.push(ch);
				}
			} else if info.unicode == u32::from(' ') {
				if contains_pre_char {
					out.push(' ');
					contains_pre_char = false;
					add_line_feed = false;
				}
			} else {
				contains_pre_char = false;
				add_line_feed = true;
			}
		}
		out
	}

	/// The URLs and email addresses that can be found in the page's text, with the character range each covers: pdfium's `CPDF_LinkExtract`.
	pub fn web_links(&self) -> Vec<(i32, i32, String)> {
		let total = self.chars.len();
		let page_text = self.page_text(0, i32::try_from(total).unwrap_or(i32::MAX));
		let chars: Vec<links::LinkChar> = self
			.chars
			.iter()
			.map(|info| links::LinkChar {
				generated: info.char_type == CharType::Generated,
				hyphen: info.char_type == CharType::Hyphen,
				normal: info.char_type == CharType::Normal,
				unicode: info.unicode,
			})
			.collect();
		links::extract(&chars, &page_text)
	}
}
