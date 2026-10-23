use std::{
	env, fs,
	fs::File,
	io::Write,
	path::{Path, PathBuf},
	time::{SystemTime, UNIX_EPOCH},
};

use zip::{ZipWriter, write::FileOptions};

use super::*;

const PICTURE: &[u8] = b"\x89PNG\r\n\x1a\nnot-really-a-png";

fn unique_temp_dir() -> PathBuf {
	let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
	let dir = env::temp_dir().join(format!("paperback_images_test_{nanos}"));
	fs::create_dir_all(&dir).unwrap();
	dir
}

fn open(path: &Path) -> DocumentSession {
	DocumentSession::new(&path.to_string_lossy(), "", "", ParseSettings::default()).expect("parse test document")
}

fn position_of(session: &DocumentSession, text: &str) -> i64 {
	let content = session.content();
	let byte = content.find(text).unwrap_or_else(|| panic!("{text:?} not in {content:?}"));
	i64::try_from(content[..byte].chars().count()).unwrap()
}

fn write_zip(path: &Path, entries: &[(&str, &[u8])]) {
	let mut writer = ZipWriter::new(File::create(path).unwrap());
	for (name, data) in entries {
		writer.start_file(*name, FileOptions::<()>::default()).unwrap();
		writer.write_all(data).unwrap();
	}
	writer.finish().unwrap();
}

#[test]
fn a_picture_beside_an_html_page_is_saved_byte_for_byte() {
	let dir = unique_temp_dir();
	fs::create_dir_all(dir.join("pics")).unwrap();
	fs::write(dir.join("pics/map 1.png"), PICTURE).unwrap();
	fs::write(
		dir.join("page.html"),
		r#"<html><body><p>Before</p><p><img src="pics/map%201.png" alt="A map"></p></body></html>"#,
	)
	.unwrap();
	let session = open(&dir.join("page.html"));
	let at = position_of(&session, "[Image: A map]") + 3;
	assert_eq!(session.image_file_name_at(at).as_deref(), Some("map 1.png"));
	let out = dir.join("out.png");
	session.save_image(at, &out.to_string_lossy()).unwrap();
	assert_eq!(fs::read(&out).unwrap(), PICTURE);
	fs::remove_dir_all(&dir).ok();
}

