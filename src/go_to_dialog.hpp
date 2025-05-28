#pragma once

#include <wx/wx.h>

class go_to_dialog : public wxDialog {
public:
	go_to_dialog(wxWindow* parent, wxTextCtrl* text_ctrl);
	int line_number() const;

private:
	wxTextCtrl* textbox;
	wxTextCtrl* input_ctrl;
};
