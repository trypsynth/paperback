/* pdf_parser.cpp - parses PDF documents to be read in Paperback.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#include "pdf_parser.hpp"
#include "utils.hpp"
#include <sstream>
#include <wx/filename.h>
#include <wx/msgdlg.h>

pdf_parser::pdf_context::pdf_context() {
	FPDF_InitLibrary();
}

pdf_parser::pdf_context::~pdf_context() {
	if (doc) FPDF_CloseDocument(doc);
	FPDF_DestroyLibrary();
}

void pdf_parser::pdf_context::open_document(const wxString& path) {
	doc = FPDF_LoadDocument(path.ToUTF8().data(), nullptr);
	if (!doc) throw pdf_parse_error("Failed to open PDF document");
	page_count = FPDF_GetPageCount(doc);
}

std::unique_ptr<document> pdf_parser::load(const wxString& path) const {
	try {
		pdf_context ctx;
		ctx.open_document(path);
		auto document_ptr = std::make_unique<document>();
		extract_text_content(ctx, document_ptr->buffer);
		extract_metadata(ctx, document_ptr->title, document_ptr->author, path);
		extract_toc(ctx, document_ptr->toc_items, document_ptr->buffer);
		document_ptr->flags = document_flags::supports_pages | document_flags::supports_toc;
		if (!document_ptr->toc_items.empty()) document_ptr->flags |= document_flags::supports_toc;
		return document_ptr;
	} catch (const std::exception& e) {
		wxMessageBox(wxString::FromUTF8(e.what()), "PDF Parse Error", wxICON_ERROR);
		return nullptr;
	}
}

void pdf_parser::extract_text_content(const pdf_context& ctx, document_buffer& buffer) const {
	buffer.clear();
	for (int page_num = 0; page_num < ctx.page_count; ++page_num) {
		buffer.add_page_break(wxString::Format("Page %d", page_num + 1));
		FPDF_PAGE page = FPDF_LoadPage(ctx.doc, page_num);
		if (!page) continue;
		FPDF_TEXTPAGE text_page = FPDFText_LoadPage(page);
		if (text_page) {
			int char_count = FPDFText_CountChars(text_page);
			if (char_count > 0) {
				std::vector<unsigned short> text_buffer(char_count + 1);
				int chars_written = FPDFText_GetText(text_page, 0, char_count, text_buffer.data());
				if (chars_written > 0) {
					wxString page_text(reinterpret_cast<const wchar_t*>(text_buffer.data()));
					std::string page_text_utf8 = page_text.ToUTF8().data();
					auto processed_lines = process_text_lines(page_text_utf8);
					for (size_t i = 0; i < processed_lines.size(); ++i) buffer.append_line(wxString::FromUTF8(processed_lines[i]));
				}
			}
			FPDFText_ClosePage(text_page);
		}
		FPDF_ClosePage(page);
	}
}

void pdf_parser::extract_metadata(const pdf_context& ctx, wxString& title, wxString& author, const wxString& path) const {
	title.Clear();
	author.Clear();
	auto extract_metadata_string = [](FPDF_DOCUMENT doc, const char* tag) -> wxString {
		unsigned long length = FPDF_GetMetaText(doc, tag, nullptr, 0);
		if (length <= 2) return wxString();
		std::vector<unsigned short> buffer(length);
		FPDF_GetMetaText(doc, tag, buffer.data(), length);
		return wxString(reinterpret_cast<const wchar_t*>(buffer.data()));
	};
	title = extract_metadata_string(ctx.doc, "Title");
	author = extract_metadata_string(ctx.doc, "Author");
	if (title.IsEmpty()) title = wxFileName(path).GetName();
	if (author.IsEmpty()) author = "Unknown";
}

void pdf_parser::extract_toc(const pdf_context& ctx, std::vector<std::unique_ptr<toc_item>>& toc_items, const document_buffer& buffer) const {
	FPDF_BOOKMARK bookmark = FPDFBookmark_GetFirstChild(ctx.doc, nullptr);
	if (bookmark) extract_outline_items(bookmark, toc_items, buffer, ctx);
}

void pdf_parser::extract_outline_items(FPDF_BOOKMARK bookmark, std::vector<std::unique_ptr<toc_item>>& toc_items, const document_buffer& buffer, const pdf_context& ctx) const {
	while (bookmark) {
		auto item = std::make_unique<toc_item>();
		unsigned long title_length = FPDFBookmark_GetTitle(bookmark, nullptr, 0);
		if (title_length > 2) {
			std::vector<unsigned short> title_buffer(title_length);
			FPDFBookmark_GetTitle(bookmark, title_buffer.data(), title_length);
			item->name = wxString(reinterpret_cast<const wchar_t*>(title_buffer.data()));
		}
		FPDF_DEST dest = FPDFBookmark_GetDest(ctx.doc, bookmark);
		if (dest) {
			unsigned long page_index = FPDFDest_GetDestPageIndex(ctx.doc, dest);
			if (page_index < static_cast<unsigned long>(buffer.count_markers_by_type(marker_type::page_break)))
				item->offset = static_cast<int>(buffer.get_marker_position_by_index(marker_type::page_break, page_index));
			else
				item->offset = -1;
		} else
			item->offset = -1;
		FPDF_BOOKMARK child = FPDFBookmark_GetFirstChild(ctx.doc, bookmark);
		if (child) extract_outline_items(child, item->children, buffer, ctx);
		toc_items.push_back(std::move(item));
		bookmark = FPDFBookmark_GetNextSibling(ctx.doc, bookmark);
	}
}

std::vector<std::string> pdf_parser::process_text_lines(const std::string& raw_text) const {
	std::vector<std::string> processed_lines;
	std::istringstream stream(raw_text);
	std::string line;
	while (std::getline(stream, line)) {
		std::string collapsed = collapse_whitespace(line);
		std::string trimmed = trim_string(collapsed);
		if (!trimmed.empty()) processed_lines.emplace_back(std::move(trimmed));
	}
	return processed_lines;
}
