#include "constants.hpp"
#include "go_to_dialog.hpp"
#include "main_window.hpp"
#include "parser.hpp"
#define UNIVERSAL_SPEECH_STATIC
#include <UniversalSpeech.h>
#include <wx/aboutdlg.h>
#include <wx/fdrepdlg.h>
#include <wx/filename.h>
#include <wx/tokenzr.h>

main_window::main_window() : wxFrame(nullptr, wxID_ANY, APP_NAME) {
	auto* panel = new wxPanel(this);
	notebook = new wxNotebook(panel, wxID_ANY);
	auto* sizer = new wxBoxSizer(wxVERTICAL);
	sizer->Add(notebook, 1, wxEXPAND | wxALL, 10);
	panel->SetSizer(sizer);
	auto* menu_bar = new wxMenuBar();
	auto* file_menu = new wxMenu();
	file_menu->Append(wxID_OPEN);
	file_menu->Append(wxID_CLOSE, "Close\tCtrl+F4");
	file_menu->Append(wxID_CLOSE_ALL, "Close &All\tCtrl+Shift+F4");
	file_menu->AppendSeparator();
	file_menu->Append(ID_EXPORT, "&Export...\tCtrl+E");
	file_menu->AppendSeparator();
	file_menu->Append(wxID_EXIT, "E&xit");
	auto* go_menu = new wxMenu();
	go_menu->Append(wxID_FIND);
	go_menu->Append(ID_FIND_NEXT, "Find Ne&xt\tF3");
	go_menu->Append(ID_FIND_PREVIOUS, "Find P&revious\tShift+F3");
	go_menu->AppendSeparator();
	go_menu->Append(ID_GO_TO, "&Go to...\tCtrl+G");
	go_menu->AppendSeparator();
	go_menu->Append(ID_PREVIOUS_SECTION, "Previous section\t[");
	go_menu->Append(ID_NEXT_SECTION, "Next section\t]");
	auto* tools_menu = new wxMenu();
	tools_menu->Append(ID_WORD_COUNT, "&Word count\tCtrl+W");
	tools_menu->AppendSeparator();
	tools_menu->Append(ID_TABLE_OF_CONTENTS, "Table of contents\tCtrl+T");
	auto* help_menu = new wxMenu();
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
	Bind(wxEVT_MENU, &main_window::on_close, this, wxID_CLOSE);
	Bind(wxEVT_MENU, &main_window::on_close_all, this, wxID_CLOSE_ALL);
	Bind(wxEVT_MENU, &main_window::on_export, this, ID_EXPORT);
	Bind(wxEVT_MENU, &main_window::on_exit, this, wxID_EXIT);
	Bind(wxEVT_MENU, &main_window::on_find, this, wxID_FIND);
	Bind(wxEVT_MENU, &main_window::on_find_next, this, ID_FIND_NEXT);
	Bind(wxEVT_MENU, &main_window::on_find_previous, this, ID_FIND_PREVIOUS);
	Bind(wxEVT_MENU, &main_window::on_go_to, this, ID_GO_TO);
	Bind(wxEVT_MENU, &main_window::on_previous_section, this, ID_PREVIOUS_SECTION);
	Bind(wxEVT_MENU, &main_window::on_next_section, this, ID_NEXT_SECTION);
	Bind(wxEVT_MENU, &main_window::on_word_count, this, ID_WORD_COUNT);
	Bind(wxEVT_MENU, &main_window::on_about, this, wxID_ABOUT);
	for (const int id : doc_command_ids)
		Bind(wxEVT_UPDATE_UI, &main_window::update_doc_commands, this, id);
}

user_data* main_window::active_user_data() const {
	auto page = notebook->GetPage(notebook->GetSelection());
	return static_cast<user_data*>(page->GetClientObject());
}

wxTextCtrl* main_window::active_text_ctrl() const {
	return static_cast<wxTextCtrl*>(active_user_data()->textbox);
}

parser* main_window::active_parser() const {
	return static_cast<parser*>(active_user_data()->par);
}

