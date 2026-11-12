//! Braille that is ready to emboss, back-translated to print: Braille Ready Files (BRF), and the Portable Embosser Format (PEF) the DAISY Consortium defined.
//!
//! Both hold braille a translator already produced, wrapped to the embosser's line length and page height. The parser joins the wrapped lines back into paragraphs, back-translates each with the reader's braille table through louis-rs, and keeps a marker at the start of every embossed page.

mod document;
mod tables;

use std::{borrow::Cow, collections::HashMap, fs};

use anyhow::{Context, Result};
use encoding_rs::WINDOWS_1252;
use louis::{TranslationOptions, Translator};
use rayon::prelude::*;
use roxmltree::{Document as XmlDocument, Node};

pub(crate) use self::document::back_translate_document;
pub use self::tables::{
	BrailleCode, BrailleTable, DEFAULT_TABLE, back_translator, braille_tables, chosen_table, table_for_code, table_name,
};
use crate::{
	document::{DocLen, Document, DocumentBuffer, Marker, MarkerType, ParserContext, TocItem},
	parser::{
		Parser,
		util::{path::extract_title_from_path, xml::read_xml_to_string},
	},
	t,
};

#[cfg(test)]
mod tests;

/// The cell for each North American Braille ASCII character from space to underscore, indexed by the character's offset from space, as dot bits.
const BRAILLE_ASCII: &[u8; 64] = b" A1B'K2L@CIF/MSP\"E3H9O6R^DJG>NTQ,*5<-U8V.%[$+X!&;:4\\0Z7(_?W]#Y)=";

const BLANK: char = '\u{2800}';

/// Dots 3 and 6, the hyphen, which a word broken across lines ends with.
const HYPHEN: char = '\u{2824}';

const NUMBER_SIGN: char = '\u{283c}';

/// A line indented this far is centered, as a heading is, rather than starting a paragraph.
const CENTERED_INDENT: usize = 4;

/// A first line repeated on this many pages is a running head.
const RUNNING_HEAD_PAGES: usize = 3;

const PEF_NAMESPACE: &str = "http://www.daisy.org/ns/2008/pef";

const DUBLIN_CORE_NAMESPACE: &str = "http://purl.org/dc/elements/1.1/";

/// An embossed page as its lines of Unicode braille.
type Page = Vec<String>;

pub struct BrfParser;

impl Parser for BrfParser {
	fn parse(&self, context: &ParserContext) -> Result<Document> {
		tracing::debug!(path = %context.file_path, table = context.braille_table, "parsing BRF file");
		let bytes =
			fs::read(&context.file_path).with_context(|| format!("Failed to open BRF file '{}'", context.file_path))?;
		let (translator, braille_ascii) = tables::back_translator(context.braille_table)?;
		let pages = brf_pages(&bytes, &braille_ascii);
		let buffer = back_translate(&pages, &translator, context);
		tracing::debug!(path = %context.file_path, pages = pages.len(), "parsed BRF file");
		let mut document = Document::new().with_title(extract_title_from_path(&context.file_path));
		document.set_buffer(buffer);
		Ok(document)
	}
}

