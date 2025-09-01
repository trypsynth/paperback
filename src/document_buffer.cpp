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
#include <wx/log.h>

void document_buffer::set_content(const wxString& text) {
	content = text;
}

void document_buffer::append(const wxString& text) {
	content += text;
}

void document_buffer::append_line(const wxString& text) {
	if (!text.IsEmpty()) content += text;
	content += "\n";
}

void document_buffer::add_heading(int level, const wxString& text) {
	size_t pos = content.length();
	marker_type type = static_cast<marker_type>(static_cast<int>(marker_type::heading_1) + level - 1);
	markers.emplace_back(pos, type, text, wxString(), level);
	content += text;
	if (!content.EndsWith("\n")) content += "\n";
}

void document_buffer::add_page_break(const wxString& label) {
	size_t pos = content.length();
	markers.emplace_back(pos, marker_type::page_break, label, wxString(), 0);
}

void document_buffer::add_section_break(const wxString& label) {
	size_t pos = content.length();
	markers.emplace_back(pos, marker_type::section_break, label, wxString(), 0);
}

void document_buffer::add_toc_marker(const wxString& text, const wxString& ref) {
	size_t pos = content.length();
	markers.emplace_back(pos, marker_type::toc_item, text, ref, 0);
}

void document_buffer::add_marker(size_t pos, marker_type type, const wxString& text, const wxString& ref, int level) {
	if (is_heading_marker(type) && level == 0) level = heading_level_from_type(type);
	markers.emplace_back(pos, type, text, ref, level);
	sort_markers();
}

void document_buffer::clear() {
	content.clear();
	markers.clear();
}

int document_buffer::next_marker_index(size_t position, marker_type type) const noexcept {
	for (size_t i = 0; i < markers.size(); ++i)
		if (markers[i].pos > position && markers[i].type == type)
			return static_cast<int>(i);
	return -1;
}

int document_buffer::previous_marker_index(size_t position, marker_type type) const noexcept {
	int current_index = -1;
	for (size_t i = 0; i < markers.size(); ++i) {
		if (markers[i].pos >= position && markers[i].type == type) {
			current_index = static_cast<int>(i);
			break;
		}
	}
	if (current_index >= 0) {
		for (int i = current_index - 1; i >= 0; --i)
			if (markers[i].type == type) return i;
	} else {
		for (int i = static_cast<int>(markers.size()) - 1; i >= 0; --i)
			if (markers[i].pos < position && markers[i].type == type) return i;
	}
	return -1;
}

int document_buffer::current_marker_index(size_t position, marker_type type) const noexcept {
	for (int i = static_cast<int>(markers.size()) - 1; i >= 0; --i)
		if (markers[i].pos <= position && markers[i].type == type)
			return i;
	return -1;
}

int document_buffer::next_heading_marker_index(size_t position, int level) const noexcept {
	auto heading_markers = get_heading_markers();
	for (size_t i = 0; i < heading_markers.size(); ++i)
		if (heading_markers[i]->pos > position)
			if (level == -1 || heading_markers[i]->level == level)
				return static_cast<int>(i);
	return -1;
}

int document_buffer::previous_heading_marker_index(size_t position, int level) const noexcept {
	auto heading_markers = get_heading_markers();
	int current_index = -1;
	for (size_t i = 0; i < heading_markers.size(); ++i) {
		if (heading_markers[i]->pos >= position) {
			current_index = static_cast<int>(i);
			break;
		}
	}
	if (current_index >= 0) {
		for (int i = current_index - 1; i >= 0; --i)
			if (level == -1 || heading_markers[i]->level == level)
				return i;
	} else {
		for (int i = static_cast<int>(heading_markers.size()) - 1; i >= 0; --i)
			if (heading_markers[i]->pos < position)
				if (level == -1 || heading_markers[i]->level == level)
					return i;
	}
	return -1;
}

size_t document_buffer::marker_position(int marker_index) const noexcept {
	if (marker_index < 0 || marker_index >= static_cast<int>(markers.size())) return 0;
	return markers[marker_index].pos;
}

const marker* document_buffer::get_marker(int marker_index) const noexcept {
	if (marker_index < 0 || marker_index >= static_cast<int>(markers.size())) return nullptr;
	return &markers[marker_index];
}

std::vector<const marker*> document_buffer::get_markers_by_type(marker_type type) const {
	std::vector<const marker*> result;
	for (const auto& m : markers)
		if (m.type == type)
			result.push_back(&m);
	return result;
}

std::vector<const marker*> document_buffer::get_heading_markers(int level) const {
	std::vector<const marker*> result;
	for (const auto& m : markers)
		if (is_heading_marker(m.type))
			if (level == -1 || m.level == level)
				result.push_back(&m);
	return result;
}

size_t document_buffer::count_markers_by_type(marker_type type) const noexcept {
	size_t count = 0;
	for (const auto& m : markers) if (m.type == type) count++;
	return count;
}

size_t document_buffer::get_marker_position_by_index(marker_type type, size_t index) const noexcept {
	size_t count = 0;
	for (const auto& m : markers) {
		if (m.type == type) {
			if (count == index) return m.pos;
			count++;
		}
	}
	return 0;
}

bool document_buffer::is_heading_marker(marker_type type) noexcept {
	switch (type) {
		case marker_type::heading_1:
		case marker_type::heading_2:
		case marker_type::heading_3:
		case marker_type::heading_4:
		case marker_type::heading_5:
		case marker_type::heading_6:
			return true;
		default:
			return false;
	}
}

int document_buffer::heading_level_from_type(marker_type type) noexcept {
	switch (type) {
		case marker_type::heading_1: return 1;
		case marker_type::heading_2: return 2;
		case marker_type::heading_3: return 3;
		case marker_type::heading_4: return 4;
		case marker_type::heading_5: return 5;
		case marker_type::heading_6: return 6;
		default: return 0;
	}
}

void document_buffer::sort_markers() {
	std::sort(markers.begin(), markers.end(), [](const marker& a, const marker& b) {
		return a.pos < b.pos;
	});
}

size_t document_buffer::utf8_byte_offset_to_wx_char_offset(const std::string& utf8_text, size_t byte_offset) noexcept {
	if (byte_offset == 0) return 0;
	if (byte_offset >= utf8_text.length()) return wxString::FromUTF8(utf8_text).length();
	while (byte_offset > 0 && (utf8_text[byte_offset] & 0xC0) == 0x80) --byte_offset;
	std::string substr = utf8_text.substr(0, byte_offset);
	return wxString::FromUTF8(substr).length();
}
