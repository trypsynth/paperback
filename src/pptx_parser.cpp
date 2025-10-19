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
#include <Poco/AutoPtr.h>
#include <Poco/DOM/DOMParser.h>
#include <Poco/DOM/Document.h>
#include <Poco/DOM/Element.h>
#include <Poco/DOM/Node.h>
#include <Poco/DOM/NodeList.h>
#include <Poco/Exception.h>
#include <Poco/SAX/InputSource.h>
#include <Poco/SAX/XMLReader.h>
#include <algorithm>
#include <cctype>
#include <cstddef>
#include <functional>
#include <map>
#include <memory>
#include <numeric>
#include <ranges>
#include <sstream>
#include <string>
#include <utility>
#include <vector>
#include <wx/filename.h>
#include <wx/msgdlg.h>
#include <wx/string.h>
#include <wx/translation.h>
#include <wx/wfstream.h>
#include <wx/zipstrm.h>

using namespace Poco;
using namespace Poco::XML;

const std::string DRAWINGML_NS = "http://schemas.openxmlformats.org/drawingml/2006/main";
const std::string REL_NS = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";

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
		for (const auto& slide_file : slide_files) {
			const std::string& slide_content = slide_contents[slide_file];
			std::map<std::string, std::string> rels;
			const std::string slide_base = slide_file.substr(slide_file.find_last_of('/') + 1);
			const std::string rels_file = "ppt/slides/_rels/" + slide_base + ".rels";
			auto rels_it = slide_rels.find(rels_file);
			if (rels_it != slide_rels.end()) {
				try {
					std::istringstream rels_stream(rels_it->second);
					InputSource rels_source(rels_stream);
					DOMParser rels_parser;
					rels_parser.setFeature(XMLReader::FEATURE_NAMESPACES, true);
					AutoPtr<Document> rels_doc = rels_parser.parse(&rels_source);
					const NodeList* rel_nodes = rels_doc->getElementsByTagNameNS(REL_NS, "Relationship");
					for (unsigned long i = 0; i < rel_nodes->length(); ++i) {
						Node* node = rel_nodes->item(i);
						auto* element = dynamic_cast<Element*>(node);
						const std::string id = element->getAttribute("Id");
						const std::string target = element->getAttribute("Target");
						const std::string type = element->getAttribute("Type");
						if (type == "http://schemas.openxmlformats.org/officeDocument/2006/relationships/hyperlink") {
							rels[id] = target;
						}
					}
				} catch (...) {
					wxMessageBox(_("Parsing of links in the document failed."), _("Warning"), wxICON_WARNING);
				}
			}
			std::istringstream content_stream(slide_content);
			InputSource source(content_stream);
			DOMParser parser;
			parser.setFeature(XMLReader::FEATURE_NAMESPACES, true);
			parser.setFeature(DOMParser::FEATURE_FILTER_WHITESPACE, false);
			AutoPtr<Document> slide_doc = parser.parse(&source);
			std::string slide_text;
			extract_text_from_node(slide_doc->documentElement(), slide_text, full_text, doc.get(), rels);
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
		for (size_t i = 0; i < slide_positions.size(); ++i) {
			doc->buffer.add_marker(slide_positions[i], marker_type::page_break, wxString::Format("Slide %zu", i + 1));
		}
		return doc;
	} catch (const Poco::Exception& e) {
		wxMessageBox("XML parsing error: " + wxString(e.displayText()), "Error", wxICON_ERROR);
		return nullptr;
	} catch (...) {
		wxMessageBox("Unknown error while parsing PPTX file", "Error", wxICON_ERROR);
		return nullptr;
	}
}

void pptx_parser::extract_text_from_node(Node* node, std::string& text, wxString& full_text, document* doc, const std::map<std::string, std::string>& rels) const {
	if (node == nullptr) {
		return;
	}
	if (node->nodeType() == Node::ELEMENT_NODE) {
		auto* element = dynamic_cast<Element*>(node);
		if (element->localName() == "t") {
			const Node* text_node = element->firstChild();
			if (text_node != nullptr && text_node->nodeType() == Node::TEXT_NODE) {
				text += text_node->getNodeValue();
			}
		} else if (element->localName() == "br") {
			text += "\n";
		} else if (element->localName() == "p") {
			Node* child = node->firstChild();
			while (child != nullptr) {
				extract_text_from_node(child, text, full_text, doc, rels);
				child = child->nextSibling();
			}
			if (!text.empty() && text.back() != '\n') {
				text += "\n";
			}
			return; // Don't process children again.
		} else if (element->localName() == "hlinkClick" && element->namespaceURI() == DRAWINGML_NS) {
			const std::string r_id = element->getAttributeNS(REL_NS, "id");
			std::string link_target;
			if (!r_id.empty()) {
				auto it = rels.find(r_id);
				if (it != rels.end()) {
					link_target = it->second;
				}
			}
			// In PPTX, the link wraps the text runs, so we need to find the text within this subtree.
			std::string link_text_utf8;
			std::function<void(Node*)> extract_link_text = [&](Node* n) {
				if (!n) {
					return;
				}
				if (n->nodeType() == Node::ELEMENT_NODE) {
					const auto* el = dynamic_cast<Element*>(n);
					if (el->localName() == "t") {
						const Node* tn = el->firstChild();
						if (tn && tn->nodeType() == Node::TEXT_NODE) {
							link_text_utf8 += tn->getNodeValue();
						}
					}
				}
				Node* c = n->firstChild();
				while (c) {
					extract_link_text(c);
					c = c->nextSibling();
				}
			};
			Node* parent = node->parentNode();
			if (parent != nullptr) {
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
	Node* child = node->firstChild();
	while (child != nullptr) {
		extract_text_from_node(child, text, full_text, doc, rels);
		child = child->nextSibling();
	}
}
