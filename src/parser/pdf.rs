use std::{
	collections::{HashMap, HashSet},
	fs, str,
	sync::Arc,
};

use anyhow::{Result, anyhow};
use hayro_interpret::{
	BlendMode, ClipPath, Context, Device, GlyphDrawMode, Image, InterpreterSettings, PageExt, Paint, PathDrawMode,
	RectExt, SoftMask, font::Glyph, interpret_page,
};
use hayro_syntax::{
	LoadPdfError, Pdf,
	object::{
		Array, Dict, MaybeRef, Name, Object, ObjectIdentifier, String as PdfObjectString,
		dict::keys::{A, D, DEST, DESTS, FIRST, KIDS, NAMES, NEXT, OUTLINES, S, TITLE},
	},
};
use kurbo::{Affine, BezPath, Point};
use wxdragon::translations::translate as t;

use crate::{
	document::{Document, DocumentBuffer, Marker, MarkerType, ParserContext, ParserFlags, TocItem},
	parser::{PASSWORD_REQUIRED_ERROR_PREFIX, Parser, path::extract_title_from_path},
	text::{collapse_whitespace, trim_string},
};

pub struct PdfParser;

impl Parser for PdfParser {
	fn name(&self) -> &'static str {
		"PDF Documents"
	}

	fn extensions(&self) -> &[&str] {
		&["pdf"]
	}

	fn supported_flags(&self) -> ParserFlags {
		ParserFlags::SUPPORTS_PAGES
	}

	fn parse(&self, context: &ParserContext) -> Result<Document> {
		let data =
			fs::read(&context.file_path).map_err(|err| anyhow!("Failed to read PDF {}: {err}", context.file_path))?;
		let password = context.password.as_deref().unwrap_or_default();
		let document = Pdf::new_with_password(Arc::new(data), password).map_err(map_load_error)?;
		let mut buffer = DocumentBuffer::new();
		let mut page_offsets = Vec::new();
		let mut has_any_text = false;
		let mut has_any_images = false;
		for (page_index, page) in document.pages().iter().enumerate() {
			let marker_position = buffer.current_position();
			page_offsets.push(marker_position);
			buffer.add_marker(
				Marker::new(MarkerType::PageBreak, marker_position).with_text(format!("Page {}", page_index + 1)),
			);
			let result = extract_page_text(&document, page);
			has_any_images |= result.has_images;
			let lines = process_text_lines(&result.text);
			if !lines.is_empty() {
				has_any_text = true;
			}
			for line in lines {
				buffer.append(&line);
				buffer.append("\n");
			}
		}
		if !has_any_text && has_any_images {
			let marker_position = buffer.current_position();
			buffer.add_marker(Marker::new(MarkerType::PageBreak, marker_position).with_text(String::new()));
			buffer.append(&t("This PDF contains images only, with no extractable text. You may need to run it through OCR software to read its contents."));
			buffer.append("\n");
		}
		let metadata = document.metadata();
		let title =
			metadata.title.as_deref().map_or_else(|| extract_title_from_path(&context.file_path), decode_pdf_string);
		let author = metadata.author.as_deref().map(decode_pdf_string).unwrap_or_default();
		let toc_items = extract_pdf_toc(&document, &page_offsets);
		let mut doc = Document::new();
		doc.set_buffer(buffer);
		doc.title = title;
		doc.author = author;
		doc.toc_items = toc_items;
		Ok(doc)
	}
}

fn process_text_lines(raw_text: &str) -> Vec<String> {
	raw_text
		.lines()
		.filter_map(|line| {
			let collapsed = collapse_whitespace(line);
			let trimmed = trim_string(&collapsed);
			if trimmed.is_empty() { None } else { Some(trimmed) }
		})
		.collect()
}

fn map_load_error(err: LoadPdfError) -> anyhow::Error {
	match err {
		LoadPdfError::Decryption(_) => {
			anyhow!("{PASSWORD_REQUIRED_ERROR_PREFIX}Password required or incorrect")
		}
		LoadPdfError::Invalid => anyhow!("Failed to open PDF document"),
	}
}

