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
#include "utils.hpp"
#include <Poco/AutoPtr.h>
#include <Poco/DOM/DOMParser.h>
#include <Poco/DOM/Document.h>
#include <Poco/DOM/Node.h>
#include <Poco/DOM/NodeList.h>
#include <Poco/DOM/Text.h>
#include <Poco/SAX/InputSource.h>
#include <sstream>
#include <wx/filename.h>
#include <wx/wfstream.h>
#include <wx/zipstrm.h>

using namespace Poco;
using namespace Poco::XML;

std::unique_ptr<document> odt_parser::load(const wxString& file_path) const {
	wxFileInputStream file_stream(file_path);
	if (!file_stream.IsOk()) {
		return nullptr;
	}
	wxZipInputStream zip_stream(file_stream);
	std::unique_ptr<wxZipEntry> entry;
	std::string content;
	while (entry.reset(zip_stream.GetNextEntry()), entry.get() != nullptr) {
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
		AutoPtr<Poco::XML::Document> pDoc = parser.parse(&source);
		auto doc = std::make_unique<document>();
		doc->title = wxFileName(file_path).GetName();
		wxString text;
		traverse(pDoc->documentElement(), text, doc.get());
		doc->buffer.set_content(text);
		doc->toc_items = build_toc_from_headings(doc->buffer);
		return doc;
	} catch (Poco::Exception&) {
		return nullptr;
	}
}

void odt_parser::traverse(Poco::XML::Node* node, wxString& text, document* doc) const {
	if (!node) {
		return;
	}
	if (node->nodeType() == Poco::XML::Node::ELEMENT_NODE) {
		auto* element = static_cast<Poco::XML::Element*>(node);
		std::string localName = element->localName();
		if (localName == "h") {
			int level = 0;
			if (element->hasAttributeNS("urn:oasis:names:tc:opendocument:xmlns:text:1.0", "outline-level")) {
				level = std::stoi(element->getAttributeNS("urn:oasis:names:tc:opendocument:xmlns:text:1.0", "outline-level"));
			}
			size_t heading_offset = text.length();
			wxString heading_text;
			traverse_children(element, heading_text, doc);
			text += heading_text + "\n";
			if (level > 0) {
				marker_type type = static_cast<marker_type>(static_cast<int>(marker_type::heading_1) + level - 1);
				doc->buffer.add_marker(heading_offset, type, heading_text, wxString(), level);
			}
		} else if (localName == "p") {
			traverse_children(element, text, doc);
			text += "\n";
		} else if (localName == "a") {
			if (element->hasAttributeNS("http://www.w3.org/1999/xlink", "href")) {
				wxString href = wxString::FromUTF8(element->getAttributeNS("http://www.w3.org/1999/xlink", "href"));
				size_t link_offset = text.length();
				wxString link_text;
				traverse_children(element, link_text, doc);
				text += link_text;
				doc->buffer.add_link(link_offset, link_text, href);
			}
		} else {
			traverse_children(element, text, doc);
		}
	} else if (node->nodeType() == Poco::XML::Node::TEXT_NODE) {
		auto* textNode = static_cast<Poco::XML::Text*>(node);
		text += wxString::FromUTF8(textNode->data());
	}
}

void odt_parser::traverse_children(Poco::XML::Node* node, wxString& text, document* doc) const {
	Poco::XML::Node* child = node->firstChild();
	while (child) {
		traverse(child, text, doc);
		child = child->nextSibling();
	}
}
