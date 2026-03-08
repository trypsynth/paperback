use std::collections::{HashMap, HashSet};

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
		let page_count = document.page_count();
		let mut any_tags_processed = false;
		let mut flat_toc_items = Vec::new();
		for page_index in 0..page_count {
			let marker_position = buffer.current_position();
			page_offsets.push(marker_position);
			id_positions.insert(format!("page_{page_index}"), marker_position);
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
			let page_start_offset = buffer.current_position();
			let mut page_display_text = String::new();
			let mut current_lines_info = Vec::new();
			let mut tags_processed = false;
			if let Some(struct_tree) = page.struct_tree() {
				let child_count = struct_tree.count_children();
				if child_count > 0 {
					let mut mcid_to_text: HashMap<i32, String> = HashMap::new();
					if let Ok(char_count) = text_page.char_count() {
						let mut current_mcid = -1;
						let mut current_text = String::new();
						for i in 0..char_count {
							let unicode = text_page.get_unicode(i);
							if let Some(ch) = char::from_u32(unicode) {
								let is_generated = text_page.is_generated(i).unwrap_or(false);
								let mut char_mcid = -1;
								if !is_generated {
									if let Ok(obj) = text_page.get_text_object(i) {
										char_mcid = obj.get_marked_content_id();
									}
								}
								if char_mcid >= 0 && char_mcid != current_mcid {
									if current_mcid >= 0 && !current_text.is_empty() {
										mcid_to_text.entry(current_mcid).or_default().push_str(&current_text);
										current_text.clear();
									}
									current_mcid = char_mcid;
								}
								current_text.push(ch);
							}
						}
						if current_mcid >= 0 && !current_text.is_empty() {
							mcid_to_text.entry(current_mcid).or_default().push_str(&current_text);
						}
					}
					let mut current_block = String::new();
					for i in 0..child_count {
						if let Ok(child) = struct_tree.get_child(i) {
							process_struct_element(
								&child,
								&mcid_to_text,
								&mut buffer,
								&mut page_display_text,
								&mut current_block,
								&mut current_lines_info,
								&mut flat_toc_items,
							);
						}
					}
					flush_block(&mut current_block, &mut buffer, &mut page_display_text, &mut current_lines_info);
					tags_processed = true;
					any_tags_processed = true;
				}
			}
			if !tags_processed {
				let raw_text = text_page.full();
				let lines = process_text_lines(&raw_text);
				let mut current_offset = buffer.current_position();
				for line in lines {
					current_lines_info.push((current_offset, line.clone()));
					current_offset += display_len(&line) + 1;
					buffer.append(&line);
					buffer.append("\n");
					page_display_text.push_str(&line);
					page_display_text.push('\n');
				}
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
				let annot_result = lib().FPDFPage_GetAnnot(&page, i);
				if let Ok(annot) = annot_result {
					if lib().FPDFAnnot_GetSubtype(&annot) == pdfium::pdfium_constants::FPDF_ANNOT_LINK {
						let mut rect = pdfium::pdfium_types::FS_RECTF { left: 0.0, top: 0.0, right: 0.0, bottom: 0.0 };
						if lib().FPDFAnnot_GetRect(&annot, &mut rect).is_ok() {
							let mut text_buffer = vec![0u16; 2048];
							let len = lib().FPDFText_GetBoundedText(
								&text_page,
								f64::from(rect.left),
								f64::from(rect.top),
								f64::from(rect.right),
								f64::from(rect.bottom),
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
								let link_result = lib().FPDFAnnot_GetLink(&annot);
								if let Ok(link) = link_result {
									let action_result = lib().FPDFLink_GetAction(&link);
									if let Ok(action) = action_result {
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
												url = format!("#page_{dest_page}");
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
		let mut toc_items = extract_toc(&document, &page_offsets, &page_lines_info);
		if any_tags_processed {
			if toc_items.is_empty() {
				toc_items = build_toc_tree(flat_toc_items);
			} else if flat_toc_items.is_empty() {
				add_heading_markers(&mut buffer, &toc_items, 1);
			}
		} else {
			add_heading_markers(&mut buffer, &toc_items, 1);
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

fn is_cjk(c: char) -> bool {
	let u = c as u32;
	(0x4E00..=0x9FFF).contains(&u) || // CJK Unified Ideographs
	(0x3400..=0x4DBF).contains(&u) || // CJK Extension A
	(0x20000..=0x2A6DF).contains(&u) || // CJK Extension B
	(0x3040..=0x309F).contains(&u) || // Hiragana
	(0x30A0..=0x30FF).contains(&u) || // Katakana
	(0xAC00..=0xD7AF).contains(&u) // Hangul
}

fn process_text_lines(raw_text: &str) -> Vec<String> {
	let clean_text = raw_text.replace('\r', "");
	let lines: Vec<String> = clean_text.lines().map(|line| trim_string(&collapse_whitespace(line))).collect();
	let mut max_len = 0;
	for line in &lines {
		let len = display_len(line);
		if len > max_len {
			max_len = len;
		}
	}
	let short_line_threshold = (max_len as f32 * 0.75) as usize;
	let mut paragraphs = Vec::new();
	let mut current_paragraph = String::new();
	let mut last_line_len = 0;
	let mut last_line_ends_with_punctuation = false;
	for line in lines {
		if line.is_empty() {
			if !current_paragraph.is_empty() {
				paragraphs.push(current_paragraph.clone());
				current_paragraph.clear();
			}
			continue;
		}
		let is_list_item = line.starts_with("- ") || line.starts_with("* ") || line.starts_with("• ");
		let starts_with_uppercase = line.chars().next().is_some_and(char::is_uppercase);
		let len = display_len(&line);
		if current_paragraph.is_empty() {
			current_paragraph = line.clone();
		} else {
			let mut is_numbered = false;
			let mut chars = line.chars();
			if let Some(first_char) = chars.next() {
				if first_char.is_ascii_digit() {
					let mut found_space = false;
					for c in chars {
						if c.is_ascii_digit() || c == '.' || c == ')' {
							continue;
						} else if c.is_whitespace() {
							found_space = true;
							break;
						} else {
							break;
						}
					}
					is_numbered = found_space;
				}
			}
			let break_paragraph = if is_list_item || is_numbered {
				true
			} else if last_line_ends_with_punctuation && last_line_len < short_line_threshold {
				true
			} else {
				last_line_len < short_line_threshold && starts_with_uppercase
			};
			if break_paragraph {
				paragraphs.push(current_paragraph.clone());
				current_paragraph = line.clone();
			} else {
				let last_char = current_paragraph.chars().last().unwrap_or(' ');
				if current_paragraph.ends_with('-') {
					current_paragraph.pop();
					current_paragraph.push_str(&line);
				} else if is_cjk(last_char) && line.chars().next().is_some_and(is_cjk) {
					current_paragraph.push_str(&line);
				} else {
					current_paragraph.push(' ');
					current_paragraph.push_str(&line);
				}
			}
		}
		last_line_len = len;
		last_line_ends_with_punctuation = line.ends_with('.')
			|| line.ends_with('?')
			|| line.ends_with('!')
			|| line.ends_with(':')
			|| line.ends_with('”')
			|| line.ends_with('"')
			|| line.ends_with('。')
			|| line.ends_with('？')
			|| line.ends_with('！')
			|| line.ends_with('：');
	}
	if !current_paragraph.is_empty() {
		paragraphs.push(current_paragraph);
	}
	paragraphs
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
	let mut used_offsets = HashSet::new();
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

fn flush_block(
	current_block: &mut String,
	buffer: &mut DocumentBuffer,
	page_display_text: &mut String,
	current_lines_info: &mut Vec<(usize, String)>,
) {
	let trimmed = trim_string(&collapse_whitespace(current_block));
	if !trimmed.is_empty() {
		let offset = buffer.current_position();
		current_lines_info.push((offset, trimmed.clone()));
		buffer.append(&trimmed);
		buffer.append("\n");
		page_display_text.push_str(&trimmed);
		page_display_text.push('\n');
	}
	current_block.clear();
}

fn process_struct_element(
	elem: &pdfium::PdfiumStructElement,
	mcid_to_text: &HashMap<i32, String>,
	buffer: &mut DocumentBuffer,
	page_display_text: &mut String,
	current_block: &mut String,
	current_lines_info: &mut Vec<(usize, String)>,
	toc_items: &mut Vec<(u32, TocItem)>,
) {
	let elem_type = elem.element_type().unwrap_or_default();
	if elem_type == "Table" {
		flush_block(current_block, buffer, page_display_text, current_lines_info);
		let html = build_html_table(elem, mcid_to_text);
		let pos = buffer.current_position();
		buffer.add_marker(Marker::new(MarkerType::Table, pos).with_reference(html));
		let table_placeholder = "[Table]";
		current_lines_info.push((pos, table_placeholder.to_string()));
		buffer.append(table_placeholder);
		buffer.append("\n");
		page_display_text.push_str(table_placeholder);
		page_display_text.push('\n');
		return;
	}
	let is_block = matches!(
		elem_type.as_str(),
		"P" | "H"
			| "H1" | "H2"
			| "H3" | "H4"
			| "H5" | "H6"
			| "L" | "LI"
			| "Div" | "Sect"
			| "Part" | "Art"
			| "TOC" | "TOCI"
	);
	if is_block {
		flush_block(current_block, buffer, page_display_text, current_lines_info);
	}
	let block_start_pos = buffer.current_position() + display_len(current_block);

	let count = elem.count_children();
	for i in 0..count {
		if let Ok(child) = elem.get_child(i) {
			process_struct_element(
				&child,
				mcid_to_text,
				buffer,
				page_display_text,
				current_block,
				current_lines_info,
				toc_items,
			);
		} else {
			let mcid = elem.child_marked_content_id(i);
			if mcid >= 0 {
				if let Some(text) = mcid_to_text.get(&mcid) {
					current_block.push_str(text);
				}
			}
		}
	}
	if is_block {
		flush_block(current_block, buffer, page_display_text, current_lines_info);
		let heading_level = match elem_type.as_str() {
			"H1" => Some(1),
			"H2" => Some(2),
			"H3" => Some(3),
			"H4" => Some(4),
			"H5" => Some(5),
			"H6" => Some(6),
			"H" => Some(1), // Fallback generic heading to H1
			_ => None,
		};
		if let Some(level) = heading_level {
			let mut title = String::new();
			collect_text(elem, mcid_to_text, &mut title);
			let title = trim_string(&collapse_whitespace(&title));
			if !title.is_empty() {
				let marker_type = match level {
					1 => MarkerType::Heading1,
					2 => MarkerType::Heading2,
					3 => MarkerType::Heading3,
					4 => MarkerType::Heading4,
					5 => MarkerType::Heading5,
					_ => MarkerType::Heading6,
				};
				buffer.add_marker(Marker::new(marker_type, block_start_pos).with_text(title.clone()).with_level(level));
				toc_items.push((level as u32, TocItem::new(title, String::new(), block_start_pos)));
			}
		}
		if elem_type == "L" || elem_type == "TOC" {
			let child_count = elem.count_children();
			buffer.add_marker(Marker::new(MarkerType::List, block_start_pos).with_level(child_count));
		}
		if elem_type == "LI" || elem_type == "TOCI" {
			let mut li_text = String::new();
			collect_text(elem, mcid_to_text, &mut li_text);
			let li_text = trim_string(&collapse_whitespace(&li_text));
			buffer.add_marker(Marker::new(MarkerType::ListItem, block_start_pos).with_text(li_text));
		}
	}
}

fn build_html_table(elem: &pdfium::PdfiumStructElement, mcid_to_text: &HashMap<i32, String>) -> String {
	let elem_type = elem.element_type().unwrap_or_default();
	if elem_type == "Table" {
		let mut html = String::from("<table border=\"1\">\n");
		let count = elem.count_children();
		for i in 0..count {
			if let Ok(child) = elem.get_child(i) {
				html.push_str(&build_html_table(&child, mcid_to_text));
			}
		}
		html.push_str("</table>\n");
		html
	} else if elem_type == "TR" {
		let mut html = String::from("<tr>\n");
		let count = elem.count_children();
		for i in 0..count {
			if let Ok(child) = elem.get_child(i) {
				html.push_str(&build_html_table(&child, mcid_to_text));
			}
		}
		html.push_str("</tr>\n");
		html
	} else if elem_type == "TH" || elem_type == "TD" {
		let mut html = format!("<{}>", elem_type.to_lowercase());
		let mut cell_text = String::new();
		collect_text(elem, mcid_to_text, &mut cell_text);
		html.push_str(&html_escape(&trim_string(&collapse_whitespace(&cell_text))));
		html.push_str(&format!("</{}>\n", elem_type.to_lowercase()));
		html
	} else {
		let mut html = String::new();
		let count = elem.count_children();
		for i in 0..count {
			if let Ok(child) = elem.get_child(i) {
				html.push_str(&build_html_table(&child, mcid_to_text));
			}
		}
		html
	}
}

fn collect_text(elem: &pdfium::PdfiumStructElement, mcid_to_text: &HashMap<i32, String>, out: &mut String) {
	let count = elem.count_children();
	for i in 0..count {
		if let Ok(child) = elem.get_child(i) {
			collect_text(&child, mcid_to_text, out);
		} else {
			let mcid = elem.child_marked_content_id(i);
			if mcid >= 0 {
				if let Some(text) = mcid_to_text.get(&mcid) {
					out.push_str(text);
				}
			}
		}
	}
}

fn html_escape(s: &str) -> String {
	s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}
