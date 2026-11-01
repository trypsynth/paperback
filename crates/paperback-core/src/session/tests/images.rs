use std::{fs, fs::File, io::Write, path::Path};

use zip::{ZipWriter, write::FileOptions};

use super::*;
use crate::util::test_support::TempDir;

const PICTURE: &[u8] = b"\x89PNG\r\n\x1a\nnot-really-a-png";

fn open(path: &Path) -> DocumentSession {
	DocumentSession::new(&path.to_string_lossy(), "", "", ParseSettings::default()).expect("parse test document")
}

fn position_of(session: &DocumentSession, text: &str) -> DocPos {
	let content = session.content();
	let byte = content.find(text).unwrap_or_else(|| panic!("{text:?} not in {content:?}"));
	DocLen::of(&content[..byte]).as_pos()
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
	let dir = TempDir::new("images");
	fs::create_dir_all(dir.join("pics")).unwrap();
	fs::write(dir.join("pics/map 1.png"), PICTURE).unwrap();
	fs::write(
		dir.join("page.html"),
		r#"<html><body><p>Before</p><p><img src="pics/map%201.png" alt="A map"></p></body></html>"#,
	)
	.unwrap();
	let session = open(&dir.join("page.html"));
	let at = position_of(&session, "[Image: A map]") + 3.into();
	assert_eq!(session.image_file_name_at(at).as_deref(), Some("map 1.png"));
	let out = dir.join("out.png");
	session.save_image(at, &out.to_string_lossy()).unwrap();
	assert_eq!(fs::read(&out).unwrap(), PICTURE);
}