/// A BRF page ends with a form feed, and an embosser's last one often leaves an empty page after it. Lines end with a line feed, a carriage return and a line feed, or, in older files, a carriage return alone.
///
/// `braille_ascii` is the table's own braille ASCII, which beyond the North American one uses letters outside ASCII, so a file is read as UTF-8 when it is valid UTF-8, and as Windows-1252 otherwise. A character the table has no cell for is read as North American Braille ASCII. Some files named `.brf` hold Unicode braille instead, as the Internet Archive's braille music does, and those cells are read as they are.
fn brf_pages(bytes: &[u8], braille_ascii: &HashMap<char, char>) -> Vec<Page> {
	let text =
		std::str::from_utf8(bytes).map_or_else(|_| WINDOWS_1252.decode_without_bom_handling(bytes).0, Cow::Borrowed);
	let characters: Vec<char> = text.chars().collect();
	let mut text = String::with_capacity(characters.len());
	for (index, &character) in characters.iter().enumerate() {
		match character {
			'\r' if characters.get(index + 1) == Some(&'\n') => {}
			'\r' => text.push('\n'),
			'\n' | '\x0c' => text.push(character),
			_ if is_cell(character) => text.push(character),
			_ => text.push(
				braille_ascii
					.get(&character)
					.copied()
					.unwrap_or_else(|| u8::try_from(character).map_or(character, ascii_to_cell)),
			),
		}
	}
	let mut pages: Vec<Page> = text.split('\x0c').map(|page| page.lines().map(str::to_string).collect()).collect();
	while pages.len() > 1 && pages.last().is_some_and(|page| page.iter().all(|line| is_blank(line))) {
		pages.pop();
	}
	pages
}

const fn is_cell(character: char) -> bool {
	matches!(character, '\u{2800}'..='\u{28ff}')
}

/// Lowercase letters stand for the same cells as capitals, as many BRF files are written in either.
fn ascii_to_cell(byte: u8) -> char {
	let byte = if (0x60..0x7f).contains(&byte) { byte - 0x20 } else { byte };
	match BRAILLE_ASCII.iter().position(|&character| character == byte) {
		Some(dots) => char::from_u32(0x2800 + u32::try_from(dots).unwrap_or_default()).unwrap_or(BLANK),
		None => char::from(byte),
	}
}

pub struct PefParser;

impl Parser for PefParser {
	fn parse(&self, context: &ParserContext) -> Result<Document> {
		tracing::debug!(path = %context.file_path, table = context.braille_table, "parsing PEF file");
		let xml = read_xml_to_string(&context.file_path)
			.with_context(|| format!("Failed to open PEF file '{}'", context.file_path))?;
		let pef = XmlDocument::parse(&xml).context("Invalid PEF document")?;
		let root = pef.root_element();
		if root.tag_name().namespace() != Some(PEF_NAMESPACE) || root.tag_name().name() != "pef" {
			anyhow::bail!("'{}' is not a PEF document", context.file_path);
		}
		let volumes: Vec<Vec<Page>> = pef_elements(root, "volume").map(volume_pages).collect();
		let pages: Vec<Page> = volumes.iter().flatten().cloned().collect();
		let (translator, _) = tables::back_translator(context.braille_table)?;
		let buffer = back_translate(&pages, &translator, context);
		tracing::debug!(path = %context.file_path, volumes = volumes.len(), pages = pages.len(), "parsed PEF file");
		let toc = if volumes.len() > 1 { volume_toc(&volumes, &buffer) } else { Vec::new() };
		let title = dublin_core(root, "title").unwrap_or_else(|| extract_title_from_path(&context.file_path));
		let mut document =
			Document::new().with_title(title).with_author(dublin_core(root, "creator").unwrap_or_default());
		document.set_buffer(buffer);
		document.toc_items = toc;
		Ok(document)
	}
}

fn pef_elements<'a, 'input>(node: Node<'a, 'input>, name: &'static str) -> impl Iterator<Item = Node<'a, 'input>> {
	node.descendants().filter(move |descendant| {
		descendant.tag_name().namespace() == Some(PEF_NAMESPACE) && descendant.tag_name().name() == name
	})
}

/// Some embossing software ends each row's text with a line break, which is markup, not braille.
fn volume_pages(volume: Node) -> Vec<Page> {
	pef_elements(volume, "page")
		.map(|page| {
			pef_elements(page, "row")
				.map(|row| {
					row.text()
						.unwrap_or_default()
						.chars()
						.filter(|&character| !matches!(character, '\n' | '\r' | '\t'))
						.collect()
				})
				.collect()
		})
		.collect()
}

