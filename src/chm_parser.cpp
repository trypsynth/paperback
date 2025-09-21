/* chm_parser.cpp - parser for Compiled HTML Help files.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#include "chm_parser.hpp"
#include "html_to_text.hpp"
#include "utils.hpp"
#include <algorithm>
#include <memory>
#include <sstream>
#include <wx/filename.h>
#include <wx/msgdlg.h>

std::unique_ptr<document> chm_parser::load(const wxString& path) const {
	chmFile* file = nullptr;
	try {
		file = chm_open(path.ToStdString().c_str());
		if (!file) return nullptr;
		chm_context ctx(file);
		enumerate_files(ctx);
		auto document_ptr = std::make_unique<document>();
		document_ptr->buffer.clear();
		parse_html_files(ctx, document_ptr->buffer);
		document_ptr->title = "CHM document";
		document_ptr->author = "Unknown";
		document_ptr->flags = document_flags::supports_toc;
		chm_close(file);
		return document_ptr;
	} catch (const std::exception& e) {
		if (file) chm_close(file);
		wxMessageBox(wxString::FromUTF8(e.what()), "Error", wxICON_ERROR);
		return nullptr;
	} catch (...) {
		if (file) chm_close(file);
		wxMessageBox("Unknown error while parsing CHM file", "Error", wxICON_ERROR);
		return nullptr;
	}
}

void chm_parser::enumerate_files(chm_context& ctx) const {
	chm_enumerate(ctx.file, CHM_ENUMERATE_ALL, file_enumerator, &ctx);
	std::sort(ctx.html_files.begin(), ctx.html_files.end());
}

void chm_parser::parse_html_files(chm_context& ctx, document_buffer& buffer) const {
	for (size_t i = 0; i < ctx.html_files.size(); ++i) {
		const auto& file_path = ctx.html_files[i];
		size_t section_start = buffer.str().length();
		std::string content = read_file_content(ctx.file, file_path);
		if (content.empty()) continue;
		html_to_text converter;
		if (!converter.convert(content)) continue;
		const auto& text = converter.get_text();
		const auto& headings = converter.get_headings();
		buffer.append(wxString::FromUTF8(text));
		for (const auto& heading : headings) {
			marker_type type = static_cast<marker_type>(static_cast<int>(marker_type::heading_1) + heading.level - 1);
			buffer.add_marker(section_start + heading.offset, type, wxString::FromUTF8(heading.text), wxString(), heading.level);
		}
		if (buffer.str().length() > 0 && !buffer.str().EndsWith("\n")) buffer.append("\n");
	}
}

std::string chm_parser::read_file_content(chmFile* file, const std::string& path) const {
	chmUnitInfo ui;
	if (chm_resolve_object(file, path.c_str(), &ui) != CHM_RESOLVE_SUCCESS) return "";
	if (ui.length == 0) return "";
	std::vector<unsigned char> buffer(static_cast<size_t>(ui.length));
	LONGINT64 bytes_read = chm_retrieve_object(file, &ui, buffer.data(), 0, ui.length);
	if (bytes_read != static_cast<LONGINT64>(ui.length)) return "";
	return std::string(buffer.begin(), buffer.end());
}

int chm_parser::file_enumerator(chmFile* h, chmUnitInfo* ui, void* context) {
	auto* ctx = static_cast<chm_context*>(context);
	std::string path(ui->path);
	if (path.find(".htm") != std::string::npos || path.find(".html") != std::string::npos) {
		if (path.find("/#") == std::string::npos && path.find("/$") == std::string::npos) ctx->html_files.push_back(path);
	}
	return CHM_ENUMERATOR_CONTINUE;
}
