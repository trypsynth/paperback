//! Braille that is ready to emboss, back-translated to print: Braille Ready Files (BRF), and the Portable Embosser Format (PEF) the DAISY Consortium defined.
//!
//! Both hold braille a translator already produced, wrapped to the embosser's line length and page height. The parser joins the wrapped lines back into paragraphs, back-translates each with the reader's braille table through louis-rs, and keeps a marker at the start of every embossed page.

mod bopf;
mod document;
mod national_ascii;
mod tables;

use std::{borrow::Cow, collections::HashMap, fs, path::Path};

use anyhow::{Context, Result};
use encoding_rs::WINDOWS_1252;
use louis::{TranslationOptions, Translator};
use rayon::prelude::*;
use roxmltree::{Document as XmlDocument, Node};

pub(crate) use self::document::back_translate_document;
use self::national_ascii::NationalAscii;
pub use self::{
	bopf::BopfParser,
	tables::{
		BrailleCode, BrailleTable, DEFAULT_TABLE, back_translator, braille_tables, chosen_table, table_for_code,
		table_name,
	},
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
		// One volume of a BARD book, opened on its own, is read with the table its book's package names.
		let table = bopf::table_beside(Path::new(&context.file_path)).unwrap_or(context.braille_table);
		let context = &ParserContext { braille_table: table, ..context.clone() };
		let (translator, braille_ascii) = tables::back_translator(table)?;
		let pages = likeliest_pages(&bytes, &braille_ascii, &translator, context);
		let buffer = back_translate(&pages, &translator, context);
		tracing::debug!(path = %context.file_path, pages = pages.len(), "parsed BRF file");
		let mut document = Document::new().with_title(extract_title_from_path(&context.file_path));
		document.set_buffer(buffer);
		Ok(document)
	}
}

/// A BRF page ends with a form feed, and an embosser's last one often leaves an empty page after it. Lines end with a line feed, a carriage return and a line feed, or, in older files, a carriage return alone.
///
/// `braille_ascii` is the braille ASCII the file is read in. A character it has no cell for is read as North American Braille ASCII. Some files named `.brf` hold Unicode braille instead, as the Internet Archive's braille music does, and those cells are read as they are.
fn brf_pages(text: &str, braille_ascii: &HashMap<char, char>) -> Vec<Page> {
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

/// A table's own braille ASCII uses letters outside ASCII beyond the North American one, so a file is read as UTF-8 when it is valid UTF-8, and as Windows-1252 otherwise.
fn decode_brf(bytes: &[u8]) -> Cow<'_, str> {
	std::str::from_utf8(bytes).map_or_else(|_| WINDOWS_1252.decode_without_bom_handling(bytes).0, Cow::Borrowed)
}

/// Countries don't all write braille ASCII one way: DAISY Pipeline, for one, writes Norwegian, Swedish, Danish, Spanish and Italian files with other tables than the one liblouis displays braille with. So a file is read with each braille ASCII its table's language is written in, and the likeliest wins, the table's own when none is likelier.
fn likeliest_pages(
	bytes: &[u8],
	braille_ascii: &HashMap<char, char>,
	translator: &Translator,
	context: &ParserContext,
) -> Vec<Page> {
	let language = tables::table_language(context.braille_table);
	let others: Vec<&NationalAscii> = NationalAscii::for_language(&language).collect();
	if others.is_empty() {
		return brf_pages(&decode_brf(bytes), braille_ascii);
	}
	let candidate = |name: &'static str, text: &str, cells: &HashMap<char, char>| {
		let pages = brf_pages(text, cells);
		let likeness = Likeness::of(text, &pages, &Layout::of(&pages, context.strip_running_text), translator);
		tracing::debug!(
			braille_ascii = name,
			unread = likeness.unread,
			odd = likeness.odd,
			words = likeness.words,
			like_words = likeness.like_words,
			"scored a braille ASCII"
		);
		(likeness, pages)
	};
	let mut best = candidate("liblouis", &decode_brf(bytes), braille_ascii);
	for national in others {
		let other = candidate(national.name, &national.decode(bytes), &national.cells());
		if other.0.is_likelier_than(&best.0) {
			best = other;
		}
	}
	best.1
}

/// How likely a braille ASCII is the one a file was written in.
struct Likeness {
	/// Characters it has no cell for, which the right one has none of.
	unread: usize,
	/// Characters outside ASCII that aren't letters, which a file read in the wrong code page is full of, as Danish æ read in Windows-1252 is ‘.
	odd: usize,
	/// Of a sample of the file's words, how many read as words, and how many there are.
	like_words: u64,
	words: u64,
}

impl Likeness {
	fn of(text: &str, pages: &[Page], layout: &Layout, translator: &Translator) -> Self {
		let unread =
			pages.iter().flatten().flat_map(|line| line.chars()).filter(|&character| !is_cell(character)).count();
		let odd = text
			.chars()
			.filter(|&character| !character.is_ascii() && !character.is_alphabetic() && !is_cell(character))
			.count();
		let (like_words, words) = word_likeness(layout, translator);
		Self { unread, odd, like_words, words }
	}

	/// Fewer characters left unread, then fewer odd ones, and then more words reading as words out of all of them, compared without dividing.
	fn is_likelier_than(&self, other: &Self) -> bool {
		(self.unread, self.odd) < (other.unread, other.odd)
			|| ((self.unread, self.odd) == (other.unread, other.odd)
				&& u128::from(self.like_words) * u128::from(other.words)
					> u128::from(other.like_words) * u128::from(self.words))
	}
}

