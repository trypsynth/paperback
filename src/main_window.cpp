#include "constants.hpp"
#include "main_window.hpp"
#include "parser.hpp"
#include <wx/aboutdlg.h>
#include <wx/filename.h>

main_window::main_window() : wxFrame(nullptr, wxID_ANY, APP_NAME) {
	auto* panel = new wxPanel(this);
	notebook = new wxNotebook(panel, wxID_ANY);
	auto* sizer = new wxBoxSizer(wxVERTICAL);
	sizer->Add(notebook, 1, wxEXPAND | wxALL, 10);
	panel->SetSizer(sizer);
	auto* menu_bar = new wxMenuBar();
	auto* doc_menu = new wxMenu();
	doc_menu->Append(wxID_OPEN);
	doc_menu->Append(wxID_CLOSE);
	doc_menu->Append(wxID_CLOSE_ALL, "Close &All\tCtrl+Shift+W");
	doc_menu->AppendSeparator();
	doc_menu->Append(ID_EXPORT, "&Export...\tCtrl+E");
	doc_menu->AppendSeparator();
	doc_menu->Append(wxID_EXIT, "E&xit");
	auto* go_menu = new wxMenu();
	go_menu->Append(wxID_FIND);
	go_menu->Append(ID_FIND_NEXT, "Find Ne&xt\tF3");
	go_menu->Append(ID_FIND_PREVIOUS, "Find P&revious\tShift+F3");
	go_menu->AppendSeparator();
	go_menu->Append(ID_GO_TO_LINE, "&Go to Line...\tCtrl+G");
	go_menu->Append(ID_GO_TO_PERCENT, "Go to &Percent...\tCtrl+Shift+G");
	go_menu->AppendSeparator();
	go_menu->Append(ID_TABLE_OF_CONTENTS, "Table of contents\tCtrl+T");
	auto* tools_menu = new wxMenu();
	tools_menu->Append(ID_WORD_COUNT, "&Word count\tAlt+W");
	auto* help_menu = new wxMenu();
	help_menu->Append(wxID_ABOUT, "About " + APP_NAME + "\tCtrl+F1");
	help_menu->Append(wxID_HELP, "&Help\tF1");
	help_menu->AppendSeparator();
	help_menu->Append(ID_CHECK_FOR_UPDATES, "&Check for updates");
	menu_bar->Append(doc_menu, "&Document");
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
	if (dlg.ShowModal() != wxID_OK) return;
	wxString path = dlg.GetPath();
	parser* par = find_parser_by_extension(wxFileName(path).GetExt());
	if (!par) {
		wxMessageBox("No suitable parser found for " + path, "Error", wxICON_ERROR);
		return;
	}
	std::unique_ptr<document> doc = par->load(path);
	if (!doc) {
		wxMessageBox("Failed to load the document: " + path, "Error", wxICON_ERROR);
		return;
	}
	wxPanel* page = new wxPanel(notebook, wxID_ANY);
	wxBoxSizer* page_sizer = new wxBoxSizer(wxVERTICAL);
	wxTextCtrl* content = new wxTextCtrl(page, wxID_ANY, doc->text_content(), wxDefaultPosition, wxDefaultSize, wxTE_MULTILINE | wxTE_READONLY | wxTE_RICH2);
	page_sizer->Add(content, 1, wxEXPAND | wxALL, 5);
	page->SetSizer(page_sizer);
	wxString label = wxFileName(path).GetFullName();
	notebook->AddPage(page, label, true);
}

void main_window::on_exit(wxCommandEvent& event) {
	Close(true);
}

void main_window::on_about(wxCommandEvent& event) {
	wxAboutDialogInfo about_info;
	about_info.SetName(APP_NAME);
	about_info.SetVersion(APP_VERSION);
	about_info.SetCopyright(APP_COPYRIGHT);
	about_info.SetWebSite(APP_WEBSITE);
	wxAboutBox(about_info);
}
