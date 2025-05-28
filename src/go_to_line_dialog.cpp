#include "go_to_line_dialog.hpp"

go_to_line_dialog::go_to_line_dialog(wxWindow* parent, wxTextCtrl* text_ctrl) :wxDialog{parent, wxID_ANY, "Go to line"}, textbox{text_ctrl} {
	wxBoxSizer* vbox = new wxBoxSizer(wxVERTICAL);
	spinner = new wxSpinCtrl(this, wxID_ANY);
	spinner->SetRange(1, textbox->GetNumberOfLines());
	long line;
	textbox->PositionToXY(textbox->GetInsertionPoint(), 0, &line);
	line++; // Account for PositionToXY being zero-based.
	spinner->SetValue(line);
	vbox->Add(new wxStaticText(this, wxID_ANY, "Line number:"), 0, wxALL, 5);
	vbox->Add(spinner, 0, wxALL | wxEXPAND, 5);
	wxBoxSizer* hbox = new wxBoxSizer(wxHORIZONTAL);
	auto* ok_btn = new wxButton(this, wxID_OK);
	ok_btn->SetDefault();
	hbox->Add(ok_btn, 0, wxALL, 5);
	hbox->Add(new wxButton(this, wxID_CANCEL), 0, wxALL, 5);
	vbox->Add(hbox, 0, wxALIGN_CENTER);
	SetSizerAndFit(vbox);
}

int go_to_line_dialog::line_number() const {
	return spinner->GetValue();
}
