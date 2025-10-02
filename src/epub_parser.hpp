/* epub_parser.hpp - epub 2/3 parser header file.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

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

	epub_section() = default;
	~epub_section() = default;
	epub_section(const epub_section&) = default;
	epub_section& operator=(const epub_section&) = default;
	epub_section(epub_section&&) = default;
	epub_section& operator=(epub_section&&) = default;
};

struct manifest_item {
	std::string path;
	std::string media_type;

	manifest_item() = default;
	~manifest_item() = default;
	manifest_item(const manifest_item&) = default;
	manifest_item& operator=(const manifest_item&) = default;
	manifest_item(manifest_item&&) = default;
	manifest_item& operator=(manifest_item&&) = default;
};

class parse_error : public std::runtime_error {
public:
	using std::runtime_error::runtime_error;
};

class epub_parser : public parser {
public:
	epub_parser() = default;
	~epub_parser() = default;
	epub_parser(const epub_parser&) = delete;
	epub_parser& operator=(const epub_parser&) = delete;
	epub_parser(epub_parser&&) = delete;
	epub_parser& operator=(epub_parser&&) = delete;
	[[nodiscard]] wxString name() const override { return "Epub Books"; }
	[[nodiscard]] std::span<const wxString> extensions() const override {
		static const wxString exts[] = {"epub"};
		return exts;
	}
	[[nodiscard]] std::unique_ptr<document> load(const wxString& path) const override;

private:
	struct epub_context {
		std::ifstream& file_stream;
		std::unique_ptr<Poco::Zip::ZipArchive>& archive;
		std::map<std::string, manifest_item> manifest_items;
		std::vector<std::string> spine_items;
		std::map<std::string, std::map<std::string, size_t>> id_positions;
		Poco::Path opf_path;
		std::string title;
		std::string author;
		std::string toc_ncx_id;
		std::string nav_doc_id;

		epub_context(std::ifstream& fs, std::unique_ptr<Poco::Zip::ZipArchive>& arch) : file_stream(fs), archive(arch) {}
	};

	void parse_opf(const std::string& filename, epub_context& ctx) const;
	void parse_section(size_t index, epub_context& ctx, document_buffer& buffer) const;
	void parse_toc(epub_context& ctx, std::vector<std::unique_ptr<toc_item>>& toc_items, const document_buffer& buffer) const;
	void parse_epub2_ncx(const std::string& ncx_id, const epub_context& ctx, std::vector<std::unique_ptr<toc_item>>& toc_items, const document_buffer& buffer) const;
	void parse_epub3_nav(const std::string& nav_id, const epub_context& ctx, std::vector<std::unique_ptr<toc_item>>& toc_items, const document_buffer& buffer) const;
	std::unique_ptr<toc_item> parse_ncx_nav_point(Poco::XML::Element* nav_point, const Poco::XML::NamespaceSupport& nsmap, const epub_context& ctx, const document_buffer& buffer) const;
	void parse_epub3_nav_list(Poco::XML::Element* ol_element, std::vector<std::unique_ptr<toc_item>>& toc_items, const epub_context& ctx, const document_buffer& buffer, const Poco::Path& nav_base_path) const;
	std::unique_ptr<toc_item> parse_epub3_nav_item(Poco::XML::Element* li_element, const epub_context& ctx, const document_buffer& buffer, const Poco::Path& nav_base_path) const;
	int calculate_offset_from_href(const std::string& href, const epub_context& ctx, const document_buffer& buffer) const;
	[[nodiscard]] bool is_html_content(const std::string& media_type) const;
	[[nodiscard]] std::string extract_zip_entry_content(const std::string& filename, const epub_context& ctx) const;
};

REGISTER_PARSER(epub_parser)
