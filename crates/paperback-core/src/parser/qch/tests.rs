use std::path::Path;

use diesel::{
	prelude::*,
	sql_query,
	sql_types::{BigInt, Binary, Text},
	sqlite::SqliteConnection,
};

use super::{QchParser, compress::q_compress, file_bytes, stream::write_contents};
use crate::{
	document::{MarkerType, ParserContext, TocItem},
	parser::Parser,
	util::test_support::TempDir,
};

const PIXEL: &[u8] = b"\x89PNG\r\n\x1a\n not really a picture";

/// As `HelpGeneratorPrivate::createTables` writes them, with `IndexTable` as files from before Qt 4.6 had it.
fn create_tables(connection: &mut SqliteConnection, legacy: bool) {
	let identifier = if legacy { "ContextName" } else { "Identifier" };
	let mut statements = vec![
		"CREATE TABLE NamespaceTable (Id INTEGER PRIMARY KEY,Name TEXT )".to_string(),
		"CREATE TABLE FilterAttributeTable (Id INTEGER PRIMARY KEY, Name TEXT )".into(),
		"CREATE TABLE FilterNameTable (Id INTEGER PRIMARY KEY, Name TEXT )".into(),
		"CREATE TABLE FilterTable (NameId INTEGER, FilterAttributeId INTEGER )".into(),
		format!(
			"CREATE TABLE IndexTable (Id INTEGER PRIMARY KEY, Name TEXT, {identifier} TEXT, NamespaceId INTEGER, FileId INTEGER, Anchor TEXT )"
		),
		"CREATE TABLE IndexFilterTable (FilterAttributeId INTEGER, IndexId INTEGER )".into(),
		"CREATE TABLE ContentsTable (Id INTEGER PRIMARY KEY, NamespaceId INTEGER, Data BLOB )".into(),
		"CREATE TABLE ContentsFilterTable (FilterAttributeId INTEGER, ContentsId INTEGER )".into(),
		"CREATE TABLE FileAttributeSetTable (Id INTEGER, FilterAttributeId INTEGER )".into(),
		"CREATE TABLE FileDataTable (Id INTEGER PRIMARY KEY, Data BLOB )".into(),
		"CREATE TABLE FileFilterTable (FilterAttributeId INTEGER, FileId INTEGER )".into(),
		"CREATE TABLE FileNameTable (FolderId INTEGER, Name TEXT, FileId INTEGER, Title TEXT )".into(),
		"CREATE TABLE FolderTable(Id INTEGER PRIMARY KEY, Name Text, NamespaceID INTEGER )".into(),
	];
	if !legacy {
		statements.push("CREATE TABLE MetaDataTable(Name Text, Value BLOB )".into());
	}
	for statement in statements {
		sql_query(statement).execute(connection).unwrap();
	}
}

struct Project<'a> {
	legacy: bool,
	namespace: &'a str,
	folder: &'a str,
	files: &'a [(&'a str, &'a [u8])],
	contents: &'a [&'a [(i32, &'a str, &'a str)]],
	metadata: &'a [(&'a str, &'a str)],
}