#[test]
fn text_that_is_not_an_image_offers_nothing_to_save() {
	let dir = TempDir::new("images");
	fs::write(dir.join("page.html"), r#"<html><body><p>Before</p><p><img src="a.png" alt="A map"></p></body></html>"#)
		.unwrap();
	let session = open(&dir.join("page.html"));
	assert_eq!(session.image_file_name_at(position_of(&session, "Before")), None);
}

#[test]
fn a_remote_picture_is_not_offered() {
	let dir = TempDir::new("images");
	fs::write(
		dir.join("page.html"),
		r#"<html><body><p><img src="https://example.com/a.png" alt="A map"></p></body></html>"#,
	)
	.unwrap();
	let session = open(&dir.join("page.html"));
	assert_eq!(session.image_file_name_at(position_of(&session, "[Image")), None);
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
	let dir = TempDir::new("images");
	let epub = dir.join("book.epub");
	write_epub(&epub);
	let session = open(&epub);
	let at = position_of(&session, "[Image: Figure one]");
	assert_eq!(session.image_file_name_at(at).as_deref(), Some("fig.png"));
	let out = dir.join("fig.png");
	session.save_image(at, &out.to_string_lossy()).unwrap();
	assert_eq!(fs::read(&out).unwrap(), PICTURE);
}

/// A picture book's plates carry no description, so the page's own markup is the only thing that says which picture it shows.
#[test]
fn an_epub_picture_page_saves_its_picture_rather_than_the_page() {
	let dir = TempDir::new("images");
	let epub = dir.join("book.epub");
	write_epub(&epub);
	let session = open(&epub);
	assert_eq!(session.image_file_name_at(0.into()).as_deref(), Some("plate.png"));
	let out = dir.join("plate.png");
	session.save_image(0.into(), &out.to_string_lossy()).unwrap();
	assert_eq!(fs::read(&out).unwrap(), b"plate-bytes");
}

#[test]
fn a_comic_page_is_saved_from_the_archive() {
	let dir = TempDir::new("images");
	let cbz = dir.join("comic.cbz");
	write_zip(&cbz, &[("001.png", b"first"), ("002.png", PICTURE)]);
	let session = open(&cbz);
	let second = session.page_position(2).unwrap();
	assert_eq!(session.image_file_name_at(second).as_deref(), Some("002.png"));
	let out = dir.join("page.png");
	session.save_image(second, &out.to_string_lossy()).unwrap();
	assert_eq!(fs::read(&out).unwrap(), PICTURE);
}

#[test]
fn a_picture_beside_an_html_page_offers_its_bytes() {
	let dir = TempDir::new("images");
	fs::create_dir_all(dir.join("pics")).unwrap();
	fs::write(dir.join("pics/map 1.png"), PICTURE).unwrap();
	fs::write(
		dir.join("page.html"),
		r#"<html><body><p>Before</p><p><img src="pics/map%201.png" alt="A map"></p></body></html>"#,
	)
	.unwrap();
	let session = open(&dir.join("page.html"));
	let at = position_of(&session, "[Image: A map]") + 3.into();
	assert_eq!(session.image_bytes_at(at).unwrap(), PICTURE);
}

#[test]
fn an_epub_picture_offers_its_bytes() {
	let dir = TempDir::new("images");
	let epub = dir.join("book.epub");
	write_epub(&epub);
	let session = open(&epub);
	let at = position_of(&session, "[Image: Figure one]");
	assert_eq!(session.image_bytes_at(at).unwrap(), PICTURE);
}

#[test]
fn text_that_is_not_an_image_has_no_bytes() {
	let dir = TempDir::new("images");
	fs::write(dir.join("page.html"), r"<html><body><p>Before</p></body></html>").unwrap();
	let session = open(&dir.join("page.html"));
	assert!(session.image_bytes_at(position_of(&session, "Before")).is_err());
}

/// Enter follows a picture's link when it has one, so a linked picture has to read as both.
#[test]
fn a_linked_picture_is_on_a_link_and_a_plain_one_is_not() {
	let dir = TempDir::new("images");
	fs::write(dir.join("map.png"), PICTURE).unwrap();
	fs::write(
		dir.join("page.html"),
		r#"<html><body><p><a href="https://example.com"><img src="map.png" alt="Linked"></a></p><p><img src="map.png" alt="Plain"></p></body></html>"#,
	)
	.unwrap();
	let session = open(&dir.join("page.html"));
	let linked = position_of(&session, "[Image: Linked]") + 3.into();
	let plain = position_of(&session, "[Image: Plain]") + 3.into();
	assert!(session.image_file_name_at(linked).is_some() && session.is_on_link(linked));
	assert!(session.image_file_name_at(plain).is_some() && !session.is_on_link(plain));
}

/// #1166: a zipped DAISY book, as Bookshare sends them, names pictures relative to the `DTBook` inside the archive.
#[test]
fn a_picture_in_a_zipped_daisy_book_offers_its_bytes() {
	let dir = TempDir::new("images");
	let book = dir.join("book.zip");
	let opf = br#"<?xml version="1.0" encoding="UTF-8"?>
<package unique-identifier="uid"><metadata><dc-metadata xmlns:dc="http://purl.org/dc/elements/1.1/"><dc:Title>Maps</dc:Title></dc-metadata></metadata>
<manifest><item id="text" href="book.xml" media-type="application/x-dtbook+xml"/></manifest><spine><itemref idref="text"/></spine></package>"#;
	let dtbook = br#"<?xml version="1.0" encoding="UTF-8"?>
<dtbook><book><bodymatter><p>Before</p><imggroup><img src="images/map.png" alt="A map"/></imggroup></bodymatter></book></dtbook>"#;
	write_zip(&book, &[("content/book.opf", opf), ("content/book.xml", dtbook), ("content/images/map.png", PICTURE)]);
	let session = open(&book);
	let at = position_of(&session, "[Image: A map]") + 3.into();
	assert_eq!(session.image_bytes_at(at).unwrap(), PICTURE);
}

/// Word pictures used to be dropped without a trace.
#[test]
fn a_word_picture_reads_as_its_alt_text_and_offers_its_bytes() {
	let dir = TempDir::new("images");
	let document = dir.join("report.docx");
	let body = br#"<document><body><p><r><t xml:space="preserve">See </t></r><r><drawing><inline><docPr id="1" name="Picture 1" descr="Sales by region"/><graphic><graphicData><pic><blipFill><blip embed="rId5"/></blipFill></pic></graphicData></graphic></inline></drawing></r></p><p><r><drawing><inline><docPr id="2" name="Rule"><extLst><ext><decorative val="1"/></ext></extLst></docPr><graphic><graphicData><pic><blipFill><blip embed="rId5"/></blipFill></pic></graphicData></graphic></inline></drawing></r><r><t>After.</t></r></p><p><r><pict><shape alt="Old logo"><imagedata id="rId5"/></shape></pict></r></p></body></document>"#;
	let rels = br#"<Relationships><Relationship Id="rId5" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/image" Target="media/image1.png"/></Relationships>"#;
	write_zip(
		&document,
		&[("word/document.xml", body), ("word/_rels/document.xml.rels", rels), ("word/media/image1.png", PICTURE)],
	);
	let session = open(&document);
	assert_eq!(session.content(), "See [Image: Sales by region]\nAfter.\n[Image: Old logo]\n");
	let at = position_of(&session, "[Image: Sales by region]") + 3.into();
	assert_eq!(session.image_bytes_at(at).unwrap(), PICTURE);
	assert_eq!(session.image_bytes_at(position_of(&session, "[Image: Old logo]")).unwrap(), PICTURE);
}

fn mobi_book(html: &str, pictures: &[&[u8]]) -> Vec<u8> {
	let mut mobi = vec![0u8; 232];
	mobi[..4].copy_from_slice(b"MOBI");
	mobi[4..8].copy_from_slice(&232u32.to_be_bytes());
	mobi[12..16].copy_from_slice(&65001u32.to_be_bytes());
	mobi[20..24].copy_from_slice(&6u32.to_be_bytes());
	// Every optional index the header can point at is absent, and the pictures start at record 2.
	for at in (24..232).step_by(4) {
		mobi[at..at + 4].copy_from_slice(&u32::MAX.to_be_bytes());
	}
	mobi[92..96].copy_from_slice(&2u32.to_be_bytes());
	mobi[112..116].copy_from_slice(&0u32.to_be_bytes());
	mobi[226..228].copy_from_slice(&0u16.to_be_bytes());
	let mut record0 = Vec::new();
	record0.extend_from_slice(&1u16.to_be_bytes());
	record0.extend_from_slice(&[0, 0]);
	record0.extend_from_slice(&u32::try_from(html.len()).unwrap().to_be_bytes());
	record0.extend_from_slice(&1u16.to_be_bytes());
	record0.extend_from_slice(&4096u16.to_be_bytes());
	record0.extend_from_slice(&[0, 0, 0, 0]);
	record0.extend(mobi);
	let records: Vec<&[u8]> =
		[record0.as_slice(), html.as_bytes()].into_iter().chain(pictures.iter().copied()).collect();
	let mut data = vec![0u8; 76];
	data[..4].copy_from_slice(b"Book");
	data.extend_from_slice(&u16::try_from(records.len()).unwrap().to_be_bytes());
	let mut offset = 78 + records.len() * 8;
	for record in &records {
		data.extend_from_slice(&u32::try_from(offset).unwrap().to_be_bytes());
		data.extend_from_slice(&[0, 0, 0, 0]);
		offset += record.len();
	}
	for record in &records {
		data.extend_from_slice(record);
	}
	data
}

/// MOBI pictures are records named by number, and nothing used to reach them.
#[test]
fn a_mobi_picture_offers_its_bytes_from_its_record() {
	let dir = TempDir::new("images");
	let path = dir.join("book.mobi");
	let gif: &[u8] = b"GIF89a-not-really";
	fs::write(
		&path,
		mobi_book(r#"<html><body><p>Before</p><p><img recindex="00001" alt="A map"></p></body></html>"#, &[gif]),
	)
	.unwrap();
	let session = open(&path);
	let at = position_of(&session, "[Image: A map]") + 3.into();
	assert_eq!(session.image_file_name_at(at).as_deref(), Some("image 2.gif"));
	assert_eq!(session.image_bytes_at(at).unwrap(), gif);
}
