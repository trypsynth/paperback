#pragma once

#include <fstream>
#include <stdexcept>
#include <string>
#include <vector>
#include <Poco/Zip/ZipArchive.h>

struct epub_section {
	std::vector<std::string> lines;
};

class parse_error : public std::runtime_error {
public:
	using std::runtime_error::runtime_error;
};

class epub {
public:
	~epub() = default;
	bool load(const std::string& fname);
	int get_num_sections() const;
	std::string get_section_text(epub_section& section);
	epub_section parse_section(unsigned int n, std::vector<std::string>* lines);
	const std::string& title() const{return title_;}
	const std::string& author() const{return author_;}

private:
	void parse_opf(const std::string& filename);
	std::ifstream fp;
	std::unique_ptr<Poco::Zip::ZipArchive> archive;
	// Map of manifest ids to hrefs
	std::map<std::string, std::string> manifest_items;
	std::vector<std::string> spine_items;
	Poco::Path opf_path;
	std::string title_;
	std::string author_;
};
