/* odt_parser.cpp - odt parser implementation.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#include "odt_parser.hpp"
#include "document.hpp"
#include "document_buffer.hpp"
#include "utils.hpp"
#include <cstddef>
#include <memory>
#include <pugixml.hpp>
#include <sstream>
#include <string>
#include <wx/filename.h>
#include <wx/string.h>
#include <wx/translation.h>
#include <wx/wfstream.h>
#include <wx/zipstrm.h>

static std::string get_local_name(const char* qname) {
	if (!qname) {
		return {};
	}
	std::string s(qname);
	size_t pos = s.find(':');
	return pos == std::string::npos ? s : s.substr(pos + 1);
}

std::unique_ptr<document> odt_parser::load(const parser_context& ctx) const {
	wxFileInputStream file_stream(ctx.file_path);
	if (!file_stream.IsOk()) {
		throw parser_exception(_("Failed to open ODT file"), ctx.file_path);
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
		throw parser_exception(_("ODT file does not contain content.xml or it is empty"), ctx.file_path);
	}
	try {
		pugi::xml_document p_doc;
		if (!p_doc.load_buffer(content.data(), content.size(), pugi::parse_default | pugi::parse_ws_pcdata)) {
			throw parser_exception("Invalid ODT content", ctx.file_path);
		}
		auto doc = std::make_unique<document>();
		doc->title = wxFileName(ctx.file_path).GetName();
		wxString text;
		traverse(p_doc.document_element(), text, doc.get());
		doc->buffer.set_content(text);
		doc->buffer.finalize_markers();
		doc->toc_items = build_toc_from_headings(doc->buffer);
		return doc;
	} catch (...) {
		throw;
	}
}

void odt_parser::traverse(pugi::xml_node node, wxString& text, document* doc) const {
	if (node == nullptr) {
		return;
	}
	if (node.type() == pugi::node_element) {
		auto element = node;
		const std::string local_name = get_local_name(element.name());
		if (local_name == "h") {
			int level = 0;
			if (auto attr = element.attribute("text:outline-level")) {
				level = std::stoi(attr.as_string());
			}
			const size_t heading_offset = text.length();
			wxString heading_text;
			traverse_children(element, heading_text, doc);
			text += heading_text + "\n";
			if (level > 0) {
				const auto type = static_cast<marker_type>(static_cast<int>(marker_type::heading_1) + level - 1);
				doc->buffer.add_marker(heading_offset, type, heading_text, wxString(), level);
			}
		} else if (local_name == "p") {
			traverse_children(element, text, doc);
			text += "\n";
		} else if (local_name == "a") {
			std::string href = element.attribute("xlink:href").as_string();
			if (!href.empty()) {
				const wxString href_wx = wxString::FromUTF8(href);
				const size_t link_offset = text.length();
				wxString link_text;
				traverse_children(element, link_text, doc);
				text += link_text;
				doc->buffer.add_link(link_offset, link_text, href_wx);
			}
		} else {
			traverse_children(element, text, doc);
		}
	} else if (node.type() == pugi::node_pcdata || node.type() == pugi::node_cdata) {
		text += wxString::FromUTF8(node.value());
	}
}

void odt_parser::traverse_children(pugi::xml_node node, wxString& text, document* doc) const {
	for (auto child : node.children()) {
		traverse(child, text, doc);
	}
}
