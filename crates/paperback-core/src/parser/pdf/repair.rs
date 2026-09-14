//! Giving a tagged PDF back the parent tree it was exported without.
//!
//! pdfium reaches a page's structure elements through the tree root's `/ParentTree` and the
//! page's `/StructParents` key: without both it leaves every top-level element unloadable, and
//! the tagged path in [`super::structure`] has nothing to walk even though the file carries a
//! full set of `H1`/`H2`/`L`/`LI` elements. A PDF exported from Apple Pages is written that way.
//! It names a `/ParentTree` and never writes the object it names, and none of its pages carry a
//! `/StructParents`, so both ends of the link are missing.
//!
//! The repair appends an incremental update that writes the parent tree and numbers the pages
//! into it. pdfium only uses the tree to seed its walk, climbing each seed element's `/P` chain
//! to the root, so a page's entry needs nothing more than the elements that name that page in
//! their `/Pg`, in any order. Nothing already in the file is rewritten in place: the update
//! carries new copies of the objects it changes, and the original bytes stay where they are.
//!
//! Only a file laid out plainly is repaired: a classic cross-reference table, objects written
//! out one by one rather than packed into object streams, and no encryption. [`repaired_bytes`]
//! returns `None` for anything else, and the caller then reads the file as it stands.

use std::{
	collections::{HashMap, HashSet},
	fs,
};

/// Largest file worth reading into memory to repair. A tagged PDF that leaves out its parent
/// tree is a word processor's export, not a scanned volume, so the cap only skips files whose
/// structure tree was never going to be the useful part.
const MAX_REPAIR_BYTES: u64 = 256 * 1024 * 1024;

/// One indirect object as found by [`scan_objects`]: its number and the span of its body,
/// between the `obj` keyword and the `endobj` that closes it.
struct RawObject {
	number: u32,
	start: usize,
	end: usize,
}

/// One page of the document, the structure elements that name it, and the parent tree key it
/// answers to.
struct PageEntry<'a> {
	object: &'a RawObject,
	elements: Vec<u32>,
	key: i64,
	/// A page that already carries a `/StructParents` keeps the key it names and is left alone;
	/// only a page given a new key needs a rewritten copy in the update.
	rewrite: bool,
}

/// Reads `path` and returns it with a parent tree appended, or `None` when the file needs no
/// repair or is not one this can repair safely.
pub(super) fn repaired_bytes(path: &str) -> Option<Vec<u8>> {
	let size = fs::metadata(path).ok()?.len();
	if size > MAX_REPAIR_BYTES {
		tracing::warn!(path, size, "pdf structure tree is unreachable but the file is too large to repair");
		return None;
	}
	let bytes = fs::read(path).ok()?;
	rebuild(&bytes)
}

