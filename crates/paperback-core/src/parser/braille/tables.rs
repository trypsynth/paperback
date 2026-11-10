//! The liblouis tables the build packs into the binary, and the back-translators built from them.

use std::{collections::HashMap, io::Read, path::Path, sync::OnceLock};

use anyhow::{Context, Result};
use flate2::read::GzDecoder;
use louis::{Direction, Translator};

/// The commonest code in Braille Ready Files, which a reader who has not picked a table most likely has.
pub const DEFAULT_TABLE: &str = "en-ueb-g2.ctb";

static PACKED: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/liblouis-tables.gz"));

/// A table a reader can pick, named as liblouis names it to people.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrailleTable {
	pub file_name: &'static str,
	pub display_name: &'static str,
}

fn tables() -> &'static HashMap<String, String> {
	static TABLES: OnceLock<HashMap<String, String>> = OnceLock::new();
	TABLES.get_or_init(|| {
		let mut packed = Vec::new();
		GzDecoder::new(PACKED).read_to_end(&mut packed).expect("the build packs valid gzip");
		let mut fields = Fields(&packed);
		let mut tables = HashMap::new();
		while let (Some(name), Some(contents)) = (fields.next(), fields.next()) {
			tables.insert(String::from_utf8_lossy(name).into_owned(), String::from_utf8_lossy(contents).into_owned());
		}
		tables
	})
}

/// Reads back what the build script wrote: each field is its length as four little-endian bytes, then the field.
struct Fields<'a>(&'a [u8]);

impl<'a> Iterator for Fields<'a> {
	type Item = &'a [u8];

	fn next(&mut self) -> Option<&'a [u8]> {
		let (length, rest) = self.0.split_first_chunk::<4>()?;
		let length = usize::try_from(u32::from_le_bytes(*length)).ok()?;
		let (field, rest) = rest.split_at_checked(length)?;
		self.0 = rest;
		Some(field)
	}
}

/// The tables a reader can choose, sorted by name: whole tables, not the `.cti` and `.uti` pieces they include or the display tables. Not every table has liblouis metadata naming it, such as `en-us-g2.ctb`, so the first line naming the table stands in, and then its file name.
#[must_use]
pub fn braille_tables() -> &'static [BrailleTable] {
	static LIST: OnceLock<Vec<BrailleTable>> = OnceLock::new();
	LIST.get_or_init(|| {
		let mut list: Vec<BrailleTable> = tables()
			.iter()
			.filter(|(file_name, _)| [".ctb", ".utb", ".tbl"].iter().any(|extension| file_name.ends_with(extension)))
			.map(|(file_name, source)| {
				let named = |prefix: &str| {
					source
						.lines()
						.find_map(|line| line.strip_prefix(prefix))
						.map(str::trim)
						.filter(|name| !name.is_empty())
				};
				let display_name = named("#-display-name:").or_else(|| named("# liblouis:")).unwrap_or(file_name);
				BrailleTable { file_name: file_name.as_str(), display_name }
			})
			.collect();
		list.sort_by(|a, b| a.display_name.cmp(b.display_name).then(a.file_name.cmp(b.file_name)));
		list
	})
}

/// The table a reader picked, read from its saved setting, or the default when the setting names no table Paperback has, as after an update drops one.
#[must_use]
pub fn chosen_table(setting: &str) -> &'static str {
	table_name(setting).unwrap_or(DEFAULT_TABLE)
}

/// The packed table named `name`, as a `'static` name, or `None` when there is no such table.
#[must_use]
pub fn table_name(name: &str) -> Option<&'static str> {
	tables().get_key_value(name).map(|(name, _)| name.as_str())
}

/// The table and everything it includes as one source, since louis-rs reads includes only from files on disk. Hyphenation dictionaries are left out, as back-translation never hyphenates.
fn inlined_source(name: &str) -> Result<String> {
	let mut source = String::new();
	let mut included = Vec::new();
	inline(name, &mut included, &mut source)?;
	Ok(source)
}

fn inline(name: &str, included: &mut Vec<&'static str>, source: &mut String) -> Result<()> {
	let (name, table) =
		tables().get_key_value(name).with_context(|| format!("There is no braille table named '{name}'"))?;
	if included.contains(&name.as_str()) {
		return Ok(());
	}
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

/// A back-translator for the packed table `name`.
///
/// # Errors
///
/// Returns an error if there is no such table, or louis-rs cannot compile it.
pub fn back_translator(name: &str) -> Result<Translator> {
	let source = inlined_source(name)?;
	Translator::from_table_source(&source, Direction::Backward)
		.map_err(|error| anyhow::anyhow!("Braille table '{name}' could not be read: {error}"))
}
