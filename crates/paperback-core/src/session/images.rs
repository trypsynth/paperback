//! Saving the picture under the caret: finding the image or figure there, and reading its bytes
//! out of wherever the document keeps them, unchanged.

use std::{
	fs::{self, File},
	io::BufReader,
	path::Path,
};

use anyhow::{Context, Result};
use base64::{
	Engine, alphabet,
	engine::{DecodePaddingMode, GeneralPurpose, GeneralPurposeConfig},
};
use libchm::ChmFile;
use zip::ZipArchive;

use super::DocumentSession;
use crate::{
	document::MarkerType,
	parser::{fb2, is_data_uri, is_external_url},
	t,
	util::{text::url_decode, zip::read_zip_entry_bytes},
};

/// Books write base64 with and without padding, and with line breaks, and none of it is worth refusing a picture over.
const LENIENT_BASE64: GeneralPurpose = GeneralPurpose::new(
	&alphabet::STANDARD,
	GeneralPurposeConfig::new().with_decode_padding_mode(DecodePaddingMode::Indifferent),
);

impl DocumentSession {
	/// The file name to offer when saving the image at `position`, or `None` when there is no image there whose picture can be saved.
	#[must_use]
	pub fn image_file_name_at(&self, position: i64) -> Option<String> {
		self.image_source_at(position).map(|source| image_file_name(&source))
	}

	/// Writes the picture of the image at `position` to `output_path`.
	///
	/// # Errors
	///
	/// Returns an error when there is no saveable image at `position`, or its picture cannot be read or written.
	pub fn save_image(&self, position: i64, output_path: &str) -> Result<()> {
		let source = self.image_source_at(position).context("no saveable image at this position")?;
		let bytes = self.image_bytes(&source)?;
		fs::write(output_path, bytes).with_context(|| format!("failed to write '{output_path}'"))
	}

	/// Where the picture of the image or figure at `position` lives. A described image covers its placeholder text; a picture page carries no length and covers its whole line.
	fn image_source_at(&self, position: i64) -> Option<String> {
		let pos = usize::try_from(position.max(0)).unwrap_or(0);
		let (line_start, line_end) = self.line_bounds_at(position)?;
		let markers = &self.handle.document().buffer.markers;
		[MarkerType::Image, MarkerType::Figure]
			.into_iter()
			.filter_map(|mtype| markers.get(self.handle.current_marker_index(pos, mtype)?))
			.filter(|marker| {
				if marker.length == 0 {
					i64::try_from(marker.position).is_ok_and(|start| start >= line_start && start <= line_end)
				} else {
					pos < marker.position + marker.length
				}
			})
			.filter(|marker| !marker.reference.is_empty() && !is_external_url(&marker.reference))
			.max_by_key(|marker| marker.position)
			.map(|marker| marker.reference.clone())
	}

	fn image_bytes(&self, source: &str) -> Result<Vec<u8>> {
		if is_data_uri(source) {
			return decode_data_uri(source);
		}
		let extension = Path::new(&self.file_path).extension().map(|ext| ext.to_string_lossy().to_ascii_lowercase());
		match extension.as_deref() {
			Some("epub" | "cbz" | "cbr") => {
				let mut archive = ZipArchive::new(BufReader::new(File::open(&self.file_path)?))?;
				read_zip_entry_bytes(&mut archive, source)
			}
			Some("chm") => {
				let mut chm = ChmFile::open(&self.file_path)?;
				let entry = chm.find(source)?;
				Ok(chm.read(&entry)?)
			}
			Some("fb2") => Ok(LENIENT_BASE64.decode(strip_whitespace(&fb2::binary_data(&self.file_path, source)?))?),
			_ => {
				let path = local_image_path(&self.file_path, source);
				fs::read(&path).with_context(|| format!("failed to read '{}'", path.display()))
			}
		}
	}
}

/// A document read from disk names its pictures relative to itself, the way a browser would resolve them.
fn local_image_path(document_path: &str, source: &str) -> std::path::PathBuf {
	let source = source.split(['#', '?']).next().unwrap_or_default();
	let source = url_decode(source);
	let source = source.strip_prefix("file:///").or_else(|| source.strip_prefix("file://")).unwrap_or(&source);
	let source = Path::new(source);
	if source.is_absolute() {
		return source.to_path_buf();
	}
	Path::new(document_path).parent().unwrap_or_else(|| Path::new("")).join(source)
}

fn decode_data_uri(uri: &str) -> Result<Vec<u8>> {
	let (header, data) = uri.split_once(',').context("malformed data URI")?;
	if header.to_ascii_lowercase().ends_with(";base64") {
		Ok(LENIENT_BASE64.decode(strip_whitespace(data))?)
	} else {
		Ok(percent_encoding::percent_decode_str(data).collect())
	}
}

fn strip_whitespace(text: &str) -> String {
	text.chars().filter(|c| !c.is_ascii_whitespace()).collect()
}

/// The name a picture is offered under: the last part of its path, or for one carried inline, a generic name with the extension its media type implies.
fn image_file_name(source: &str) -> String {
	// TRANSLATORS: File name stem offered when saving a picture that has no file name of its own
	let fallback = t("image");
	if is_data_uri(source) {
		let media_type = source[5..].split([';', ',']).next().unwrap_or_default().to_ascii_lowercase();
		let subtype = media_type.strip_prefix("image/").unwrap_or_default().split('+').next().unwrap_or_default();
		return match subtype {
			"" => fallback,
			"jpeg" => format!("{fallback}.jpg"),
			subtype => format!("{fallback}.{subtype}"),
		};
	}
	let path = source.split(['#', '?']).next().unwrap_or_default();
	let path = if path.is_empty() { source.trim_start_matches('#') } else { path };
	let name = url_decode(path.rsplit(['/', '\\']).next().unwrap_or_default());
	if name.is_empty() { fallback } else { name }
}

#[cfg(test)]
mod tests {
	use rstest::rstest;

	use super::{decode_data_uri, image_file_name, local_image_path};

	#[rstest]
	#[case("OEBPS/Images/cover.jpg", "cover.jpg")]
	#[case("/images/my%20map.gif", "my map.gif")]
	#[case("pics/plate.png?v=2#top", "plate.png")]
	#[case("#cover.jpg", "cover.jpg")]
	#[case("data:image/png;base64,AAAA", "image.png")]
	#[case("data:image/jpeg;base64,AAAA", "image.jpg")]
	#[case("data:image/svg+xml,%3Csvg%3E", "image.svg")]
	#[case("data:;base64,AAAA", "image")]
	fn a_picture_is_offered_under_its_own_name(#[case] source: &str, #[case] expected: &str) {
		assert_eq!(image_file_name(source), expected);
	}

	#[test]
	fn an_inline_picture_decodes_from_base64_however_it_is_wrapped() {
		assert_eq!(decode_data_uri("data:image/png;base64,aGVs\nbG8").unwrap(), b"hello");
	}

	#[test]
	fn an_inline_picture_without_base64_is_percent_decoded() {
		assert_eq!(decode_data_uri("data:image/svg+xml,%3Csvg%2F%3E").unwrap(), b"<svg/>");
	}

	#[test]
	fn a_local_picture_resolves_against_the_document_folder() {
		let path = local_image_path("/books/guide/index.html", "img/fig%201.png#x");
		assert_eq!(path, std::path::Path::new("/books/guide").join("img/fig 1.png"));
	}
}
