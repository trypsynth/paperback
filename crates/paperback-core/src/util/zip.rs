use std::{
	fs::{self, File},
	io::{self, Read, Seek},
	path::Path,
};

use anyhow::{Context, Result};
use zip::{ZipArchive, result::ZipError};

use crate::{parser::PASSWORD_REQUIRED_ERROR_PREFIX, t, util::encoding::convert_to_utf8};

pub fn read_zip_entry_by_name<R: Read + Seek>(archive: &mut ZipArchive<R>, name: &str) -> Result<String> {
	read_zip_entry_by_name_with_password(archive, name, None)
}

pub fn read_zip_entry_by_name_with_password<R: Read + Seek>(
	archive: &mut ZipArchive<R>,
	name: &str,
	password: Option<&str>,
) -> Result<String> {
	let mut entry = match password {
		Some(pass) => match archive.by_name_decrypt(name, pass.as_bytes()) {
			Ok(e) => e,
			Err(ZipError::UnsupportedArchive(msg)) if msg == ZipError::PASSWORD_REQUIRED => {
				// TRANSLATORS: Error detail shown when a password-protected ZIP-based document needs a password (the internal sentinel prefix before it is not translated)
				anyhow::bail!("{PASSWORD_REQUIRED_ERROR_PREFIX}{}", t("Password required"));
			}
			Err(ZipError::InvalidPassword) => {
				// TRANSLATORS: Error detail shown when the password for a ZIP-based document is wrong (the internal sentinel prefix before it is not translated)
				anyhow::bail!("{PASSWORD_REQUIRED_ERROR_PREFIX}{}", t("Password incorrect"));
			}
			Err(e) => return Err(e.into()),
		},
		None => match archive.by_name(name) {
			Ok(e) => e,
			Err(ZipError::UnsupportedArchive(msg)) if msg == ZipError::PASSWORD_REQUIRED => {
				// TRANSLATORS: Error detail shown when a password-protected ZIP-based document needs a password (the internal sentinel prefix before it is not translated)
				anyhow::bail!("{PASSWORD_REQUIRED_ERROR_PREFIX}{}", t("Password required"));
			}
			Err(e) => return Err(e.into()),
		},
	};
	let mut contents = Vec::new();
	entry.read_to_end(&mut contents).with_context(|| format!("Failed to read entry '{name}'"))?;
	Ok(convert_to_utf8(&contents))
}

/// Reads a zip entry's raw bytes, unlike `read_zip_entry_by_name` which assumes text and
/// converts it to UTF-8. For binary payloads such as audio clips.
pub fn read_zip_entry_bytes<R: Read + Seek>(archive: &mut ZipArchive<R>, name: &str) -> Result<Vec<u8>> {
	let mut entry = archive.by_name(name).with_context(|| format!("Failed to get entry '{name}'"))?;
	let mut contents = Vec::new();
	entry.read_to_end(&mut contents).with_context(|| format!("Failed to read entry '{name}'"))?;
	Ok(contents)
}

pub fn extract_zip_entry_to_file<R: Read + Seek>(
	archive: &mut ZipArchive<R>,
	name: &str,
	output_path: &Path,
) -> Result<()> {
	extract_zip_entry_to_file_with_password(archive, name, output_path, None)
}

pub fn extract_zip_entry_to_file_with_password<R: Read + Seek>(
	archive: &mut ZipArchive<R>,
	name: &str,
	output_path: &Path,
	password: Option<&str>,
) -> Result<()> {
	let mut entry = match password {
		Some(pass) => {
			archive.by_name_decrypt(name, pass.as_bytes()).with_context(|| format!("Failed to get entry '{name}'"))?
		}
		None => archive.by_name(name).with_context(|| format!("Failed to get entry '{name}'"))?,
	};
	if let Some(parent) = output_path.parent() {
		fs::create_dir_all(parent).with_context(|| format!("Failed to create directory '{}'", parent.display()))?;
	}
	let mut out_file =
		File::create(output_path).with_context(|| format!("Failed to create file '{}'", output_path.display()))?;
	io::copy(&mut entry, &mut out_file).with_context(|| format!("Failed to extract entry '{name}'"))?;
	Ok(())
}

/// Extracts every entry of `archive` for which `skip` returns `false` into
/// `output_dir`, preserving the archive's internal directory structure so that
/// relative references between entries (e.g. an XHTML file's
/// `<img src="../images/foo.jpg">`) keep resolving once extracted. Entries
/// whose name would escape `output_dir` are always skipped.
pub fn extract_zip_to_dir<R: Read + Seek>(
	archive: &mut ZipArchive<R>,
	output_dir: &Path,
	skip: impl Fn(&Path) -> bool,
) -> Result<()> {
	for i in 0..archive.len() {
		let mut entry = archive.by_index(i).with_context(|| format!("Failed to get entry at index {i}"))?;
		let Some(enclosed) = entry.enclosed_name() else { continue };
		if !entry.is_dir() && skip(&enclosed) {
			continue;
		}
		let output_path = output_dir.join(enclosed);
		if entry.is_dir() {
			fs::create_dir_all(&output_path)
				.with_context(|| format!("Failed to create directory '{}'", output_path.display()))?;
			continue;
		}
		if let Some(parent) = output_path.parent() {
			fs::create_dir_all(parent).with_context(|| format!("Failed to create directory '{}'", parent.display()))?;
		}
		let mut out_file =
			File::create(&output_path).with_context(|| format!("Failed to create file '{}'", output_path.display()))?;
		io::copy(&mut entry, &mut out_file)
			.with_context(|| format!("Failed to extract entry '{}'", output_path.display()))?;
	}
	Ok(())
}

