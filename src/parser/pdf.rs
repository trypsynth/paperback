use std::collections::HashMap;

use anyhow::{Result, anyhow};
use pdfium::{PdfiumDocument, PdfiumError, lib};

use crate::{
	document::{Document, DocumentBuffer, Marker, MarkerType, ParserContext, ParserFlags, TocItem},
	parser::{PASSWORD_REQUIRED_ERROR_PREFIX, Parser, path::extract_title_from_path},
	text::{collapse_whitespace, display_len, trim_string},
};

pub struct PdfParser;

impl Parser for PdfParser {
	fn name(&self) -> &'static str {
		"PDF Documents"
	}

	fn extensions(&self) -> &[&str] {
		&["pdf"]
	}

	fn supported_flags(&self) -> ParserFlags {
		ParserFlags::SUPPORTS_PAGES | ParserFlags::SUPPORTS_TOC
	}

	fn parse(&self, context: &ParserContext) -> Result<Document> {
		let document =
			PdfiumDocument::new_from_path(&context.file_path, context.password.as_deref()).map_err(map_load_error)?;
		let mut buffer = DocumentBuffer::new();
		let mut page_offsets = Vec::new();
		let mut id_positions = HashMap::new();
		let mut page_lines_info: Vec<Vec<(usize, String)>> = Vec::new();
		let mut heuristic_headings: Vec<(usize, String, f64, f64)> = Vec::new(); // offset, line text, font_size, font_weight
		let page_count = document.page_count();
		for page_index in 0..page_count {
			let marker_position = buffer.current_position();
			page_offsets.push(marker_position);
			id_positions.insert(format!("page_{}", page_index), marker_position);
			buffer.add_marker(
				Marker::new(MarkerType::PageBreak, marker_position).with_text(format!("Page {}", page_index + 1)),
			);
			let Ok(page) = document.page(page_index) else {
				page_lines_info.push(Vec::new());
				continue;
			};
			let Ok(text_page) = page.text() else {
				page_lines_info.push(Vec::new());
				continue;
			};
			let raw_text = text_page.full();
			let lines = process_text_lines(&raw_text);

			let page_start_offset = buffer.current_position();
			let mut page_display_text = String::new();
			let mut current_lines_info = Vec::new();
			let mut current_offset = page_start_offset;
			let mut last_search_pos = 0;
			let mut last_utf16_pos = 0;

			for line in lines {
				let first_word = line.split_whitespace().next().unwrap_or("");
				let mut line_font_size = 0.0;
				let mut line_font_weight = 400.0;
				if !first_word.is_empty() {
					// Search for the word boundary to prevent partial matches like "In" inside "Index"
					let mut search_idx = last_search_pos;
					while let Some(pos) = raw_text[search_idx..].find(first_word) {
						let char_idx = search_idx + pos;

						// Basic word boundary check: is the preceding character whitespace (or start of text)?
						let is_valid_start =
							char_idx == 0 || raw_text[..char_idx].ends_with(|c: char| c.is_whitespace());

						if is_valid_start {
							// Update UTF-16 index efficiently without O(N^2) recount
							let utf16_chunk_len = raw_text[last_search_pos..char_idx].encode_utf16().count();
							last_utf16_pos += utf16_chunk_len;

							line_font_size = text_page.get_font_size(last_utf16_pos as i32);
							line_font_weight = text_page.get_font_weight(last_utf16_pos as i32).unwrap_or(400) as f64;

							last_search_pos = char_idx + first_word.len();
							last_utf16_pos += first_word.encode_utf16().count();
							break;
						}
						search_idx = char_idx + first_word.len();
					}
				}

				if line_font_size > 0.0 {
					heuristic_headings.push((current_offset, line.clone(), line_font_size, line_font_weight));
				}

				current_lines_info.push((current_offset, line.clone()));
				current_offset += display_len(&line) + 1;

				buffer.append(&line);
				buffer.append("\n");
				page_display_text.push_str(&line);
				page_display_text.push('\n');
			}

			// Load implicit web links
			if let Ok(links) = text_page.load_web_links() {
				let count = lib().FPDFLink_CountWebLinks(&links);
				let mut last_search_pos = 0;
				for i in 0..count {
					let mut start = 0;
					let mut char_count = 0;
					if lib().FPDFLink_GetTextRange(&links, i, &mut start, &mut char_count).is_ok() {
						let link_text = text_page.extract(start, char_count);
						let trimmed_link = trim_string(&collapse_whitespace(&link_text));
						if trimmed_link.is_empty() {
							continue;
						}

						let mut url_buffer = vec![0u16; 2048];
						let len = lib().FPDFLink_GetURL(&links, i, &mut url_buffer[0], 2048);
						if len > 0 {
							let url = String::from_utf16_lossy(&url_buffer[..(len as usize - 1)]);
							if let Some(pos) = page_display_text[last_search_pos..].find(&trimmed_link) {
								let text_before = &page_display_text[last_search_pos..last_search_pos + pos];
								let marker_pos = page_start_offset
									+ display_len(&page_display_text[..last_search_pos])
									+ display_len(text_before);
								let link_len = display_len(&trimmed_link);
								buffer.add_marker(
									Marker::new(MarkerType::Link, marker_pos)
										.with_text(trimmed_link.clone())
										.with_reference(url)
										.with_length(link_len),
								);
								last_search_pos += pos + trimmed_link.len();
							}
						}
					}
				}
			}

			// Load explicit annotations (internal and external links)
			let annot_count = lib().FPDFPage_GetAnnotCount(&page);
			let mut last_search_pos = 0;
			for i in 0..annot_count {
				if let Ok(annot) = lib().FPDFPage_GetAnnot(&page, i) {
					if lib().FPDFAnnot_GetSubtype(&annot) == pdfium::pdfium_constants::FPDF_ANNOT_LINK {
						let mut rect = pdfium::pdfium_types::FS_RECTF { left: 0.0, top: 0.0, right: 0.0, bottom: 0.0 };
						if lib().FPDFAnnot_GetRect(&annot, &mut rect).is_ok() {
							let mut text_buffer = vec![0u16; 2048];
							let len = lib().FPDFText_GetBoundedText(
								&text_page,
								rect.left as f64,
								rect.top as f64,
								rect.right as f64,
								rect.bottom as f64,
								&mut text_buffer[0],
								2048,
							);
							if len > 0 {
								let text = String::from_utf16_lossy(&text_buffer[..(len as usize - 1)]);
								let trimmed_link = trim_string(&collapse_whitespace(&text));
								if trimmed_link.is_empty() {
									continue;
								}

								let mut url = String::new();
								if let Ok(link) = lib().FPDFAnnot_GetLink(&annot) {
									if let Ok(action) = lib().FPDFLink_GetAction(&link) {
										let action_type = lib().FPDFAction_GetType(&action);
										// PDFACTION_URI is 3
										if action_type == 3 {
											let mut uri_buffer = vec![0u8; 2048];
											let uri_len = lib().FPDFAction_GetURIPath(
												&document,
												&action,
												Some(&mut uri_buffer),
												2048,
											);
											if uri_len > 0 {
												url = String::from_utf8_lossy(&uri_buffer[..(uri_len as usize - 1)])
													.to_string();
											}
										}
									}
									if url.is_empty() {
										let dest_result = lib().FPDFLink_GetDest(&document, &link);
										let dest = dest_result.ok().or_else(|| {
											lib()
												.FPDFLink_GetAction(&link)
												.ok()
												.and_then(|action| lib().FPDFAction_GetDest(&document, &action).ok())
										});
										if let Some(dest) = dest {
											let dest_page = lib().FPDFDest_GetDestPageIndex(&document, &dest);
											if dest_page >= 0 {
												url = format!("#page_{}", dest_page);
											}
										}
									}
								}

								if !url.is_empty() {
									if let Some(pos) = page_display_text[last_search_pos..].find(&trimmed_link) {
										let text_before = &page_display_text[last_search_pos..last_search_pos + pos];
										let marker_pos = page_start_offset
											+ display_len(&page_display_text[..last_search_pos])
											+ display_len(text_before);
										let link_len = display_len(&trimmed_link);
										buffer.add_marker(
											Marker::new(MarkerType::Link, marker_pos)
												.with_text(trimmed_link.clone())
												.with_reference(url)
												.with_length(link_len),
										);
										last_search_pos += pos + trimmed_link.len();
									}
								}
							}
						}
					}
				}
			}

			page_lines_info.push(current_lines_info);
		}

		let title = metadata_value(&document, "Title").unwrap_or_else(|| extract_title_from_path(&context.file_path));
		let author = metadata_value(&document, "Author").unwrap_or_default();
		let toc_items = extract_toc(&document, &page_offsets, &page_lines_info);

		let mut median_font_size = 0.0;
		if !heuristic_headings.is_empty() {
			let mut sizes: Vec<f64> = heuristic_headings.iter().map(|(_, _, size, _)| *size).collect();
			sizes.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
			median_font_size = sizes[sizes.len() / 2];
		}

		let mut median_font_weight = 400.0;
		if !heuristic_headings.is_empty() {
			let mut weights: Vec<f64> = heuristic_headings.iter().map(|(_, _, _, weight)| *weight).collect();
			weights.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
			median_font_weight = weights[weights.len() / 2];
		}

		add_heading_markers(&mut buffer, &toc_items, 1);

		let mut toc_offsets = std::collections::HashSet::new();
		fn collect_toc_offsets(items: &[TocItem], offsets: &mut std::collections::HashSet<usize>) {
			for item in items {
				for i in 0..=50 {
					offsets.insert(item.offset + i);
					offsets.insert(item.offset.saturating_sub(i));
				}
				collect_toc_offsets(&item.children, offsets);
			}
		}
		collect_toc_offsets(&toc_items, &mut toc_offsets);

		for (offset, line, size, weight) in heuristic_headings {
			if toc_offsets.contains(&offset) {
				continue;
			}

			if line.len() < 100
				&& (size > median_font_size + 1.0 || (weight > median_font_weight + 200.0 && size >= median_font_size))
			{
				let has_sentence_punctuation = line.contains(". ")
					|| line.contains("? ")
					|| line.contains("! ")
					|| line.contains(": ")
					|| line.ends_with('.')
					|| line.ends_with('?')
					|| line.ends_with('!');
				let ends_with_continuation = line.ends_with(',')
					|| line.ends_with("and")
					|| line.ends_with("or")
					|| line.ends_with("by")
					|| line.ends_with("the");
				let ends_with_number = line.chars().last().unwrap_or(' ').is_ascii_digit();
				let first_char = line.chars().next().unwrap_or(' ');
				let has_url_indicators = line.contains("http")
					|| line.contains("://")
					|| line.contains(".com")
					|| line.contains(".org")
					|| line.contains(".html")
					|| line.contains('/');

				let is_likely_heading = (first_char.is_uppercase() || first_char.is_ascii_digit())
					&& !has_sentence_punctuation
					&& !ends_with_continuation
					&& !ends_with_number
					&& !has_url_indicators;

				if is_likely_heading {
					let has_marker = buffer.markers.iter().any(|m| {
						m.position == offset
							&& matches!(
								m.mtype,
								MarkerType::Heading1
									| MarkerType::Heading2 | MarkerType::Heading3
									| MarkerType::Heading4 | MarkerType::Heading5
									| MarkerType::Heading6
							)
					});
					if !has_marker {
						buffer.add_marker(
							Marker::new(MarkerType::Heading3, offset).with_text(line.clone()).with_level(3),
						);
					}
				}
			}
		}

		let mut doc = Document::new();
		doc.set_buffer(buffer);
		doc.title = title;
		doc.author = author;
		doc.toc_items = toc_items;
		doc.id_positions = id_positions;
		Ok(doc)
	}
}