void main_window::open_document(const wxString& path, parser* par) {
	std::unique_ptr<document> doc = par->load(path);
	if (!doc) {
		wxMessageBox("Failed to load document.", "Error", wxICON_ERROR);
		return;
	}
	auto* page = new wxPanel(notebook, wxID_ANY);
	auto* page_sizer = new wxBoxSizer(wxVERTICAL);
	auto* content = new wxTextCtrl(page, wxID_ANY, "", wxDefaultPosition, wxDefaultSize, wxTE_MULTILINE | wxTE_READONLY | wxTE_RICH2 | wxTE_DONTWRAP);
	auto* data = new user_data;
	data->textbox = content;
	data->par = par;
	page->SetClientObject(data);
	page_sizer->Add(content, 1, wxEXPAND | wxALL, 5);
	page->SetSizer(page_sizer);
	wxString label = wxFileName(path).GetFullName();
	notebook->AddPage(page, label, true);
	content->Freeze();
	content->SetValue(doc->text_content());
	content->Thaw();
	content->SetFocus();
}

void main_window::update_doc_commands(wxUpdateUIEvent& e) {
	const bool has_doc = notebook->GetPageCount() > 0;
	e.Enable(has_doc);
}

void main_window::on_open(wxCommandEvent& event) {
	wxFileDialog dlg(this, "Select a document to read", "", "", get_supported_wildcards(), wxFD_OPEN | wxFD_FILE_MUST_EXIST);
	if (dlg.ShowModal() != wxID_OK) return;
	wxString path = dlg.GetPath();
	parser* par = find_parser_by_extension(wxFileName(path).GetExt());
	if (!par) {
		const bool open_as_txt = wxMessageBox("No suitable parser was found for " + path + ". Would you like to treat it as plain text?", "Warning", wxICON_WARNING | wxYES_NO) == wxYES;
		if (!open_as_txt) return;
		par = find_parser_by_extension("txt");
	}
	open_document(path, par);
}

void main_window::on_close(wxCommandEvent& event) {
	notebook->DeletePage(notebook->GetSelection());
}

void main_window::on_close_all(wxCommandEvent& event) {
	notebook->DeleteAllPages();
}

void main_window::on_export(wxCommandEvent& event) {
	wxWindow* page = notebook->GetPage(notebook->GetSelection());
	wxFileDialog save_dialog(this, "Export Document", "", "", "Text files (*.txt)|*.txt|All files (*.*)|*.*", wxFD_SAVE | wxFD_OVERWRITE_PROMPT);
	if (save_dialog.ShowModal() != wxID_OK) return;
	wxString file_path = save_dialog.GetPath();
	auto* content = active_text_ctrl();
	if (!content) {
		wxMessageBox("Failed to get edit control for active tab.", "Error", wxICON_ERROR);
		return;
	}
	wxFile file;
	if (!file.Open(file_path, wxFile::write)) {
		wxMessageBox("Failed to write to the selected file.", "Error", wxICON_ERROR);
		return;
	}
	file.Write(content->GetValue());
	file.Close();
}

void main_window::on_exit(wxCommandEvent& event) {
	Close(true);
}

void main_window::on_find(wxCommandEvent& event) {
	if (find_dialog) {
		// This horribleness is to focus the "Find what:" text field on dialog raise.
		wxWindowList children = find_dialog->GetChildren();
		int num_children = children.GetCount();
		wxTextCtrl* tc = nullptr;
		for(int i = 0; i < num_children; i++) {
			if (children[i]->IsKindOf(CLASSINFO(wxTextCtrl))) {
				tc = static_cast<wxTextCtrl*>(children[i]);
				break;
			}
		}
		find_dialog->Raise();
		if (tc) tc->SetFocus();
	}
	find_data.SetFlags(wxFR_DOWN); // Make down the default direction.
	find_dialog = new wxFindReplaceDialog(this, &find_data, "Find");
	find_dialog->Bind(wxEVT_FIND, &main_window::on_find_dialog, this);
	Bind(wxEVT_FIND_NEXT, &main_window::on_find_dialog, this);
	Bind(wxEVT_FIND_CLOSE, &main_window::on_find_close, this);
	find_dialog->Show();
}

void main_window::on_find_next(wxCommandEvent&) {
	if (!find_dialog) return;
	wxFindDialogEvent e(wxEVT_FIND_NEXT, find_dialog->GetId());
	e.SetFindString(find_data.GetFindString());
	e.SetFlags(find_data.GetFlags());
	wxPostEvent(this, e);
}

void main_window::on_find_previous(wxCommandEvent&) {
	if (!find_dialog) return;
	wxFindDialogEvent e(wxEVT_FIND_NEXT, find_dialog->GetId());
	e.SetFindString(find_data.GetFindString());
	e.SetFlags(find_data.GetFlags() & ~wxFR_DOWN); // Reverse direction.
	wxPostEvent(this, e);
}

