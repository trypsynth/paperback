//! A contents row is what `HelpGeneratorPrivate::writeTree` streams with `QDataStream`: for each entry, depth-first, a `qint32` depth, then the link and the title as `QString`s.

/// One entry as the stream holds it, before the depths become a tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ContentsEntry {
	pub depth: i32,
	pub link: String,
	pub title: String,
}

/// A `QString` length of all ones is a null string.
const NULL_STRING: u32 = 0xFFFF_FFFF;
/// Since Qt 6.7, a string too long for a `u32` length writes this, then a `u64` one.
const EXTENDED_SIZE: u32 = 0xFFFF_FFFE;

struct Reader<'a> {
	data: &'a [u8],
}

impl<'a> Reader<'a> {
	fn take(&mut self, count: usize) -> Option<&'a [u8]> {
		let (head, tail) = self.data.split_at_checked(count)?;
		self.data = tail;
		Some(head)
	}

	fn read_u32(&mut self) -> Option<u32> {
		self.take(4).map(|bytes| u32::from_be_bytes(bytes.try_into().unwrap()))
	}

	fn read_i32(&mut self) -> Option<i32> {
		self.take(4).map(|bytes| i32::from_be_bytes(bytes.try_into().unwrap()))
	}

	fn read_u64(&mut self) -> Option<u64> {
		self.take(8).map(|bytes| u64::from_be_bytes(bytes.try_into().unwrap()))
	}

	/// UTF-16 in big-endian order; an odd byte count is data `QDataStream` calls corrupt, so it ends the read.
	fn read_qstring(&mut self) -> Option<String> {
		let length = match self.read_u32()? {
			NULL_STRING => return Some(String::new()),
			EXTENDED_SIZE => usize::try_from(self.read_u64()?).ok()?,
			length => length as usize,
		};
		if length % 2 != 0 {
			return None;
		}
		let (pairs, _) = self.take(length)?.as_chunks::<2>();
		let units: Vec<u16> = pairs.iter().map(|pair| u16::from_be_bytes(*pair)).collect();
		Some(String::from_utf16_lossy(&units))
	}
}

/// Stops where Qt's reader does: at the end of the data, at anything it can't read, or at an entry with no title, which is what a read past the end gives Qt.
pub(super) fn read_contents(data: &[u8]) -> Vec<ContentsEntry> {
	let mut reader = Reader { data };
	let mut entries = Vec::new();
	let mut next = || {
		let depth = reader.read_i32()?;
		let link = reader.read_qstring()?;
		let title = reader.read_qstring()?;
		Some(ContentsEntry { depth, link, title })
	};
	while let Some(entry) = next() {
		if entry.title.is_empty() {
			break;
		}
		entries.push(entry);
	}
	entries
}

/// A node of the tree before its links are resolved against the book.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ContentsNode {
	pub link: String,
	pub title: String,
	pub children: Vec<Self>,
}

/// Depths come from the file, so they are repaired as Qt Assistant repairs them: a negative depth is 0, and an entry's parent is the nearest one before it with a lower depth, so a jump of several levels nests one level deeper and no empty levels are made up.
pub(super) fn build_tree(entries: Vec<ContentsEntry>) -> Vec<ContentsNode> {
	let mut roots: Vec<ContentsNode> = Vec::new();
	// Each ancestor of the next entry, with its depth and its index among its siblings.
	let mut stack: Vec<(i32, usize)> = Vec::new();
	for entry in entries {
		let depth = entry.depth.max(0);
		while stack.last().is_some_and(|&(ancestor_depth, _)| ancestor_depth >= depth) {
			stack.pop();
		}
		let mut siblings = &mut roots;
		for &(_, index) in &stack {
			siblings = &mut siblings[index].children;
		}
		siblings.push(ContentsNode { link: entry.link, title: entry.title, children: Vec::new() });
		stack.push((depth, siblings.len() - 1));
	}
	roots
}

#[cfg(test)]
pub(super) fn write_contents(entries: &[(i32, &str, &str)]) -> Vec<u8> {
	let mut data = Vec::new();
	for &(depth, link, title) in entries {
		data.extend_from_slice(&depth.to_be_bytes());
		for text in [link, title] {
			let units: Vec<u16> = text.encode_utf16().collect();
			data.extend_from_slice(&u32::try_from(units.len() * 2).unwrap().to_be_bytes());
			for unit in units {
				data.extend_from_slice(&unit.to_be_bytes());
			}
		}
	}
	data
}

#[cfg(test)]
mod tests;
