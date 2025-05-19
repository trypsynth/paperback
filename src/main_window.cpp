#include "constants.hpp"
#include "main_window.hpp"
#include "parser_registry.hpp"
#include <wx/filename.h>
#include <wx/notebook.h>

main_window::main_window() : wxFrame(nullptr, wxID_ANY, APP_NAME) {
	wxPanel* panel = new wxPanel(this);
	wxNotebook* notebook = new wxNotebook(panel, wxID_ANY);
	wxBoxSizer* sizer = new wxBoxSizer(wxVERTICAL);
	sizer->Add(notebook, 1, wxEXPAND | wxALL, 10);
	panel->SetSizer(sizer);
	wxMenuBar* menu_bar = new wxMenuBar();
	wxMenu* file_menu = new wxMenu();
	file_menu->Append(wxID_OPEN);
	file_menu->Append(wxID_CLOSE);
	file_menu->Append(wxID_CLOSE_ALL, "Close &All\tCtrl+Shift+W");
	file_menu->AppendSeparator();
	file_menu->Append(ID_EXPORT, "&Export...\tCtrl+E");
	file_menu->AppendSeparator();
	file_menu->Append(wxID_EXIT, "E&xit");
	wxMenu* go_menu = new wxMenu();
	go_menu->Append(wxID_FIND);
	go_menu->Append(ID_FIND_NEXT, "Find Ne&xt\tF3");
	go_menu->Append(ID_FIND_PREVIOUS, "Find P&revious\tShift+F3");
	go_menu->AppendSeparator();
	go_menu->Append(ID_GO_TO_LINE, "&Go to Line...\tCtrl+G");
	go_menu->Append(ID_GO_TO_PERCENT, "Go to &Percent...\tCtrl+Shift+G");
	go_menu->AppendSeparator();
	go_menu->Append(ID_TABLE_OF_CONTENTS, "Table of contents\tCtrl+T");
	wxMenu* tools_menu = new wxMenu();
	tools_menu->Append(ID_WORD_COUNT, "&Word count\tAlt+W");
	wxMenu* help_menu = new wxMenu();
	help_menu->Append(wxID_ABOUT, "About " + APP_NAME + "\tCtrl+F1");
	help_menu->Append(wxID_HELP, "&Help\tF1");
	help_menu->AppendSeparator();
	help_menu->Append(ID_CHECK_FOR_UPDATES, "&Check for updates");
	menu_bar->Append(file_menu, "&File");
	menu_bar->Append(go_menu, "&Go");
	menu_bar->Append(tools_menu, "&Tools");
	menu_bar->Append(help_menu, "&Help");
	SetMenuBar(menu_bar);
	Bind(wxEVT_MENU, &main_window::on_open, this, wxID_OPEN);
	Bind(wxEVT_MENU, &main_window::on_exit, this, wxID_EXIT);
	Bind(wxEVT_MENU, &main_window::on_about, this, wxID_ABOUT);
}

void main_window::on_open(wxCommandEvent& event) {
	wxFileDialog dlg(this, "Select a document to read", "", "", get_supported_wildcards(), wxFD_OPEN | wxFD_FILE_MUST_EXIST);
	if (dlg.ShowModal() == wxID_OK) {
		wxString path = dlg.GetPath();
		parser* par = find_parser_by_extension(wxFileName(path).GetExt());
		if (!par) return;
		wxMessageBox(par->name(), "Found", wxICON_INFORMATION);
	}
}

void main_window::on_exit(wxCommandEvent& event) {
	Close(true);
}

void main_window::on_about(wxCommandEvent& event) {
	wxMessageBox(APP_NAME + " is copyright (c) 2025 by Quin Gillespie. All rights reserved.", "About " + APP_NAME, wxICON_INFORMATION);
}
