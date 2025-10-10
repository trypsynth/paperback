/* chm_parser.hpp - header file for Compiled HTML Help file parser.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#pragma once
#include "document.hpp"
#include "document_buffer.hpp"
#include "parser.hpp"
#include <chm_lib.h>
#include <map>
#include <memory>
#include <string>
#include <vector>
#include <wx/string.h>

struct chm_context {
	chmFile* file;
	std::vector<std::string> html_files;
	std::map<std::string, std::map<std::string, size_t>> id_positions;
	std::string title;
	std::string author;
	std::string hhc_file;

	chm_context(chmFile* f)
		: file(f) {}
};

class chm_parser : public parser {
public:
	chm_parser() = default;
	~chm_parser() = default;
	[[nodiscard]] wxString name() const override { return "Compiled HTML Help files"; }
	[[nodiscard]] std::span<const wxString> extensions() const override {
		static const wxString exts[] = {"chm"};
		return exts;
	}
	[[nodiscard]] parser_flags supported_flags() const override { return parser_flags::supports_toc; }
	[[nodiscard]] std::unique_ptr<document> load(const wxString& path) const override;

private:
	void enumerate_files(chm_context& ctx) const;
	void parse_system_file(chm_context& ctx) const;
	void parse_hhc_file(chm_context& ctx, std::vector<std::unique_ptr<toc_item>>& toc_items) const;
	void parse_html_files(chm_context& ctx, document_buffer& buffer, const std::vector<std::unique_ptr<toc_item>>& toc_items) const;
	void collect_html_files_from_toc(const std::vector<std::unique_ptr<toc_item>>& items, std::vector<std::string>& files) const;
	void calculate_toc_offsets(std::vector<std::unique_ptr<toc_item>>& items, const chm_context& ctx) const;
	int calculate_offset_from_path(const std::string& path, const chm_context& ctx) const;
	std::string read_file_content(chmFile* file, const std::string& path) const;
	std::string normalize_path(const std::string& path) const;
	static int file_enumerator(chmFile* h, chmUnitInfo* ui, void* context);
};

REGISTER_PARSER(chm_parser)