/// The whole repair, over the bytes of a file already read in. Split from [`repaired_bytes`] so
/// it can be tested without one.
fn rebuild(bytes: &[u8]) -> Option<Vec<u8>> {
	// Objects packed into an object stream are not written out one by one, so the scan below
	// cannot see them and would build the tree out of whatever half of the file it can read.
	if find_token(bytes, b"/ObjStm").is_some() {
		return None;
	}
	let trailer = last_trailer(bytes)?;
	// An encrypted file's strings and streams are wrapped in a way this does not undo, and a
	// mismatched update would leave pdfium worse off than the unreachable tree it already has.
	if find_token(trailer, b"/Encrypt").is_some() {
		return None;
	}
	let root = ref_value(trailer, b"/Root")?;
	let previous_xref = last_startxref(bytes)?;
	let objects = scan_objects(bytes);
	let tree_root = objects.iter().find_map(|object| ref_value(body(bytes, object), b"/StructTreeRoot"))?;
	let tree_root = objects.iter().find(|object| object.number == tree_root)?;
	let (parent_tree_number, rewrite_tree_root) = parent_tree_slot(bytes, &objects, tree_root)?;
	let mut pages = collect_pages(bytes, &objects);
	pages.retain(|page| !page.elements.is_empty());
	if pages.is_empty() {
		return None;
	}
	// A number tree's keys have to run in order for a lookup to find them.
	pages.sort_unstable_by_key(|page| page.key);
	let mut out = bytes.to_vec();
	if !out.ends_with(b"\n") {
		out.push(b'\n');
	}
	// Object number and offset of everything the update adds, for its cross-reference section.
	let mut added: Vec<(u32, usize)> = Vec::new();
	let mut nums = String::new();
	for page in &pages {
		let elements: String = page.elements.iter().map(|element| format!(" {element} 0 R")).collect();
		nums.push_str(&format!(" {} [{elements} ]", page.key));
		if !page.rewrite {
			continue;
		}
		added.push((page.object.number, out.len()));
		out.extend_from_slice(format!("{} 0 obj\n", page.object.number).as_bytes());
		out.extend_from_slice(&with_entry(body(bytes, page.object), &format!("/StructParents {}", page.key))?);
		out.extend_from_slice(b"\nendobj\n");
	}
	if rewrite_tree_root {
		added.push((tree_root.number, out.len()));
		out.extend_from_slice(format!("{} 0 obj\n", tree_root.number).as_bytes());
		out.extend_from_slice(&with_entry(body(bytes, tree_root), &format!("/ParentTree {parent_tree_number} 0 R"))?);
		out.extend_from_slice(b"\nendobj\n");
	}
	added.push((parent_tree_number, out.len()));
	out.extend_from_slice(format!("{parent_tree_number} 0 obj\n<< /Nums [{nums} ] >>\nendobj\n").as_bytes());
	let xref_offset = out.len();
	added.sort_unstable();
	// One subsection per object, which keeps the section right whatever the numbers are, and a
	// free head so the section reads like any other. Every entry is twenty bytes long.
	out.extend_from_slice(b"xref\n0 1\n0000000000 65535 f \n");
	for (number, offset) in &added {
		out.extend_from_slice(format!("{number} 1\n{offset:010} 00000 n \n").as_bytes());
	}
	let highest = objects.iter().map(|object| object.number).max().unwrap_or(0).max(parent_tree_number);
	let size = int_value(trailer, b"/Size").unwrap_or(0).max(i64::from(highest) + 1);
	let trailer = format!(
		"trailer\n<< /Size {size} /Root {root} 0 R /Prev {previous_xref} >>\nstartxref\n{xref_offset}\n%%EOF\n"
	);
	out.extend_from_slice(trailer.as_bytes());
	Some(out)
}

/// The object number to write the parent tree as, and whether the tree root has to be rewritten
/// to point at it.
///
/// A tree root that names a `/ParentTree` already may be naming an object the file never wrote
/// out, which is what an Apple Pages export does and what pdfium reads as no parent tree at all.
/// The number it names is then filled in rather than replaced, and the tree root is left exactly
/// as it is. A root naming an object the file does write has a parent tree, and nothing here can
/// improve on it.
fn parent_tree_slot(bytes: &[u8], objects: &[RawObject], tree_root: &RawObject) -> Option<(u32, bool)> {
	let root_body = body(bytes, tree_root);
	if find_token(root_body, b"/ParentTree").is_none() {
		let free = objects.iter().map(|object| object.number).max().unwrap_or(0) + 1;
		return Some((free, true));
	}
	let named = ref_value(root_body, b"/ParentTree")?;
	if objects.iter().any(|object| object.number == named) {
		return None;
	}
	Some((named, false))
}

/// Gathers every page of the file with the structure elements that name it in their `/Pg`. An
/// element with no `/Pg` of its own (a list, a list item, the document element itself) is left
/// out: pdfium reaches it by climbing from the elements below it.
fn collect_pages<'a>(bytes: &[u8], objects: &'a [RawObject]) -> Vec<PageEntry<'a>> {
	let mut pages: Vec<PageEntry> = objects
		.iter()
		.filter(|object| name_value(body(bytes, object), b"/Type") == Some(b"Page".as_slice()))
		.map(|object| PageEntry { object, elements: Vec::new(), key: -1, rewrite: true })
		.collect();
	let index: HashMap<u32, usize> =
		pages.iter().enumerate().map(|(index, page)| (page.object.number, index)).collect();
	for object in objects {
		let element = body(bytes, object);
		if name_value(element, b"/Type") != Some(b"StructElem".as_slice()) {
			continue;
		}
		if let Some(page) = ref_value(element, b"/Pg")
			&& let Some(index) = index.get(&page)
		{
			pages[*index].elements.push(object.number);
		}
	}
	// Keys already in the file are kept, so a page the exporter did number keeps answering to
	// the number it names. Fresh keys start past every one of them.
	let mut next_key = 0;
	for page in &mut pages {
		if let Some(existing) = int_value(body(bytes, page.object), b"/StructParents") {
			page.key = existing;
			page.rewrite = false;
			next_key = next_key.max(existing + 1);
		}
	}
	for page in &mut pages {
		if page.rewrite {
			page.key = next_key;
			next_key += 1;
		}
	}
	pages
}

