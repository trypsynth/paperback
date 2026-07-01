use icu_properties::{
	CodePointMapData,
	props::{BidiClass, BidiMirroringGlyph},
};
use unicode_bidi::{BidiInfo, Level};

/// True when some maximal run of consecutive RTL-strong base characters is stored
/// left-to-right (x increasing) — i.e. visual order that needs reversing. Combining
/// marks are transparent; any non-RTL strong/neutral/space breaks the current run.
/// Also covers the "no RTL character at all" case (the loop simply never enters the
/// RTL branch), so callers need no separate "any RTL" pre-check.
///
/// The `0.5` threshold is half a PDF user-space unit (typically 1/72 inch at the
/// default, unscaled CTM): glyph x-origins within that distance are treated as the
/// "same position" jitter PDF producers emit for stacked/overlapping marks, not a
/// genuine left-to-right step.
fn has_visual_rtl_run(chars: &[(char, f32)], bidi: icu_properties::CodePointMapDataBorrowed<BidiClass>) -> bool {
	let mut prev_x: Option<f32> = None;
	for &(c, x) in chars {
		match bidi.get(c) {
			BidiClass::NonspacingMark => {} // transparent
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

/// Cheap presence check for whether `chars` contains any strong RTL character,
/// with no x-origin needed. Callers use this to skip fetching origins entirely
/// for runs that turn out to be pure LTR — the overwhelming majority of text.
pub fn contains_rtl(mut chars: impl Iterator<Item = char>) -> bool {
	let bidi = CodePointMapData::<BidiClass>::new();
	chars.any(|c| matches!(bidi.get(c), BidiClass::RightToLeft | BidiClass::ArabicLetter))
}

/// One base character plus the combining marks that attach to it, in logical
/// (base-first) order.
struct Cluster {
	ch: char,
	x: f32,
	marks: Vec<char>,
}

/// Reorder one visually-ordered line of `(char, x_origin)` pairs (as pdfium emits
/// them) into logical order. Returns the line unchanged when it contains no
/// right-to-left character, so left-to-right documents are byte-for-byte identical
/// and pay only a single cheap scan.
///
/// The returned string contains exactly the same multiset of scalar values as the
/// input (marks re-attached, clusters reordered, mirrored brackets swap for their
/// pair), so its display length is unchanged and downstream marker offsets stay
/// valid.
#[must_use]
pub fn reorder_line(chars: &[(char, f32)]) -> String {
	let bidi = CodePointMapData::<BidiClass>::new();

	// Fast path, covering both "nothing right-to-left" and "RTL present but already
	// stored in logical order": return input verbatim, in a single scan. Many PDFs
	// (and all faithfully-exported vocalized Hebrew) already store RTL text in
	// logical order with x decreasing right-to-left; re-deriving order from x there
	// is a no-op at best and, because producers sprinkle stray spaces with imprecise
	// x, can wrongly relocate them. Leaving already-logical lines byte-for-byte
	// untouched guarantees no regression on correctly-stored documents.
	if !has_visual_rtl_run(chars, bidi) {
		return chars.iter().map(|&(c, _)| c).collect();
	}

	// 1. Split into base clusters and combining marks. A mark attaches to the
	//    nearest non-whitespace base by x origin (PDFs emit marks adjacent in x but
	//    sometimes out of sequence, and occasionally in the wrong base's slot).
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
		// Nearest non-whitespace cluster; fall back to nearest of any kind
		// (whitespace sorts after non-whitespace at equal distance, for free).
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

	// 2. Sort clusters by ascending x → true visual (left-to-right) order.
	clusters.sort_by(|a, b| a.x.total_cmp(&b.x));

	// 3. Determine the paragraph base direction from the strong-character majority,
	//    overriding the Unicode Bidi Algorithm's own first-strong-character (P2/P3)
	//    detection: a line that opens with punctuation or a short embedded word in
	//    the other script would otherwise pick the wrong base direction for the
	//    dominant script.
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

	// 4. Resolve per-character embedding levels (UBA rules N0-N3, L1), delegated to
	//    `unicode_bidi`. The x-sorted cluster text is fed in as if it were logical
	//    order: resolving levels and reordering (L2) a single-embedding paragraph is
	//    a self-inverse transform, so running the standard logical→visual reorder
	//    over this pretend-logical (actually visual) text recovers the true logical
	//    order.
	let base_text: String = clusters.iter().map(|cl| cl.ch).collect();
	let bidi_info = BidiInfo::new_with_data_source(&bidi, &base_text, Some(base_level));
	let para = &bidi_info.paragraphs[0];
	let char_levels = bidi_info.reordered_levels_per_char(para, para.range.clone());

	// Mirror paired punctuation that sits at an odd (right-to-left) level.
	let mirror = CodePointMapData::<BidiMirroringGlyph>::new();
	for (cl, lvl) in clusters.iter_mut().zip(&char_levels) {
		if lvl.is_rtl()
			&& let Some(m) = mirror.get(cl.ch).mirroring_glyph
		{
			cl.ch = m;
		}
	}

	// 5. Apply the Unicode Bidi rule L2 (reverse, from the highest level down to the
	//    lowest odd level, every contiguous run whose level is at or above the
	//    current level) to recover the logical order.
	let order = BidiInfo::reorder_visual(&char_levels);

	// 6. Emit clusters in final order: each base followed by its marks.
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
		// "יֹאבַד" captured from the PDF in visual order (increasing x within the run),
		// with the holam/patach marks emitted out of place. x values from the probe.
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
		// Consonants already emitted right-to-left (descending x): must stay as-is.
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
		// Logical RTL emission (x strictly decreasing) with a producer-inserted stray
		// space mid-word. Must be returned byte-for-byte so well-formed vocalized
		// Hebrew is never disturbed (no x-sort relocating the space).
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
		// RTL base line with an embedded Latin word "ok"; the Latin must not reverse.
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
