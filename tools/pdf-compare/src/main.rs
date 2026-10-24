//! Lines up what pdfium and hayro each find on every page of a folder of PDFs.
//!
//! `cargo run --release -- <folder>` with pdfium in `PAPERBACK_PDFIUM_DIR` (the workspace's `target/release` by default).
//!
//! Paperback reads untagged text from pdfium's character stream. A hayro backend would build that stream from hayro's glyph runs instead, so before anything is ported the question is how close the raw material already is: the same characters, at the same places, in the same fonts. Whitespace is left out on both sides: pdfium generates spaces and line breaks of its own and flags some of the content stream's real spaces as generated too, and deciding where spaces go is the port's job rather than hayro's.

use std::{
	env,
	ffi::{c_int, c_ulong, c_void},
	fs,
	panic::{AssertUnwindSafe, catch_unwind},
	path::{Path, PathBuf},
	time::Instant,
};

use hayro_interpret::{
	BlendMode, ClipPath, Context, Device, DrawMode, DrawProps, Image, ImageDrawProps, InterpreterCache,
	InterpreterSettings, SoftMask,
	font::GlyphRun,
	hayro_cmap::BfString,
	interpret_page,
	kurbo::{Affine, BezPath, Point, Rect},
};
use hayro_syntax::Pdf;
use pdfium_render::prelude::{Pdfium, PdfiumLibraryBindings};
use similar::{Algorithm, DiffOp, capture_diff_slices};

/// How far apart, in PDF points, two readings of one glyph's origin may be and still count as the same place.
const SAME_PLACE: f64 = 0.5;

struct Glyph {
	ch: char,
	x: f64,
	y: f64,
	font: String,
	flags: Option<u32>,
}

#[derive(Default)]
struct Tally {
	pdfium: usize,
	hayro: usize,
	matched: usize,
	same_place: usize,
	fonts_compared: usize,
	same_font: usize,
	flags_compared: usize,
	same_flags: usize,
	pdfium_ms: u128,
	hayro_ms: u128,
}

impl Tally {
	fn add(&mut self, other: &Self) {
		self.pdfium += other.pdfium;
		self.hayro += other.hayro;
		self.matched += other.matched;
		self.same_place += other.same_place;
		self.fonts_compared += other.fonts_compared;
		self.same_font += other.same_font;
		self.flags_compared += other.flags_compared;
		self.same_flags += other.same_flags;
		self.pdfium_ms += other.pdfium_ms;
		self.hayro_ms += other.hayro_ms;
	}

	fn line(&self) -> String {
		format!(
			"chars {:>7} / {:>7}  matched {:>6}  place {:>6}  font {:>6}  flags {:>6}  ms {:>6} / {:>6}",
			self.pdfium,
			self.hayro,
			percent(self.matched, self.pdfium.max(self.hayro)),
			percent(self.same_place, self.matched),
			percent(self.same_font, self.fonts_compared),
			percent(self.same_flags, self.flags_compared),
			self.pdfium_ms,
			self.hayro_ms
		)
	}
}

fn percent(part: usize, whole: usize) -> String {
	if whole == 0 { "-".to_string() } else { format!("{:.1}%", part as f64 * 100.0 / whole as f64) }
}

