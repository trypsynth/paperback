use icu_properties::{
	CodePointMapData,
	props::{BidiClass, BidiMirroringGlyph},
};
use unicode_bidi::{BidiInfo, Level};

/// The `0.5` threshold, half a PDF user-space unit, absorbs the x jitter producers emit for stacked marks so it isn't taken for a left-to-right step.
fn has_visual_rtl_run(chars: &[(char, f32)], bidi: icu_properties::CodePointMapDataBorrowed<BidiClass>) -> bool {
	let mut prev_x: Option<f32> = None;
	for &(c, x) in chars {
		match bidi.get(c) {
			BidiClass::NonspacingMark => {}
			BidiClass::RightToLeft | BidiClass::ArabicLetter => {
				if let Some(px) = prev_x
					&& x > px + 0.5
				{
					return true;
				}
				prev_x = Some(x);
			}
			_ => prev_x = None,
		}
	}
	false
}

/// Lets callers skip fetching x-origins for pure left-to-right runs, the overwhelming majority of text.
pub fn contains_rtl(mut chars: impl Iterator<Item = char>) -> bool {
	let bidi = CodePointMapData::<BidiClass>::new();
	chars.any(|c| matches!(bidi.get(c), BidiClass::RightToLeft | BidiClass::ArabicLetter))
}

struct Cluster {
	ch: char,
	x: f32,
	marks: Vec<char>,
}

/// The output holds exactly the input's scalar values, so its length is unchanged and downstream marker offsets stay valid.
#[must_use]
pub fn reorder_line(chars: &[(char, f32)]) -> String {
	let bidi = CodePointMapData::<BidiClass>::new();
	// Many PDFs (and all faithfully exported vocalized Hebrew) already store RTL text in logical order, where re-deriving order from x can wrongly move the stray spaces producers emit with imprecise x.
	if !has_visual_rtl_run(chars, bidi) {
		return chars.iter().map(|&(c, _)| c).collect();
	}
	// PDFs emit marks adjacent in x but sometimes out of sequence or in the wrong base's slot, so each attaches to the nearest base by x.
	let mut clusters: Vec<Cluster> = Vec::with_capacity(chars.len());
	let mut mark_targets: Vec<(char, f32)> = Vec::new();
	for &(c, x) in chars {
		let bc = bidi.get(c);
		if bc == BidiClass::NonspacingMark {
			mark_targets.push((c, x));
		} else {
			clusters.push(Cluster { ch: c, x, marks: Vec::new() });
		}
	}
	for (mc, mx) in mark_targets {
		// Any non-whitespace base beats whitespace, which is picked only when there is nothing else.
		let dist = |cl: &Cluster| (cl.x - mx).abs();
		let pick = clusters
			.iter()
			.enumerate()
			.min_by(|a, b| a.1.ch.is_whitespace().cmp(&b.1.ch.is_whitespace()).then(dist(a.1).total_cmp(&dist(b.1))))
			.map(|(i, _)| i);
		if let Some(i) = pick {
			clusters[i].marks.push(mc);
		}
	}
	clusters.sort_by(|a, b| a.x.total_cmp(&b.x));
	// Majority rather than UBA's first-strong rule (P2/P3), which picks wrong for a line opening with punctuation or a short word in the other script.
	let (mut rtl, mut ltr) = (0usize, 0usize);
	for cl in &clusters {
		match bidi.get(cl.ch) {
			BidiClass::RightToLeft | BidiClass::ArabicLetter => rtl += 1,
			BidiClass::LeftToRight => ltr += 1,
			_ => {}
		}
	}
	let base_rtl = rtl >= ltr && rtl > 0;
	let base_level = if base_rtl { Level::rtl() } else { Level::ltr() };
	// Visual text is fed to `unicode_bidi` as if logical, which works because reordering a single-embedding paragraph is self-inverse.
	let base_text: String = clusters.iter().map(|cl| cl.ch).collect();
	let bidi_info = BidiInfo::new_with_data_source(&bidi, &base_text, Some(base_level));
	let para = &bidi_info.paragraphs[0];
	let char_levels = bidi_info.reordered_levels_per_char(para, para.range.clone());
	let mirror = CodePointMapData::<BidiMirroringGlyph>::new();
	for (cl, lvl) in clusters.iter_mut().zip(&char_levels) {
		if lvl.is_rtl()
			&& let Some(m) = mirror.get(cl.ch).mirroring_glyph
		{
			cl.ch = m;
		}
	}
	let order = BidiInfo::reorder_visual(&char_levels);
	let mut out = String::with_capacity(chars.len());
	for &idx in &order {
		out.push(clusters[idx].ch);
		for &m in &clusters[idx].marks {
			out.push(m);
		}
	}
	out
}

