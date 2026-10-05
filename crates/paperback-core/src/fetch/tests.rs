use rstest::rstest;

use super::*;

const PAGE: &str = "https://example.org/download?id=3";

#[rstest]
#[case(
	"https://example.org/a.epub",
	"https://example.org/a.epub",
	None,
	Some("application/octet-stream"),
	Verdict::Pass
)]
#[case(PAGE, PAGE, None, Some("application/pdf"), Verdict::Pass)]
#[case(PAGE, PAGE, None, Some("Application/PDF; name=x"), Verdict::Pass)]
#[case(PAGE, PAGE, None, Some("application/octet-stream"), Verdict::Warn)]
#[case(PAGE, PAGE, None, None, Verdict::Warn)]
#[case("https://example.org/setup.exe", "https://example.org/setup.exe", None, Some("application/pdf"), Verdict::Refuse("exe".into()))]
#[case("https://example.org/book", "https://cdn.example.org/setup.exe", None, Some("application/epub+zip"), Verdict::Refuse("exe".into()))]
#[case("https://example.org/download.php?id=3", "https://example.org/download.php?id=3", None, Some("application/pdf"), Verdict::Refuse("php".into()))]
#[case(PAGE, PAGE, Some("setup.exe"), Some("application/pdf"), Verdict::Refuse("exe".into()))]
#[case(PAGE, PAGE, Some("Report.pdf"), Some("application/octet-stream"), Verdict::Pass)]
#[case("https://example.org/a.epub", "https://example.org/a.epub", Some("a.exe"), None, Verdict::Refuse("exe".into()))]
fn verdict_follows_the_extension_rules(
	#[case] given: &str,
	#[case] final_url: &str,
	#[case] disposition: Option<&str>,
	#[case] content_type: Option<&str>,
	#[case] expected: Verdict,
) {
	assert_eq!(verdict(given, final_url, disposition, content_type), expected);
}

#[rstest]
#[case("https://github.com/o/r/blob/main/book.pdf", None, Some("text/html; charset=utf-8"), Verdict::Warn)]
#[case("https://example.org/book.pdf", Some("book.pdf"), Some("application/xhtml+xml"), Verdict::Warn)]
#[case("https://example.org/page.html", None, Some("text/html"), Verdict::Pass)]
#[case("https://example.org/book.xhtml", None, Some("application/xhtml+xml"), Verdict::Pass)]
#[case(PAGE, None, Some("text/html"), Verdict::Pass)]
#[case("https://example.org/notes.md", None, Some("text/plain"), Verdict::Pass)]
fn a_web_page_sent_for_another_document_gets_the_warning(
	#[case] url: &str,
	#[case] disposition: Option<&str>,
	#[case] content_type: Option<&str>,
	#[case] expected: Verdict,
) {
	assert_eq!(verdict(url, url, disposition, content_type), expected);
}

#[rstest]
#[case("text/html", true)]
#[case("Text/HTML; charset=utf-8", true)]
#[case("application/xhtml+xml", true)]
#[case("application/pdf", false)]
fn is_web_page_reads_the_html_types(#[case] content_type: &str, #[case] expected: bool) {
	assert_eq!(is_web_page(content_type), expected);
}

#[rstest]
#[case("https://example.org/setup.exe", Some("exe"))]
#[case("https://example.org/a.epub", None)]
#[case(PAGE, None)]
fn refused_extension_needs_no_server(#[case] url: &str, #[case] expected: Option<&str>) {
	assert_eq!(refused_extension(url).as_deref(), expected);
}

#[rstest]
#[case("application/pdf", Some("pdf"))]
#[case("Application/PDF", Some("pdf"))]
#[case("text/html; charset=utf-8", Some("htm"))]
#[case("text/plain;charset=windows-1252", Some("txt"))]
#[case("application/epub+zip", Some("epub"))]
#[case("application/octet-stream", None)]
#[case("application/zip", None)]
#[case("application/vnd.openxmlformats-officedocument.wordprocessingml.document", Some("docx"))]
fn extension_for_mime_ignores_parameters_and_case(#[case] content_type: &str, #[case] expected: Option<&str>) {
	assert_eq!(extension_for_mime(content_type), expected);
}

