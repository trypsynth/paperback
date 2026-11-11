//! The liblouis tables the build packs into the binary, and the back-translators built from them.

use std::{collections::HashMap, io::Read, path::Path};

use anyhow::{Context, Result};
use flate2::read::GzDecoder;
use louis::{Direction, Translator};

/// The commonest code in Braille Ready Files, which a reader who has not picked a table most likely has.
pub const DEFAULT_TABLE: &str = "en-ueb-g2.ctb";

/// A table a reader can pick, named as liblouis names it to people.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrailleTable {
	pub file_name: &'static str,
	pub display_name: &'static str,
}

static PACKED: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/liblouis-tables.bin"));

// The build script writes `PACKED_INDEX`, each table's name and where its gzip member lies in `PACKED`, sorted by name, and `CHOOSABLE`.
include!(concat!(env!("OUT_DIR"), "/liblouis-tables.rs"));

/// The tables a reader can choose, sorted by name: whole tables, not the `.cti` and `.uti` pieces they include or the display tables. Not every table has liblouis metadata naming it, such as `en-us-g2.ctb`, so the first line naming the table stands in, and then its file name.
#[must_use]
pub const fn braille_tables() -> &'static [BrailleTable] {
	CHOOSABLE
}

fn packed_entry(name: &str) -> Option<&'static (&'static str, usize, usize)> {
	PACKED_INDEX.binary_search_by(|(packed_name, ..)| (*packed_name).cmp(name)).ok().map(|index| &PACKED_INDEX[index])
}

fn table_source(name: &str) -> Option<String> {
	let (_, start, length) = packed_entry(name)?;
	let mut source = Vec::new();
	GzDecoder::new(PACKED.get(*start..start + length)?).read_to_end(&mut source).ok()?;
	Some(String::from_utf8_lossy(&source).into_owned())
}

/// The table a reader picked, read from its saved setting, or the default when the setting names no table Paperback has, as after an update drops one.
#[must_use]
pub fn chosen_table(setting: &str) -> &'static str {
	table_name(setting).unwrap_or(DEFAULT_TABLE)
}

/// The packed table named `name`, as a `'static` name, or `None` when there is no such table.
#[must_use]
pub fn table_name(name: &str) -> Option<&'static str> {
	packed_entry(name).map(|(name, ..)| *name)
}

/// The table and everything it includes as one source, since louis-rs reads includes only from files on disk. Hyphenation dictionaries are left out, as back-translation never hyphenates.
fn inlined_source(name: &str) -> Result<String> {
	let mut source = String::new();
	let mut included = Vec::new();
	inline(name, &mut included, &mut source)?;
	Ok(source)
}

fn inline(name: &str, included: &mut Vec<&'static str>, source: &mut String) -> Result<()> {
	let name = table_name(name).with_context(|| format!("There is no braille table named '{name}'"))?;
	if included.contains(&name) {
		return Ok(());
	}
	let table = table_source(name).with_context(|| format!("Braille table '{name}' could not be unpacked"))?;
	included.push(name);
	for line in table.lines() {
		if let ["include", file, ..] = line.split_whitespace().collect::<Vec<_>>().as_slice() {
			if !Path::new(file).extension().is_some_and(|extension| extension.eq_ignore_ascii_case("dic")) {
				inline(file, included, source)?;
			}
		} else {
			source.push_str(line);
		}
		source.push('\n');
	}
	Ok(())
}

/// A back-translator for the packed table `name`, and the cell each character of its braille ASCII stands for.
///
/// # Errors
///
/// Returns an error if there is no such table, or louis-rs cannot compile it.
pub fn back_translator(name: &str) -> Result<(Translator, HashMap<char, char>)> {
	let source = inlined_source(name)?;
	let translator = Translator::from_table_source(&source, Direction::Backward)
		.map_err(|error| anyhow::anyhow!("Braille table '{name}' could not be read: {error}"))?;
	Ok((translator, braille_ascii(&source)))
}

/// Braille ASCII differs by country, and a table gives the characters its country writes cells as, which is what liblouis writes a braille file with: German writes "hello world" as `heqo w?ld`, and Norwegian marks a capital with `´`. As in liblouis, they come from the `display` rules and the character definitions of one cell, such as French `lowercase é 123456`, and the first rule for a character wins. A braille file is six-dot, so a rule for a cell with dot 7 or 8 says nothing about one, such as the computer braille capital A, dots 1 and 7, that `en-ueb-g2.ctb` displays.
fn braille_ascii(source: &str) -> HashMap<char, char> {
	const OPCODES: [&str; 10] =
		["display", "space", "digit", "litdigit", "punctuation", "math", "sign", "letter", "uppercase", "lowercase"];
	let mut cells = HashMap::new();
	for line in source.lines() {
		let fields: Vec<&str> =
			line.split_whitespace().skip_while(|field| matches!(*field, "noback" | "nofor")).collect();
		let (character, dots) = match fields.as_slice() {
			[opcode, character, dots, ..] if OPCODES.contains(opcode) => (character, dots),
			_ => continue,
		};
		if let (Some(character), Some(cell)) = (unescape(character), cell(dots))
			&& u32::from(cell) < 0x2840
		{
			cells.entry(character).or_insert(cell);
		}
	}
	cells
}

/// A table's character operand: the character itself, or an escape such as `\s` or `\x00a8`.
fn unescape(operand: &str) -> Option<char> {
	let mut characters = operand.chars();
	let first = characters.next()?;
	if first != '\\' {
		return (characters.next().is_none()).then_some(first);
	}
	let rest = characters.as_str();
	match rest {
		"s" => Some(' '),
		"t" => Some('\t'),
		"\\" => Some('\\'),
		_ => {
			let (kind, hex) = rest.split_at_checked(1)?;
			matches!(kind, "x" | "y" | "z").then_some(())?;
			char::from_u32(u32::from_str_radix(hex, 16).ok()?)
		}
	}
}

/// A dots operand of one cell, such as `1279`, or `0` for the blank cell.
fn cell(dots: &str) -> Option<char> {
	if dots == "0" {
		return Some('\u{2800}');
	}
	let mut bits = 0;
	for digit in dots.chars() {
		let dot = digit.to_digit(10).filter(|dot| (1..=8).contains(dot))?;
		bits |= 1 << (dot - 1);
	}
	char::from_u32(0x2800 + bits)
}
