/* fb2_parser.hpp - fb2 parser header file.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#pragma once
#include "parser.hpp"
#include <Poco/DOM/Element.h>

class fb2_parser : public parser {
public:
	fb2_parser() = default;
	~fb2_parser() = default;
	fb2_parser(const fb2_parser&) = delete;
	fb2_parser& operator=(const fb2_parser&) = delete;
	fb2_parser(fb2_parser&&) = delete;
	fb2_parser& operator=(fb2_parser&&) = delete;
	[[nodiscard]] wxString name() const override { return "FB2 Books"; }
	[[nodiscard]] std::span<const wxString> extensions() const override {
		static const wxString exts[] = {"fb2"};
		return exts;
	}
	[[nodiscard]] parser_flags supported_flags() const override { return parser_flags::supports_sections; }
	[[nodiscard]] std::unique_ptr<document> load(const wxString &path) const override;

private:
	static std::string get_element_text(Poco::XML::Element* element);
};

REGISTER_PARSER(fb2_parser);
