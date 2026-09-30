//! Finding where a KF8 book's original XHTML files started.
//!
//! A KF8 (`.azw3`) book was an EPUB before Amazon compiled it, and the compiler did not keep
//! those XHTML files whole. It split each one into a *skeleton*, the outer markup, and a run
//! of *fragments*, the content belonging inside it, then laid them out one after another in
//! the raw text. The skeleton index records where each file's outer markup sits.
//!
//! Reading the raw text straight through, which is what this parser does, already puts the
//! words in order, because a skeleton is markup and carries no text of its own. What it
//! loses is where one file ended and the next began. Those boundaries are the book's
//! sections, and this index is the only place they are written down.

use super::index::read_index;

/// `mobi_header` offset of the record holding the skeleton index.
const SKELETON_INDEX_OFFSET: usize = 236;

/// Tag carrying a start position and a length, in that order.
const TAG_POSITION: u8 = 6;

/// Where each of the book's original files starts in the raw text, in reading order.
///
/// Returns `None` when the book does not say: an ordinary MOBI has no such index, and a KF8
/// written without one reads perfectly well as a single flow.
pub(super) fn section_starts(data: &[u8], records: &[usize], mobi_header: &[u8]) -> Option<Vec<usize>> {
	let skeletons = read_index(data, records, index_record(mobi_header, SKELETON_INDEX_OFFSET)?)?;
	let mut starts: Vec<usize> = skeletons
		.entries
		.iter()
		// A skeleton that does not say where it lives cannot start a section, but the rest of
		// the book is still worth reading, so it is passed over rather than giving up.
		.filter_map(|entry| entry.tags.get(&TAG_POSITION)?.first().copied())
		.collect();
	starts.sort_unstable();
	starts.dedup();
	// A book of one section is a book with no sections to navigate between.
	if starts.len() < 2 { None } else { Some(starts) }
}

/// The record number an index pointer names, or `None` when the book has no such index.
fn index_record(mobi_header: &[u8], offset: usize) -> Option<usize> {
	let bytes = mobi_header.get(offset..offset + 4)?;
	let record = u32::from_be_bytes(bytes.try_into().ok()?);
	if record == 0 || record == 0xFFFF_FFFF { None } else { Some(record as usize) }
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn an_index_pointer_of_none_reads_as_absent() {
		let mut header = vec![0u8; 256];
		let field = SKELETON_INDEX_OFFSET..SKELETON_INDEX_OFFSET + 4;
		header[field.clone()].copy_from_slice(&0xFFFF_FFFFu32.to_be_bytes());
		assert!(index_record(&header, SKELETON_INDEX_OFFSET).is_none());
		// Zero means the same thing: the compiler wrote no such index.
		header[field.clone()].copy_from_slice(&0u32.to_be_bytes());
		assert!(index_record(&header, SKELETON_INDEX_OFFSET).is_none());
		header[field].copy_from_slice(&42u32.to_be_bytes());
		assert_eq!(index_record(&header, SKELETON_INDEX_OFFSET), Some(42));
	}

	#[test]
	fn a_header_too_short_to_hold_the_pointer_reads_as_absent() {
		assert!(index_record(&[0u8; 64], SKELETON_INDEX_OFFSET).is_none());
	}

	// A book with no skeleton index is an ordinary MOBI, and this has to decline rather than
	// fail so the reader still gets the book as one flow.
	#[test]
	fn a_book_with_no_index_reports_no_sections() {
		assert!(section_starts(&[], &[], &vec![0u8; 256]).is_none());
	}
}
