//! Manual pages: roff source written with the `man` macro package, as `man(7)` describes it.
//!
//! A page is roff, which is a typesetting language, but a manual page uses only a small,
//! well-known part of it: section headings, paragraphs, tagged paragraphs for options, runs of
//! bold and italic, and blocks where the line breaks are the content. Everything here reads
//! that part and ignores the rest, which is what leaves a page readable rather than sprinkled
//! with typesetting instructions.
//!
//! The BSD `mdoc` package is a different set of macros carried in the same files, and is not
//! read here yet.
//!
//! Two things a page can carry are simplified rather than reproduced. A `tbl` table (`.TS`)
//! comes out as its rows one after another rather than as a table, and `.if` and friends are
//! skipped rather than evaluated, so a page that writes a paragraph inside a conditional loses
//! that paragraph.

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
mod render;

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
		let source = remove_soft_hyphens(&normalize_line_endings(&convert_to_utf8(&bytes)));
		if !looks_like_roff(&source) {
			anyhow::bail!("'{}' is not a manual page", context.file_path);
		}
		let page = Page::read(&source);
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

/// How much of a file to read before deciding it is not roff at all. A page opens with its
/// comments and its `.TH`, and the longest licence header among the pages here runs to about
/// forty lines.
const SNIFF_LINES: usize = 200;

/// Whether a file is roff source.
///
/// The extensions a manual page goes by say very little: a section number is a plausible name
/// for any file, and `gz` is claimed here only so that an installed page can be opened at all.
/// So the content is asked. Roff writes its instructions on lines of their own beginning with
/// a full stop, and a manual page cannot be written without them.
fn looks_like_roff(source: &str) -> bool {
	source
		.lines()
		.take(SNIFF_LINES)
		.filter_map(|line| control_line(line.trim_end()))
		.any(|(name, _)| !name.is_empty() && name.chars().all(|c| c.is_ascii_alphanumeric()))
}

/// Whether the bytes start with the gzip magic number. An installed page is gzipped and one in
/// a source tree is not, so the file itself is asked rather than its name.
fn is_gzipped(bytes: &[u8]) -> bool {
	bytes.starts_with(&[0x1f, 0x8b])
}

fn gunzip(bytes: &[u8], path: &str) -> Result<Vec<u8>> {
	let mut out = Vec::new();
	GzDecoder::new(bytes).read_to_end(&mut out).with_context(|| format!("Failed to decompress '{path}'"))?;
	Ok(out)
}

/// Whether a path names a manual page by its section: `ls.1`, `printf.3`, `Tcl_Init.3tcl`, and
/// the gzipped form of each.
///
/// The section is the only thing a manual page's name has to say for itself, the stem being the
/// name of whatever is documented. It is a digit, sometimes with letters after it naming a
/// sub-collection.
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

/// A page as read.
struct Page {
	buffer: DocumentBuffer,
	title: Option<String>,
	toc: Vec<TocItem>,
}

/// What the next line is expected to be, for the macros that take their argument from the line
/// after them rather than from their own.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Pending {
	Nothing,
	/// `.SH` and `.SS` with no arguments take the next line as the heading.
	Heading(i32),
	/// `.TP` always takes the next line as the tag of its tagged paragraph.
	Tag,
	/// `.B` and its friends with no arguments set the next line in that font.
	Font(Font),
}

struct Reader<'a> {
	renderer: Renderer,
	title: Option<String>,
	pending: Pending,
	lines: Vec<&'a str>,
	index: usize,
	/// The sections read so far, with subsections nested under the section they fall in.
	toc: Vec<TocItem>,
}

