use rstest::rstest;

use super::*;

fn book() -> BookLocation {
	BookLocation { namespace: "org.qt-project.qtcore.680".into(), folder: "qtcore".into() }
}

#[rstest]
#[case("", "qobject.html", "/qobject.html")]
#[case("", "qobject.html#details", "/qobject.html#details")]
#[case("", "./qobject.html", "/qobject.html")]
#[case("index.html", "images/arrow.png", "/images/arrow.png")]
#[case("sub/page.html", "other.html", "/sub/other.html")]
#[case("sub/page.html", "../top.html#x", "/top.html#x")]
#[case("sub/page.html", "#here", "/sub/page.html#here")]
#[case("page.html", "my%20page.html", "/my page.html")]
#[case("page.html", "other.html?x=1#y", "/other.html#y")]
#[case("page.html", "qthelp://org.qt-project.qtcore.680/qtcore/qstring.html#arg", "/qstring.html#arg")]
#[case("page.html", "QTHELP://ORG.QT-PROJECT.QTCORE.680/qtcore/qstring.html", "/qstring.html")]
fn resolves_links_inside_the_book(#[case] current: &str, #[case] href: &str, #[case] expected: &str) {
	assert_eq!(book().resolve(current, href).into_reference(), expected);
}

#[rstest]
#[case("page.html", "https://doc.qt.io/qt-6/", "https://doc.qt.io/qt-6/")]
#[case("page.html", "mailto:someone@example.com", "mailto:someone@example.com")]
#[case(
	"page.html",
	"qthelp://org.qt-project.qtgui.680/qtgui/qwindow.html",
	"qthelp://org.qt-project.qtgui.680/qtgui/qwindow.html"
)]
#[case("page.html", "../qtgui/qwindow.html#x", "qthelp://org.qt-project.qtcore.680/qtgui/qwindow.html#x")]
#[case("page.html", "javascript:void(0)", "javascript:void(0)")]
fn leaves_links_out_of_the_book_as_addresses(#[case] current: &str, #[case] href: &str, #[case] expected: &str) {
	assert!(matches!(book().resolve(current, href), Target::External(_)));
	assert_eq!(book().resolve(current, href).into_reference(), expected);
}

#[test]
fn a_bare_fragment_in_the_contents_has_no_page() {
	assert!(matches!(book().resolve("", "#top"), Target::External(_)));
}

#[test]
fn pictures_resolve_to_their_file_names() {
	assert_eq!(
		book().resolve("sub/page.html", "../images/a.png"),
		Target::Internal { name: "images/a.png".into(), fragment: None }
	);
}
