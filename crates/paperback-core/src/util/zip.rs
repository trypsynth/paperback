use std::{
	collections::{HashMap, HashSet},
	fs::{self, File},
	io::{self, BufReader, Read, Seek},
	path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use sha1::{Digest, Sha1};
use zip::{ZipArchive, result::ZipError};

use crate::{
	parser::{PASSWORD_REQUIRED_ERROR_PREFIX, parser_supports_path},
	t,
	util::encoding::decode_html,
};

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
	Ok(decode_html(&contents, None))
}

/// Reads a zip entry's raw bytes, unlike `read_zip_entry_by_name` which assumes text and
/// converts it to UTF-8. For binary payloads such as audio clips.
pub fn read_zip_entry_bytes<R: Read + Seek>(archive: &mut ZipArchive<R>, name: &str) -> Result<Vec<u8>> {
	read_zip_entry_bytes_with_password(archive, name, None)
}

/// [`read_zip_entry_bytes`], but for an entry that may be AES-encrypted.
pub fn read_zip_entry_bytes_with_password<R: Read + Seek>(
	archive: &mut ZipArchive<R>,
	name: &str,
	password: Option<&str>,
) -> Result<Vec<u8>> {
	let mut entry = match password {
		Some(pass) => {
			archive.by_name_decrypt(name, pass.as_bytes()).with_context(|| format!("Failed to get entry '{name}'"))?
		}
		None => archive.by_name(name).with_context(|| format!("Failed to get entry '{name}'"))?,
	};
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

/// Returns a function mapping a reference to the archive entry it names, matching case-insensitively when no entry has the exact name. Books are often written against a case-insensitive filesystem, so a reference to `01.Mp3` that plays fine once extracted on Windows or macOS has to find the zip's `01.mp3` too. A reference matching nothing comes back unchanged, so the lookup still reports it missing.
pub fn zip_entry_name_resolver<R: Read + Seek>(archive: &ZipArchive<R>) -> impl Fn(&str) -> String + use<R> {
	let names: HashSet<String> = archive.file_names().flatten().map(String::from).collect();
	let mut lowercase = HashMap::new();
	for name in &names {
		lowercase.entry(name.to_lowercase()).or_insert_with(|| name.clone());
	}
	move |reference: &str| {
		if names.contains(reference) {
			return reference.to_string();
		}
		lowercase.get(&reference.to_lowercase()).cloned().unwrap_or_else(|| reference.to_string())
	}
}

/// One file inside an archive, as shown by the "browse a zip" picker.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZipEntryInfo {
	/// The entry's name exactly as stored, so it can be passed back to the extract helpers.
	pub name: String,
	pub size: u64,
	/// Whether a parser can open this entry.
	pub supported: bool,
}

/// Lists the files in `archive`, skipping directories and any entry whose name would escape an
/// extraction directory. Reads only the central directory, so encrypted entries are listed too.
pub fn list_zip_entries<R: Read + Seek>(archive: &mut ZipArchive<R>) -> Result<Vec<ZipEntryInfo>> {
	let mut entries = Vec::new();
	for i in 0..archive.len() {
		let entry = archive.by_index_raw(i).with_context(|| format!("Failed to get entry at index {i}"))?;
		let Ok(name) = entry.name() else { continue };
		if entry.is_dir() || entry.enclosed_name().is_none() {
			continue;
		}
		entries.push(ZipEntryInfo {
			size: entry.size(),
			supported: parser_supports_path(Path::new(&*name)),
			name: name.into_owned(),
		});
	}
	Ok(entries)
}

