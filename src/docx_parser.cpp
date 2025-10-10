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
#include "utils.hpp"
#include <Poco/AutoPtr.h>
#include <Poco/DOM/DOMParser.h>
#include <Poco/DOM/Document.h>
#include <Poco/DOM/Node.h>
#include <Poco/DOM/NodeList.h>
#include <Poco/DOM/Text.h>
#include <Poco/SAX/InputSource.h>
#include <algorithm>
#include <sstream>
#include <vector>
#include <wx/filename.h>
#include <wx/msgdlg.h>
#include <wx/wfstream.h>
#include <wx/zipstrm.h>

using namespace Poco;
using namespace Poco::XML;

const std::string WORDML_NS = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";

std::unique_ptr<document> docx_parser::load(const wxString& path) const {
	try {
		auto fp = std::make_unique<wxFileInputStream>(path);
		if (!fp->IsOk()) return nullptr;
		wxZipInputStream zip(*fp);
		if (!zip.IsOk()) return nullptr;
		std::unique_ptr<wxZipEntry> entry;
		bool found = false;
		while ((entry.reset(zip.GetNextEntry())), entry.get() != nullptr) {
			if (entry->GetInternalName() == "word/document.xml") {
				found = true;
				break;
			}
		}
		if (!found) return nullptr;
		std::string content = read_zip_entry(zip);
		if (content.empty()) return nullptr;
		std::istringstream content_stream(content);
		InputSource source(content_stream);
		DOMParser parser;
		parser.setFeature(XMLReader::FEATURE_NAMESPACES, true);
		parser.setFeature(DOMParser::FEATURE_FILTER_WHITESPACE, false);
		AutoPtr<Document> pDoc = parser.parse(&source);
		wxString text;
		std::vector<heading_info> headings;
		traverse(pDoc->documentElement(), text, headings);
		auto doc = std::make_unique<document>();
		doc->title = wxFileName(path).GetName();
		doc->buffer.clear();
		doc->buffer.set_content(text);
		for (const auto& heading : headings) {
			marker_type type = static_cast<marker_type>(static_cast<int>(marker_type::heading_1) + heading.level - 1);
			doc->buffer.add_marker(heading.offset, type, wxString::FromUTF8(heading.text), wxString(), heading.level);
		}
		doc->toc_items = build_toc_from_headings(doc->buffer);
		return doc;
	} catch (const Poco::Exception& e) {
		wxMessageBox("XML parsing error: " + wxString(e.displayText()), "Error", wxICON_ERROR);
		return nullptr;
	} catch (...) {
		wxMessageBox("Unknown error while parsing DOCX file", "Error", wxICON_ERROR);
		return nullptr;
	}
}

void docx_parser::traverse(Node* node, wxString& text, std::vector<heading_info>& headings) const {
	if (!node) return;
	if (node->nodeType() == Node::ELEMENT_NODE) {
		auto* element = static_cast<Element*>(node);
		if (element->localName() == "p") {
			process_paragraph(element, text, headings);
			return;
		}
	}
	Node* child = node->firstChild();
	while (child) {
		traverse(child, text, headings);
		child = child->nextSibling();
	}
}

void docx_parser::process_paragraph(Element* element, wxString& text, std::vector<heading_info>& headings) const {
	std::string paragraph_text_utf8;
	int heading_level = 0;
	Node* child = element->firstChild();
	while (child) {
		if (child->nodeType() == Node::ELEMENT_NODE) {
			auto* child_element = static_cast<Element*>(child);
			std::string localName = child_element->localName();
			if (localName == "pPr")
				heading_level = get_heading_level(child_element);
			else if (localName == "r")
				paragraph_text_utf8 += get_run_text(child_element);
		}
		child = child->nextSibling();
	}
	wxString paragraph_wx = wxString::FromUTF8(paragraph_text_utf8);
	paragraph_wx.Trim(true).Trim(false);
	if (!paragraph_wx.IsEmpty()) {
		size_t offset = text.length();
		text += paragraph_wx;
		text += "\n";
		if (heading_level > 0) {
			heading_info h;
			h.offset = offset;
			h.level = heading_level;
			h.text = std::string(paragraph_wx.utf8_str());
			if (!h.text.empty()) headings.push_back(h);
		}
	}
}

int docx_parser::get_heading_level(Element* pr_element) const {
	Node* child = pr_element->firstChild();
	while (child) {
		if (child->nodeType() == Node::ELEMENT_NODE) {
			auto* element = static_cast<Element*>(child);
			std::string localName = element->localName();
			if (localName == "pStyle") {
				std::string style = element->getAttributeNS(WORDML_NS, "val");
				if (!style.empty()) {
					std::string style_lower = style;
					std::transform(style_lower.begin(), style_lower.end(), style_lower.begin(), ::tolower);
					if (style_lower.rfind("heading", 0) == 0) {
						try {
							size_t num_pos = style.find_first_of("0123456789");
							if (num_pos != std::string::npos) {
								int level = std::stoi(style.substr(num_pos));
								if (level > 0 && level <= 9) return level;
							}
						} catch (...) {
						}
					}
				}
			} else if (localName == "outlineLvl") {
				std::string level_str = element->getAttributeNS(WORDML_NS, "val");
				if (!level_str.empty()) {
					try {
						int level = std::stoi(level_str) + 1;
						if (level > 0 && level <= 9) return level;
					} catch (...) {
					}
				}
			}
		}
		child = child->nextSibling();
	}
	return 0;
}

std::string docx_parser::get_run_text(Element* run_element) const {
	std::string run_text;
	Node* child = run_element->firstChild();
	while (child) {
		if (child->nodeType() == Node::ELEMENT_NODE) {
			auto* element = static_cast<Element*>(child);
			if (element->localName() == "t") {
				if (element->firstChild() && element->firstChild()->nodeType() == Node::TEXT_NODE)
					run_text += element->firstChild()->getNodeValue();
			} else if (element->localName() == "tab")
				run_text += "\t";
			else if (element->localName() == "br")
				run_text += "\n";
		}
		child = child->nextSibling();
	}
	return run_text;
}
