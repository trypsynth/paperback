/* document.cpp - document interface implementation.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#include "document.hpp"
#include "document_buffer.hpp"
#include <climits>
#include <cstddef>
#include <cstdlib>
#include <functional>
#include <memory>
#include <utility>
#include <vector>
#include <wx/tokenzr.h>

int document::next_section_index(long position) const noexcept {
	return buffer.next_marker_index(position, marker_type::section_break);
}

int document::previous_section_index(long position) const noexcept {
	return buffer.previous_marker_index(position, marker_type::section_break);
}

int document::section_index(size_t position) const noexcept {
	return buffer.current_marker_index(position, marker_type::section_break);
}

size_t document::offset_for_section(int section_index) const noexcept {
	return buffer.marker_position(section_index);
}

int document::next_page_index(long position) const noexcept {
	return buffer.next_marker_index(position, marker_type::page_break);
}

int document::previous_page_index(long position) const noexcept {
	return buffer.previous_marker_index(position, marker_type::page_break);
}

int document::page_index(size_t position) const noexcept {
	return buffer.current_marker_index(position, marker_type::page_break);
}

size_t document::offset_for_page(int page_index) const noexcept {
	return buffer.marker_position(page_index);
}

size_t document::find_closest_toc_offset(size_t position) const noexcept {
	size_t best_offset = 0;
	size_t best_distance = SIZE_MAX;
	std::function<void(const std::vector<std::unique_ptr<toc_item>>&)> search_items = [&](const std::vector<std::unique_ptr<toc_item>>& items) {
		for (const auto& item : items) {
			const auto off = item->offset;
			if (off <= position) {
				const auto distance = position - off;
				if (distance < best_distance) {
					best_offset = off;
					best_distance = distance;
				}
			}
			if (!item->children.empty()) {
				search_items(item->children);
			}
		}
	};
	search_items(toc_items);
	return best_offset;
}

int document::next_heading_index(long position, int level) const noexcept {
	return buffer.next_heading_marker_index(position, level);
}

int document::previous_heading_index(long position, int level) const noexcept {
	return buffer.previous_heading_marker_index(position, level);
}

size_t document::offset_for_heading(int heading_index) const noexcept {
	const auto& heading_markers = buffer.get_heading_markers();
	if (heading_index < 0 || std::cmp_greater_equal(heading_index, heading_markers.size())) {
		return 0;
	}
	return heading_markers[heading_index]->pos;
}

const marker* document::get_heading_marker(int heading_index) const noexcept {
	const auto& heading_markers = buffer.get_heading_markers();
	if (heading_index < 0 || std::cmp_greater_equal(heading_index, heading_markers.size())) {
		return nullptr;
	}
	return heading_markers[heading_index];
}

void document::calculate_statistics() const {
	const auto& text_content = buffer.str();
	stats.char_count = text_content.Length();
	stats.char_count_no_whitespace = 0;
	for (const auto ch : text_content) {
		if (ch != ' ' && ch != '\t' && ch != '\r' && ch != '\n') {
			++stats.char_count_no_whitespace;
		}
	}
	if (text_content.IsEmpty()) {
		stats.line_count = 0;
	} else {
		stats.line_count = 1;
		for (const auto ch : text_content) {
			if (ch == '\n') {
				++stats.line_count;
			}
		}
	}
	if (text_content.IsEmpty()) {
		stats.word_count = 0;
	} else {
		wxStringTokenizer tokenizer(text_content, " \t\r\n", wxTOKEN_STRTOK);
		stats.word_count = 0;
		while (tokenizer.HasMoreTokens()) {
			tokenizer.GetNextToken();
			++stats.word_count;
		}
	}
}
