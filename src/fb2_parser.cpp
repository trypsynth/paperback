/* fb2_parser.cpp - fb2 parser implementation.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#include "fb2_parser.hpp"
#include "utils.hpp"
#include "xml_to_text.hpp"
#include <pugixml.hpp>
#include <sstream>
#include <wx/filename.h>
#include <wx/log.h>
#include <wx/translation.h>
#include <wx/wfstream.h>

inline const char* FB2_NS = "http://www.gribuser.ru/xml/fictionbook/2.0";

std::unique_ptr<document> fb2_parser::load(const parser_context& ctx) const {
	wxFileInputStream input(ctx.file_path);
	if (!input.IsOk()) {
		throw parser_exception(_("Failed to open FB2 file"), ctx.file_path);
	}
	const size_t size = input.GetSize();
	std::string xml_content(size, 0);
	input.Read(&xml_content[0], size);
	const std::string closing_tag = "</FictionBook>";
	const size_t closing_tag_pos = xml_content.rfind(closing_tag);
	if (closing_tag_pos != std::string::npos) {
		xml_content.resize(closing_tag_pos + closing_tag.length());
	}
	// If the tag isn't found, we'll try to parse the whole file, which may fail but is the best we can do.
	if (xml_content.empty()) {
		throw parser_exception(_("FB2 file is empty or could not be read"), ctx.file_path);
	}
	try {
		pugi::xml_document d;
		if (d.load_buffer(xml_content.data(), xml_content.size())) {
			for (auto n : d.select_nodes("//*[local-name()='binary']")) {
				n.node().parent().remove_child(n.node());
			}
			std::ostringstream oss;
			d.save(oss);
			xml_content = oss.str();
		}
	} catch (...) {}
	xml_to_text converter;
	if (!converter.convert(xml_content)) {
		throw parser_exception(_("Failed to convert FB2 XML to text"), ctx.file_path);
	}
	auto doc = std::make_unique<document>();
	doc->buffer.set_content(wxString::FromUTF8(converter.get_text()));
	try {
		pugi::xml_document d;
		if (d.load_buffer(xml_content.data(), xml_content.size())) {
			auto title = d.select_node("/*[local-name()='FictionBook']/*[local-name()='description']/*[local-name()='title-info']/*[local-name()='book-title']");
			if (title) {
				doc->title = wxString::FromUTF8(title.node().text().as_string());
			}
			auto first = d.select_node("/*[local-name()='FictionBook']/*[local-name()='description']/*[local-name()='title-info']/*[local-name()='author']/*[local-name()='first-name']");
			auto last = d.select_node("/*[local-name()='FictionBook']/*[local-name()='description']/*[local-name()='title-info']/*[local-name()='author']/*[local-name()='last-name']");
			wxString author;
			if (first) {
				author += wxString::FromUTF8(first.node().text().as_string());
			}
			if (last) {
				if (!author.IsEmpty()) {
					author += " ";
				}
				author += wxString::FromUTF8(last.node().text().as_string());
			}
			if (!author.IsEmpty()) {
				doc->author = author;
			}
		}
	} catch (...) {
		// Ignore XML parsing errors, we still have the text
	}
	for (const auto& heading : converter.get_headings()) {
		doc->buffer.add_heading(heading.level, wxString::FromUTF8(heading.text));
	}
	for (const auto& offset : converter.get_section_offsets()) {
		doc->buffer.add_marker(offset, marker_type::section_break);
	}
	return doc;
}

std::string fb2_parser::get_element_text(pugi::xml_node element) {
	if (element == nullptr) {
		return {};
	}
	std::string text;
	for (auto child : element.children()) {
		if (child.type() == pugi::node_pcdata || child.type() == pugi::node_cdata) {
			text += child.value();
		} else if (child.type() == pugi::node_element) {
			text += get_element_text(child);
		}
	}
	return text;
}
