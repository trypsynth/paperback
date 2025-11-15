/* html_parser.cpp - HTML document parsing implementation.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#include "html_parser.hpp"
#include "document.hpp"
#include "document_buffer.hpp"
#include "html_to_text.hpp"
#include "utils.hpp"
#include <memory>
#include <wx/filename.h>
#include <wx/stream.h>
#include <wx/string.h>
#include <wx/translation.h>
#include <wx/txtstrm.h>
#include <wx/wfstream.h>

std::unique_ptr<document> html_parser::load(const parser_context& ctx) const {
	wxFileInputStream file_stream(ctx.file_path);
	if (!file_stream.IsOk()) {
		throw parser_exception(_("Failed to open HTML file"), ctx.file_path);
	}
	wxBufferedInputStream bs(file_stream);
	wxTextInputStream text_stream(bs);
	wxString content;
	while (!bs.Eof()) {
		content += text_stream.ReadLine() + "\n";
	}
	html_to_text converter;
	if (!converter.convert(content.utf8_string(), html_source_mode::native_html)) {
		throw parser_exception(_("Failed to convert HTML to text"), ctx.file_path);
	}
	auto doc = std::make_unique<document>();
	const auto& extracted_title = converter.get_title();
	doc->title = extracted_title.empty() ? wxFileName(ctx.file_path).GetName() : wxString::FromUTF8(extracted_title);
	doc->buffer.clear();
	const auto& text = converter.get_text();
	const auto& headings = converter.get_headings();
	const auto& links = converter.get_links();
	const auto& lists = converter.get_lists();
	const auto& list_items = converter.get_list_items();
	doc->buffer.set_content(wxString::FromUTF8(text));
	for (const auto& pair : converter.get_id_positions()) {
		doc->id_positions[pair.first] = pair.second;
	}
	for (const auto& heading : headings) {
		const auto type = static_cast<marker_type>(static_cast<int>(marker_type::heading_1) + heading.level - 1);
		doc->buffer.add_marker(heading.offset, type, wxString::FromUTF8(heading.text), wxString(), heading.level);
	}
	for (const auto& link : links) {
		doc->buffer.add_link(link.offset, wxString::FromUTF8(link.text), wxString::FromUTF8(link.ref));
	}
	for (const auto& list : lists) {
		doc->buffer.add_marker(list.offset, marker_type::list, wxString(), wxString(), list.item_count);
	}
	for (const auto& list_item : list_items) {
		doc->buffer.add_marker(list_item.offset, marker_type::list_item, wxString::FromUTF8(list_item.text), wxString(), list_item.level);
	}
	doc->buffer.finalize_markers();
	doc->toc_items = build_toc_from_headings(doc->buffer);
	return doc;
}
