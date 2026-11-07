use std::{
	collections::HashMap,
	io::{Read, Seek},
	path::Path,
	sync::{Arc, Mutex, OnceLock, PoisonError},
};

use skrifa::{
	FontRef, GlyphId, MetadataProvider,
	attribute::Weight,
	instance::{LocationRef, Size},
};

use super::{
	lines::RunStyle,
	package::{Package, resolve_part},
	page::{Run, Span, clusters},
};

/// The first 32 bytes of an embedded font are often scrambled, so that it can't be picked out of the file and installed, with a key spelled out in the part's name.
fn unscrambled(mut data: Vec<u8>, part: &str) -> Option<Vec<u8>> {
	let name = Path::new(part).file_stem()?.to_str()?.replace('-', "");
	if name.len() != 32 || data.len() < 32 {
		return None;
	}
	let key: Vec<u8> =
		(0..16).map(|index| u8::from_str_radix(name.get(index * 2..index * 2 + 2)?, 16).ok()).collect::<Option<_>>()?;
	for (index, byte) in data.iter_mut().take(32).enumerate() {
		*byte ^= key[15 - index % 16];
	}
	Some(data)
}

/// Measured whole when it is read, so a run is measured without reading the font again.
struct Face {
	/// Each glyph's advance, in the font's units.
	advances: Vec<Option<f32>>,
	units_per_em: f64,
	/// From character to glyph.
	glyphs: HashMap<u32, u32>,
	bold: bool,
	monospaced: bool,
}

impl Face {
	fn load<R: Read + Seek>(package: &mut Package<R>, part: &str, index: u32) -> Option<Self> {
		let data = package.read_bytes(part).ok()?;
		let data = if FontRef::from_index(&data, index).is_ok() { data } else { unscrambled(data, part)? };
		let font = FontRef::from_index(&data, index).ok()?;
		let metrics = font.metrics(Size::unscaled(), LocationRef::default());
		let glyph_metrics = font.glyph_metrics(Size::unscaled(), LocationRef::default());
		let advances =
			(0..glyph_metrics.glyph_count()).map(|glyph| glyph_metrics.advance_width(GlyphId::new(glyph))).collect();
		Some(Self {
			advances,
			units_per_em: f64::from(metrics.units_per_em).max(1.0),
			glyphs: font.charmap().mappings().map(|(character, glyph)| (character, glyph.to_u32())).collect(),
			bold: font.attributes().weight >= Weight::BOLD,
			monospaced: metrics.is_monospace,
		})
	}

	/// In hundredths of the size, the unit `Indices` uses.
	fn of_glyph(&self, glyph: u32) -> Option<f64> {
		let advance = (*self.advances.get(usize::try_from(glyph).ok()?)?)?;
		Some(f64::from(advance) / self.units_per_em * 100.0)
	}

	fn of_character(&self, character: char) -> Option<f64> {
		self.of_glyph(*self.glyphs.get(&u32::from(character))?)
	}
}

/// Places glyphs in hundredths of the size. A right-to-left run draws each glyph leftward from its origin and advances leftward, and there a positive offset moves a glyph further left (ECMA-388, 12.1.6.2).
struct Pen {
	right_to_left: bool,
	at: f64,
	/// The left edge of each glyph placed so far: the first, and the extremes.
	first: Option<f64>,
	last: f64,
	left: f64,
	right: f64,
}

impl Pen {
	const fn new(right_to_left: bool) -> Self {
		Self { right_to_left, at: 0.0, first: None, last: 0.0, left: f64::INFINITY, right: f64::NEG_INFINITY }
	}

	/// `drawn` is how wide the glyph is, which a file can set its advance apart from to place the next glyph.
	fn place(&mut self, advance: f64, drawn: f64, offset: f64) {
		let left = if self.right_to_left { self.at - drawn - offset } else { self.at + offset };
		self.at += if self.right_to_left { -advance } else { advance };
		self.first.get_or_insert(left);
		self.last = left;
		self.left = self.left.min(left);
		self.right = self.right.max(left + drawn);
	}

