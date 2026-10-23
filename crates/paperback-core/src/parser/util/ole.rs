//! The title and author old Office files keep in their `SummaryInformation` stream, an OLE property set: `.doc`, `.ppt` and their kin.

use std::{fs::File, io::Read};

use cfb::CompoundFile;
use encoding_rs::Encoding;

use crate::document::Document;

const CODE_PAGE: u32 = 1;
const TITLE: u32 = 2;
const AUTHOR: u32 = 4;
const VT_LPSTR: u32 = 0x1E;
const VT_LPWSTR: u32 = 0x1F;
const VT_I2: u32 = 2;

/// Names `document` by the title and author its file's summary information gives, where it gives them.
pub fn apply_summary_information(document: &mut Document, path: &str) {
	let (title, author) = File::open(path)
		.ok()
		.and_then(|file| CompoundFile::open(file).ok())
		.map(|mut file| {
			let mut stream = Vec::new();
			let read =
				file.open_stream("/\u{5}SummaryInformation").and_then(|mut summary| summary.read_to_end(&mut stream));
			if read.is_ok() { title_and_author(&stream) } else { (None, None) }
		})
		.unwrap_or_default();
	if let Some(title) = title {
		document.title = title;
	}
	if let Some(author) = author {
		document.author = author;
	}
}

/// The title and author in a `SummaryInformation` property set, each `None` when it is missing or empty.
fn title_and_author(stream: &[u8]) -> (Option<String>, Option<String>) {
	let read_u32 =
		|at: usize| stream.get(at..at + 4).map(|bytes| u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]));
	// The header is 28 bytes, then each property set's format id (16 bytes) and offset; the summary is the first set.
	let Some(set) = read_u32(44).and_then(|offset| usize::try_from(offset).ok()) else { return (None, None) };
	let Some(count) = read_u32(set + 4) else { return (None, None) };
	let mut properties = Vec::new();
	for index in 0..usize::try_from(count).unwrap_or(0).min(256) {
		let entry = set + 8 + index * 8;
		if let (Some(id), Some(offset)) = (read_u32(entry), read_u32(entry + 4)) {
			properties.push((id, set + usize::try_from(offset).unwrap_or(usize::MAX - set)));
		}
	}
	let code_page = properties
		.iter()
		.find(|(id, _)| *id == CODE_PAGE)
		.filter(|(_, at)| read_u32(*at) == Some(VT_I2))
		.and_then(|(_, at)| stream.get(at + 4..at + 6))
		.map_or(1252, |bytes| u16::from_le_bytes([bytes[0], bytes[1]]));
	let text = |wanted: u32| {
		let (_, at) = properties.iter().find(|(id, _)| *id == wanted)?;
		let length = usize::try_from(read_u32(at + 4)?).ok()?;
		let text = match read_u32(*at)? {
			VT_LPSTR => {
				encoding_for(code_page).decode_without_bom_handling(stream.get(at + 8..at + 8 + length)?).0.into_owned()
			}
			VT_LPWSTR => {
				let (pairs, _) = stream.get(at + 8..at + 8 + length * 2)?.as_chunks::<2>();
				let units: Vec<u16> = pairs.iter().map(|pair| u16::from_le_bytes(*pair)).collect();
				String::from_utf16_lossy(&units)
			}
			_ => return None,
		};
		let text = text.trim_end_matches('\0').trim().to_string();
		(!text.is_empty()).then_some(text)
	};
	(text(TITLE), text(AUTHOR))
}

/// The encoding a Windows code page number names, Windows-1252 for one this does not know.
fn encoding_for(code_page: u16) -> &'static Encoding {
	match code_page {
		65001 => encoding_rs::UTF_8,
		874 => encoding_rs::WINDOWS_874,
		932 => encoding_rs::SHIFT_JIS,
		936 => encoding_rs::GBK,
		949 => encoding_rs::EUC_KR,
		950 => encoding_rs::BIG5,
		1250 => encoding_rs::WINDOWS_1250,
		1251 => encoding_rs::WINDOWS_1251,
		1253 => encoding_rs::WINDOWS_1253,
		1254 => encoding_rs::WINDOWS_1254,
		1255 => encoding_rs::WINDOWS_1255,
		1256 => encoding_rs::WINDOWS_1256,
		1257 => encoding_rs::WINDOWS_1257,
		1258 => encoding_rs::WINDOWS_1258,
		_ => encoding_rs::WINDOWS_1252,
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	/// A summary property set holding a code page, a title and an author, laid out as Office writes it.
	fn summary(title: &[u8], author: &[u8]) -> Vec<u8> {
		let mut set = Vec::new();
		let properties: [(u32, Vec<u8>); 3] = [
			(CODE_PAGE, [VT_I2.to_le_bytes().as_slice(), &1251u16.to_le_bytes(), &[0, 0]].concat()),
			(
				TITLE,
				[VT_LPSTR.to_le_bytes().as_slice(), &u32::try_from(title.len()).unwrap().to_le_bytes(), title].concat(),
			),
			(
				AUTHOR,
				[VT_LPSTR.to_le_bytes().as_slice(), &u32::try_from(author.len()).unwrap().to_le_bytes(), author]
					.concat(),
			),
		];
		let mut offset = 8 + 8 * properties.len();
		let mut index = Vec::new();
		let mut values = Vec::new();
		for (id, value) in &properties {
			index.extend_from_slice(&id.to_le_bytes());
			index.extend_from_slice(&u32::try_from(offset).unwrap().to_le_bytes());
			offset += value.len();
			values.extend_from_slice(value);
		}
		set.extend_from_slice(&u32::try_from(offset).unwrap().to_le_bytes());
		set.extend_from_slice(&u32::try_from(properties.len()).unwrap().to_le_bytes());
		set.extend(index);
		set.extend(values);
		let mut stream = vec![0u8; 48];
		stream[44..48].copy_from_slice(&48u32.to_le_bytes());
		stream.extend(set);
		stream
	}

	#[test]
	fn the_summary_gives_the_title_and_author_in_its_code_page() {
		// "Отчёт" in Windows-1251, with the trailing nul Office writes.
		let stream = summary(b"\xce\xf2\xf7\xb8\xf2\0", b"Ann Smith\0");
		assert_eq!(title_and_author(&stream), (Some("Отчёт".into()), Some("Ann Smith".into())));
	}

	#[test]
	fn an_empty_or_broken_summary_gives_nothing() {
		assert_eq!(title_and_author(&summary(b"\0", b"  \0")), (None, None));
		assert_eq!(title_and_author(b"short"), (None, None));
	}
}
