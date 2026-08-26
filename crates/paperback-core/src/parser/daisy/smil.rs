//! Extracts the `<par>` alignments (text id ↔ audio clip) from a DAISY SMIL file.

use roxmltree::{Document as XmlDocument, Node, NodeType, ParsingOptions};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SmilPar {
	/// The text file the `<text src>` names explicitly. A bare `#id` fragment leaves this
	/// `None` for the caller to resolve.
	pub text_file: Option<String>,
	pub text_id: String,
	pub audio_src: String,
	pub clip_begin_ms: u64,
	/// `None` when `clipEnd` is absent, which SMIL 2.0 defines as running to the end of the
	/// media. The caller bounds it against whatever plays next from the same audio file.
	pub clip_end_ms: Option<u64>,
	/// Ids within the SMIL document that name this par's position: its own `id`, its
	/// `<text>`'s `id`, and its ancestors' (`<seq>`, `<a>`, `<body>`). A DAISY 3 NCX points
	/// its `content/@src` at one of these, not at a `DTBook` id, so the caller needs them to
	/// resolve the table of contents. Ancestor ids repeat across every par they contain; the
	/// caller keeps the first, which is where that `<seq>` begins.
	pub anchor_ids: Vec<String>,
}

/// All `<par>` elements with both a `<text>` and an `<audio>` child, in document order.
/// Pars missing either child (image-only pars, unrecorded text) are skipped.
#[must_use]
pub fn parse_smil_pars(content: &str) -> Vec<SmilPar> {
	let Ok(doc) =
		XmlDocument::parse_with_options(content, ParsingOptions { allow_dtd: true, ..ParsingOptions::default() })
	else {
		return Vec::new();
	};
	doc.descendants()
		.filter(|n| n.node_type() == NodeType::Element && n.tag_name().name() == "par")
		.filter_map(par_from_node)
		.collect()
}

fn par_from_node(par: Node) -> Option<SmilPar> {
	let text = par.children().find(|n| n.is_element() && n.tag_name().name() == "text")?;
	// DAISY 2.02 producers commonly nest `<audio>` inside a `<seq>` rather than placing it
	// directly under `<par>`, so this searches descendants, not just direct children.
	let audio = par.descendants().find(|n| n.is_element() && n.tag_name().name() == "audio")?;
	let src = text.attribute("src")?;
	let (text_file, text_id) = match src.split_once('#') {
		Some((file, id)) => (if file.is_empty() { None } else { Some(file.to_string()) }, id.to_string()),
		None => (None, src.to_string()),
	};
	let audio_src = audio.attribute("src")?.to_string();
	// SMIL 1.0 (DAISY 2.02) spells these `clip-begin`/`clip-end`; SMIL 2.0 (DAISY 3) uses
	// `clipBegin`/`clipEnd`.
	let clip_begin_ms =
		audio.attribute("clipBegin").or_else(|| audio.attribute("clip-begin")).map_or(0, parse_smil_time);
	let clip_end_ms = audio.attribute("clipEnd").or_else(|| audio.attribute("clip-end")).map(parse_smil_time);
	let mut anchor_ids: Vec<String> = par
		.attribute("id")
		.into_iter()
		.chain(text.attribute("id"))
		.chain(par.ancestors().skip(1).filter_map(|n| n.attribute("id")))
		.map(str::to_string)
		.collect();
	anchor_ids.dedup();
	Some(SmilPar { text_file, text_id, audio_src, clip_begin_ms, clip_end_ms, anchor_ids })
}

/// Parses a SMIL 2.0 clock value into milliseconds.
///
/// Covers both grammars: a full or partial clock value (`"hh:mm:ss.fff"` / `"mm:ss.fff"`) and
/// a timecount value with any of the defined units (`"12.345"`, `"12.345s"`, `"500ms"`,
/// `"1.5min"`, `"2h"`), optionally behind the `npt=` prefix DAISY inherits from SMIL 1.0. A
/// bare number is seconds. Unparseable input maps to zero rather than failing the whole book
/// over one bad timestamp.
#[must_use]
pub fn parse_smil_time(raw: &str) -> u64 {
	let mut value = raw.trim();
	if let Some(rest) = value.strip_prefix("npt=") {
		value = rest.trim();
	}
	if value.contains(':') {
		let seconds =
			value.split(':').fold(0.0_f64, |acc, part| acc.mul_add(60.0, part.trim().parse::<f64>().unwrap_or(0.0)));
		return scale_to_ms(seconds, 1000.0);
	}
	// Longest suffix first, so "ms" isn't read as the "s" of a seconds value.
	for (suffix, ms_per_unit) in [("ms", 1.0), ("min", 60_000.0), ("h", 3_600_000.0), ("s", 1000.0)] {
		if let Some(rest) = value.strip_suffix(suffix) {
			return rest.trim().parse::<f64>().map_or(0, |count| scale_to_ms(count, ms_per_unit));
		}
	}
	value.parse::<f64>().map_or(0, |secs| scale_to_ms(secs, 1000.0))
}

