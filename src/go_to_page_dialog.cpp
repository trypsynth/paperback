#include "go_to_page_dialog.hpp"

go_to_page_dialog::go_to_page_dialog(wxWindow* parent, document* doc, int current_page) : wxDialog(parent, wxID_ANY, "Go to page"), doc_{doc} {
	auto* main_sizer = new wxBoxSizer(wxVERTICAL);
	auto* page_sizer = new wxBoxSizer(wxHORIZONTAL);
	auto* label = new wxStaticText(this, wxID_ANY, "Go to page:");
	input_ctrl = new wxTextCtrl(this, wxID_ANY);
	page_sizer->Add(label, 0, wxALIGN_CENTER_VERTICAL | wxRIGHT, 5);
	page_sizer->Add(input_ctrl, 1, wxEXPAND);
	input_ctrl->SetValue(wxString::Format("%d", current_page));
	input_ctrl->SetSelection(-1, -1);
	input_ctrl->Bind(wxEVT_KEY_DOWN, &go_to_page_dialog::on_key_down, this);
	input_ctrl->Bind(wxEVT_CHAR, &go_to_page_dialog::on_char, this);
	auto* button_sizer = new wxStdDialogButtonSizer();
	auto* ok_button = new wxButton(this, wxID_OK);
	button_sizer->AddButton(ok_button);
	button_sizer->AddButton(new wxButton(this, wxID_CANCEL));
	ok_button->SetDefault();
	button_sizer->Realize();
	main_sizer->Add(page_sizer, 0, wxALL | wxEXPAND, 5);
	main_sizer->Add(button_sizer, 0, wxALIGN_RIGHT | wxALL, 10);
	SetSizerAndFit(main_sizer);
}

int go_to_page_dialog::get_page_number() const {
	wxString input = input_ctrl->GetValue().Trim(true).Trim(false);
	long page;
	if (input.ToLong(&page) && page >= 1 && page <= get_max_page())
		return static_cast<int>(page);
	return 1;
}

void go_to_page_dialog::on_key_down(wxKeyEvent& event) {
	int key_code = event.GetKeyCode();
	if (key_code == WXK_UP)
		adjust_page_number(1);
	else if (key_code == WXK_DOWN)
		adjust_page_number(-1);
	else
		event.Skip();
}

void go_to_page_dialog::on_char(wxKeyEvent& event) {
	int key = event.GetKeyCode();
	if (key < WXK_SPACE || key == WXK_DELETE || key == WXK_BACK || key == WXK_LEFT || key == WXK_RIGHT || key == WXK_TAB) {
		event.Skip();
		return;
	}
	wxChar ch = static_cast<wxChar>(key);
	if (wxIsdigit(ch))
		event.Skip();
	else
		wxBell();
}

void go_to_page_dialog::adjust_page_number(int delta) {
	wxString current_value = input_ctrl->GetValue().Trim(true).Trim(false);
	long current_page;
	if (current_value.ToLong(&current_page)) {
		long new_page = current_page + delta;
		long max_page = get_max_page();
		if (new_page < 1) new_page = 1;
		else if (new_page > max_page) new_page = max_page;
		input_ctrl->SetValue(wxString::Format("%ld", new_page));
		input_ctrl->SetSelection(-1, -1);
	}
}

int go_to_page_dialog::get_max_page() const {
	if (!doc_ || !doc_->has_flag(document_flags::supports_pages)) return 1;
	return static_cast<int>(doc_->page_offsets.size());
}
