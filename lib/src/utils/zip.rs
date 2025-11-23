use std::io::{Read, Seek};

use anyhow::{Context, Result};
use zip::ZipArchive;

use super::text::url_decode;

pub fn read_zip_entry_by_name<R: Read + Seek>(archive: &mut ZipArchive<R>, name: &str) -> Result<String> {
	let mut entry = archive.by_name(name).with_context(|| format!("Failed to get entry '{name}'"))?;
	let mut contents = String::new();
	entry.read_to_string(&mut contents).with_context(|| format!("Failed to read entry '{name}'"))?;
	Ok(contents)
}

pub fn find_zip_entry<R: Read + Seek>(archive: &mut ZipArchive<R>, filename: &str) -> Option<usize> {
	for i in 0..archive.len() {
		if let Ok(entry) = archive.by_index(i) {
			if entry.name() == filename {
				return Some(i);
			}
		}
	}
	let decoded = url_decode(filename);
	if decoded != filename {
		for i in 0..archive.len() {
			if let Ok(entry) = archive.by_index(i) {
				if entry.name() == decoded {
					return Some(i);
				}
			}
		}
	}
	for i in 0..archive.len() {
		if let Ok(entry) = archive.by_index(i) {
			let entry_name = entry.name();
			let decoded_entry_name = url_decode(entry_name);
			if decoded_entry_name == filename || decoded_entry_name == decoded {
				return Some(i);
			}
		}
	}
	None
}
