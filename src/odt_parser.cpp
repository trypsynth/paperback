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
#include <wx/filename.h>
#include <wx/string.h>
#include <wx/translation.h>
#include <wx/wfstream.h>
#include <wx/zipstrm.h>

using namespace Poco;
using namespace Poco::XML;

std::unique_ptr<document> odt_parser::load(const wxString& file_path) const {
	wxFileInputStream file_stream(file_path);
	if (!file_stream.IsOk()) {
		throw parser_exception(_("Failed to open ODT file"), file_path);
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
		throw parser_exception(_("ODT file does not contain content.xml or it is empty"), file_path);
	}
	try {
		std::istringstream content_stream(content);
		InputSource source(content_stream);
		DOMParser parser;
		parser.setFeature(XMLReader::FEATURE_NAMESPACES, true);
		AutoPtr<Poco::XML::Document> p_doc = parser.parse(&source);
		auto doc = std::make_unique<document>();
		doc->title = wxFileName(file_path).GetName();
		wxString text;
		traverse(p_doc->documentElement(), text, doc.get());
		doc->buffer.set_content(text);
		doc->buffer.finalize_markers();
		doc->toc_items = build_toc_from_headings(doc->buffer);
		return doc;
	} catch (const Poco::Exception& e) {
		throw parser_exception(wxString::Format(_("XML parsing error: %s"), wxString::FromUTF8(e.displayText())), file_path);
	}
}

void odt_parser::traverse_children(Poco::XML::Node* node, wxString& text, document* doc) const {
	Poco::XML::Node* child = node->firstChild();
	while (child != nullptr) {
		traverse(child, text, doc);
		child = child->nextSibling();
	}
}

void odt_parser::traverse(Poco::XML::Node* node, wxString& text, document* doc) const {
	if (node == nullptr) {
		return;
	}
	if (node->nodeType() == Poco::XML::Node::ELEMENT_NODE) {
		auto* element = dynamic_cast<Poco::XML::Element*>(node);
		const std::string local_name = element->localName();
		if (local_name == "h") {
			int level = 0;
			if (element->hasAttributeNS("urn:oasis:names:tc:opendocument:xmlns:text:1.0", "outline-level")) {
				level = std::stoi(element->getAttributeNS("urn:oasis:names:tc:opendocument:xmlns:text:1.0", "outline-level"));
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
		} else if (local_name == "table") {
			auto table_data = process_table(element);
			doc->buffer.add_table(text.length(), table_data.first, table_data.second);
			text += table_data.first + "\n";
		} else if (local_name == "a") {
			if (element->hasAttributeNS("http://www.w3.org/1999/xlink", "href")) {
				const wxString href = wxString::FromUTF8(element->getAttributeNS("http://www.w3.org/1999/xlink", "href"));
				const size_t link_offset = text.length();
				wxString link_text;
				traverse_children(element, link_text, doc);
				text += link_text;
				doc->buffer.add_link(link_offset, link_text, href);
			}
		} else {
			traverse_children(element, text, doc);
		}
	} else if (node->nodeType() == Poco::XML::Node::TEXT_NODE) {
		auto* text_node = dynamic_cast<Poco::XML::Text*>(node);
		text += wxString::FromUTF8(text_node->data());
	}
}

std::pair<wxString, wxString> odt_parser::process_table(Poco::XML::Element* table_element) {
	wxString html = "<table border=\"1\" style=\"border-collapse: collapse; width: 100%;\">";
	wxString placeholder = "table: ";
	Poco::XML::Node* child = table_element->firstChild();
	bool first_row = true;
	while (child != nullptr) {
		if (child->nodeType() == Poco::XML::Node::ELEMENT_NODE) {
			auto* element = dynamic_cast<Poco::XML::Element*>(child);
			if (element->localName() == "table-row") {
				html += "<tr>";
				Poco::XML::Node* cell_node = element->firstChild();
				while (cell_node != nullptr) {
					if (cell_node->nodeType() == Poco::XML::Node::ELEMENT_NODE) {
						auto* cell_element = dynamic_cast<Poco::XML::Element*>(cell_node);
						if (cell_element->localName() == "table-cell") {
							wxString cell_text = get_cell_text(cell_element);
							if (first_row) {
								placeholder += cell_text + " ";
							}
							wxString style;
							if (cell_element->hasAttributeNS("urn:oasis:names:tc:opendocument:xmlns:table:1.0", "style-name")) {
								// You could parse style information here if needed
							}
							html += wxString::Format("<td style=\"%s\">%s</td>", style, cell_text);
						}
					}
					cell_node = cell_node->nextSibling();
				}
				html += "</tr>";
				first_row = false;
			}
		}
		child = child->nextSibling();
	}
	html += "</table>";
	placeholder.Trim();
	return {placeholder, html};
}

wxString odt_parser::get_cell_text(Poco::XML::Element* cell_element) {
	wxString cell_text;
	Poco::XML::Node* child = cell_element->firstChild();
	bool first_para = true;
	while (child != nullptr) {
		if (child->nodeType() == Poco::XML::Node::ELEMENT_NODE) {
			auto* element = dynamic_cast<Poco::XML::Element*>(child);
			if (element->localName() == "p") {
				if (!first_para) {
					cell_text += "\n";
				}
				Poco::XML::Node* text_node = element->firstChild();
				while (text_node != nullptr) {
					if (text_node->nodeType() == Poco::XML::Node::TEXT_NODE) {
						auto* text = dynamic_cast<Poco::XML::Text*>(text_node);
						cell_text += wxString::FromUTF8(text->data());
					} else if (text_node->nodeType() == Poco::XML::Node::ELEMENT_NODE) {
						auto* text_element = dynamic_cast<Poco::XML::Element*>(text_node);
						if (text_element->localName() == "span") {
							Poco::XML::Node* span_child = text_element->firstChild();
							while (span_child != nullptr) {
								if (span_child->nodeType() == Poco::XML::Node::TEXT_NODE) {
									auto* text = dynamic_cast<Poco::XML::Text*>(span_child);
									cell_text += wxString::FromUTF8(text->data());
								}
								span_child = span_child->nextSibling();
							}
						}
					}
					text_node = text_node->nextSibling();
				}
				first_para = false;
			}
		}
		child = child->nextSibling();
	}
	return cell_text;
}