fn main() {
	let args: Vec<String> = env::args().skip(1).collect();
	let samples = args.iter().any(|arg| arg == "--samples");
	let Some(folder) = args.iter().find(|arg| !arg.starts_with("--")).map(PathBuf::from) else {
		eprintln!("usage: pdf-compare <folder> [--samples]");
		return;
	};
	let library_dir = env::var("PAPERBACK_PDFIUM_DIR")
		.map_or_else(|_| Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/release"), PathBuf::from);
	let bindings = Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path(&library_dir))
		.expect("pdfium should load from PAPERBACK_PDFIUM_DIR");
	unsafe { bindings.FPDF_InitLibrary() };
	let mut pdfs = Vec::new();
	collect_pdfs(&folder, &mut pdfs);
	pdfs.sort();
	let mut total = Tally::default();
	let mut worst = Vec::new();
	let mut failures = Vec::new();
	for pdf in &pdfs {
		let name = pdf.strip_prefix(&folder).unwrap_or(pdf).display().to_string();
		match compare(bindings.as_ref(), pdf, samples) {
			Ok(tally) => {
				println!("{}  {name}", tally.line());
				let unmatched = tally.pdfium.max(tally.hayro) - tally.matched;
				worst.push((unmatched, name));
				total.add(&tally);
			}
			Err(reason) => {
				println!("{reason:<40}  {name}");
				failures.push((reason, name));
			}
		}
	}
	println!("\n{} pdfs, {} could not be compared", pdfs.len(), failures.len());
	println!("{}  total", total.line());
	worst.sort_by_key(|(unmatched, _)| std::cmp::Reverse(*unmatched));
	println!("\nmost unmatched characters:");
	for (unmatched, name) in worst.iter().take(20) {
		println!("{unmatched:>8}  {name}");
	}
	println!("\ncould not be compared:");
	for (reason, name) in &failures {
		println!("{reason:<40}  {name}");
	}
}

fn collect_pdfs(folder: &Path, out: &mut Vec<PathBuf>) {
	let Ok(entries) = fs::read_dir(folder) else { return };
	for entry in entries.flatten() {
		let path = entry.path();
		if path.is_dir() {
			collect_pdfs(&path, out);
		} else if path.extension().is_some_and(|extension| extension.eq_ignore_ascii_case("pdf")) {
			out.push(path);
		}
	}
}

fn password(pdf: &Path) -> Option<String> {
	let mut file = pdf.as_os_str().to_owned();
	file.push(".password");
	fs::read_to_string(file).ok().map(|password| password.trim_end_matches(['\r', '\n']).to_string())
}

fn compare(bindings: &dyn PdfiumLibraryBindings, pdf: &Path, samples: bool) -> Result<Tally, String> {
	let password = password(pdf);
	let started = Instant::now();
	let pdfium_pages = pdfium_glyphs(bindings, pdf, password.as_deref())?;
	let pdfium_ms = started.elapsed().as_millis();
	let started = Instant::now();
	let data = fs::read(pdf).map_err(|error| error.to_string())?;
	let hayro_pages = catch_unwind(AssertUnwindSafe(|| hayro_glyphs(data, password.as_deref())))
		.map_err(|_| "hayro panicked".to_string())??;
	let hayro_ms = started.elapsed().as_millis();
	if pdfium_pages.len() != hayro_pages.len() {
		return Err(format!("pages: pdfium {} hayro {}", pdfium_pages.len(), hayro_pages.len()));
	}
	let mut tally = Tally { pdfium_ms, hayro_ms, ..Tally::default() };
	let mut shown = if samples { 3 } else { 0 };
	for (pdfium, hayro) in pdfium_pages.iter().zip(&hayro_pages) {
		compare_page(pdfium, hayro, &mut tally, &mut shown);
	}
	Ok(tally)
}

