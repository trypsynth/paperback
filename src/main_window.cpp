#include "main_window.hpp"
#include "constants.hpp"
#include "go_to_dialog.hpp"
#include "parser.hpp"
#include "toc_dialog.hpp"
#define UNIVERSAL_SPEECH_STATIC
#include "utils.hpp"
#include "document_info_dialog.hpp"
#include <UniversalSpeech.h>
#include <wx/aboutdlg.h>
#include <wx/config.h>
#include <wx/fdrepdlg.h>
#include <wx/filename.h>
#include <wx/tokenzr.h>
#include <wx/timer.h>

main_window::main_window() : wxFrame(nullptr, wxID_ANY, APP_NAME) {
	auto* panel = new wxPanel(this);
	notebook = new wxNotebook(panel, wxID_ANY);
	auto* sizer = new wxBoxSizer(wxVERTICAL);
	sizer->Add(notebook, 1, wxEXPAND | wxALL, 10);
	panel->SetSizer(sizer);
	create_menus();
	status_bar = CreateStatusBar(1);
	status_bar->SetStatusText("Ready");
	const std::pair<int, void (main_window::*)(wxCommandEvent&)> menu_bindings[] = {
		{wxID_OPEN, &main_window::on_open},
		{wxID_CLOSE, &main_window::on_close},
		{wxID_CLOSE_ALL, &main_window::on_close_all},
		{ID_EXPORT, &main_window::on_export},
		{wxID_EXIT, &main_window::on_exit},
		{wxID_FIND, &main_window::on_find},
		{ID_FIND_NEXT, &main_window::on_find_next},
		{ID_FIND_PREVIOUS, &main_window::on_find_previous},
		{ID_GO_TO, &main_window::on_go_to},
		{ID_PREVIOUS_SECTION, &main_window::on_previous_section},
		{ID_NEXT_SECTION, &main_window::on_next_section},
		{ID_WORD_COUNT, &main_window::on_word_count},
		{ID_DOC_INFO, &main_window::on_doc_info},
		{ID_TABLE_OF_CONTENTS, &main_window::on_toc},
		{wxID_ABOUT, &main_window::on_about},
	};
	for (const auto& [id, handler] : menu_bindings)
		Bind(wxEVT_MENU, handler, this, id);
	Bind(wxEVT_NOTEBOOK_PAGE_CHANGED, &main_window::on_notebook_page_changed, this);
	Bind(wxEVT_CLOSE_WINDOW, &main_window::on_close_window, this);
	for (const int id : doc_command_ids)
		Bind(wxEVT_UPDATE_UI, &main_window::update_doc_commands, this, id);
	
	// Initialize periodic position saving timer (30 seconds)
	position_save_timer = new wxTimer(this);
	Bind(wxEVT_TIMER, &main_window::on_position_save_timer, this, position_save_timer->GetId());
	position_save_timer->Start(5000);
}

wxTextCtrl* main_window::active_text_ctrl() const {
	return static_cast<wxTextCtrl*>(active_user_data()->textbox);
}

document* main_window::active_document() const {
	return active_user_data()->doc.get();
}

void main_window::open_document(const wxString& path, const parser* par) {
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
	data->doc = std::move(doc);
	data->file_path = path;
	page->SetClientObject(data);
	page_sizer->Add(content, 1, wxEXPAND | wxALL, 5);
	page->SetSizer(page_sizer);
	wxString label = wxFileName(path).GetFullName();
	notebook->AddPage(page, label, true);
	update_title();
	content->Freeze();
	content->SetValue(active_document()->text_content);
	content->Thaw();
	
	// Load and restore saved position
	long saved_position = load_document_position(path);
	if (saved_position > 0) {
		long max_position = content->GetLastPosition();
		if (saved_position <= max_position) {
			content->SetInsertionPoint(saved_position);
			content->ShowPosition(saved_position);
		}
	}
	
	content->SetFocus();
	content->Bind(wxEVT_LEFT_UP, &main_window::on_text_cursor_changed, this);
	content->Bind(wxEVT_KEY_UP, &main_window::on_text_cursor_changed, this);
	update_status_bar();
}

