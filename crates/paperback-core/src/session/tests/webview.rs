use std::{
	env,
	fs::{self, File},
	io::Write,
	path::{Path, PathBuf},
	time::{SystemTime, UNIX_EPOCH},
};

use super::*;

#[test]
fn webview_target_path_returns_none_for_missing_markdown_file() {
	let session = DocumentSession {
		handle: sample_session(ParserFlags::NONE).handle,
		file_path: "C:\\docs\\chapter.md".to_string(),
		history: Vec::new(),
		history_index: 0,
		parser_flags: ParserFlags::NONE,
		last_stable_position: None,
	};
	assert!(session.webview_target_path(0, "C:\\temp").is_none());
}

#[test]
fn webview_target_path_returns_none_for_non_webview_extensions() {
	let session = sample_session(ParserFlags::NONE);
	assert!(session.webview_target_path(0, "C:\\temp").is_none());
}

#[test]
fn extract_resource_returns_false_for_non_epub_files() {
	let session = DocumentSession {
		handle: sample_session(ParserFlags::NONE).handle,
		file_path: "C:\\docs\\chapter.txt".to_string(),
		history: Vec::new(),
		history_index: 0,
		parser_flags: ParserFlags::NONE,
		last_stable_position: None,
	};
	assert_eq!(session.extract_resource("anything", "out.file").ok(), Some(false));
}

fn session_with_path(file_path: &str) -> DocumentSession {
	DocumentSession {
		handle: sample_session(ParserFlags::NONE).handle,
		file_path: file_path.to_string(),
		history: Vec::new(),
		history_index: 0,
		parser_flags: ParserFlags::NONE,
		last_stable_position: None,
	}
}

fn unique_temp_dir() -> PathBuf {
	let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
	env::temp_dir().join(format!("paperback_source_test_{nanos}"))
}

#[test]
fn source_view_available_matches_text_source_formats() {
	assert!(session_with_path("book.epub").source_view_available());
	assert!(session_with_path("page.html").source_view_available());
	assert!(session_with_path("page.htm").source_view_available());
	assert!(session_with_path("page.xhtml").source_view_available());
	assert!(session_with_path("notes.md").source_view_available());
	assert!(session_with_path("notes.markdown").source_view_available());
	assert!(!session_with_path("doc.pdf").source_view_available());
	assert!(!session_with_path("doc.docx").source_view_available());
	assert!(!session_with_path("plain.txt").source_view_available());
}

#[test]
fn view_source_returns_none_for_unsupported_format() {
	let dir = unique_temp_dir();
	let src = dir.join("doc.pdf");
	fs::create_dir_all(&dir).unwrap();
	fs::write(&src, b"%PDF-1.7").unwrap();
	let session = session_with_path(&src.to_string_lossy());
	assert!(session.view_source(0, &dir.to_string_lossy()).is_none());
	let _ = fs::remove_dir_all(&dir);
}

#[test]
fn view_source_writes_html_source_and_maps_caret_forward() {
	let dir = unique_temp_dir();
	fs::create_dir_all(&dir).unwrap();
	let html = "<html><body><p id=\"a\">Alpha</p><p id=\"b\">Bravo</p></body></html>";
	let src = dir.join("page.html");
	fs::write(&src, html.as_bytes()).unwrap();
	let session = session_with_path(&src.to_string_lossy());
	let at_start = session.view_source(0, &dir.to_string_lossy()).expect("source at start");
	// Source written verbatim to a .txt file.
	assert!(at_start.path.ends_with("page.html.source.txt"));
	assert_eq!(fs::read_to_string(&at_start.path).unwrap(), html);
	// A later reading position maps to a caret deeper in the source.
	let at_bravo = session.view_source(6, &dir.to_string_lossy()).expect("source at bravo");
	assert!(at_bravo.caret > at_start.caret);
	let tail: String = html.chars().skip(usize::try_from(at_bravo.caret).unwrap()).collect();
	assert!(tail.contains("Bravo"), "caret should land at/before the second paragraph: {tail}");
	let _ = fs::remove_dir_all(&dir);
}

