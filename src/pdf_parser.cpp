/* pdf_parser.cpp - parses PDF documents to be read in Paperback.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#include "pdf_parser.hpp"
#include "document.hpp"
#include "document_buffer.hpp"
#include "utils.hpp"
#include <exception>
#include <fpdf_doc.h>
#include <fpdf_text.h>
#include <fpdfview.h>
#include <limits>
#include <memory>
#include <sstream>
#include <string>
#include <utility>
#include <vector>
#include <wx/filename.h>
#include <wx/string.h>

pdf_parser::pdf_context::pdf_context() {
	FPDF_InitLibrary();
}

pdf_parser::pdf_context::~pdf_context() {
	if (doc != nullptr) {
		FPDF_CloseDocument(doc);
	}
	FPDF_DestroyLibrary();
}

void pdf_parser::pdf_context::open_document(const wxString& path, const std::string& password) {
	const char* pwd_ptr = password.empty() ? nullptr : password.c_str();
	doc = FPDF_LoadDocument(path.ToUTF8().data(), pwd_ptr);
	if (doc == nullptr) {
		const unsigned long error = FPDF_GetLastError();
		if (error == FPDF_ERR_PASSWORD) {
			throw parser_exception("Password required or incorrect", path, error_severity::error, parser_error_code::password_required);
		}
		throw parser_exception("Failed to open PDF document", path);
	}
	page_count = FPDF_GetPageCount(doc);
}

std::unique_ptr<document> pdf_parser::load(const parser_context& ctx) const {
	try {
		pdf_context pdf_ctx;
		const std::string password = ctx.password.has_value() ? ctx.password.value() : std::string{};
		pdf_ctx.open_document(ctx.file_path, password);
		auto document_ptr = std::make_unique<document>();
		extract_text_content(pdf_ctx, document_ptr->buffer);
		extract_metadata(pdf_ctx, document_ptr->title, document_ptr->author, ctx.file_path);
		extract_toc(pdf_ctx, document_ptr->toc_items, document_ptr->buffer);
		return document_ptr;
	} catch (const parser_exception&) {
		throw;
	} catch (const std::exception& e) {
		throw parser_exception(wxString::FromUTF8(e.what()), ctx.file_path);
	}
}

void pdf_parser::extract_text_content(const pdf_context& ctx, document_buffer& buffer) {
	buffer.clear();
	for (int page_num = 0; page_num < ctx.page_count; ++page_num) {
		buffer.add_page_break(wxString::Format("Page %d", page_num + 1));
		FPDF_PAGE page = FPDF_LoadPage(ctx.doc, page_num);
		if (page == nullptr) {
			continue;
		}
		FPDF_TEXTPAGE text_page = FPDFText_LoadPage(page);
		if (text_page != nullptr) {
			const int char_count = FPDFText_CountChars(text_page);
			if (char_count > 0) {
				std::vector<unsigned short> text_buffer(char_count + 1);
				const int chars_written = FPDFText_GetText(text_page, 0, char_count, text_buffer.data());
				if (chars_written > 0) {
					const wxString page_text(reinterpret_cast<const wchar_t*>(text_buffer.data()));
					const std::string page_text_utf8 = page_text.ToUTF8().data();
					auto processed_lines = process_text_lines(page_text_utf8);
					for (const auto& processed_line : processed_lines) {
						buffer.append_line(wxString::FromUTF8(processed_line));
					}
				}
			}
			FPDFText_ClosePage(text_page);
		}
		FPDF_ClosePage(page);
	}
}

void pdf_parser::extract_metadata(const pdf_context& ctx, wxString& title, wxString& author, const wxString& path) {
	title.Clear();
	author.Clear();
	auto extract_metadata_string = [](FPDF_DOCUMENT doc, const char* tag) -> wxString {
		const unsigned long length = FPDF_GetMetaText(doc, tag, nullptr, 0);
		if (length <= 2) {
			return {};
		}
		std::vector<unsigned short> buffer(length);
		FPDF_GetMetaText(doc, tag, buffer.data(), length);
		return {reinterpret_cast<const wchar_t*>(buffer.data())};
	};
	title = extract_metadata_string(ctx.doc, "Title");
	author = extract_metadata_string(ctx.doc, "Author");
	if (title.IsEmpty()) {
		title = wxFileName(path).GetName();
	}
}

void pdf_parser::extract_toc(const pdf_context& ctx, std::vector<std::unique_ptr<toc_item>>& toc_items, const document_buffer& buffer) const {
	FPDF_BOOKMARK bookmark = FPDFBookmark_GetFirstChild(ctx.doc, nullptr);
	if (bookmark != nullptr) {
		extract_outline_items(bookmark, toc_items, buffer, ctx);
	}
}

void pdf_parser::extract_outline_items(FPDF_BOOKMARK bookmark, std::vector<std::unique_ptr<toc_item>>& toc_items, const document_buffer& buffer, const pdf_context& ctx) const {
	while (bookmark != nullptr) {
		auto item = std::make_unique<toc_item>();
		const unsigned long title_length = FPDFBookmark_GetTitle(bookmark, nullptr, 0);
		if (title_length > 2) {
			std::vector<unsigned short> title_buffer(title_length);
			FPDFBookmark_GetTitle(bookmark, title_buffer.data(), title_length);
			item->name = wxString(reinterpret_cast<const wchar_t*>(title_buffer.data()));
		}
		FPDF_DEST dest = FPDFBookmark_GetDest(ctx.doc, bookmark);
		if (dest != nullptr) {
			const unsigned long page_index = FPDFDest_GetDestPageIndex(ctx.doc, dest);
			if (page_index < static_cast<unsigned long>(buffer.count_markers_by_type(marker_type::page_break))) {
				item->offset = static_cast<int>(buffer.get_marker_position_by_index(marker_type::page_break, page_index));
			} else {
				item->offset = std::numeric_limits<size_t>::max();
			}
		} else {
			item->offset = std::numeric_limits<size_t>::max();
		}
		FPDF_BOOKMARK child = FPDFBookmark_GetFirstChild(ctx.doc, bookmark);
		if (child != nullptr) {
			extract_outline_items(child, item->children, buffer, ctx);
		}
		toc_items.push_back(std::move(item));
		bookmark = FPDFBookmark_GetNextSibling(ctx.doc, bookmark);
	}
}

std::vector<std::string> pdf_parser::process_text_lines(const std::string& raw_text) {
	std::vector<std::string> processed_lines;
	std::istringstream stream(raw_text);
	std::string line;
	while (std::getline(stream, line)) {
		const std::string collapsed = collapse_whitespace(line);
		std::string trimmed = trim_string(collapsed);
		if (!trimmed.empty()) {
			processed_lines.emplace_back(std::move(trimmed));
		}
	}
	return processed_lines;
}
