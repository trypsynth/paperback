#pragma once
#include "parser.hpp"

class markdown_parser : public parser {
public:
	wxString name() const override { return "Markdown Documents"; }
	std::span<const wxString> extensions() const override {
		static const wxString exts[] = {"md", "markdown", "mdx", "mdown", "mdwn", "mkd", "mkdn", "mkdown", "ronn"};
		return exts;
	}
	std::unique_ptr<document> load(const wxString& path) const override;
};

REGISTER_PARSER(markdown_parser)