struct PageResult {
	text: String,
	has_images: bool,
}

fn extract_page_text(document: &Pdf, page: &hayro_syntax::page::Page<'_>) -> PageResult {
	let settings = InterpreterSettings::default();
	let bbox = page.intersected_crop_box().to_kurbo();
	let initial_transform = page.initial_transform(true);
	let mut context = Context::new(initial_transform, bbox, document.xref(), settings);
	let mut extractor = TextExtractor::default();
	interpret_page(page, &mut context, &mut extractor);
	let text = assemble_glyphs_to_text(&extractor.glyphs);
	PageResult { text, has_images: extractor.has_images }
}

struct GlyphEntry {
	x: f64,
	y: f64,
	ch: char,
}

#[derive(Default)]
struct TextExtractor {
	glyphs: Vec<GlyphEntry>,
	has_images: bool,
}

impl Device<'_> for TextExtractor {
	fn set_soft_mask(&mut self, _: Option<SoftMask<'_>>) {}

	fn set_blend_mode(&mut self, _: BlendMode) {}

	fn draw_path(&mut self, _: &BezPath, _: Affine, _: &Paint<'_>, _: &PathDrawMode) {}

	fn push_clip_path(&mut self, _: &ClipPath) {}

	fn push_transparency_group(&mut self, _: f32, _: Option<SoftMask<'_>>, _: BlendMode) {}

	fn draw_glyph(
		&mut self,
		glyph: &Glyph<'_>,
		transform: Affine,
		glyph_transform: Affine,
		_: &Paint<'_>,
		_: &GlyphDrawMode,
	) {
		let Some(ch) = glyph.as_unicode() else { return };
		if ch == '\0' {
			return;
		}
		let position = transform * glyph_transform * Point::new(0.0, 0.0);
		self.glyphs.push(GlyphEntry { x: position.x, y: position.y, ch });
	}

	fn draw_image(&mut self, _: Image<'_, '_>, _: Affine) {
		self.has_images = true;
	}

	fn pop_clip_path(&mut self) {}

	fn pop_transparency_group(&mut self) {}
}

/// Reorders PDF text into visual reading order by splitting the draw-order glyph
/// sequence at large vertical jumps and sorting those sections by their topmost
/// y-coordinate, then applying the original sequential space/line-break heuristics
/// within each section.
///
/// PDFs may draw sections of a page out of reading order (e.g. the bottom half of a
/// form before the top half). Sorting every individual glyph by position fixes that
/// but destroys word/line order within compact table headers and multi-line form
/// fields. This approach is more conservative: only the coarse section order changes;
/// the original draw order is preserved within each section.
fn assemble_glyphs_to_text(glyphs: &[GlyphEntry]) -> String {
	// A y-jump larger than this between consecutive glyphs (in draw order) indicates
	// the renderer has moved to a different region of the page, i.e. sections are
	// drawn out of top-to-bottom order.
	const SECTION_GAP: f64 = 200.0;
	// Sentinel used instead of a plain ' ' for heuristically-detected spaces so that
	// remove_cap_advance_spaces() can later distinguish them from real PDF spaces.
	const DETECTED_SPACE: char = '\u{E000}';

	if glyphs.is_empty() {
		return String::new();
	}

	// Split the draw-order sequence into sections at large y-jumps.
	let mut sections: Vec<&[GlyphEntry]> = Vec::new();
	let mut start = 0;
	for i in 1..glyphs.len() {
		if (glyphs[i].y - glyphs[i - 1].y).abs() > SECTION_GAP {
			sections.push(&glyphs[start..i]);
			start = i;
		}
	}
	sections.push(&glyphs[start..]);

	// Sort sections by their topmost (minimum) y so that sections higher on the page
	// are read first. Use a stable sort to keep sections at equal y in draw order.
	sections.sort_by(|a, b| {
		let min_y = |s: &&[GlyphEntry]| s.iter().map(|g| g.y).fold(f64::INFINITY, f64::min);
		min_y(a).partial_cmp(&min_y(b)).unwrap_or(std::cmp::Ordering::Equal)
	});

	// Assemble text using the same sequential space/newline heuristics as the
	// original single-pass extractor, applied within each section independently.
	let mut text = String::new();
	for section in &sections {
		if !text.is_empty() && !text.ends_with('\n') {
			text.push('\n');
		}
		let mut last_pos: Option<(f64, f64)> = None;
		let mut avg_dx: Option<f64> = None;
		for glyph in *section {
			if let Some((last_x, last_y)) = last_pos {
				let dy = (glyph.y - last_y).abs();
				let dx = glyph.x - last_x;
				if dy > 7.0 {
					text.push('\n');
					avg_dx = None;
				} else if dx > 0.0 {
					let avg = avg_dx.unwrap_or(dx);
					let last_char = text.chars().last();
					let alnum_pair = last_char.is_some_and(char::is_alphanumeric) && glyph.ch.is_alphanumeric();
					let gap_threshold = if alnum_pair { (avg * 2.4).max(4.0) } else { (avg * 1.6).max(3.0) };
					if dx > gap_threshold && !text.ends_with([' ', '\n', '\r', '\t', DETECTED_SPACE]) && glyph.ch != ' '
					{
						text.push(DETECTED_SPACE);
					}
					if dx < avg * 2.8 {
						avg_dx = Some(avg.mul_add(0.8, dx * 0.2));
					}
				}
			}
			text.push(glyph.ch);
			last_pos = Some((glyph.x, glyph.y));
		}
	}
	remove_cap_advance_spaces(&text, DETECTED_SPACE)
}

/// Remove spurious heuristic spaces inserted between a word-initial uppercase letter
/// and the lowercase continuation of the same word.
///
/// Some PDFs encode capital letters in a different font with a wider advance width
/// than lowercase, causing the gap-based space detector to fire mid-word, e.g.
/// "M iddle" instead of "Middle". We use a sentinel for detected spaces so that real
/// PDF space characters (' ') are never touched.
fn remove_cap_advance_spaces(text: &str, sentinel: char) -> String {
	let chars: Vec<char> = text.chars().collect();
	let n = chars.len();
	let mut result = String::with_capacity(text.len());
	let mut i = 0;
	while i < n {
		let ch = chars[i];
		if ch == sentinel {
			let prev = if i > 0 { chars[i - 1] } else { '\n' };
			let next = if i + 1 < n { chars[i + 1] } else { '\n' };
			// Remove the space only when:
			//  - the preceding character is an uppercase letter (the split capital), AND
			//  - the following character is a lowercase letter (the word continuation), AND
			//  - the uppercase letter is at a word boundary (preceded by whitespace or
			//    start-of-text), so standalone capitals like "I" are not accidentally joined
			//    to the next word.
			let at_word_boundary = i < 2 || chars[i - 2].is_whitespace();
			if prev.is_uppercase() && next.is_lowercase() && at_word_boundary {
				i += 1;
				continue; // drop this sentinel
			}
			result.push(' '); // keep as regular space
		} else {
			result.push(ch);
		}
		i += 1;
	}
	result
}

fn decode_pdf_string(bytes: &[u8]) -> String {
	if bytes.starts_with(&[0xFE, 0xFF]) {
		return decode_utf16(bytes.get(2..).unwrap_or_default(), true);
	}
	if bytes.starts_with(&[0xFF, 0xFE]) {
		return decode_utf16(bytes.get(2..).unwrap_or_default(), false);
	}
	if let Ok(text) = str::from_utf8(bytes) {
		return text.to_string();
	}
	bytes.iter().map(|byte| char::from(*byte)).collect()
}

fn decode_utf16(bytes: &[u8], big_endian: bool) -> String {
	let mut units = Vec::with_capacity(bytes.len() / 2);
	for chunk in bytes.chunks_exact(2) {
		let value = if big_endian {
			u16::from_be_bytes([chunk[0], chunk[1]])
		} else {
			u16::from_le_bytes([chunk[0], chunk[1]])
		};
		units.push(value);
	}
	String::from_utf16_lossy(&units)
}

#[derive(Debug, Clone)]
struct OutlineEntry {
	level: i32,
	title: String,
	offset: usize,
}

fn extract_pdf_toc(document: &Pdf, page_offsets: &[usize]) -> Vec<TocItem> {
	let page_by_obj_id = build_page_object_index(document);
	let Some(catalog) = document.xref().get::<Dict<'_>>(document.xref().root_id()) else {
		return Vec::new();
	};
	let Some(outlines) = catalog.get::<Dict<'_>>(OUTLINES) else {
		return Vec::new();
	};
	let Some(first) = outlines.get::<Dict<'_>>(FIRST) else {
		return Vec::new();
	};
	let mut entries = Vec::new();
	let mut visited_items = HashSet::new();
	let mut visited_name_tree = HashSet::new();
	collect_outline_entries(
		&catalog,
		&page_by_obj_id,
		page_offsets,
		&first,
		1,
		&mut visited_items,
		&mut visited_name_tree,
		&mut entries,
	);
	build_toc_tree(&entries)
}

