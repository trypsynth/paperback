use std::{
	fs::{self, File},
	io::{self, Read, Seek},
	path::Path,
};

use anyhow::{Context, Result};
use zip::ZipArchive;

use super::text::url_decode;

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
		fs::create_dir_all(parent).with_context(|| format!("Failed to create directory '{parent:?}'"))?;
	}
	let mut out_file = File::create(output_path).with_context(|| format!("Failed to create file '{output_path:?}'"))?;
	io::copy(&mut entry, &mut out_file).with_context(|| format!("Failed to extract entry '{name}'"))?;
	Ok(())
}

pub fn find_zip_entry<R: Read + Seek>(archive: &mut ZipArchive<R>, filename: &str) -> Option<usize> {
	let decoded_filename = url_decode(filename);
	let needs_decode = decoded_filename != filename;
	(0..archive.len()).find(|&i| {
		if let Ok(entry) = archive.by_index(i) {
			let entry_name = entry.name();
			if entry_name == filename {
				return true;
			}
			if needs_decode && entry_name == decoded_filename {
				return true;
			}
			let decoded_entry = url_decode(entry_name);
			decoded_entry == filename || (needs_decode && decoded_entry == decoded_filename)
		} else {
			false
		}
	})
}
