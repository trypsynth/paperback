use std::fs;

use super::*;
use crate::{
	document::ParserContext,
	parser::{Parser, daisy::DaisyParser},
	util::test_support::TempDir,
};

// Regression test for https://github.com/trypsynth/paperback/issues/606: a real-world DAISY
// book declared its DTBook XML as ISO-8859-1 but was actually encoded as Windows-1252 (a very
// common mislabeling), which made `fs::read_to_string` fail outright since the bytes were not
// valid UTF-8.
#[test]
fn parses_dtbook_xml_declared_as_iso_8859_1_but_encoded_as_windows_1252() {
	let dir = TempDir::new("daisy");
	let opf_path = dir.path().join("book.opf");
	let xml_path = dir.path().join("book.xml");
	fs::write(
		&opf_path,
		br#"<?xml version="1.0" encoding="ISO-8859-1"?>
<package unique-identifier="uid">
  <metadata>
    <dc-metadata xmlns:dc="http://purl.org/dc/elements/1.0/">
      <dc:Title>Test Book</dc:Title>
      <dc:Creator>Test Author</dc:Creator>
    </dc-metadata>
  </metadata>
  <manifest>
    <item href="book.xml" media-type="application/x-dtbook+xml"/>
  </manifest>
</package>
"#,
	)
	.expect("write opf");
	// Windows-1252 bytes for curly quotes (0x93/0x94) and 0xE7 for the c-cedilla in
	// "Fran\xE7ois" -- both invalid as standalone UTF-8.
	let mut xml_bytes = Vec::new();
	xml_bytes.extend_from_slice(b"<?xml version=\"1.0\" encoding=\"ISO-8859-1\"?>\n");
	xml_bytes.extend_from_slice(b"<dtbook><book><frontmatter><p id=\"p1\">Fran\xE7ois said, \x93hello\x94.</p>");
	xml_bytes.extend_from_slice(b"</frontmatter></book></dtbook>");
	fs::write(&xml_path, &xml_bytes).expect("write dtbook xml");
	let context = ParserContext::new(opf_path.to_string_lossy().to_string());
	let document = DaisyParser.parse(&context).expect("DAISY parse should succeed on mislabeled encoding");
	assert_eq!(document.title, "Test Book");
	assert_eq!(document.author, "Test Author");
	assert!(!document.buffer.content.contains('\u{FFFD}'), "no replacement characters expected");
	assert!(document.buffer.content.contains("François"), "c-cedilla should decode correctly");
	assert!(document.buffer.content.contains('\u{201C}'), "left curly quote should decode correctly");
	assert!(document.buffer.content.contains('\u{201D}'), "right curly quote should decode correctly");
	assert!(document.audio.is_none());
}

