/* document_buffer.hpp - marker-based document content management header file.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#pragma once
#include <string>
#include <vector>
#include <wx/string.h>

enum class marker_type {
	heading_1 = 0,
	heading_2,
	heading_3,
	heading_4,
	heading_5,
	heading_6,
	page_break,
	section_break,
	toc_item,
	link
};

struct marker {
	int pos;
	marker_type type;
	wxString text;
	wxString ref;
	int level;

	marker(int position, marker_type marker_type, const wxString& marker_text = wxString(), const wxString& marker_ref = wxString(), int marker_level = 0) : pos{position}, type{marker_type}, text{marker_text}, ref{marker_ref}, level{marker_level} {
	}

	[[nodiscard]] auto operator<=>(const marker& other) const noexcept {
		return pos <=> other.pos;
	}

	[[nodiscard]] bool operator==(const marker& other) const noexcept {
		return pos == other.pos;
	}
};

class document_buffer {
public:
	document_buffer() = default;
	~document_buffer() = default;
	document_buffer(const document_buffer&) = default;
	document_buffer& operator=(const document_buffer&) = default;
	document_buffer(document_buffer&&) = default;
	document_buffer& operator=(document_buffer&&) = default;
	void set_content(const wxString& text);

	[[nodiscard]] const wxString& str() const noexcept {
		return content;
	}

	void append(const wxString& text);
	void append_line(const wxString& text = wxString());
	void add_heading(int level, const wxString& text);
	void add_page_break(const wxString& label = wxString());
	void add_section_break(const wxString& label = wxString());
	void add_toc_marker(const wxString& text, const wxString& ref = wxString());
	void add_link(int pos, const wxString& text, const wxString& ref);
	void add_marker(int pos, marker_type type, const wxString& text = wxString(), const wxString& ref = wxString(), int level = 0);
	void clear();
	[[nodiscard]] int next_marker_index(int position, marker_type type) const noexcept;
	[[nodiscard]] int previous_marker_index(int position, marker_type type) const noexcept;
	[[nodiscard]] int current_marker_index(size_t position, marker_type type) const noexcept;
	[[nodiscard]] int next_heading_marker_index(int position, int level = -1) const;
	[[nodiscard]] int previous_heading_marker_index(int position, int level = -1) const;
	[[nodiscard]] int marker_position(int marker_index) const noexcept;
	[[nodiscard]] const marker* get_marker(int marker_index) const noexcept;
	[[nodiscard]] std::vector<const marker*> get_markers_by_type(marker_type type) const;
	[[nodiscard]] std::vector<const marker*> get_heading_markers(int level = -1) const;
	[[nodiscard]] size_t count_markers_by_type(marker_type type) const noexcept;
	[[nodiscard]] int get_marker_position_by_index(marker_type type, int index) const noexcept;

private:
	wxString content;
	std::vector<marker> markers;

	[[nodiscard]] static bool is_heading_marker(marker_type type) noexcept;
	[[nodiscard]] static int heading_level_from_type(marker_type type) noexcept;
	void sort_markers();
};
