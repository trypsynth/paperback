#include "go_to_dialog.hpp"

go_to_dialog::go_to_dialog(wxWindow* parent, wxTextCtrl* text_ctrl) : wxDialog(parent, wxID_ANY, "Go to"), textbox{text_ctrl} {
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
	input_ctrl->Bind(wxEVT_KEY_DOWN, &go_to_dialog::on_key_down, this);
	input_ctrl->Bind(wxEVT_CHAR, &go_to_dialog::on_char, this);
	auto* button_sizer = new wxStdDialogButtonSizer();
	auto* ok_button = new wxButton(this, wxID_OK);
	button_sizer->AddButton(ok_button);
	button_sizer->AddButton(new wxButton(this, wxID_CANCEL));
	ok_button->SetDefault();
	button_sizer->Realize();
	main_sizer->Add(line_sizer, 0, wxALL | wxEXPAND, 5);
	main_sizer->Add(button_sizer, 0, wxALIGN_RIGHT | wxALL, 10);
	SetSizerAndFit(main_sizer);
}

long go_to_dialog::get_position() const {
	wxString input = input_ctrl->GetValue().Trim(true).Trim(false);
	if (input.EndsWith("%")) {
		input.RemoveLast();
		long percent;
		if (input.ToLong(&percent) && percent >= 0 && percent <= 100) {
			long total_chars = textbox->GetLastPosition();
			return (percent * total_chars) / 100;
		}
	} else {
		long line;
		if (input.ToLong(&line) && line >= 1 && line <= textbox->GetNumberOfLines())
			return textbox->XYToPosition(0, line - 1);
	}
	return textbox->GetInsertionPoint();
}

void go_to_dialog::on_key_down(wxKeyEvent& event) {
	int key_code = event.GetKeyCode();
	if (key_code == WXK_UP)
		adjust_line_number(1);
	else if (key_code == WXK_DOWN)
		adjust_line_number(-1);
	else
		event.Skip();
}

void go_to_dialog::on_char(wxKeyEvent& event) {
	int key = event.GetKeyCode();
	if (key < WXK_SPACE || key == WXK_DELETE || key == WXK_BACK || key == WXK_LEFT || key == WXK_RIGHT || key == WXK_TAB) {
		event.Skip();
		return;
	}
	wxString current = input_ctrl->GetValue();
	long from, to;
	input_ctrl->GetSelection(&from, &to);
	wxChar ch = static_cast<wxChar>(key);
	if (wxIsdigit(ch))
		event.Skip();
	else if (ch == '%' && !current.Contains('%'))
		event.Skip(); // allow a single percent sign
	else
		wxBell();
}

void go_to_dialog::adjust_line_number(int delta) {
	wxString current_value = input_ctrl->GetValue().Trim(true).Trim(false);
	if (!current_value.EndsWith("%")) {
		long current_line;
		if (current_value.ToLong(&current_line)) {
			long new_line = current_line + delta;
			long max_line = get_max_line();
			if (new_line < 1)
				new_line = 1;
			else if (new_line > max_line)
				new_line = max_line;
			input_ctrl->SetValue(wxString::Format("%ld", new_line));
			input_ctrl->SetSelection(-1, -1); // Select all text
		}
	}
}

long go_to_dialog::get_max_line() const {
	return textbox->GetNumberOfLines();
}
