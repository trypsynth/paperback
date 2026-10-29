//! A MOBI picture is named by its place among the picture records (`recindex`, or KF8's `kindle:embed`), which nothing reaches once the book is text, so each is rewritten to its absolute record, `mobi-record:57.jpg`, which saving a picture reads straight out of the file.

use std::sync::LazyLock;

use regex::{Captures, Regex};

/// `mobi_header` offset of the record number the book's pictures, or for KF8 its resources, start at.
const FIRST_IMAGE_OFFSET: usize = 92;

pub(super) fn point_pictures_at_records(html: &str, data: &[u8], records: &[usize], mobi_header: &[u8]) -> String {
	static IMG: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?i)<img\b[^>]*>").unwrap());
	static RECINDEX: LazyLock<Regex> =
		LazyLock::new(|| Regex::new(r#"(?i)\srecindex\s*=\s*["']?0*(\d+)["']?"#).unwrap());
	static EMBED: LazyLock<Regex> =
		LazyLock::new(|| Regex::new(r#"(?i)\ssrc\s*=\s*["']kindle:embed:([0-9a-v]+)[^"']*["']"#).unwrap());
	let Some(first) = first_image_record(mobi_header) else { return html.to_string() };
	let source = |ordinal: usize| {
		let record = first + ordinal.checked_sub(1)?;
		let bytes = record_bytes(data, records, record)?;
		Some(format!(" src=\"mobi-record:{record}{}\"", extension(bytes)))
	};
	IMG.replace_all(html, |tag: &Captures| {
		let tag = &tag[0];
		if let Some(embed) = EMBED.captures(tag) {
			let Some(attribute) = usize::from_str_radix(&embed[1], 32).ok().and_then(source) else {
				return tag.to_string();
			};
			return EMBED.replace(tag, attribute.as_str()).into_owned();
		}
		if let Some(index) = RECINDEX.captures(tag)
			&& let Some(attribute) = index[1].parse().ok().and_then(source)
		{
			return tag.replacen("<img", &format!("<img{attribute}"), 1).replacen(
				"<IMG",
				&format!("<IMG{attribute}"),
				1,
			);
		}
		tag.to_string()
	})
	.into_owned()
}

fn first_image_record(mobi_header: &[u8]) -> Option<usize> {
	let bytes = mobi_header.get(FIRST_IMAGE_OFFSET..FIRST_IMAGE_OFFSET + 4)?;
	let first = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
	(first != u32::MAX && first > 0).then(|| usize::try_from(first).ok()).flatten()
}

#[must_use]
pub fn record_bytes<'a>(data: &'a [u8], records: &[usize], index: usize) -> Option<&'a [u8]> {
	let start = *records.get(index)?;
	let end = records.get(index + 1).copied().unwrap_or(data.len());
	data.get(start..end).filter(|bytes| !bytes.is_empty())
}

/// Lets a record be read without parsing the rest of the book.
#[must_use]
pub fn record_offsets(data: &[u8]) -> Vec<usize> {
	let Some(count) = data.get(76..78).map(|bytes| usize::from(u16::from_be_bytes([bytes[0], bytes[1]]))) else {
		return Vec::new();
	};
	(0..count)
		.map_while(|index| {
			let at = 78 + index * 8;
			data.get(at..at + 4).map(|bytes| u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as usize)
		})
		.collect()
}

/// So a saved picture opens as what it is.
fn extension(bytes: &[u8]) -> &'static str {
	if bytes.starts_with(&[0xFF, 0xD8]) {
		".jpg"
	} else if bytes.starts_with(b"\x89PNG") {
		".png"
	} else if bytes.starts_with(b"GIF8") {
		".gif"
	} else if bytes.starts_with(b"BM") {
		".bmp"
	} else {
		""
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn book(records: &[&[u8]], first: u32) -> (Vec<u8>, Vec<usize>, Vec<u8>) {
		let header_end = 78 + records.len() * 8;
		let mut data = vec![0u8; 76];
		data.extend_from_slice(&u16::try_from(records.len()).unwrap().to_be_bytes());
		let mut offset = header_end;
		let mut offsets = Vec::new();
		for record in records {
			offsets.push(offset);
			data.extend_from_slice(&u32::try_from(offset).unwrap().to_be_bytes());
			data.extend_from_slice(&[0, 0, 0, 0]);
			offset += record.len();
		}
		for record in records {
			data.extend_from_slice(record);
		}
		let mut mobi_header = vec![0u8; 96];
		mobi_header[92..96].copy_from_slice(&first.to_be_bytes());
		(data, offsets, mobi_header)
	}

	#[test]
	fn old_and_kf8_pictures_point_at_their_records() {
		let (data, offsets, header) = book(&[b"rec0", b"text", b"\xFF\xD8jpeg", b"GIF89a"], 2);
		assert_eq!(record_offsets(&data), offsets);
		let html = r#"<p><img recindex="00002" alt="Map"></p><p><img src="kindle:embed:0001?mime=image/jpeg"/></p>"#;
		assert_eq!(
			point_pictures_at_records(html, &data, &offsets, &header),
			r#"<p><img src="mobi-record:3.gif" recindex="00002" alt="Map"></p><p><img src="mobi-record:2.jpg"/></p>"#
		);
		assert_eq!(record_bytes(&data, &offsets, 3), Some(b"GIF89a".as_slice()));
	}

	#[test]
	fn a_picture_past_the_last_record_is_left_alone() {
		let (data, offsets, header) = book(&[b"rec0", b"\xFF\xD8"], 1);
		let html = r#"<img recindex="9">"#;
		assert_eq!(point_pictures_at_records(html, &data, &offsets, &header), html);
	}
}
