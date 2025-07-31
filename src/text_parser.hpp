#pragma once
#include "parser.hpp"

class text_parser : public parser {
public:
	wxString name() const override { return "Text Files"; }
	std::span<const wxString> extensions() const override {
		static const wxString exts[] = {"txt", "log"};
		return exts;
	}
	std::unique_ptr<document> load(const wxString& path) const override;
};

REGISTER_PARSER(text_parser)
