use encoding_rs::Encoding;

/// The ANSI code page Windows uses for a language, for the languages where that is not Windows-1252.
///
/// A CHM compiled before Unicode holds its title, contents and usually its pages in the code page of the language it was compiled for, and that language is the one thing about its encoding it reliably records. Western languages are left out so their books keep going through the guessing in [`crate::util::encoding::convert_to_utf8`], which also catches the Chinese CHMs compiled under an English language ID.
pub(super) fn encoding_for_lcid(lcid: u32) -> Option<&'static Encoding> {
	// Serbian, Bosnian, Azeri and Uzbek are each written in both Latin and Cyrillic, which only the full LCID tells apart.
	let cyrillic_variant = matches!(lcid, 0x0C1A | 0x1C1A | 0x201A | 0x281A | 0x301A | 0x082C | 0x0843);
	let encoding = match lcid & 0x3FF {
		_ if cyrillic_variant => encoding_rs::WINDOWS_1251,
		0x04 if matches!(lcid, 0x0404 | 0x0C04 | 0x1404) => encoding_rs::BIG5,
		0x04 => encoding_rs::GBK,
		0x11 => encoding_rs::SHIFT_JIS,
		0x12 => encoding_rs::EUC_KR,
		0x1E => encoding_rs::WINDOWS_874,
		0x02 | 0x19 | 0x22 | 0x23 | 0x2F | 0x3F | 0x40 | 0x44 | 0x50 => encoding_rs::WINDOWS_1251,
		0x05 | 0x0E | 0x15 | 0x18 | 0x1A | 0x1B | 0x1C | 0x24 | 0x42 => encoding_rs::WINDOWS_1250,
		0x08 => encoding_rs::WINDOWS_1253,
		0x1F | 0x2C | 0x43 => encoding_rs::WINDOWS_1254,
		0x0D => encoding_rs::WINDOWS_1255,
		0x01 | 0x20 | 0x29 => encoding_rs::WINDOWS_1256,
		0x25..=0x27 => encoding_rs::WINDOWS_1257,
		0x2A => encoding_rs::WINDOWS_1258,
		_ => return None,
	};
	Some(encoding)
}

/// Reads text that came out of HTML entities as Latin-1 back as the bytes of the CHM's own code page.
///
/// HTML Help Workshop writes the contents file of a non-Western book in ASCII, spelling each byte of the code page as the Latin-1 entity with the same number, so `&Iuml;&eth;` in a Russian book is windows-1251 for "Пр", not "Ïð". Text with anything past U+00FF in it was not written that way and is left alone.
pub(super) fn reinterpret_latin1(text: &str, encoding: &'static Encoding) -> Option<String> {
	if text.is_ascii() {
		return None;
	}
	let bytes = text.chars().map(|c| u8::try_from(u32::from(c)).ok()).collect::<Option<Vec<u8>>>()?;
	Some(encoding.decode_without_bom_handling(&bytes).0.into_owned())
}

#[cfg(test)]
mod tests {
	use rstest::rstest;

	use super::*;
	use crate::util::encoding::decode_html;

	#[rstest]
	#[case(0x0419, Some("windows-1251"))]
	#[case(0x0C1A, Some("windows-1251"))]
	#[case(0x081A, Some("windows-1250"))]
	#[case(0x0405, Some("windows-1250"))]
	#[case(0x0804, Some("GBK"))]
	#[case(0x0404, Some("Big5"))]
	#[case(0x0411, Some("Shift_JIS"))]
	#[case(0x0412, Some("EUC-KR"))]
	#[case(0x0408, Some("windows-1253"))]
	#[case(0x0409, None)]
	#[case(0x0407, None)]
	fn maps_language_ids_to_their_code_page(#[case] lcid: u32, #[case] expected: Option<&str>) {
		assert_eq!(encoding_for_lcid(lcid).map(Encoding::name), expected);
	}

	/// The Russian textbook this came from (archive.org item B-001-003-927) declares windows-1251 in every page and Russian as its language, and came out entirely as Windows-1252 mojibake.
	#[test]
	fn a_russian_page_is_decoded_as_cyrillic() {
		let (bytes, _, _) = encoding_rs::WINDOWS_1251.encode("<p>Москва 1994</p>");
		assert_eq!(decode_html(&bytes, encoding_for_lcid(0x0419)), "<p>Москва 1994</p>");
	}

	#[test]
	fn a_page_charset_wins_over_the_book_language() {
		let (bytes, _, _) = encoding_rs::WINDOWS_1253
			.encode("<meta http-equiv=\"Content-Type\" content=\"text/html; charset=windows-1253\"><p>Αθήνα</p>");
		assert!(decode_html(&bytes, encoding_for_lcid(0x0419)).ends_with("<p>Αθήνα</p>"));
	}

	#[test]
	fn a_false_utf8_claim_falls_back_to_the_book_language() {
		let (bytes, _, _) = encoding_rs::WINDOWS_1251.encode("<meta charset=\"utf-8\"><p>Москва</p>");
		assert!(decode_html(&bytes, encoding_for_lcid(0x0419)).ends_with("<p>Москва</p>"));
	}

	#[test]
	fn latin1_entities_are_read_as_the_book_code_page() {
		assert_eq!(reinterpret_latin1("[Ïðåäèñëîâèå]", encoding_rs::WINDOWS_1251).as_deref(), Some("[Предисловие]"));
	}

	#[rstest]
	#[case("Contents")]
	#[case("Предисловие")]
	fn names_not_spelled_in_latin1_are_left_alone(#[case] name: &str) {
		assert_eq!(reinterpret_latin1(name, encoding_rs::WINDOWS_1251), None);
	}

	#[test]
	fn valid_utf8_is_kept_whatever_the_language() {
		assert_eq!(decode_html("Москва".as_bytes(), encoding_for_lcid(0x0804)), "Москва");
	}

	#[test]
	fn an_unknown_language_still_detects_gbk() {
		let (bytes, _, _) = encoding_rs::GBK.encode("你好，世界。这是一个测试。");
		assert_eq!(decode_html(&bytes, None), "你好，世界。这是一个测试。");
	}
}