void main_window::create_menus() {
	auto* menu_bar = new wxMenuBar();
	menu_bar->Append(create_file_menu(), "&File");
	menu_bar->Append(create_go_menu(), "&Go");
	menu_bar->Append(create_tools_menu(), "&Tools");
	menu_bar->Append(create_help_menu(), "&Help");
	SetMenuBar(menu_bar);
}

wxMenu* main_window::create_file_menu() {
	auto* menu = new wxMenu();
	menu->Append(wxID_OPEN);
	menu->Append(wxID_CLOSE, "Close\tCtrl+F4");
	menu->Append(wxID_CLOSE_ALL, "Close &All\tCtrl+Shift+F4");
	menu->AppendSeparator();
	menu->Append(ID_EXPORT, "&Export...\tCtrl+E");
	menu->AppendSeparator();
	menu->Append(wxID_EXIT, "E&xit");
	return menu;
}

wxMenu* main_window::create_go_menu() {
	auto* menu = new wxMenu();
	menu->Append(wxID_FIND);
	menu->Append(ID_FIND_NEXT, "Find Ne&xt\tF3");
	menu->Append(ID_FIND_PREVIOUS, "Find P&revious\tShift+F3");
	menu->AppendSeparator();
	menu->Append(ID_GO_TO, "&Go to...\tCtrl+G");
	menu->AppendSeparator();
	menu->Append(ID_PREVIOUS_SECTION, "Previous section\t[");
	menu->Append(ID_NEXT_SECTION, "Next section\t]");
	return menu;
}

wxMenu* main_window::create_tools_menu() {
	auto* menu = new wxMenu();
	menu->Append(ID_WORD_COUNT, "&Word count\tCtrl+W");
	menu->Append(ID_DOC_INFO, "Document &info\tCtrl+I");
	menu->AppendSeparator();
	menu->Append(ID_TABLE_OF_CONTENTS, "Table of contents\tCtrl+T");
	return menu;
}

wxMenu* main_window::create_help_menu() {
	auto* menu = new wxMenu();
	menu->Append(wxID_ABOUT, "About " + APP_NAME + "\tCtrl+F1");
	menu->Append(wxID_HELP, "&Help\tF1");
	menu->AppendSeparator();
	menu->Append(ID_CHECK_FOR_UPDATES, "&Check for updates");
	return menu;
}

user_data* main_window::active_user_data() const {
	auto* page = notebook->GetPage(notebook->GetSelection());
	return static_cast<user_data*>(page->GetClientObject());
}

void main_window::update_doc_commands(wxUpdateUIEvent& e) {
	const bool has_doc = notebook->GetPageCount() > 0;
	e.Enable(has_doc);
}

void main_window::update_title() {
	if (notebook->GetPageCount() == 0)
		SetTitle(APP_NAME);
	else
		SetTitle(active_document()->title + " - " + APP_NAME);
}

void main_window::update_status_bar() {
	if (notebook->GetPageCount() == 0) {
		status_bar->SetStatusText("Ready");
		return;
	}
	auto* text_ctrl = active_text_ctrl();
	if (!text_ctrl) {
		status_bar->SetStatusText("Ready");
		return;
	}
	long current_pos = text_ctrl->GetInsertionPoint();
	long line;
	text_ctrl->PositionToXY(current_pos, 0, &line);
	int total_lines = text_ctrl->GetNumberOfLines();
	int current_line = line + 1;
	int percentage = total_lines > 0 ? (current_line * 100) / total_lines : 0;
	status_bar->SetStatusText(wxString::Format("%d%%", percentage));
}

void main_window::save_document_position(const wxString& path, long position) {
	wxConfigBase* config = wxConfigBase::Get();
	if (!config) return;
	config->SetPath("/documents");
	config->Write(path, position);
	config->Flush();
}

long main_window::load_document_position(const wxString& path) {
	wxConfigBase* config = wxConfigBase::Get();
	if (!config) return 0;
	config->SetPath("/documents");
	return config->Read(path, 0L);
}

void main_window::save_current_tab_position() {
	if (notebook->GetPageCount() == 0) return;
	auto* data = active_user_data();
	if (!data || !data->textbox) return;
	long position = data->textbox->GetInsertionPoint();
	save_document_position(data->file_path, position);
}