/// Each volume's entry starts at its first page, which is where the page markers say it does.
fn volume_toc(volumes: &[Vec<Page>], buffer: &DocumentBuffer) -> Vec<TocItem> {
	let page_starts: Vec<_> = buffer
		.markers
		.iter()
		.filter(|marker| marker.mtype == MarkerType::PageBreak)
		.map(|marker| marker.position)
		.collect();
	let mut first_page = 0;
	let mut toc = Vec::new();
	for (index, pages) in volumes.iter().enumerate() {
		if let Some(&offset) = page_starts.get(first_page) {
			// TRANSLATORS: A table of contents entry for one volume of a braille book; {} is the volume's number
			toc.push(TocItem::new(t("Volume {}").replace("{}", &(index + 1).to_string()), String::new(), offset));
		}
		first_page += pages.len();
	}
	toc
}

fn dublin_core(root: Node, name: &str) -> Option<String> {
	root.descendants()
		.find(|node| node.tag_name().namespace() == Some(DUBLIN_CORE_NAMESPACE) && node.tag_name().name() == name)
		.and_then(|node| node.text())
		.map(str::trim)
		.filter(|text| !text.is_empty())
		.map(str::to_string)
}

fn is_blank(line: &str) -> bool {
	line.chars().all(|character| character == BLANK || character == ' ')
}

/// The pages as print, a paragraph to a line, with a marker where each page begins.
fn back_translate(pages: &[Page], translator: &Translator, context: &ParserContext) -> DocumentBuffer {
	let layout = Layout::of(pages, context.strip_running_text);
	let translated: Vec<(String, Vec<usize>)> =
		layout.paragraphs.par_iter().map(|paragraph| translate_paragraph(translator, paragraph)).collect();
	let mut buffer = DocumentBuffer::new();
	let mut paragraph_starts = Vec::with_capacity(translated.len());
	for (text, _) in &translated {
		paragraph_starts.push(buffer.end());
		buffer.append(text);
		buffer.append("\n");
	}
	for (index, start) in layout.page_starts.iter().enumerate() {
		let position = match *start {
			Some((paragraph, cell)) => {
				let (text, positions) = &translated[paragraph];
				let offset = positions.get(cell).copied().unwrap_or_else(|| text.chars().count());
				paragraph_starts[paragraph] + DocLen::from_raw(offset)
			}
			None => buffer.end(),
		};
		buffer.add_marker(Marker::new(MarkerType::PageBreak, position).with_text(format!("Page {}", index + 1)));
	}
	buffer
}

/// The paragraph's print, and for each of its cells where that cell's print begins.
fn translate_paragraph(translator: &Translator, paragraph: &str) -> (String, Vec<usize>) {
	let result = translator.translate_with_options(paragraph, TranslationOptions::default());
	(result.output, result.output_positions)
}

/// The braille rejoined into paragraphs, and where in them each page begins.
#[derive(Debug, Default, PartialEq, Eq)]
struct Layout {
	paragraphs: Vec<String>,
	/// The paragraph and cell each page's first line starts at, or `None` for pages after the last text.
	page_starts: Vec<Option<(usize, usize)>>,
}

