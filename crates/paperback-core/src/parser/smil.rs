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
	pub clip_end_ms: u64,
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
	let audio = par.children().find(|n| n.is_element() && n.tag_name().name() == "audio")?;
	let src = text.attribute("src")?;
	let (text_file, text_id) = match src.split_once('#') {
		Some((file, id)) => (if file.is_empty() { None } else { Some(file.to_string()) }, id.to_string()),
		None => (None, src.to_string()),
	};
	let audio_src = audio.attribute("src")?.to_string();
	let clip_begin_ms = audio.attribute("clipBegin").map_or(0, parse_smil_time);
	let clip_end_ms = parse_smil_time(audio.attribute("clipEnd")?);
	Some(SmilPar { text_file, text_id, audio_src, clip_begin_ms, clip_end_ms })
}

/// Parses a SMIL clock value into milliseconds: plain seconds (`"12.345"`), an `s` suffix,
/// an `npt=` prefix, and clock time (`"hh:mm:ss.fff"` / `"mm:ss.fff"`). Unparseable input
/// maps to zero rather than failing the whole book over one bad timestamp.
#[must_use]
pub fn parse_smil_time(raw: &str) -> u64 {
	let mut value = raw.trim();
	if let Some(rest) = value.strip_prefix("npt=") {
		value = rest;
	}
	if let Some(rest) = value.strip_suffix('s') {
		value = rest;
	}
	if value.contains(':') {
		let seconds =
			value.split(':').fold(0.0_f64, |acc, part| acc.mul_add(60.0, part.trim().parse::<f64>().unwrap_or(0.0)));
		return (seconds * 1000.0).round() as u64;
	}
	value.parse::<f64>().map_or(0, |secs| (secs * 1000.0).round() as u64)
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

	#[test]
	fn unparseable_time_is_zero() {
		assert_eq!(parse_smil_time("garbage"), 0);
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
				clip_end_ms: 2500,
			}
		);
		assert_eq!(pars[1].text_id, "p2");
		assert_eq!(pars[1].clip_begin_ms, 2500);
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
