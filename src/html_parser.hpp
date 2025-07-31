#pragma once
#include "parser.hpp"

class html_parser : public parser {
public:
	wxString name() const override { return "HTML Documents"; }
	std::span<const wxString> extensions() const override {
		static const wxString exts[] = {"htm", "html", "xhtml"};
		return exts;
	}
	std::unique_ptr<document> load(const wxString& path) const override;
};

REGISTER_PARSER(html_parser)
