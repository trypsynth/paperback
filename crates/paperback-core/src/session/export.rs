//! Getting document content out of the session: exporting to text/HTML/Markdown, resolving
//! a webview target for EPUB/HTML/Markdown (extracting sibling resources so relative
//! `<img src="../images/...">`-style references resolve on disk), and writing the
//! underlying source to a temp file with the caret mapped to the current reading position.

use std::{
	fs::{self, File},
	io::{self, BufReader, Write},
	path::Path,
};

use base64::Engine;
use zip::ZipArchive;

use super::{DocumentSession, SourceView, WebviewTarget};
use crate::{
	config::compute_document_hash,
	document::MarkerType,
	export::{ExportFormat, render},
	parser,
	reader_core::{encode_url_fragment, nearest_fragment_before},
	util::{encoding::convert_to_utf8, zip as zip_utils},
};

impl DocumentSession {
	#[must_use]
	pub fn webview_target_path(&self, position: i64, temp_dir: &str) -> Option<WebviewTarget> {
		let section_path = self.get_current_section_path(position).filter(|path| !path.is_empty());
		if let Some(section_path) = section_path {
			let digest = compute_document_hash(&self.file_path);
			let hash = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(digest);
			let doc_temp_dir = Path::new(temp_dir).join(format!("paperback_{hash}"));
			if fs::create_dir_all(&doc_temp_dir).is_ok() {
				// Extract every entry (sections, images, stylesheets, fonts, ...) once,
				// preserving the epub's internal layout, so the section's relative
				// references resolve on disk: both its resources (e.g.
				// `<img src="../images/foo.jpg">`) and its links to other sections,
				// which is what a table of contents is made of.
				let _ = self.ensure_epub_resources_extracted(&doc_temp_dir);
				// Re-extract the section itself fresh at its original relative path so
				// the reading-position anchor below is injected into a clean copy.
				let output_path = doc_temp_dir.join(&section_path);
				let output_str = output_path.to_string_lossy().to_string();
				if self.extract_resource(&section_path, &output_str).ok() == Some(true) {
					let fragment = self.inject_reading_anchor(position, &output_str);
					return Some(WebviewTarget { path: output_str, fragment });
				}
			}
		}
		let ext = Path::new(&self.file_path).extension().map(|ext| ext.to_string_lossy().to_ascii_lowercase());
		match ext.as_deref() {
			Some("html" | "htm" | "xhtml") => Some(WebviewTarget { path: self.file_path.clone(), fragment: None }),
			Some("md" | "markdown") => {
				let digest = compute_document_hash(&self.file_path);
				let hash = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(digest);
				let doc_temp_dir = Path::new(temp_dir).join(format!("paperback_{hash}"));
				if fs::create_dir_all(&doc_temp_dir).is_ok() {
					let html_path = doc_temp_dir.join("document.html");
					if let Ok(bytes) = fs::read(&self.file_path) {
						let markdown_text = convert_to_utf8(&bytes);
						let html_body = parser::markdown::markdown_to_html(&markdown_text);
						let full_html =
							format!("<html><head><meta charset=\"utf-8\"></head><body>{html_body}</body></html>");
						if fs::write(&html_path, full_html.as_bytes()).is_ok() {
							return Some(WebviewTarget {
								path: html_path.to_string_lossy().to_string(),
								fragment: None,
							});
						}
					}
				}
				None
			}
			_ => None,
		}
	}

	/// Inserts an empty anchor element at the current reading position into the
	/// extracted section file and returns its id, for use as a URL `#fragment`.
	fn inject_reading_anchor(&self, position: i64, file_path: &str) -> Option<String> {
		const READING_POS_ANCHOR_ID: &str = "paperback-reading-pos";
		let pos = usize::try_from(position.max(0)).unwrap_or(0);
		let section_index = self.handle.current_marker_index(pos, MarkerType::SectionBreak)?;
		let section_start = self.handle.document().buffer.markers.get(section_index)?.position;
		let relative = pos.checked_sub(section_start)?;
		let content = convert_to_utf8(&fs::read(file_path).ok()?);
		let injected =
			parser::convert::xml_to_text::inject_anchor_at_position(&content, relative, READING_POS_ANCHOR_ID)?;
		fs::write(file_path, injected.as_bytes()).ok()?;
		Some(READING_POS_ANCHOR_ID.to_string())
	}

	/// Returns the id of the element closest at-or-before `position` in the current
	/// section, for use as a `#fragment` when opening the section in a web view.
	#[must_use]
	pub fn webview_fragment_for_position(&self, position: i64) -> Option<String> {
		let pos = usize::try_from(position.max(0)).unwrap_or(0);
		nearest_fragment_before(&self.handle, pos).map(|id| encode_url_fragment(&id))
	}

	fn is_epub(&self) -> bool {
		self.file_path.to_lowercase().ends_with(".epub")
	}

	/// Returns true when the document's underlying source can be shown as text.
	#[must_use]
	pub fn source_view_available(&self) -> bool {
		if self.is_epub() {
			return true;
		}
		let ext = Path::new(&self.file_path).extension().map(|ext| ext.to_string_lossy().to_ascii_lowercase());
		matches!(ext.as_deref(), Some("html" | "htm" | "xhtml" | "md" | "markdown"))
	}