/// A minimal two-section DAISY 3 "full text, full audio" book, exercising the multi-file
/// offset math and the SMIL-to-text-position audio linkage end to end.
#[test]
fn parses_multi_file_daisy_book_with_audio() {
	let opf = br#"<?xml version="1.0" encoding="UTF-8"?>
<package xmlns="http://openebook.org/namespaces/oeb-package/1.0/" unique-identifier="bookid">
  <metadata>
    <dc-metadata xmlns:dc="http://purl.org/dc/elements/1.1/">
      <dc:Title>Two Section Book</dc:Title>
      <dc:Creator>A. Author</dc:Creator>
    </dc-metadata>
  </metadata>
  <manifest>
    <item id="xml1" href="book1.xml" media-type="application/x-dtbook+xml" />
    <item id="xml2" href="book2.xml" media-type="application/x-dtbook+xml" />
    <item id="ncx" href="book.ncx" media-type="application/x-dtbncx+xml" />
    <item id="audio1" href="book1.mp3" media-type="audio/mpeg" />
    <item id="audio2" href="book2.mp3" media-type="audio/mpeg" />
    <item id="smil1" href="section1.smil" media-type="application/smil" />
    <item id="smil2" href="section2.smil" media-type="application/smil" />
  </manifest>
  <spine>
    <itemref idref="smil1" />
    <itemref idref="smil2" />
  </spine>
</package>"#;
	let book1 = br#"<?xml version="1.0" encoding="UTF-8"?>
<dtbook><book><bodymatter>
<h1 id="h1">Chapter One</h1>
<p id="p1">First paragraph.</p>
</bodymatter></book></dtbook>"#;
	let book2 = br#"<?xml version="1.0" encoding="UTF-8"?>
<dtbook><book><bodymatter>
<h1 id="h2">Chapter Two</h1>
<p id="p2">Second paragraph.</p>
</bodymatter></book></dtbook>"#;
	let smil1 = br#"<smil xmlns="http://www.w3.org/2001/SMIL20/"><body><seq id="s">
<par id="par_h1"><text src="book1.xml#h1" /><audio src="book1.mp3" clipBegin="0s" clipEnd="2s" /></par>
<par id="par_p1"><text src="book1.xml#p1" /><audio src="book1.mp3" clipBegin="2s" clipEnd="5s" /></par>
</seq></body></smil>"#;
	let smil2 = br#"<smil xmlns="http://www.w3.org/2001/SMIL20/"><body><seq id="s">
<par id="par_h2"><text src="book2.xml#h2" /><audio src="book2.mp3" clipBegin="0s" clipEnd="1.5s" /></par>
<par id="par_p2"><text src="book2.xml#p2" /><audio src="book2.mp3" clipBegin="1.5s" clipEnd="4s" /></par>
</seq></body></smil>"#;
	let ncx = br#"<?xml version="1.0" encoding="UTF-8"?>
<ncx xmlns="http://www.daisy.org/z3986/2005/ncx/"><navMap>
<navPoint id="np1"><navLabel><text>Chapter One</text></navLabel><content src="book1.xml#h1" /></navPoint>
<navPoint id="np2"><navLabel><text>Chapter Two</text></navLabel><content src="book2.xml#h2" /></navPoint>
</navMap></ncx>"#;
	let zip_bytes = write_zip(&[
		("book.opf", opf.as_slice()),
		("book1.xml", book1.as_slice()),
		("book2.xml", book2.as_slice()),
		("book.ncx", ncx.as_slice()),
		("section1.smil", smil1.as_slice()),
		("section2.smil", smil2.as_slice()),
		("book1.mp3", b"fake-mp3-1"),
		("book2.mp3", b"fake-mp3-2"),
	]);
	let dir = TempDir::new("daisy_multi");
	let zip_path = dir.path().join("book.zip");
	fs::write(&zip_path, &zip_bytes).expect("write zip");
	let context = ParserContext::new(zip_path.to_string_lossy().to_string());
	let document = DaisyParser.parse(&context).expect("multi-file DAISY parse should succeed");
	assert_eq!(document.title, "Two Section Book");
	assert_eq!(document.author, "A. Author");
	assert!(document.buffer.content.contains("Chapter One"));
	assert!(document.buffer.content.contains("Chapter Two"));
	assert!(
		document.buffer.content.find("Chapter One").unwrap() < document.buffer.content.find("Chapter Two").unwrap()
	);
	assert_eq!(document.toc_items.len(), 2);
	assert_eq!(document.toc_items[1].name, "Chapter Two");
	let audio = document.audio.expect("audio timeline should be populated");
	assert_eq!(audio.clips().len(), 4);
	assert_eq!(audio.sources().len(), 2);
	// The second file's clips resume the elapsed clock where the first file's left off.
	assert_eq!(audio.total_duration_ms(), 2000 + 3000 + 1500 + 2500);
	let h2_pos = document.buffer.content.find("Chapter Two").unwrap();
	let clip_index = audio.clip_index_at_position(h2_pos).expect("chapter two should be narrated");
	assert_eq!(audio.clip_start_ms(clip_index), Some(5000));
}

