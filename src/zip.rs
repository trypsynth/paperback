use std::{
	fs::{self, File},
	io::{self, Read, Seek},
	path::Path,
};

use anyhow::{Context, Result};
use zip::ZipArchive;

pub fn read_zip_entry_by_name<R: Read + Seek>(archive: &mut ZipArchive<R>, name: &str) -> Result<String> {
	let mut entry = archive.by_name(name).with_context(|| format!("Failed to get entry '{name}'"))?;
	let mut contents = String::new();
	entry.read_to_string(&mut contents).with_context(|| format!("Failed to read entry '{name}'"))?;
	Ok(contents)
}

pub fn extract_zip_entry_to_file<R: Read + Seek>(
	archive: &mut ZipArchive<R>,
	name: &str,
	output_path: &Path,
) -> Result<()> {
	let mut entry = archive.by_name(name).with_context(|| format!("Failed to get entry '{name}'"))?;
	if let Some(parent) = output_path.parent() {
		fs::create_dir_all(parent).with_context(|| format!("Failed to create directory '{}'", parent.display()))?;
	}
	let mut out_file =
		File::create(output_path).with_context(|| format!("Failed to create file '{}'", output_path.display()))?;
	io::copy(&mut entry, &mut out_file).with_context(|| format!("Failed to extract entry '{name}'"))?;
	Ok(())
}

#[cfg(test)]
mod tests {
	use std::{
		io::{Cursor, Write},
		time::{SystemTime, UNIX_EPOCH},
	};

	use zip::{ZipWriter, write::FileOptions};

	use super::*;

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

	fn unique_temp_path(suffix: &str) -> std::path::PathBuf {
		let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
		let mut path = std::env::temp_dir();
		path.push(format!("paperback_test_{nanos}"));
		path.push(suffix);
		path
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
	fn extract_zip_entry_to_file_writes_to_nested_path() {
		let mut archive = build_test_archive();
		let output_path = unique_temp_path("nested/out.txt");
		extract_zip_entry_to_file(&mut archive, "nested/bar.txt", &output_path).expect("extract entry");
		let contents = fs::read_to_string(&output_path).expect("read output");
		assert_eq!(contents, "nested");
	}

	#[test]
	fn extract_zip_entry_to_file_reports_missing_entry() {
		let mut archive = build_test_archive();
		let output_path = unique_temp_path("nested/missing.txt");
		assert!(extract_zip_entry_to_file(&mut archive, "does-not-exist.txt", &output_path).is_err());
	}

	#[test]
	fn extract_zip_entry_to_file_overwrites_existing_file_contents() {
		let mut archive = build_test_archive();
		let output_path = unique_temp_path("nested/overwrite.txt");
		if let Some(parent) = output_path.parent() {
			fs::create_dir_all(parent).expect("create parent");
		}
		fs::write(&output_path, "old").expect("seed file");
		extract_zip_entry_to_file(&mut archive, "foo.txt", &output_path).expect("extract entry");
		let contents = fs::read_to_string(&output_path).expect("read output");
		assert_eq!(contents, "hello world");
	}

	#[test]
	fn read_zip_entry_by_name_reads_nested_entry() {
		let mut archive = build_test_archive();
		let contents = read_zip_entry_by_name(&mut archive, "nested/bar.txt").expect("read nested entry");
		assert_eq!(contents, "nested");
	}
}