#[test]
fn view_source_for_markdown_maps_caret_to_current_block() {
	let dir = unique_temp_dir();
	fs::create_dir_all(&dir).unwrap();
	let md = "# Title\n\nFirst paragraph.\n\nSecond paragraph.\n";
	let src = dir.join("notes.md");
	fs::write(&src, md.as_bytes()).unwrap();
	// A real session populates id_positions with pb-block-N anchors.
	let session = DocumentSession::new(&src.to_string_lossy(), "", "", false).expect("open markdown");
	let rendered = session.content();
	let pos = i64::try_from(rendered.find("Second").expect("second block rendered")).unwrap();
	let view = session.view_source(pos, &dir.to_string_lossy()).expect("markdown source");
	assert!(view.path.ends_with("notes.md.source.txt"));
	assert_eq!(fs::read_to_string(&view.path).unwrap(), md);
	// Caret lands at the start of the second paragraph in the raw Markdown.
	let tail: String = md.chars().skip(usize::try_from(view.caret).unwrap()).collect();
	assert!(tail.starts_with("Second paragraph."), "caret should be at the current block: {tail}");
	let _ = fs::remove_dir_all(&dir);
}

#[test]
fn extract_resource_for_missing_epub_returns_error() {
	let session = DocumentSession {
		handle: sample_session(ParserFlags::NONE).handle,
		file_path: "C:\\path\\does\\not\\exist.epub".to_string(),
		history: Vec::new(),
		history_index: 0,
		parser_flags: ParserFlags::NONE,
		last_stable_position: None,
	};
	assert!(session.extract_resource("x", "y").is_err());
}

/// Builds a minimal real EPUB on disk whose spine chapter references an image
/// via a relative path that only resolves if sibling directory structure is
/// preserved on extraction, then returns its path.
fn build_epub_with_relative_image(dir: &Path) -> PathBuf {
	use zip::{ZipWriter, write::FileOptions};
	let epub_path = dir.join("book.epub");
	let file = File::create(&epub_path).expect("create epub file");
	let mut writer = ZipWriter::new(file);
	let opts = FileOptions::<()>::default();
	writer.start_file("mimetype", opts).unwrap();
	writer.write_all(b"application/epub+zip").unwrap();
	writer.start_file("META-INF/container.xml", opts).unwrap();
	writer
		.write_all(
			br#"<?xml version="1.0"?>
<container version="1.0" xmlns="urn:oasis:names:tc:opendocument:xmlns:container">
	<rootfiles>
		<rootfile full-path="OEBPS/content.opf" media-type="application/oebps-package+xml"/>
	</rootfiles>
</container>"#,
		)
		.unwrap();
	writer.start_file("OEBPS/content.opf", opts).unwrap();
	writer
		.write_all(
			br#"<?xml version="1.0"?>
<package xmlns="http://www.idpf.org/2007/opf" version="3.0" unique-identifier="bookid">
	<metadata xmlns:dc="http://purl.org/dc/elements/1.1/">
		<dc:title>Test Book</dc:title>
		<dc:identifier id="bookid">test-book</dc:identifier>
	</metadata>
	<manifest>
		<item id="chapter1" href="Text/chapter1.xhtml" media-type="application/xhtml+xml"/>
		<item id="cover-img" href="Images/cover.jpg" media-type="image/jpeg"/>
	</manifest>
	<spine>
		<itemref idref="chapter1"/>
	</spine>
</package>"#,
		)
		.unwrap();
	writer.start_file("OEBPS/Text/chapter1.xhtml", opts).unwrap();
	writer
		.write_all(
			br#"<?xml version="1.0"?>
<html xmlns="http://www.w3.org/1999/xhtml"><body>
	<p>Chapter text.</p>
	<img src="../Images/cover.jpg" alt="Cover"/>
</body></html>"#,
		)
		.unwrap();
	writer.start_file("OEBPS/Images/cover.jpg", opts).unwrap();
	// The filler text intentionally avoids starting with a hex digit right after the
	// \xNN escapes above: `gen-pot`'s xgettext pass parses this file in C mode, where
	// \x escapes are greedy and would otherwise swallow leading hex-looking characters
	// (e.g. "fake" starting with a valid hex digit) into a wildly out-of-range escape.
	writer.write_all(b"\xFF\xD8\xFF\xE0placeholder-jpeg-bytes").unwrap();
	writer.finish().unwrap();
	epub_path
}