fn build_page_object_index(document: &Pdf) -> HashMap<ObjectIdentifier, usize> {
	document.pages().iter().enumerate().filter_map(|(idx, page)| page.raw().obj_id().map(|id| (id, idx))).collect()
}

fn collect_outline_entries(
	catalog: &Dict<'_>,
	page_by_obj_id: &HashMap<ObjectIdentifier, usize>,
	page_offsets: &[usize],
	start_item: &Dict<'_>,
	level: i32,
	visited_items: &mut HashSet<ObjectIdentifier>,
	visited_name_tree: &mut HashSet<ObjectIdentifier>,
	out: &mut Vec<OutlineEntry>,
) {
	let mut current = Some(start_item.clone());
	let mut sibling_count = 0usize;
	while let Some(item) = current {
		sibling_count += 1;
		if sibling_count > 10_000 {
			break;
		}
		if let Some(item_id) = item.obj_id()
			&& !visited_items.insert(item_id)
		{
			break;
		}
		let title =
			item.get::<PdfObjectString<'_>>(TITLE).map(|s| decode_pdf_string(s.get().as_ref())).unwrap_or_default();
		if !title.is_empty()
			&& let Some(page_index) = resolve_outline_page_index(catalog, page_by_obj_id, &item, visited_name_tree)
			&& let Some(&offset) = page_offsets.get(page_index)
		{
			out.push(OutlineEntry { level, title, offset });
		}
		if let Some(first_child) = item.get::<Dict<'_>>(FIRST) {
			collect_outline_entries(
				catalog,
				page_by_obj_id,
				page_offsets,
				&first_child,
				level + 1,
				visited_items,
				visited_name_tree,
				out,
			);
		}
		current = item.get::<Dict<'_>>(NEXT);
	}
}

