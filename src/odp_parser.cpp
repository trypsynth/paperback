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
#include <Poco/AutoPtr.h>
#include <Poco/DOM/DOMParser.h>
#include <Poco/DOM/Document.h>
#include <Poco/DOM/Element.h>
#include <Poco/DOM/Node.h>
#include <Poco/DOM/NodeList.h>
#include <Poco/DOM/Text.h>
#include <Poco/Exception.h>
#include <Poco/SAX/InputSource.h>
#include <Poco/SAX/XMLReader.h>
#include <cstddef>
#include <memory>
#include <sstream>
#include <string>
#include <vector>
#include <wx/filename.h>
#include <wx/string.h>
#include <wx/wfstream.h>
#include <wx/zipstrm.h>

using namespace Poco;
using namespace Poco::XML;


inline const XMLString DRAW_NS = "urn:oasis:names:tc:opendocument:xmlns:drawing:1.0";
inline const XMLString TEXT_NS = "urn:oasis:names:tc:opendocument:xmlns:text:1.0";
inline const XMLString XLINK_NS = "http://www.w3.org/1999/xlink";

std::unique_ptr<document> odp_parser::load(const wxString& file_path) const {
	wxFileInputStream file_stream(file_path);
	if (!file_stream.IsOk()) {
		return nullptr;
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
		return nullptr;
	}
	try {
		std::istringstream content_stream(content);
		InputSource source(content_stream);
		DOMParser parser;
		parser.setFeature(XMLReader::FEATURE_NAMESPACES, true);
		AutoPtr<Poco::XML::Document> p_doc = parser.parse(&source);
		auto doc = std::make_unique<document>();
		doc->title = wxFileName(file_path).GetName();
		wxString full_text;
		std::vector<size_t> slide_positions;
		const NodeList* pages = p_doc->getElementsByTagNameNS(DRAW_NS, "page");
		if (pages == nullptr) {
			return nullptr;
		}
		for (unsigned long i = 0; i < pages->length(); ++i) {
			Node* page_node = pages->item(i);
			wxString slide_text;
			traverse(page_node, slide_text, doc.get(), &full_text);
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
		return doc;
	} catch (Poco::Exception&) {
		return nullptr;
	}
}

void odp_parser::traverse(Poco::XML::Node* node, wxString& text, document* doc, wxString* full_text) const {
	if (node == nullptr) {
		return;
	}
	if (node->nodeType() == Poco::XML::Node::ELEMENT_NODE) {
		auto* element = dynamic_cast<Poco::XML::Element*>(node);
		const std::string local_name = element->localName();
		if (local_name == "a" && element->namespaceURI() == TEXT_NS) {
			const std::string href = element->getAttributeNS(XLINK_NS, "href");
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
	} else if (node->nodeType() == Poco::XML::Node::TEXT_NODE) {
		auto* text_node = dynamic_cast<Poco::XML::Text*>(node);
		text += wxString::FromUTF8(text_node->data());
	}
}

void odp_parser::traverse_children(Poco::XML::Node* node, wxString& text, document* doc, wxString* full_text) const {
	Poco::XML::Node* child = node->firstChild();
	while (child != nullptr) {
		traverse(child, text, doc, full_text);
		child = child->nextSibling();
	}
}
