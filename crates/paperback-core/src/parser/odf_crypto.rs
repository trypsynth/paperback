//! Reading the one entry of an OpenDocument file that a password protects.
//!
//! An encrypted ODF is still an ordinary ZIP, and its `content.xml` is still stored there under
//! that name. What is stored is ciphertext, though, so reading the entry and handing its bytes to
//! an XML parser fails with a parse error that says nothing about the password the reader is
//! missing. `META-INF/manifest.xml` says outright which entries are encrypted and how, so this
//! reads the manifest first, and where an entry is encrypted, derives the key from the password
//! and turns the ciphertext back into the XML the parser wanted.
//!
//! The scheme is the one OpenDocument 1.2 defines: a start key hashed from the password, a real
//! key derived from it with PBKDF2, AES-256-CBC over the entry, and raw DEFLATE under that. Only
//! that common shape is handled; an entry encrypted any other way is reported as needing a
//! password this build cannot supply rather than decoded wrongly.

use std::io::{Read, Seek};

use aes::Aes256;
use anyhow::{Context, Result};
use base64::{Engine, engine::general_purpose::STANDARD};
use cbc::cipher::{BlockModeDecrypt, KeyIvInit, block_padding::NoPadding};
use roxmltree::Document as XmlDocument;
use sha2::{Digest, Sha256};
use zip::ZipArchive;

use crate::{
	parser::PASSWORD_REQUIRED_ERROR_PREFIX,
	t,
	util::{encoding::convert_to_utf8, zip::read_zip_entry_bytes},
};

type Aes256CbcDec = cbc::Decryptor<Aes256>;

const MANIFEST_NS: &str = "urn:oasis:names:tc:opendocument:xmlns:manifest:1.0";

/// Reads an OpenDocument content entry as text, decrypting it with `password` when the file's
/// manifest marks it encrypted.
///
/// This is the whole encryption story an ODT or ODP reader needs: it reads the manifest and the
/// entry, decrypts where the manifest says to, and hands back UTF-8 either way. A plain file
/// costs only the extra read of its manifest.
pub fn read_odf_content<R: Read + Seek>(
	archive: &mut ZipArchive<R>,
	entry: &str,
	password: Option<&str>,
) -> Result<String> {
	let ciphertext = read_zip_entry_bytes(archive, entry).with_context(|| format!("failed to read '{entry}'"))?;
	// A file with no manifest is not encrypted; its entry is read as it stands.
	let manifest = read_zip_entry_bytes(archive, "META-INF/manifest.xml").map(|bytes| convert_to_utf8(&bytes)).ok();
	let plain = match manifest.as_deref() {
		Some(manifest) => decrypt_entry(manifest, entry, &ciphertext, password)?,
		None => None,
	};
	Ok(plain.map_or_else(|| convert_to_utf8(&ciphertext), |bytes| convert_to_utf8(&bytes)))
}

/// How the manifest says one entry is encrypted, read from its `encryption-data`.
struct Encryption {
	iv: Vec<u8>,
	salt: Vec<u8>,
	iterations: u32,
	key_size: usize,
	/// The digest the password is hashed through before key derivation. Newer files use SHA-256,
	/// older ones SHA-1.
	start_key_sha256: bool,
	/// The digest and length the checksum in the manifest was written with, when this build knows
	/// it. `None` leaves the password to be judged by whether the data decodes.
	checksum: Option<Vec<u8>>,
}

/// Whether the file's manifest marks `entry` as encrypted, and if so how.
///
/// `manifest_xml` is the text of `META-INF/manifest.xml`. Returns `None` when the entry is stored
/// in the clear, which is every ODF a word processor writes without a password.
fn encryption_for(manifest_xml: &str, entry: &str) -> Option<Encryption> {
	let doc = XmlDocument::parse(manifest_xml).ok()?;
	let file_entry = doc
		.descendants()
		.find(|node| node.tag_name().name() == "file-entry" && attr(node, "full-path") == Some(entry))?;
	let data = file_entry.children().find(|node| node.tag_name().name() == "encryption-data")?;
	let algorithm = data.children().find(|node| node.tag_name().name() == "algorithm")?;
	let derivation = data.children().find(|node| node.tag_name().name() == "key-derivation")?;
	let start = data.children().find(|node| node.tag_name().name() == "start-key-generation");
	let iv = STANDARD.decode(attr(&algorithm, "initialisation-vector")?).ok()?;
	let salt = STANDARD.decode(attr(&derivation, "salt")?).ok()?;
	let iterations = attr(&derivation, "iteration-count")?.parse().ok()?;
	let key_size = attr(&derivation, "key-size").and_then(|s| s.parse().ok()).unwrap_or(32);
	// A start-key element that names SHA-1, or its absence, both mean the pre-1.2 SHA-1 start key.
	let start_key_sha256 =
		start.and_then(|node| attr(&node, "start-key-generation-name")).is_some_and(|name| name.contains("sha256"));
	let checksum =
		attr(&file_entry, "checksum").or_else(|| attr(&data, "checksum")).and_then(|value| STANDARD.decode(value).ok());
	Some(Encryption { iv, salt, iterations, key_size, start_key_sha256, checksum })
}

