/* document.hpp - document interface header file.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#pragma once
#include <memory>
#include <vector>
#include <wx/string.h>
#include "document_buffer.hpp"

enum class document_flags {
	none = 0,
	supports_sections = 1 << 0,
	supports_toc = 1 << 1,
	supports_pages = 1 << 2,
};

inline constexpr document_flags operator|(document_flags a, document_flags b) noexcept {
	return static_cast<document_flags>(static_cast<int>(a) | static_cast<int>(b));
}

inline constexpr document_flags operator&(document_flags a, document_flags b) noexcept {
	return static_cast<document_flags>(static_cast<int>(a) & static_cast<int>(b));
}

inline constexpr document_flags& operator|=(document_flags& a, document_flags b) noexcept {
	return a = a | b;
}

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

struct document_stats {
	int word_count{0};
	int line_count{0};
	int char_count{0};
	int char_count_no_whitespace{0};

	document_stats() = default;
	~document_stats() = default;
	document_stats(const document_stats&) = default;
	document_stats& operator=(const document_stats&) = default;
	document_stats(document_stats&&) = default;
	document_stats& operator=(document_stats&&) = default;
};

struct document {
	wxString title;
	wxString author;
	document_buffer buffer;
	document_flags flags{document_flags::none};
	std::vector<std::unique_ptr<toc_item>> toc_items;
	mutable document_stats stats;

	document() = default;
	~document() = default;
	document(const document&) = delete;
	document& operator=(const document&) = delete;
	document(document&&) = default;
	document& operator=(document&&) = default;

	[[nodiscard]] inline bool has_flag(document_flags flag) const noexcept {
		return (flags & flag) == flag;
	}

	[[nodiscard]] int next_section_index(size_t position) const noexcept;
	[[nodiscard]] int previous_section_index(size_t position) const noexcept;
	[[nodiscard]] int section_index(size_t position) const noexcept;
	[[nodiscard]] size_t offset_for_section(int section_index) const noexcept;
	[[nodiscard]] int next_page_index(size_t position) const noexcept;
	[[nodiscard]] int previous_page_index(size_t position) const noexcept;
	[[nodiscard]] int page_index(size_t position) const noexcept;
	[[nodiscard]] size_t offset_for_page(int page_index) const noexcept;
	[[nodiscard]] int find_closest_toc_offset(size_t position) const noexcept;
	[[nodiscard]] int next_heading_index(size_t position, int level) const noexcept;
	[[nodiscard]] int previous_heading_index(size_t position, int level) const noexcept;
	[[nodiscard]] size_t offset_for_heading(int heading_index) const noexcept;
	[[nodiscard]] const marker* get_heading_marker(int heading_index) const noexcept;
	void calculate_statistics() const;
};