fn add_heading_markers(buffer: &mut DocumentBuffer, items: &[TocItem], level: i32) {
	for item in items {
		let marker_type = match level {
			1 => MarkerType::Heading1,
			2 => MarkerType::Heading2,
			3 => MarkerType::Heading3,
			4 => MarkerType::Heading4,
			5 => MarkerType::Heading5,
			_ => MarkerType::Heading6,
		};
		buffer.add_marker(Marker::new(marker_type, item.offset).with_text(item.name.clone()).with_level(level));
		add_heading_markers(buffer, &item.children, level + 1);
	}
}

fn process_text_lines(raw_text: &str) -> Vec<String> {
	let clean_text = raw_text.replace('\r', "");
	clean_text
		.lines()
		.filter_map(|line| {
			let collapsed = collapse_whitespace(line);
			let trimmed = trim_string(&collapsed);
			if trimmed.is_empty() { None } else { Some(trimmed) }
		})
		.collect()
}

fn map_load_error(err: PdfiumError) -> anyhow::Error {
	match err {
		PdfiumError::PasswordError => anyhow!("{PASSWORD_REQUIRED_ERROR_PREFIX}Password required or incorrect"),
		other => anyhow!("Failed to open PDF document: {other}"),
	}
}

fn metadata_value(document: &PdfiumDocument, key: &str) -> Option<String> {
	document.metadata_value(key).ok().map(|value| trim_string(&value)).filter(|value| !value.is_empty())
}

