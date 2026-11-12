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

impl BrailleTable {
	/// The name with its file name after it, since tables with much the same name, such as two called "Slovak braille" and "Slovak Braille Table", can only be told apart by their files.
	#[must_use]
	pub fn label(&self) -> String {
		if self.display_name == self.file_name {
			self.file_name.to_owned()
		} else {
			format!("{} ({})", self.display_name, self.file_name)
		}
	}
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

/// A braille code as an eBraille book names it: the language of its `xx-Brai` tag, and `a11y:brailleSystem`, such as `UEB grade2` or `German grade1 comp8`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BrailleCode<'a> {
	pub language: &'a str,
	pub system: &'a str,
	pub cells: Option<u8>,
}

/// The table for `code`, found as liblouis finds one, by the `#+` metadata tables describe themselves with: the same language and grade, and computer braille only for a computer code. A table that also names the code's system, such as `ueb`, or the language's region, wins, then one that reads both ways, then the shortest name, which is the plain table rather than a variant. `None` when no table matches.
#[must_use]
pub fn table_for_code(code: BrailleCode) -> Option<&'static str> {
	let mut subtags = code.language.split('-');
	let language = subtags.next().unwrap_or_default().to_ascii_lowercase();
	let region =
		subtags.find(|subtag| subtag.len() == 2 && !subtag.eq_ignore_ascii_case("Brai")).map(str::to_ascii_lowercase);
	let words: Vec<String> = code.system.split_whitespace().map(str::to_ascii_lowercase).collect();
	let grade = words.iter().find_map(|word| word.strip_prefix("grade")?.parse::<u8>().ok());
	let computer = words.iter().any(|word| word.starts_with("comp"));
	let systems: Vec<&str> = words
		.iter()
		.map(String::as_str)
		.filter(|word| !word.starts_with("grade") && *word != "no-grade" && !word.starts_with("comp"))
		.collect();
	// A word of the code that some table names as its system, such as `ueb` or `bfu`, rather than a language's name, such as `german`.
	let is_system = |word: &str| {
		TABLE_METADATA.iter().any(|(_, metadata)| {
			metadata.iter().any(|(key, value)| *key == "system" && value.eq_ignore_ascii_case(word))
		})
	};
	let named_system = systems.iter().copied().find(|word| is_system(word));
	// U.S. grade 2 (EBAE) and grade 1 carry no metadata.
	if language == "en" && systems.contains(&"ebae") {
		return table_name(if grade == Some(1) { "en-us-g1.ctb" } else { "en-us-g2.ctb" });
	}
	TABLE_METADATA
		.iter()
		.filter_map(|(name, metadata)| {
			let get = |key: &str| metadata.iter().find(|(k, _)| *k == key).map(|(_, value)| value.to_ascii_lowercase());
			let table_language = get("language")?;
			let (table_primary, table_region) = table_language
				.split_once('-')
				.map_or((table_language.as_str(), None), |(primary, region)| (primary, Some(region)));
			if table_primary != language {
				return None;
			}
			let is_computer = get("type").is_some_and(|kind| kind == "computer");
			if is_computer != computer {
				return None;
			}
			let table_grade = get("grade").and_then(|grade| grade.parse::<u8>().ok()).or_else(|| {
				match get("contraction").as_deref() {
					Some("full") => Some(2),
					Some("partial") => Some(1),
					Some("no") => Some(0),
					_ => None,
				}
			});
			if grade.is_some() && table_grade != grade {
				return None;
			}
			if let (Some(cells), Some(dots)) = (code.cells, get("dots").and_then(|dots| dots.parse::<u8>().ok()))
				&& cells != dots
			{
				return None;
			}
			let system = get("system");
			if named_system.is_some() && system.is_some() && system.as_deref() != named_system {
				return None;
			}
			let mut score = 0;
			if system.is_some() && system.as_deref() == named_system {
				score += 4;
			}
			let table_region =
				table_region.map(str::to_owned).or_else(|| get("region")?.split('-').nth(1).map(str::to_owned));
			if region.is_some() && region == table_region {
				score += 2;
			}
			if get("direction").is_none_or(|direction| direction != "forward") {
				score += 1;
			}
			Some((score, std::cmp::Reverse(name.len()), *name))
		})
		.max()
		.map(|(_, _, name)| name)
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