/// What `qhelpgenerator` does with a project, in its order: the missing-file page first, then the folder, the files and the contents.
fn write_qch(dir: &TempDir, file_name: &str, project: &Project) -> String {
	let path = dir.path().join(file_name);
	let mut connection = SqliteConnection::establish(path.to_str().unwrap()).unwrap();
	create_tables(&mut connection, project.legacy);
	sql_query("INSERT INTO FileDataTable VALUES (NULL, ?)")
		.bind::<Binary, _>(Vec::<u8>::new())
		.execute(&mut connection)
		.unwrap();
	sql_query("INSERT INTO FileNameTable (FolderId, Name, FileId, Title) VALUES (0, '', 1, '')")
		.execute(&mut connection)
		.unwrap();
	for (name, value) in project.metadata {
		sql_query("INSERT INTO MetaDataTable VALUES(?, ?)")
			.bind::<Text, _>(name)
			.bind::<Text, _>(value)
			.execute(&mut connection)
			.unwrap();
	}
	sql_query("INSERT INTO NamespaceTable VALUES(NULL, ?)")
		.bind::<Text, _>(project.namespace)
		.execute(&mut connection)
		.unwrap();
	sql_query("INSERT INTO FolderTable (NamespaceId, Name) VALUES (1, ?)")
		.bind::<Text, _>(project.folder)
		.execute(&mut connection)
		.unwrap();
	for (index, (name, data)) in project.files.iter().enumerate() {
		let file_id = i64::try_from(index).unwrap() + 2;
		sql_query("INSERT INTO FileDataTable VALUES (Null, ?)")
			.bind::<Binary, _>(q_compress(data))
			.execute(&mut connection)
			.unwrap();
		sql_query("INSERT INTO FileNameTable (FolderId, Name, FileId, Title) VALUES (1, ?, ?, ?)")
			.bind::<Text, _>(name)
			.bind::<BigInt, _>(file_id)
			.bind::<Text, _>(name)
			.execute(&mut connection)
			.unwrap();
	}
	for entries in project.contents {
		sql_query("INSERT INTO ContentsTable (NamespaceId, Data) VALUES(1, ?)")
			.bind::<Binary, _>(write_contents(entries))
			.execute(&mut connection)
			.unwrap();
	}
	path.to_string_lossy().into_owned()
}

const INDEX: &[u8] = br#"<html><head><title>Widgets Manual</title></head><body><h1>Widgets Manual</h1><p>Start with <a href="sub/button.html#clicked">the clicked signal</a>, then <a href="qthelp://org.example.widgets.10/widgets/extra.html">the extra page</a> or <a href="../other/x.html">another book</a>.</p><p><img src="images/pic.png" alt="A picture"></p></body></html>"#;
const BUTTON: &[u8] = br#"<html><head><title>Button</title></head><body><h1>Button</h1><p>Intro.</p><h2 id="clicked">clicked()</h2><p>Emitted when <a href="../index.html">the manual</a> says so.</p></body></html>"#;
const EXTRA: &[u8] = b"<html><body><p>Extra page that the contents leave out.</p></body></html>";

fn widgets_project() -> Project<'static> {
	Project {
		legacy: false,
		namespace: "org.example.widgets.10",
		folder: "widgets",
		files: &[
			("extra.html", EXTRA),
			("index.html", INDEX),
			("sub/button.html", BUTTON),
			("images/pic.png", PIXEL),
			("style.css", b"p {}"),
		],
		contents: &[
			&[
				(0, "index.html", "Widgets Manual"),
				(1, "sub/button.html", "Button"),
				(2, "sub/button.html#clicked", "clicked()"),
			],
			&[
				(0, "index.html", "Widgets Manual"),
				(1, "sub/button.html", "Button"),
				(2, "sub/button.html#clicked", "clicked()"),
			],
		],
		metadata: &[("qchVersion", "1.0"), ("author", "The Widget Team")],
	}
}

fn text_at(text: &str, offset: usize, length: usize) -> String {
	text.chars().skip(offset).take(length).collect()
}

fn flatten(items: &[TocItem]) -> Vec<&TocItem> {
	items.iter().flat_map(|item| std::iter::once(item).chain(flatten(&item.children))).collect()
}

#[test]
fn reads_a_book_as_qt_assistant_shows_it() {
	let dir = TempDir::new("qch-book");
	let path = write_qch(&dir, "widgets.qch", &widgets_project());
	let document = QchParser.parse(&ParserContext::new(path)).expect("read the book");
	assert_eq!(document.title, "Widgets Manual");
	assert_eq!(document.author, "The Widget Team");
	let text = document.buffer.content.clone();
	// The contents order the pages, the page they leave out follows, and the stylesheet and picture are no pages.
	let manual = text.find("Widgets Manual").unwrap();
	let button = text.find("Button").unwrap();
	let extra = text.find("Extra page").unwrap();
	assert!(manual < button && button < extra, "{text}");
	assert_eq!(document.buffer.markers.iter().filter(|m| m.mtype == MarkerType::SectionBreak).count(), 3);
	// Two filter sections with the same contents are one table of contents.
	assert_eq!(document.toc_items.len(), 1);
	let entries = flatten(&document.toc_items);
	assert_eq!(
		entries.iter().map(|item| item.name.as_str()).collect::<Vec<_>>(),
		["Widgets Manual", "Button", "clicked()"]
	);
	let clicked = entries[2];
	assert_eq!(clicked.reference, "/sub/button.html#clicked");
	assert!(text_at(&text, clicked.offset.get(), 9).starts_with("clicked()"));
}