#[cfg(test)]
mod tests {
	use super::reorder_line;

	#[test]
	fn pure_latin_is_unchanged() {
		let input: Vec<(char, f32)> = "Hello, world. (test)".chars().enumerate().map(|(i, c)| (c, i as f32)).collect();
		assert_eq!(reorder_line(&input), "Hello, world. (test)");
	}

	#[test]
	fn empty_is_empty() {
		assert_eq!(reorder_line(&[]), "");
	}

	#[test]
	fn hebrew_run_stored_visually_is_reversed_to_logical() {
		// "יֹאבַד" captured from a real PDF in visual order, with the holam/patach marks emitted out of place.
		let input = [
			('\u{05D3}', 456.06), // dalet
			('\u{05B7}', 463.25), // patach (belongs to bet)
			('\u{05D0}', 467.55), // aleph
			('\u{05D1}', 461.57), // bet
			('\u{05B9}', 472.73), // holam (belongs to yod)
			('\u{05D9}', 473.80), // yod
		];
		// Logical: yod+holam, aleph, bet+patach, dalet.
		assert_eq!(reorder_line(&input), "\u{05D9}\u{05B9}\u{05D0}\u{05D1}\u{05B7}\u{05D3}");
	}

	#[test]
	fn hebrew_run_stored_logically_is_left_intact() {
		let input = [
			('\u{05E2}', 240.65), // ayin
			('\u{05DE}', 234.02), // mem
			('\u{05D3}', 228.41), // dalet
		];
		assert_eq!(reorder_line(&input), "\u{05E2}\u{05DE}\u{05D3}"); // עמד
	}

	#[test]
	fn arabic_run_is_reversed_to_logical() {
		// "سلام" (peace): s-l-a-m. Stored visually (increasing x) → reverse to logical.
		let input = [
			('\u{0645}', 10.0), // meem  (logical last)
			('\u{0627}', 20.0), // alef
			('\u{0644}', 30.0), // lam
			('\u{0633}', 40.0), // seen  (logical first)
		];
		assert_eq!(reorder_line(&input), "\u{0633}\u{0644}\u{0627}\u{0645}");
	}

	#[test]
	fn arabic_mark_reattaches_to_nearest_base() {
		// seen + fatha(mark) stored with the mark just off the base by x.
		let input = [
			('\u{0628}', 10.0), // beh (logical last)
			('\u{0633}', 20.0), // seen
			('\u{064E}', 20.2), // fatha → nearest base seen
		];
		// logical: seen+fatha, beh
		assert_eq!(reorder_line(&input), "\u{0633}\u{064E}\u{0628}");
	}

	#[test]
	fn already_logical_line_with_stray_space_is_untouched() {
		// A producer-inserted stray space mid-word must not be relocated by an x-sort.
		let input = [
			('\u{05EA}', 100.0), // tav
			('\u{05D5}', 92.0),  // vav
			(' ', 88.0),         // stray artifact space
			('\u{05D4}', 80.0),  // he
		];
		assert_eq!(reorder_line(&input), "\u{05EA}\u{05D5} \u{05D4}");
	}

	#[test]
	fn latin_word_in_rtl_line_keeps_its_order() {
		let input = [
			('\u{05D0}', 100.0), // aleph (rightmost, logical first)
			('\u{05D1}', 90.0),  // bet
			(' ', 80.0),
			('o', 70.0),
			('k', 76.0),
		];
		// logical: aleph, bet, space, o, k
		assert_eq!(reorder_line(&input), "\u{05D0}\u{05D1} ok");
	}
}
