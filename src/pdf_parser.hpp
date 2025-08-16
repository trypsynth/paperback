#pragma once
#include "parser.hpp"
#include <memory>
#include <mupdf/fitz.h>
#include <stdexcept>
#include <string>
#include <vector>

class pdf_parse_error : public std::runtime_error {
public:
	using std::runtime_error::runtime_error;
};

class pdf_parser : public parser {
public:
	wxString name() const override { return "PDF Documents"; }
	std::span<const wxString> extensions() const override {
		static const wxString exts[] = {"pdf"};
		return exts;
	}
	std::unique_ptr<document> load(const wxString& path) const override;

private:
	struct pdf_context {
		fz_context* ctx{nullptr};
		fz_document* doc{nullptr};
		int page_count{0};

		pdf_context();
		~pdf_context();
		void open_document(const wxString& path);
	};

	void extract_text_content(const pdf_context& ctx, wxString& content, std::vector<size_t>& page_offsets) const;
	void extract_metadata(const pdf_context& ctx, wxString& title, wxString& author) const;
	void extract_toc(const pdf_context& ctx, std::vector<std::unique_ptr<toc_item>>& toc_items, const std::vector<size_t>& page_offsets) const;
	void extract_outline_items(fz_outline* outline, std::vector<std::unique_ptr<toc_item>>& toc_items, const std::vector<size_t>& page_offsets, const pdf_context& ctx) const;
	std::vector<std::string> process_text_lines(const std::string& raw_text) const;
};

REGISTER_PARSER(pdf_parser)
