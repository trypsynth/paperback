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
	int next_section_index() const;
	int previous_section_index() const;
	size_t current_offset() const;
	size_t offset_for_section(int section_index) const;

private:
	mutable std::vector<size_t> section_offsets;
	mutable int cur_section = 0;
};

static epub_parser epub_par;