fn compare_page(pdfium: &[Glyph], hayro: &[Glyph], tally: &mut Tally, shown: &mut usize) {
	tally.pdfium += pdfium.len();
	tally.hayro += hayro.len();
	let a: Vec<char> = pdfium.iter().map(|glyph| glyph.ch).collect();
	let b: Vec<char> = hayro.iter().map(|glyph| glyph.ch).collect();
	for op in capture_diff_slices(Algorithm::Myers, &a, &b) {
		if *shown > 0 && !matches!(op, DiffOp::Equal { .. }) {
			*shown -= 1;
			let (only_pdfium, only_hayro) = (op.old_range(), op.new_range());
			let snippet = |chars: &[char], range: std::ops::Range<usize>| {
				chars[range.start..range.end.min(range.start + 80)].iter().collect::<String>()
			};
			println!("    pdfium only {:?}  hayro only {:?}", snippet(&a, only_pdfium), snippet(&b, only_hayro));
		}
		let DiffOp::Equal { old_index, new_index, len } = op else { continue };
		tally.matched += len;
		for offset in 0..len {
			let (p, h) = (&pdfium[old_index + offset], &hayro[new_index + offset]);
			if *shown > 0
				&& ((p.x - h.x).hypot(p.y - h.y) > SAME_PLACE
					|| p.font != h.font
					|| (p.flags.is_some() && h.flags.is_some() && p.flags != h.flags))
			{
				*shown -= 1;
				println!(
					"    {:?} pdfium ({:.2}, {:.2}) {:?} {:?}  hayro ({:.2}, {:.2}) {:?} {:?}",
					p.ch, p.x, p.y, p.font, p.flags, h.x, h.y, h.font, h.flags
				);
			}
			if (p.x - h.x).hypot(p.y - h.y) <= SAME_PLACE {
				tally.same_place += 1;
			}
			if !p.font.is_empty() && !h.font.is_empty() {
				tally.fonts_compared += 1;
				if p.font == h.font {
					tally.same_font += 1;
				}
			}
			// pdfium makes flags up for a font with no descriptor, which hayro rightly reports as absent, so only the fonts that state them are compared.
			if let (Some(pf), Some(hf)) = (p.flags, h.flags) {
				tally.flags_compared += 1;
				if pf == hf {
					tally.same_flags += 1;
				}
			}
		}
	}
}

fn pdfium_glyphs(
	bindings: &dyn PdfiumLibraryBindings,
	pdf: &Path,
	password: Option<&str>,
) -> Result<Vec<Vec<Glyph>>, String> {
	let document = unsafe { bindings.FPDF_LoadDocument(&pdf.to_string_lossy(), password) };
	if document.is_null() {
		return Err(format!("pdfium could not open it ({})", unsafe { bindings.FPDF_GetLastError() }));
	}
	let mut pages = Vec::new();
	for index in 0..unsafe { bindings.FPDF_GetPageCount(document) } {
		let page = unsafe { bindings.FPDF_LoadPage(document, index) };
		let text = if page.is_null() { std::ptr::null_mut() } else { unsafe { bindings.FPDFText_LoadPage(page) } };
		let mut glyphs = Vec::new();
		if !text.is_null() {
			for i in 0..unsafe { bindings.FPDFText_CountChars(text) } {
				if unsafe { bindings.FPDFText_IsGenerated(text, i) } == 1 {
					continue;
				}
				// pdfium hands back a hyphen it thinks splits a word as 0x02.
				let ch = match unsafe { bindings.FPDFText_GetUnicode(text, i) } {
					0x02 => '-',
					code => char::from_u32(code).unwrap_or(char::REPLACEMENT_CHARACTER),
				};
				if ch.is_whitespace() {
					continue;
				}
				let (mut x, mut y) = (0.0, 0.0);
				unsafe { bindings.FPDFText_GetCharOrigin(text, i, &mut x, &mut y) };
				let mut name = [0u8; 256];
				let mut flags: c_int = 0;
				let written = unsafe {
					bindings.FPDFText_GetFontInfo(
						text,
						i,
						name.as_mut_ptr().cast::<c_void>(),
						name.len() as c_ulong,
						&mut flags,
					)
				} as usize;
				let font = if written > 1 && written <= name.len() {
					String::from_utf8_lossy(&name[..written - 1]).into_owned()
				} else {
					String::new()
				};
				// The bit pdfium sets at 1 << 19 is its own bookkeeping, not one the PDF states.
				glyphs.push(Glyph {
					ch,
					x,
					y,
					font: without_subset_tag(&font).to_string(),
					flags: Some(flags as u32 & !(1 << 19)),
				});
			}
			unsafe { bindings.FPDFText_ClosePage(text) };
		}
		if !page.is_null() {
			unsafe { bindings.FPDF_ClosePage(page) };
		}
		pages.push(glyphs);
	}
	unsafe { bindings.FPDF_CloseDocument(document) };
	Ok(pages)
}

