#pragma once

#include <wx/wx.h>
#include <wx/notebook.h>

enum {
	ID_EXPORT = wxID_HIGHEST + 1,
	ID_FIND_NEXT,
	ID_FIND_PREVIOUS,
	ID_GO_TO_LINE,
	ID_GO_TO_PERCENT,
	ID_TABLE_OF_CONTENTS,
	ID_WORD_COUNT,
	ID_CHECK_FOR_UPDATES,
};

class main_window : public wxFrame {
public:
	main_window();

private:
	wxNotebook* notebook;
	void on_open(wxCommandEvent& event);
	void on_exit(wxCommandEvent& event);
	void on_about(wxCommandEvent& event);
};
