//! Geometry-driven visual→logical reordering for RTL text extracted from PDFs.
//!
//! pdfium returns glyphs in the order they were painted, which for right-to-left
//! scripts (Hebrew, Arabic, …) is frequently *visual* order: the consonants come
//! out reversed and combining marks (niqqud / harakat) are detached from their
//! base letter. Other engines (MuPDF) reorder to logical order; pdfium does not,
//! and exposes no API to do so, so we reconstruct logical order here.
//!
//! Unlike a pure codepoint Unicode Bidi Algorithm pass, we have the **x origin**
//! of every glyph. That is decisive: PDFs are inconsistent about whether a given
//! RTL run was stored visually or logically, and the x coordinate disambiguates
//! them (within an RTL run, logical reading order is right-to-left = descending x).
//! Character classification (strong direction, combining marks, bracket mirroring)
//! comes from `icu_properties` so every RTL script — not just Hebrew — is covered.
//!
//! Scope: this fixes consonant/word **order** and re-attaches combining marks to
//! the base whose x position they match. It deliberately does not try to repair
//! duplicated/biased diacritics that some PDF producers bake into the glyph stream
//! (a generation-time defect every faithful extractor reproduces).

use icu_properties::{
	CodePointMapData,
	props::{BidiClass, BidiMirroringGlyph},
};

/// Resolved coarse direction of a character.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Strong {
	Ltr,
	Rtl,
}

/// True when some maximal run of consecutive RTL-strong base characters is stored
/// left-to-right (x increasing) — i.e. visual order that needs reversing. Combining
/// marks are transparent; any non-RTL strong/neutral/space breaks the current run.
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

const fn strong_dir(bc: BidiClass) -> Option<Strong> {
	match bc {
		BidiClass::LeftToRight => Some(Strong::Ltr),
		BidiClass::RightToLeft | BidiClass::ArabicLetter => Some(Strong::Rtl),
		_ => None,
	}
}

/// One base character plus the combining marks that attach to it, in logical
/// (base-first) order.
struct Cluster {
	ch: char,
	x: f32,
	dir: Option<Strong>,
	is_space: bool,
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
	// Fast path: nothing right-to-left → return input verbatim.
	if !chars.iter().any(|&(c, _)| matches!(bidi.get(c), BidiClass::RightToLeft | BidiClass::ArabicLetter)) {
		return chars.iter().map(|&(c, _)| c).collect();
	}

