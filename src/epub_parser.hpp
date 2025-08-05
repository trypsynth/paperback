#pragma once
#include "parser.hpp"
#include <Poco/DOM/Element.h>
#include <Poco/Path.h>
#include <Poco/Zip/ZipArchive.h>
#include <fstream>
#include <map>
#include <memory>
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
	std::span<const wxString> extensions() const override {
		static const wxString exts[] = {"epub"};
		return exts;
	}
	std::unique_ptr<document> load(const wxString& path) const override;

private:
	struct epub_context {
		std::ifstream& file_stream;
		std::unique_ptr<Poco::Zip::ZipArchive>& archive;
		std::map<std::string, std::string> manifest_items;
		std::vector<std::string> spine_items;
		std::vector<size_t> section_offsets;
		Poco::Path opf_path;
		std::string title;
		std::string author;
		std::string toc_ncx_id;
		std::string nav_doc_id;
		std::string epub_version;

		epub_context(std::ifstream& fs, std::unique_ptr<Poco::Zip::ZipArchive>& arch) : file_stream(fs), archive(arch) {}

		bool is_epub3() const {
			return epub_version.starts_with("3.");
		}
	};

	void parse_opf(const std::string& filename, epub_context& ctx) const;
	epub_section parse_section(size_t index, const epub_context& ctx) const;
	void parse_toc(epub_context& ctx, std::vector<std::unique_ptr<toc_item>>& toc_items) const;
	void parse_epub2_ncx(const std::string& ncx_id, const epub_context& ctx, std::vector<std::unique_ptr<toc_item>>& toc_items) const;
	void parse_epub3_nav(const std::string& nav_id, const epub_context& ctx, std::vector<std::unique_ptr<toc_item>>& toc_items) const;
	std::unique_ptr<toc_item> parse_ncx_nav_point(Poco::XML::Element* nav_point, const Poco::XML::NamespaceSupport& nsmap, const epub_context& ctx) const;
	void parse_epub3_nav_list(Poco::XML::Element* ol_element, std::vector<std::unique_ptr<toc_item>>& toc_items, const epub_context& ctx) const;
	std::unique_ptr<toc_item> parse_epub3_nav_item(Poco::XML::Element* li_element, const epub_context& ctx) const;
	int calculate_offset_from_href(const std::string& href, const epub_context& ctx) const;
	std::string extract_zip_entry_content(const std::string& filename, const epub_context& ctx) const;
};

REGISTER_PARSER(epub_parser)
