#include "toc_dialog.hpp"
#include <wx/treectrl.h>

toc_dialog::toc_dialog(wxWindow* parent) :wxDialog(parent, wxID_ANY, "Table of Contents") {
	auto* main_sizer = new wxBoxSizer(wxVERTICAL);
	auto* tree = new wxTreeCtrl(this, wxID_ANY);
	auto* button_sizer = new wxStdDialogButtonSizer();
	button_sizer->AddButton(new wxButton(this, wxID_OK));
	button_sizer->AddButton(new wxButton(this, wxID_CANCEL));
	button_sizer->Realize();
	main_sizer->Add(tree, 1, wxEXPAND | wxALL, 10);
	main_sizer->Add(button_sizer, 0, wxALIGN_RIGHT | wxALL, 10);
	SetSizer(main_sizer);
	CentreOnParent();
}
