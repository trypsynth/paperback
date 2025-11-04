/* pptx_parser.cpp - parser for pptx documents.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#include "pptx_parser.hpp"
#include "document.hpp"
#include "document_buffer.hpp"
#include "utils.hpp"
#include <algorithm>
#include <cctype>
#include <cstddef>
#include <functional>
#include <map>
#include <memory>
#include <numeric>
#include <pugixml.hpp>
#include <ranges>
#include <sstream>
#include <string>
#include <utility>
#include <vector>
#include <wx/filename.h>
#include <wx/string.h>
#include <wx/translation.h>
#include <wx/wfstream.h>
#include <wx/zipstrm.h>

inline const char* DRAWINGML_NS = "http://schemas.openxmlformats.org/drawingml/2006/main";
inline const char* REL_NS = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";

static std::string get_local_name(const char* qname) {
	if (qname == nullptr) {
		return {};
	}
	std::string s(qname);
	size_t pos = s.find(':');
	return pos == std::string::npos ? s : s.substr(pos + 1);
}

std::unique_ptr<document> pptx_parser::load(const wxString& path) const {
	try {
		auto fp = std::make_unique<wxFileInputStream>(path);
		if (!fp->IsOk()) {
			return nullptr;
		}
		wxZipInputStream zip(*fp);
		if (!zip.IsOk()) {
			return nullptr;
		}
		std::map<std::string, std::string> slide_contents;
		std::map<std::string, std::string> slide_rels;
		std::unique_ptr<wxZipEntry> entry;
		while ((entry.reset(zip.GetNextEntry())), entry != nullptr) {
			const std::string name = entry->GetInternalName().ToStdString();
			if (name.starts_with("ppt/slides/slide") && name.ends_with(".xml")) {
				if (name.find("slideLayout") == std::string::npos && name.find("slideMaster") == std::string::npos) {
					const std::string content = read_zip_entry(zip);
					if (!content.empty()) {
						slide_contents[name] = content;
					}
				}
			} else if (name.starts_with("ppt/slides/_rels/slide") && name.ends_with(".xml.rels")) {
				std::string content = read_zip_entry(zip);
				if (!content.empty()) {
					slide_rels[name] = std::move(content);
				}
			}
		}
		if (slide_contents.empty()) {
			return nullptr;
		}
		std::vector<std::string> slide_files;
		slide_files.reserve(slide_contents.size());
		for (const auto& [name, content] : slide_contents) {
			slide_files.push_back(name);
		}
		auto extract_number_view = [](const std::string& s) -> int {
			constexpr int decimal_base = 10;
			auto start_it = s.rfind('/') == std::string::npos ? s.begin() : s.begin() + static_cast<std::string::difference_type>(s.rfind('/'));
			auto digits_view = std::ranges::subrange(start_it, s.end()) | std::views::filter([](char c) { return std::isdigit(c); });
			return std::accumulate(digits_view.begin(), digits_view.end(), 0, [](int acc, char c) { return (acc * decimal_base) + (c - '0'); });
		};
		std::ranges::sort(slide_files, [&](const std::string& a, const std::string& b) {
			return extract_number_view(a) < extract_number_view(b);
		});
		auto doc = std::make_unique<document>();
		doc->title = wxFileName(path).GetName();
		doc->buffer.clear();
		wxString full_text;
		std::vector<size_t> slide_positions;
		std::vector<wxString> slide_titles;
		for (size_t i = 0; i < slide_files.size(); ++i) {
			const auto& slide_file = slide_files[i];
			const std::string& slide_content = slide_contents[slide_file];
			std::map<std::string, std::string> rels;
			const std::string slide_base = slide_file.substr(slide_file.find_last_of('/') + 1);
			const std::string rels_file = "ppt/slides/_rels/" + slide_base + ".rels";
			auto rels_it = slide_rels.find(rels_file);
			if (rels_it != slide_rels.end()) {
				try {
					pugi::xml_document rels_doc;
					if (rels_doc.load_buffer(rels_it->second.data(), rels_it->second.size())) {
						for (auto rel : rels_doc.child("Relationships").children("Relationship")) {
							std::string id = rel.attribute("Id").as_string();
							std::string target = rel.attribute("Target").as_string();
							std::string type = rel.attribute("Type").as_string();
							if (type == "http://schemas.openxmlformats.org/officeDocument/2006/relationships/hyperlink") {
								rels[id] = target;
							}
						}
					}
				} catch (...) {
					throw parser_exception(_("Parsing of links in the document failed."), error_severity::warning);
				}
			}
			pugi::xml_document slide_doc;
			if (!slide_doc.load_buffer(slide_content.data(), slide_content.size(), pugi::parse_default | pugi::parse_ws_pcdata)) {
				continue;
			}
			wxString slide_title = extract_slide_title(slide_doc);
			std::string slide_text;
			extract_text_from_node(slide_doc.document_element(), slide_text, full_text, doc.get(), rels);
			if (!slide_text.empty()) {
				wxString slide_wx = wxString::FromUTF8(slide_text);
				slide_wx.Trim(true).Trim(false);
				if (!slide_wx.IsEmpty()) {
					slide_positions.push_back(full_text.length());
					slide_titles.push_back(slide_title);
					full_text += slide_wx;
					full_text += "\n";
					if (i + 1 < slide_files.size()) {
						// Ensure we don't get double blank lines at the end of the document.
						full_text += "\n";
					}
				}
			}
		}
		doc->buffer.set_content(full_text);
		for (size_t i = 0; i < slide_positions.size(); ++i) {
			doc->buffer.add_marker(slide_positions[i], marker_type::page_break, wxString::Format("Slide %zu", i + 1));
		}
		doc->buffer.finalize_markers();
		for (size_t i = 0; i < slide_positions.size(); ++i) {
			auto toc_entry = std::make_unique<toc_item>();
			if (slide_titles[i].IsEmpty()) {
				toc_entry->name = wxString::Format("Slide %zu", i + 1);
			} else {
				toc_entry->name = slide_titles[i];
			}
			toc_entry->offset = static_cast<int>(slide_positions[i]);
			doc->toc_items.push_back(std::move(toc_entry));
		}
		return doc;
	} catch (const std::exception& e) {
		throw parser_exception(wxString::Format(_("Error parsing PPTX file: %s"), wxString::FromUTF8(e.what())), path);
	} catch (...) {
		throw parser_exception(_("Unknown error while parsing PPTX file"), path);
	}
}

void pptx_parser::extract_text_from_node(pugi::xml_node node, std::string& text, wxString& full_text, document* doc, const std::map<std::string, std::string>& rels) const {
	if (node == nullptr) {
		return;
	}
	if (node.type() == pugi::node_element) {
		auto element = node;
		if (get_local_name(element.name()) == "t") {
			text += element.text().as_string();
		} else if (get_local_name(element.name()) == "br") {
			text += "\n";
		} else if (get_local_name(element.name()) == "p") {
			for (auto child : node.children()) {
				extract_text_from_node(child, text, full_text, doc, rels);
			}
			if (!text.empty() && text.back() != '\n') {
				text += "\n";
			}
			return; // Don't process children again.
		} else if (get_local_name(element.name()) == "hlinkClick") {
			const std::string r_id = element.attribute("r:id").as_string();
			std::string link_target;
			if (!r_id.empty()) {
				auto it = rels.find(r_id);
				if (it != rels.end()) {
					link_target = it->second;
				}
			}
			// In PPTX, the link wraps the text runs, so we need to find the text within this subtree.
			std::string link_text_utf8;
			std::function<void(pugi::xml_node)> extract_link_text = [&](pugi::xml_node n) {
				if (!n) {
					return;
				}
				if (n.type() == pugi::node_element) {
					if (get_local_name(n.name()) == "t") {
						link_text_utf8 += n.text().as_string();
					}
				}
				for (auto c : n.children()) {
					extract_link_text(c);
				}
			};
			auto parent = node.parent();
			if (parent) {
				extract_link_text(parent);
			}
			if (!link_text_utf8.empty() && !link_target.empty()) {
				const size_t link_start = full_text.length() + text.length();
				text += link_text_utf8;
				const wxString link_text_wx = wxString::FromUTF8(link_text_utf8);
				doc->buffer.add_link(link_start, link_text_wx, wxString::FromUTF8(link_target));
			}
			return; // Don't process children again since we already extracted text.
		}
	}
	for (auto child : node.children()) {
		extract_text_from_node(child, text, full_text, doc, rels);
	}
}

wxString pptx_parser::extract_slide_title(pugi::xml_document& slide_doc) const {
	if (slide_doc.empty()) {
		return wxEmptyString;
	}
	std::function<std::string(pugi::xml_node)> extract_text = [&](pugi::xml_node node) -> std::string {
		std::string result;
		if (!node) {
			return result;
		}
		if (node.type() == pugi::node_element) {
			if (get_local_name(node.name()) == "t") {
				result += node.text().as_string();
			}
		}
		for (auto child : node.children()) {
			result += extract_text(child);
		}
		return result;
	};
	auto shapes = slide_doc.select_nodes("//*[local-name()='sp']");
	if (shapes.empty()) {
		return wxEmptyString;
	}
	for (auto sn : shapes) {
		auto shape = sn.node();
		bool is_title = false;
		std::function<void(pugi::xml_node)> find_title_placeholder = [&](pugi::xml_node node) {
			if (!node || is_title) {
				return;
			}
			if (node.type() == pugi::node_element) {
				if (get_local_name(node.name()) == "ph") {
					std::string type = node.attribute("type").as_string();
					if (type == "title" || type == "ctrTitle") {
						is_title = true;
						return;
					}
				}
			}
			for (auto child : node.children()) {
				find_title_placeholder(child);
			}
		};
		find_title_placeholder(shape);
		if (is_title) {
			std::string title_text = extract_text(shape);
			if (!title_text.empty()) {
				wxString title = wxString::FromUTF8(title_text);
				title.Trim(true).Trim(false);
				if (!title.IsEmpty()) {
					return title;
				}
			}
		}
	}
	return wxEmptyString;
}
