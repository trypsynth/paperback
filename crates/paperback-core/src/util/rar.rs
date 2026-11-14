//! A RAR is read by repacking it once into an uncompressed zip in a cache, so everything that reads a zip reads a RAR too: the book picker, zipped DAISY books and audiobooks, and comics.

use std::{
	cell::RefCell,
	collections::HashMap,
	env,
	fs::{self, File},
	io::{self, BufWriter, Read, Write},
	path::{Path, PathBuf},
	rc::Rc,
	time::UNIX_EPOCH,
};

use anyhow::{Context, Result};
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use rars::{ArchiveReadOptions, ArchiveReader, ErrorKind, entry_relative_path, filename::decoded_name};
use sha1::{Digest, Sha1};
use zip::{CompressionMethod, ZipWriter, write::SimpleFileOptions};

use crate::{parser::PASSWORD_REQUIRED_ERROR_PREFIX, t};

/// Far more than any book, but it stops an archive that unpacks to far more than it holds, which a small one can, from filling the disk.
const MAX_UNPACKED_BYTES: u64 = 16 << 30;

/// Whether `path` starts with a RAR signature: RAR 1.3's, or the one RAR 1.5 to 7 share. Self-extracting archives are programs and are left alone.
#[must_use]
pub fn is_rar(path: &Path) -> bool {
	let mut start = [0; 6];
	File::open(path).and_then(|mut file| file.read_exact(&mut start)).is_ok()
		&& (start == *b"Rar!\x1a\x07" || start[..4] == *b"RE~^")
}

/// Where repacked archives are kept: temporary, since a copy can always be made again, and the system clears it.
#[must_use]
pub fn cache_root() -> PathBuf {
	env::temp_dir().join("paperback-rar-cache")
}

/// The zip copy of the RAR at `path`, made the first time and reused while the RAR is unchanged, so a book keeps its reading position. A `.cbr` becomes a `.cbz`, and anything else a `.zip`.
///
/// # Errors
///
/// Returns an error if the RAR can't be read, needs a password that `password` doesn't give, is split into parts, or would unpack to more than 16 GiB.
pub fn zip_copy(path: &Path, password: Option<&str>, cache_root: &Path) -> Result<PathBuf> {
	let metadata = fs::metadata(path).with_context(|| format!("Failed to open '{}'", path.display()))?;
	let source = fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
	let mut hasher = Sha1::new();
	hasher.update(source.to_string_lossy().as_bytes());
	hasher.update(metadata.len().to_le_bytes());
	let modified = metadata.modified().ok().and_then(|time| time.duration_since(UNIX_EPOCH).ok()).unwrap_or_default();
	hasher.update(modified.as_nanos().to_le_bytes());
	let is_comic = path.extension().is_some_and(|extension| extension.eq_ignore_ascii_case("cbr"));
	let stem = path.file_stem().map_or_else(|| "archive".into(), |stem| stem.to_string_lossy());
	let dir = cache_root.join(URL_SAFE_NO_PAD.encode(hasher.finalize()));
	let output = dir.join(format!("{stem}.{}", if is_comic { "cbz" } else { "zip" }));
	if output.is_file() {
		return Ok(output);
	}
	fs::create_dir_all(&dir).with_context(|| format!("Failed to create directory '{}'", dir.display()))?;
	// Written beside the copy and renamed into place, so an interrupted repack is never taken for a finished one.
	let partial = dir.join("repacking.partial");
	let result = repack(path, password, &partial);
	if let Err(error) = result {
		let _ = fs::remove_file(&partial);
		return Err(error);
	}
	fs::rename(&partial, &output).with_context(|| format!("Failed to create '{}'", output.display()))?;
	Ok(output)
}

/// The zip copy of `path` if it is a RAR, made without a password; otherwise `path` itself.
///
/// # Errors
///
/// Returns an error if `path` is a RAR that [`zip_copy`] can't repack.
pub fn as_zip(path: &Path) -> Result<PathBuf> {
	if is_rar(path) { zip_copy(path, None, &cache_root()) } else { Ok(path.to_path_buf()) }
}