#[rstest]
#[case("attachment; filename=\"Report 2024.pdf\"", Some("Report 2024.pdf"))]
#[case("attachment; FILENAME=a.epub", Some("a.epub"))]
#[case("attachment; filename*=UTF-8''R%C3%A9sum%C3%A9.pdf; filename=\"Resume.pdf\"", Some("Résumé.pdf"))]
#[case("attachment; filename=\"..\\\\..\\\\evil.pdf\"", Some("evil.pdf"))]
#[case("attachment; filename=\"a:b?.pdf\"", Some("a_b_.pdf"))]
#[case("inline", None)]
fn disposition_file_name_reads_both_forms(#[case] header: &str, #[case] expected: Option<&str>) {
	assert_eq!(disposition_file_name(header).as_deref(), expected);
}

#[rstest]
#[case("attachment; filename=\"CON.pdf\"", "_CON.pdf")]
#[case("attachment; filename=\"nul.epub\"", "_nul.epub")]
#[case("attachment; filename=\"com1.txt\"", "_com1.txt")]
#[case("attachment; filename=\"Lpt9\"", "_Lpt9")]
#[case("attachment; filename=\"console.pdf\"", "console.pdf")]
#[case("attachment; filename=\"com10.pdf\"", "com10.pdf")]
fn windows_device_names_get_an_underscore(#[case] header: &str, #[case] expected: &str) {
	assert_eq!(disposition_file_name(header).as_deref(), Some(expected));
}

#[rstest]
#[case("a")]
#[case("\u{e9}")]
fn a_very_long_name_is_shortened_and_keeps_its_extension(#[case] letter: &str) {
	let header = format!("attachment; filename=\"{}.pdf\"", letter.repeat(300));
	let name = disposition_file_name(&header).expect("a name");
	assert!(name.len() <= 200, "{} bytes", name.len());
	assert_eq!(Path::new(&name).extension().and_then(|extension| extension.to_str()), Some("pdf"), "{name}");
}

#[rstest]
#[case("https://example.org/books/a.epub", None, Some("application/epub+zip"), "a.epub")]
#[case(PAGE, None, Some("application/pdf"), "download.pdf")]
#[case("https://example.org/", None, Some("application/pdf"), "document.pdf")]
#[case(PAGE, Some("Report 2024.pdf"), Some("application/octet-stream"), "Report 2024.pdf")]
#[case("https://example.org/a%3Ab.pdf", None, None, "a_b.pdf")]
#[case("https://example.org/notes", None, Some("text/plain; charset=utf-8"), "notes.txt")]
#[case("https://example.org/files/CON.pdf", None, None, "_CON.pdf")]
fn file_name_for_names_the_working_copy(
	#[case] final_url: &str,
	#[case] disposition: Option<&str>,
	#[case] content_type: Option<&str>,
	#[case] expected: &str,
) {
	assert_eq!(file_name_for(final_url, final_url, disposition, content_type), expected);
}

#[rstest]
#[case(
	"https://example.org/files/report.pdf",
	"https://bucket.example.net/7f3a2b?sig=1",
	None,
	Some("binary/octet-stream"),
	"7f3a2b.pdf"
)]
#[case(
	"https://example.org/files/report.pdf",
	"https://example.org/files/report.pdf",
	Some("report"),
	None,
	"report.pdf"
)]
#[case("https://example.org/book", "https://cdn.example.org/files/book.epub", Some("book"), None, "book.epub")]
#[case(
	"https://example.org/files/report.pdf",
	"https://bucket.example.net/7f3a2b",
	None,
	Some("application/epub+zip"),
	"7f3a2b.epub"
)]
fn a_name_without_an_extension_takes_one_from_the_links(
	#[case] given_url: &str,
	#[case] final_url: &str,
	#[case] disposition: Option<&str>,
	#[case] content_type: Option<&str>,
	#[case] expected: &str,
) {
	assert_eq!(file_name_for(given_url, final_url, disposition, content_type), expected);
}
