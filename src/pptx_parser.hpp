/* pptx_parser.hpp - header file for pptx document parser.
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
#include <Poco/DOM/Node.h>

class pptx_parser : public parser {
public:
	pptx_parser() = default;
	~pptx_parser() = default;
	pptx_parser(const pptx_parser&) = delete;
	pptx_parser& operator=(const pptx_parser&) = delete;
	pptx_parser(pptx_parser&&) = delete;
	pptx_parser& operator=(pptx_parser&&) = delete;

	[[nodiscard]] wxString name() const override {
		return "PowerPoint Presentations";
	}

	[[nodiscard]] std::span<const wxString> extensions() const override {
		static const wxString exts[] = {"pptx", "pptm"};
		return exts;
	}

	[[nodiscard]] parser_flags supported_flags() const override {
		return parser_flags::supports_pages | parser_flags::supports_toc;
	}

	[[nodiscard]] std::unique_ptr<document> load(const wxString& path) const override;

private:
	void extract_text_from_node(Poco::XML::Node* node, std::string& text, wxString& full_text, document* doc, const std::map<std::string, std::string>& rels) const;
	wxString extract_slide_title(Poco::XML::Document* slide_doc) const;
};

REGISTER_PARSER(pptx_parser)
