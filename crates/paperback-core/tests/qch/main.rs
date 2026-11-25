//! Qt help files as Qt made them. The fixtures are qttools' own test files, under the GPL as `fixtures/qch/README.md` explains; `latest` downloads the newest Qt and CMake documentation and is opt-in.

mod check;
mod latest;

use check::{Report, read_book, split_qthelp};

fn fixture(name: &str) -> String {
	format!("{}/tests/fixtures/qch/{name}", env!("CARGO_MANIFEST_DIR"))
}

/// Qt's own test book, whose pages are empty: only its contents are something to read.
#[test]
fn qt_test_manual() {
	let book = read_book(&fixture("test.qch"));
	assert_eq!(book.title, "Test Manual");
	assert_eq!(
		book.report,
		Report { contents_entries: 11, ..Report::default() },
		"two manuals of contents, every entry landing on its page"
	);
}

/// From Qt 3.3.8, before `MetaDataTable`; its pages link to the rest of Qt's reference, which the file doesn't have.
#[test]
fn qmake_manual_from_qt_3() {
	let book = read_book(&fixture("qmake-3.3.8.qch"));
	assert_eq!(book.title, "qmake Manual");
	assert_eq!(book.report.contents_entries, 9);
	assert_eq!(book.report.contents_missing_from_qt, 0);
	assert_eq!(book.report.pictures, 9);
	assert_eq!(book.report.links_in_book, 267);
	assert_eq!(book.report.links_broken_in_qt, 74);
}

#[test]
fn linguist_manual_from_qt_3() {
	let book = read_book(&fixture("linguist-3.3.8.qch"));
	assert_eq!(book.title, "Qt Linguist Manual");
	assert_eq!(book.report.contents_entries, 5);
	assert_eq!(book.report.pictures, 41);
	assert_eq!(book.report.links_in_book, 144);
	assert_eq!(book.report.links_broken_in_qt, 57);
}

#[test]
fn qmake_manual_from_qt_4_3() {
	let book = read_book(&fixture("qmake-4.3.0.qch"));
	assert_eq!(book.title, "qmake Manual");
	assert_eq!(book.report.contents_entries, 13);
	assert_eq!(book.report.pictures, 26);
	assert_eq!(book.report.links_in_book, 916);
	assert_eq!(book.report.links_broken_in_qt, 127);
}

/// A module of Qt 6's documentation, which links into the other modules' books.
#[test]
fn qt_qml_module_from_qt_6() {
	let book = read_book(&fixture("qtqml.qch"));
	assert_eq!(book.title, "Qt Qml");
	assert_eq!(book.folder, "qtqml");
	assert_eq!(book.report.contents_entries, 89);
	assert_eq!(book.report.contents_missing_from_qt, 0);
	assert_eq!(book.report.links_in_book, 5361);
	// `qtqml-module.html` links to a `#namespaces` its page doesn't have.
	assert_eq!(book.report.links_broken_in_qt, 1);
	assert_eq!(book.report.links_to_other_books, 2206);
	let folders: std::collections::BTreeSet<&str> =
		book.foreign_links.iter().filter_map(|link| split_qthelp(link)).map(|(_, folder, _)| folder).collect();
	assert!(folders.contains("qtquick") && folders.contains("qtcore"), "links into other modules: {folders:?}");
}