/// Two chapters can legally reuse the same bare id; the merged map keeps the first file's
/// position while still exposing both via their path-qualified keys.
#[test]
fn multi_file_daisy_book_keeps_first_occurrence_of_a_duplicate_bare_id() {
	let opf = br#"<?xml version="1.0" encoding="UTF-8"?>
<package xmlns="http://openebook.org/namespaces/oeb-package/1.0/" unique-identifier="bookid">
  <metadata>
    <dc-metadata xmlns:dc="http://purl.org/dc/elements/1.1/">
      <dc:Title>Duplicate Id Book</dc:Title>
    </dc-metadata>
  </metadata>
  <manifest>
    <item id="xml1" href="book1.xml" media-type="application/x-dtbook+xml" />
    <item id="xml2" href="book2.xml" media-type="application/x-dtbook+xml" />
  </manifest>
  <spine>
    <itemref idref="xml1" />
    <itemref idref="xml2" />
  </spine>
</package>"#;
	let book1 = br#"<?xml version="1.0" encoding="UTF-8"?>
<dtbook><book><bodymatter><p id="dup">First occurrence.</p></bodymatter></book></dtbook>"#;
	let book2 = br#"<?xml version="1.0" encoding="UTF-8"?>
<dtbook><book><bodymatter><p id="dup">Second occurrence.</p></bodymatter></book></dtbook>"#;
	let zip_bytes =
		write_zip(&[("book.opf", opf.as_slice()), ("book1.xml", book1.as_slice()), ("book2.xml", book2.as_slice())]);
	let dir = TempDir::new("daisy_dup_id");
	let zip_path = dir.path().join("book.zip");
	fs::write(&zip_path, &zip_bytes).expect("write zip");
	let context = ParserContext::new(zip_path.to_string_lossy().to_string());
	let document = DaisyParser.parse(&context).expect("DAISY parse should succeed");
	let first_pos = document.buffer.content.find("First occurrence.").unwrap();
	let second_pos = document.buffer.content.find("Second occurrence.").unwrap();
	assert_eq!(
		document.id_positions.get("dup").copied(),
		Some(first_pos),
		"bare id should keep the first file's position"
	);
	assert_eq!(document.id_positions.get("book1.xml#dup").copied(), Some(first_pos));
	assert_eq!(document.id_positions.get("book2.xml#dup").copied(), Some(second_pos));
}

/// Some OPFs label their DTBook items as generic `text/xml` rather than the proper
/// `application/x-dtbook+xml`. `find_single_dtbook_href`'s legacy fallback already tolerates
/// this for a single-file book; the multi-file spine walk in `build_daisy_document` must too,
/// or every such chapter gets skipped, `converted_any` never becomes true, and the whole book
/// falls back to that same single-file path, which then only recovers the first chapter.
#[test]
fn multi_file_daisy_book_accepts_untyped_xml_chapters_referenced_directly_from_the_spine() {
	let opf = br#"<?xml version="1.0" encoding="UTF-8"?>
<package xmlns="http://openebook.org/namespaces/oeb-package/1.0/" unique-identifier="bookid">
  <metadata>
    <dc-metadata xmlns:dc="http://purl.org/dc/elements/1.1/">
      <dc:Title>Untyped Chapters Book</dc:Title>
    </dc-metadata>
  </metadata>
  <manifest>
    <item id="xml1" href="book1.xml" media-type="text/xml" />
    <item id="xml2" href="book2.xml" media-type="text/xml" />
  </manifest>
  <spine>
    <itemref idref="xml1" />
    <itemref idref="xml2" />
  </spine>
</package>"#;
	let book1 = br#"<?xml version="1.0" encoding="UTF-8"?>
<dtbook><book><bodymatter><h1 id="h1">Chapter One</h1></bodymatter></book></dtbook>"#;
	let book2 = br#"<?xml version="1.0" encoding="UTF-8"?>
<dtbook><book><bodymatter><h1 id="h2">Chapter Two</h1></bodymatter></book></dtbook>"#;
	let zip_bytes =
		write_zip(&[("book.opf", opf.as_slice()), ("book1.xml", book1.as_slice()), ("book2.xml", book2.as_slice())]);
	let dir = TempDir::new("daisy_untyped_xml_chapters");
	let zip_path = dir.path().join("book.zip");
	fs::write(&zip_path, &zip_bytes).expect("write zip");
	let context = ParserContext::new(zip_path.to_string_lossy().to_string());
	let document = DaisyParser.parse(&context).expect("DAISY parse should succeed");
	assert!(document.buffer.content.contains("Chapter One"));
	assert!(document.buffer.content.contains("Chapter Two"), "second chapter should not be silently dropped");
	assert!(
		document.buffer.content.find("Chapter One").unwrap() < document.buffer.content.find("Chapter Two").unwrap()
	);
}