impl Layout {
	/// Embossed braille wraps paragraphs at the line length, marks a new paragraph by indenting it, centers headings, and breaks a word only at a hyphen it already has. Pages are padded with blank cells, and number themselves at the end of their first or last line.
	fn of(pages: &[Page], strip_running_heads: bool) -> Self {
		let pages: Vec<Vec<String>> = pages.iter().map(page_lines).collect();
		let running_heads = if strip_running_heads { running_heads(&pages) } else { Vec::new() };
		let mut layout = Self::default();
		let mut current: Option<String> = None;
		let mut waiting_pages = Vec::new();
		for (page_index, lines) in pages.iter().enumerate() {
			layout.page_starts.push(None);
			waiting_pages.push(page_index);
			for (line_index, line) in lines.iter().enumerate() {
				if line_index == 0 && running_heads.contains(line) {
					continue;
				}
				let indent = line.chars().take_while(|&character| character == BLANK).count();
				let content: String = line.chars().skip(indent).collect();
				if content.is_empty() {
					layout.finish(&mut current);
					continue;
				}
				let standalone = indent >= CENTERED_INDENT || is_separator(&content);
				if indent > 0 || standalone {
					layout.finish(&mut current);
				}
				let paragraph_index = layout.paragraphs.len();
				let paragraph = current.get_or_insert_with(String::new);
				if !paragraph.is_empty() && !paragraph.ends_with(HYPHEN) {
					paragraph.push(BLANK);
				}
				let cell = paragraph.chars().count();
				for page in std::mem::take(&mut waiting_pages) {
					layout.page_starts[page] = Some((paragraph_index, cell));
				}
				paragraph.push_str(&content);
				if standalone {
					layout.finish(&mut current);
				}
			}
		}
		layout.finish(&mut current);
		layout
	}

	fn finish(&mut self, current: &mut Option<String>) {
		if let Some(paragraph) = current.take() {
			self.paragraphs.push(paragraph);
		}
	}
}

/// A page's lines without the blank cells padding them, their page numbers, or the blank lines at the page's end.
fn page_lines(page: &Page) -> Vec<String> {
	let mut lines: Vec<String> =
		page.iter().map(|line| line.replace(' ', "\u{2800}").trim_end_matches(BLANK).to_string()).collect();
	let last = lines.iter().rposition(|line| !line.is_empty()).unwrap_or_default();
	for index in [0, last] {
		if let Some(line) = lines.get_mut(index) {
			*line = without_page_number(line);
		}
	}
	while lines.last().is_some_and(String::is_empty) {
		lines.pop();
	}
	lines
}

/// A braille page number ends a line after a run of blank cells, or stands alone on it: a number sign and digits, perhaps after a letter for a continuation page, as in ⠁⠼⠁.
fn without_page_number(line: &str) -> String {
	let cells: Vec<char> = line.chars().collect();
	let token_start = cells.iter().rposition(|&cell| cell == BLANK).map_or(0, |index| index + 1);
	let token = &cells[token_start..];
	let blanks = cells[..token_start].iter().rev().take_while(|&&cell| cell == BLANK).count();
	let is_page_number =
		token.len() <= 6 && token.contains(&NUMBER_SIGN) && token.last().is_some_and(|&cell| is_digit_cell(cell));
	if is_page_number && token_start > 0 && (blanks >= 3 || blanks == token_start) {
		cells[..token_start - blanks].iter().collect()
	} else {
		line.to_string()
	}
}

/// The cells of the letters a to j, which are the digits after a number sign.
const fn is_digit_cell(cell: char) -> bool {
	matches!(
		cell,
		'\u{2801}'
			| '\u{2803}'
			| '\u{2809}'
			| '\u{2819}'
			| '\u{2811}'
			| '\u{280b}'
			| '\u{281b}'
			| '\u{2813}'
			| '\u{280a}'
			| '\u{281a}'
	)
}

/// A line of hyphens or colons marks where a print page turns, and belongs to neither paragraph around it.
fn is_separator(content: &str) -> bool {
	content.chars().take_while(|&cell| cell == HYPHEN || cell == '\u{2812}').count() >= 6
}

/// First lines that repeat on enough pages to be a running head rather than text.
fn running_heads(pages: &[Vec<String>]) -> Vec<String> {
	let mut counts: HashMap<&str, usize> = HashMap::new();
	for first in pages.iter().filter_map(|page| page.first()).filter(|line| !line.is_empty()) {
		*counts.entry(first.as_str()).or_default() += 1;
	}
	counts.into_iter().filter(|&(_, count)| count >= RUNNING_HEAD_PAGES).map(|(line, _)| line.to_string()).collect()
}
