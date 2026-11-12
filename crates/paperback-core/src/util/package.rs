//! An unpacked EPUB or eBraille book, a folder of files opened by its package document, read by zipping the files it names into a cache once, so it is read as a packaged book is.
//!
//! Only the package documents and the files their manifests list go in, so opening a stray `.opf` in a folder of other things doesn't zip the folder.

use std::{
	collections::BTreeSet,
	env,
	fs::{self, File},
	io::{BufWriter, Write},
	path::{Path, PathBuf},
	time::UNIX_EPOCH,
};

use anyhow::{Context, Result};
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use percent_encoding::percent_decode_str;
use roxmltree::{Document as XmlDocument, ParsingOptions};
use sha1::{Digest, Sha1};
use zip::{CompressionMethod, ZipWriter, write::SimpleFileOptions};

use crate::document::{Document, MarkerType};

const CONTAINER: &str = "META-INF/container.xml";

/// A zipped copy of an unpacked book, and the folder it was made from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageCopy {
	pub zip: PathBuf,
	pub root: PathBuf,
}

/// Where copies are kept: temporary, since a copy can always be made again, and the system clears it.
#[must_use]
pub fn cache_root() -> PathBuf {
	env::temp_dir().join("paperback-package-cache")
}

/// Whether `path` is a package document, by its name.
#[must_use]
pub fn is_package_document(path: &Path) -> bool {
	path.extension().is_some_and(|extension| extension.eq_ignore_ascii_case("opf"))
}

/// Whether a package document is an eBraille book's: it names a braille language, such as `en-Brai-US`, or the eBraille format.
#[must_use]
pub fn is_braille_package(package: &str) -> bool {
	let Ok(document) =
		XmlDocument::parse_with_options(package, ParsingOptions { allow_dtd: true, ..ParsingOptions::default() })
	else {
		return false;
	};
	document.descendants().filter(roxmltree::Node::is_element).any(|node| {
		let text = node.text().unwrap_or_default().trim();
		match node.tag_name().name() {
			"language" => text.split('-').any(|subtag| subtag.eq_ignore_ascii_case("Brai")),
			"format" => text.to_ascii_lowercase().starts_with("ebraille"),
			_ => false,
		}
	})
}

/// Whether a package document is a DAISY 3 book's: its manifest holds DTBook or SMIL, or it names the DAISY standard, Z39.86.
#[must_use]
pub fn is_daisy_package(package: &str) -> bool {
	let lower = package.to_ascii_lowercase();
	lower.contains("application/x-dtbook+xml") || lower.contains("application/smil") || lower.contains("z39.86")
}

/// The zip copy of the book whose package document is `opf`, made the first time and reused while none of its files change, so the book keeps its reading position. The book's folder is the nearest one above `opf` whose `META-INF/container.xml` names it, as in an unzipped EPUB, or else `opf`'s own, as in an eBraille file set, whose container this writes.
///
/// # Errors
///
/// Returns an error if a package document can't be read, or the copy can't be written.
pub fn zip_copy(opf: &Path, extension: &str, cache_root: &Path) -> Result<PackageCopy> {
	let (root, container, rootfiles) = book_folder(opf)?;
	let mut files = BTreeSet::new();
	for rootfile in &rootfiles {
		files.insert(rootfile.clone());
		let package =
			fs::read_to_string(root.join(rootfile)).with_context(|| format!("Failed to read '{rootfile}'"))?;
		let base = rootfile.rsplit_once('/').map_or("", |(dir, _)| dir);
		files.extend(manifest_files(&package, base).into_iter().filter(|file| root.join(file).is_file()));
	}
	let mut hasher = Sha1::new();
	hasher.update(root.to_string_lossy().as_bytes());
	hasher.update(container.as_bytes());
	for file in &files {
		let metadata = fs::metadata(root.join(file)).ok();
		let modified = metadata
			.as_ref()
			.and_then(|metadata| metadata.modified().ok())
			.and_then(|time| time.duration_since(UNIX_EPOCH).ok())
			.unwrap_or_default();
		hasher.update(file.as_bytes());
		hasher.update(metadata.map_or(0, |metadata| metadata.len()).to_le_bytes());
		hasher.update(modified.as_nanos().to_le_bytes());
	}
	let name = root.file_name().map_or_else(|| "book".into(), |name| name.to_string_lossy());
	let dir = cache_root.join(URL_SAFE_NO_PAD.encode(hasher.finalize()));
	let zip = dir.join(format!("{name}.{extension}"));
	if !zip.is_file() {
		fs::create_dir_all(&dir).with_context(|| format!("Failed to create directory '{}'", dir.display()))?;
		// Written beside the copy and renamed into place, so an interrupted copy is never taken for a finished one.
		let partial = dir.join("copying.partial");
		let written = write_zip(&root, &container, &files, &partial);
		if let Err(error) = written {
			let _ = fs::remove_file(&partial);
			return Err(error);
		}
		fs::rename(&partial, &zip).with_context(|| format!("Failed to create '{}'", zip.display()))?;
	}
	Ok(PackageCopy { zip, root })
}