/// A `<text src="#id">` with no file part resolves against the most recent explicit
/// `<text src="file#id">` in the same SMIL, rather than losing that clip.
#[test]
fn smil_bare_fragment_resolves_against_the_last_explicit_text_file() {
	let opf = br#"<?xml version="1.0" encoding="UTF-8"?>
<package xmlns="http://openebook.org/namespaces/oeb-package/1.0/" unique-identifier="bookid">
  <metadata>
    <dc-metadata xmlns:dc="http://purl.org/dc/elements/1.1/">
      <dc:Title>Bare Fragment Book</dc:Title>
    </dc-metadata>
  </metadata>
  <manifest>
    <item id="xml1" href="book1.xml" media-type="application/x-dtbook+xml" />
    <item id="audio1" href="book1.mp3" media-type="audio/mpeg" />
    <item id="smil1" href="section1.smil" media-type="application/smil" />
  </manifest>
  <spine>
    <itemref idref="smil1" />
  </spine>
</package>"#;
	let book1 = br#"<?xml version="1.0" encoding="UTF-8"?>
<dtbook><book><bodymatter>
<p id="p1">First sentence.</p>
<p id="p2">Second sentence.</p>
</bodymatter></book></dtbook>"#;
	let smil1 = br##"<smil xmlns="http://www.w3.org/2001/SMIL20/"><body><seq id="s">
<par id="par_p1"><text src="book1.xml#p1" /><audio src="book1.mp3" clipBegin="0s" clipEnd="2s" /></par>
<par id="par_p2"><text src="#p2" /><audio src="book1.mp3" clipBegin="2s" clipEnd="4s" /></par>
</seq></body></smil>"##;
	let zip_bytes = write_zip(&[
		("book.opf", opf.as_slice()),
		("book1.xml", book1.as_slice()),
		("section1.smil", smil1.as_slice()),
		("book1.mp3", b"fake-mp3"),
	]);
	let dir = TempDir::new("daisy_bare_fragment");
	let zip_path = dir.path().join("book.zip");
	fs::write(&zip_path, &zip_bytes).expect("write zip");
	let context = ParserContext::new(zip_path.to_string_lossy().to_string());
	let document = DaisyParser.parse(&context).expect("DAISY parse should succeed");
	let audio = document.audio.expect("audio timeline should be populated");
	assert_eq!(audio.clips().len(), 2, "the bare-fragment par's clip must not be dropped");
	let p2_pos = document.buffer.content.find("Second sentence.").unwrap();
	let clip_index = audio.clip_index_at_position(p2_pos).expect("second sentence should be narrated");
	assert_eq!(audio.clip_start_ms(clip_index), Some(2000));
}

/// A DAISY 3 NCX points `content/@src` at SMIL ids, not `DTBook` ids. Here the SMIL par
/// ids are deliberately unrelated to the `DTBook` ids (as producers other than Bookshare
/// number them), so resolving against `DTBook` ids alone would strand every entry at 0.
#[test]
fn ncx_targets_resolve_through_smil_par_ids() {
	let opf = br#"<?xml version="1.0" encoding="UTF-8"?>
<package xmlns="http://openebook.org/namespaces/oeb-package/1.0/" unique-identifier="bookid">
  <metadata><dc-metadata xmlns:dc="http://purl.org/dc/elements/1.1/"><dc:Title>Smil Ncx Book</dc:Title></dc-metadata></metadata>
  <manifest>
    <item id="xml1" href="book1.xml" media-type="application/x-dtbook+xml" />
    <item id="ncx" href="book.ncx" media-type="application/x-dtbncx+xml" />
    <item id="audio1" href="book1.mp3" media-type="audio/mpeg" />
    <item id="smil1" href="section1.smil" media-type="application/smil" />
  </manifest>
  <spine><itemref idref="smil1" /></spine>
</package>"#;
	let book1 = br#"<?xml version="1.0" encoding="UTF-8"?>
<dtbook><book><bodymatter>
<h1 id="h1">Chapter One</h1>
<p id="p1">First paragraph.</p>
<h1 id="h2">Chapter Two</h1>
</bodymatter></book></dtbook>"#;
	let smil1 = br#"<smil xmlns="http://www.w3.org/2001/SMIL20/"><body><seq id="baseseq">
<par id="tcp00001"><text src="book1.xml#h1" /><audio src="book1.mp3" clipBegin="0s" clipEnd="2s" /></par>
<par id="tcp00002"><text src="book1.xml#p1" /><audio src="book1.mp3" clipBegin="2s" clipEnd="5s" /></par>
<par id="tcp00003"><text src="book1.xml#h2" /><audio src="book1.mp3" clipBegin="5s" clipEnd="7s" /></par>
</seq></body></smil>"#;
	let ncx = br#"<?xml version="1.0" encoding="UTF-8"?>
<ncx xmlns="http://www.daisy.org/z3986/2005/ncx/"><navMap>
<navPoint id="np1"><navLabel><text>Chapter One</text></navLabel><content src="section1.smil#tcp00001" /></navPoint>
<navPoint id="np2"><navLabel><text>Chapter Two</text></navLabel><content src="section1.smil#tcp00003" /></navPoint>
</navMap></ncx>"#;
	let zip_bytes = write_zip(&[
		("book.opf", opf.as_slice()),
		("book1.xml", book1.as_slice()),
		("book.ncx", ncx.as_slice()),
		("section1.smil", smil1.as_slice()),
		("book1.mp3", b"fake-mp3"),
	]);
	let dir = TempDir::new("daisy_smil_ncx");
	let zip_path = dir.path().join("book.zip");
	fs::write(&zip_path, &zip_bytes).expect("write zip");
	let context = ParserContext::new(zip_path.to_string_lossy().to_string());
	let document = DaisyParser.parse(&context).expect("DAISY parse should succeed");
	assert_eq!(document.toc_items.len(), 2);
	let chapter_two = document.buffer.content.find("Chapter Two").unwrap();
	assert_eq!(document.toc_items[0].offset, document.buffer.content.find("Chapter One").unwrap());
	assert_eq!(document.toc_items[1].offset, chapter_two, "second entry must not be stranded at 0");
}

