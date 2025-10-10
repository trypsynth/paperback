/* docx_parser.cpp - parser for docx documents.
 *
 * Paperback.
 * Copyright (c) 2025 Your Name.
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
#include <Poco/DOM/Element.h>
#include <Poco/DOM/Node.h>
#include <Poco/DOM/NodeList.h>
#include <Poco/DOM/Text.h>
#include <Poco/SAX/InputSource.h>
#include <wx/filename.h>
#include <wx/msgdlg.h>
#include <wx/wfstream.h>
#include <wx/zipstrm.h>

#include <algorithm>
#include <sstream>
#include <vector>

const std::string WORDML_NS = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";

class DocxExtractor {
public:
	DocxExtractor() = default;

	void extract(Poco::XML::Node* pNode) {
		if (!pNode) return;
		traverse(pNode);
	}

	const wxString& get_text() const { return text_; }
	const std::vector<heading_info>& get_headings() const { return headings_; }

private:
	void traverse(Poco::XML::Node* pNode) {
		if (!pNode) return;

		if (pNode->nodeType() == Poco::XML::Node::ELEMENT_NODE) {
			Poco::XML::Element* pElement = static_cast<Poco::XML::Element*>(pNode);
			if (pElement->localName() == "p") {
				process_paragraph(pElement);
				return;
			}
		}

		Poco::XML::Node* pChild = pNode->firstChild();
		while (pChild) {
			traverse(pChild);
			pChild = pChild->nextSibling();
		}
	}

	void process_paragraph(Poco::XML::Element* pElement) {
		std::string paragraph_text_utf8;
		int heading_level = 0;

		Poco::XML::Node* pChild = pElement->firstChild();
		while (pChild) {
			if (pChild->nodeType() == Poco::XML::Node::ELEMENT_NODE) {
				Poco::XML::Element* pChildElement = static_cast<Poco::XML::Element*>(pChild);
				std::string localName = pChildElement->localName();

				if (localName == "pPr") {
					heading_level = get_heading_level(pChildElement);
				} else if (localName == "r") {
					paragraph_text_utf8 += get_run_text(pChildElement);
				}
			}
			pChild = pChild->nextSibling();
		}

		size_t offset = text_.length();
		wxString paragraph_wx = wxString::FromUTF8(paragraph_text_utf8);

		text_ += paragraph_wx;
		text_ += "\n";

		if (heading_level > 0) {
			heading_info h;
			h.offset = offset;
			h.level = heading_level;
			h.text = std::string(paragraph_wx.Trim().utf8_str());

			if (!h.text.empty()) {
				headings_.push_back(h);
			}
		}
	}

	int get_heading_level(Poco::XML::Element* pPrElement) {
		Poco::XML::Node* pChild = pPrElement->firstChild();
		while (pChild) {
			if (pChild->nodeType() == Poco::XML::Node::ELEMENT_NODE) {
				Poco::XML::Element* pElement = static_cast<Poco::XML::Element*>(pChild);
				std::string localName = pElement->localName();

				if (localName == "pStyle") {
					std::string style = pElement->getAttributeNS(WORDML_NS, "val");
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
							} catch (...) {}
						}
					}
				} else if (localName == "outlineLvl") {
					std::string level_str = pElement->getAttributeNS(WORDML_NS, "val");
					if (!level_str.empty()) {
						try {
							int level = std::stoi(level_str) + 1;
							if (level > 0 && level <= 9) return level;
						} catch (...) {}
					}
				}
			}
			pChild = pChild->nextSibling();
		}
		return 0;
	}

	std::string get_run_text(Poco::XML::Element* pRunElement) {
		std::string run_text;
		Poco::XML::Node* pChild = pRunElement->firstChild();
		while (pChild) {
			if (pChild->nodeType() == Poco::XML::Node::ELEMENT_NODE) {
				auto* pElement = static_cast<Poco::XML::Element*>(pChild);
				if (pElement->localName() == "t") {
					if (pElement->firstChild() && pElement->firstChild()->nodeType() == Poco::XML::Node::TEXT_NODE) {
						run_text += pElement->firstChild()->getNodeValue();
					}
				} else if (pElement->localName() == "tab") {
					run_text += "\t";
				} else if (pElement->localName() == "br") {
					run_text += "\n";
				}
			}
			pChild = pChild->nextSibling();
		}
		return run_text;
	}

	wxString text_;
	std::vector<heading_info> headings_;
};

static std::string read_zip_entry(wxZipInputStream& zip) {
	std::ostringstream oss;
	char buffer[4096];

	while (!zip.Eof()) {
		zip.Read(buffer, sizeof(buffer));
		size_t bytes_read = zip.LastRead();
		if (bytes_read > 0) {
			oss.write(buffer, bytes_read);
		}
	}

	return oss.str();
}

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
		Poco::XML::InputSource source(content_stream);
		Poco::XML::DOMParser parser;
		parser.setFeature(Poco::XML::XMLReader::FEATURE_NAMESPACES, true);
		parser.setFeature(Poco::XML::DOMParser::FEATURE_FILTER_WHITESPACE, false);

		Poco::AutoPtr<Poco::XML::Document> pDoc = parser.parse(&source);

		DocxExtractor extractor;
		extractor.extract(pDoc->documentElement());

		auto doc = std::make_unique<document>();
		doc->title = wxFileName(path).GetName();

		doc->buffer.clear();
		doc->buffer.set_content(extractor.get_text());

		const auto& headings = extractor.get_headings();
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