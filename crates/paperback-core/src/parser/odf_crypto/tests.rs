//! The sample files are odf-crypto's own test files, made by `LibreOffice` and Apache `OpenOffice` (MIT OR Apache-2.0).

use rstest::rstest;

use super::read_odf_content;
use crate::{parser::PASSWORD_REQUIRED_ERROR_PREFIX, util::test_support::TempDir};

const PASSWORD: &str = "password";
const NONASCII_PASSWORD: &str = "äbcdefghijklmnopqrstuvwxyz0123456789ABCDEFGHIJKLMNOP";

fn content(name: &str, bytes: &[u8], password: Option<&str>) -> anyhow::Result<String> {
	let dir = TempDir::new("odf-crypto");
	let path = dir.write_str(name, bytes);
	read_odf_content(&path, "content.xml", password)
}

/// AES-GCM with Argon2id is what current `LibreOffice` writes, and none of these but AES-CBC opened before.
#[rstest]
#[case("wholesome.odt", include_bytes!("lo-wholesome-gcm-argon2.odt").as_slice(), PASSWORD, "S6 wholesome GCM+Argon2 golden.")]
#[case("aes-cbc.odt", include_bytes!("lo-legacy-aes-cbc.odt").as_slice(), PASSWORD, "S6 legacy per-entry AES-CBC golden.")]
#[case("blowfish.odt", include_bytes!("aoo-blowfish-pbkdf2.odt").as_slice(), PASSWORD, "S6 classic Blowfish+PBKDF2 golden.")]
#[case("nonascii.odt", include_bytes!("lo-odf11-nonascii-password.odt").as_slice(), NONASCII_PASSWORD, "non-ASCII password.")]
fn every_scheme_libreoffice_writes_opens_with_its_password(
	#[case] name: &str,
	#[case] bytes: &[u8],
	#[case] password: &str,
	#[case] expected: &str,
) {
	let text = content(name, bytes, Some(password)).expect("the right password opens it");
	assert!(text.contains(expected), "{name}: {text}");
}

#[test]
fn a_plain_file_needs_no_password() {
	let text = content("plain.odt", include_bytes!("lo-unencrypted.odt"), None).expect("a plain file opens");
	assert!(text.contains("S1 real unencrypted ODT."), "{text}");
}

#[rstest]
#[case(None)]
#[case(Some(""))]
#[case(Some("wrong"))]
fn a_missing_or_wrong_password_asks_for_one_rather_than_reporting_a_broken_file(#[case] password: Option<&str>) {
	let error =
		content("wholesome.odt", include_bytes!("lo-wholesome-gcm-argon2.odt"), password).expect_err("it stays locked");
	assert!(error.to_string().starts_with(PASSWORD_REQUIRED_ERROR_PREFIX), "{error}");
}