#[test]
fn webview_target_path_extracts_sibling_image_resources() {
	let temp_root = env::temp_dir()
		.join(format!("paperback_webview_test_{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()));
	fs::create_dir_all(&temp_root).unwrap();
	let epub_path = build_epub_with_relative_image(&temp_root);
	let session = DocumentSession::new(&epub_path.to_string_lossy(), "", "", false).expect("parse test epub");
	let target = session.webview_target_path(0, &temp_root.to_string_lossy()).expect("webview target");
	let section_content = fs::read_to_string(&target.path).expect("read extracted section");
	assert!(section_content.contains("Images/cover.jpg"));
	// The image referenced relatively from the section must have been
	// extracted alongside it at the same relative location.
	let image_path = Path::new(&target.path).parent().unwrap().parent().unwrap().join("Images/cover.jpg");
	assert!(image_path.exists(), "expected image extracted at {}", image_path.display());
	fs::remove_dir_all(&temp_root).ok();
}

/// Builds a minimal real EPUB whose first spine section is a table of contents
/// linking to a second section, then returns its path.
fn build_epub_with_linked_sections(dir: &Path) -> PathBuf {
	let epub_path = dir.join("linked.epub");
	let file = fs::File::create(&epub_path).unwrap();
	let mut writer = zip::ZipWriter::new(file);
	let opts = zip::write::FileOptions::<()>::default();
	writer.start_file("META-INF/container.xml", opts).unwrap();
	writer
		.write_all(
			br#"<?xml version="1.0"?>
<container xmlns="urn:oasis:names:tc:opendocument:xmlns:container" version="1.0">
	<rootfiles><rootfile full-path="OEBPS/content.opf" media-type="application/oebps-package+xml"/></rootfiles>
</container>"#,
		)
		.unwrap();
	writer.start_file("OEBPS/content.opf", opts).unwrap();
	writer
		.write_all(
			br#"<?xml version="1.0"?>
<package xmlns="http://www.idpf.org/2007/opf" version="3.0" unique-identifier="bookid">
	<metadata xmlns:dc="http://purl.org/dc/elements/1.1/">
		<dc:title>Linked Book</dc:title>
		<dc:identifier id="bookid">linked-book</dc:identifier>
	</metadata>
	<manifest>
		<item id="toc" href="Text/toc.xhtml" media-type="application/xhtml+xml"/>
		<item id="chapter1" href="Text/chapter1.xhtml" media-type="application/xhtml+xml"/>
	</manifest>
	<spine>
		<itemref idref="toc"/>
		<itemref idref="chapter1"/>
	</spine>
</package>"#,
		)
		.unwrap();
	writer.start_file("OEBPS/Text/toc.xhtml", opts).unwrap();
	writer
		.write_all(
			br#"<?xml version="1.0"?>
<html xmlns="http://www.w3.org/1999/xhtml"><body>
	<h1>Contents</h1>
	<p><a href="chapter1.xhtml">Chapter One</a></p>
</body></html>"#,
		)
		.unwrap();
	writer.start_file("OEBPS/Text/chapter1.xhtml", opts).unwrap();
	writer
		.write_all(
			br#"<?xml version="1.0"?>
<html xmlns="http://www.w3.org/1999/xhtml"><body>
	<p>Chapter text.</p>
</body></html>"#,
		)
		.unwrap();
	writer.finish().unwrap();
	epub_path
}

/// The section a table of contents links to has to be on disk too, or following
/// that link in the web view fails with ERR_FILE_NOT_FOUND (issue #719).
#[test]
fn webview_target_path_extracts_linked_sibling_sections() {
	let temp_root = env::temp_dir()
		.join(format!("paperback_webview_toc_{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()));
	fs::create_dir_all(&temp_root).unwrap();
	let epub_path = build_epub_with_linked_sections(&temp_root);
	let session = DocumentSession::new(&epub_path.to_string_lossy(), "", "", false).expect("parse test epub");
	let target = session.webview_target_path(0, &temp_root.to_string_lossy()).expect("webview target");
	let toc_content = fs::read_to_string(&target.path).expect("read extracted toc");
	assert!(toc_content.contains("chapter1.xhtml"), "expected the toc to link to the chapter");
	let linked = Path::new(&target.path).parent().unwrap().join("chapter1.xhtml");
	assert!(linked.exists(), "expected linked section extracted at {}", linked.display());
	fs::remove_dir_all(&temp_root).ok();
}
