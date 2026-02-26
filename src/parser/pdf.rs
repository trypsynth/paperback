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
	PageResult { text: extractor.text, has_images: extractor.has_images }
}

#[derive(Default)]
struct TextExtractor {
	text: String,
	last_pos: Option<(f64, f64)>,
	avg_dx: Option<f64>,
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
		let Some(unicode_char) = glyph.as_unicode() else { return };
		if unicode_char == '\0' {
			return;
		}
		let position = transform * glyph_transform * Point::new(0.0, 0.0);
		if let Some((last_x, last_y)) = self.last_pos {
			let dy = (position.y - last_y).abs();
			let dx = position.x - last_x;
			// Simple heuristics to separate lines/words without full layout reconstruction.
			if dy > 7.0 {
				self.text.push('\n');
				self.avg_dx = None;
			} else if dx > 0.0 {
				let avg = self.avg_dx.unwrap_or(dx);
				let last_char = self.text.chars().last();
				let alnum_pair = last_char.is_some_and(char::is_alphanumeric) && unicode_char.is_alphanumeric();
				let gap_threshold = if alnum_pair { (avg * 2.4).max(4.0) } else { (avg * 1.6).max(3.0) };
				if dx > gap_threshold && !self.text.ends_with([' ', '\n', '\r', '\t']) && unicode_char != ' ' {
					self.text.push(' ');
				}
				if dx < avg * 2.8 {
					self.avg_dx = Some(avg * 0.8 + dx * 0.2);
				}
			}
		}
		self.text.push(unicode_char);
		self.last_pos = Some((position.x, position.y));
	}

	fn draw_image(&mut self, _: Image<'_, '_>, _: Affine) {
		self.has_images = true;
	}

	fn pop_clip_path(&mut self) {}

	fn pop_transparency_group(&mut self) {}
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