/// Rounds `count * ms_per_unit` to whole milliseconds, clamping negatives (which a clock
/// value can't legally hold) to zero rather than wrapping around on the cast.
fn scale_to_ms(count: f64, ms_per_unit: f64) -> u64 {
	let ms = (count * ms_per_unit).round();
	if ms > 0.0 { ms as u64 } else { 0 }
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parses_plain_seconds() {
		assert_eq!(parse_smil_time("12.345"), 12345);
		assert_eq!(parse_smil_time("0.0"), 0);
	}

	#[test]
	fn parses_seconds_with_s_suffix() {
		assert_eq!(parse_smil_time("288.0s"), 288_000);
		assert_eq!(parse_smil_time("0s"), 0);
	}

	#[test]
	fn parses_clock_format() {
		assert_eq!(parse_smil_time("00:01:02.500"), 62_500);
		assert_eq!(parse_smil_time("01:00:00"), 3_600_000);
	}

	#[test]
	fn parses_npt_prefix() {
		assert_eq!(parse_smil_time("npt=12.345s"), 12345);
	}

	/// SMIL 2.0's timecount grammar allows `h`, `min`, `ms` as well as `s`. Reading `"500ms"`
	/// as seconds (or failing it to zero) silently mistimes a clip.
	#[test]
	fn parses_every_timecount_unit() {
		assert_eq!(parse_smil_time("500ms"), 500);
		assert_eq!(parse_smil_time("1.5min"), 90_000);
		assert_eq!(parse_smil_time("2h"), 7_200_000);
		assert_eq!(parse_smil_time("2.5s"), 2500);
		assert_eq!(parse_smil_time("npt=250ms"), 250);
	}

	#[test]
	fn unparseable_time_is_zero() {
		assert_eq!(parse_smil_time("garbage"), 0);
		assert_eq!(parse_smil_time(""), 0);
		assert_eq!(parse_smil_time("-5s"), 0);
	}

	#[test]
	fn extracts_pars_with_text_and_audio() {
		let smil = r##"<smil xmlns="http://www.w3.org/2001/SMIL20/">
			<body>
				<seq id="baseseq">
					<par id="p1"><text src="book.xml#p1" region="textRegion" /><audio src="book.mp3" clipBegin="0s" clipEnd="2.5s" /></par>
					<par id="img1"><img src="cover.png" /></par>
					<a href="#x"><par id="p2"><text src="book.xml#p2" /><audio src="book.mp3" clipBegin="2.5" clipEnd="4.0" /></par></a>
				</seq>
			</body>
		</smil>"##;
		let pars = parse_smil_pars(smil);
		assert_eq!(pars.len(), 2);
		assert_eq!(
			pars[0],
			SmilPar {
				text_file: Some("book.xml".to_string()),
				text_id: "p1".to_string(),
				audio_src: "book.mp3".to_string(),
				clip_begin_ms: 0,
				clip_end_ms: Some(2500),
				anchor_ids: vec!["p1".to_string(), "baseseq".to_string()],
			}
		);
		assert_eq!(pars[1].text_id, "p2");
		assert_eq!(pars[1].clip_begin_ms, 2500);
	}

	/// `clipEnd` is `#IMPLIED` in the DAISY 3 SMIL DTD and means "to the end of the media" in
	/// SMIL 2.0, so the par must survive for the caller to bound, not be dropped outright.
	#[test]
	fn absent_clip_end_is_none_rather_than_dropping_the_par() {
		let smil = r#"<smil xmlns="http://www.w3.org/2001/SMIL20/"><body><seq id="s">
			<par id="p1"><text src="book.xml#p1" /><audio src="book.mp3" clipBegin="4s" /></par>
		</seq></body></smil>"#;
		let pars = parse_smil_pars(smil);
		assert_eq!(pars.len(), 1);
		assert_eq!(pars[0].clip_begin_ms, 4000);
		assert_eq!(pars[0].clip_end_ms, None);
	}

	/// A DAISY 3 NCX targets ids in the SMIL, so a par has to report its own id, its
	/// `<text>`'s id, and the ids of the `<seq>`/`<a>` elements containing it.
	#[test]
	fn collects_anchor_ids_from_the_par_its_text_and_its_ancestors() {
		let smil = r#"<smil xmlns="http://www.w3.org/2001/SMIL20/"><body id="bod"><seq id="baseseq">
			<seq id="chapter1">
				<par id="par1"><text src="book.xml#p1" id="txt1" /><audio src="book.mp3" clipBegin="0s" clipEnd="1s" /></par>
			</seq>
		</seq></body></smil>"#;
		let pars = parse_smil_pars(smil);
		assert_eq!(pars.len(), 1);
		assert_eq!(pars[0].anchor_ids, vec!["par1", "txt1", "chapter1", "baseseq", "bod"]);
	}

	#[test]
	fn bare_fragment_leaves_text_file_none() {
		let smil = r##"<smil xmlns="http://www.w3.org/2001/SMIL20/"><body><par id="p1">
			<text src="#p1" /><audio src="book.mp3" clipBegin="0s" clipEnd="1s" /></par></body></smil>"##;
		let pars = parse_smil_pars(smil);
		assert_eq!(pars.len(), 1);
		assert_eq!(pars[0].text_file, None);
	}

	#[test]
	fn par_without_audio_is_skipped() {
		let smil = r#"<smil xmlns="http://www.w3.org/2001/SMIL20/"><body><par id="p1">
			<text src="book.xml#p1" /></par></body></smil>"#;
		assert!(parse_smil_pars(smil).is_empty());
	}
}