/// Points each picture at its file in the unpacked folder, since the copy that held it may be cleared from the cache while the book is open.
pub fn point_images_at(document: &mut Document, root: &Path) {
	for marker in &mut document.buffer.markers {
		let is_picture = matches!(marker.mtype, MarkerType::Image | MarkerType::Figure);
		if is_picture && !marker.reference.is_empty() && !marker.reference.contains(':') {
			marker.reference = root.join(&marker.reference).to_string_lossy().into_owned();
		}
	}
}

/// The book's folder, its `container.xml`, and the package documents that names, relative to the folder.
fn book_folder(opf: &Path) -> Result<(PathBuf, String, Vec<String>)> {
	// Only for comparing, since on Windows it gives `\\?\` paths, which shouldn't reach a picture's path.
	let canonical = fs::canonicalize(opf).ok();
	let mut dir = opf.parent();
	for _ in 0..4 {
		let Some(candidate) = dir else { break };
		if let Ok(container) = fs::read_to_string(candidate.join(CONTAINER)) {
			let rootfiles = rootfiles(&container);
			let names_opf = rootfiles
				.iter()
				.any(|rootfile| canonical.is_some() && fs::canonicalize(candidate.join(rootfile)).ok() == canonical);
			if names_opf {
				return Ok((candidate.to_path_buf(), container, rootfiles));
			}
		}
		dir = candidate.parent();
	}
	let root = opf.parent().context("A package document is in no folder")?.to_path_buf();
	let name = opf.file_name().context("A package document has no name")?.to_string_lossy().into_owned();
	let container = format!(
		r#"<?xml version="1.0" encoding="UTF-8"?><container xmlns="urn:oasis:names:tc:opendocument:xmlns:container" version="1.0"><rootfiles><rootfile full-path="{}" media-type="application/oebps-package+xml"/></rootfiles></container>"#,
		name.replace('&', "&amp;").replace('"', "&quot;").replace('<', "&lt;")
	);
	Ok((root, container, vec![name]))
}

fn rootfiles(container: &str) -> Vec<String> {
	let Ok(document) =
		XmlDocument::parse_with_options(container, ParsingOptions { allow_dtd: true, ..ParsingOptions::default() })
	else {
		return Vec::new();
	};
	document
		.descendants()
		.filter(|node| node.is_element() && node.tag_name().name() == "rootfile")
		.filter_map(|node| node.attribute("full-path"))
		.filter_map(|path| normalize("", path))
		.collect()
}

/// The files a package's manifest lists, relative to the book's folder; `base` is the package's own folder within it.
fn manifest_files(package: &str, base: &str) -> Vec<String> {
	let Ok(document) =
		XmlDocument::parse_with_options(package, ParsingOptions { allow_dtd: true, ..ParsingOptions::default() })
	else {
		return Vec::new();
	};
	document
		.descendants()
		.filter(|node| node.is_element() && node.tag_name().name() == "item")
		.filter_map(|node| node.attribute("href"))
		.filter(|href| !href.contains(':'))
		.filter_map(|href| {
			normalize(base, &percent_decode_str(href.split('#').next().unwrap_or_default()).decode_utf8_lossy())
		})
		.collect()
}

/// `href` resolved against `base`, both relative to the book's folder, or `None` for a path that would leave it.
fn normalize(base: &str, href: &str) -> Option<String> {
	let mut parts: Vec<&str> =
		if href.starts_with('/') { Vec::new() } else { base.split('/').filter(|part| !part.is_empty()).collect() };
	for part in href.split(['/', '\\']) {
		match part {
			"" | "." => {}
			".." => {
				parts.pop()?;
			}
			part => parts.push(part),
		}
	}
	(!parts.is_empty()).then(|| parts.join("/"))
}

fn write_zip(root: &Path, container: &str, files: &BTreeSet<String>, output: &Path) -> Result<()> {
	let file = File::create(output).with_context(|| format!("Failed to create '{}'", output.display()))?;
	let mut zip = ZipWriter::new(BufWriter::new(file));
	// Stored, since the copy is only read here and compressing it would only slow opening the book.
	let options = SimpleFileOptions::default().compression_method(CompressionMethod::Stored).large_file(true);
	zip.start_file("mimetype", options)?;
	zip.write_all(b"application/epub+zip")?;
	zip.start_file(CONTAINER, options)?;
	zip.write_all(container.as_bytes())?;
	for name in files.iter().filter(|name| name.as_str() != CONTAINER && name.as_str() != "mimetype") {
		zip.start_file(name.as_str(), options)?;
		let mut source = File::open(root.join(name)).with_context(|| format!("Failed to read '{name}'"))?;
		std::io::copy(&mut source, &mut zip).with_context(|| format!("Failed to copy '{name}'"))?;
	}
	zip.finish()?.flush()?;
	Ok(())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn resolves_manifest_paths_inside_the_book() {
		assert_eq!(normalize("OEBPS", "text/one.xhtml").as_deref(), Some("OEBPS/text/one.xhtml"));
		assert_eq!(normalize("OEBPS/text", "../images/a.png").as_deref(), Some("OEBPS/images/a.png"));
		assert_eq!(normalize("", "ebraille/one.html").as_deref(), Some("ebraille/one.html"));
		assert_eq!(normalize("", "../secret.txt"), None);
	}

	#[test]
	fn knows_a_braille_package() {
		assert!(is_braille_package(
			r#"<package xmlns:dc="http://purl.org/dc/elements/1.1/"><metadata><dc:language>en-Brai-US</dc:language></metadata></package>"#
		));
		assert!(is_braille_package(
			r#"<package xmlns:dc="http://purl.org/dc/elements/1.1/"><metadata><dc:format>eBraille 1.0</dc:format></metadata></package>"#
		));
		assert!(!is_braille_package(
			r#"<package xmlns:dc="http://purl.org/dc/elements/1.1/"><metadata><dc:language>en</dc:language></metadata></package>"#
		));
	}
}

