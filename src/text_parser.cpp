/* text_parser.cpp - handles the reading of plain text files.
 * This is decidedly the smallest and most simple parser, and it is recommended to use this file and its corresponding header as a base whenever adding a new parser.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#include "text_parser.hpp"
#include "utils.hpp"
#include <vector>
#include <wx/filename.h>
#include <wx/strconv.h>
#include <wx/txtstrm.h>
#include <wx/wfstream.h>

std::unique_ptr<document> text_parser::load(const wxString& path) const {
	wxFileInputStream file_stream(path);
	if (!file_stream.IsOk()) return nullptr;
	wxBufferedInputStream bs(file_stream);
	size_t file_size = bs.GetSize();
	if (file_size == 0) return nullptr;
	std::vector<char> buffer(file_size);
	bs.Read(buffer.data(), file_size);
	wxString content;
	content = wxString::FromUTF8(buffer.data(), file_size);
	if (content.empty()) content = wxString(buffer.data(), wxConvLocal, file_size);
	if (content.empty()) {
		wxCSConv conv("windows-1252");
		content = wxString(buffer.data(), conv, file_size);
	}
	if (content.empty()) content = wxString(buffer.data(), wxConvISO8859_1, file_size);
	auto doc = std::make_unique<document>();
	doc->title = wxFileName(path).GetName();
	doc->author = "Unknown";
	std::string utf8_content = content.ToUTF8().data();
	std::string processed = remove_soft_hyphens(utf8_content);
	doc->buffer.set_content(wxString::FromUTF8(processed));
	doc->flags = document_flags::none;
	return doc;
}
