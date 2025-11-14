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
#include <pugixml.hpp>
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

	[[nodiscard]] wxString name() const override {
		return "Word Documents";
	}

	[[nodiscard]] std::span<const wxString> extensions() const override {
		static const wxString exts[] = {"docx", "docm"};
		return exts;
	}

	[[nodiscard]] parser_flags supported_flags() const override {
		return parser_flags::supports_toc;
	}

	[[nodiscard]] std::unique_ptr<document> load(const wxString& path) const override;

private:
	void traverse(pugi::xml_node node, wxString& text, std::vector<heading_info>& headings, document* doc, const std::map<std::string, std::string>& rels) const;
	static void process_paragraph(pugi::xml_node element, wxString& text, std::vector<heading_info>& headings, document* doc, const std::map<std::string, std::string>& rels);
	static void process_hyperlink(pugi::xml_node element, wxString& text, document* doc, const std::map<std::string, std::string>& rels, size_t paragraph_start_offset);
	static int get_paragraph_heading_level(pugi::xml_node pr_element);
	static int get_run_heading_level(pugi::xml_node rpr_element);
	static std::string get_run_text(pugi::xml_node prun_element);
	static std::string parse_hyperlink_instruction(const std::string& instruction);
};

REGISTER_PARSER(docx_parser)
