/* html_parser.cpp - HTML document parsing implementation.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#include "html_parser.hpp"
#include "html_to_text.hpp"
#include <wx/filename.h>
#include <wx/txtstrm.h>
#include <wx/wfstream.h>

std::unique_ptr<document> html_parser::load(const wxString& path) const {
	wxFileInputStream file_stream(path);
	if (!file_stream.IsOk()) return nullptr;
	wxBufferedInputStream bs(file_stream);
	wxTextInputStream text_stream(bs);
	wxString content;
	while (!bs.Eof()) content += text_stream.ReadLine() + "\n";
	html_to_text converter;
	if (!converter.convert(content.utf8_string())) return nullptr;
	auto doc = std::make_unique<document>();
	doc->title = wxFileName(path).GetName();
	doc->buffer.clear();
	doc->flags = document_flags::supports_toc;
	const auto& text = converter.get_text();
	const auto& headings = converter.get_headings();
	doc->buffer.set_content(wxString::FromUTF8(text));
	for (const auto& heading : headings) {
		marker_type type = static_cast<marker_type>(static_cast<int>(marker_type::heading_1) + heading.level - 1);
		doc->buffer.add_marker(heading.offset, type, wxString::FromUTF8(heading.text), wxString(), heading.level);
	}
	return doc;
}