void main_window::on_open(wxCommandEvent& event) {
	wxFileDialog dlg(this, "Select a document to read", "", "", get_supported_wildcards(), wxFD_OPEN | wxFD_FILE_MUST_EXIST);
	if (dlg.ShowModal() != wxID_OK) return;
	wxString path = dlg.GetPath();
	const parser* par = find_parser_by_extension(wxFileName(path).GetExt());
	if (!par) {
		const bool open_as_txt = wxMessageBox("No suitable parser was found for " + path + ". Would you like to treat it as plain text?", "Warning", wxICON_WARNING | wxYES_NO) == wxYES;
		if (!open_as_txt) return;
		par = find_parser_by_extension("txt");
	}
	open_document(path, par);
}

void main_window::on_close(wxCommandEvent& event) {
	save_current_tab_position();
	notebook->DeletePage(notebook->GetSelection());
	update_title();
	update_status_bar();
}

void main_window::on_close_all(wxCommandEvent& event) {
	for (size_t i = 0; i < notebook->GetPageCount(); ++i) {
		auto* page = notebook->GetPage(i);
		auto* data = static_cast<user_data*>(page->GetClientObject());
		if (data && data->textbox) {
			long position = data->textbox->GetInsertionPoint();
			save_document_position(data->file_path, position);
		}
	}
	notebook->DeleteAllPages();
	update_title();
	update_status_bar();
}

