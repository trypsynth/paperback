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

class epub_parser : public parser {
public:
	wxString name() const override {return "Epub Books";}

	const std::vector<wxString>& extensions() const override {
		static const std::vector<wxString> exts = {"epub"};
		return exts;
	}

	std::unique_ptr<document> load(const wxString& path) const override;

private:
	void parse_opf(const std::string& filename, std::ifstream& fp, std::unique_ptr<Poco::Zip::ZipArchive>& archive, std::map<std::string, std::string>& manifest_items, std::vector<std::string>& spine_items, Poco::Path& opf_path, std::string& title, std::string& author) const;
	epub_section parse_section(size_t n, std::ifstream& fp, std::unique_ptr<Poco::Zip::ZipArchive>& archive, const std::map<std::string, std::string>& manifest_items, const std::vector<std::string>& spine_items) const;
	std::string get_section_text(epub_section& section) const;
};

static epub_parser epub_par;