	fn span(&self, size: f64) -> Span {
		let Some(first) = self.first else { return Span { left: 0.0, right: 0.0, leftward: self.right_to_left } };
		let leftward = self.right_to_left || self.last < first;
		Span { left: self.left / 100.0 * size, right: self.right / 100.0 * size, leftward }
	}
}

fn run_span(face: Option<&Face>, run: &Run) -> Option<Span> {
	let mut characters = run.source.chars();
	let mut pen = Pen::new(run.bidi_level % 2 == 1);
	for cluster in clusters(&run.indices) {
		let mut first = None;
		let mut code_units = 0;
		while code_units < cluster.code_units {
			let Some(character) = characters.next() else { break };
			first.get_or_insert(character);
			code_units += character.len_utf16();
		}
		if first.is_none() && cluster.advance.is_none() {
			continue;
		}
		let natural = face.and_then(|face| {
			cluster.glyph.and_then(|glyph| face.of_glyph(glyph)).or_else(|| face.of_character(first?))
		});
		let advance = cluster.advance.or(natural)?;
		pen.place(advance, natural.unwrap_or(advance), cluster.offset);
	}
	for character in characters {
		let natural = face?.of_character(character)?;
		pen.place(natural, natural, 0.0);
	}
	Some(pen.span(run.size))
}

type Slot = Arc<OnceLock<Option<Face>>>;

/// Each font is read once and shared by every worker, because the same few are named by every run of the file.
#[derive(Default)]
pub(super) struct Fonts {
	faces: Mutex<HashMap<String, Slot>>,
}

impl Fonts {
	fn face<R: Read + Seek>(&self, package: &mut Package<R>, page_part: &str, font: &str) -> Slot {
		let (uri, fragment) = font.split_once('#').unwrap_or((font, ""));
		let part = resolve_part(page_part, uri);
		let index = fragment.parse().unwrap_or(0);
		let mut faces = self.faces.lock().unwrap_or_else(PoisonError::into_inner);
		let slot = Arc::clone(faces.entry(format!("{part}#{index}")).or_default());
		drop(faces);
		// Loaded outside the lock, so workers that need different fonts don't wait on each other.
		slot.get_or_init(|| Face::load(package, &part, index));
		slot
	}