fn resolve_outline_page_index(
	catalog: &Dict<'_>,
	page_by_obj_id: &HashMap<ObjectIdentifier, usize>,
	item: &Dict<'_>,
	visited_name_tree: &mut HashSet<ObjectIdentifier>,
) -> Option<usize> {
	if let Some(dest_obj) = item.get::<Object<'_>>(DEST)
		&& let Some(page) = resolve_destination_object(catalog, page_by_obj_id, dest_obj, visited_name_tree)
	{
		return Some(page);
	}
	let action = item.get::<Dict<'_>>(A)?;
	let action_kind = action.get::<Name<'_>>(S)?;
	if &*action_kind != b"GoTo" {
		return None;
	}
	let dest_obj = action.get::<Object<'_>>(D)?;
	resolve_destination_object(catalog, page_by_obj_id, dest_obj, visited_name_tree)
}

fn resolve_destination_object(
	catalog: &Dict<'_>,
	page_by_obj_id: &HashMap<ObjectIdentifier, usize>,
	dest_obj: Object<'_>,
	visited_name_tree: &mut HashSet<ObjectIdentifier>,
) -> Option<usize> {
	match dest_obj {
		Object::Array(arr) => resolve_page_from_destination_array(page_by_obj_id, &arr),
		Object::Name(name) => resolve_named_destination(catalog, page_by_obj_id, name.as_ref(), visited_name_tree),
		Object::String(name) => {
			resolve_named_destination(catalog, page_by_obj_id, name.get().as_ref(), visited_name_tree)
		}
		Object::Dict(dict) => dict
			.get::<Object<'_>>(D)
			.and_then(|inner| resolve_destination_object(catalog, page_by_obj_id, inner, visited_name_tree)),
		_ => None,
	}
}

