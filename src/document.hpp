#pragma once

#include <wx/string.h>

class document {
public:
	void set_title(const wxString& title);
	void set_author(const wxString& author);
	void set_text_content(const wxString& text_content);
	const wxString& title() const;
	const wxString& author() const;
	const wxString& text_content() const;

private:
	wxString title_;
	wxString author_;
	wxString text_content_;
};
