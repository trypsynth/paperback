#include "find_dialog.hpp"
#include "constants.hpp"

find_dialog::find_dialog(wxWindow* parent) : wxDialog(parent, wxID_ANY, "Find") {
	create_controls();
	bind_events();
	find_what_combo->SetFocus();
	Fit();
	CenterOnParent();
}

void find_dialog::create_controls() {
	auto* const main_sizer = new wxBoxSizer(wxVERTICAL);
	auto* const find_sizer = new wxBoxSizer(wxHORIZONTAL);
	auto* const find_label = new wxStaticText(this, wxID_ANY, "Find &what:");
	find_what_combo = new wxComboBox(this, wxID_ANY, "", wxDefaultPosition, wxSize(250, -1), 0, nullptr, wxTE_PROCESS_ENTER);
	find_sizer->Add(find_label, 0, wxALIGN_CENTER_VERTICAL | wxRIGHT, 10);
	find_sizer->Add(find_what_combo, 1, wxEXPAND);
	auto* const options_box = new wxStaticBoxSizer(wxVERTICAL, this, "Options");
	match_case_check = new wxCheckBox(this, wxID_ANY, "&Match case");
	match_whole_word_check = new wxCheckBox(this, wxID_ANY, "Match &whole word");
	use_regex_check = new wxCheckBox(this, wxID_ANY, "Use &regular expressions");
	options_box->Add(match_case_check, 0, wxALL, 2);
	options_box->Add(match_whole_word_check, 0, wxALL, 2);
	options_box->Add(use_regex_check, 0, wxALL, 2);
	auto* const button_sizer = new wxBoxSizer(wxHORIZONTAL);
	find_previous_btn = new wxButton(this, wxID_ANY, "Find &Previous");
	find_next_btn = new wxButton(this, wxID_ANY, "Find &Next");
	cancel_btn = new wxButton(this, wxID_CANCEL, "Cancel");
	button_sizer->Add(find_previous_btn, 0, wxRIGHT, 5);
	button_sizer->Add(find_next_btn, 0, wxRIGHT, 5);
	button_sizer->AddStretchSpacer();
	button_sizer->Add(cancel_btn, 0);
	find_next_btn->SetDefault();
	main_sizer->Add(find_sizer, 0, wxEXPAND | wxALL, 10);
	main_sizer->Add(options_box, 0, wxEXPAND | wxLEFT | wxRIGHT | wxBOTTOM, 10);
	main_sizer->Add(button_sizer, 0, wxEXPAND | wxLEFT | wxRIGHT | wxBOTTOM, 10);
	SetSizer(main_sizer);
}

void find_dialog::bind_events() {
	find_previous_btn->Bind(wxEVT_BUTTON, &find_dialog::on_find_previous, this);
	find_next_btn->Bind(wxEVT_BUTTON, &find_dialog::on_find_next, this);
	cancel_btn->Bind(wxEVT_BUTTON, &find_dialog::on_cancel, this);
	find_what_combo->Bind(wxEVT_TEXT_ENTER, &find_dialog::on_find_text_enter, this);
	Bind(wxEVT_CLOSE_WINDOW, &find_dialog::on_close, this);
}

wxString find_dialog::get_find_text() const {
	return find_what_combo->GetValue();
}

bool find_dialog::get_match_case() const noexcept {
	return match_case_check->GetValue();
}

bool find_dialog::get_match_whole_word() const noexcept {
	return match_whole_word_check->GetValue();
}

bool find_dialog::get_use_regex() const noexcept {
	return use_regex_check->GetValue();
}

void find_dialog::set_find_text(const wxString& text) {
	find_what_combo->SetValue(text);
	find_what_combo->SetSelection(-1, -1); // Select all text
}

void find_dialog::add_to_history(const wxString& text) {
	if (text.IsEmpty()) return;
	const int existing = find_what_combo->FindString(text);
	if (existing != wxNOT_FOUND)
		find_what_combo->Delete(existing);
	find_what_combo->Insert(text, 0);
	while (find_what_combo->GetCount() > 10)
		find_what_combo->Delete(find_what_combo->GetCount() - 1);
	find_what_combo->SetValue(text);
}

void find_dialog::focus_find_text() {
	find_what_combo->SetFocus();
	find_what_combo->SetSelection(-1, -1); // Select all text
}

void find_dialog::on_find_previous(wxCommandEvent& event) {
	const wxString text = get_find_text();
	if (!text.IsEmpty()) {
		add_to_history(text);
		wxCommandEvent find_event(wxEVT_COMMAND_MENU_SELECTED, ID_FIND_PREVIOUS);
		wxPostEvent(GetParent(), find_event);
	}
}

void find_dialog::on_find_next(wxCommandEvent& event) {
	const wxString text = get_find_text();
	if (!text.IsEmpty()) {
		add_to_history(text);
		wxCommandEvent find_event(wxEVT_COMMAND_MENU_SELECTED, ID_FIND_NEXT);
		wxPostEvent(GetParent(), find_event);
	}
}

void find_dialog::on_cancel(wxCommandEvent& event) {
	Hide();
}

void find_dialog::on_find_text_enter(wxCommandEvent& event) {
	on_find_next(event);
}

void find_dialog::on_close(wxCloseEvent& event) {
	Hide();
}