fn resolve_page_from_destination_array(
	page_by_obj_id: &HashMap<ObjectIdentifier, usize>,
	dest_array: &Array<'_>,
) -> Option<usize> {
	let first = dest_array.raw_iter().next()?;
	match first {
		MaybeRef::Ref(obj_ref) => page_by_obj_id.get(&obj_ref.into()).copied(),
		MaybeRef::NotRef(Object::Dict(dict)) => dict.obj_id().and_then(|id| page_by_obj_id.get(&id).copied()),
		_ => None,
	}
}

fn resolve_named_destination(
	catalog: &Dict<'_>,
	page_by_obj_id: &HashMap<ObjectIdentifier, usize>,
	name: &[u8],
	visited_name_tree: &mut HashSet<ObjectIdentifier>,
) -> Option<usize> {
	if let Some(dests) = catalog.get::<Dict<'_>>(DESTS)
		&& let Some(dest_obj) = dests.get::<Object<'_>>(name)
	{
		return resolve_destination_object(catalog, page_by_obj_id, dest_obj, visited_name_tree);
	}
	let names_root = catalog.get::<Dict<'_>>(NAMES)?.get::<Dict<'_>>(DESTS)?;
	resolve_named_destination_in_tree(catalog, page_by_obj_id, &names_root, name, visited_name_tree)
}

fn resolve_named_destination_in_tree(
	catalog: &Dict<'_>,
	page_by_obj_id: &HashMap<ObjectIdentifier, usize>,
	node: &Dict<'_>,
	target_name: &[u8],
	visited_name_tree: &mut HashSet<ObjectIdentifier>,
) -> Option<usize> {
	if let Some(node_id) = node.obj_id()
		&& !visited_name_tree.insert(node_id)
	{
		return None;
	}
	if let Some(names) = node.get::<Array<'_>>(NAMES) {
		let mut iter = names.flex_iter();
		loop {
			let Some(name_obj) = iter.next::<Object<'_>>() else {
				break;
			};
			let Some(value_obj) = iter.next::<Object<'_>>() else {
				break;
			};
			let key = match name_obj {
				Object::String(s) => s.get().to_vec(),
				Object::Name(n) => n.as_ref().to_vec(),
				_ => continue,
			};
			if key.as_slice() == target_name {
				return resolve_destination_object(catalog, page_by_obj_id, value_obj, visited_name_tree);
			}
		}
	}
	if let Some(kids) = node.get::<Array<'_>>(KIDS) {
		for child in kids.iter::<Dict<'_>>() {
			if let Some(found) =
				resolve_named_destination_in_tree(catalog, page_by_obj_id, &child, target_name, visited_name_tree)
			{
				return Some(found);
			}
		}
	}
	None
}

fn build_toc_tree(entries: &[OutlineEntry]) -> Vec<TocItem> {
	let mut toc = Vec::new();
	let mut stack: Vec<usize> = Vec::new();
	let mut levels: Vec<i32> = Vec::new();
	for entry in entries {
		while let Some(&last_level) = levels.last() {
			if last_level < entry.level {
				break;
			}
			stack.pop();
			levels.pop();
		}
		let siblings = children_at_mut(&mut toc, &stack);
		siblings.push(TocItem::new(entry.title.clone(), String::new(), entry.offset));
		stack.push(siblings.len() - 1);
		levels.push(entry.level);
	}
	toc
}

fn children_at_mut<'a>(toc: &'a mut Vec<TocItem>, path: &[usize]) -> &'a mut Vec<TocItem> {
	let mut current = toc;
	for &idx in path {
		current = &mut current[idx].children;
	}
	current
}
