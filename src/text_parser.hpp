#pragma once

#include "parser.hpp"

class text_parser : public parser {
public:
	wxString name() const override {return "Text Files";}
	const std::vector<wxString>& extensions() const override {
		static const std::vector<wxString> exts = { "txt", "log" };
		return exts;
	}
	std::unique_ptr<document> load(const wxString& path) const override;
};

static text_parser text_par;