impl Page {
	fn read(source: &str) -> Self {
		let mut reader = Reader {
			renderer: Renderer::new(),
			title: None,
			pending: Pending::Nothing,
			lines: source.lines().collect(),
			index: 0,
			toc: Vec::new(),
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

	/// The next source line, with any continued lines joined onto it.
	///
	/// A line ending in a backslash continues onto the next, which is how a long macro call is
	/// written. An even number of trailing backslashes is not a continuation: those are escaped
	/// backslashes standing for themselves.
	fn joined_line(&mut self) -> String {
		let mut line = self.lines[self.index].to_string();
		self.index += 1;
		while trailing_backslashes(&line) % 2 == 1 && self.index < self.lines.len() {
			line.pop();
			line.push_str(self.lines[self.index]);
			self.index += 1;
		}
		line
	}

	/// A line of text, which may be the argument a macro took from the line after itself.
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

	/// One macro call.
	fn control(&mut self, name: &str, rest: &str) {
		// A macro standing where a tag was expected supplies the tag, as long as it is one that
		// lays down text. `.TP` followed by `.B \-\-help` is how most options are written.
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
				// Anything else is a typesetting instruction with nothing to say to a reader:
				// spacing, hyphenation, indentation, page geometry.
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

	/// Writes a heading and lists it in the table of contents. A subsection is listed under
	/// the section it falls in, and one that opens a page with no section above it stands on
	/// its own.
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

	/// `.TH name section ...`, which names the page. A manual page is known by its name and
	/// section together, which is how one page refers to another.
	fn title_header(&mut self, args: &[String]) {
		let Some(name) = args.first() else { return };
		let name = escape::plain(name);
		self.title = Some(match args.get(1) {
			Some(section) if !section.trim().is_empty() => format!("{name}({})", escape::plain(section).trim()),
			_ => name,
		});
	}

	/// `.IP` with a tag is a list item; without one it is an indented paragraph.
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

	/// `.UE` closes the link. Its argument is the punctuation that follows the link, which
	/// belongs after it rather than inside it.
	fn close_link(&mut self, args: &[String]) {
		self.renderer.end_link();
		if let Some(trailing) = args.first() {
			self.renderer.write_line(&escape::pieces(trailing));
		}
	}

	/// A `tbl` table, read as its rows rather than as a table.
	///
	/// The lines after `.TS` describe the table's options and its column formats, and a cell
	/// spanning several lines is wrapped in `T{` and `T}`. None of that is content. What is left
	/// is the rows, whose cells are separated by tabs.
	fn table(&mut self) {
		self.renderer.end_paragraph();
		let mut past_format = false;
		while self.index < self.lines.len() {
			let line = self.joined_line();
			let trimmed = line.trim().to_string();
			if trimmed == ".TE" {
				break;
			}
			if is_comment(&line) {
				continue;
			}
			if !past_format {
				if trimmed.ends_with(';') {
					continue;
				}
				if trimmed.ends_with('.') && !trimmed.starts_with('.') {
					past_format = true;
					continue;
				}
			}
			if matches!(trimmed.as_str(), "_" | "=") {
				continue;
			}
			// A cell running over several lines is wrapped in these, which are not content and can
			// share a line with the cells beside them.
			let row = line.replace("T{", "").replace("T}", "").replace('\t', "  ");
			if row.trim().is_empty() {
				continue;
			}
			if let Some((name, rest)) = control_line(&row) {
				let (name, rest) = (name.to_string(), rest.to_string());
				self.control(&name, &rest);
			} else {
				self.renderer.write_line(&escape::pieces(&row));
				self.renderer.break_line();
			}
		}
		self.renderer.end_paragraph();
	}

	/// Skips a macro definition, which runs to a line holding only `..`.
	fn skip_definition(&mut self) {
		while self.index < self.lines.len() {
			if self.joined_line().trim() == ".." {
				return;
			}
		}
	}

	/// Skips a conditional, including the block it opens with `\{`.
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

/// Whether a line is a roff comment, whose macro name is `\"`.
fn is_comment(line: &str) -> bool {
	let trimmed = line.trim_start();
	let Some(rest) = trimmed.strip_prefix('.').or_else(|| trimmed.strip_prefix('\'')) else {
		return false;
	};
	rest.is_empty() || rest.starts_with("\\\"") || rest.starts_with("\\#")
}

/// Splits a control line into its macro name and the rest of the line.
///
/// A control line starts with `.` or `'`, the second of which asks for no break before the
/// macro runs. Nothing here needs that distinction.
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

/// Splits a macro's arguments, which are separated by spaces unless quoted. Inside a quoted
/// argument a doubled quotation mark stands for one.
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

/// The two fonts a macro alternates its arguments between, or the one font it sets them all in.
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

/// The pieces a font macro lays down, or `None` if this is not one of them.
///
/// An alternating macro puts no space between its arguments: `.BR ls (1)` is how a page writes
/// a cross-reference, and it reads `ls(1)`. A single-font macro sets the whole line, so its
/// arguments keep the spaces between them.
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

/// The font a bare `.B`, `.I` or `.SM` sets the line after it in.
fn bare_font(name: &str, rest: &str) -> Option<Font> {
	if !rest.trim().is_empty() {
		return None;
	}
	let (first, second) = alternating_fonts(name)?;
	(first == second).then_some(first)
}