fn extract_toc(
	document: &PdfiumDocument,
	page_offsets: &[usize],
	page_lines_info: &[Vec<(usize, String)>],
) -> Vec<TocItem> {
	let Ok(bookmarks) = document.toc(16) else {
		return Vec::new();
	};
	if bookmarks.is_empty() {
		return Vec::new();
	}

	let mut items = Vec::<(u32, TocItem)>::new();
	let mut used_offsets = std::collections::HashSet::new();

	for bookmark in &bookmarks {
		let Some(level) = bookmark.level() else {
			continue;
		};
		let Ok(raw_title) = bookmark.title() else {
			continue;
		};
		let title = trim_string(&collapse_whitespace(&raw_title));
		if title.is_empty() {
			continue;
		}
		let Ok(dest) = bookmark.dest(document) else {
			continue;
		};
		let Some(page_index) = dest.index(document) else {
			continue;
		};
		let Ok(page_index) = usize::try_from(page_index) else {
			continue;
		};
		let Some(&page_start_offset) = page_offsets.get(page_index) else {
			continue;
		};

		let mut actual_offset = page_start_offset;
		let mut actual_title = title.clone();
		if let Some(lines) = page_lines_info.get(page_index) {
			let title_alpha: String = title.to_lowercase().chars().filter(|c| c.is_alphabetic()).collect();
			for (line_offset, line) in lines {
				let line_alpha: String = line.to_lowercase().chars().filter(|c| c.is_alphabetic()).collect();

				let ends_with_number = line.chars().last().unwrap_or(' ').is_ascii_digit();
				let is_all_caps = line.chars().filter(|c| c.is_alphabetic()).all(char::is_uppercase);
				let is_page_header = ends_with_number && is_all_caps;

				if (line_alpha == title_alpha
					|| line_alpha.starts_with(&title_alpha)
					|| line_alpha.ends_with(&title_alpha))
					&& !title_alpha.is_empty()
					&& !is_page_header
				{
					actual_offset = *line_offset;
					if line.len() < 150 {
						actual_title.clone_from(line);
					}
					break;
				}
			}
		}

		while used_offsets.contains(&actual_offset) {
			actual_offset += 1;
		}
		used_offsets.insert(actual_offset);

		items.push((level, TocItem::new(actual_title, String::new(), actual_offset)));
	}

	build_toc_tree(items)
}

fn build_toc_tree(flat_items: Vec<(u32, TocItem)>) -> Vec<TocItem> {
	let mut root = Vec::<TocItem>::new();
	let mut path = Vec::<usize>::new();
	let mut level_stack = Vec::<u32>::new();

	for (level, item) in flat_items {
		while let Some(&last_level) = level_stack.last() {
			if last_level < level {
				break;
			}
			level_stack.pop();
			path.pop();
		}

		let siblings = children_at_path_mut(&mut root, &path);
		siblings.push(item);
		path.push(siblings.len() - 1);
		level_stack.push(level);
	}

	root
}

fn children_at_path_mut<'a>(nodes: &'a mut Vec<TocItem>, path: &[usize]) -> &'a mut Vec<TocItem> {
	let mut current = nodes;
	for &index in path {
		current = &mut current[index].children;
	}
	current
}