#[test]
fn text_that_is_not_an_image_offers_nothing_to_save() {
	let dir = unique_temp_dir();
	fs::write(dir.join("page.html"), r#"<html><body><p>Before</p><p><img src="a.png" alt="A map"></p></body></html>"#)
		.unwrap();
	let session = open(&dir.join("page.html"));
	assert_eq!(session.image_file_name_at(position_of(&session, "Before")), None);
	fs::remove_dir_all(&dir).ok();
}

#[test]
fn a_remote_picture_is_not_offered() {
	let dir = unique_temp_dir();
	fs::write(
		dir.join("page.html"),
		r#"<html><body><p><img src="https://example.com/a.png" alt="A map"></p></body></html>"#,
	)
	.unwrap();
	let session = open(&dir.join("page.html"));
	assert_eq!(session.image_file_name_at(position_of(&session, "[Image")), None);
	fs::remove_dir_all(&dir).ok();
}

fn write_epub(path: &Path) {
	let container = br#"<?xml version="1.0"?>
<container version="1.0" xmlns="urn:oasis:names:tc:opendocument:xmlns:container">
	<rootfiles><rootfile full-path="OEBPS/content.opf" media-type="application/oebps-package+xml"/></rootfiles>
</container>"#;
	let opf = br#"<?xml version="1.0"?>
<package xmlns="http://www.idpf.org/2007/opf" version="3.0" unique-identifier="id">
	<metadata xmlns:dc="http://purl.org/dc/elements/1.1/"><dc:title>Pictures</dc:title><dc:identifier id="id">pictures</dc:identifier></metadata>
	<manifest>
		<item id="plate" href="Text/plate.xhtml" media-type="application/xhtml+xml"/>
		<item id="chapter" href="Text/chapter.xhtml" media-type="application/xhtml+xml"/>
		<item id="p1" href="Images/plate.png" media-type="image/png"/>
		<item id="p2" href="Images/fig.png" media-type="image/png"/>
	</manifest>
	<spine><itemref idref="plate"/><itemref idref="chapter"/></spine>
</package>"#;
	let plate = br#"<?xml version="1.0"?>
<html xmlns="http://www.w3.org/1999/xhtml"><body><div><img src="../Images/plate.png"/></div></body></html>"#;
	let chapter = br#"<?xml version="1.0"?>
<html xmlns="http://www.w3.org/1999/xhtml"><body><p>Chapter text.</p><p><img src="../Images/fig.png" alt="Figure one"/></p></body></html>"#;
	write_zip(
		path,
		&[
			("mimetype", b"application/epub+zip"),
			("META-INF/container.xml", container),
			("OEBPS/content.opf", opf),
			("OEBPS/Text/plate.xhtml", plate),
			("OEBPS/Text/chapter.xhtml", chapter),
			("OEBPS/Images/plate.png", b"plate-bytes"),
			("OEBPS/Images/fig.png", PICTURE),
		],
	);
}

#[test]
fn an_epub_picture_resolves_against_its_chapter() {
	let dir = unique_temp_dir();
	let epub = dir.join("book.epub");
	write_epub(&epub);
	let session = open(&epub);
	let at = position_of(&session, "[Image: Figure one]");
	assert_eq!(session.image_file_name_at(at).as_deref(), Some("fig.png"));
	let out = dir.join("fig.png");
	session.save_image(at, &out.to_string_lossy()).unwrap();
	assert_eq!(fs::read(&out).unwrap(), PICTURE);
	fs::remove_dir_all(&dir).ok();
}

/// A picture book's plates carry no description, so the page's own markup is the only thing that says which picture it shows.
#[test]
fn an_epub_picture_page_saves_its_picture_rather_than_the_page() {
	let dir = unique_temp_dir();
	let epub = dir.join("book.epub");
	write_epub(&epub);
	let session = open(&epub);
	assert_eq!(session.image_file_name_at(0).as_deref(), Some("plate.png"));
	let out = dir.join("plate.png");
	session.save_image(0, &out.to_string_lossy()).unwrap();
	assert_eq!(fs::read(&out).unwrap(), b"plate-bytes");
	fs::remove_dir_all(&dir).ok();
}

#[test]
fn a_comic_page_is_saved_from_the_archive() {
	let dir = unique_temp_dir();
	let cbz = dir.join("comic.cbz");
	write_zip(&cbz, &[("001.png", b"first"), ("002.png", PICTURE)]);
	let session = open(&cbz);
	let second = session.page_offset(2);
	assert_eq!(session.image_file_name_at(second).as_deref(), Some("002.png"));
	let out = dir.join("page.png");
	session.save_image(second, &out.to_string_lossy()).unwrap();
	assert_eq!(fs::read(&out).unwrap(), PICTURE);
	fs::remove_dir_all(&dir).ok();
}

/// The bytes have to be readable without writing a file, because that is what reading the text
/// off a picture does with them.
#[test]
fn a_picture_beside_an_html_page_offers_its_bytes() {
	let dir = unique_temp_dir();
	fs::create_dir_all(dir.join("pics")).unwrap();
	fs::write(dir.join("pics/map 1.png"), PICTURE).unwrap();
	fs::write(
		dir.join("page.html"),
		r#"<html><body><p>Before</p><p><img src="pics/map%201.png" alt="A map"></p></body></html>"#,
	)
	.unwrap();
	let session = open(&dir.join("page.html"));
	let at = position_of(&session, "[Image: A map]") + 3;
	assert_eq!(session.image_bytes_at(at).unwrap(), PICTURE);
	fs::remove_dir_all(&dir).ok();
}

#[test]
fn an_epub_picture_offers_its_bytes() {
	let dir = unique_temp_dir();
	let epub = dir.join("book.epub");
	write_epub(&epub);
	let session = open(&epub);
	let at = position_of(&session, "[Image: Figure one]");
	assert_eq!(session.image_bytes_at(at).unwrap(), PICTURE);
	fs::remove_dir_all(&dir).ok();
}

/// A line that is not a picture has nothing to hand the OCR engine, and saying so is better than
/// recognizing whatever happened to be there.
#[test]
fn text_that_is_not_an_image_has_no_bytes() {
	let dir = unique_temp_dir();
	fs::write(dir.join("page.html"), r"<html><body><p>Before</p></body></html>").unwrap();
	let session = open(&dir.join("page.html"));
	assert!(session.image_bytes_at(position_of(&session, "Before")).is_err());
	fs::remove_dir_all(&dir).ok();
}

/// Enter follows a picture's link when it has one, so a linked picture has to read as both.
#[test]
fn a_linked_picture_is_on_a_link_and_a_plain_one_is_not() {
	let dir = unique_temp_dir();
	fs::create_dir_all(&dir).unwrap();
	fs::write(dir.join("map.png"), PICTURE).unwrap();
	fs::write(
		dir.join("page.html"),
		r#"<html><body><p><a href="https://example.com"><img src="map.png" alt="Linked"></a></p><p><img src="map.png" alt="Plain"></p></body></html>"#,
	)
	.unwrap();
	let session = open(&dir.join("page.html"));
	let linked = position_of(&session, "[Image: Linked]") + 3;
	let plain = position_of(&session, "[Image: Plain]") + 3;
	assert!(session.image_file_name_at(linked).is_some() && session.is_on_link(linked));
	assert!(session.image_file_name_at(plain).is_some() && !session.is_on_link(plain));
	fs::remove_dir_all(&dir).ok();
}

/// #1166: a zipped DAISY book, as Bookshare sends them, names its pictures relative to the `DTBook` inside the archive, so reading one has to find it there rather than on disk.
#[test]
fn a_picture_in_a_zipped_daisy_book_offers_its_bytes() {
	let dir = unique_temp_dir();
	let book = dir.join("book.zip");
	let opf = br#"<?xml version="1.0" encoding="UTF-8"?>
<package unique-identifier="uid"><metadata><dc-metadata xmlns:dc="http://purl.org/dc/elements/1.1/"><dc:Title>Maps</dc:Title></dc-metadata></metadata>
<manifest><item id="text" href="book.xml" media-type="application/x-dtbook+xml"/></manifest><spine><itemref idref="text"/></spine></package>"#;
	let dtbook = br#"<?xml version="1.0" encoding="UTF-8"?>
<dtbook><book><bodymatter><p>Before</p><imggroup><img src="images/map.png" alt="A map"/></imggroup></bodymatter></book></dtbook>"#;
	write_zip(&book, &[("content/book.opf", opf), ("content/book.xml", dtbook), ("content/images/map.png", PICTURE)]);
	let session = open(&book);
	let at = position_of(&session, "[Image: A map]") + 3;
	assert_eq!(session.image_bytes_at(at).unwrap(), PICTURE);
	fs::remove_dir_all(&dir).ok();
}

/// Word pictures were dropped without a trace. A picture now reads as its alt text, can be saved and read with OCR from the `.docx` itself, and one its author marked as decorative stays out of the way.
#[test]
fn a_word_picture_reads_as_its_alt_text_and_offers_its_bytes() {
	let dir = unique_temp_dir();
	let document = dir.join("report.docx");
	let body = br#"<document><body><p><r><t xml:space="preserve">See </t></r><r><drawing><inline><docPr id="1" name="Picture 1" descr="Sales by region"/><graphic><graphicData><pic><blipFill><blip embed="rId5"/></blipFill></pic></graphicData></graphic></inline></drawing></r></p><p><r><drawing><inline><docPr id="2" name="Rule"><extLst><ext><decorative val="1"/></ext></extLst></docPr><graphic><graphicData><pic><blipFill><blip embed="rId5"/></blipFill></pic></graphicData></graphic></inline></drawing></r><r><t>After.</t></r></p><p><r><pict><shape alt="Old logo"><imagedata id="rId5"/></shape></pict></r></p></body></document>"#;
	let rels = br#"<Relationships><Relationship Id="rId5" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/image" Target="media/image1.png"/></Relationships>"#;
	write_zip(
		&document,
		&[("word/document.xml", body), ("word/_rels/document.xml.rels", rels), ("word/media/image1.png", PICTURE)],
	);
	let session = open(&document);
	assert_eq!(session.content(), "See [Image: Sales by region]\nAfter.\n[Image: Old logo]\n");
	let at = position_of(&session, "[Image: Sales by region]") + 3;
	assert_eq!(session.image_bytes_at(at).unwrap(), PICTURE);
	assert_eq!(session.image_bytes_at(position_of(&session, "[Image: Old logo]")).unwrap(), PICTURE);
	fs::remove_dir_all(&dir).ok();
}