/// An NCX naming a `<seq>` rather than a `<par>` resolves to where that `<seq>` begins.
#[test]
fn ncx_targets_resolve_through_smil_seq_ids() {
	let opf = br#"<?xml version="1.0" encoding="UTF-8"?>
<package xmlns="http://openebook.org/namespaces/oeb-package/1.0/" unique-identifier="bookid">
  <metadata><dc-metadata xmlns:dc="http://purl.org/dc/elements/1.1/"><dc:Title>Seq Ncx Book</dc:Title></dc-metadata></metadata>
  <manifest>
    <item id="xml1" href="book1.xml" media-type="application/x-dtbook+xml" />
    <item id="ncx" href="book.ncx" media-type="application/x-dtbncx+xml" />
    <item id="audio1" href="book1.mp3" media-type="audio/mpeg" />
    <item id="smil1" href="section1.smil" media-type="application/smil" />
  </manifest>
  <spine><itemref idref="smil1" /></spine>
</package>"#;
	let book1 = br#"<?xml version="1.0" encoding="UTF-8"?>
<dtbook><book><bodymatter>
<h1 id="h1">Chapter One</h1>
<h1 id="h2">Chapter Two</h1>
</bodymatter></book></dtbook>"#;
	let smil1 = br#"<smil xmlns="http://www.w3.org/2001/SMIL20/"><body><seq id="baseseq">
<seq id="chapter1"><par id="p_a"><text src="book1.xml#h1" /><audio src="book1.mp3" clipBegin="0s" clipEnd="2s" /></par></seq>
<seq id="chapter2"><par id="p_b"><text src="book1.xml#h2" /><audio src="book1.mp3" clipBegin="2s" clipEnd="4s" /></par></seq>
</seq></body></smil>"#;
	let ncx = br#"<?xml version="1.0" encoding="UTF-8"?>
<ncx xmlns="http://www.daisy.org/z3986/2005/ncx/"><navMap>
<navPoint id="np2"><navLabel><text>Chapter Two</text></navLabel><content src="section1.smil#chapter2" /></navPoint>
</navMap></ncx>"#;
	let zip_bytes = write_zip(&[
		("book.opf", opf.as_slice()),
		("book1.xml", book1.as_slice()),
		("book.ncx", ncx.as_slice()),
		("section1.smil", smil1.as_slice()),
		("book1.mp3", b"fake-mp3"),
	]);
	let dir = TempDir::new("daisy_seq_ncx");
	let zip_path = dir.path().join("book.zip");
	fs::write(&zip_path, &zip_bytes).expect("write zip");
	let context = ParserContext::new(zip_path.to_string_lossy().to_string());
	let document = DaisyParser.parse(&context).expect("DAISY parse should succeed");
	assert_eq!(document.toc_items.len(), 1);
	assert_eq!(document.toc_items[0].offset, document.buffer.content.find("Chapter Two").unwrap());
}