	// Only reorder when an RTL run is actually stored in *visual* (reversed) order.
	// Many PDFs (and all faithfully-exported vocalized Hebrew) already store RTL
	// text in logical order with x decreasing right-to-left; re-deriving order from
	// x there is a no-op at best and, because producers sprinkle stray spaces with
	// imprecise x, can wrongly relocate them. Leaving already-logical lines byte-for-
	// byte untouched guarantees no regression on correctly-stored documents.
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
			clusters.push(Cluster { ch: c, x, dir: strong_dir(bc), is_space: c.is_whitespace(), marks: Vec::new() });
		}
	}
	if clusters.is_empty() {
		// Marks only (degenerate); preserve input order.
		return chars.iter().map(|&(c, _)| c).collect();
	}
	for (mc, mx) in mark_targets {
		// Nearest non-whitespace cluster; fall back to nearest of any kind.
		let dist = |cl: &Cluster| (cl.x - mx).abs();
		let pick = clusters
			.iter()
			.enumerate()
			.filter(|(_, cl)| !cl.is_space)
			.min_by(|a, b| dist(a.1).total_cmp(&dist(b.1)))
			.or_else(|| clusters.iter().enumerate().min_by(|a, b| dist(a.1).total_cmp(&dist(b.1))))
			.map(|(i, _)| i);
		if let Some(i) = pick {
			clusters[i].marks.push(mc);
		}
	}

	// 2. Sort clusters by ascending x → true visual (left-to-right) order.
	clusters.sort_by(|a, b| a.x.total_cmp(&b.x));

	// 3. Resolve a base/paragraph direction from the strong-character majority.
	let (mut rtl, mut ltr) = (0usize, 0usize);
	for cl in &clusters {
		match cl.dir {
			Some(Strong::Rtl) => rtl += 1,
			Some(Strong::Ltr) => ltr += 1,
			None => {}
		}
	}
	let base_rtl = rtl >= ltr && rtl > 0;
	let base = if base_rtl { Strong::Rtl } else { Strong::Ltr };

	// 4. Resolve neutral runs (UBA N1/N2): a neutral takes the surrounding strong
	//    direction when both sides agree, else the paragraph base direction.
	let n = clusters.len();
	let mut resolved: Vec<Strong> = vec![base; n];
	let mut i = 0;
	let mut prev_strong = base;
	while i < n {
		match clusters[i].dir {
			Some(d) => {
				resolved[i] = d;
				prev_strong = d;
				i += 1;
			}
			None => {
				let start = i;
				while i < n && clusters[i].dir.is_none() {
					i += 1;
				}
				let next_strong = if i < n { clusters[i].dir.unwrap() } else { base };
				let run_dir = if prev_strong == next_strong { prev_strong } else { base };
				for r in resolved.iter_mut().take(i).skip(start) {
					*r = run_dir;
				}
			}
		}
	}

	// 5. Assign embedding levels and apply the Unicode Bidi rule L2: reverse, from
	//    the highest level down to the lowest odd level, every contiguous run whose
	//    level is at or above the current level. With a single embedding this is
	//    levels {0,1} for an LTR base (reverse RTL runs) and {1,2} for an RTL base
	//    (reverse LTR runs, then reverse the whole line).
	let base_level: u8 = u8::from(base_rtl);
	let levels: Vec<u8> = resolved
		.iter()
		.map(|d| match d {
			Strong::Rtl => 1,
			Strong::Ltr => base_level + base_level, // 0 for LTR base, 2 for RTL base
		})
		.collect();

	// Mirror paired punctuation that sits at an odd (right-to-left) level.
	let mirror = CodePointMapData::<BidiMirroringGlyph>::new();
	for (cl, &lvl) in clusters.iter_mut().zip(levels.iter()) {
		if lvl % 2 == 1
			&& let Some(m) = mirror.get(cl.ch).mirroring_glyph
		{
			cl.ch = m;
		}
	}

	let mut order: Vec<usize> = (0..n).collect();
	let max_level = levels.iter().copied().max().unwrap_or(0);
	let mut lvl = max_level;
	while lvl >= 1 {
		let mut k = 0;
		while k < n {
			if levels[order[k]] >= lvl {
				let start = k;
				while k < n && levels[order[k]] >= lvl {
					k += 1;
				}
				order[start..k].reverse();
			} else {
				k += 1;
			}
		}
		lvl -= 1;
	}

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

	/// Helper: build an input line with synthetic descending/ascending x.
	fn line(chars: &[(char, f32)]) -> String {
		reorder_line(chars)
	}

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
		assert_eq!(line(&input), "\u{05D9}\u{05B9}\u{05D0}\u{05D1}\u{05B7}\u{05D3}");
	}

	#[test]
	fn hebrew_run_stored_logically_is_left_intact() {
		// Consonants already emitted right-to-left (descending x): must stay as-is.
		let input = [
			('\u{05E2}', 240.65), // ayin
			('\u{05DE}', 234.02), // mem
			('\u{05D3}', 228.41), // dalet
		];
		assert_eq!(line(&input), "\u{05E2}\u{05DE}\u{05D3}"); // עמד
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
		assert_eq!(line(&input), "\u{0633}\u{0644}\u{0627}\u{0645}");
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
		assert_eq!(line(&input), "\u{0633}\u{064E}\u{0628}");
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
		assert_eq!(line(&input), "\u{05D0}\u{05D1} ok");
	}
}
