/* docx_parser.hpp - header file for docx document parser.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#pragma once
#include "parser.hpp"
#include "document.hpp"
#include <vector>
#include <Poco/DOM/Element.h>

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
		static const wxString exts[] = {"docx"};
		return exts;
	}
	[[nodiscard]] parser_flags supported_flags() const override { return parser_flags::supports_toc; }
	[[nodiscard]] std::unique_ptr<document> load(const wxString& path) const override;

private:
	void traverse(Poco::XML::Node* pNode, wxString& text, std::vector<heading_info>& headings) const;
	void process_paragraph(Poco::XML::Element* pElement, wxString& text, std::vector<heading_info>& headings) const;
	[[nodiscard]] int get_heading_level(Poco::XML::Element* pPrElement) const;
	[[nodiscard]] std::string get_run_text(Poco::XML::Element* pRunElement) const;
};

REGISTER_PARSER(docx_parser)