/// `clipEnd` is optional in SMIL 2.0 and means "to the end of the media". Such a par must
/// keep its audio, bounded by whatever plays next from the same file.
#[test]
fn par_without_clip_end_is_bounded_by_the_next_clip_on_the_same_source() {
	let opf = br#"<?xml version="1.0" encoding="UTF-8"?>
<package xmlns="http://openebook.org/namespaces/oeb-package/1.0/" unique-identifier="bookid">
  <metadata><dc-metadata xmlns:dc="http://purl.org/dc/elements/1.1/"><dc:Title>Open Clip Book</dc:Title></dc-metadata></metadata>
  <manifest>
    <item id="xml1" href="book1.xml" media-type="application/x-dtbook+xml" />
    <item id="audio1" href="book1.mp3" media-type="audio/mpeg" />
    <item id="smil1" href="section1.smil" media-type="application/smil" />
  </manifest>
  <spine><itemref idref="smil1" /></spine>
</package>"#;
	let book1 = br#"<?xml version="1.0" encoding="UTF-8"?>
<dtbook><book><bodymatter>
<p id="p1">First sentence.</p>
<p id="p2">Second sentence.</p>
<p id="p3">Third sentence.</p>
</bodymatter></book></dtbook>"#;
	// p1 has no clipEnd and must be bounded by p2's clipBegin; p3 is the trailing
	// open-ended clip, which has nothing to measure against.
	let smil1 = br#"<smil xmlns="http://www.w3.org/2001/SMIL20/"><body><seq id="s">
<par id="par1"><text src="book1.xml#p1" /><audio src="book1.mp3" clipBegin="0s" /></par>
<par id="par2"><text src="book1.xml#p2" /><audio src="book1.mp3" clipBegin="3s" clipEnd="5s" /></par>
<par id="par3"><text src="book1.xml#p3" /><audio src="book1.mp3" clipBegin="5s" /></par>
</seq></body></smil>"#;
	let zip_bytes = write_zip(&[
		("book.opf", opf.as_slice()),
		("book1.xml", book1.as_slice()),
		("section1.smil", smil1.as_slice()),
		("book1.mp3", b"fake-mp3"),
	]);
	let dir = TempDir::new("daisy_open_clip");
	let zip_path = dir.path().join("book.zip");
	fs::write(&zip_path, &zip_bytes).expect("write zip");
	let context = ParserContext::new(zip_path.to_string_lossy().to_string());
	let document = DaisyParser.parse(&context).expect("DAISY parse should succeed");
	let audio = document.audio.expect("audio timeline should be populated");
	assert_eq!(audio.clips().len(), 2, "the open-ended first par must survive; only the trailing one is dropped");
	assert_eq!(audio.clips()[0].clip_begin_ms, 0);
	assert_eq!(audio.clips()[0].clip_end_ms, 3000, "bounded by the next clip against the same source");
	assert_eq!(audio.total_duration_ms(), 3000 + 2000);
}

/// One unparseable chapter shouldn't cost the reader the rest of the book.
#[test]
fn corrupt_chapter_does_not_abort_the_whole_book() {
	let opf = br#"<?xml version="1.0" encoding="UTF-8"?>
<package xmlns="http://openebook.org/namespaces/oeb-package/1.0/" unique-identifier="bookid">
  <metadata>
    <dc-metadata xmlns:dc="http://purl.org/dc/elements/1.1/">
      <dc:Title>Partly Corrupt Book</dc:Title>
    </dc-metadata>
  </metadata>
  <manifest>
    <item id="xml1" href="book1.xml" media-type="application/x-dtbook+xml" />
    <item id="xml2" href="book2.xml" media-type="application/x-dtbook+xml" />
  </manifest>
  <spine>
    <itemref idref="xml1" />
    <itemref idref="xml2" />
  </spine>
</package>"#;
	let book1 = br#"<?xml version="1.0" encoding="UTF-8"?>
<dtbook><book><bodymatter><p id="p1">Valid chapter text.</p></bodymatter></book></dtbook>"#;
	// Deliberately malformed: unclosed tags fail XML parsing outright.
	let book2 = b"<dtbook><book><bodymatter><p id=\"p2\">Broken chapter";
	let zip_bytes =
		write_zip(&[("book.opf", opf.as_slice()), ("book1.xml", book1.as_slice()), ("book2.xml", book2.as_slice())]);
	let dir = TempDir::new("daisy_corrupt_chapter");
	let zip_path = dir.path().join("book.zip");
	fs::write(&zip_path, &zip_bytes).expect("write zip");
	let context = ParserContext::new(zip_path.to_string_lossy().to_string());
	let document = DaisyParser.parse(&context).expect("DAISY parse should succeed despite one corrupt chapter");
	assert!(document.buffer.content.contains("Valid chapter text."));
}
