use std::{
	fs::{self, File},
	io::{self, BufReader, Write},
	ops::Range,
	path::{Path, PathBuf},
};

use base64::Engine;
use zip::ZipArchive;

use super::{
	DocumentSession, SourceView, WebviewTarget,
	window::{WHOLE_DOCUMENT_LEN, WINDOW_LEN, snap_end_to_paragraph_boundary, snap_start_to_paragraph_boundary},
};
use crate::{
	config::compute_document_hash,
	document::{DocLen, DocPos, MarkerType},
	export::{ExportFormat, html as export_html, render},
	parser,
	reader_core::{encode_url_fragment, nearest_fragment_before},
	util::{encoding::convert_to_utf8, html::MATHML_STYLES, zip as zip_utils},
};

// A book of tens of millions of characters locks the machine up while the web engine lays out all of it.
const MAX_WEBVIEW_LEN: DocLen = WINDOW_LEN;

// The reader's threshold, so the two always agree about whether a document is shown whole.
const WHOLE_WEBVIEW_LEN: DocLen = WHOLE_DOCUMENT_LEN;

#[cfg_attr(feature = "uniffi", uniffi::export)]
impl DocumentSession {
	#[must_use]
	pub fn get_supported_export_formats_ffi(&self) -> Vec<ExportFormat> {
		vec![ExportFormat::Text, ExportFormat::Html, ExportFormat::Markdown]
	}

	#[must_use]
	pub fn render_export_ffi(&self, format: ExportFormat) -> String {
		render(&self.handle, format)
	}
}

