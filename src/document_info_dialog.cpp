#include "document_info_dialog.hpp"
#include "utils.hpp"

document_info_dialog::document_info_dialog(wxWindow* parent, const document* doc) : wxDialog(parent, wxID_ANY, "Document Info") {
	auto* main_sizer = new wxBoxSizer(wxVERTICAL);
	info_text_ctrl = new wxTextCtrl(this, wxID_ANY, wxEmptyString, wxDefaultPosition, wxSize(600, 400), wxTE_MULTILINE | wxTE_READONLY);
	wxString info_text;
	info_text << "Title: " << doc->title << "\n";
	info_text << "Author: " << doc->author << "\n";
	int word_count = get_word_count(doc->text_content);
	int line_count = get_line_count(doc->text_content);
	int char_count = doc->text_content.Length();
	int char_count_no_whitespace = get_char_count_no_whitespace(doc->text_content);
	info_text << "Total number of characters: " << char_count << "\n";
	info_text << "Total number of characters (excluding whitespace): " << char_count_no_whitespace << "\n";
	info_text << "Total number of words: " << word_count << "\n";
	info_text << "Total number of lines: " << line_count << "\n";
	info_text_ctrl->SetValue(info_text);
	main_sizer->Add(info_text_ctrl, 1, wxEXPAND | wxALL, 10);
	auto* button_sizer = new wxStdDialogButtonSizer();
	button_sizer->AddButton(new wxButton(this, wxID_OK));
	button_sizer->Realize();
	main_sizer->Add(button_sizer, 0, wxALIGN_RIGHT | wxALL, 10);
	SetSizerAndFit(main_sizer);
	SetMinSize(wxSize(350, 250));
	CentreOnParent();
}
