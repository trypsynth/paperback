/* document_buffer.cpp - marker-based document content management implementation.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#include "document_buffer.hpp"
#include <algorithm>
#include <cstddef>
#include <utility>
#include <vector>
#include <wx/string.h>

void document_buffer::set_content(const wxString& text) {
	content = text;
}

void document_buffer::append(const wxString& text) {
	content += text;
}

void document_buffer::append_line(const wxString& text) {
	if (!text.IsEmpty()) {
		content += text;
	}
	content += "\n";
}

void document_buffer::add_heading(int level, const wxString& text) {
	const size_t pos = content.length();
	const auto type = static_cast<marker_type>(static_cast<int>(marker_type::heading_1) + level - 1);
	markers.emplace_back(pos, type, text, wxString(), level);
	content += text;
	if (!content.EndsWith("\n")) {
		content += "\n";
	}
}

void document_buffer::add_page_break(const wxString& label) {
	const size_t pos = content.length();
	markers.emplace_back(pos, marker_type::page_break, label, wxString(), 0);
}

void document_buffer::add_section_break(const wxString& label) {
	const size_t pos = content.length();
	markers.emplace_back(pos, marker_type::section_break, label, wxString(), 0);
}

void document_buffer::add_toc_marker(const wxString& text, const wxString& ref) {
	const size_t pos = content.length();
	markers.emplace_back(pos, marker_type::toc_item, text, ref, 0);
}

void document_buffer::add_link(size_t pos, const wxString& text, const wxString& ref) {
	markers.emplace_back(pos, marker_type::link, text, ref, 0);
}

void document_buffer::finalize_markers() {
	sort_markers();
}

void document_buffer::add_marker(size_t pos, marker_type type, const wxString& text, const wxString& ref, int level) {
	if (is_heading_marker(type) && level == 0) {
		level = heading_level_from_type(type);
	}
	markers.emplace_back(pos, type, text, ref, level);
	sort_markers();
}

void document_buffer::clear() {
	content.clear();
	markers.clear();
}

int document_buffer::next_marker_index(long position, marker_type type) const noexcept {
	auto it = std::upper_bound(markers.begin(), markers.end(), position, [](long pos, const marker& m) { return pos < static_cast<long>(m.pos); });
	while (it != markers.end()) {
		if (it->type == type) {
			return static_cast<int>(std::distance(markers.begin(), it));
		}
		++it;
	}
	return -1;
}

int document_buffer::find_first_marker_after(long position, marker_type type) const noexcept {
	for (size_t i = 0; i < markers.size(); ++i) {
		if (static_cast<long>(markers[i].pos) >= position && markers[i].type == type) {
			return static_cast<int>(i);
		}
	}
	return -1;
}

int document_buffer::previous_marker_index(long position, marker_type type) const noexcept {
	auto it = std::lower_bound(markers.begin(), markers.end(), position, [](const marker& m, long pos) { return static_cast<long>(m.pos) < pos; });
	auto rit = std::make_reverse_iterator(it);
	while (rit != markers.rend()) {
		if (rit->type == type && static_cast<long>(rit->pos) < position) {
			return static_cast<int>(std::distance(markers.begin(), rit.base()) - 1);
		}
		++rit;
	}
	return -1;
}

int document_buffer::current_marker_index(size_t position, marker_type type) const noexcept {
	auto it = std::upper_bound(markers.begin(), markers.end(), position, [](size_t pos, const marker& m) { return pos < m.pos; });
	auto rit = std::make_reverse_iterator(it);
	while (rit != markers.rend()) {
		if (rit->type == type) {
			return static_cast<int>(std::distance(markers.begin(), rit.base()) - 1);
		}
		++rit;
	}
	return -1;
}

int document_buffer::next_heading_marker_index(long position, int level) const {
	const auto heading_markers = get_heading_markers();
	auto it = std::upper_bound(heading_markers.begin(), heading_markers.end(), position, [](long pos, const marker* m) { return pos < static_cast<long>(m->pos); });
	while (it != heading_markers.end()) {
		if (level == -1 || (*it)->level == level) {
			return static_cast<int>(std::distance(heading_markers.begin(), it));
		}
		++it;
	}
	return -1;
}

int document_buffer::previous_heading_marker_index(long position, int level) const {
	const auto heading_markers = get_heading_markers();
	auto it = std::lower_bound(heading_markers.begin(), heading_markers.end(), position, [](const marker* m, long pos) { return static_cast<long>(m->pos) < pos; });
	auto rit = std::make_reverse_iterator(it);
	while (rit != heading_markers.rend()) {
		if (static_cast<long>((*rit)->pos) < position && (level == -1 || (*rit)->level == level)) {
			return static_cast<int>(std::distance(heading_markers.begin(), rit.base()) - 1);
		}
		++rit;
	}
	return -1;
}

size_t document_buffer::marker_position(int marker_index) const noexcept {
	if (marker_index < 0 || std::cmp_greater_equal(marker_index, markers.size())) {
		return 0;
	}
	return markers[marker_index].pos;
}

const marker* document_buffer::get_marker(int marker_index) const noexcept {
	if (marker_index < 0 || std::cmp_greater_equal(marker_index, markers.size())) {
		return nullptr;
	}
	return &markers[marker_index];
}

std::vector<const marker*> document_buffer::get_markers_by_type(marker_type type) const {
	std::vector<const marker*> result;
	for (const auto& m : markers) {
		if (m.type == type) {
			result.push_back(&m);
		}
	}
	return result;
}

std::vector<const marker*> document_buffer::get_heading_markers(int level) const {
	std::vector<const marker*> result;
	for (const auto& m : markers) {
		if (is_heading_marker(m.type)) {
			if (level == -1 || m.level == level) {
				result.push_back(&m);
			}
		}
	}
	return result;
}

size_t document_buffer::count_markers_by_type(marker_type type) const noexcept {
	size_t count = 0;
	for (const auto& m : markers) {
		if (m.type == type) {
			count++;
		}
	}
	return count;
}

size_t document_buffer::get_marker_position_by_index(marker_type type, int index) const noexcept {
	int count = 0;
	for (const auto& m : markers) {
		if (m.type == type) {
			if (count == index) {
				return m.pos;
			}
			count++;
		}
	}
	return 0;
}

bool document_buffer::is_heading_marker(marker_type type) noexcept {
	return type >= marker_type::heading_1 && type <= marker_type::heading_6;
}

int document_buffer::heading_level_from_type(marker_type type) noexcept {
	if (!is_heading_marker(type)) {
		return 0;
	}
	return static_cast<int>(type) - static_cast<int>(marker_type::heading_1) + 1;
}

void document_buffer::sort_markers() {
	std::ranges::sort(markers);
}
