#pragma once

#include "parser.hpp"

class html_parser : public parser {
public:
	wxString name() const override {return "HTML documents";}

	const std::vector<wxString>& extensions() const override {
		static const std::vector<wxString> exts = {"html", "htm"};
		return exts;
	}

	parser_flags flags() const override { 
		return parser_flags::supports_toc;
	}

	std::unique_ptr<document> load(const wxString& path) override;
};

static html_parser html_par;