/// How many cells of a file's paragraphs are back-translated to judge its braille ASCII by: enough for a few hundred words, skipping the title pages at the start.
const SAMPLE_CELLS: usize = 6000;

/// Of a sample of the file's words, how many read as words, and how many there are. A wrong braille ASCII turns letters into digits and signs, and leaves cells the table has no print for.
fn word_likeness(layout: &Layout, translator: &Translator) -> (u64, u64) {
	let mut cells = 0;
	let sample: Vec<&String> = layout
		.paragraphs
		.iter()
		.skip(layout.paragraphs.len() / 10)
		.take_while(|paragraph| {
			let before = cells;
			cells += paragraph.chars().count();
			before < SAMPLE_CELLS
		})
		.collect();
	sample
		.par_iter()
		.map(|paragraph| words_like_words(&translate_paragraph(translator, paragraph).0))
		.reduce(|| (0, 0), |a, b| (a.0 + b.0, a.1 + b.1))
}

/// A word reads as one when, without the punctuation around it, it is letters, perhaps with an apostrophe or a hyphen, or a number. Signs such as `*` or `|` stuck to a word aren't punctuation, as a capital sign read in the wrong braille ASCII is.
fn words_like_words(text: &str) -> (u64, u64) {
	let punctuation = |character: char| ".,;:!?\"'()[]«»„“”‚‘’–—…-/¡¿".contains(character);
	let mut like = 0;
	let mut all = 0;
	for word in text.split_whitespace() {
		let core = word.trim_matches(punctuation);
		if core.is_empty() {
			continue;
		}
		all += 1;
		let letters = !core.is_empty()
			&& core.chars().all(|character| character.is_alphabetic() || matches!(character, '\'' | '’' | '-'));
		let number = !core.is_empty()
			&& core.chars().all(|character| character.is_ascii_digit() || matches!(character, '.' | ',' | ':' | '-'));
		if letters || number {
			like += 1;
		}
	}
	(like, all)
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
		let mut led_to_next = false;
		// Whether the last paragraph was a contents entry, and whether the open one started right after one.
		let mut after_entry = false;
		let mut opened_after_entry = false;
		let mut opened_at = 0;
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
					after_entry = false;
					continue;
				}
				let guided = without_guide_dots(&content);
				let led = guided != content;
				// A line of guide dots alone leads from an entry on the line before to its page number on the next.
				if guided.is_empty() {
					led_to_next = true;
					continue;
				}
				let content = guided;
				// A contents entry ends at its page number, after its guide dots. The next starts at the margin, as a paragraph's run-on line does, so it's told apart by the dots.
				let ends_entry = led || std::mem::take(&mut led_to_next);
				// Contents entries are indented as far as a centered heading, and aren't one.
				let standalone = (indent >= CENTERED_INDENT && !after_entry) || is_separator(&content);
				// An entry's run-on line is indented, where a paragraph's first line is, and ends the entry with its dots. It's indented less than the entry's first line, as one indented further is an entry under a heading.
				let runs_on = indent > 0
					&& indent < opened_at
					&& !standalone
					&& ends_entry
					&& current.is_some()
					&& opened_after_entry;
				if (indent > 0 && !runs_on) || standalone {
					if current.is_some() {
						after_entry = false;
					}
					layout.finish(&mut current);
				}
				if current.is_none() {
					opened_after_entry = after_entry;
					opened_at = indent;
				}
				let paragraph_index = layout.paragraphs.len();
				let paragraph = current.get_or_insert_with(String::new);
				if !paragraph.is_empty() && !ends_in_broken_word(paragraph) {
					paragraph.push(BLANK);
				}
				let cell = paragraph.chars().count();
				for page in std::mem::take(&mut waiting_pages) {
					layout.page_starts[page] = Some((paragraph_index, cell));
				}
				paragraph.push_str(&content);
				if standalone || ends_entry {
					layout.finish(&mut current);
					after_entry = ends_entry;
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

/// The guide dots of a contents page, a run of dot 5 cells between an entry and its page number, read as one space, as print's leaders are: three or more in a row, or two between spaces, are never text.
fn without_guide_dots(line: &str) -> String {
	let mut out = String::with_capacity(line.len());
	let mut cells = line.chars().peekable();
	while let Some(cell) = cells.next() {
		if cell != GUIDE_DOT {
			out.push(cell);
			continue;
		}
		let mut run = 1;
		while cells.next_if_eq(&GUIDE_DOT).is_some() {
			run += 1;
		}
		// An entry that nearly fills its line leaves room for only two, a pair with a space on either side.
		let alone = (out.is_empty() || out.ends_with(BLANK)) && cells.peek().is_none_or(|&next| next == BLANK);
		if run < 2 || (run == 2 && !alone) {
			out.extend(std::iter::repeat_n(GUIDE_DOT, run));
			continue;
		}
		while cells.next_if_eq(&BLANK).is_some() {}
		if !out.is_empty() && !out.ends_with(BLANK) && cells.peek().is_some() {
			out.push(BLANK);
		}
	}
	out.trim_end_matches(BLANK).to_string()
}

/// Dot 5, which guide dots are made of.
const GUIDE_DOT: char = '\u{2810}';

/// A line ends in a word broken at its hyphen when its last cell is the hyphen and the one before isn't dot 6 or another hyphen, which make it a dash: UEB and German write a dash as ⠠⠤, and some codes as ⠤⠤. The next line goes on that word, so no space joins them.
fn ends_in_broken_word(line: &str) -> bool {
	let mut cells = line.chars().rev();
	cells.next() == Some(HYPHEN) && !matches!(cells.next(), Some(HYPHEN | '\u{2820}'))
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
