#pragma once

#include "parser.hpp"

class epub_parser : public parser, public section_navigable {
public:
	wxString name() const override {return "Epub Books";}

	const std::vector<wxString>& extensions() const override {
		static const std::vector<wxString> exts = {"epub"};
		return exts;
	}

	parser_flags flags() const override { 
		return parser_flags::supports_sections | parser_flags::supports_toc;
	}

	std::unique_ptr<document> load(const wxString& path) const override;
	int next_section_index(size_t position) const override;
	int previous_section_index(size_t position) const override;
	int section_index(size_t position) const override;
	size_t offset_for_section(int section_index) const override;
	size_t section_count() const override;
};

static epub_parser epub_par;