	/// Each font is found once for the page, by the name its runs give it.
	pub fn styles_of<R: Read + Seek>(&self, package: &mut Package<R>, page_part: &str, runs: &[Run]) -> Vec<RunStyle> {
		let mut faces: HashMap<&str, Slot> = HashMap::new();
		let mut styles = Vec::with_capacity(runs.len());
		for run in runs {
			let slot = faces.entry(run.font.as_str()).or_insert_with(|| self.face(package, page_part, &run.font));
			let face = slot.get().and_then(Option::as_ref);
			styles.push(RunStyle {
				span: run_span(face, run),
				bold: run.simulated_bold || face.is_some_and(|face| face.bold),
				monospaced: face.is_some_and(|face| face.monospaced),
			});
		}
		styles
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const GUID: &str = "2F909FF7-3927-4A67-97FC-C2A73886EE8B";

	fn be16(value: u16) -> [u8; 2] {
		value.to_be_bytes()
	}

	fn be32(value: u32) -> [u8; 4] {
		value.to_be_bytes()
	}

	fn sfnt(tables: &[(&[u8; 4], Vec<u8>)]) -> Vec<u8> {
		let count = u16::try_from(tables.len()).expect("table count");
		let mut font = vec![0, 1, 0, 0];
		font.extend(be16(count));
		font.extend([0; 6]);
		let mut offset = u32::from(count) * 16 + 12;
		for (tag, data) in tables {
			let length = u32::try_from(data.len()).expect("table length");
			font.extend(tag.iter());
			font.extend(be32(0));
			font.extend(be32(offset));
			font.extend(be32(length));
			offset += length;
		}
		font.extend(tables.iter().flat_map(|(_, data)| data.iter().copied()));
		font
	}

	/// `a` is half an em wide and `b` a whole one, in a font of a thousand units.
	fn test_font(weight: u16, fixed_pitch: bool) -> Vec<u8> {
		let mut head = vec![0; 54];
		head[1] = 1;
		head[12..16].copy_from_slice(&be32(0x5F0F_3CF5));
		head[18..20].copy_from_slice(&be16(1000));
		let mut hhea = vec![0; 36];
		hhea[1] = 1;
		hhea[34..36].copy_from_slice(&be16(3));
		let mut maxp = be32(0x0000_5000).to_vec();
		maxp.extend(be16(3));
		let hmtx: Vec<u8> = [0u16, 500, 1000].into_iter().flat_map(|width| [be16(width), be16(0)].concat()).collect();
		let mut cmap = [be16(0), be16(1), be16(3), be16(10)].concat();
		cmap.extend(be32(12));
		cmap.extend(
			[be16(12).to_vec(), be16(0).to_vec(), be32(28).to_vec(), be32(0).to_vec(), be32(1).to_vec()].concat(),
		);
		cmap.extend([be32(u32::from('a')), be32(u32::from('b')), be32(1)].concat());
		let mut os2 = vec![0; 78];
		os2[4..6].copy_from_slice(&be16(weight));
		let mut post = vec![0; 32];
		post[0..4].copy_from_slice(&be32(0x0003_0000));
		post[12..16].copy_from_slice(&be32(u32::from(fixed_pitch)));
		sfnt(&[
			(b"OS/2", os2),
			(b"cmap", cmap),
			(b"head", head),
			(b"hhea", hhea),
			(b"hmtx", hmtx),
			(b"maxp", maxp),
			(b"post", post),
		])
	}

	fn obfuscated(mut font: Vec<u8>) -> Vec<u8> {
		let key: Vec<u8> = (0..16)
			.map(|index| u8::from_str_radix(&GUID.replace('-', "")[index * 2..index * 2 + 2], 16).expect("hex"))
			.collect();
		for index in 0..32 {
			font[index] ^= key[15 - index % 16];
		}
		font
	}

	fn run(text: &str, indices: &str) -> Run {
		Run {
			text: text.to_string(),
			source: text.to_string(),
			indices: indices.to_string(),
			font: "F.ttf".to_string(),
			size: 10.0,
			..Run::default()
		}
	}

	fn style_in(fonts: &Fonts, package: &mut Package<std::io::Cursor<Vec<u8>>>, page: &str, run: &Run) -> RunStyle {
		fonts.styles_of(package, page, std::slice::from_ref(run)).remove(0)
	}

	fn style_of(font: &[u8], part: &str, run: &Run) -> RunStyle {
		let mut package = Package::from_bytes(&[(part, font)]);
		style_in(&Fonts::default(), &mut package, "page.fpage", run)
	}

	fn width_of(run: &Run) -> Option<f64> {
		style_of(&test_font(400, false), "F.ttf", run).span.map(Span::width)
	}

	#[test]
	fn the_font_gives_the_width_of_a_run() {
		assert_eq!(width_of(&run("ab", "")), Some(15.0));
	}

	#[test]
	fn a_declared_advance_counts_over_the_font() {
		assert_eq!(width_of(&run("ab", ",200")), Some(30.0));
	}

	#[test]
	fn a_declared_glyph_counts_over_the_character() {
		assert_eq!(width_of(&run("a", "2")), Some(10.0));
	}

	#[test]
	fn a_cluster_measures_as_one() {
		assert_eq!(width_of(&run("aab", "(2:1)1;")), Some(15.0));
	}

	#[test]
	fn an_obfuscated_font_is_read_from_its_part_name() {
		let part = format!("Resources/{GUID}.odttf");
		let font = obfuscated(test_font(400, false));
		let run = Run { font: part.clone(), ..run("ab", "") };
		assert_eq!(style_of(&font, &part, &run).span.map(Span::width), Some(15.0));
	}

	#[test]
	fn a_font_is_found_relative_to_its_page() {
		let run = Run { font: "../Resources/F.ttf".to_string(), ..run("ab", "") };
		let mut package = Package::from_bytes(&[("Resources/F.ttf", &test_font(400, false))]);
		assert_eq!(style_in(&Fonts::default(), &mut package, "Pages/1.fpage", &run).span.map(Span::width), Some(15.0));
	}

	#[test]
	fn a_font_collection_index_is_not_part_of_the_name() {
		let run = Run { font: "F.ttf#0".to_string(), ..run("ab", "") };
		assert_eq!(width_of(&run), Some(15.0));
	}

	#[test]
	fn a_missing_or_unreadable_font_leaves_the_width_unknown() {
		let run = Run { font: "Gone.ttf".to_string(), ..run("ab", "") };
		assert_eq!(width_of(&run), None);
		assert_eq!(style_of(b"not a font", "F.ttf", &run).span, None);
	}

	#[test]
	fn declared_advances_alone_measure_a_run_with_no_font() {
		let run = Run { font: "Gone.ttf".to_string(), ..run("ab", ",50;,100") };
		assert_eq!(width_of(&run), Some(15.0));
	}

	#[test]
	fn bold_comes_from_the_font_or_a_simulation() {
		assert!(style_of(&test_font(700, false), "F.ttf", &run("a", "")).bold);
		assert!(!style_of(&test_font(400, false), "F.ttf", &run("a", "")).bold);
		let simulated = Run { simulated_bold: true, ..run("a", "") };
		assert!(style_of(&test_font(400, false), "F.ttf", &simulated).bold);
	}

	#[test]
	fn monospace_comes_from_the_font() {
		assert!(style_of(&test_font(400, true), "F.ttf", &run("a", "")).monospaced);
		assert!(!style_of(&test_font(400, false), "F.ttf", &run("a", "")).monospaced);
	}

	#[test]
	fn glyphs_moved_left_by_their_offsets_are_measured_where_they_are_drawn() {
		let style = style_of(&test_font(400, false), "F.ttf", &run("ab", "1,0;2,,-100"));
		assert_eq!(style.span, Some(Span { left: -10.0, right: 5.0, leftward: true }));
	}

	#[test]
	fn a_right_to_left_run_is_drawn_leftward_from_its_origin() {
		let style = style_of(&test_font(400, false), "F.ttf", &Run { bidi_level: 1, ..run("ab", "") });
		assert_eq!(style.span, Some(Span { left: -15.0, right: 0.0, leftward: true }));
	}

	#[test]
	fn an_offset_on_a_right_to_left_run_moves_its_glyph_further_left() {
		let style = style_of(&test_font(400, false), "F.ttf", &Run { bidi_level: 1, ..run("a", ",,10") });
		assert_eq!(style.span, Some(Span { left: -6.0, right: -1.0, leftward: true }));
	}

	#[test]
	fn each_run_on_a_page_is_measured_with_its_own_font() {
		let (regular, bold) = (test_font(400, false), test_font(700, true));
		let mut package = Package::from_bytes(&[("R.ttf", &regular), ("B.ttf", &bold)]);
		let runs = [
			Run { font: "R.ttf".to_string(), ..run("a", "") },
			Run { font: "B.ttf".to_string(), ..run("ab", "") },
			Run { font: "R.ttf".to_string(), ..run("b", "") },
		];
		let styles = Fonts::default().styles_of(&mut package, "page.fpage", &runs);
		let found: Vec<_> =
			styles.iter().map(|style| (style.span.map(Span::width), style.bold, style.monospaced)).collect();
		assert_eq!(found, [(Some(5.0), false, false), (Some(15.0), true, true), (Some(10.0), false, false)]);
	}

	#[test]
	fn workers_with_their_own_package_share_each_font_they_load() {
		let font = test_font(400, false);
		let fonts = Fonts::default();
		std::thread::scope(|scope| {
			for _ in 0..2 {
				scope.spawn(|| {
					let mut package = Package::from_bytes(&[("F.ttf", &font)]);
					assert_eq!(
						style_in(&fonts, &mut package, "page.fpage", &run("ab", "")).span.map(Span::width),
						Some(15.0)
					);
				});
			}
		});
		assert_eq!(fonts.faces.lock().expect("lock").len(), 1);
	}

	#[test]
	fn a_font_is_read_once() {
		let mut package = Package::from_bytes(&[("F.ttf", &test_font(400, false))]);
		let fonts = Fonts::default();
		style_in(&fonts, &mut package, "page.fpage", &run("a", ""));
		assert_eq!(fonts.faces.lock().expect("lock").len(), 1);
		style_in(&fonts, &mut package, "page.fpage", &run("b", ""));
		assert_eq!(fonts.faces.lock().expect("lock").len(), 1);
	}
}
