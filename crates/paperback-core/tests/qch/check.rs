//! What any Qt help file Qt made should read as: a title and contents, every picture there, and every link Paperback leaves unresolved one that Qt Assistant can't follow either.

use std::collections::{HashMap, HashSet};

use paperback_core::{
	document::{DocPos, Document, Marker, MarkerType, ParseSettings, TocItem},
	parser::qch::file_bytes,
	session::{DocumentSession, LinkAction},
	util::text::url_decode,
};
use rusqlite::Connection;

/// Where the parser leaves a contents entry whose page the book doesn't have.
const UNRESOLVED: usize = usize::MAX;

/// What one book holds that another's links are checked against, kept rather than the document so a whole documentation set fits in memory.
pub struct Book {
	pub title: String,
	pub folder: String,
	pub report: Report,
	/// The parser's `id_positions` keys: a page's `/name`, and `/name#id` for each id on it.
	pub targets: HashSet<String>,
	/// `qthelp://` links into other books, as the parser left them.
	pub foreign_links: Vec<String>,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Report {
	pub contents_entries: usize,
	/// Entries whose page Qt's file itself lacks.
	pub contents_missing_from_qt: usize,
	pub pictures: usize,
	/// Pictures a page shows that the file has under no name at all.
	pub pictures_missing_from_qt: usize,
	pub links_in_book: usize,
	/// Links to pages the file lacks, or to ids its page doesn't have.
	pub links_broken_in_qt: usize,
	pub links_to_other_books: usize,
}

fn query_name(connection: &Connection, query: &str) -> String {
	connection.query_row(query, [], |row| row.get(0)).unwrap_or_else(|err| panic!("{query}: {err}"))
}

fn flatten(items: &[TocItem]) -> Vec<&TocItem> {
	items.iter().flat_map(|item| std::iter::once(item).chain(flatten(&item.children))).collect()
}

/// The page's HTML, from the file rather than the parser, so a check doesn't take the parser's word for what Qt wrote.
struct Pages<'a> {
	path: &'a str,
	html: HashMap<String, Option<String>>,
}

impl Pages<'_> {
	fn html(&mut self, name: &str) -> Option<&str> {
		self.html
			.entry(name.to_string())
			.or_insert_with(|| {
				file_bytes(self.path, name)
					.ok()
					.filter(|bytes| !bytes.is_empty())
					.map(|bytes| String::from_utf8_lossy(&bytes).into_owned())
			})
			.as_deref()
	}

	fn has_id(&mut self, name: &str, id: &str) -> bool {
		let decoded = url_decode(id);
		self.html(name).is_some_and(|html| {
			[id, decoded.as_str()].iter().any(|id| {
				[format!("id=\"{id}\""), format!("name=\"{id}\""), format!("id='{id}'"), format!("name='{id}'")]
					.iter()
					.any(|attribute| html.contains(attribute.as_str()))
			})
		})
	}
}

/// Panics at anything Paperback should have read and didn't; what Qt's own file gets wrong is only counted.
pub fn read_book(path: &str) -> Book {
	// Opened as the app opens it, so a link is followed as pressing Enter on it does.
	let session =
		DocumentSession::new(path, "", "", ParseSettings::default()).unwrap_or_else(|err| panic!("{path}: {err}"));
	let document = session.handle().document();
	let connection = Connection::open(path).unwrap();
	let namespace = query_name(&connection, "SELECT Name AS name FROM NamespaceTable LIMIT 1");
	let folder = query_name(&connection, "SELECT Name AS name FROM FolderTable WHERE Id = 1");
	assert!(!document.title.trim().is_empty(), "{path}: no title");
	assert!(!document.toc_items.is_empty(), "{path}: no contents");
	let mut pages = Pages { path, html: HashMap::new() };
	let mut report = Report::default();
	check_contents(path, document, &mut pages, &mut report);
	check_pictures(path, document, &connection, &mut report);
	let foreign_links = check_links(path, &session, &namespace, &folder, &mut pages, &mut report);
	Book {
		title: document.title.clone(),
		folder,
		report,
		targets: document.id_positions.keys().cloned().collect(),
		foreign_links,
	}
}

/// Where following a link lands: on its id, or at the start of its page when the page has no such id, as a browser has it.
fn check_landing(path: &str, session: &DocumentSession, marker: &Marker, expected: DocPos) {
	let result = session.activate_link(marker.position);
	assert!(
		result.found && result.action == LinkAction::Internal && result.offset == expected,
		"{path}: following {:?} lands at {} ({:?}), not at {}",
		marker.reference,
		result.offset.get(),
		result.action,
		expected.get()
	);
}