	/// Writes the underlying source of the document at `position` to a temp `.txt`
	/// file and returns its path plus the caret offset matching the reading position.
	///
	/// For EPUB the current spine section is used; for standalone HTML/XHTML and
	/// Markdown the original file is used. The caret is mapped to the source the
	/// same way the web view positions it: HTML/XHTML/EPUB via the source byte
	/// offset of the element at the reading position, Markdown via the nearest
	/// block anchor. Returns `None` for formats without a meaningful text source.
	#[must_use]
	pub fn view_source(&self, position: i64, temp_dir: &str) -> Option<SourceView> {
		let (content, caret, name) = self.source_content_for_position(position)?;
		let digest = compute_document_hash(&self.file_path);
		let hash = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(digest);
		let doc_temp_dir = Path::new(temp_dir).join(format!("paperback_{hash}"));
		fs::create_dir_all(&doc_temp_dir).ok()?;
		let output_path = doc_temp_dir.join(format!("{name}.source.txt"));
		fs::write(&output_path, content.as_bytes()).ok()?;
		Some(SourceView { path: output_path.to_string_lossy().to_string(), caret: i64::try_from(caret).unwrap_or(0) })
	}

	/// Returns `(source_text, caret_char_offset, file_name)` for the document at
	/// `position`. The caret is mapped into the returned source text.
	fn source_content_for_position(&self, position: i64) -> Option<(String, usize, String)> {
		let pos = usize::try_from(position.max(0)).unwrap_or(0);
		if self.is_epub() {
			let section_path = self.get_current_section_path(position).filter(|path| !path.is_empty())?;
			let file = File::open(&self.file_path).ok()?;
			let mut archive = ZipArchive::new(BufReader::new(file)).ok()?;
			let content = zip_utils::read_zip_entry_by_name(&mut archive, &section_path).ok()?;
			let section_index = self.handle.current_marker_index(pos, MarkerType::SectionBreak)?;
			let section_start = self.handle.document().buffer.markers.get(section_index)?.position;
			let relative = pos.saturating_sub(section_start);
			let caret = Self::xml_caret(&content, relative);
			let name = Path::new(&section_path).file_name()?.to_string_lossy().to_string();
			return Some((content, caret, name));
		}
		let ext = Path::new(&self.file_path).extension().map(|ext| ext.to_string_lossy().to_ascii_lowercase());
		let name = Path::new(&self.file_path).file_name()?.to_string_lossy().to_string();
		let content = convert_to_utf8(&fs::read(&self.file_path).ok()?);
		let caret = match ext.as_deref() {
			Some("html" | "htm" | "xhtml") => Self::xml_caret(&content, pos),
			Some("md" | "markdown") => self.markdown_caret(&content, pos),
			_ => return None,
		};
		Some((content, caret, name))
	}

	/// Maps a rendered character position to a caret offset in XML/HTML source
	/// via the byte offset of the element at that position.
	fn xml_caret(content: &str, relative: usize) -> usize {
		parser::convert::xml_to_text::XmlToText::new()
			.find_anchor_byte_offset(content, relative)
			.and_then(|byte| Some(content.get(..byte)?.chars().count()))
			.unwrap_or(0)
	}

	/// Maps a rendered character position to a caret offset in Markdown source
	/// via the nearest `pb-block-N` anchor recorded during parsing.
	fn markdown_caret(&self, content: &str, pos: usize) -> usize {
		nearest_fragment_before(&self.handle, pos)
			.and_then(|id| id.strip_prefix("pb-block-").and_then(|n| n.parse::<usize>().ok()))
			.and_then(|index| parser::markdown::block_source_offset(content, index))
			.and_then(|byte| Some(content.get(..byte)?.chars().count()))
			.unwrap_or(0)
	}

	/// Extracts every entry of the EPUB into `doc_temp_dir`, preserving the
	/// archive's internal directory structure, so that anything referenced
	/// relatively from a spine section is present on disk wherever a webview
	/// loading that section would look for it. That covers resources
	/// (images, stylesheets, fonts, ...) and the other spine sections, which a
	/// table of contents links to. Runs at most once per `doc_temp_dir`;
	/// subsequent calls are a no-op.
	fn ensure_epub_resources_extracted(&self, doc_temp_dir: &Path) -> anyhow::Result<()> {
		if !self.is_epub() {
			return Ok(());
		}
		// Versioned so a temp directory written by a release that left markup out is
		// refilled rather than trusted as complete.
		let marker = doc_temp_dir.join(".resources_extracted_v2");
		if marker.exists() {
			return Ok(());
		}
		let file = File::open(&self.file_path)?;
		let mut archive = ZipArchive::new(BufReader::new(file))?;
		// Nothing is skipped. The caller re-extracts the current section fresh
		// afterwards, so the anchor-free copy written here does not matter.
		zip_utils::extract_zip_to_dir(&mut archive, doc_temp_dir, |_| false)?;
		fs::write(&marker, b"").ok();
		Ok(())
	}

	/// # Errors
	///
	/// Returns an error if the EPUB cannot be opened or the resource cannot be written.
	pub fn extract_resource(&self, resource_path: &str, output_path: &str) -> anyhow::Result<bool> {
		if self.is_epub() {
			let file = File::open(&self.file_path)?;
			let mut archive = ZipArchive::new(BufReader::new(file))?;
			zip_utils::extract_zip_entry_to_file(&mut archive, resource_path, Path::new(output_path))?;
			Ok(true)
		} else {
			Ok(false)
		}
	}

	/// Exports the document content to a file.
	///
	/// # Errors
	///
	/// Returns an error if the file cannot be written.
	pub fn export_as(&self, output_path: &str, format: ExportFormat) -> io::Result<()> {
		let content = render(&self.handle, format);
		let mut file = File::create(output_path)?;
		file.write_all(content.as_bytes())?;
		file.flush()?;
		Ok(())
	}

	#[must_use]
	pub fn get_supported_export_formats_ffi(&self) -> Vec<ExportFormat> {
		vec![ExportFormat::Text, ExportFormat::Html, ExportFormat::Markdown]
	}

	#[must_use]
	pub fn render_export_ffi(&self, format: ExportFormat) -> String {
		render(&self.handle, format)
	}
}
