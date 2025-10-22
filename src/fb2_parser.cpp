/* fb2_parser.cpp - fb2 parser implementation.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#include "fb2_parser.hpp"
#include "xml_to_text.hpp"
#include "utils.hpp"
#include <wx/filename.h>
#include <wx/wfstream.h>
#include <Poco/DOM/DOMParser.h>
#include <Poco/DOM/Element.h>
#include <Poco/DOM/Node.h>
#include <Poco/DOM/Text.h>
#include <Poco/SAX/InputSource.h>
#include <Poco/Exception.h>
#include <Poco/AutoPtr.h>
#include <Poco/XML/XMLString.h>
#include <Poco/XML/XMLWriter.h>
#include <Poco/DOM/DOMWriter.h>
#include <Poco/DOM/NodeList.h>
#include <sstream>
#include <wx/log.h>

inline const Poco::XML::XMLString FB2_NS = "http://www.gribuser.ru/xml/fictionbook/2.0";

static std::string get_element_text(Poco::XML::Element* element) {
	if (element == nullptr) {
		return {};
	}
	std::string text;
	auto* child = element->firstChild();
	while (child != nullptr) {
		if (child->nodeType() == Poco::XML::Node::TEXT_NODE) {
			auto* text_node = dynamic_cast<Poco::XML::Text*>(child);
			text += text_node->data();
		} else if (child->nodeType() == Poco::XML::Node::ELEMENT_NODE) {
			text += get_element_text(dynamic_cast<Poco::XML::Element*>(child));
		}
		child = child->nextSibling();
	}
	return text;
}

std::unique_ptr<document> fb2_parser::load(const wxString &path) const {
	wxFileInputStream input(path);
	if (!input.IsOk()) {
		return nullptr;
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

		return nullptr;
	}

	// Remove <binary> tags and their content
	try {
		Poco::XML::DOMParser dom_parser;
		std::istringstream iss_dom(xml_content);
		Poco::XML::InputSource source_dom(iss_dom);
		Poco::AutoPtr<Poco::XML::Document> poco_dom_doc = dom_parser.parse(&source_dom);

		Poco::XML::NodeList* binary_nodes = poco_dom_doc->getElementsByTagNameNS(FB2_NS, "binary");
		for (int i = binary_nodes->length() - 1; i >= 0; --i) {
			Poco::XML::Node* node_to_remove = binary_nodes->item(i);
			node_to_remove->parentNode()->removeChild(node_to_remove);
		}

		std::ostringstream oss_cleaned_xml;
		Poco::XML::DOMWriter writer;
		writer.writeNode(oss_cleaned_xml, poco_dom_doc);
		xml_content = oss_cleaned_xml.str();

	} catch (const Poco::Exception& exc) {

		// This might lead to the "random garbage" but won't crash the app.
		wxLogError("Failed to remove binary tags from FB2: %s", exc.displayText().c_str());
	}

	xml_to_text converter;
	if (!converter.convert(xml_content)) {
		return nullptr;
	}

	auto doc = std::make_unique<document>();
	doc->buffer.set_content(wxString::FromUTF8(converter.get_text()));

	try {
		Poco::XML::DOMParser parser;
		std::istringstream iss(xml_content);
		Poco::XML::InputSource source(iss);
		Poco::AutoPtr<Poco::XML::Document> poco_doc = parser.parse(&source);
		Poco::XML::Element* root = poco_doc->documentElement();
		if (root) {
			Poco::XML::Element* description = root->getChildElementNS(FB2_NS, "description");
			if (description) {
				Poco::XML::Element* title_info = description->getChildElementNS(FB2_NS, "title-info");
				if (title_info) {
					Poco::XML::Element* title_node = title_info->getChildElementNS(FB2_NS, "book-title");
					if (title_node) {
						doc->title = wxString::FromUTF8(get_element_text(title_node));
					}
					Poco::XML::Element* author_node = title_info->getChildElementNS(FB2_NS, "author");
					if(author_node) {
						Poco::XML::Element* first_name_node = author_node->getChildElementNS(FB2_NS, "first-name");
						if (first_name_node) {
							doc->author = wxString::FromUTF8(get_element_text(first_name_node));
						}
						Poco::XML::Element* last_name_node = author_node->getChildElementNS(FB2_NS, "last-name");
						if (last_name_node) {
							if (!doc->author.IsEmpty()) {
								doc->author += " ";
							}
							doc->author += wxString::FromUTF8(get_element_text(last_name_node));
						}
					}
				}
			}
		}
	} catch (const Poco::Exception&) {
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
