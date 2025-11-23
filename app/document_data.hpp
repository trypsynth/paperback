/* document_data.hpp - plain document data shared with the Rust bridge.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#pragma once
#include "libpaperback/src/bridge.rs.h"
#include <map>
#include <memory>
#include <optional>
#include <string>
#include <vector>
#include <wx/string.h>

enum class marker_type : int {
	heading_1 = 0,
	heading_2,
	heading_3,
	heading_4,
	heading_5,
	heading_6,
	page_break,
	section_break,
	toc_item,
	link,
	list,
	list_item
};

struct marker {
	size_t pos;
	marker_type type;
	wxString text;
	wxString ref;
	int level;
};

struct toc_item {
	wxString name;
	wxString ref;
	std::vector<std::unique_ptr<toc_item>> children;
	size_t offset;

	toc_item() = default;
	~toc_item() = default;
	toc_item(const toc_item&) = delete;
	toc_item& operator=(const toc_item&) = delete;
	toc_item(toc_item&&) = default;
	toc_item& operator=(toc_item&&) = default;
};

struct document_data {
	std::optional<rust::Box<DocumentHandle>> handle;
	wxString title{"Untitled"};
	wxString author{"Unknown"};
	wxString content;
	std::vector<marker> markers;
	std::vector<std::unique_ptr<toc_item>> toc_items;
	std::map<std::string, size_t> id_positions;
	std::vector<std::string> spine_items;
	std::map<std::string, std::string> manifest_items;
	FfiDocumentStats stats;
	std::vector<long> history;
	size_t history_index{0};

	document_data() = default;
	~document_data() = default;
	document_data(const document_data&) = delete;
	document_data& operator=(const document_data&) = delete;
	document_data(document_data&&) = default;
	document_data& operator=(document_data&&) = default;
};

using document = document_data;
