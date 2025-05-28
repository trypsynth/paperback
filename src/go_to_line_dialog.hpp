#pragma once

#include <wx/wx.h>
#include <wx/spinctrl.h>

class go_to_line_dialog : public wxDialog {
public:
	go_to_line_dialog(wxWindow* parent, int max_line);
	int line_number() const;

private:
	wxSpinCtrl* spinner;
};
