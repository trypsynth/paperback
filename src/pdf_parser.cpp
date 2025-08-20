#include "pdf_parser.hpp"
#include "utils.hpp"
#include <sstream>
#include <wx/msgdlg.h>

pdf_parser::pdf_context::pdf_context() {
	ctx = fz_new_context(nullptr, nullptr, FZ_STORE_UNLIMITED);
	if (!ctx) throw pdf_parse_error("Failed to create fitz context");
	fz_register_document_handlers(ctx);
}

pdf_parser::pdf_context::~pdf_context() {
	if (doc) fz_drop_document(ctx, doc);
	if (ctx) fz_drop_context(ctx);
}

void pdf_parser::pdf_context::open_document(const wxString& path) {
	fz_try(ctx) {
		doc = fz_open_document(ctx, path.ToUTF8().data());
		if (!doc) throw pdf_parse_error("Failed to open PDF document");
		page_count = fz_count_pages(ctx, doc);
	}
	fz_catch(ctx) {
		throw pdf_parse_error("Error opening PDF: " + std::string(fz_caught_message(ctx)));
	}
}

std::unique_ptr<document> pdf_parser::load(const wxString& path) const {
	try {
		pdf_context ctx;
		ctx.open_document(path);
		auto document_ptr = std::make_unique<document>();
		extract_metadata(ctx, document_ptr->title, document_ptr->author);
		std::vector<size_t> page_offsets;
		extract_text_content(ctx, document_ptr->text_content, page_offsets);
		document_ptr->page_offsets = page_offsets;
		extract_toc(ctx, document_ptr->toc_items, page_offsets);
		document_ptr->flags = document_flags::supports_pages | document_flags::supports_toc;
		if (!document_ptr->toc_items.empty()) document_ptr->flags |= document_flags::supports_toc;
		return document_ptr;
	} catch (const std::exception& e) {
		wxMessageBox(wxString::FromUTF8(e.what()), "PDF Parse Error", wxICON_ERROR);
		return nullptr;
	}
}

void pdf_parser::extract_metadata(const pdf_context& ctx, wxString& title, wxString& author) const {
	fz_try(ctx.ctx) {
		char buf[256];
		if (fz_lookup_metadata(ctx.ctx, ctx.doc, FZ_META_INFO_TITLE, buf, sizeof(buf)) > 0)
			title = wxString::FromUTF8(buf);
		if (fz_lookup_metadata(ctx.ctx, ctx.doc, FZ_META_INFO_AUTHOR, buf, sizeof(buf)) > 0)
			author = wxString::FromUTF8(buf);
	}
	fz_catch(ctx.ctx) {
	}
}

void pdf_parser::extract_text_content(const pdf_context& ctx, wxString& content, std::vector<size_t>& page_offsets) const {
	fz_try(ctx.ctx) {
		for (int page_num = 0; page_num < ctx.page_count; ++page_num) {
			page_offsets.push_back(content.length());
			fz_page* page = fz_load_page(ctx.ctx, ctx.doc, page_num);
			if (!page) continue;
			fz_stext_page* text_page = fz_new_stext_page_from_page(ctx.ctx, page, nullptr);
			if (text_page) {
				fz_buffer* buffer = fz_new_buffer_from_stext_page(ctx.ctx, text_page);
				if (buffer) {
					unsigned char* data;
					size_t data_size = fz_buffer_extract(ctx.ctx, buffer, &data);
					if (data && data_size > 0) {
						std::string page_text(reinterpret_cast<char*>(data), data_size);
						auto processed_lines = process_text_lines(page_text);
						if (!content.empty() && !processed_lines.empty()) content += "\n";
						for (size_t i = 0; i < processed_lines.size(); ++i) {
							content += wxString::FromUTF8(processed_lines[i]);
							if (i < processed_lines.size() - 1) content += "\n";
						}
					}
					fz_drop_buffer(ctx.ctx, buffer);
				}
				fz_drop_stext_page(ctx.ctx, text_page);
			}
			fz_drop_page(ctx.ctx, page);
		}
	}
	fz_catch(ctx.ctx) {
		throw pdf_parse_error("Error extracting text: " + std::string(fz_caught_message(ctx.ctx)));
	}
}

void pdf_parser::extract_toc(const pdf_context& ctx, std::vector<std::unique_ptr<toc_item>>& toc_items, const std::vector<size_t>& page_offsets) const {
	fz_try(ctx.ctx) {
		fz_outline* outline = fz_load_outline(ctx.ctx, ctx.doc);
		if (outline) {
			extract_outline_items(outline, toc_items, page_offsets, ctx);
			fz_drop_outline(ctx.ctx, outline);
		}
	}
	fz_catch(ctx.ctx) {
	}
}

void pdf_parser::extract_outline_items(fz_outline* outline, std::vector<std::unique_ptr<toc_item>>& toc_items, const std::vector<size_t>& page_offsets, const pdf_context& ctx) const {
	while (outline) {
		auto item = std::make_unique<toc_item>();
		if (outline->title) item->name = wxString::FromUTF8(outline->title);
		if (outline->page.page >= 0 && outline->page.page < static_cast<int>(page_offsets.size()))
			item->offset = static_cast<int>(page_offsets[outline->page.page]);
		else
			item->offset = -1;
		if (outline->uri) item->ref = wxString::FromUTF8(outline->uri);
		if (outline->down) extract_outline_items(outline->down, item->children, page_offsets, ctx);
		toc_items.push_back(std::move(item));
		outline = outline->next;
	}
}

std::vector<std::string> pdf_parser::process_text_lines(const std::string& raw_text) const {
	std::vector<std::string> processed_lines;
	std::istringstream stream(raw_text);
	std::string line;
	
	while (std::getline(stream, line)) {
		std::string collapsed = collapse_whitespace(line);
		std::string trimmed = trim_string(collapsed);
		if (!trimmed.empty()) {
			processed_lines.emplace_back(std::move(trimmed));
		}
	}
	
	return processed_lines;
}
