#pragma once

#include <wx/wx.h>

class go_to_dialog : public wxDialog {
public:
	go_to_dialog(wxWindow* parent, wxTextCtrl* text_ctrl);
	long get_position() const;

private:
	wxTextCtrl* textbox = nullptr;
	wxTextCtrl* input_ctrl = nullptr;
};
