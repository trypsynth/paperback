//! PowerPoint 95, whose format Microsoft never published. Its records carry the PowerPoint 97 type numbers under a 16-byte header (type, a field that is 0xFFFF on containers, length, and one more), and each paragraph run of text is a 0x7EE record whose last header field is 0x35, in Windows-1252.

use encoding_rs::WINDOWS_1252;

const HEADER_SIZE: usize = 16;
const CONTAINER_MARK: u32 = 0xFFFF;
const DOCUMENT: u32 = 3;
const SLIDE: u32 = 0x3EE;
const NOTES: u32 = 0x3F0;
const TEXT: u32 = 0x7EE;
const TEXT_MARK: u32 = 0x35;

/// The stream of a PowerPoint 95 file opens with its document container rather than a PowerPoint 97 record.
pub(super) fn is_pp95(stream: &[u8]) -> bool {
	header(stream, 0).is_some_and(|(record_type, mark, _, _)| record_type == DOCUMENT && mark == CONTAINER_MARK)
}

/// Each slide's text, in the order the presentation holds its slides; notes and masters are left out, as for PowerPoint 97.
pub(super) fn slide_texts(stream: &[u8]) -> Vec<String> {
	let mut slides = Vec::new();
	collect(stream, None, false, &mut slides);
	slides.into_iter().map(|parts| parts.join("\r")).collect()
}

/// `slide` is the slide being read, if any; a run under that slide's notes, or outside any slide, is not slide text.
fn collect(data: &[u8], slide: Option<usize>, in_notes: bool, slides: &mut Vec<Vec<String>>) {
	let mut offset = 0;
	while let Some((record_type, mark, len, last)) = header(data, offset) {
		let start = offset + HEADER_SIZE;
		let Some(body) = data.get(start..start + len) else { return };
		if record_type == TEXT && last == TEXT_MARK {
			if let (Some(index), false) = (slide, in_notes) {
				slides[index].push(WINDOWS_1252.decode(body).0.into_owned());
			}
		} else if mark == CONTAINER_MARK && len >= HEADER_SIZE {
			let slide = if record_type == SLIDE && slide.is_none() {
				slides.push(Vec::new());
				Some(slides.len() - 1)
			} else {
				slide
			};
			collect(body, slide, in_notes || record_type == NOTES, slides);
		}
		offset = start + len;
	}
}

fn header(data: &[u8], offset: usize) -> Option<(u32, u32, usize, u32)> {
	let field = |at: usize| {
		data.get(offset + at..offset + at + 4).map(|bytes| u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
	};
	Some((field(0)?, field(4)?, usize::try_from(field(8)?).ok()?, field(12)?))
}

#[cfg(test)]
mod tests;