#[cfg(test)]
mod tests {
	use std::io::{Cursor, Write};

	use zip::{ZipWriter, write::FileOptions};

	use super::*;
	use crate::util::test_support::TempDir;

	fn build_test_archive() -> ZipArchive<Cursor<Vec<u8>>> {
		let mut cursor = Cursor::new(Vec::new());
		{
			let mut writer = ZipWriter::new(&mut cursor);
			writer.start_file("foo.txt", FileOptions::<()>::default()).expect("start file");
			writer.write_all(b"hello world").expect("write file");
			writer.start_file("nested/bar.txt", FileOptions::<()>::default()).expect("start file");
			writer.write_all(b"nested").expect("write file");
			writer.finish().expect("finish zip");
		}
		cursor.set_position(0);
		ZipArchive::new(cursor).expect("open zip")
	}

	#[test]
	fn read_zip_entry_by_name_reads_contents() {
		let mut archive = build_test_archive();
		let contents = read_zip_entry_by_name(&mut archive, "foo.txt").expect("read entry");
		assert_eq!(contents, "hello world");
	}

	#[test]
	fn read_zip_entry_by_name_reports_missing_entry() {
		let mut archive = build_test_archive();
		assert!(read_zip_entry_by_name(&mut archive, "missing.txt").is_err());
	}

	#[test]
	fn read_zip_entry_bytes_reads_raw_contents() {
		let mut archive = build_test_archive();
		let contents = read_zip_entry_bytes(&mut archive, "foo.txt").expect("read entry");
		assert_eq!(contents, b"hello world");
	}

	#[test]
	fn read_zip_entry_bytes_reports_missing_entry() {
		let mut archive = build_test_archive();
		assert!(read_zip_entry_bytes(&mut archive, "missing.txt").is_err());
	}

	#[test]
	fn extract_zip_entry_to_file_writes_to_nested_path() {
		let mut archive = build_test_archive();
		let dir = TempDir::new("zip");
		let output_path = dir.path().join("nested/out.txt");
		extract_zip_entry_to_file(&mut archive, "nested/bar.txt", &output_path).expect("extract entry");
		let contents = fs::read_to_string(&output_path).expect("read output");
		assert_eq!(contents, "nested");
	}

	#[test]
	fn extract_zip_entry_to_file_reports_missing_entry() {
		let mut archive = build_test_archive();
		let dir = TempDir::new("zip");
		let output_path = dir.path().join("nested/missing.txt");
		assert!(extract_zip_entry_to_file(&mut archive, "does-not-exist.txt", &output_path).is_err());
	}

	#[test]
	fn extract_zip_entry_to_file_overwrites_existing_file_contents() {
		let mut archive = build_test_archive();
		let dir = TempDir::new("zip");
		let output_path = dir.path().join("nested/overwrite.txt");
		if let Some(parent) = output_path.parent() {
			fs::create_dir_all(parent).expect("create parent");
		}
		fs::write(&output_path, "old").expect("seed file");
		extract_zip_entry_to_file(&mut archive, "foo.txt", &output_path).expect("extract entry");
		let contents = fs::read_to_string(&output_path).expect("read output");
		assert_eq!(contents, "hello world");
	}

	fn build_encrypted_test_archive() -> ZipArchive<Cursor<Vec<u8>>> {
		let mut cursor = Cursor::new(Vec::new());
		{
			let mut writer = ZipWriter::new(&mut cursor);
			let options = FileOptions::<()>::default().with_aes_encryption(zip::AesMode::Aes256, "hunter2");
			writer.start_file("secret.mp3", options).expect("start file");
			writer.write_all(b"secret-audio-bytes").expect("write file");
			writer.finish().expect("finish zip");
		}
		cursor.set_position(0);
		ZipArchive::new(cursor).expect("open zip")
	}

	#[test]
	fn extract_zip_entry_to_file_with_password_decrypts_with_the_right_password() {
		let mut archive = build_encrypted_test_archive();
		let dir = TempDir::new("zip");
		let output_path = dir.path().join("secret.mp3");
		extract_zip_entry_to_file_with_password(&mut archive, "secret.mp3", &output_path, Some("hunter2"))
			.expect("extract entry");
		let contents = fs::read(&output_path).expect("read output");
		assert_eq!(contents, b"secret-audio-bytes");
	}

	#[test]
	fn extract_zip_entry_to_file_with_password_rejects_the_wrong_password() {
		let mut archive = build_encrypted_test_archive();
		let dir = TempDir::new("zip");
		let output_path = dir.path().join("secret.mp3");
		assert!(
			extract_zip_entry_to_file_with_password(&mut archive, "secret.mp3", &output_path, Some("wrong")).is_err()
		);
	}

	#[test]
	fn extract_zip_entry_to_file_with_password_reports_a_missing_password() {
		let mut archive = build_encrypted_test_archive();
		let dir = TempDir::new("zip");
		let output_path = dir.path().join("secret.mp3");
		assert!(extract_zip_entry_to_file_with_password(&mut archive, "secret.mp3", &output_path, None).is_err());
	}

	#[test]
	fn read_zip_entry_by_name_reads_nested_entry() {
		let mut archive = build_test_archive();
		let contents = read_zip_entry_by_name(&mut archive, "nested/bar.txt").expect("read nested entry");
		assert_eq!(contents, "nested");
	}
}
