#include "document.hpp"

void document::set_title(const wxString& title) {
	title_ = title;
}

void document::set_author(const wxString& author) {
	author_ = author;
}

void document::set_text_content(const wxString& text_content) {
	text_content_ = text_content;
}

const wxString& document::title() const {
	return title_;
}

const wxString& document::author() const {
	return author_;
}

const wxString& document::text_content() const {
	return text_content_;
}
