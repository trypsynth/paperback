#include "go_to_line_dialog.hpp"

go_to_line_dialog::go_to_line_dialog(wxWindow* parent, int max_line) :wxDialog{parent, wxID_ANY, "Go to line"} {
	wxBoxSizer* vbox = new wxBoxSizer(wxVERTICAL);
	spinner = new wxSpinCtrl(this, wxID_ANY);
	spinner->SetRange(1, max_line);
	spinner->SetValue(1);
	vbox->Add(new wxStaticText(this, wxID_ANY, "Line number:"), 0, wxALL, 5);
	vbox->Add(spinner, 0, wxALL | wxEXPAND, 5);
	wxBoxSizer* hbox = new wxBoxSizer(wxHORIZONTAL);
	hbox->Add(new wxButton(this, wxID_OK), 0, wxALL, 5);
	hbox->Add(new wxButton(this, wxID_CANCEL), 0, wxALL, 5);
	vbox->Add(hbox, 0, wxALIGN_CENTER);
	SetSizerAndFit(vbox);
}

int go_to_line_dialog::line_number() const {
	return spinner->GetValue();
}
