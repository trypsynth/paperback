#include "document.hpp"

void document::set_title(const wxString& title) {
	title_ = title;
}

void document::set_author(const wxString& author) {
	author_ = author;
}

const wxString& document::title() const {
	return title_;
}

const wxString& document::author() const {
	return author_;
}
