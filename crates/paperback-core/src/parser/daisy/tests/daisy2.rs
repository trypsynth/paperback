use std::fs;

use super::*;
use crate::{
	document::ParserContext,
	parser::{Parser, daisy::DaisyParser},
	util::test_support::TempDir,
};

/// Regression test for <https://github.com/trypsynth/paperback/issues/672>: DAISY 2.02
/// "full text, full audio" books (SMIL 1.0) were reported as having no audio at all. Real
/// producers (e.g. `EasePublisher`) write `clip-begin`/`clip-end` rather than SMIL 2.0's
/// `clipBegin`/`clipEnd`, and nest `<audio>` inside a `<seq>` under `<par>` rather than
/// placing it directly under `<par>`; both must be handled for the audio timeline to build
/// at all.
#[test]
fn parses_daisy_2_02_book_with_smil_audio() {
	let ncc = br#"<?xml version="1.0" encoding="UTF-8"?>
<html xmlns="http://www.w3.org/1999/xhtml"><head><title>T</title>
<meta name="dc:title" content="Fire Safety"/>
<meta name="dc:creator" content="Wendy Blaxland"/>
</head><body>
<h1 id="ncc1"><a href="dtb_0001.smil#txt1">Chapter One</a></h1>
<h1 id="ncc2"><a href="dtb_0002.smil#txt2">Chapter Two</a></h1>
</body></html>"#;
	let smil1 = br#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE smil PUBLIC "-//W3C//DTD SMIL 1.0//EN" "smil10.dtd">
<smil><body><seq dur="4s">
<par id="par1" endsync="last"><text src="content.html#h1" id="txt1"/><seq><audio src="ch1.mp3" clip-begin="npt=0.000s" clip-end="npt=2.000s" id="a1"/></seq></par>
<par id="par2" endsync="last"><text src="content.html#p1" id="txt1b"/><seq><audio src="ch1.mp3" clip-begin="npt=2.000s" clip-end="npt=4.000s" id="a2"/></seq></par>
</seq></body></smil>"#;
	let smil2 = br#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE smil PUBLIC "-//W3C//DTD SMIL 1.0//EN" "smil10.dtd">
<smil><body><seq dur="1.5s">
<par id="par3" endsync="last"><text src="content.html#h2" id="txt2"/><seq><audio src="ch2.mp3" clip-begin="npt=0.000s" clip-end="npt=1.500s" id="a3"/></seq></par>
</seq></body></smil>"#;
	let content = br#"<?xml version="1.0" encoding="UTF-8"?>
<html xmlns="http://www.w3.org/1999/xhtml"><body>
<h1 id="h1">Chapter One</h1>
<p id="p1">First paragraph.</p>
<h1 id="h2">Chapter Two</h1>
</body></html>"#;
	let zip_bytes = write_zip(&[
		("ncc.html", ncc.as_slice()),
		("dtb_0001.smil", smil1.as_slice()),
		("dtb_0002.smil", smil2.as_slice()),
		("content.html", content.as_slice()),
		("ch1.mp3", b"fake-mp3-1"),
		("ch2.mp3", b"fake-mp3-2"),
	]);
	let dir = TempDir::new("daisy2_audio");
	let zip_path = dir.path().join("book.zip");
	fs::write(&zip_path, &zip_bytes).expect("write zip");
	let context = ParserContext::new(zip_path.to_string_lossy().to_string());
	let document = DaisyParser.parse(&context).expect("DAISY 2.02 book with SMIL audio should parse");
	assert_eq!(document.title, "Fire Safety", "dc:title from ncc.html should override the path-derived fallback");
	assert_eq!(document.author, "Wendy Blaxland");
	assert!(document.buffer.content.contains("Chapter One"));
	assert!(document.buffer.content.contains("Chapter Two"));
	assert!(document.buffer.content.contains("First paragraph."));
	assert_eq!(document.toc_items.len(), 2);
	assert_eq!(document.toc_items[0].name, "Chapter One");
	assert_eq!(document.toc_items[1].name, "Chapter Two");
	let chapter_two_pos = document.buffer.content.find("Chapter Two").unwrap();
	assert_eq!(document.toc_items[1].offset, chapter_two_pos, "second heading must not be stranded at 0");
	let audio = document.audio.expect("audio timeline should be populated for a DAISY 2.02 book with SMIL audio");
	assert_eq!(audio.clips().len(), 3, "clip-begin/clip-end and audio nested under seq must both be recognized");
	assert_eq!(audio.sources().len(), 2);
	assert_eq!(audio.total_duration_ms(), 2000 + 2000 + 1500);
	let clip_index = audio.clip_index_at_position(chapter_two_pos).expect("chapter two should be narrated");
	assert_eq!(audio.clip_start_ms(clip_index), Some(4000));
}

