/* docx_parser.hpp - header file for docx document parser.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#pragma once
#include "document.hpp"
#include "parser.hpp"
#include <Poco/DOM/Element.h>
#include <vector>
#include <wx/stream.h>

class docx_parser : public parser {
public:
	docx_parser() = default;
	~docx_parser() = default;
	docx_parser(const docx_parser&) = delete;
	docx_parser& operator=(const docx_parser&) = delete;
	docx_parser(docx_parser&&) = delete;
	docx_parser& operator=(docx_parser&&) = delete;
	[[nodiscard]] wxString name() const override { return "Word Documents"; }
	[[nodiscard]] std::span<const wxString> extensions() const override {
		static const wxString exts[] = {"docx", "docm"};
		return exts;
	}
	[[nodiscard]] parser_flags supported_flags() const override { return parser_flags::supports_toc; }
	[[nodiscard]] std::unique_ptr<document> load(const wxString& path) const override;

private:
	void traverse(Poco::XML::Node* node, wxString& text, std::vector<heading_info>& headings, document* doc, const std::map<std::string, std::string>& rels) const;
	void process_paragraph(Poco::XML::Element* pElement, wxString& text, std::vector<heading_info>& headings, document* doc, const std::map<std::string, std::string>& rels) const;
	void process_hyperlink(Poco::XML::Element* element, wxString& text, document* doc, const std::map<std::string, std::string>& rels, size_t paragraph_start_offset) const;
	[[nodiscard]] int get_heading_level(Poco::XML::Element* pPrElement) const;
	[[nodiscard]] std::string get_run_text(Poco::XML::Element* pRunElement) const;
	[[nodiscard]] std::string parse_hyperlink_instruction(const std::string& instruction) const;
};

REGISTER_PARSER(docx_parser)
