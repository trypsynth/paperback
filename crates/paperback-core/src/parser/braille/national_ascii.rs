//! The braille ASCII of countries whose braille files are often written with another table than the one liblouis displays braille with, such as by DAISY Pipeline, whose tables come from Dotify's braille-utils. Each gives the character for each of the 64 cells, in the order of the cell's dots as bits, as `BRAILLE_ASCII` does.

use std::{borrow::Cow, collections::HashMap};

use encoding_rs::{WINDOWS_1250, WINDOWS_1252};

pub(super) enum Encoding {
	Windows1252,
	Windows1250,
	Ibm850,
}

pub(super) struct NationalAscii {
	pub name: &'static str,
	pub languages: &'static [&'static str],
	pub encoding: Encoding,
	pub cells: &'static str,
}

#[rustfmt::skip]
const TABLES: &[NationalAscii] = &[
	// Danish
	NationalAscii { name: "DA_DK", languages: &["da"], encoding: Encoding::Ibm850, cells: " a,b.k;l'cif/msp`e:h*o!r~djgæntq@å?ê-u(v\\îøë§xèç^û_ü)z\"à|ôwï#yùé" },
	// German
	NationalAscii { name: "DE_DE", languages: &["de"], encoding: Encoding::Windows1252, cells: " a,b.k;l\"cif|msp!e:h*o+r>djg`ntq'1?2-u(v$3960x~&<5/8)z={_4w7#y}%" },
	// Czech
	NationalAscii { name: "CS_CZ", languages: &["cs"], encoding: Encoding::Windows1250, cells: " a,b.k;l'cifímsp~e:h*o!r^djgéntq§á?ě-u(v$čóňúxžý¤š+ť)z\"w|ďř/#yůľ" },
	// Spanish
	NationalAscii { name: "ES_ES", languages: &["es"], encoding: Encoding::Windows1252, cells: " a,b.k;l'cif/msp@e:h}o+r^djg|ntq_1?2-u<v{3960x$&\"5*8>z=(%4w7#y)\\" },
	// Spanish, table 2
	NationalAscii { name: "ES_ES_TABLE_2", languages: &["es"], encoding: Encoding::Windows1252, cells: " a,b.k;l'cifímsp@e:h}o+r^djgÌntq_1?2-u<v{396óxé&\"5*8>z=á%4w7#yú\\" },
	// Italian, Firenze
	NationalAscii { name: "IT_IT_FIRENZE", languages: &["it"], encoding: Encoding::Windows1252, cells: " a,b'k;l\"cif/msp)e:h*o!r%djg&ntq(1?2-u<v#396^x\\@+5.8>z=[$4w7_y]0" },
	// Dutch, Braillo 031
	NationalAscii { name: "BRAILLO_6DOT_031_01", languages: &["nl"], encoding: Encoding::Windows1252, cells: " a,b'k;l&cif/msp|e:h*o!r]djgäntq@\\?ê-u}v{îöëòxè#~û.ü\"z=à[ôwï%yùé" },
	// Swedish, Braillo 046
	NationalAscii { name: "BRAILLO_6DOT_046_01", languages: &["sv"], encoding: Encoding::Windows1252, cells: " a,b'k;l^cif/msp!e:h*o+r\"djg[ntq_1?2-u<v%396]x\\&#5.8>z=($4w70y)@" },
	// Swedish, CX
	NationalAscii { name: "SV_SE_CX", languages: &["sv"], encoding: Encoding::Windows1252, cells: " a,b.k;l^cif/msp'e:h*o!r~djgäntq_å?ê-u(v@îöë§xèç\"û+ü)z=à|ôwï#yùé" },
	// Norwegian, Braillo 047
	NationalAscii { name: "BRAILLO_6DOT_047_01", languages: &["no", "nb", "nn"], encoding: Encoding::Windows1252, cells: " A,B.K;L`CIF/MSP'E:H@O!RaDJG[NTQ*]?r-U\"Vqm\\h&Xli_e%u$Z=k|dWg#Ynj" },
	// North American, which Slovak and Czech files are written in although their tables define the same characters as other cells
	NationalAscii { name: "NABCC", languages: &["sk", "cs", "hu"], encoding: Encoding::Windows1252, cells: " A1B'K2L@CIF/MSP\"E3H9O6R^DJG>NTQ,*5<-U8V.%[$+X!&;:4\\0Z7(_?W]#Y)=" },
];