void main_window::on_export(wxCommandEvent& event) {
	wxFileDialog save_dialog(this, "Export Document", "", active_document()->title + ".txt", "Text files (*.txt)|*.txt|All files (*.*)|*.*", wxFD_SAVE | wxFD_OVERWRITE_PROMPT);
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
		for (int i = 0; i < num_children; i++) {
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

void main_window::on_find_next(wxCommandEvent& event) {
	if (!find_dialog) return;
	wxFindDialogEvent e(wxEVT_FIND_NEXT, find_dialog->GetId());
	e.SetFindString(find_data.GetFindString());
	e.SetFlags(find_data.GetFlags());
	wxPostEvent(this, e);
}

void main_window::on_find_previous(wxCommandEvent& event) {
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
	update_status_bar();
}

void main_window::on_previous_section(wxCommandEvent& event) {
	auto* doc = active_document();
	if (!doc) return;
	if (!doc->has_flag(document_flags::supports_sections)) {
		speechSayA("Document has no sections", 1);
		return;
	}
	size_t current_pos = active_text_ctrl()->GetInsertionPoint();
	int prev_index = doc->previous_section_index(current_pos);
	if (prev_index == -1) {
		speechSayA("No previous section", 1);
		return;
	}
	size_t offset = doc->offset_for_section(prev_index);
	active_text_ctrl()->SetInsertionPoint(offset);
	long line;
	active_text_ctrl()->PositionToXY(active_text_ctrl()->GetInsertionPoint(), 0, &line);
	wxString current_line = active_text_ctrl()->GetLineText(line);
	speechSayA(current_line, 1);
	update_status_bar();
}

void main_window::on_next_section(wxCommandEvent& event) {
	auto* doc = active_document();
	if (!doc) return;
	if (!doc->has_flag(document_flags::supports_sections)) {
		speechSayA("Document has no sections", 1);
		return;
	}
	size_t current_pos = active_text_ctrl()->GetInsertionPoint();
	int next_index = doc->next_section_index(current_pos);
	if (next_index == -1) {
		speechSayA("No next section", 1);
		return;
	}
	size_t offset = doc->offset_for_section(next_index);
	active_text_ctrl()->SetInsertionPoint(offset);
	long line;
	active_text_ctrl()->PositionToXY(active_text_ctrl()->GetInsertionPoint(), 0, &line);
	wxString current_line = active_text_ctrl()->GetLineText(line);
	speechSayA(current_line, 1);
	update_status_bar();
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

void main_window::on_doc_info(wxCommandEvent& event) {
	auto* doc = active_document();
	if (!doc) return;
	document_info_dialog dlg(this, doc);
	dlg.ShowModal();
}

void main_window::on_toc(wxCommandEvent& event) {
	auto* doc = active_document();
	if (!doc) return;
	if (!doc->has_flag(document_flags::supports_toc)) {
		speechSayA("No table of contents", 1);
		return;
	}
	if (doc->toc_items.empty()) {
		speechSayA("Table of contents is empty", 1);
		return;
	}
	size_t current_pos = active_text_ctrl()->GetInsertionPoint();
	int current_section_idx = doc->section_index(current_pos);
	size_t current_offset = doc->offset_for_section(current_section_idx);
	toc_dialog dlg(this, doc, current_offset);
	if (dlg.ShowModal() != wxID_OK) return;
	int offset = dlg.get_selected_offset();
	if (offset < 0) return;
	auto* text_ctrl = active_text_ctrl();
	if (!text_ctrl) return;
	long max_pos = text_ctrl->GetLastPosition();
	offset = offset > max_pos ? max_pos : offset < 0 ? 0 : offset;
	text_ctrl->SetInsertionPoint(offset);
	text_ctrl->ShowPosition(offset);
	text_ctrl->SetFocus();
	update_status_bar();
}

void main_window::on_about(wxCommandEvent& event) {
	wxAboutDialogInfo about_info;
	about_info.SetName(APP_NAME);
	about_info.SetVersion(APP_VERSION);
	about_info.SetCopyright(APP_COPYRIGHT);
	about_info.SetWebSite(APP_WEBSITE);
	wxAboutBox(about_info);
}

void main_window::on_notebook_page_changed(wxBookCtrlEvent& event) {
	// Save position of the previously active tab
	int old_selection = event.GetOldSelection();
	if (old_selection >= 0) {
		auto* page = notebook->GetPage(old_selection);
		auto* data = static_cast<user_data*>(page->GetClientObject());
		if (data && data->textbox) {
			long position = data->textbox->GetInsertionPoint();
			save_document_position(data->file_path, position);
		}
	}
	update_title();
	update_status_bar();
	event.Skip();
}

void main_window::on_text_cursor_changed(wxEvent& event) {
	update_status_bar();
	event.Skip();
}

void main_window::on_find_dialog(wxFindDialogEvent& event) {
	auto* text_ctrl = active_text_ctrl();
	if (!text_ctrl) return;
	const wxString& full_text = text_ctrl->GetValue();
	const wxString& query = event.GetFindString();
	const long flags = event.GetFlags();
	long sel_start, sel_end;
	text_ctrl->GetSelection(&sel_start, &sel_end);
	bool forward = flags & wxFR_DOWN;
	bool match_case = flags & wxFR_MATCHCASE;
	long start_pos = forward ? sel_end : sel_start;
	long found_pos = find_text(full_text, query, start_pos, forward, match_case);
	if (found_pos == wxNOT_FOUND) {
		speechSayA("No more results. Wrapping search.", 1);
		start_pos = forward ? 0 : full_text.Length();
		found_pos = find_text(full_text, query, start_pos, forward, match_case);
		if (found_pos == wxNOT_FOUND) {
			speechSayA("Not found.", 1);
			return;
		}
	}
	text_ctrl->SetFocus();
	text_ctrl->SetSelection(found_pos, found_pos + query.Length());
	text_ctrl->ShowPosition(found_pos);
	update_status_bar();
}

void main_window::on_find_close(wxFindDialogEvent& event) {
	find_dialog->Destroy();
	find_dialog = nullptr;
}

void main_window::on_close_window(wxCloseEvent& event) {
	for (size_t i = 0; i < notebook->GetPageCount(); ++i) {
		auto* page = notebook->GetPage(i);
		auto* data = static_cast<user_data*>(page->GetClientObject());
		if (data && data->textbox) {
			long position = data->textbox->GetInsertionPoint();
			save_document_position(data->file_path, position);
		}
	}
	if (position_save_timer) {
		position_save_timer->Stop();
		delete position_save_timer;
		position_save_timer = nullptr;
	}
	event.Skip();
}

void main_window::on_position_save_timer(wxTimerEvent& event) {
	save_current_tab_position();
}
