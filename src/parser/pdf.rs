use std::sync::Arc;

use anyhow::{Result, anyhow};
use hayro_interpret::{
	BlendMode, ClipPath, Context, Device, GlyphDrawMode, Image, InterpreterSettings, PageExt, Paint, PathDrawMode,
	RectExt, SoftMask, font::Glyph, interpret_page,
};
use hayro_syntax::{LoadPdfError, Pdf};
use kurbo::{Affine, BezPath, Point};

use crate::{
	document::{Document, DocumentBuffer, Marker, MarkerType, ParserContext, ParserFlags},
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
		let data = std::fs::read(&context.file_path)
			.map_err(|err| anyhow!("Failed to read PDF {}: {err}", context.file_path))?;
		let password = context.password.as_deref().unwrap_or_default();
		let document = Pdf::new_with_password(Arc::new(data), password).map_err(map_load_error)?;
		let mut buffer = DocumentBuffer::new();
		for (page_index, page) in document.pages().iter().enumerate() {
			let marker_position = buffer.current_position();
			buffer.add_marker(
				Marker::new(MarkerType::PageBreak, marker_position).with_text(format!("Page {}", page_index + 1)),
			);
			let raw_text = extract_page_text(&document, page);
			let lines = process_text_lines(&raw_text);
			for line in lines {
				buffer.append(&line);
				buffer.append("\n");
			}
		}
		let metadata = document.metadata();
		let title =
			metadata.title.as_deref().map_or_else(|| extract_title_from_path(&context.file_path), decode_pdf_string);
		let author = metadata.author.as_deref().map(decode_pdf_string).unwrap_or_default();
		let toc_items = Vec::new();
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

fn extract_page_text(document: &Pdf, page: &hayro_syntax::page::Page<'_>) -> String {
	let settings = InterpreterSettings::default();
	let bbox = page.intersected_crop_box().to_kurbo();
	let initial_transform = page.initial_transform(true);
	let mut context = Context::new(initial_transform, bbox, document.xref(), settings);
	let mut extractor = TextExtractor::default();
	interpret_page(page, &mut context, &mut extractor);
	extractor.text
}

#[derive(Default)]
struct TextExtractor {
	text: String,
	last_pos: Option<(f64, f64)>,
	avg_dx: Option<f64>,
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

	fn draw_image(&mut self, _: Image<'_, '_>, _: Affine) {}

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
	if let Ok(text) = std::str::from_utf8(bytes) {
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