impl DocumentSession {
	#[must_use]
	pub fn webview_target_path(&self, position: DocPos, temp_dir: &str) -> Option<WebviewTarget> {
		let section_path = self.get_current_section_path(position).filter(|path| !path.is_empty());
		if let Some(section_path) = section_path
			&& let Some(doc_temp_dir) = self.document_temp_dir(temp_dir)
		{
			// The whole archive, so the section's relative references resolve on disk: its images and stylesheets, and its links to other sections, which is what a table of contents is.
			let _ = self.ensure_epub_resources_extracted(&doc_temp_dir);
			// Re-extracted fresh so the reading-position anchor goes into a clean copy.
			let output_path = doc_temp_dir.join(&section_path);
			let output_str = output_path.to_string_lossy().to_string();
			if self.extract_resource(&section_path, &output_str).ok() == Some(true) {
				let fragment = self.inject_reading_anchor(position, &output_str);
				return Some(WebviewTarget { path: output_str, fragment });
			}
		}
		let ext = Path::new(&self.file_path).extension().map(|ext| ext.to_string_lossy().to_ascii_lowercase());
		match ext.as_deref() {
			Some("html" | "htm" | "xhtml") => Some(WebviewTarget { path: self.file_path.clone(), fragment: None }),
			Some("md" | "markdown") => {
				if let Some(doc_temp_dir) = self.document_temp_dir(temp_dir) {
					let html_path = doc_temp_dir.join("document.html");
					if let Ok(bytes) = fs::read(&self.file_path) {
						let markdown_text = convert_to_utf8(&bytes);
						let html_body = parser::markdown::markdown_to_html(
							&markdown_text,
							self.parse_settings.markdown_dollar_math,
						);
						let full_html = format!(
							"<html><head><meta charset=\"utf-8\"><style>{MATHML_STYLES}</style></head><body>{html_body}</body></html>"
						);
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
			// Every other format has no markup of its own to open, so the parsed document is rendered as HTML.
			_ => {
				if self.handle.document().buffer.content.trim().is_empty() {
					return None;
				}
				let doc_temp_dir = self.document_temp_dir(temp_dir)?;
				let html_path = doc_temp_dir.join("document.html");
				let offset = position.min(self.handle.document().buffer.end());
				let html = export_html::render_range(&self.handle, self.webview_range(offset), Some(offset));
				fs::write(&html_path, html.as_bytes()).ok()?;
				Some(WebviewTarget {
					path: html_path.to_string_lossy().to_string(),
					fragment: Some(export_html::anchor_id(offset)),
				})
			}
		}
	}

	fn webview_range(&self, position: DocPos) -> Range<DocPos> {
		let buffer = &self.handle.document().buffer;
		if buffer.len() <= WHOLE_WEBVIEW_LEN {
			return DocPos::ZERO..buffer.end();
		}
		// Stays a full window wide at either end of the book rather than running off it.
		let position = position.min(buffer.end());
		let half = DocLen::from_raw(MAX_WEBVIEW_LEN.get() / 2);
		let end = (position + half).min(buffer.end()).max(MAX_WEBVIEW_LEN.as_pos());
		let start = end - MAX_WEBVIEW_LEN;
		let byte_start = snap_start_to_paragraph_boundary(buffer, buffer.byte_at(start));
		let byte_end = snap_end_to_paragraph_boundary(buffer, buffer.byte_at(end));
		buffer.pos_at_byte(byte_start)..buffer.pos_at_byte(byte_end)
	}

	fn document_temp_dir(&self, temp_dir: &str) -> Option<PathBuf> {
		let digest = compute_document_hash(&self.file_path);
		let hash = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(digest);
		let doc_temp_dir = Path::new(temp_dir).join(format!("paperback_{hash}"));
		fs::create_dir_all(&doc_temp_dir).ok()?;
		Some(doc_temp_dir)
	}

	fn inject_reading_anchor(&self, position: DocPos, file_path: &str) -> Option<String> {
		const READING_POS_ANCHOR_ID: &str = "paperback-reading-pos";
		let pos = position.min(self.handle.document().buffer.end());
		let section_index = self.handle.current_marker_index(pos, MarkerType::SectionBreak)?;
		let section_start = self.handle.document().buffer.markers.get(section_index)?.position;
		let relative = (pos >= section_start).then(|| (pos - section_start).as_pos())?;
		let content = convert_to_utf8(&fs::read(file_path).ok()?);
		let injected =
			parser::convert::xml_to_text::inject_anchor_at_position(&content, relative, READING_POS_ANCHOR_ID)?;
		fs::write(file_path, injected.as_bytes()).ok()?;
		Some(READING_POS_ANCHOR_ID.to_string())
	}

	#[must_use]
	pub fn webview_fragment_for_position(&self, position: DocPos) -> Option<String> {
		let pos = position.min(self.handle.document().buffer.end());
		nearest_fragment_before(&self.handle, pos).map(|id| encode_url_fragment(&id))
	}

	fn is_epub(&self) -> bool {
		self.file_path.to_lowercase().ends_with(".epub")
	}

	#[must_use]
	pub fn source_view_available(&self) -> bool {
		if self.is_epub() {
			return true;
		}
		let ext = Path::new(&self.file_path).extension().map(|ext| ext.to_string_lossy().to_ascii_lowercase());
		matches!(ext.as_deref(), Some("html" | "htm" | "xhtml" | "md" | "markdown"))
	}

	#[must_use]
	pub fn view_source(&self, position: DocPos, temp_dir: &str) -> Option<SourceView> {
		let (content, caret, name) = self.source_content_for_position(position)?;
		let doc_temp_dir = self.document_temp_dir(temp_dir)?;
		let output_path = doc_temp_dir.join(format!("{name}.source.txt"));
		fs::write(&output_path, content.as_bytes()).ok()?;
		Some(SourceView { path: output_path.to_string_lossy().to_string(), caret })
	}

	fn source_content_for_position(&self, position: DocPos) -> Option<(String, DocPos, String)> {
		let pos = position.min(self.handle.document().buffer.end());
		if self.is_epub() {
			let section_path = self.get_current_section_path(position).filter(|path| !path.is_empty())?;
			let file = File::open(&self.file_path).ok()?;
			let mut archive = ZipArchive::new(BufReader::new(file)).ok()?;
			let content = zip_utils::read_zip_entry_by_name(&mut archive, &section_path).ok()?;
			let section_index = self.handle.current_marker_index(pos, MarkerType::SectionBreak)?;
			let section_start = self.handle.document().buffer.markers.get(section_index)?.position;
			let relative = pos.since(section_start).as_pos();
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

	fn xml_caret(content: &str, relative: DocPos) -> DocPos {
		parser::convert::xml_to_text::XmlToText::new()
			.find_anchor_byte_offset(content, relative)
			.and_then(|byte| Some(DocLen::of(content.get(..byte)?).as_pos()))
			.unwrap_or_default()
	}

	fn markdown_caret(&self, content: &str, pos: DocPos) -> DocPos {
		nearest_fragment_before(&self.handle, pos)
			.and_then(|id| id.strip_prefix("pb-block-").and_then(|n| n.parse::<usize>().ok()))
			.and_then(|index| {
				parser::markdown::block_source_offset(content, index, self.parse_settings.markdown_dollar_math)
			})
			.and_then(|byte| Some(DocLen::of(content.get(..byte)?).as_pos()))
			.unwrap_or_default()
	}

	fn ensure_epub_resources_extracted(&self, doc_temp_dir: &Path) -> anyhow::Result<()> {
		if !self.is_epub() {
			return Ok(());
		}
		// Versioned so a temp directory written by a release that left markup out is refilled rather than trusted as complete.
		let marker = doc_temp_dir.join(".resources_extracted_v2");
		if marker.exists() {
			return Ok(());
		}
		let file = File::open(&self.file_path)?;
		let mut archive = ZipArchive::new(BufReader::new(file))?;
		// The caller re-extracts the current section afterwards, so its anchor-free copy here does not matter.
		zip_utils::extract_zip_to_dir(&mut archive, doc_temp_dir, |_| false)?;
		fs::write(&marker, b"").ok();
		Ok(())
	}

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

	pub fn export_as(&self, output_path: &str, format: ExportFormat) -> io::Result<()> {
		let content = render(&self.handle, format);
		let mut file = File::create(output_path)?;
		file.write_all(content.as_bytes())?;
		file.flush()?;
		Ok(())
	}
}