/// Reads the value of a manifest attribute, which carry the manifest namespace.
fn attr<'a>(node: &roxmltree::Node<'a, '_>, name: &str) -> Option<&'a str> {
	node.attributes().find(|a| a.name() == name && a.namespace().is_none_or(|ns| ns == MANIFEST_NS)).map(|a| a.value())
}

/// Turns an encrypted ODF entry back into its bytes, given the manifest and the entry's stored
/// ciphertext.
///
/// Returns `Ok(None)` when the manifest does not mark the entry as encrypted, so the caller uses
/// the bytes as they are. Returns the `[password_required]` error when there is no password, when
/// it is wrong, or when the entry is encrypted a way this does not handle: the reader is then told
/// to supply a password rather than that the file is broken.
pub fn decrypt_entry(
	manifest_xml: &str,
	entry: &str,
	ciphertext: &[u8],
	password: Option<&str>,
) -> Result<Option<Vec<u8>>> {
	let Some(encryption) = encryption_for(manifest_xml, entry) else {
		return Ok(None);
	};
	let Some(password) = password else {
		// TRANSLATORS: Error detail shown when an OpenDocument file is encrypted and no password was given (the internal sentinel prefix before it is not translated)
		anyhow::bail!("{PASSWORD_REQUIRED_ERROR_PREFIX}{}", t("Password required"));
	};
	match decrypt(&encryption, ciphertext, password) {
		Some(plain) => Ok(Some(plain)),
		// TRANSLATORS: Error detail shown when the password for an OpenDocument file is wrong (the internal sentinel prefix before it is not translated)
		None => anyhow::bail!("{PASSWORD_REQUIRED_ERROR_PREFIX}{}", t("Password incorrect")),
	}
}

/// The decryption itself. `None` for a wrong password or a shape this cannot read, which the
/// caller turns into the password error; the two are not told apart because the reader does the
/// same thing about either, which is to try another password.
fn decrypt(encryption: &Encryption, ciphertext: &[u8], password: &str) -> Option<Vec<u8>> {
	if ciphertext.is_empty() || !ciphertext.len().is_multiple_of(16) || encryption.iv.len() != 16 {
		return None;
	}
	let start_key: Vec<u8> = if encryption.start_key_sha256 {
		Sha256::digest(password.as_bytes()).to_vec()
	} else {
		sha1::Sha1::digest(password.as_bytes()).to_vec()
	};
	let mut key = vec![0u8; encryption.key_size];
	pbkdf2::pbkdf2_hmac::<sha1::Sha1>(&start_key, &encryption.salt, encryption.iterations, &mut key);
	if key.len() != 32 {
		// Only AES-256 is handled, so a manifest asking for another key length is one to decline.
		return None;
	}
	let cipher = Aes256CbcDec::new_from_slices(&key, &encryption.iv).ok()?;
	// The entry is padded to the block size with bytes that are not PKCS#7, so the padding is left
	// on: raw DEFLATE stops at its own end marker and never reads into it.
	let mut buffer = ciphertext.to_vec();
	let plain = cipher.decrypt_padded::<NoPadding>(&mut buffer).ok()?;
	let inflated = inflate(plain)?;
	// The checksum confirms the password where the manifest carries one this build knows. It is
	// taken over the first kilobyte of the compressed-but-decrypted data, so it can only be
	// checked once that much has been recovered; a shorter entry is judged by the inflate alone.
	if let Some(expected) = &encryption.checksum
		&& plain.len() >= 1024
		&& Sha256::digest(&plain[..1024]).as_slice() != expected.as_slice()
	{
		return None;
	}
	Some(inflated)
}

/// Raw DEFLATE inflate, which is what ODF compresses each entry with before encrypting. A wrong
/// password turns the ciphertext into noise that does not inflate, so a failure here reads as a
/// wrong password rather than a broken file.
fn inflate(data: &[u8]) -> Option<Vec<u8>> {
	use std::io::Read;

	use flate2::read::DeflateDecoder;
	let mut out = Vec::new();
	DeflateDecoder::new(data).read_to_end(&mut out).ok().filter(|_| !out.is_empty())?;
	Some(out)
}

#[cfg(test)]
mod tests;
