//! Finding the live objects in a legacy PowerPoint stream, which keeps every save's objects: the Current User stream leads to the latest `UserEditAtom`, and its persist directory to each object.
//!
//! <https://docs.microsoft.com/en-us/openspecs/office_file_formats/ms-ppt/1fc22d56-28f9-4818-bd45-67c2bf721ccf>

use std::collections::BTreeMap;

/// Offset of the `CurrentUserAtom`'s `offsetToCurrentEdit`, past its header, its size and its encryption token.
const OFFSET_TO_CURRENT_EDIT_AT: usize = 16;

const RT_USER_EDIT_ATOM: u16 = 0x0FF5;
const RT_PERSIST_DIRECTORY_ATOM: u16 = 0x1772;

/// Offsets within a `UserEditAtom`, from the start of its record header.
const OFFSET_LAST_EDIT_AT: usize = 16;
const OFFSET_PERSIST_DIRECTORY_AT: usize = 20;
const DOC_PERSIST_ID_REF_AT: usize = 24;

/// How many saves to follow before deciding the chain does not end, so that a malformed file
/// cannot loop forever.
const MAX_USER_EDITS: usize = 1024;

pub(super) const RECORD_HEADER_SIZE: usize = 8;

fn read_u32(data: &[u8], at: usize) -> Option<u32> {
	let bytes = data.get(at..at + 4)?;
	Some(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}

fn read_u16(data: &[u8], at: usize) -> Option<u16> {
	let bytes = data.get(at..at + 2)?;
	Some(u16::from_le_bytes([bytes[0], bytes[1]]))
}

/// The whole record at `at`, its header included, or `None` if it runs past the end.
fn record_at(stream: &[u8], at: usize) -> Option<&[u8]> {
	let length = read_u32(stream, at + 4)? as usize;
	stream.get(at..at + RECORD_HEADER_SIZE + length)
}

/// The persist objects the current save points at.
pub(super) struct Presentation<'a> {
	stream: &'a [u8],
	/// Persist identifier to the offset of the object it names.
	directory: BTreeMap<u32, usize>,
	/// The identifier of the `DocumentContainer`, where the presentation starts.
	document_id: u32,
}

impl<'a> Presentation<'a> {
	/// Reads the persist directory of the current save, or `None` if the file does not carry the
	/// records that lead to one.
	pub(super) fn read(stream: &'a [u8], current_user: &[u8]) -> Option<Self> {
		let mut at = read_u32(current_user, OFFSET_TO_CURRENT_EDIT_AT)? as usize;
		let mut directories = Vec::new();
		let mut document_id = None;

		for _ in 0..MAX_USER_EDITS {
			if read_u16(stream, at + 2)? != RT_USER_EDIT_ATOM {
				return None;
			}
			// The newest save is the one that says where the presentation begins.
			document_id.get_or_insert(read_u32(stream, at + DOC_PERSIST_ID_REF_AT)?);
			directories.push(read_u32(stream, at + OFFSET_PERSIST_DIRECTORY_AT)? as usize);

			let last_edit = read_u32(stream, at + OFFSET_LAST_EDIT_AT)? as usize;
			if last_edit == 0 {
				break;
			}
			// A chain that goes forwards, or stands still, is a chain that never ends.
			if last_edit >= at {
				return None;
			}
			at = last_edit;
		}

		// Oldest save first, so that where two saves name the same object the newer one wins.
		let mut directory = BTreeMap::new();
		for offset in directories.into_iter().rev() {
			read_persist_directory(stream, offset, &mut directory);
		}

		let document_id = document_id?;
		(!directory.is_empty()).then_some(Self { stream, directory, document_id })
	}

	/// The object a persist identifier names, header and all.
	pub(super) fn object(&self, id: u32) -> Option<&'a [u8]> {
		record_at(self.stream, *self.directory.get(&id)?)
	}

	pub(super) fn document(&self) -> Option<&'a [u8]> {
		self.object(self.document_id)
	}

	/// Every object of the presentation whose own record is of `record_type`, in identifier
	/// order.
	pub(super) fn objects_of_type(&self, record_type: u16) -> Vec<(u32, &'a [u8])> {
		self.directory
			.iter()
			.filter_map(|(id, offset)| {
				if read_u16(self.stream, offset + 2)? != record_type {
					return None;
				}
				Some((*id, record_at(self.stream, *offset)?))
			})
			.collect()
	}
}

/// Adds one `PersistDirectoryAtom`'s runs of identifiers and offsets to `directory`.
/// <https://docs.microsoft.com/en-us/openspecs/office_file_formats/ms-ppt/d10a093d-860f-409c-b065-aeb24b830505>
fn read_persist_directory(stream: &[u8], at: usize, directory: &mut BTreeMap<u32, usize>) {
	if read_u16(stream, at + 2) != Some(RT_PERSIST_DIRECTORY_ATOM) {
		return;
	}
	let Some(length) = read_u32(stream, at + 4).map(|length| length as usize) else {
		return;
	};
	let body_start = at + RECORD_HEADER_SIZE;
	let body_end = body_start + length;
	if body_end > stream.len() {
		return;
	}

	let mut pos = body_start;
	while pos + 4 <= body_end {
		let Some(word) = read_u32(stream, pos) else { return };
		let first_id = word & 0x000F_FFFF;
		let count = word >> 20;
		for index in 0..count as usize {
			let Some(offset) = read_u32(stream, pos + 4 + index * 4) else { return };
			if (pos + 4 + index * 4 + 4) > body_end {
				return;
			}
			directory.insert(first_id + index as u32, offset as usize);
		}
		// A run of no identifiers still takes up its four bytes, so the walk moves on rather
		// than standing still.
		pos += 4 + count as usize * 4;
	}
}