fn repack(path: &Path, password: Option<&str>, output: &Path) -> Result<()> {
	let options = ArchiveReadOptions::with_optional_password(password.map(str::as_bytes))
		.with_max_member_output_bytes(MAX_UNPACKED_BYTES)
		.with_max_total_output_bytes(MAX_UNPACKED_BYTES);
	let archive = ArchiveReader::read_path_with_options(path, options).map_err(|error| rar_error(&error, password))?;
	let mut names = HashMap::new();
	for member in archive.members() {
		if member.meta.is_split_before || member.meta.is_split_after {
			// TRANSLATORS: Error shown when opening a RAR archive split into several files, such as book.part1.rar and book.part2.rar
			anyhow::bail!(t(
				"Paperback can't open a RAR archive split into parts. Join the parts into one archive to read it here."
			));
		}
		let decoded = decoded_name(&member.meta.name, member.name_is_unicode(), None)
			.map_or_else(|_| member.meta.name_lossy(), |name| String::from_utf8_lossy(&name).into_owned());
		names.insert(member.meta.name, decoded);
	}
	let file = File::create(output).with_context(|| format!("Failed to create '{}'", output.display()))?;
	let writer = Rc::new(RefCell::new(ZipWriter::new(BufWriter::new(file))));
	// Stored, since the copy is only read here and compressing it again would only slow opening a book.
	let entry_options = SimpleFileOptions::default().compression_method(CompressionMethod::Stored).large_file(true);
	let zip = Rc::clone(&writer);
	archive
		.extract_to_with_options(options, move |entry| -> rars::Result<Box<dyn Write>> {
			let name = names.get(&entry.name).cloned().unwrap_or_else(|| entry.name_lossy());
			// A name that would leave the archive's folder, such as `../x`, is passed over like a directory.
			let Ok(relative) = entry_relative_path(name.as_bytes()) else { return Ok(Box::new(io::sink())) };
			if entry.is_directory {
				return Ok(Box::new(io::sink()));
			}
			let name = relative
				.components()
				.map(|component| component.as_os_str().to_string_lossy())
				.collect::<Vec<_>>()
				.join("/");
			zip.borrow_mut().start_file(name, entry_options).map_err(io::Error::other)?;
			Ok(Box::new(ZipEntryWriter(Rc::clone(&zip))))
		})
		.map_err(|error| rar_error(&error, password))?;
	let writer =
		Rc::try_unwrap(writer).map_err(|_| anyhow::anyhow!("the zip copy is still being written"))?.into_inner();
	writer.finish().context("Failed to finish the zip copy")?.flush().context("Failed to finish the zip copy")?;
	Ok(())
}

/// Hands each extracted entry's bytes to the zip being written, whose current file is that entry.
struct ZipEntryWriter<W: Write + io::Seek>(Rc<RefCell<ZipWriter<W>>>);

impl<W: Write + io::Seek> Write for ZipEntryWriter<W> {
	fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
		self.0.borrow_mut().write(buf)
	}

	fn flush(&mut self) -> io::Result<()> {
		self.0.borrow_mut().flush()
	}
}

/// A wrong password and damaged data look the same in a RAR, so with a password given either is reported as the password being wrong, which is the likelier.
fn rar_error(error: &rars::Error, password: Option<&str>) -> anyhow::Error {
	match error.kind() {
		ErrorKind::PasswordRequired => {
			// TRANSLATORS: Error detail shown when a password-protected ZIP-based document needs a password (the internal sentinel prefix before it is not translated)
			anyhow::anyhow!("{PASSWORD_REQUIRED_ERROR_PREFIX}{}", t("Password required"))
		}
		ErrorKind::BadPassword if password.is_some() => {
			// TRANSLATORS: Error detail shown when the password for a ZIP-based document is wrong (the internal sentinel prefix before it is not translated)
			anyhow::anyhow!("{PASSWORD_REQUIRED_ERROR_PREFIX}{}", t("Password incorrect"))
		}
		_ => anyhow::anyhow!("{error}"),
	}
}

#[cfg(test)]
mod tests {
	use std::io::Read;

	use rars::{ArchiveVersion, Builder};
	use zip::ZipArchive;

	use super::*;
	use crate::util::test_support::TempDir;

	const CHAPTER: &[u8] = b"Chapter one. It was a bright cold day in April, and the clocks were striking thirteen.";

	fn write_rar(dir: &TempDir, name: &str, builder: &Builder) -> PathBuf {
		let path = PathBuf::from(dir.join_str(name));
		fs::write(&path, builder.to_bytes().expect("build the RAR")).expect("write the RAR");
		path
	}

	fn book(format: ArchiveVersion) -> Builder {
		let mut builder = Builder::new(format);
		builder.add_bytes(b"book/chapter1.txt".to_vec(), CHAPTER.repeat(50), None, None).expect("add a chapter");
		builder.add_bytes(b"readme.txt".to_vec(), b"Read me first.".to_vec(), None, None).expect("add a readme");
		builder
	}

	fn entries(path: &Path) -> Vec<(String, Vec<u8>)> {
		let mut archive = ZipArchive::new(File::open(path).expect("open the copy")).expect("the copy is a zip");
		let mut entries: Vec<(String, Vec<u8>)> = (0..archive.len())
			.map(|index| {
				let mut entry = archive.by_index(index).expect("read an entry");
				let mut contents = Vec::new();
				entry.read_to_end(&mut contents).expect("read an entry");
				(entry.name().expect("a name").into_owned(), contents)
			})
			.collect();
		entries.sort();
		entries
	}

