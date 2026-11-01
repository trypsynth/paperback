//! The per-font `\fcharsetN` overrides matter because a `\'xx` byte escape decodes under whichever font is active at that point in the document.

use std::{collections::HashMap, str};

use encoding_rs::Encoding;

fn encoding_for_codepage(cpg: i32) -> &'static Encoding {
	match cpg {
		874 => encoding_rs::WINDOWS_874,
		932 => encoding_rs::SHIFT_JIS,
		936 => encoding_rs::GBK,
		949 => encoding_rs::EUC_KR,
		950 => encoding_rs::BIG5,
		1250 => encoding_rs::WINDOWS_1250,
		1251 => encoding_rs::WINDOWS_1251,
		1253 => encoding_rs::WINDOWS_1253,
		1254 => encoding_rs::WINDOWS_1254,
		1255 => encoding_rs::WINDOWS_1255,
		1256 => encoding_rs::WINDOWS_1256,
		1257 => encoding_rs::WINDOWS_1257,
		1258 => encoding_rs::WINDOWS_1258,
		_ => encoding_rs::WINDOWS_1252, // Default per RTF spec
	}
}

pub(super) fn extract_codepage(rtf: &str) -> &'static Encoding {
	if let Some(pos) = rtf.find("\\ansicpg") {
		let after = &rtf[pos + 8..];
		let num_str: String = after.chars().take_while(char::is_ascii_digit).collect();
		if let Ok(cpg) = num_str.parse::<i32>() {
			return encoding_for_codepage(cpg);
		}
	}
	tracing::debug!("no ansicpg control word found in rtf document, defaulting to windows-1252");
	encoding_rs::WINDOWS_1252
}

fn encoding_for_fcharset(charset: i32, default: &'static Encoding) -> &'static Encoding {
	match charset {
		128 => encoding_rs::SHIFT_JIS,
		129 => encoding_rs::EUC_KR,
		134 => encoding_rs::GBK,
		136 => encoding_rs::BIG5,
		161 => encoding_rs::WINDOWS_1253,
		162 => encoding_rs::WINDOWS_1254,
		163 => encoding_rs::WINDOWS_1258,
		177 => encoding_rs::WINDOWS_1255,
		178 => encoding_rs::WINDOWS_1256,
		186 => encoding_rs::WINDOWS_1257,
		204 => encoding_rs::WINDOWS_1251,
		238 => encoding_rs::WINDOWS_1250,
		222 => encoding_rs::WINDOWS_874,
		// ANSI and Symbol use the document default.
		0 | 2 => default,
		_ => {
			tracing::warn!(
				fcharset = charset,
				"unrecognized fcharset value, falling back to document default encoding"
			);
			default
		}
	}
}

pub(super) fn extract_font_table(rtf: &str, default_encoding: &'static Encoding) -> HashMap<u32, &'static Encoding> {
	let mut map = HashMap::new();
	let Some(start) = rtf.find("{\\fonttbl") else {
		tracing::debug!("no font table found in rtf document, per-font encoding is unavailable");
		return map;
	};
	let bytes = rtf.as_bytes();
	let mut depth = 0usize;
	let mut fonttbl_end = start;
	for (i, &b) in bytes[start..].iter().enumerate() {
		match b {
			b'{' => depth += 1,
			b'}' => {
				depth = depth.saturating_sub(1);
				if depth == 0 {
					fonttbl_end = start + i + 1;
					break;
				}
			}
			_ => {}
		}
	}
	let fonttbl = &rtf[start..fonttbl_end];
	let fb = fonttbl.as_bytes();
	// Start at 1 to skip the outer '{' of {\fonttbl} itself; inner {\fN...} entries follow.
	let mut j = 1;
	while j < fb.len() {
		if fb[j] != b'{' {
			j += 1;
			continue;
		}
		let entry_start = j;
		let mut d = 0usize;
		let mut entry_end = j;
		let mut k = j;
		while k < fb.len() {
			match fb[k] {
				b'{' => d += 1,
				b'}' => {
					d = d.saturating_sub(1);
					if d == 0 {
						entry_end = k + 1;
						break;
					}
				}
				_ => {}
			}
			k += 1;
		}
		let entry = &fonttbl[entry_start..entry_end];
		let eb = entry.as_bytes();
		// \fcharset, \fbidi, \froman and the like are \f plus a non-digit, so only the font number matches.
		let mut font_num: Option<u32> = None;
		let mut ei = 0;
		while ei + 2 < eb.len() {
			if eb[ei] == b'\\' && eb[ei + 1] == b'f' && eb[ei + 2].is_ascii_digit() {
				let num_start = ei + 2;
				let mut num_end = num_start;
				while num_end < eb.len() && eb[num_end].is_ascii_digit() {
					num_end += 1;
				}
				if let Some(n) = str::from_utf8(&eb[num_start..num_end]).ok().and_then(|s| s.parse::<u32>().ok()) {
					font_num = Some(n);
					break;
				}
			}
			ei += 1;
		}
		if let Some(fnum) = font_num
			&& let Some(cs_pos) = entry.find("\\fcharset")
		{
			let after = &entry[cs_pos + 9..];
			let num_str: String = after.chars().take_while(char::is_ascii_digit).collect();
			if let Ok(cs) = num_str.parse::<i32>() {
				map.insert(fnum, encoding_for_fcharset(cs, default_encoding));
			}
		}
		j = entry_end.max(j + 1);
	}
	map
}

#[cfg(test)]
mod tests {
	use encoding_rs::Encoding;
	use rstest::rstest;

	use super::{encoding_for_codepage, extract_codepage, extract_font_table};

	fn enc_name(enc: &'static Encoding) -> &'static str {
		enc.name()
	}

	#[rstest]
	#[case(1252, "windows-1252")]
	#[case(1251, "windows-1251")]
	#[case(1258, "windows-1258")]
	#[case(874, "windows-874")]
	#[case(9999, "windows-1252")]
	fn encoding_for_codepage_maps_supported_and_defaults(#[case] codepage: i32, #[case] expected: &str) {
		assert_eq!(enc_name(encoding_for_codepage(codepage)), expected);
	}

	#[rstest]
	#[case("{\\rtf1\\ansi\\ansicpg1251 hello}", "windows-1251")]
	#[case("{\\rtf1\\ansi\\ansicpg1258 hello}", "windows-1258")]
	#[case("{\\rtf1\\ansi\\ansicpgNOTNUM hello}", "windows-1252")]
	#[case("{\\rtf1\\ansi hello}", "windows-1252")]
	fn extract_codepage_reads_ansicpg_when_present(#[case] rtf: &str, #[case] expected: &str) {
		assert_eq!(enc_name(extract_codepage(rtf)), expected);
	}

	#[test]
	fn extract_font_table_maps_fcharset_to_encoding() {
		// Font 1 = ANSI (charset 0 → default), font 2 = CE (charset 238 → Windows-1250)
		let rtf =
			r"{\rtf1\ansi\ansicpg1252{\fonttbl{\f1\fcharset0 Arial;}{\f2\fcharset238 Times New Roman CE;}}\pard hello}";
		let default_enc = encoding_rs::WINDOWS_1252;
		let map = extract_font_table(rtf, default_enc);
		assert_eq!(map.get(&1).map(|e| e.name()), Some("windows-1252"));
		assert_eq!(map.get(&2).map(|e| e.name()), Some("windows-1250"));
	}
}