#[test]
fn resolves_links_and_pictures_against_the_book() {
	let dir = TempDir::new("qch-links");
	let path = write_qch(&dir, "widgets.qch", &widgets_project());
	let document = QchParser.parse(&ParserContext::new(path.clone())).expect("read the book");
	let links: Vec<&str> =
		document.buffer.markers.iter().filter(|m| m.mtype == MarkerType::Link).map(|m| m.reference.as_str()).collect();
	assert!(links.contains(&"/sub/button.html#clicked"), "{links:?}");
	assert!(links.contains(&"/extra.html"), "{links:?}");
	assert!(links.contains(&"/index.html"), "{links:?}");
	assert!(links.contains(&"qthelp://org.example.widgets.10/other/x.html"), "{links:?}");
	assert!(document.id_positions.contains_key("/sub/button.html#clicked"));
	assert!(document.id_positions.contains_key("/extra.html"));
	let picture = document.buffer.markers.iter().find(|m| m.mtype == MarkerType::Image).expect("an image marker");
	assert_eq!(picture.reference, "images/pic.png");
	assert_eq!(file_bytes(&path, &picture.reference).unwrap(), PIXEL);
	assert!(file_bytes(&path, "images/missing.png").is_err());
}

#[test]
fn reads_a_file_from_before_qt_4_4() {
	let dir = TempDir::new("qch-legacy");
	let project = Project {
		legacy: true,
		namespace: "trolltech.com.3_3_8.qmake",
		folder: "qdoc",
		files: &[("qmake-manual.html", b"<html><body><h1>qmake Manual</h1><p>Text.</p></body></html>")],
		contents: &[&[(0, "qmake-manual.html", "qmake Manual")]],
		metadata: &[],
	};
	let path = write_qch(&dir, "qmake.qch", &project);
	let document = QchParser.parse(&ParserContext::new(path)).expect("read the old book");
	assert_eq!(document.title, "qmake Manual");
	assert!(document.author.is_empty());
	assert!(document.buffer.content.contains("Text."));
}

#[test]
fn a_book_without_contents_is_named_by_its_file() {
	let dir = TempDir::new("qch-no-contents");
	let project = Project {
		legacy: false,
		namespace: "org.example.bare",
		folder: "bare",
		files: &[("a.html", b"<p>Only page.</p>")],
		contents: &[],
		metadata: &[],
	};
	let path = write_qch(&dir, "bare-book.qch", &project);
	let document = QchParser.parse(&ParserContext::new(path)).expect("read the book");
	assert_eq!(document.title, "bare-book");
	assert!(document.buffer.content.contains("Only page."));
}

#[test]
fn turns_away_what_is_not_a_qt_help_file() {
	let dir = TempDir::new("qch-not");
	let other = dir.path().join("other.qch");
	let mut connection = SqliteConnection::establish(other.to_str().unwrap()).unwrap();
	sql_query("CREATE TABLE Notes (Id INTEGER PRIMARY KEY)").execute(&mut connection).unwrap();
	drop(connection);
	assert!(QchParser.parse(&ParserContext::new(other.to_string_lossy().into_owned())).is_err());
	let garbage = dir.write("garbage.qch", b"this is not an SQLite database at all, not even close");
	assert!(QchParser.parse(&ParserContext::new(garbage.to_string_lossy().into_owned())).is_err());
	let missing = Path::new(dir.path()).join("missing.qch");
	assert!(QchParser.parse(&ParserContext::new(missing.to_string_lossy().into_owned())).is_err());
}
