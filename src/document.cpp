/* document.cpp - document interface implementation.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#include "document.hpp"
#include <wx/tokenzr.h>
#include <functional>
#include <climits>

int document::next_section_index(size_t position) const noexcept {
	for (size_t i = 0; i < section_offsets.size(); ++i)
		if (section_offsets[i] > position)
			return static_cast<int>(i);
	return -1;
}

int document::previous_section_index(size_t position) const noexcept {
	for (int i = static_cast<int>(section_offsets.size()) - 1; i >= 0; --i)
		if (section_offsets[i] < position)
			return i;
	return -1;
}

int document::section_index(size_t position) const noexcept {
	for (int i = static_cast<int>(section_offsets.size()) - 1; i >= 0; --i)
		if (position >= section_offsets[i])
			return i;
	return -1;
}

size_t document::offset_for_section(int section_index) const noexcept {
	if (section_index < 0 || section_index >= static_cast<int>(section_offsets.size())) return 0;
	return section_offsets[section_index];
}

int document::next_page_index(size_t position) const noexcept {
	for (size_t i = 0; i < page_offsets.size(); ++i)
		if (page_offsets[i] > position)
			return static_cast<int>(i);
	return -1;
}

int document::previous_page_index(size_t position) const noexcept {
	for (int i = static_cast<int>(page_offsets.size()) - 1; i >= 0; --i)
		if (page_offsets[i] < position)
			return i;
	return -1;
}

int document::page_index(size_t position) const noexcept {
	for (int i = static_cast<int>(page_offsets.size()) - 1; i >= 0; --i)
		if (position >= page_offsets[i])
			return i;
	return -1;
}

size_t document::offset_for_page(int page_index) const noexcept {
	if (page_index < 0 || page_index >= static_cast<int>(page_offsets.size())) return 0;
	return page_offsets[page_index];
}

int document::find_closest_toc_offset(size_t position) const noexcept {
	int best_offset = -1;
	int best_distance = INT_MAX;
	std::function<void(const std::vector<std::unique_ptr<toc_item>>&)> search_items = [&](const std::vector<std::unique_ptr<toc_item>>& items) {
		for (const auto& item : items) {
			if (item->offset >= 0) {
				int distance = std::abs(static_cast<int>(position) - item->offset);
				if (item->offset <= static_cast<int>(position) && distance < best_distance) {
					best_offset = item->offset;
					best_distance = distance;
				}
			}
			if (!item->children.empty()) search_items(item->children);
		}
	};
	search_items(toc_items);
	return best_offset;
}

void document::calculate_statistics() const {
	stats.char_count = text_content.Length();
	stats.char_count_no_whitespace = 0;
	for (const auto ch : text_content)
		if (ch != ' ' && ch != '\t' && ch != '\r' && ch != '\n') ++stats.char_count_no_whitespace;
	if (text_content.IsEmpty())
		stats.line_count = 0;
	else {
		stats.line_count = 1;
		for (const auto ch : text_content)
			if (ch == '\n') ++stats.line_count;
	}
	if (text_content.IsEmpty())
		stats.word_count = 0;
	else {
		wxStringTokenizer tokenizer(text_content, " \t\r\n", wxTOKEN_STRTOK);
		stats.word_count = 0;
		while (tokenizer.HasMoreTokens()) {
			tokenizer.GetNextToken();
			++stats.word_count;
		}
	}
}
