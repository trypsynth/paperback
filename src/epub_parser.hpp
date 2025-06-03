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
	int next_section_index() const override;
	int previous_section_index() const override;
	size_t current_offset() const override;
	size_t offset_for_section(int section_index) const override;
	int current_section_index() const override;
	size_t section_count() const override;

private:
	mutable std::vector<size_t> section_offsets;
	mutable int cur_section = 0;
};

static epub_parser epub_par;