#[cfg(test)]
mod book_tests {
	use std::fs;

	use crate::{
		document::{MarkerType, ParserContext},
		parser::parse_document,
		util::test_support::TempDir,
	};

	fn write(dir: &TempDir, name: &str, contents: &[u8]) {
		let path = std::path::PathBuf::from(dir.join_str(name));
		fs::create_dir_all(path.parent().expect("a folder")).expect("create the folder");
		fs::write(path, contents).expect("write a file");
	}

	#[test]
	fn reads_an_ebraille_file_set_by_its_package_and_finds_its_pictures() {
		let dir = TempDir::new("package-ebraille");
		write(&dir, "package.opf", br#"<?xml version="1.0" encoding="utf-8"?><package xmlns="http://www.idpf.org/2007/opf" version="3.0" unique-identifier="id"><metadata xmlns:dc="http://purl.org/dc/elements/1.1/"><dc:title>Max at the Zoo</dc:title><dc:identifier id="id">zoo</dc:identifier><dc:language>en-Brai-US</dc:language><meta property="a11y:brailleSystem">UEB grade2</meta></metadata><manifest><item id="one" href="ebraille/one.html" media-type="application/xhtml+xml"/><item id="pic" href="ebraille/pic.png" media-type="image/png"/></manifest><spine><itemref idref="one"/></spine></package>"#);
		write(&dir, "ebraille/one.html", "<?xml version=\"1.0\" encoding=\"utf-8\"?><html xmlns=\"http://www.w3.org/1999/xhtml\"><head><title>one</title></head><body><h1>⠠⠍⠁⠭ ⠁⠞ ⠮ ⠠⠵⠕⠕</h1><p><img src=\"pic.png\" alt=\"⠠⠁ ⠛⠊⠗⠁⠖⠑\"/></p></body></html>".as_bytes());
		write(&dir, "ebraille/pic.png", b"not really a picture");
		write(&dir, "unrelated.txt", b"left out of the copy");
		let document = parse_document(&ParserContext::new(dir.join_str("package.opf"))).expect("read the file set");
		assert!(document.buffer.content.contains("Max at the Zoo"), "{}", document.buffer.content);
		let picture =
			document.buffer.markers.iter().find(|marker| marker.mtype == MarkerType::Image).expect("a picture");
		let path = std::path::Path::new(&picture.reference);
		assert!(path.is_absolute() && path.is_file(), "{}", picture.reference);
	}

	#[test]
	fn reads_an_unzipped_epub_by_its_package() {
		let dir = TempDir::new("package-epub");
		write(&dir, "META-INF/container.xml", br#"<?xml version="1.0"?><container xmlns="urn:oasis:names:tc:opendocument:xmlns:container" version="1.0"><rootfiles><rootfile full-path="OEBPS/content.opf" media-type="application/oebps-package+xml"/></rootfiles></container>"#);
		write(&dir, "OEBPS/content.opf", br#"<?xml version="1.0" encoding="utf-8"?><package xmlns="http://www.idpf.org/2007/opf" version="3.0" unique-identifier="id"><metadata xmlns:dc="http://purl.org/dc/elements/1.1/"><dc:title>Unzipped</dc:title><dc:identifier id="id">u</dc:identifier><dc:language>en</dc:language></metadata><manifest><item id="one" href="text/one.xhtml" media-type="application/xhtml+xml"/></manifest><spine><itemref idref="one"/></spine></package>"#);
		write(&dir, "OEBPS/text/one.xhtml", br#"<?xml version="1.0" encoding="utf-8"?><html xmlns="http://www.w3.org/1999/xhtml"><head><title>one</title></head><body><p>An EPUB that was never zipped.</p></body></html>"#);
		let document =
			parse_document(&ParserContext::new(dir.join_str("OEBPS/content.opf"))).expect("read the unzipped EPUB");
		assert!(document.buffer.content.contains("An EPUB that was never zipped."), "{}", document.buffer.content);
	}
}
