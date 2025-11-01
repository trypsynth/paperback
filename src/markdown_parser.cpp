/* markdown_parser.cpp - parses markdown documents to be read in Paperback.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#include "markdown_parser.hpp"
#include "document.hpp"
#include "document_buffer.hpp"
#include "html_to_text.hpp"
#include "utils.hpp"
#include <maddy/parser.h>
#include <memory>
#include <sstream>
#include <string>
#include <vector>
#include <wx/filename.h>
#include <wx/stream.h>
#include <wx/string.h>
#include <wx/translation.h>
#include <wx/wfstream.h>

std::unique_ptr<document> markdown_parser::load(const wxString& path) const {
	wxFileInputStream file_stream(path);
	if (!file_stream.IsOk()) {
		throw parser_exception(_("Failed to open Markdown file"), path);
	}
	wxBufferedInputStream bs(file_stream);
	const size_t file_size = bs.GetSize();
	if (file_size == 0) {
		throw parser_exception(_("Markdown file is empty"), path);
	}
	std::vector<char> buffer(file_size);
	bs.Read(buffer.data(), file_size);
	std::string markdown_content(buffer.data(), file_size);
	markdown_content = preprocess_markdown(markdown_content);
	const std::shared_ptr<maddy::Parser> parser = std::make_shared<maddy::Parser>();
	std::istringstream iss(markdown_content);
	const std::string html = parser->Parse(iss);
	html_to_text converter;
	if (!converter.convert(html, html_source_mode::markdown)) {
		throw parser_exception(_("Failed to convert Markdown HTML to text"), path);
	}
	auto doc = std::make_unique<document>();
	doc->title = wxFileName(path).GetName();
	doc->buffer.clear();
	const auto& text = converter.get_text();
	const auto& headings = converter.get_headings();
	const auto& links = converter.get_links();
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
	doc->buffer.finalize_markers();
	doc->toc_items = build_toc_from_headings(doc->buffer);
	return doc;
}

// Maddy expects Markdown formatted according to CommonMark, so we try to hack that together by adding blank lines before headings and lists when missing
std::string markdown_parser::preprocess_markdown(const std::string& input) {
	std::istringstream iss(input);
	std::ostringstream oss;
	std::string line;
	std::string prev_line;
	bool first_line = true;
	bool prev_was_list = false;
	while (std::getline(iss, line)) {
		if (!line.empty() && line.back() == '\r') {
			line.pop_back();
		}
		const bool is_heading = !line.empty() && (line[0] == '#');
		// Strip custom ID syntax from headings
		if (is_heading) {
			const auto pos = line.rfind(" {#");
			if (pos != std::string::npos && line.back() == '}') {
				line.erase(pos);
			}
		}
		const bool is_list = !line.empty() && ((line[0] >= '0' && line[0] <= '9' && line.length() > 1 && line[1] == '.') || line[0] == '-' || line[0] == '*' || line[0] == '+');
		// Add blank line before headings if previous line wasn't blank
		if (!first_line && is_heading && !prev_line.empty()) {
			oss << "\n";
		}
		// Add blank line before first list item if previous wasn't list or blank
		if (!first_line && is_list && !prev_was_list && !prev_line.empty()) {
			oss << "\n";
		}
		oss << line << "\n";
		prev_line = line;
		prev_was_list = is_list;
		first_line = false;
	}
	return oss.str();
}
