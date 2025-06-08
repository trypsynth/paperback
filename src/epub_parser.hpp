#pragma once

#include <fstream>
#include "parser.hpp"
#include <Poco/Zip/ZipArchive.h>
#include <stdexcept>
#include <string>
#include <vector>

struct epub_section {
	std::vector<std::string> lines;
};

class parse_error : public std::runtime_error {
public:
	using std::runtime_error::runtime_error;
};

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

	std::unique_ptr<document> load(const wxString& path) override;
	int next_section_index(size_t position) const override;
	int previous_section_index(size_t position) const override;
	int section_index(size_t position) const override;
	size_t offset_for_section(int section_index) const override;
	size_t section_count() const override;

private:
	std::ifstream fp;
	std::unique_ptr<Poco::Zip::ZipArchive> archive;
	// Map of manifest ids to hrefs
	std::map<std::string, std::string> manifest_items;
	std::vector<std::string> spine_items;
	Poco::Path opf_path;
	std::string title, author;

	void parse_opf(const std::string& filename);
	epub_section parse_section(size_t n);
	std::string get_section_text(epub_section& section);
};

static epub_parser epub_par;