/// Extracts `entry_name` of the archive at `zip_path` to a stable path under `cache_root` and
/// returns it. The path depends only on the archive and entry, and the file keeps the entry's own
/// name, so the extracted copy opens like any other file: the same reading position, bookmarks
/// and recent-documents entry come back each time. An earlier copy is overwritten.
pub fn extract_zip_entry_to_cache(zip_path: &Path, entry_name: &str, cache_root: &Path) -> Result<PathBuf> {
	let mut archive = ZipArchive::new(BufReader::new(
		File::open(zip_path).with_context(|| format!("Failed to open '{}'", zip_path.display()))?,
	))?;
	// Only the last component is used, so a name like `../x` can't leave `cache_root`.
	let file_name = Path::new(entry_name).file_name().with_context(|| format!("Invalid entry name '{entry_name}'"))?;
	let source = fs::canonicalize(zip_path).unwrap_or_else(|_| zip_path.to_path_buf());
	let mut hasher = Sha1::new();
	hasher.update(source.to_string_lossy().as_bytes());
	hasher.update(b"!");
	hasher.update(entry_name.as_bytes());
	let output_path = cache_root.join(URL_SAFE_NO_PAD.encode(hasher.finalize())).join(file_name);
	extract_zip_entry_to_file(&mut archive, entry_name, &output_path)?;
	Ok(output_path)
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
	fn zip_entry_name_resolver_matches_case_insensitively() {
		let archive = build_test_archive();
		let resolve = zip_entry_name_resolver(&archive);
		assert_eq!(resolve("foo.txt"), "foo.txt");
		assert_eq!(resolve("FOO.Txt"), "foo.txt");
		assert_eq!(resolve("Nested/Bar.TXT"), "nested/bar.txt");
		assert_eq!(resolve("missing.txt"), "missing.txt");
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

	#[test]
	fn read_zip_entry_bytes_with_password_decrypts_with_the_right_password() {
		let mut archive = build_encrypted_test_archive();
		let contents =
			read_zip_entry_bytes_with_password(&mut archive, "secret.mp3", Some("hunter2")).expect("read entry");
		assert_eq!(contents, b"secret-audio-bytes");
	}

	#[test]
	fn read_zip_entry_bytes_with_password_rejects_the_wrong_password() {
		let mut archive = build_encrypted_test_archive();
		assert!(read_zip_entry_bytes_with_password(&mut archive, "secret.mp3", Some("wrong")).is_err());
	}

	#[test]
	fn list_zip_entries_returns_files_only_with_support_flag() {
		let mut cursor = Cursor::new(Vec::new());
		{
			let mut writer = ZipWriter::new(&mut cursor);
			let options = FileOptions::<()>::default();
			writer.add_directory("dir/", options).expect("add dir");
			writer.start_file("dir/book.epub", options).expect("start file");
			writer.write_all(b"abc").expect("write file");
			writer.start_file("pic.xyz", options).expect("start file");
			writer.write_all(b"12345").expect("write file");
			writer.start_file("../evil.epub", options).expect("start file");
			writer.write_all(b"x").expect("write file");
			writer.finish().expect("finish zip");
		}
		cursor.set_position(0);
		let mut archive = ZipArchive::new(cursor).expect("open zip");
		let entries = list_zip_entries(&mut archive).expect("list entries");
		assert_eq!(
			entries,
			[
				ZipEntryInfo { name: "dir/book.epub".into(), size: 3, supported: true },
				ZipEntryInfo { name: "pic.xyz".into(), size: 5, supported: false },
			]
		);
	}

	#[test]
	fn list_zip_entries_lists_encrypted_entries_without_a_password() {
		let mut archive = build_encrypted_test_archive();
		let entries = list_zip_entries(&mut archive).expect("list entries");
		assert_eq!(entries.len(), 1);
		assert_eq!(entries[0].name, "secret.mp3");
	}

	#[test]
	fn list_zip_entries_of_an_empty_archive_is_empty() {
		let mut cursor = Cursor::new(Vec::new());
		ZipWriter::new(&mut cursor).finish().expect("finish zip");
		cursor.set_position(0);
		let mut archive = ZipArchive::new(cursor).expect("open zip");
		assert!(list_zip_entries(&mut archive).expect("list entries").is_empty());
	}

	#[test]
	fn extract_zip_entry_to_cache_is_stable_and_keeps_the_file_name() {
		let dir = TempDir::new("zip");
		let zip_path = dir.path().join("pack.zip");
		let mut writer = ZipWriter::new(File::create(&zip_path).expect("create zip"));
		writer.start_file("nested/bar.txt", FileOptions::<()>::default()).expect("start file");
		writer.write_all(b"nested").expect("write file");
		writer.finish().expect("finish zip");
		let cache = dir.path().join("cache");
		let first = extract_zip_entry_to_cache(&zip_path, "nested/bar.txt", &cache).expect("extract");
		let second = extract_zip_entry_to_cache(&zip_path, "nested/bar.txt", &cache).expect("extract again");
		assert_eq!(first, second);
		assert_eq!(first.file_name().and_then(|n| n.to_str()), Some("bar.txt"));
		assert!(first.starts_with(&cache));
		assert_eq!(fs::read_to_string(&first).expect("read"), "nested");
		assert!(extract_zip_entry_to_cache(&zip_path, "missing.txt", &cache).is_err());
	}
}