fn check_contents(path: &str, document: &Document, pages: &mut Pages, report: &mut Report) {
	let length = document.buffer.content.chars().count();
	for entry in flatten(&document.toc_items) {
		report.contents_entries += 1;
		let offset = entry.offset.get();
		if offset != UNRESOLVED {
			assert!(offset <= length, "{path}: contents entry {:?} lands past the end", entry.name);
			continue;
		}
		// An entry Qt gives no link is a heading for the ones under it.
		if entry.reference.is_empty() {
			continue;
		}
		let page = entry.reference.split_once('#').map_or(entry.reference.as_str(), |(page, _)| page);
		let name = page
			.strip_prefix('/')
			.unwrap_or_else(|| panic!("{path}: contents entry {:?} points outside the book", entry.reference));
		assert!(
			pages.html(name).is_none(),
			"{path}: contents entry {:?} has a page but no place in the book",
			entry.reference
		);
		report.contents_missing_from_qt += 1;
	}
}

/// A picture the lookup can't find counts as Qt's only when no file of the book has its file name, in any folder; one that is there under another path is Paperback naming it wrongly.
fn check_pictures(path: &str, document: &Document, connection: &Connection, report: &mut Report) {
	for marker in document.buffer.markers.iter().filter(|marker| marker.mtype == MarkerType::Image) {
		report.pictures += 1;
		match file_bytes(path, &marker.reference) {
			Ok(bytes) => assert!(!bytes.is_empty(), "{path}: picture {:?} is empty", marker.reference),
			Err(err) => {
				let file_name = marker.reference.rsplit('/').next().unwrap_or_default();
				let pattern = format!("%/{}", file_name.replace('\\', "\\\\").replace('%', "\\%").replace('_', "\\_"));
				let found: i64 = connection
					.query_row(
						"SELECT COUNT(*) FROM FileNameTable WHERE Name = ?1 OR Name LIKE ?2 ESCAPE '\'",
						[file_name, pattern.as_str()],
						|row| row.get(0),
					)
					.unwrap();
				assert_eq!(
					found, 0,
					"{path}: picture {:?} is in the file under another name: {err:#}",
					marker.reference
				);
				report.pictures_missing_from_qt += 1;
			}
		}
	}
}

fn check_links(
	path: &str,
	session: &DocumentSession,
	namespace: &str,
	folder: &str,
	pages: &mut Pages,
	report: &mut Report,
) -> Vec<String> {
	let document = session.handle().document();
	let links: Vec<&Marker> =
		document.buffer.markers.iter().filter(|marker| marker.mtype == MarkerType::Link).collect();
	let mut at_position: HashMap<DocPos, usize> = HashMap::new();
	for link in &links {
		*at_position.entry(link.position).or_default() += 1;
	}
	// Enter can only reach a link that has text and is the only one where it starts.
	let reachable = |marker: &Marker| marker.link_len().get() > 0 && at_position[&marker.position] == 1;
	let mut foreign = Vec::new();
	for marker in links {
		let reference = marker.reference.as_str();
		if let Some(target) = reference.strip_prefix('/') {
			report.links_in_book += 1;
			if let Some(&expected) = document.id_positions.get(reference) {
				if reachable(marker) {
					check_landing(path, session, marker, expected);
				}
				continue;
			}
			report.links_broken_in_qt += 1;
			match target.split_once('#') {
				Some((name, id)) if document.id_positions.contains_key(&format!("/{name}")) => {
					assert!(
						!pages.has_id(name, id),
						"{path}: {reference} is on its page, but Paperback didn't find it"
					);
					if reachable(marker) {
						check_landing(path, session, marker, document.id_positions[&format!("/{name}")]);
					}
				}
				_ => {
					let name = target.split_once('#').map_or(target, |(name, _)| name);
					assert!(
						pages.html(name).is_none(),
						"{path}: {reference} is in the file, but Paperback didn't read it"
					);
				}
			}
		} else if let Some((link_namespace, link_folder, _)) = split_qthelp(reference) {
			assert!(
				!(link_namespace.eq_ignore_ascii_case(namespace) && link_folder == folder),
				"{path}: {reference} is in this book, but was left as a qthelp link"
			);
			report.links_to_other_books += 1;
			foreign.push(reference.to_string());
		}
	}
	foreign
}

/// `qthelp://namespace/folder/name#fragment` as its namespace, folder, and `/name#fragment`.
pub fn split_qthelp(url: &str) -> Option<(&str, &str, String)> {
	let rest = url.get(..9).filter(|scheme| scheme.eq_ignore_ascii_case("qthelp://")).map(|_| &url[9..])?;
	let (namespace, rest) = rest.split_once('/')?;
	let (folder, name) = rest.split_once('/')?;
	Some((namespace, folder, format!("/{name}")))
}
