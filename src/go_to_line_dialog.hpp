#pragma once

#include <wx/wx.h>
#include <wx/spinctrl.h>

class go_to_line_dialog : public wxDialog {
public:
	go_to_line_dialog(wxWindow* parent, wxTextCtrl* text_ctrl);
	int line_number() const;

private:
	wxSpinCtrl* spinner;
	wxTextCtrl* textbox;
};
