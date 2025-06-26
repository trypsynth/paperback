#include "go_to_dialog.hpp"

go_to_dialog::go_to_dialog(wxWindow* parent, wxTextCtrl* text_ctrl) : wxDialog{ parent, wxID_ANY, "Go to" }, textbox{ text_ctrl } {
	auto* main_sizer = new wxBoxSizer(wxVERTICAL);
	auto* line_sizer = new wxBoxSizer(wxHORIZONTAL);
	auto* label = new wxStaticText(this, wxID_ANY, "Go to:");
	input_ctrl = new wxTextCtrl(this, wxID_ANY);
	line_sizer->Add(label, 0, wxALIGN_CENTER_VERTICAL | wxRIGHT, 5);
	line_sizer->Add(input_ctrl, 1, wxEXPAND);
	long line;
	textbox->PositionToXY(textbox->GetInsertionPoint(), 0, &line);
	input_ctrl->SetValue(wxString::Format("%d", line + 1));
	input_ctrl->SetSelection(-1, -1);
	auto* button_sizer = new wxStdDialogButtonSizer();
	button_sizer->AddButton(new wxButton(this, wxID_OK));
	button_sizer->AddButton(new wxButton(this, wxID_CANCEL));
	button_sizer->Realize();
	main_sizer->Add(line_sizer, 0, wxALL | wxEXPAND, 5);
	main_sizer->Add(button_sizer, 0, wxALIGN_RIGHT | wxALL, 10);
	SetSizerAndFit(main_sizer);
}

int go_to_dialog::line_number() const {
	wxString input = input_ctrl->GetValue().Trim(true).Trim(false);
	long cur_line;
	input_ctrl->PositionToXY(input_ctrl->GetInsertionPoint(), 0, &cur_line);
	if (input.EndsWith("%")) {
		input.RemoveLast();
		long percent;
		if (input.ToLong(&percent) && percent >= 0 && percent <= 100) {
			int total_lines = textbox->GetNumberOfLines();
			int line = (percent * total_lines) / 100;
			return std::max(1, line);
		}
	} else {
		long line;
		if (input.ToLong(&line) && line >= 1 && line <= textbox->GetNumberOfLines()) return line;
	}
	return cur_line;
}
