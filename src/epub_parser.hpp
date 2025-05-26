#pragma once

#include "parser.hpp"

class epub_parser : public parser {
public:
	wxString name() const override {return "Epub Books";}

	const std::vector<wxString>& extensions() const override {
		static const std::vector<wxString> exts = {"epub"};
		return exts;
	}

	std::unique_ptr<document> load(const wxString& path) const override;
};

static epub_parser epub_par;
