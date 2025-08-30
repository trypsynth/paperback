/* markdown_parser.cpp - parses markdown documents to be read in Paperback.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#include "markdown_parser.hpp"
#include "html_to_text.hpp"
#include <maddy/parser.h>
#include <memory>
#include <wx/txtstrm.h>
#include <wx/wfstream.h>

std::unique_ptr<document> markdown_parser::load(const wxString& path) const {
	wxFileInputStream file_stream(path);
	if (!file_stream.IsOk()) return nullptr;
	wxBufferedInputStream bs(file_stream);
	wxTextInputStream text_stream(bs);
	wxString content;
	while (!bs.Eof()) content += text_stream.ReadLine() + "\n";
	std::shared_ptr<maddy::Parser> parser = std::make_shared<maddy::Parser>();
	std::istringstream iss(content.ToStdString());
	std::string html = parser->Parse(iss);
	html_to_text converter;
	if (!converter.convert(html)) return nullptr;
	auto doc = std::make_unique<document>();
	doc->text_content = converter.get_text();
	doc->flags = document_flags::supports_toc;
	for (const auto& heading : converter.get_headings()) {
		heading_info info{};
		info.offset = heading.offset;
		info.level = heading.level;
		info.text = wxString::FromUTF8(heading.text);
		doc->heading_offsets.push_back(std::move(info));
	}
	return doc;
}
