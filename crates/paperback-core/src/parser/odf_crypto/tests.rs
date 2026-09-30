use std::io::Write;

use base64::{Engine, engine::general_purpose::STANDARD};
use flate2::{Compression, write::DeflateEncoder};
use sha2::{Digest, Sha256};

use super::{decrypt_entry, encryption_for};
use crate::parser::PASSWORD_REQUIRED_ERROR_PREFIX;

const PASSWORD: &str = "secret123";
const PLAINTEXT: &str = "<office:body><office:text><text:p>The hidden paragraph.</text:p></office:text></office:body>";

/// Encrypts `plaintext` the way an OpenDocument writer would, and returns the manifest that
/// describes it alongside the ciphertext. This is the file the reader has to be able to open.
fn encrypted_entry(plaintext: &str, password: &str) -> (String, Vec<u8>) {
	use aes::Aes256;
	use cbc::cipher::{BlockModeEncrypt, KeyIvInit, block_padding::NoPadding};

	let mut encoder = DeflateEncoder::new(Vec::new(), Compression::best());
	encoder.write_all(plaintext.as_bytes()).unwrap();
	let compressed = encoder.finish().unwrap();
	let checksum = STANDARD.encode(Sha256::digest(&compressed[..compressed.len().min(1024)]));

	let salt = [7u8; 16];
	let iv = [9u8; 16];
	let iterations = 1000u32;
	let start_key = Sha256::digest(password.as_bytes());
	let mut key = [0u8; 32];
	pbkdf2::pbkdf2_hmac::<sha1::Sha1>(&start_key, &salt, iterations, &mut key);

	let mut padded = compressed.clone();
	padded.resize(compressed.len().div_ceil(16) * 16, 0);
	let block_count = padded.len();
	let ciphertext = cbc::Encryptor::<Aes256>::new_from_slices(&key, &iv)
		.unwrap()
		.encrypt_padded::<NoPadding>(&mut padded, block_count)
		.unwrap()
		.to_vec();

	let manifest = format!(
		r#"<?xml version="1.0" encoding="UTF-8"?>
<manifest:manifest xmlns:manifest="urn:oasis:names:tc:opendocument:xmlns:manifest:1.0">
 <manifest:file-entry manifest:full-path="content.xml" manifest:media-type="text/xml">
  <manifest:encryption-data manifest:checksum-type="SHA256/1K" manifest:checksum="{checksum}">
   <manifest:algorithm manifest:algorithm-name="http://www.w3.org/2001/04/xmlenc#aes256-cbc" manifest:initialisation-vector="{iv}"/>
   <manifest:key-derivation manifest:key-derivation-name="PBKDF2" manifest:key-size="32" manifest:iteration-count="{iterations}" manifest:salt="{salt}"/>
   <manifest:start-key-generation manifest:start-key-generation-name="http://www.w3.org/2000/09/xmldsig#sha256"/>
  </manifest:encryption-data>
 </manifest:file-entry>
</manifest:manifest>"#,
		iv = STANDARD.encode(iv),
		salt = STANDARD.encode(salt),
	);
	(manifest, ciphertext)
}

#[test]
fn the_right_password_recovers_the_entry() {
	let (manifest, ciphertext) = encrypted_entry(PLAINTEXT, PASSWORD);
	let plain =
		decrypt_entry(&manifest, "content.xml", &ciphertext, Some(PASSWORD)).expect("a valid password decrypts");
	let bytes = plain.expect("an encrypted entry yields its bytes");
	assert_eq!(String::from_utf8(bytes).unwrap(), PLAINTEXT);
}

#[test]
fn a_wrong_password_is_the_password_error_not_a_broken_file() {
	let (manifest, ciphertext) = encrypted_entry(PLAINTEXT, PASSWORD);
	let err = decrypt_entry(&manifest, "content.xml", &ciphertext, Some("wrong")).expect_err("a wrong password fails");
	assert!(err.to_string().starts_with(PASSWORD_REQUIRED_ERROR_PREFIX), "got {err}");
}

#[test]
fn no_password_asks_for_one_rather_than_reporting_a_broken_file() {
	let (manifest, ciphertext) = encrypted_entry(PLAINTEXT, PASSWORD);
	let err =
		decrypt_entry(&manifest, "content.xml", &ciphertext, None).expect_err("an encrypted file needs a password");
	assert!(err.to_string().starts_with(PASSWORD_REQUIRED_ERROR_PREFIX), "got {err}");
}

#[test]
fn a_plain_entry_is_left_for_the_caller_to_read() {
	// A manifest that lists the entry with no encryption-data is an ordinary, unencrypted file.
	let manifest = r#"<?xml version="1.0" encoding="UTF-8"?>
<manifest:manifest xmlns:manifest="urn:oasis:names:tc:opendocument:xmlns:manifest:1.0">
 <manifest:file-entry manifest:full-path="content.xml" manifest:media-type="text/xml"/>
</manifest:manifest>"#;
	let result = decrypt_entry(manifest, "content.xml", b"<office:body/>", None).expect("a plain entry is fine");
	assert!(result.is_none(), "a plain entry is left for the caller to read as it stands");
}

#[test]
fn the_manifest_encryption_is_read() {
	let (manifest, _) = encrypted_entry(PLAINTEXT, PASSWORD);
	let encryption = encryption_for(&manifest, "content.xml").expect("the entry is marked encrypted");
	assert_eq!(encryption.iv.len(), 16);
	assert_eq!(encryption.salt.len(), 16);
	assert_eq!(encryption.iterations, 1000);
	assert!(encryption.start_key_sha256, "the manifest names sha256 for its start key");
	assert!(encryption.checksum.is_some(), "the manifest carries a checksum");
}
