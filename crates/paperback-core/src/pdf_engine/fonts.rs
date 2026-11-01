//! hayro's stand-ins for the 14 standard fonts draw only Latin, and a glyph with no outline gives OCR and the text layer nothing, so like pdfium this looks unembedded fonts up on the system first.

use std::{
	collections::HashMap,
	sync::{Arc, Mutex, OnceLock},
};

use fontdb::{Database, Family, ID, Query, Source, Stretch, Style, Weight};
use hayro_interpret::{
	InterpreterSettings,
	font::{FallbackFontQuery, FontData, FontQuery, StandardFont},
	hayro_cmap::CidFamily,
};

const SANS: &[&str] = &["Helvetica", "Arial", "Liberation Sans", "Nimbus Sans", "Arimo", "DejaVu Sans"];
const SERIF: &[&str] = &["Times New Roman", "Times", "Liberation Serif", "Nimbus Roman", "Tinos", "DejaVu Serif"];
const MONO: &[&str] = &["Courier New", "Courier", "Liberation Mono", "Nimbus Mono PS", "Cousine", "DejaVu Sans Mono"];
const JAPANESE: &[&str] = &[
	"MS Mincho",
	"MS Gothic",
	"Yu Mincho",
	"Yu Gothic",
	"Hiragino Mincho ProN",
	"Noto Serif CJK JP",
	"Noto Sans CJK JP",
];
const SIMPLIFIED_CHINESE: &[&str] =
	&["SimSun", "Microsoft YaHei", "Songti SC", "PingFang SC", "Noto Serif CJK SC", "Noto Sans CJK SC"];
const TRADITIONAL_CHINESE: &[&str] =
	&["MingLiU", "PMingLiU", "Microsoft JhengHei", "Songti TC", "PingFang TC", "Noto Serif CJK TC", "Noto Sans CJK TC"];
const KOREAN: &[&str] =
	&["Batang", "Malgun Gothic", "AppleMyungjo", "Apple SD Gothic Neo", "Noto Serif CJK KR", "Noto Sans CJK KR"];

pub(super) fn settings() -> InterpreterSettings {
	static SETTINGS: OnceLock<InterpreterSettings> = OnceLock::new();
	SETTINGS
		.get_or_init(|| InterpreterSettings { font_resolver: Arc::new(resolve), ..InterpreterSettings::default() })
		.clone()
}

fn resolve(query: &FontQuery) -> Option<(FontData, u32)> {
	match query {
		FontQuery::Standard(font) => installed_standard(*font).or_else(|| Some(font.get_font_data())),
		FontQuery::Fallback(query) => installed_fallback(query).or_else(|| {
			let font = query.pick_standard_font();
			installed_standard(font).or_else(|| Some(font.get_font_data()))
		}),
	}
}

/// Symbol and `ZapfDingbats` are left to hayro, whose stand-ins follow those fonts' own encodings.
fn installed_standard(font: StandardFont) -> Option<(FontData, u32)> {
	let (families, bold, italic) = match font {
		StandardFont::Helvetica => (SANS, false, false),
		StandardFont::HelveticaBold => (SANS, true, false),
		StandardFont::HelveticaOblique => (SANS, false, true),
		StandardFont::HelveticaBoldOblique => (SANS, true, true),
		StandardFont::TimesRoman => (SERIF, false, false),
		StandardFont::TimesBold => (SERIF, true, false),
		StandardFont::TimesItalic => (SERIF, false, true),
		StandardFont::TimesBoldItalic => (SERIF, true, true),
		StandardFont::Courier => (MONO, false, false),
		StandardFont::CourierBold => (MONO, true, false),
		StandardFont::CourierOblique => (MONO, false, true),
		StandardFont::CourierBoldOblique => (MONO, true, true),
		StandardFont::Symbol | StandardFont::ZapfDingBats => return None,
	};
	find(families, if bold { Weight::BOLD } else { Weight::NORMAL }, italic)
}

fn installed_fallback(query: &FallbackFontQuery) -> Option<(FontData, u32)> {
	if let Some(name) = &query.post_script_name
		&& let Some(&id) = post_script_names().get(name)
	{
		return load(id);
	}
	let weight = Weight(u16::try_from(query.font_weight).unwrap_or(400));
	let named: Vec<&str> = [&query.font_family, &query.font_name].into_iter().flatten().map(String::as_str).collect();
	if let Some(found) = find(&named, weight, query.is_italic) {
		return Some(found);
	}
	let families = match query.character_collection.as_ref().map(|collection| &collection.family) {
		Some(CidFamily::AdobeJapan1) => JAPANESE,
		Some(CidFamily::AdobeGB1) => SIMPLIFIED_CHINESE,
		Some(CidFamily::AdobeCNS1) => TRADITIONAL_CHINESE,
		Some(CidFamily::AdobeKorea1) => KOREAN,
		_ => return None,
	};
	find(families, weight, query.is_italic)
}

fn find(families: &[&str], weight: Weight, italic: bool) -> Option<(FontData, u32)> {
	if families.is_empty() {
		return None;
	}
	let families: Vec<Family<'_>> = families.iter().map(|name| Family::Name(name)).collect();
	let style = if italic { Style::Italic } else { Style::Normal };
	let id = database().query(&Query { families: &families, weight, stretch: Stretch::Normal, style })?;
	load(id)
}

/// Scanned the first time a PDF needs a font rather than at startup, since most documents embed everything they use.
fn database() -> &'static Database {
	static DATABASE: OnceLock<Database> = OnceLock::new();
	DATABASE.get_or_init(|| {
		let mut database = Database::new();
		database.load_system_fonts();
		// fontdb looks nowhere on Android, and on iOS only where Linux keeps its fonts, so those systems' own font folders are named here.
		#[cfg(target_os = "android")]
		database.load_fonts_dir("/system/fonts");
		#[cfg(target_os = "ios")]
		database.load_fonts_dir("/System/Library/Fonts");
		database
	})
}

fn post_script_names() -> &'static HashMap<String, ID> {
	static NAMES: OnceLock<HashMap<String, ID>> = OnceLock::new();
	NAMES.get_or_init(|| database().faces().map(|face| (face.post_script_name.clone(), face.id)).collect())
}

fn load(id: ID) -> Option<(FontData, u32)> {
	static LOADED: OnceLock<Mutex<HashMap<ID, (FontData, u32)>>> = OnceLock::new();
	let loaded = LOADED.get_or_init(|| Mutex::new(HashMap::new()));
	if let Some(found) = loaded.lock().ok()?.get(&id) {
		return Some(found.clone());
	}
	let (source, index) = database().face_source(id)?;
	let data: FontData = match source {
		Source::Binary(data) => data,
		Source::File(path) => Arc::new(std::fs::read(path).ok()?),
	};
	loaded.lock().ok()?.insert(id, (data.clone(), index));
	Some((data, index))
}
