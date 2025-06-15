#pragma once

#include <fstream>
#include "parser.hpp"
#include <Poco/DOM/Element.h>
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
	wxString name() const override { return "Epub Books"; }
	const std::vector<wxString>& extensions() const override {
		static const std::vector<wxString> exts = {"epub"};
		return exts;
	}
	std::unique_ptr<document> load(const wxString& path) const override;

private:
	void parse_opf(const std::string& filename, std::ifstream& fp, std::unique_ptr<Poco::Zip::ZipArchive>& archive, std::map<std::string, std::string>& manifest_items, std::vector<std::string>& spine_items, Poco::Path& opf_path, std::string& title, std::string& author, std::string& toc_ncx_id, std::string& nav_doc_id) const;
	epub_section parse_section(size_t n, std::ifstream& fp, std::unique_ptr<Poco::Zip::ZipArchive>& archive, const std::map<std::string, std::string>& manifest_items, const std::vector<std::string>& spine_items) const;
	std::string get_section_text(epub_section& section) const;
	void parse_epub2_ncx(const std::string& ncx_id, std::ifstream& fp, std::unique_ptr<Poco::Zip::ZipArchive>& archive, const std::map<std::string, std::string>& manifest_items, const Poco::Path& opf_path, std::vector<std::unique_ptr<toc_item>>& toc_items, const std::map<std::string, std::string>& all_manifest_items, const std::vector<std::string>& spine_items, const std::vector<size_t>& section_offsets) const;
	std::unique_ptr<toc_item> parse_ncx_nav_point(Poco::XML::Element* nav_point, const Poco::XML::NamespaceSupport& nsmap, const Poco::Path& opf_path, const std::map<std::string, std::string>& manifest_items, const std::vector<std::string>& spine_items, const std::vector<size_t>& section_offsets) const;
	void parse_epub3_nav(const std::string& nav_id, std::ifstream& fp, std::unique_ptr<Poco::Zip::ZipArchive>& archive, const std::map<std::string, std::string>& manifest_items, const Poco::Path& opf_path, std::vector<std::unique_ptr<toc_item>>& toc_items, const std::map<std::string, std::string>& all_manifest_items, const std::vector<std::string>& spine_items, const std::vector<size_t>& section_offsets) const;
	void parse_epub3_nav_list(Poco::XML::Element* ol_element, std::vector<std::unique_ptr<toc_item>>& toc_items, const Poco::Path& opf_path, const std::map<std::string, std::string>& manifest_items, const std::vector<std::string>& spine_items, const std::vector<size_t>& section_offsets) const;
	std::unique_ptr<toc_item> parse_epub3_nav_item(Poco::XML::Element* li_element, const Poco::Path& opf_path, const std::map<std::string, std::string>& manifest_items, const std::vector<std::string>& spine_items, const std::vector<size_t>& section_offsets) const;
	int calculate_offset_from_href(const std::string& href, const Poco::Path& opf_path, const std::map<std::string, std::string>& manifest_items, const std::vector<std::string>& spine_items, const std::vector<size_t>& section_offsets) const;
};

static epub_parser epub_par;
