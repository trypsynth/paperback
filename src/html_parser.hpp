#pragma once

#include "parser.hpp"

class html_parser : public parser {
public:
	wxString name() const override {return "HTML Documents";}

	const std::vector<wxString>& extensions() const override {
		static const std::vector<wxString> exts = {"htm", "html", "xhtml"};
		return exts;
	}

	parser_flags flags() const {return parser_flags::none;}

	std::unique_ptr<document> load(const wxString& path) const override;
};

static html_parser html_par;
