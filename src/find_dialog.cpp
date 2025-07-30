#include "find_dialog.hpp"
#include <wx/statbox.h>
#include <wx/sizer.h>

find_dialog::find_dialog(wxWindow* parent) : wxDialog(parent, wxID_ANY, "Find") {
	create_controls();
	Bind(wxEVT_BUTTON, &find_dialog::on_find_next, this, ID_FIND_NEXT_BTN);
	Bind(wxEVT_BUTTON, &find_dialog::on_find_previous, this, ID_FIND_PREVIOUS_BTN);
	Bind(wxEVT_BUTTON, &find_dialog::on_cancel, this, wxID_CANCEL);
	Bind(wxEVT_TEXT_ENTER, &find_dialog::on_text_enter, this, ID_FIND_TEXT_CTRL);
	Bind(wxEVT_CLOSE_WINDOW, &find_dialog::on_close, this);
	SetMinSize(wxSize(400, 200));
	CenterOnParent();
}

void find_dialog::create_controls() {
	auto* main_sizer = new wxBoxSizer(wxVERTICAL);
	auto* find_box = new wxStaticBoxSizer(wxVERTICAL, this, "Find what:");
	find_text_ctrl = new wxTextCtrl(this, ID_FIND_TEXT_CTRL, "", wxDefaultPosition, wxDefaultSize, wxTE_PROCESS_ENTER);
	find_box->Add(find_text_ctrl, 0, wxEXPAND | wxALL, 5);
	main_sizer->Add(find_box, 0, wxEXPAND | wxALL, 10);
	auto* options_box = new wxStaticBoxSizer(wxVERTICAL, this, "Options:");
	match_case_cb = new wxCheckBox(this, wxID_ANY, "Match case");
	whole_word_cb = new wxCheckBox(this, wxID_ANY, "Whole word only");
	regex_cb = new wxCheckBox(this, wxID_ANY, "Regular expressions");
	options_box->Add(match_case_cb, 0, wxALL, 3);
	options_box->Add(whole_word_cb, 0, wxALL, 3);
	options_box->Add(regex_cb, 0, wxALL, 3);
	main_sizer->Add(options_box, 0, wxEXPAND | wxLEFT | wxRIGHT, 10);
	auto* button_sizer = new wxStdDialogButtonSizer();
	find_next_btn = new wxButton(this, ID_FIND_NEXT_BTN, "Find Next");
	find_previous_btn = new wxButton(this, ID_FIND_PREVIOUS_BTN, "Find Previous");
	cancel_btn = new wxButton(this, wxID_CANCEL, "Cancel");
	find_next_btn->SetDefault();
	button_sizer->Add(find_next_btn, 0, wxALL, 5);
	button_sizer->Add(find_previous_btn, 0, wxALL, 5);
	button_sizer->AddButton(cancel_btn);
	button_sizer->Realize();
	main_sizer->Add(button_sizer, 0, wxEXPAND | wxALL, 10);
	SetSizer(main_sizer);
	Layout();
	Fit();
	find_text_ctrl->SetFocus();
}
