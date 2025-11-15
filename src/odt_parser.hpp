/* odt_parser.hpp - odt parser header file.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#pragma once
#include "parser.hpp"
#include <pugixml.hpp>

class odt_parser : public parser {
public:
	odt_parser() = default;
	~odt_parser() = default;
	odt_parser(const odt_parser&) = delete;
	odt_parser& operator=(const odt_parser&) = delete;
	odt_parser(odt_parser&&) = delete;
	odt_parser& operator=(odt_parser&&) = delete;

	[[nodiscard]] wxString name() const override {
		return "OpenDocument files";
	}

	[[nodiscard]] std::span<const wxString> extensions() const override {
		static const wxString exts[] = {"odt"};
		return exts;
	}

	[[nodiscard]] parser_flags supported_flags() const override {
		return parser_flags::supports_toc;
	}

	[[nodiscard]] std::unique_ptr<document> load(const parser_context& ctx) const override;

private:
	void traverse(pugi::xml_node node, wxString& text, document* doc) const;
	void traverse_children(pugi::xml_node node, wxString& text, document* doc) const;
};

REGISTER_PARSER(odt_parser);
