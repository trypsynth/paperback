#include "go_to_line_dialog.hpp"

go_to_line_dialog::go_to_line_dialog(wxWindow* parent, wxTextCtrl* text_ctrl) :wxDialog{parent, wxID_ANY, "Go to line"}, textbox{text_ctrl} {
	wxBoxSizer* main_sizer = new wxBoxSizer(wxVERTICAL);
	wxBoxSizer* line_sizer = new wxBoxSizer(wxHORIZONTAL);
	auto* label = new wxStaticText(this, wxID_ANY, "Line number:");
	spinner = new wxSpinCtrl(this, wxID_ANY);
	line_sizer->Add(label, 0, wxALIGN_CENTER_VERTICAL | wxRIGHT, 5);
	line_sizer->Add(spinner, 1, wxEXPAND);
	spinner->SetRange(1, textbox->GetNumberOfLines());
	long line;
	textbox->PositionToXY(textbox->GetInsertionPoint(), 0, &line);
	line++; // Account for PositionToXY being zero-based.
	spinner->SetValue(line);
	spinner->SetSelection(0, -1);
	main_sizer->Add(line_sizer, 0, wxALL | wxEXPAND, 5);
	wxBoxSizer* button_sizer = new wxBoxSizer(wxHORIZONTAL);
	auto* ok_btn = new wxButton(this, wxID_OK);
	ok_btn->SetDefault();
	button_sizer->Add(ok_btn, 0, wxALL, 5);
	button_sizer->Add(new wxButton(this, wxID_CANCEL), 0, wxALL, 5);
	main_sizer->Add(button_sizer, 0, wxALIGN_CENTER);
	SetSizerAndFit(main_sizer);
}

int go_to_line_dialog::line_number() const {
	return spinner->GetValue();
}
