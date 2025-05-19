#pragma once

#include <wx/string.h>

class document {
public:
	void set_title(const wxString& title);
	void set_author(const wxString& author);
	const wxString& title() const;
	const wxString& author() const;

private:
	wxString title_;
	wxString author_;
};
