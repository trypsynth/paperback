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
#include "document.hpp"
#include "utils.hpp"
#include <cstdlib>
#include <memory>
#include <string>
#include <vector>
#include <wx/filename.h>
#include <wx/stream.h>
#include <wx/string.h>
#include <wx/translation.h>
#include <wx/wfstream.h>

std::unique_ptr<document> text_parser::load(const parser_context& ctx) const {
	wxFileInputStream file_stream(ctx.file_path);
	if (!file_stream.IsOk()) {
		throw parser_exception(_("Failed to open text file"), ctx.file_path);
	}
	wxBufferedInputStream bs(file_stream);
	const size_t file_size = bs.GetSize();
	if (file_size == 0) {
		throw parser_exception(_("Text file is empty"), ctx.file_path);
	}
	std::vector<char> buffer(file_size);
	bs.Read(buffer.data(), file_size);
	const std::string utf8_content = convert_to_utf8(std::string(buffer.data(), file_size));
	auto doc = std::make_unique<document>();
	doc->title = wxFileName(ctx.file_path).GetName();
	const std::string processed = remove_soft_hyphens(utf8_content);
	doc->buffer.set_content(wxString::FromUTF8(processed));
	return doc;
}
