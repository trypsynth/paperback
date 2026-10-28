//! Only the small part of roff a manual page uses is read; `.if` and friends are skipped rather than evaluated, so a paragraph written inside a conditional is lost.

use std::{fs, io::Read, path::Path};

use anyhow::{Context, Result};
use flate2::read::GzDecoder;

use crate::{
	document::{Document, DocumentBuffer, ParserContext, TocItem},
	parser::{Parser, util::path::extract_title_from_path},
	util::{
		encoding::convert_to_utf8,
		text::{normalize_line_endings, remove_soft_hyphens},
	},
};

mod escape;
mod mdoc;
mod render;
mod table;

#[cfg(test)]
mod mdoc_tests;
#[cfg(test)]
mod tests;

use escape::{Font, Piece};
use render::Renderer;

pub struct ManParser;

impl Parser for ManParser {
	fn parse(&self, context: &ParserContext) -> Result<Document> {
		tracing::debug!(path = %context.file_path, "parsing manual page");
		let bytes = fs::read(&context.file_path)
			.with_context(|| format!("Failed to open manual page '{}'", context.file_path))?;
		let bytes = if is_gzipped(&bytes) { gunzip(&bytes, &context.file_path)? } else { bytes };
		let source = remove_soft_hyphens(normalize_line_endings(convert_to_utf8(&bytes)));
		if !looks_like_roff(&source) {
			anyhow::bail!("'{}' is not a manual page", context.file_path);
		}
		// `man` and `mdoc` pages share file names; only the first macro tells them apart.
		let page = if mdoc::is_mdoc(&source) {
			let read = mdoc::read(&source, context.render_tables_inline);
			Page { buffer: read.buffer, title: read.title, toc: read.toc }
		} else {
			Page::read(&source, context.render_tables_inline)
		};
		if page.buffer.content.trim().is_empty() {
			anyhow::bail!("'{}' holds no manual page text", context.file_path);
		}
		let title = page.title.unwrap_or_else(|| extract_title_from_path(&context.file_path));
		tracing::debug!(path = %context.file_path, chars = page.buffer.content.chars().count(), "parsed manual page");
		let mut document = Document::new().with_title(title);
		document.set_buffer(page.buffer);
		document.toc_items = page.toc;
		Ok(document)
	}
}

/// The longest licence header seen before a page's `.TH` runs to about forty lines.
const SNIFF_LINES: usize = 200;

/// A section number is a plausible extension for any file, so the content is checked for roff control lines.
fn looks_like_roff(source: &str) -> bool {
	source
		.lines()
		.take(SNIFF_LINES)
		.filter_map(|line| control_line(line.trim_end()))
		.any(|(name, _)| !name.is_empty() && name.chars().all(|c| c.is_ascii_alphanumeric()))
}

/// An installed page is gzipped and one in a source tree is not, so the bytes are checked rather than the name.
fn is_gzipped(bytes: &[u8]) -> bool {
	bytes.starts_with(&[0x1f, 0x8b])
}

fn gunzip(bytes: &[u8], path: &str) -> Result<Vec<u8>> {
	let mut out = Vec::new();
	GzDecoder::new(bytes).read_to_end(&mut out).with_context(|| format!("Failed to decompress '{path}'"))?;
	Ok(out)
}

#[must_use]
pub fn is_manual_page_name(path: &Path) -> bool {
	let unpacked = path.extension().is_some_and(|ext| ext.eq_ignore_ascii_case("gz")).then(|| path.file_stem());
	let name = match unpacked {
		Some(Some(stem)) => Path::new(stem),
		Some(None) => return false,
		None => path,
	};
	let Some(section) = name.extension().and_then(|ext| ext.to_str()) else { return false };
	let mut chars = section.chars();
	chars.next().is_some_and(|c| c.is_ascii_digit()) && chars.all(|c| c.is_ascii_alphanumeric())
}

struct Page {
	buffer: DocumentBuffer,
	title: Option<String>,
	toc: Vec<TocItem>,
}

/// Macros that take their argument from the next line: `.SH`, `.SS` and `.B` and friends when bare, and `.TP` always.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Pending {
	Nothing,
	Heading(i32),
	Tag,
	Font(Font),
}

struct Reader<'a> {
	renderer: Renderer,
	title: Option<String>,
	pending: Pending,
	lines: Vec<&'a str>,
	index: usize,
	toc: Vec<TocItem>,
	render_tables_inline: bool,
}

impl Page {
	fn read(source: &str, render_tables_inline: bool) -> Self {
		let mut reader = Reader {
			renderer: Renderer::new(),
			title: None,
			pending: Pending::Nothing,
			lines: source.lines().collect(),
			index: 0,
			toc: Vec::new(),
			render_tables_inline,
		};
		reader.run();
		Self { buffer: reader.renderer.finish(), title: reader.title, toc: reader.toc }
	}
}

