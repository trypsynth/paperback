//! Reading the content of an `OpenDocument` file, which a password may protect.
//!
//! An encrypted ODF is still an ordinary ZIP, but its `content.xml` is ciphertext, so handing it to an XML parser fails with an error that says nothing about the password the reader is missing. The decryption is odf-crypto's, which follows what `LibreOffice` itself writes: AES-GCM with Argon2id from current versions, AES-CBC from older ones, and Blowfish for ODF 1.1. It works on the whole package, so an encrypted file is turned into a plain package in memory and read from there, while a plain file is read as it stands without going near it.

use std::{
	fs,
	io::{Cursor, Read, Seek},
};

use anyhow::{Context, Result, anyhow};
use odf_crypto::DecryptError;
use zip::ZipArchive;

use crate::{
	parser::PASSWORD_REQUIRED_ERROR_PREFIX,
	t,
	util::{encoding::convert_to_utf8, zip::read_zip_entry_bytes},
};

/// Reads `entry` of the `OpenDocument` file at `path` as text, decrypting the file with `password` first when it is encrypted.
///
/// Returns the `[password_required]` error when the file is encrypted and there is no password, or the password is wrong, so the reader is asked for one rather than told the file is broken.
pub fn read_odf_content(path: &str, entry: &str, password: Option<&str>) -> Result<String> {
	let package = fs::read(path).with_context(|| format!("Failed to open '{path}'"))?;
	let mut archive =
		ZipArchive::new(Cursor::new(package.as_slice())).with_context(|| format!("Failed to read '{path}' as zip"))?;
	if !is_encrypted(&mut archive) {
		return read_entry(&mut archive, entry);
	}
	let plain = decrypt(&package, password)?;
	let mut archive = ZipArchive::new(Cursor::new(plain.as_slice())).context("Failed to read the decrypted package")?;
	read_entry(&mut archive, entry)
}

/// Whether the package says it is encrypted, either as one `encrypted-package` stream or entry by entry in its manifest. A file with neither never reaches the decryption.
fn is_encrypted<R: Read + Seek>(archive: &mut ZipArchive<R>) -> bool {
	archive.by_name("encrypted-package").is_ok()
		|| read_zip_entry_bytes(archive, "META-INF/manifest.xml")
			.is_ok_and(|manifest| convert_to_utf8(&manifest).contains("encryption-data"))
}

fn read_entry<R: Read + Seek>(archive: &mut ZipArchive<R>, entry: &str) -> Result<String> {
	let bytes = read_zip_entry_bytes(archive, entry).with_context(|| format!("failed to read '{entry}'"))?;
	Ok(convert_to_utf8(&bytes))
}

fn decrypt(package: &[u8], password: Option<&str>) -> Result<Vec<u8>> {
	let Some(password) = password.filter(|password| !password.is_empty()) else {
		// TRANSLATORS: Error detail shown when an OpenDocument file is encrypted and no password was given (the internal sentinel prefix before it is not translated)
		return Err(anyhow!("{PASSWORD_REQUIRED_ERROR_PREFIX}{}", t("Password required")));
	};
	odf_crypto::decrypt(package, password).map_err(|error| match error {
		// TRANSLATORS: Error detail shown when the password for an OpenDocument file is wrong (the internal sentinel prefix before it is not translated)
		DecryptError::WrongPassword => anyhow!("{PASSWORD_REQUIRED_ERROR_PREFIX}{}", t("Password incorrect")),
		// Not the password error: no password opens a PGP-encrypted file or one LibreOffice itself would refuse, so asking for another would only go round in circles.
		// TRANSLATORS: Error shown when an OpenDocument file is encrypted in a way Paperback does not support, such as with a PGP key; {} is the technical reason
		other => anyhow!(
			t("This document is encrypted in a way Paperback cannot open: {}").replace("{}", &other.to_string())
		),
	})
}

#[cfg(test)]
mod tests;
