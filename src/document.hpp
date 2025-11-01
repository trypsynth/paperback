/* document.hpp - document interface header file.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#pragma once
#include "document_buffer.hpp"
#include <map>
#include <memory>
#include <string>
#include <vector>
#include <wx/string.h>

struct toc_item {
	wxString name;
	wxString ref;
	std::vector<std::unique_ptr<toc_item>> children;
	int offset;

	toc_item() = default;
	~toc_item() = default;
	toc_item(const toc_item&) = delete;
	toc_item& operator=(const toc_item&) = delete;
	toc_item(toc_item&&) = default;
	toc_item& operator=(toc_item&&) = default;
};

struct heading_info {
	size_t offset;
	int level;
	std::string text;
};

struct table_info {
	size_t offset;
	std::string text;
	std::string ref;
};

struct document_stats {
	size_t word_count{0};
	size_t line_count{0};
	size_t char_count{0};
	size_t char_count_no_whitespace{0};
};

struct document {
	wxString title{"Untitled"};
	wxString author{"Unknown"};
	document_buffer buffer;
	std::vector<std::unique_ptr<toc_item>> toc_items;
	std::map<std::string, size_t> id_positions;
	std::map<size_t, wxString> html_tables;
	std::vector<std::string> spine_items;
	std::map<std::string, std::string> manifest_items;
	mutable document_stats stats;

	document() = default;
	~document() = default;
	document(const document&) = delete;
	document& operator=(const document&) = delete;
	document(document&&) = default;
	document& operator=(document&&) = default;

	[[nodiscard]] int next_section_index(int position) const noexcept;
	[[nodiscard]] int previous_section_index(int position) const noexcept;
	[[nodiscard]] int section_index(size_t position) const noexcept;
	[[nodiscard]] int offset_for_section(int section_index) const noexcept;
	[[nodiscard]] int next_page_index(int position) const noexcept;
	[[nodiscard]] int previous_page_index(int position) const noexcept;
	[[nodiscard]] int page_index(size_t position) const noexcept;
	[[nodiscard]] int offset_for_page(int page_index) const noexcept;
	[[nodiscard]] int find_closest_toc_offset(size_t position) const noexcept;
	[[nodiscard]] int next_heading_index(int position, int level) const noexcept;
	[[nodiscard]] int previous_heading_index(int position, int level) const noexcept;
	[[nodiscard]] int offset_for_heading(int heading_index) const noexcept;
	[[nodiscard]] const marker* get_heading_marker(int heading_index) const noexcept;
	void calculate_statistics() const;
};
