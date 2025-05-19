#pragma once

#include "parser.hpp"

class text_parser : public parser {
public:
	wxString name() const override {return "Text Files";}
	std::vector<wxString> extensions() const override {return {"txt", "log"};}
};

static text_parser text_par;