	#[rstest::rstest]
	#[case::rar5(ArchiveVersion::Rar50, false)]
	#[case::rar7(ArchiveVersion::Rar70, false)]
	#[case::rar4_solid(ArchiveVersion::Rar29, true)]
	#[case::rar2(ArchiveVersion::Rar20, false)]
	fn repacks_every_entry_with_its_folders(#[case] format: ArchiveVersion, #[case] solid: bool) {
		let dir = TempDir::new("rar-repack");
		let rar = write_rar(&dir, "book.rar", &book(format).solid(solid));
		assert!(is_rar(&rar));
		let copy = zip_copy(&rar, None, Path::new(&dir.join_str("cache"))).expect("repack");
		assert_eq!(copy.extension().and_then(|extension| extension.to_str()), Some("zip"));
		assert_eq!(
			entries(&copy),
			[
				("book/chapter1.txt".to_owned(), CHAPTER.repeat(50)),
				("readme.txt".to_owned(), b"Read me first.".to_vec())
			]
		);
	}

	#[test]
	fn reuses_the_copy_until_the_archive_changes() {
		let dir = TempDir::new("rar-reuse");
		let cache = PathBuf::from(dir.join_str("cache"));
		let rar = write_rar(&dir, "book.rar", &book(ArchiveVersion::Rar50));
		let first = zip_copy(&rar, None, &cache).expect("repack");
		assert_eq!(zip_copy(&rar, None, &cache).expect("reuse"), first);
		let mut builder = Builder::new(ArchiveVersion::Rar50);
		builder
			.add_bytes(b"other.txt".to_vec(), b"A different book, a different size.".to_vec(), None, None)
			.expect("add");
		write_rar(&dir, "book.rar", &builder);
		let second = zip_copy(&rar, None, &cache).expect("repack the new archive");
		assert_ne!(second, first);
		assert_eq!(entries(&second)[0].0, "other.txt");
	}

	#[test]
	fn a_comic_becomes_a_cbz() {
		let dir = TempDir::new("rar-comic");
		let rar = write_rar(&dir, "issue.cbr", &book(ArchiveVersion::Rar50));
		let copy = zip_copy(&rar, None, Path::new(&dir.join_str("cache"))).expect("repack");
		assert_eq!(copy.file_name().and_then(|name| name.to_str()), Some("issue.cbz"));
	}

	#[test]
	fn asks_for_the_password_and_says_when_it_is_wrong() {
		let dir = TempDir::new("rar-password");
		let cache = PathBuf::from(dir.join_str("cache"));
		let rar = write_rar(&dir, "locked.rar", &book(ArchiveVersion::Rar50).password(Some(b"open sesame".to_vec())));
		let missing = zip_copy(&rar, None, &cache).expect_err("needs a password").to_string();
		assert!(missing.starts_with(PASSWORD_REQUIRED_ERROR_PREFIX), "{missing}");
		assert!(missing.ends_with("Password required"), "{missing}");
		let wrong = zip_copy(&rar, Some("wrong"), &cache).expect_err("wrong password").to_string();
		assert!(wrong.ends_with("Password incorrect"), "{wrong}");
		let copy = zip_copy(&rar, Some("open sesame"), &cache).expect("the right password opens it");
		assert_eq!(entries(&copy).len(), 2);
		assert!(
			!PathBuf::from(dir.join_str("cache"))
				.read_dir()
				.unwrap()
				.flatten()
				.any(|dir| dir.path().join("repacking.partial").exists())
		);
	}

	#[test]
	fn refuses_one_part_of_a_split_archive() {
		let dir = TempDir::new("rar-split");
		let mut builder = Builder::new(ArchiveVersion::Rar50).store(true).volume_size(Some(4096));
		builder.add_bytes(b"big.txt".to_vec(), CHAPTER.repeat(200), None, None).expect("add");
		let volumes = builder.build_volumes(None).expect("build volumes");
		assert!(volumes.len() > 1);
		let path = PathBuf::from(dir.join_str("big.part1.rar"));
		fs::write(&path, &volumes[0]).expect("write the first part");
		let error =
			zip_copy(&path, None, Path::new(&dir.join_str("cache"))).expect_err("a part is refused").to_string();
		assert!(error.contains("split into parts"), "{error}");
	}

	#[test]
	fn leaves_other_files_as_they_are() {
		let dir = TempDir::new("rar-not");
		let path = PathBuf::from(dir.join_str("book.zip"));
		fs::write(&path, b"PK\x03\x04 not a RAR").expect("write");
		assert!(!is_rar(&path));
		assert_eq!(as_zip(&path).expect("no repack"), path);
	}
}