/// A text-only DAISY 2.02 book (no SMIL, `ncc.html` links straight at content HTML) must
/// still work via the plain-text fallback path, with no audio timeline.
#[test]
fn parses_text_only_daisy_2_02_book_without_smil() {
	let ncc = br#"<?xml version="1.0" encoding="UTF-8"?>
<html xmlns="http://www.w3.org/1999/xhtml"><head><title>T</title>
<meta name="dc:title" content="Text Only Book"/>
</head><body>
<h1 id="ncc1"><a href="content.html#h1">Chapter One</a></h1>
</body></html>"#;
	let content = br#"<?xml version="1.0" encoding="UTF-8"?>
<html xmlns="http://www.w3.org/1999/xhtml"><body>
<h1 id="h1">Chapter One</h1>
<p>Some text with no narration.</p>
</body></html>"#;
	let zip_bytes = write_zip(&[("ncc.html", ncc.as_slice()), ("content.html", content.as_slice())]);
	let dir = TempDir::new("daisy2_text_only");
	let zip_path = dir.path().join("book.zip");
	fs::write(&zip_path, &zip_bytes).expect("write zip");
	let context = ParserContext::new(zip_path.to_string_lossy().to_string());
	let document = DaisyParser.parse(&context).expect("text-only DAISY 2.02 book should still parse");
	assert_eq!(document.title, "Text Only Book");
	assert!(document.buffer.content.contains("Chapter One"));
	assert!(document.buffer.content.contains("Some text with no narration."));
	assert!(document.audio.is_none());
}

/// DAISY 2.02 laid out as loose files (`ncc.html` sitting on disk next to its SMIL/content/
/// audio files, rather than zipped) must work the same way the zip archive path does.
#[test]
fn parses_loose_daisy_2_02_book_with_smil_audio() {
	let ncc = br#"<?xml version="1.0" encoding="UTF-8"?>
<html xmlns="http://www.w3.org/1999/xhtml"><head><title>T</title>
<meta name="dc:title" content="Loose Book"/>
</head><body>
<h1 id="ncc1"><a href="dtb_0001.smil#txt1">Chapter One</a></h1>
</body></html>"#;
	let smil1 = br#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE smil PUBLIC "-//W3C//DTD SMIL 1.0//EN" "smil10.dtd">
<smil><body><seq dur="2s">
<par id="par1" endsync="last"><text src="content.html#h1" id="txt1"/><seq><audio src="ch1.mp3" clip-begin="npt=0.000s" clip-end="npt=2.000s" id="a1"/></seq></par>
</seq></body></smil>"#;
	let content = br#"<?xml version="1.0" encoding="UTF-8"?>
<html xmlns="http://www.w3.org/1999/xhtml"><body><h1 id="h1">Chapter One</h1></body></html>"#;
	let dir = TempDir::new("daisy2_loose_audio");
	fs::write(dir.path().join("ncc.html"), ncc).expect("write ncc.html");
	fs::write(dir.path().join("dtb_0001.smil"), smil1).expect("write smil");
	fs::write(dir.path().join("content.html"), content).expect("write content html");
	fs::write(dir.path().join("ch1.mp3"), b"fake-mp3").expect("write mp3");
	let context = ParserContext::new(dir.path().join("ncc.html").to_string_lossy().to_string());
	let document = DaisyParser.parse(&context).expect("loose DAISY 2.02 book with SMIL audio should parse");
	assert_eq!(document.title, "Loose Book", "dc:title from ncc.html should override the ncc.html-stem fallback");
	assert!(document.buffer.content.contains("Chapter One"));
	let audio = document.audio.expect("audio timeline should be populated");
	assert_eq!(audio.clips().len(), 1);
	assert_eq!(audio.total_duration_ms(), 2000);
}

#[test]
fn zip_with_no_audio_and_no_daisy_markup_still_errors() {
	let zip_bytes = write_zip(&[("notes.txt", b"just some text"), ("cover.jpg", b"not-audio")]);
	let dir = TempDir::new("daisy_no_audio_zip");
	let zip_path = dir.path().join("book.zip");
	fs::write(&zip_path, &zip_bytes).expect("write zip");
	let context = ParserContext::new(zip_path.to_string_lossy().to_string());
	let err = DaisyParser.parse(&context).expect_err("a zip with neither DAISY markup nor audio should still fail");
	assert!(err.to_string().contains("does not appear to be a valid DAISY"));
}