fn hayro_glyphs(data: Vec<u8>, password: Option<&str>) -> Result<Vec<Vec<Glyph>>, String> {
	let pdf = Pdf::new_with_password(data, password.unwrap_or(""))
		.map_err(|error| format!("hayro could not open it ({error:?})"))?;
	let cache = InterpreterCache::new();
	let mut pages = Vec::new();
	for page in pdf.pages().iter() {
		// The context's bounds only decide what gets drawn, and text off the page is still text, so they are made too large to cull anything.
		let mut context = Context::new(
			Affine::IDENTITY,
			Rect::new(-1e7, -1e7, 1e7, 1e7),
			&cache,
			pdf.xref(),
			InterpreterSettings::default(),
		);
		let mut collector = Collector::default();
		interpret_page(page, &mut context, &mut collector);
		pages.push(collector.glyphs);
	}
	Ok(pages)
}

type RunIdentity = (usize, usize, Option<[f64; 6]>);

#[derive(Default)]
struct Collector {
	glyphs: Vec<Glyph>,
	last_run: Option<RunIdentity>,
}

impl<'a> Device<'a> for Collector {
	fn draw_path(&mut self, _: &BezPath, _: DrawProps<'a>, _: &DrawMode) {}
	fn push_clip_path(&mut self, _: &ClipPath) {}
	fn push_transparency_group(&mut self, _: f32, _: Option<SoftMask<'a>>, _: BlendMode) {}
	fn draw_glyph_run(&mut self, run: &GlyphRun<'_, 'a>, props: DrawProps<'a>, draw_mode: &DrawMode) {
		// A fill-and-stroke run arrives twice in a row, once per paint, from the same buffer, and its glyphs are only on the page once.
		let identity = (
			run.glyphs().as_ptr() as usize,
			run.glyphs().len(),
			run.glyphs().first().map(|glyph| glyph.transform().as_coeffs()),
		);
		let repeated = matches!(draw_mode, DrawMode::Stroke(_)) && self.last_run == Some(identity);
		self.last_run = Some(identity);
		if repeated {
			return;
		}
		// pdfium reports the name of the font program, which only sometimes keeps the six-letter tag of a subset, so both sides compare without it.
		let font = run
			.font()
			.and_then(|font| font.base_font())
			.map(|name| without_subset_tag(name.as_str()).to_string())
			.unwrap_or_default();
		let flags = run.font().and_then(|font| font.flags()).map(|flags| flags.bits());
		for glyph in run.glyphs() {
			let origin = props.transform * glyph.transform() * Point::ZERO;
			let text = match glyph.as_unicode() {
				Some(BfString::Char(ch)) => decompose(ch),
				Some(BfString::String(text)) => text.chars().flat_map(decompose).collect(),
				None => vec![char::REPLACEMENT_CHARACTER],
			};
			for ch in text.into_iter().filter(|ch| !ch.is_whitespace()) {
				self.glyphs.push(Glyph { ch, x: origin.x, y: origin.y, font: font.clone(), flags });
			}
		}
	}
	fn draw_image(&mut self, _: Image<'a, '_>, _: ImageDrawProps<'a>) {}
	fn pop_clip(&mut self) {}
	fn pop_transparency_group(&mut self) {}
}

fn without_subset_tag(name: &str) -> &str {
	match name.split_once('+') {
		Some((tag, rest)) if tag.len() == 6 && tag.bytes().all(|byte| byte.is_ascii_uppercase()) => rest,
		_ => name,
	}
}

// pdfium splits the Latin ligatures back into their letters, so they are split here too to compare like with like.
fn decompose(ch: char) -> Vec<char> {
	match ch {
		'\u{FB00}' => vec!['f', 'f'],
		'\u{FB01}' => vec!['f', 'i'],
		'\u{FB02}' => vec!['f', 'l'],
		'\u{FB03}' => vec!['f', 'f', 'i'],
		'\u{FB04}' => vec!['f', 'f', 'l'],
		'\u{FB05}' | '\u{FB06}' => vec!['s', 't'],
		_ => vec![ch],
	}
}
