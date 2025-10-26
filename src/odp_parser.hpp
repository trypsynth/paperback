/* odp_parser.hpp - header file for odp document parser.
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

class odp_parser : public parser {
public:
	odp_parser() = default;
	~odp_parser() = default;
	odp_parser(const odp_parser&) = delete;
	odp_parser& operator=(const odp_parser&) = delete;
	odp_parser(odp_parser&&) = delete;
	odp_parser& operator=(odp_parser&&) = delete;

	[[nodiscard]] wxString name() const override {
		return "OpenDocument Presentations";
	}

	[[nodiscard]] std::span<const wxString> extensions() const override {
		static const wxString exts[] = {"odp"};
		return exts;
	}

	[[nodiscard]] parser_flags supported_flags() const override {
		return parser_flags::supports_pages;
	}

	[[nodiscard]] std::unique_ptr<document> load(const wxString& path) const override;

private:
	void traverse(Poco::XML::Node* node, wxString& text, document* doc, wxString* full_text) const;
	void traverse_children(Poco::XML::Node* node, wxString& text, document* doc, wxString* full_text) const;
};

REGISTER_PARSER(odp_parser)
