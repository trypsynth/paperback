#include "toc_dialog.hpp"
#include <wx/statline.h>
#include <wx/treectrl.h>

toc_dialog::toc_dialog(wxWindow* parent) :wxDialog{parent, wxID_ANY, "Table of Contents"} {
	wxTreeCtrl* tree = new wxTreeCtrl(this, wxID_ANY);
	wxStdDialogButtonSizer* button_sizer = new wxStdDialogButtonSizer();
	button_sizer->AddButton(new wxButton(this, wxID_OK));
	button_sizer->AddButton(new wxButton(this, wxID_CANCEL));
	button_sizer->Realize();
	wxBoxSizer* main_sizer = new wxBoxSizer(wxVERTICAL);
	main_sizer->Add(tree, 1, wxEXPAND | wxALL, 10);
	main_sizer->Add(new wxStaticLine(this), 0, wxEXPAND | wxLEFT | wxRIGHT, 10);
	main_sizer->Add(button_sizer, 0, wxALIGN_RIGHT | wxALL, 10);
	SetSizer(main_sizer);
	CentreOnParent();
}
