#pragma once

#include <memory>
#include <vector>
#include <wx/string.h>

struct toc_item {
	wxString name;
	std::vector<std::unique_ptr<toc_item>> children;
	std::weak_ptr<toc_item> parent;
};

class document {
public:
	void set_title(const wxString& title) {title_ = title;}
	void set_author(const wxString& author) {author_ = author;}
	void set_text_content(const wxString& text_content) {text_content_ = text_content;}
	[[nodiscard]] const wxString& title() const {return title_;}
	[[nodiscard]] const wxString& author() const {return author_;}
	[[nodiscard]] const wxString& text_content() const {return text_content_;}

private:
	wxString title_;
	wxString author_;
	wxString text_content_;
};
