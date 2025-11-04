/* odp_parser.cpp - parser for odp documents.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#include "odp_parser.hpp"
#include "document.hpp"
#include "document_buffer.hpp"
#include "utils.hpp"
#include <cstddef>
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

inline const char* DRAW_NS = "urn:oasis:names:tc:opendocument:xmlns:drawing:1.0";
inline const char* TEXT_NS = "urn:oasis:names:tc:opendocument:xmlns:text:1.0";
inline const char* XLINK_NS = "http://www.w3.org/1999/xlink";

static std::string get_local_name(const char* qname) {
	if (!qname) {
		return {};
	}
	std::string s(qname);
	size_t pos = s.find(':');
	return pos == std::string::npos ? s : s.substr(pos + 1);
}

std::unique_ptr<document> odp_parser::load(const wxString& file_path) const {
	wxFileInputStream file_stream(file_path);
	if (!file_stream.IsOk()) {
		throw parser_exception(_("Failed to open ODP file"), file_path);
	}
	wxZipInputStream zip_stream(file_stream);
	std::unique_ptr<wxZipEntry> entry;
	std::string content;
	while (entry.reset(zip_stream.GetNextEntry()), entry != nullptr) {
		if (entry->GetName() == "content.xml") {
			content = read_zip_entry(zip_stream);
			break;
		}
	}
	if (content.empty()) {
		throw parser_exception(_("ODP file does not contain content.xml or it is empty"), file_path);
	}
	try {
		pugi::xml_document p_doc;
		if (!p_doc.load_buffer(content.data(), content.size(), pugi::parse_default | pugi::parse_ws_pcdata)) {
			throw parser_exception("Invalid ODP content", file_path);
		}
		auto doc = std::make_unique<document>();
		doc->title = wxFileName(file_path).GetName();
		wxString full_text;
		std::vector<size_t> slide_positions;
		auto root = p_doc.document_element();
		if (!root) {
			throw parser_exception(_("ODP file does not contain any pages"), file_path);
		}
		for (auto page_node : root.select_nodes("//*[local-name()='page']")) {
			wxString slide_text;
			traverse(page_node.node(), slide_text, doc.get(), &full_text);
			if (!slide_text.IsEmpty()) {
				slide_text.Trim(true).Trim(false);
				if (!slide_text.IsEmpty()) {
					slide_positions.push_back(full_text.length());
					full_text += slide_text;
					full_text += "\n";
				}
			}
		}
		doc->buffer.set_content(full_text);
		for (size_t i = 0; i < slide_positions.size(); ++i) {
			doc->buffer.add_marker(slide_positions[i], marker_type::page_break, wxString::Format("Slide %zu", i + 1));
		}
		doc->buffer.finalize_markers();
		return doc;
	} catch (...) {
		throw;
	}
}

void odp_parser::traverse(pugi::xml_node node, wxString& text, document* doc, wxString* full_text) const {
	if (node == nullptr) {
		return;
	}
	if (node.type() == pugi::node_element) {
		auto element = node;
		const std::string local_name = get_local_name(element.name());
		if (local_name == "a") {
			const std::string href = element.attribute("xlink:href").as_string();
			if (!href.empty()) {
				const size_t link_start = full_text->length() + text.length();
				wxString link_text;
				traverse_children(element, link_text, doc, full_text);
				if (!link_text.IsEmpty()) {
					doc->buffer.add_link(link_start, link_text, wxString::FromUTF8(href));
					text += link_text;
				}
			}
		} else if (local_name == "p" || local_name == "span") {
			traverse_children(element, text, doc, full_text);
			if (local_name == "p" && !text.EndsWith("\n")) {
				text += "\n";
			}
		} else {
			traverse_children(element, text, doc, full_text);
		}
	} else if (node.type() == pugi::node_pcdata || node.type() == pugi::node_cdata) {
		text += wxString::FromUTF8(node.value());
	}
}

void odp_parser::traverse_children(pugi::xml_node node, wxString& text, document* doc, wxString* full_text) const {
	for (auto child : node.children()) {
		traverse(child, text, doc, full_text);
	}
}