/// The upper half of code page 850, DOS Western European, which Danish braille files are written in.
const IBM850: &str = "ÇüéâäàåçêëèïîìÄÅÉæÆôöòûùÿÖÜø£Ø×ƒáíóúñÑªº¿®¬½¼¡«»░▒▓│┤ÁÂÀ©╣║╗╝¢¥┐└┴┬├─┼ãÃ╚╔╩╦╠═╬¤ðÐÊËÈıÍÎÏ┘┌█▄¦Ì▀ÓßÔÒõÕµþÞÚÛÙýÝ¯´\u{ad}±‗¾¶§÷¸°¨·¹³²■\u{a0}";

impl NationalAscii {
	pub fn for_language(language: &str) -> impl Iterator<Item = &'static Self> + '_ {
		TABLES.iter().filter(move |table| table.languages.contains(&language))
	}

	/// A file that isn't UTF-8 is read in the code page the table's embossers use.
	pub fn decode<'a>(&self, bytes: &'a [u8]) -> Cow<'a, str> {
		if let Ok(text) = std::str::from_utf8(bytes) {
			return Cow::Borrowed(text);
		}
		match self.encoding {
			Encoding::Windows1252 => WINDOWS_1252.decode_without_bom_handling(bytes).0,
			Encoding::Windows1250 => WINDOWS_1250.decode_without_bom_handling(bytes).0,
			Encoding::Ibm850 => {
				let upper: Vec<char> = IBM850.chars().collect();
				Cow::Owned(
					bytes
						.iter()
						.map(|&byte| {
							upper
								.get(usize::from(byte.wrapping_sub(0x80)))
								.copied()
								.filter(|_| byte >= 0x80)
								.unwrap_or_else(|| char::from(byte))
						})
						.collect(),
				)
			}
		}
	}

	/// The cell each character stands for. A letter the table writes in one case stands for the same cell in the other, as files are written in either, but Norwegian's capitals and small letters are different cells.
	pub fn cells(&self) -> HashMap<char, char> {
		let mut cells = HashMap::new();
		for (dots, character) in self.cells.chars().enumerate() {
			if let Some(cell) = char::from_u32(0x2800 + u32::try_from(dots).unwrap_or_default()) {
				cells.insert(character, cell);
			}
		}
		let same_letter: Vec<(char, char)> = cells
			.iter()
			.filter_map(|(&character, &cell)| {
				let other = if character.is_lowercase() {
					character.to_uppercase().next()
				} else {
					character.to_lowercase().next()
				};
				other.filter(|other| *other != character && !cells.contains_key(other)).map(|other| (other, cell))
			})
			.collect();
		cells.extend(same_letter);
		cells
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn each_table_names_64_different_cells() {
		for table in TABLES {
			let mut unique: Vec<char> = table.cells.chars().collect();
			assert_eq!(unique.len(), 64, "{}", table.name);
			unique.sort_unstable();
			unique.dedup();
			assert_eq!(unique.len(), 64, "{}", table.name);
		}
	}

	#[test]
	fn reads_danish_letters_in_code_page_850() {
		let danish = NationalAscii::for_language("da").next().expect("a Danish table");
		assert_eq!(danish.decode(&[0x91, 0x86, 0x9b]), "æåø");
		assert_eq!(danish.cells()[&'æ'], '\u{281c}');
	}

	#[test]
	fn keeps_norwegian_capitals_and_small_letters_apart() {
		let norwegian = NationalAscii::for_language("no").next().expect("a Norwegian table");
		let cells = norwegian.cells();
		assert_eq!(cells[&'A'], '\u{2801}');
		assert_eq!(cells[&'*'], '\u{2820}');
		assert_ne!(cells[&'a'], cells[&'A']);
	}
}
