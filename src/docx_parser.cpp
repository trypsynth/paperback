/* docx_parser.cpp - parser for docx documents.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#include "docx_parser.hpp"
#include "document.hpp"
#include "document_buffer.hpp"
#include "utils.hpp"
#include <algorithm>
#include <cctype>
#include <cstddef>
#include <map>
#include <memory>
#include <pugixml.hpp>
#include <sstream>
#include <string>
#include <vector>
#include <wx/filename.h>
#include <wx/string.h>
#include <wx/translation.h>
#include <wx/wfstream.h>
#include <wx/zipstrm.h>

inline const char* WORDML_NS = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";
inline const char* REL_NS = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";

static std::string get_local_name(const char* qname) {
	if (!qname) {
		return {};
	}
	std::string s(qname);
	size_t pos{s.find(':')};
	return pos == std::string::npos ? s : s.substr(pos + 1);
}

std::unique_ptr<document> docx_parser::load(const wxString& path) const {
	auto fp = std::make_unique<wxFileInputStream>(path);
	if (!fp->IsOk()) {
		return nullptr;
	}
	wxZipInputStream zip(*fp);
	if (!zip.IsOk()) {
		return nullptr;
	}
	std::string rels_content;
	std::string doc_content;
	std::unique_ptr<wxZipEntry> entry;
	while ((entry.reset(zip.GetNextEntry())), entry != nullptr) {
		const std::string entry_name = entry->GetInternalName().ToStdString();
		if (entry_name == "word/_rels/document.xml.rels") {
			rels_content = read_zip_entry(zip);
		} else if (entry_name == "word/document.xml") {
			doc_content = read_zip_entry(zip);
		}
		if (!rels_content.empty() && !doc_content.empty()) {
			break;
		}
	}
	if (doc_content.empty()) {
		return nullptr;
	}
	std::map<std::string, std::string> rels;
	if (!rels_content.empty()) {
		pugi::xml_document rels_doc;
		if (rels_doc.load_buffer(rels_content.data(), rels_content.size())) {
			for (auto rel : rels_doc.child("Relationships").children("Relationship")) {
				std::string id = rel.attribute("Id").as_string();
				std::string target = rel.attribute("Target").as_string();
				std::string type = rel.attribute("Type").as_string();
				if (type == "http://schemas.openxmlformats.org/officeDocument/2006/relationships/hyperlink") {
					rels[id] = target;
				}
			}
		}
	}
	pugi::xml_document p_doc;
	if (!p_doc.load_buffer(doc_content.data(), doc_content.size(), pugi::parse_default | pugi::parse_ws_pcdata)) {
		return nullptr;
	}
	auto doc = std::make_unique<document>();
	doc->title = wxFileName(path).GetName();
	doc->buffer.clear();
	wxString text;
	std::vector<heading_info> headings;
	traverse(p_doc.document_element(), text, headings, doc.get(), rels);
	doc->buffer.set_content(text);
	for (const auto& heading : headings) {
		const auto type = static_cast<marker_type>(static_cast<int>(marker_type::heading_1) + heading.level - 1);
		doc->buffer.add_marker(heading.offset, type, wxString::FromUTF8(heading.text), wxString(), heading.level);
	}
	doc->buffer.finalize_markers();
	doc->toc_items = build_toc_from_headings(doc->buffer);
	return doc;
}

void docx_parser::traverse(pugi::xml_node node, wxString& text, std::vector<heading_info>& headings, document* doc, const std::map<std::string, std::string>& rels) const {
	if (node == nullptr) {
		return;
	}
	if (node.type() == pugi::node_element) {
		auto element = node;
		const std::string local_name = get_local_name(element.name());
		const std::string id_attr = element.attribute("w:id").as_string();
		if (!id_attr.empty()) {
			doc->id_positions[id_attr] = text.length();
		}
		if (local_name == "p") {
			process_paragraph(element, text, headings, doc, rels);
			return; // process_paragraph handles its children
		}
	}
	for (auto child : node.children()) {
		traverse(child, text, headings, doc, rels);
	}
}

void docx_parser::process_paragraph(pugi::xml_node element, wxString& text, std::vector<heading_info>& headings, document* doc, const std::map<std::string, std::string>& rels) {
	wxString paragraph_text;
	int heading_level = 0;
	bool is_paragraph_style_heading = false;
	const size_t paragraph_start_offset{text.length()};
	for (auto child : element.children()) {
		if (child.type() != pugi::node_element) {
			continue;
		}
		const std::string local_name = get_local_name(child.name());
		if (local_name == "pPr") {
			heading_level = get_paragraph_heading_level(child);
			if (heading_level > 0) {
				is_paragraph_style_heading = true;
			}
		} else if (local_name == "bookmarkStart") {
			const std::string name_attr = child.attribute("w:name").as_string();
			if (!name_attr.empty()) {
				doc->id_positions[name_attr] = paragraph_start_offset + paragraph_text.length();
			}
		} else if (local_name == "hyperlink") {
			process_hyperlink(child, paragraph_text, doc, rels, paragraph_start_offset);
		} else if (local_name == "r") {
			if (heading_level == 0) {
				if (const auto rpr_node = child.child("w:rPr")) {
					heading_level = get_run_heading_level(rpr_node);
				}
			}
			std::string instruction;
			if (auto itn = child.child("w:instrText")) {
				instruction = itn.text().as_string();
			}
			if (!instruction.empty() && instruction.find("HYPERLINK") != std::string::npos) {
				const std::string link_target = parse_hyperlink_instruction(instruction);
				if (!link_target.empty()) {
					std::string display_text_utf8;
					const size_t link_offset_in_paragraph = paragraph_text.length();
					auto field_node = child.next_sibling();
					bool in_display_text = false;
					while (field_node) {
						if (field_node.type() == pugi::node_element && get_local_name(field_node.name()) == "r") {
							auto field_run = field_node;
							std::string type;
							if (auto fld = field_run.child("w:fldChar")) {
								type = fld.attribute("w:fldCharType").as_string();
							}
							if (type == "separate") {
								in_display_text = true;
							} else if (type == "end") {
								break;
							} else if (in_display_text) {
								display_text_utf8 += get_run_text(field_run);
							}
						}
						field_node = field_node.next_sibling();
					}
					const wxString display_text_wx = wxString::FromUTF8(display_text_utf8);
					if (!display_text_wx.IsEmpty()) {
						paragraph_text += display_text_wx;
						doc->buffer.add_link(paragraph_start_offset + link_offset_in_paragraph, display_text_wx, wxString::FromUTF8(link_target));
					}
				}
			}
			paragraph_text += wxString::FromUTF8(get_run_text(child));
		}
	}
	if (!paragraph_text.IsEmpty()) {
		paragraph_text.Trim(true).Trim(false);
	}
	text += paragraph_text;
	text += "\n";
	if (heading_level > 0 && !paragraph_text.IsEmpty()) {
		heading_info h;
		h.offset = paragraph_start_offset;
		h.level = heading_level;
		wxString heading_text_for_toc;
		if (is_paragraph_style_heading) {
			heading_text_for_toc = paragraph_text;
		} else {
			for (auto child : element.children()) {
				const std::string local_name = get_local_name(child.name());
				if (local_name == "r") {
					int run_level = 0;
					if (const auto rpr_node = child.child("w:rPr")) {
						run_level = get_run_heading_level(rpr_node);
					}
					if (run_level == heading_level) {
						heading_text_for_toc += wxString::FromUTF8(get_run_text(child));
					}
				} else if (local_name == "hyperlink") {
					for (auto link_child : child.children()) {
						if (get_local_name(link_child.name()) == "r") {
							int run_level = 0;
							if (const auto rpr_node = link_child.child("w:rPr")) {
								run_level = get_run_heading_level(rpr_node);
							}
							if (run_level == heading_level) {
								heading_text_for_toc += wxString::FromUTF8(get_run_text(link_child));
							}
						}
					}
				}
			}
		}
		heading_text_for_toc.Trim(true).Trim(false);
		h.text = std::string(heading_text_for_toc.utf8_str());
		if (!h.text.empty()) {
			headings.push_back(h);
		}
	}
}

std::string docx_parser::parse_hyperlink_instruction(const std::string& instruction) {
	const size_t first_quote = instruction.find('"');
	const size_t last_quote = instruction.rfind('"');
	if (first_quote != std::string::npos && last_quote != std::string::npos && first_quote != last_quote) {
		std::string target = instruction.substr(first_quote + 1, last_quote - first_quote - 1);
		if (instruction.find("\\l") != std::string::npos) {
			return "#" + target;
		}
		return target;
	}
	return "";
}

void docx_parser::process_hyperlink(pugi::xml_node element, wxString& text, document* doc, const std::map<std::string, std::string>& rels, size_t paragraph_start_offset) {
	const std::string r_id = element.attribute("r:id").as_string();
	const std::string anchor = element.attribute("w:anchor").as_string();
	std::string link_target;
	if (!r_id.empty()) {
		auto it = rels.find(r_id);
		if (it != rels.end()) {
			link_target = it->second;
		}
	} else if (!anchor.empty()) {
		link_target = "#" + anchor;
	}
	if (link_target.empty()) { // If no target, just process the text
		for (auto child : element.children()) {
			if (child.type() == pugi::node_element && get_local_name(child.name()) == "r") {
				text += wxString::FromUTF8(get_run_text(child));
			}
		}
		return;
	}
	const size_t link_offset = text.length();
	std::string link_text_utf8;
	for (auto child : element.children()) {
		if (child.type() == pugi::node_element && get_local_name(child.name()) == "r") {
			link_text_utf8 += get_run_text(child);
		}
	}
	const wxString link_text_wx = wxString::FromUTF8(link_text_utf8);
	if (!link_text_wx.IsEmpty()) {
		text += link_text_wx;
		doc->buffer.add_link(paragraph_start_offset + link_offset, link_text_wx, wxString::FromUTF8(link_target));
	}
}

int docx_parser::get_paragraph_heading_level(pugi::xml_node pr_element) {
	constexpr int max_heading_level = 9;
	for (auto child : pr_element.children()) {
		if (child.type() == pugi::node_element) {
			const std::string local_name = get_local_name(child.name());
			if (local_name == "pStyle") {
				const std::string style = child.attribute("w:val").as_string();
				if (!style.empty()) {
					std::string style_lower = style;
					std::ranges::transform(style_lower, style_lower.begin(), ::tolower);
					if (style_lower.starts_with("heading")) {
						try {
							const size_t num_pos = style.find_first_of("0123456789");
							if (num_pos != std::string::npos) {
								const int level = std::stoi(style.substr(num_pos));
								if (level > 0 && level <= max_heading_level) {
									return level;
								}
							}
						} catch (...) {
						}
					}
				}
			} else if (local_name == "outlineLvl") {
				const std::string level_str = child.attribute("w:val").as_string();
				if (!level_str.empty()) {
					try {
						const int level = std::stoi(level_str) + 1;
						if (level > 0 && level <= max_heading_level) {
							return level;
						}
					} catch (...) {
					}
				}
			}
		}
	}
	return 0;
}

int docx_parser::get_run_heading_level(pugi::xml_node rpr_element) {
	constexpr int max_heading_level = 9;
	if (const auto rstyle_node = rpr_element.child("w:rStyle")) {
		const std::string style = rstyle_node.attribute("w:val").as_string();
		if (!style.empty()) {
			std::string style_lower = style;
			std::ranges::transform(style_lower, style_lower.begin(), ::tolower);
			if (style_lower.starts_with("heading") && style_lower.ends_with("char")) {
				try {
					const size_t num_pos = style.find_first_of("0123456789");
					if (num_pos != std::string::npos) {
						const int level = std::stoi(style.substr(num_pos));
						if (level > 0 && level <= max_heading_level) {
							return level;
						}
					}
				} catch (...) {
				}
			}
		}
	}
	return 0;
}

std::string docx_parser::get_run_text(pugi::xml_node run_element) {
	std::string run_text;
	for (auto child : run_element.children()) {
		if (child.type() == pugi::node_element) {
			const std::string local_name = get_local_name(child.name());
			if (local_name == "t") {
				run_text += child.text().as_string();
			} else if (local_name == "tab") {
				run_text += "\t";
			} else if (local_name == "br") {
				run_text += "\n";
			}
		}
	}
	return run_text;
}
