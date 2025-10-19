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
#include <Poco/XML/XMLString.h>
#include <algorithm>
#include <cctype>
#include <cstddef>
#include <map>
#include <memory>
#include <sstream>
#include <string>
#include <vector>
#include <wx/filename.h>
#include <wx/msgdlg.h>
#include <wx/string.h>
#include <wx/wfstream.h>
#include <wx/zipstrm.h>

using namespace Poco;
using namespace Poco::XML;

// NOLINTNEXTLINE(cert-err58-cpp) - String construction from literals is safe in practice
inline const XMLString WORDML_NS = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";
// NOLINTNEXTLINE(cert-err58-cpp) - String construction from literals is safe in practice
inline const XMLString REL_NS = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";

std::unique_ptr<document> docx_parser::load(const wxString& path) const {
	try {
		auto fp = std::make_unique<wxFileInputStream>(path);
		if (!fp->IsOk()) {
			return nullptr;
		}
		wxZipInputStream zip(*fp);
		if (!zip.IsOk()) {
			return nullptr;
		}
		std::string rels_content;
		std::string doc_content;
		std::unique_ptr<wxZipEntry> entry;
		while ((entry.reset(zip.GetNextEntry())), entry != nullptr) {
			const std::string entry_name = entry->GetInternalName().ToStdString();
			if (entry_name == "word/_rels/document.xml.rels") {
				rels_content = read_zip_entry(zip);
			} else if (entry_name == "word/document.xml") {
				doc_content = read_zip_entry(zip);
			}
			if (!rels_content.empty() && !doc_content.empty()) {
				break;
			}
		}
		if (doc_content.empty()) {
			return nullptr;
		}
		std::map<std::string, std::string> rels;
		if (!rels_content.empty()) {
			std::istringstream rels_stream(rels_content);
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
		}
		std::istringstream content_stream(doc_content);
		InputSource source(content_stream);
		DOMParser parser;
		parser.setFeature(XMLReader::FEATURE_NAMESPACES, true);
		parser.setFeature(DOMParser::FEATURE_FILTER_WHITESPACE, false);
		AutoPtr<Document> p_doc = parser.parse(&source);
		auto doc = std::make_unique<document>();
		doc->title = wxFileName(path).GetName();
		doc->buffer.clear();
		wxString text;
		std::vector<heading_info> headings;
		traverse(p_doc->documentElement(), text, headings, doc.get(), rels);
		doc->buffer.set_content(text);
		for (const auto& heading : headings) {
			const auto type = static_cast<marker_type>(static_cast<int>(marker_type::heading_1) + heading.level - 1);
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

void docx_parser::traverse(Node* node, wxString& text, std::vector<heading_info>& headings, document* doc, const std::map<std::string, std::string>& rels) const {
	if (node == nullptr) {
		return;
	}
	if (node->nodeType() == Node::ELEMENT_NODE) {
		auto* element = dynamic_cast<Element*>(node);
		const std::string local_name = element->localName();
		const std::string id_attr = element->getAttributeNS(WORDML_NS, "id");
		if (!id_attr.empty()) {
			doc->id_positions[id_attr] = text.length();
		}
		if (local_name == "p") {
			process_paragraph(element, text, headings, doc, rels);
			return; // process_paragraph handles its children
		}
	}
	Node* child = node->firstChild();
	while (child != nullptr) {
		traverse(child, text, headings, doc, rels);
		child = child->nextSibling();
	}
}

void docx_parser::process_paragraph(Element* element, wxString& text, std::vector<heading_info>& headings, document* doc, const std::map<std::string, std::string>& rels) {
	wxString paragraph_text;
	int heading_level = 0;
	const size_t paragraph_start_offset{text.length()};
	Node* child = element->firstChild();
	while (child != nullptr) {
		if (child->nodeType() != Node::ELEMENT_NODE) {
			child = child->nextSibling();
			continue;
		}
		auto* child_element = dynamic_cast<Element*>(child);
		const std::string local_name = child_element->localName();
		if (local_name == "pPr") {
			heading_level = get_heading_level(child_element);
		} else if (local_name == "bookmarkStart") {
			const std::string name_attr = child_element->getAttributeNS(WORDML_NS, "name");
			if (!name_attr.empty()) {
				doc->id_positions[name_attr] = paragraph_start_offset + paragraph_text.length();
			}
		} else if (local_name == "hyperlink") {
			process_hyperlink(child_element, paragraph_text, doc, rels, paragraph_start_offset);
		} else if (local_name == "r") {
			const Element* instr_text_element = nullptr;
			Node* node = child_element->firstChild();
			while (node != nullptr) {
				if (node->nodeType() == Node::ELEMENT_NODE) {
					const auto* el = dynamic_cast<Element*>(node);
					if (el->localName() == "instrText" && el->namespaceURI() == WORDML_NS) {
						instr_text_element = el;
						break;
					}
				}
				node = node->nextSibling();
			}
			if (instr_text_element != nullptr && instr_text_element->innerText().find("HYPERLINK") != std::string::npos) {
				const std::string instruction = instr_text_element->innerText();
				const std::string link_target = parse_hyperlink_instruction(instruction);
				if (!link_target.empty()) {
					std::string display_text_utf8;
					const size_t link_offset_in_paragraph = paragraph_text.length();
					Node* field_node = child->nextSibling();
					bool in_display_text = false;
					while (field_node != nullptr) {
						if (field_node->nodeType() == Node::ELEMENT_NODE && dynamic_cast<Element*>(field_node)->localName() == "r") {
							auto* field_run = dynamic_cast<Element*>(field_node);
							const Element* fld_char_element = nullptr;
							Node* node = field_run->firstChild();
							while (node != nullptr) {
								if (node->nodeType() == Node::ELEMENT_NODE) {
									auto* el = dynamic_cast<Element*>(node);
									if (el->localName() == "fldChar" && el->namespaceURI() == WORDML_NS) {
										fld_char_element = el;
										break;
									}
								}
								node = node->nextSibling();
							}
							if (fld_char_element != nullptr) {
								const std::string type = fld_char_element->getAttributeNS(WORDML_NS, "fldCharType");
								if (type == "separate") {
									in_display_text = true;
								} else if (type == "end") {
									break;
								}
							} else if (in_display_text) {
								display_text_utf8 += get_run_text(field_run);
							}
						}
						field_node = field_node->nextSibling();
					}
					const wxString display_text_wx = wxString::FromUTF8(display_text_utf8);
					if (!display_text_wx.IsEmpty()) {
						paragraph_text += display_text_wx;
						doc->buffer.add_link(paragraph_start_offset + link_offset_in_paragraph, display_text_wx, wxString::FromUTF8(link_target));
					}
					child = field_node;
					if (child != nullptr) {
						child = child->nextSibling();
					}
					continue;
				}
			}
			paragraph_text += wxString::FromUTF8(get_run_text(child_element));
		}
		child = child->nextSibling();
	}
	paragraph_text.Trim(true).Trim(false);
	if (!paragraph_text.IsEmpty()) {
		text += paragraph_text;
		text += "\n";
		if (heading_level > 0) {
			heading_info h;
			h.offset = paragraph_start_offset;
			h.level = heading_level;
			h.text = std::string(paragraph_text.utf8_str());
			if (!h.text.empty()) {
				headings.push_back(h);
			}
		}
	}
}

std::string docx_parser::parse_hyperlink_instruction(const std::string& instruction) {
	const size_t first_quote = instruction.find('"');
	const size_t last_quote = instruction.rfind('"');
	if (first_quote != std::string::npos && last_quote != std::string::npos && first_quote != last_quote) {
		std::string target = instruction.substr(first_quote + 1, last_quote - first_quote - 1);
		if (instruction.find("\\l") != std::string::npos) {
			return "#" + target;
		}
		return target;
	}
	return "";
}

void docx_parser::process_hyperlink(Element* element, wxString& text, document* doc, const std::map<std::string, std::string>& rels, size_t paragraph_start_offset) {
	const std::string r_id = element->getAttributeNS(REL_NS, "id");
	const std::string anchor = element->getAttributeNS(WORDML_NS, "anchor");
	std::string link_target;
	if (!r_id.empty()) {
		auto it = rels.find(r_id);
		if (it != rels.end()) {
			link_target = it->second;
		}
	} else if (!anchor.empty()) {
		link_target = "#" + anchor;
	}
	if (link_target.empty()) { // If no target, just process the text
		Node* child = element->firstChild();
		while (child != nullptr) {
			if (child->nodeType() == Node::ELEMENT_NODE) {
				auto* child_element = dynamic_cast<Element*>(child);
				if (child_element->localName() == "r") {
					text += wxString::FromUTF8(get_run_text(child_element));
				}
			}
			child = child->nextSibling();
		}
		return;
	}
	const size_t link_offset = text.length();
	std::string link_text_utf8;
	Node* child = element->firstChild();
	while (child != nullptr) {
		if (child->nodeType() == Node::ELEMENT_NODE) {
			auto* child_element = dynamic_cast<Element*>(child);
			if (child_element->localName() == "r") {
				link_text_utf8 += get_run_text(child_element);
			}
		}
		child = child->nextSibling();
	}
	const wxString link_text_wx = wxString::FromUTF8(link_text_utf8);
	if (!link_text_wx.IsEmpty()) {
		text += link_text_wx;
		doc->buffer.add_link(paragraph_start_offset + link_offset, link_text_wx, wxString::FromUTF8(link_target));
	}
}

int docx_parser::get_heading_level(Element* pr_element) {
	constexpr int max_heading_level = 9;
	Node* child = pr_element->firstChild();
	while (child != nullptr) {
		if (child->nodeType() == Node::ELEMENT_NODE) {
			auto* element = dynamic_cast<Element*>(child);
			const std::string local_name = element->localName();
			if (local_name == "pStyle") {
				const std::string style = element->getAttributeNS(WORDML_NS, "val");
				if (!style.empty()) {
					std::string style_lower = style;
					std::ranges::transform(style_lower, style_lower.begin(), ::tolower);
					if (style_lower.starts_with("heading")) {
						try {
							const size_t num_pos = style.find_first_of("0123456789");
							if (num_pos != std::string::npos) {
								const int level = std::stoi(style.substr(num_pos));
								if (level > 0 && level <= max_heading_level) {
									return level;
								}
							}
						} catch (...) { // NOLINT(bugprone-empty-catch) - Invalid heading numbers are silently skipped
						}
					}
				}
			} else if (local_name == "outlineLvl") {
				const std::string level_str = element->getAttributeNS(WORDML_NS, "val");
				if (!level_str.empty()) {
					try {
						const int level = std::stoi(level_str) + 1;
						if (level > 0 && level <= max_heading_level) {
							return level;
						}
					} catch (...) { // NOLINT(bugprone-empty-catch) - Invalid outline level values are silently skipped
					}
				}
			}
		}
		child = child->nextSibling();
	}
	return 0;
}

std::string docx_parser::get_run_text(Element* run_element) {
	std::string run_text;
	Node* child = run_element->firstChild();
	while (child != nullptr) {
		if (child->nodeType() == Node::ELEMENT_NODE) {
			auto* element = dynamic_cast<Element*>(child);
			if (element->localName() == "t") {
				if (element->firstChild() != nullptr && element->firstChild()->nodeType() == Node::TEXT_NODE) {
					run_text += element->firstChild()->getNodeValue();
				}
			} else if (element->localName() == "tab") {
				run_text += "\t";
			} else if (element->localName() == "br") {
				run_text += "\n";
			}
		}
		child = child->nextSibling();
	}
	return run_text;
}