impl Reader<'_> {
	fn run(&mut self) {
		while self.index < self.lines.len() {
			let line = self.joined_line();
			if is_comment(&line) {
				continue;
			}
			if let Some((name, rest)) = control_line(&line) {
				let (name, rest) = (name.to_string(), rest.to_string());
				self.control(&name, &rest);
			} else {
				self.text(&line);
			}
		}
	}

	fn joined_line(&mut self) -> String {
		let mut line = self.lines[self.index].to_string();
		self.index += 1;
		// An even number of trailing backslashes is escaped backslashes, not a continuation.
		while trailing_backslashes(&line) % 2 == 1 && self.index < self.lines.len() {
			line.pop();
			line.push_str(self.lines[self.index]);
			self.index += 1;
		}
		line
	}

	fn text(&mut self, line: &str) {
		match std::mem::replace(&mut self.pending, Pending::Nothing) {
			Pending::Heading(level) => self.add_heading(&escape::plain(line), level),
			Pending::Tag => {
				let pieces = escape::pieces(line);
				self.renderer.start_item(&pieces);
			}
			Pending::Font(font) => {
				let mut pieces = vec![Piece::Font(font)];
				pieces.extend(escape::pieces(line));
				pieces.push(Piece::Font(Font::Roman));
				self.renderer.write_line(&pieces);
			}
			Pending::Nothing => self.renderer.write_line(&escape::pieces(line)),
		}
	}

	fn control(&mut self, name: &str, rest: &str) {
		// A text-laying macro where a tag was expected supplies the tag: `.TP` then `.B \-\-help` is how most options are written.
		if self.pending == Pending::Tag {
			self.pending = Pending::Nothing;
			if let Some(pieces) = font_macro_pieces(name, rest) {
				self.renderer.start_item(&pieces);
				return;
			}
		}
		let args = split_args(rest);
		match name {
			"TH" => self.title_header(&args),
			"SH" | "SS" => self.section_heading(name, &args),
			"PP" | "LP" | "P" | "HP" | "sp" | "RS" | "RE" | "IX" => self.renderer.end_paragraph(),
			"TP" | "TQ" => {
				self.renderer.end_paragraph();
				self.pending = Pending::Tag;
			}
			"IP" => self.indented_paragraph(&args),
			"br" => self.renderer.break_line(),
			"nf" | "EX" => {
				self.renderer.break_line();
				self.renderer.set_filling(false);
			}
			"fi" | "EE" => {
				self.renderer.break_line();
				self.renderer.set_filling(true);
			}
			"UR" | "MT" => self.open_link(name, &args),
			"UE" | "ME" => self.close_link(&args),
			"TS" => self.table(),
			"de" | "de1" | "am" | "ig" => self.skip_definition(),
			"if" | "ie" | "el" => self.skip_conditional(rest),
			_ => {
				if let Some(pieces) = font_macro_pieces(name, rest) {
					self.renderer.write_line(&pieces);
				} else if let Some(font) = bare_font(name, rest) {
					self.pending = Pending::Font(font);
				}
				// Anything else is spacing, hyphenation, indentation or page geometry, with nothing to say to a reader.
			}
		}
	}

	fn section_heading(&mut self, name: &str, args: &[String]) {
		let level = i32::from(name == "SS") + 1;
		if args.is_empty() {
			self.pending = Pending::Heading(level);
		} else {
			let text: Vec<String> = args.iter().map(|arg| escape::plain(arg)).collect();
			self.add_heading(&text.join(" "), level);
		}
	}

	fn add_heading(&mut self, text: &str, level: i32) {
		let text = text.trim();
		let Some(position) = self.renderer.heading(text, level) else {
			return;
		};
		let item = TocItem::new(text.to_string(), String::new(), position);
		match self.toc.last_mut() {
			Some(section) if level > 1 => section.children.push(item),
			_ => self.toc.push(item),
		}
	}

	/// A manual page is known by its name and section together, which is how one page refers to another.
	fn title_header(&mut self, args: &[String]) {
		let Some(name) = args.first() else { return };
		let name = escape::plain(name);
		self.title = Some(match args.get(1) {
			Some(section) if !section.trim().is_empty() => format!("{name}({})", escape::plain(section).trim()),
			_ => name,
		});
	}

	fn indented_paragraph(&mut self, args: &[String]) {
		match args.first() {
			Some(tag) if !tag.trim().is_empty() => {
				let pieces = escape::pieces(tag);
				self.renderer.start_item(&pieces);
			}
			_ => self.renderer.end_paragraph(),
		}
	}

	fn open_link(&mut self, name: &str, args: &[String]) {
		let target = args.first().map(|arg| escape::plain(arg)).unwrap_or_default();
		let url = if name == "MT" { format!("mailto:{target}") } else { target };
		self.renderer.begin_link(url);
	}

	/// `.UE`'s argument is the punctuation that follows the link, which belongs after it rather than inside it.
	fn close_link(&mut self, args: &[String]) {
		self.renderer.end_link();
		if let Some(trailing) = args.first() {
			self.renderer.write_line(&escape::pieces(trailing));
		}
	}

	fn table(&mut self) {
		self.renderer.end_paragraph();
		let mut table = table::Table::new();
		let mut rows: Vec<Vec<String>> = Vec::new();
		while self.index < self.lines.len() {
			let line = self.joined_line();
			// A cell left open at the end of a line carries on into the lines below it.
			let mut index = self.index;
			let lines = &self.lines;
			let row = table.read_line(&line, || {
				let next = lines.get(index).map(|line| (*line).to_string());
				index += 1;
				next
			});
			self.index = index;
			match row {
				table::Row::Cells(cells) => rows.push(cells),
				table::Row::Nothing => {}
				table::Row::End => break,
			}
		}
		self.renderer.table(&rows, self.render_tables_inline);
	}

	fn skip_definition(&mut self) {
		while self.index < self.lines.len() {
			if self.joined_line().trim() == ".." {
				return;
			}
		}
	}

	fn skip_conditional(&mut self, rest: &str) {
		if !rest.contains("\\{") {
			return;
		}
		let mut depth = rest.matches("\\{").count().saturating_sub(rest.matches("\\}").count());
		while self.index < self.lines.len() && depth > 0 {
			let line = self.joined_line();
			depth += line.matches("\\{").count();
			depth = depth.saturating_sub(line.matches("\\}").count());
		}
	}
}

