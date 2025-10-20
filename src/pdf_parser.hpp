/* pdf_parser.hpp - PDF parser header file.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#pragma once
#include "parser.hpp"
#include <memory>
#ifdef _WIN32
struct HDC__;
typedef HDC__* HDC;
struct HBITMAP__;
typedef HBITMAP__* HBITMAP;
#endif
#include "fpdf_doc.h"
#include "fpdf_text.h"
#include "fpdfview.h"
#include <stdexcept>
#include <string>
#include <vector>

class pdf_parse_error : public std::runtime_error {
public:
	using std::runtime_error::runtime_error;
};

class pdf_parser : public parser {
public:
	pdf_parser() = default;
	~pdf_parser() = default;
	pdf_parser(const pdf_parser&) = delete;
	pdf_parser& operator=(const pdf_parser&) = delete;
	pdf_parser(pdf_parser&&) = delete;
	pdf_parser& operator=(pdf_parser&&) = delete;
	[[nodiscard]] wxString name() const override { return "PDF Documents"; }
	[[nodiscard]] std::span<const wxString> extensions() const override {
		static const wxString exts[] = {"pdf"};
		return exts;
	}
	[[nodiscard]] parser_flags supported_flags() const override { return parser_flags::supports_pages | parser_flags::supports_toc; }
	[[nodiscard]] std::unique_ptr<document> load(const wxString& path) const override;

private:
	struct pdf_context {
		FPDF_DOCUMENT doc{nullptr};
		int page_count{0};

		pdf_context();
		~pdf_context();
		void open_document(const wxString& path);
	};

	static void extract_text_content(const pdf_context& ctx, document_buffer& buffer);
	static void extract_metadata(const pdf_context& ctx, wxString& title, wxString& author, const wxString& path);
	void extract_toc(const pdf_context& ctx, std::vector<std::unique_ptr<toc_item>>& toc_items, const document_buffer& buffer) const;
	void extract_outline_items(FPDF_BOOKMARK bookmark, std::vector<std::unique_ptr<toc_item>>& toc_items, const document_buffer& buffer, const pdf_context& ctx) const;
	static std::vector<std::string> process_text_lines(const std::string& raw_text);
};

REGISTER_PARSER(pdf_parser)