fn body<'a>(bytes: &'a [u8], object: &RawObject) -> &'a [u8] {
	&bytes[object.start..object.end]
}

/// Copies an object's dictionary with one more entry in it, placed just inside the opening `<<`.
/// The whitespace the original body is padded with is left behind. Returns `None` for a body
/// that does not open with a dictionary.
fn with_entry(body: &[u8], entry: &str) -> Option<Vec<u8>> {
	let start = body.iter().position(|byte| !is_whitespace(*byte))?;
	let end = body.iter().rposition(|byte| !is_whitespace(*byte))? + 1;
	let dictionary = &body[start..end];
	if !dictionary.starts_with(b"<<") {
		return None;
	}
	let mut out = Vec::with_capacity(dictionary.len() + entry.len() + 1);
	out.extend_from_slice(b"<< ");
	out.extend_from_slice(entry.as_bytes());
	out.extend_from_slice(&dictionary[2..]);
	Some(out)
}

/// Walks the file for `N G obj ... endobj` spans. The walk steps over stream data rather than
/// searching it, so bytes that happen to spell an object header inside a compressed stream are
/// never taken for one.
fn scan_objects(bytes: &[u8]) -> Vec<RawObject> {
	let mut objects = Vec::new();
	let mut pos = 0;
	while let Some(keyword) = find_from(bytes, b"obj", pos) {
		pos = keyword + b"obj".len();
		if keyword == 0 || !is_whitespace(bytes[keyword - 1]) || !is_delimiter(bytes.get(pos).copied()) {
			continue;
		}
		let Some(number) = object_number_before(bytes, keyword) else { continue };
		let Some(end) = object_end(bytes, pos) else { break };
		objects.push(RawObject { number, start: pos, end });
		pos = end + b"endobj".len();
	}
	// A file that has been added to since it was written names some of its objects twice. The
	// definition nearest the end of the file is the one in force, so it is the one kept.
	let mut seen = HashSet::new();
	let mut newest: Vec<RawObject> = objects.into_iter().rev().filter(|object| seen.insert(object.number)).collect();
	newest.reverse();
	newest
}

/// Reads the `N G` pair that stands in front of an `obj` keyword, backwards from it.
fn object_number_before(bytes: &[u8], keyword: usize) -> Option<u32> {
	let generation_end = skip_whitespace_back(bytes, keyword);
	let generation_start = skip_digits_back(bytes, generation_end);
	if generation_start == generation_end {
		return None;
	}
	let number_end = skip_whitespace_back(bytes, generation_start);
	if number_end == generation_start {
		return None;
	}
	let number_start = skip_digits_back(bytes, number_end);
	if number_start == number_end {
		return None;
	}
	std::str::from_utf8(&bytes[number_start..number_end]).ok()?.parse().ok()
}

/// Finds the `endobj` that closes the object whose body starts at `pos`, stepping over any
/// stream the object carries. A stream whose dictionary states its own length is stepped over by
/// that length and never searched: compressed data spells `endstream` and `endobj` often enough
/// to matter.
fn object_end(bytes: &[u8], pos: usize) -> Option<usize> {
	let mut at = pos;
	loop {
		let end = find_from(bytes, b"endobj", at)?;
		let Some(stream) = find_token_from(bytes, b"stream", at) else { return Some(end) };
		if stream > end {
			return Some(end);
		}
		let data = stream_data_start(bytes, stream);
		let dictionary = &bytes[at..stream];
		// A `/Length` that is itself an indirect reference says nothing here, and the data has to
		// be searched for its end after all.
		let stated = match ref_value(dictionary, b"/Length") {
			Some(_) => None,
			None => int_value(dictionary, b"/Length").and_then(|length| usize::try_from(length).ok()),
		};
		let from = match stated {
			Some(length) if data.saturating_add(length) <= bytes.len() => data + length,
			_ => data,
		};
		// A body with no `endstream` after it never held a stream: the word was part of a name or
		// a string, and the `endobj` already found is the right one.
		let Some(endstream) = find_from(bytes, b"endstream", from) else { return Some(end) };
		at = endstream + b"endstream".len();
	}
}

/// The first byte of a stream's data, past the end-of-line that follows the `stream` keyword.
fn stream_data_start(bytes: &[u8], stream: usize) -> usize {
	let mut at = stream + b"stream".len();
	if bytes.get(at) == Some(&b'\r') {
		at += 1;
	}
	if bytes.get(at) == Some(&b'\n') {
		at += 1;
	}
	at
}

