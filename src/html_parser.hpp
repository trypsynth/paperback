/* html_parser.hpp - HTML parsing header file.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#pragma once
#include "parser.hpp"

class html_parser : public parser {
public:
	html_parser() = default;
	~html_parser() = default;
	html_parser(const html_parser&) = delete;
	html_parser& operator=(const html_parser&) = delete;
	html_parser(html_parser&&) = delete;
	html_parser& operator=(html_parser&&) = delete;

	[[nodiscard]] wxString name() const override {
		return "HTML Documents";
	}

	[[nodiscard]] std::span<const wxString> extensions() const override {
		static const wxString exts[] = {"htm", "html", "xhtml"};
		return exts;
	}
<<<<<<< HEAD
	[[nodiscard]] parser_flags supported_flags() const override { return parser_flags::supports_toc | parser_flags::supports_lists; }
=======

	[[nodiscard]] parser_flags supported_flags() const override {
		return parser_flags::supports_toc;
	}

>>>>>>> master
	[[nodiscard]] std::unique_ptr<document> load(const wxString& path) const override;
};

REGISTER_PARSER(html_parser)
