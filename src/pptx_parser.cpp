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
#include "utils.hpp"
#include <Poco/AutoPtr.h>
#include <Poco/DOM/DOMParser.h>
#include <Poco/DOM/Document.h>
#include <Poco/DOM/Element.h>
#include <Poco/DOM/Node.h>
#include <Poco/DOM/NodeList.h>
#include <Poco/DOM/Text.h>
#include <Poco/SAX/InputSource.h>
#include <algorithm>
#include <map>
#include <sstream>
#include <string>
#include <vector>
#include <wx/filename.h>
#include <wx/msgdlg.h>
#include <wx/wfstream.h>
#include <wx/zipstrm.h>

using namespace Poco;
using namespace Poco::XML;

const std::string DRAWINGML_NS = "http://schemas.openxmlformats.org/drawingml/2006/main";

std::unique_ptr<document> pptx_parser::load(const wxString& path) const {
	try {
		auto fp = std::make_unique<wxFileInputStream>(path);
		if (!fp->IsOk()) return nullptr;
		wxZipInputStream zip(*fp);
		if (!zip.IsOk()) return nullptr;
		std::map<std::string, std::string> slide_contents;
		std::unique_ptr<wxZipEntry> entry;
		while ((entry.reset(zip.GetNextEntry())), entry.get() != nullptr) {
			std::string name = entry->GetInternalName().ToStdString();
			if (name.find("ppt/slides/slide") == 0 && name.ends_with(".xml")) {
				if (name.find("slideLayout") == std::string::npos && name.find("slideMaster") == std::string::npos) {
					std::string content = read_zip_entry(zip);
					if (!content.empty()) slide_contents[name] = std::move(content);
				}
			}
		}
		if (slide_contents.empty()) return nullptr;
		std::vector<std::string> slide_files;
		for (const auto& [name, content] : slide_contents)
			slide_files.push_back(name);
		std::sort(slide_files.begin(), slide_files.end(), [](const std::string& a, const std::string& b) {
			auto extract_number = [](const std::string& s) {
				size_t pos = s.find_last_of('/');
				if (pos == std::string::npos) pos = 0;
				std::string num_str;
				for (char c : s.substr(pos))
					if (c >= '0' && c <= '9') num_str += c;
				return num_str.empty() ? 0 : std::stoi(num_str);
			};
			return extract_number(a) < extract_number(b);
		});
		auto doc = std::make_unique<document>();
		doc->title = wxFileName(path).GetName();
		doc->buffer.clear();
		wxString full_text;
		std::vector<size_t> slide_positions;
		for (const auto& slide_file : slide_files) {
			const std::string& slide_content = slide_contents[slide_file];
			std::istringstream content_stream(slide_content);
			InputSource source(content_stream);
			DOMParser parser;
			parser.setFeature(XMLReader::FEATURE_NAMESPACES, true);
			parser.setFeature(DOMParser::FEATURE_FILTER_WHITESPACE, false);
			AutoPtr<Document> slide_doc = parser.parse(&source);
			std::string slide_text;
			extract_text_from_node(slide_doc->documentElement(), slide_text);
			if (!slide_text.empty()) {
				wxString slide_wx = wxString::FromUTF8(slide_text);
				slide_wx.Trim(true).Trim(false);
				if (!slide_wx.IsEmpty()) {
					slide_positions.push_back(full_text.length());
					full_text += slide_wx;
					full_text += "\n";
				}
			}
		}
		doc->buffer.set_content(full_text);
		for (size_t i = 0; i < slide_positions.size(); ++i)
			doc->buffer.add_marker(slide_positions[i], marker_type::page_break, wxString::Format("Slide %zu", i + 1));
		return doc;
	} catch (const Poco::Exception& e) {
		wxMessageBox("XML parsing error: " + wxString(e.displayText()), "Error", wxICON_ERROR);
		return nullptr;
	} catch (...) {
		wxMessageBox("Unknown error while parsing PPTX file", "Error", wxICON_ERROR);
		return nullptr;
	}
}

void pptx_parser::extract_text_from_node(Node* node, std::string& text) const {
	if (!node) return;
	if (node->nodeType() == Node::ELEMENT_NODE) {
		auto* element = static_cast<Element*>(node);
		if (element->localName() == "t") {
			Node* text_node = element->firstChild();
			if (text_node && text_node->nodeType() == Node::TEXT_NODE) text += text_node->getNodeValue();
		} else if (element->localName() == "br")
			text += "\n";
		else if (element->localName() == "p") {
			Node* child = node->firstChild();
			while (child) {
				extract_text_from_node(child, text);
				child = child->nextSibling();
			}
			if (!text.empty() && text.back() != '\n') text += "\n";
			return; // Don't process children again.
		}
	}
	Node* child = node->firstChild();
	while (child) {
		extract_text_from_node(child, text);
		child = child->nextSibling();
	}
}