void main_window::on_go_to(wxCommandEvent& event) {
	auto* content = active_text_ctrl();
	go_to_dialog dlg(this, content);
	if (dlg.ShowModal() != wxID_OK) return;
	long pos = content->XYToPosition(0, dlg.line_number() - 1);
	content->SetInsertionPoint(pos);
}

void main_window::on_previous_section(wxCommandEvent& event) {
	parser* par = active_parser();
	if (!par) return;
	if (!par->has_flag(parser_flags::supports_sections)) {
		speechSayA("No sections", 1);
		return;
	}
	auto nav = par->as_section_navigable();
	if (!nav) return;
	int prev_index = nav->previous_section_index();
	if (prev_index == -1) {
		speechSayA("No previous section", 1);
		return;
	}
	size_t offset = nav->offset_for_section(prev_index);
	active_text_ctrl()->SetInsertionPoint(offset);
	speechSayA(wxString::Format("Section %d", prev_index + 1), 1);
}

void main_window::on_next_section(wxCommandEvent& event) {
	parser* par = active_parser();
	if (!par) return;
	if (!par->has_flag(parser_flags::supports_sections)) {
		speechSayA("No sections", 1);
		return;
	}
	auto nav = par->as_section_navigable();
	if (!nav) return;
	int next_index = nav->next_section_index();
	if (next_index == -1) {
		speechSayA("No next section", 1);
		return;
	}
	size_t offset = nav->offset_for_section(next_index);
	active_text_ctrl()->SetInsertionPoint(offset);
	speechSayA(wxString::Format("Section %d", next_index + 1), 1);
}

void main_window::on_word_count(wxCommandEvent& event) {
	auto* content = active_text_ctrl();
	wxStringTokenizer tokenizer(content->GetValue(), " \t\r\n", wxTOKEN_STRTOK);
	int count = 0;
	while (tokenizer.HasMoreTokens()) {
		tokenizer.GetNextToken();
		++count;
	}
	wxMessageBox(wxString::Format("The document contains %d %s", count, count == 1 ? "word" : "words"), "Word count", wxICON_INFORMATION);
}

void main_window::on_about(wxCommandEvent& event) {
	wxAboutDialogInfo about_info;
	about_info.SetName(APP_NAME);
	about_info.SetVersion(APP_VERSION);
	about_info.SetCopyright(APP_COPYRIGHT);
	about_info.SetWebSite(APP_WEBSITE);
	wxAboutBox(about_info);
}

void main_window::on_find_dialog(wxFindDialogEvent& event) {
	auto* text_ctrl = active_text_ctrl();
	if (!text_ctrl) return;
	wxString query = event.GetFindString();
	const long flags = event.GetFlags();
	long sel_start, sel_end;
	text_ctrl->GetSelection(&sel_start, &sel_end);
	const long start_pos = (flags & wxFR_DOWN) ? sel_end : sel_start;
	wxString search_text = text_ctrl->GetValue();
	long found_pos = wxNOT_FOUND;
	if (!(flags & wxFR_MATCHCASE)) {
		query.MakeLower();
		search_text.MakeLower();
	}
	if (flags & wxFR_DOWN)
		found_pos = search_text.find(query, start_pos);
	else {
		search_text = search_text.substr(0, start_pos);
		found_pos = search_text.rfind(query);
	}
	if (found_pos == wxNOT_FOUND) {
		long wrap_start = (flags & wxFR_DOWN) ? 0 : text_ctrl->GetLastPosition();
		wxString wrap_text = text_ctrl->GetValue();
		if (!(flags & wxFR_MATCHCASE)) wrap_text.MakeLower();
		if (flags & wxFR_DOWN)
			found_pos = wrap_text.find(query, wrap_start);
		else
			found_pos = wrap_text.rfind(query);
		if (found_pos == wxNOT_FOUND) {
			wxMessageBox("Text not found.", "Find", wxICON_INFORMATION);
			return;
		}
		wxMessageBox("No more results. Wrapping search.", "Find", wxICON_INFORMATION);
	}
	text_ctrl->SetFocus();
	text_ctrl->SetSelection(found_pos, found_pos + query.Length());
	text_ctrl->ShowPosition(found_pos);
}

void main_window::on_find_close(wxFindDialogEvent& event) {
	find_dialog->Destroy();
	find_dialog = nullptr;
}