fn is_comment(line: &str) -> bool {
	let trimmed = line.trim_start();
	let Some(rest) = trimmed.strip_prefix('.').or_else(|| trimmed.strip_prefix('\'')) else {
		return false;
	};
	rest.is_empty() || rest.starts_with("\\\"") || rest.starts_with("\\#")
}

fn control_line(line: &str) -> Option<(&str, &str)> {
	let rest = line.strip_prefix('.').or_else(|| line.strip_prefix('\''))?;
	let rest = rest.trim_start();
	let end = rest.find(char::is_whitespace).unwrap_or(rest.len());
	let (name, arguments) = rest.split_at(end);
	Some((name, arguments.trim_start()))
}

fn trailing_backslashes(line: &str) -> usize {
	line.chars().rev().take_while(|&c| c == '\\').count()
}

fn split_args(rest: &str) -> Vec<String> {
	let mut args = Vec::new();
	let mut current = String::new();
	let mut in_quotes = false;
	let mut started = false;
	let mut chars = rest.chars().peekable();
	while let Some(c) = chars.next() {
		match c {
			'"' if in_quotes => {
				if chars.peek() == Some(&'"') {
					chars.next();
					current.push('"');
				} else {
					in_quotes = false;
				}
			}
			'"' if !started => {
				in_quotes = true;
				started = true;
			}
			c if c.is_whitespace() && !in_quotes => {
				if started {
					args.push(std::mem::take(&mut current));
					started = false;
				}
			}
			c => {
				current.push(c);
				started = true;
			}
		}
	}
	if started {
		args.push(current);
	}
	args
}

const fn alternating_fonts(name: &str) -> Option<(Font, Font)> {
	Some(match name.as_bytes() {
		b"BR" => (Font::Bold, Font::Roman),
		b"RB" => (Font::Roman, Font::Bold),
		b"BI" => (Font::Bold, Font::Italic),
		b"IB" => (Font::Italic, Font::Bold),
		b"IR" => (Font::Italic, Font::Roman),
		b"RI" => (Font::Roman, Font::Italic),
		b"B" | b"SB" => (Font::Bold, Font::Bold),
		b"I" => (Font::Italic, Font::Italic),
		b"SM" | b"R" => (Font::Roman, Font::Roman),
		_ => return None,
	})
}

/// An alternating macro puts no space between its arguments, so the cross-reference `.BR ls (1)` reads `ls(1)`.
fn font_macro_pieces(name: &str, rest: &str) -> Option<Vec<Piece>> {
	let (first, second) = alternating_fonts(name)?;
	let args = split_args(rest);
	if args.is_empty() {
		return None;
	}
	let one_font = first == second;
	let mut pieces = Vec::new();
	for (index, arg) in args.iter().enumerate() {
		if one_font && index > 0 {
			pieces.push(Piece::Text(" ".to_string()));
		}
		pieces.push(Piece::Font(if index % 2 == 0 { first } else { second }));
		pieces.extend(escape::pieces(arg));
	}
	pieces.push(Piece::Font(Font::Roman));
	Some(pieces)
}

fn bare_font(name: &str, rest: &str) -> Option<Font> {
	if !rest.trim().is_empty() {
		return None;
	}
	let (first, second) = alternating_fonts(name)?;
	(first == second).then_some(first)
}