/// Everything from the file's last `trailer` keyword on, which is where its newest trailer
/// dictionary sits.
fn last_trailer(bytes: &[u8]) -> Option<&[u8]> {
	find_last(bytes, b"trailer").map(|keyword| &bytes[keyword..])
}

/// The offset the file's last `startxref` names.
fn last_startxref(bytes: &[u8]) -> Option<u64> {
	let mut at = find_last(bytes, b"startxref")? + b"startxref".len();
	let value = read_integer(bytes, &mut at)?;
	u64::try_from(value).ok()
}

/// The name a key is set to, as in the `Page` of `/Type /Page`.
fn name_value<'a>(body: &'a [u8], key: &[u8]) -> Option<&'a [u8]> {
	let mut at = find_token(body, key)? + key.len();
	while at < body.len() && is_whitespace(body[at]) {
		at += 1;
	}
	if body.get(at) != Some(&b'/') {
		return None;
	}
	at += 1;
	let start = at;
	while at < body.len() && !is_delimiter(Some(body[at])) {
		at += 1;
	}
	Some(&body[start..at])
}

/// The object number a key refers to, as in the `4` of `/Pg 4 0 R`.
fn ref_value(body: &[u8], key: &[u8]) -> Option<u32> {
	let mut at = find_token(body, key)? + key.len();
	let number = read_integer(body, &mut at)?;
	read_integer(body, &mut at)?;
	while at < body.len() && is_whitespace(body[at]) {
		at += 1;
	}
	if body.get(at) != Some(&b'R') {
		return None;
	}
	u32::try_from(number).ok()
}

/// The number a key is set to, as in the `0` of `/StructParents 0`.
fn int_value(body: &[u8], key: &[u8]) -> Option<i64> {
	let mut at = find_token(body, key)? + key.len();
	read_integer(body, &mut at)
}

fn read_integer(body: &[u8], at: &mut usize) -> Option<i64> {
	while *at < body.len() && is_whitespace(body[*at]) {
		*at += 1;
	}
	let start = *at;
	while *at < body.len() && body[*at].is_ascii_digit() {
		*at += 1;
	}
	if start == *at {
		return None;
	}
	std::str::from_utf8(&body[start..*at]).ok()?.parse().ok()
}

/// Finds a key only where it stands as a whole token, so `/Page` never matches inside `/Pages`
/// and `stream` never matches inside `endstream`.
fn find_token(body: &[u8], key: &[u8]) -> Option<usize> {
	find_token_from(body, key, 0)
}

fn find_token_from(body: &[u8], key: &[u8], from: usize) -> Option<usize> {
	let mut at = from;
	while let Some(found) = find_from(body, key, at) {
		at = found + 1;
		let after = is_delimiter(body.get(found + key.len()).copied());
		let before = found == 0 || is_delimiter(Some(body[found - 1]));
		if before && after {
			return Some(found);
		}
	}
	None
}

fn find_from(haystack: &[u8], needle: &[u8], from: usize) -> Option<usize> {
	if from >= haystack.len() || needle.len() > haystack.len() - from {
		return None;
	}
	haystack[from..].windows(needle.len()).position(|window| window == needle).map(|found| found + from)
}

fn find_last(haystack: &[u8], needle: &[u8]) -> Option<usize> {
	if needle.len() > haystack.len() {
		return None;
	}
	haystack.windows(needle.len()).rposition(|window| window == needle)
}

fn skip_whitespace_back(bytes: &[u8], mut at: usize) -> usize {
	while at > 0 && is_whitespace(bytes[at - 1]) {
		at -= 1;
	}
	at
}

fn skip_digits_back(bytes: &[u8], mut at: usize) -> usize {
	while at > 0 && bytes[at - 1].is_ascii_digit() {
		at -= 1;
	}
	at
}

const fn is_whitespace(byte: u8) -> bool {
	matches!(byte, b' ' | b'\t' | b'\r' | b'\n' | b'\x0c' | b'\0')
}

/// True where a name or keyword ends: PDF's whitespace and delimiter characters, and the end of
/// the buffer.
const fn is_delimiter(byte: Option<u8>) -> bool {
	match byte {
		None => true,
		Some(byte) => {
			is_whitespace(byte) || matches!(byte, b'(' | b')' | b'<' | b'>' | b'[' | b']' | b'{' | b'}' | b'/' | b'%')
		}
	}
}

#[cfg(test)]
mod tests;
