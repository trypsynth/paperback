use std::{
	collections::HashMap,
	io::{Read, Seek},
	path::Path,
};

use skrifa::{
	FontRef, GlyphId, MetadataProvider,
	attribute::Weight,
	charmap::Charmap,
	instance::{LocationRef, Size},
	metrics::GlyphMetrics,
};

use super::{
	lines::RunStyle,
	package::{Package, resolve_part},
	page::{Run, clusters},
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

struct Face {
	data: Vec<u8>,
	index: u32,
}

impl Face {
	fn load<R: Read + Seek>(package: &mut Package<R>, part: &str, index: u32) -> Option<Self> {
		let data = package.read_bytes(part).ok()?;
		if FontRef::from_index(&data, index).is_ok() {
			return Some(Self { data, index });
		}
		let data = unscrambled(data, part)?;
		FontRef::from_index(&data, index).is_ok().then_some(Self { data, index })
	}

	fn font(&self) -> Option<FontRef<'_>> {
		FontRef::from_index(&self.data, self.index).ok()
	}
}

/// Advances in hundredths of the size, the unit `Indices` uses.
struct Metrics<'a> {
	charmap: Charmap<'a>,
	glyphs: GlyphMetrics<'a>,
	units_per_em: f64,
}

impl<'a> Metrics<'a> {
	fn of(font: &FontRef<'a>) -> Self {
		let units_per_em = f64::from(font.metrics(Size::unscaled(), LocationRef::default()).units_per_em).max(1.0);
		Self {
			charmap: font.charmap(),
			glyphs: font.glyph_metrics(Size::unscaled(), LocationRef::default()),
			units_per_em,
		}
	}

	fn of_glyph(&self, glyph: GlyphId) -> Option<f64> {
		Some(f64::from(self.glyphs.advance_width(glyph)?) / self.units_per_em * 100.0)
	}

	fn of_character(&self, character: char) -> Option<f64> {
		self.of_glyph(self.charmap.map(character)?)
	}
}

fn run_width(font: Option<&FontRef>, run: &Run) -> Option<f64> {
	let metrics = font.map(Metrics::of);
	let mut characters = run.source.chars();
	let mut hundredths = 0.0;
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
		let natural = || {
			let metrics = metrics.as_ref()?;
			cluster
				.glyph
				.and_then(|glyph| metrics.of_glyph(GlyphId::new(glyph)))
				.or_else(|| metrics.of_character(first?))
		};
		hundredths += cluster.advance.or_else(natural)?;
	}
	for character in characters {
		hundredths += metrics.as_ref()?.of_character(character)?;
	}
	Some(hundredths / 100.0 * run.size)
}

/// Each font is read once, because the same few are named by every run of the file.
#[derive(Default)]
pub(super) struct Fonts {
	faces: HashMap<String, Option<Face>>,
}

impl Fonts {
	pub fn style_of<R: Read + Seek>(&mut self, package: &mut Package<R>, page_part: &str, run: &Run) -> RunStyle {
		let (uri, fragment) = run.font.split_once('#').unwrap_or((&run.font, ""));
		let part = resolve_part(page_part, uri);
		let index = fragment.parse().unwrap_or(0);
		let face = self.faces.entry(format!("{part}#{index}")).or_insert_with(|| Face::load(package, &part, index));
		let font = face.as_ref().and_then(Face::font);
		let font = font.as_ref();
		RunStyle {
			width: run_width(font, run),
			bold: run.simulated_bold || font.is_some_and(|font| font.attributes().weight >= Weight::BOLD),
			monospaced: font.is_some_and(|font| font.metrics(Size::unscaled(), LocationRef::default()).is_monospace),
		}
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

	fn style_of(font: &[u8], part: &str, run: &Run) -> RunStyle {
		let mut package = Package::from_bytes(&[(part, font)]);
		Fonts::default().style_of(&mut package, "page.fpage", run)
	}

	fn width_of(run: &Run) -> Option<f64> {
		style_of(&test_font(400, false), "F.ttf", run).width
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
		assert_eq!(style_of(&font, &part, &run).width, Some(15.0));
	}

	#[test]
	fn a_font_is_found_relative_to_its_page() {
		let run = Run { font: "../Resources/F.ttf".to_string(), ..run("ab", "") };
		let mut package = Package::from_bytes(&[("Resources/F.ttf", &test_font(400, false))]);
		assert_eq!(Fonts::default().style_of(&mut package, "Pages/1.fpage", &run).width, Some(15.0));
	}

	#[test]
	fn a_font_collection_index_is_not_part_of_the_name() {
		let run = Run { font: "F.ttf#0".to_string(), ..run("ab", "") };
		assert_eq!(width_of(&run), Some(15.0));
	}

	#[test]
	fn a_missing_or_unreadable_font_leaves_the_width_unknown() {
		let run = Run { font: "Gone.ttf".to_string(), ..run("ab", "") };
		assert_eq!(style_of(&test_font(400, false), "F.ttf", &run).width, None);
		assert_eq!(style_of(b"not a font", "F.ttf", &run).width, None);
	}

	#[test]
	fn declared_advances_alone_measure_a_run_with_no_font() {
		let run = Run { font: "Gone.ttf".to_string(), ..run("ab", ",50;,100") };
		assert_eq!(style_of(&test_font(400, false), "F.ttf", &run).width, Some(15.0));
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
	fn a_font_is_read_once() {
		let mut package = Package::from_bytes(&[("F.ttf", &test_font(400, false))]);
		let mut fonts = Fonts::default();
		fonts.style_of(&mut package, "page.fpage", &run("a", ""));
		assert_eq!(fonts.faces.len(), 1);
		fonts.style_of(&mut package, "page.fpage", &run("b", ""));
		assert_eq!(fonts.faces.len(), 1);
	}
}
