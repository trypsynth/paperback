#include "main_window.hpp"
#include "constants.hpp"
#include "go_to_dialog.hpp"
#include "parser.hpp"
#include "utils.hpp"
#include <wx/aboutdlg.h>
#include <wx/filename.h>
#include <wx/stdpaths.h>
#include <wx/timer.h>

main_window::main_window() : wxFrame(nullptr, wxID_ANY, APP_NAME) {
	auto* const panel = new wxPanel(this);
	notebook = new wxNotebook(panel, wxID_ANY);
	auto* const sizer = new wxBoxSizer(wxVERTICAL);
	sizer->Add(notebook, 1, wxEXPAND | wxALL, 10);
	panel->SetSizer(sizer);
	doc_manager = std::make_unique<document_manager>(notebook);
	create_menus();
	status_bar = CreateStatusBar(1);
	status_bar->SetStatusText("Ready");
	position_save_timer = new wxTimer(this);
	bind_events();
	position_save_timer->Start(5000);
	update_ui();
}

void main_window::create_menus() {
	auto* const menu_bar = new wxMenuBar();
	menu_bar->Append(create_file_menu(), "&File");
	menu_bar->Append(create_go_menu(), "&Go");
	menu_bar->Append(create_tools_menu(), "&Tools");
	menu_bar->Append(create_help_menu(), "&Help");
	SetMenuBar(menu_bar);
}

wxMenu* main_window::create_file_menu() {
	auto* const menu = new wxMenu();
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
	auto* const menu = new wxMenu();
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
	auto* const menu = new wxMenu();
	menu->Append(ID_WORD_COUNT, "&Word count\tCtrl+W");
	menu->Append(ID_DOC_INFO, "Document &info\tCtrl+I");
	menu->AppendSeparator();
	menu->Append(ID_TABLE_OF_CONTENTS, "Table of contents\tCtrl+T");
	return menu;
}

wxMenu* main_window::create_help_menu() {
	auto* const menu = new wxMenu();
	menu->Append(wxID_ABOUT, "About " + APP_NAME + "\tCtrl+F1");
	menu->Append(wxID_HELP, "&Help\tF1");
	return menu;
}

void main_window::bind_events() {
	constexpr std::pair<int, void (main_window::*)(wxCommandEvent&)> menu_bindings[] = {
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
		{wxID_HELP, &main_window::on_help},
	};
	for (const auto& [id, handler] : menu_bindings)
		Bind(wxEVT_MENU, handler, this, id);
	Bind(wxEVT_NOTEBOOK_PAGE_CHANGED, &main_window::on_notebook_page_changed, this);
	Bind(wxEVT_CLOSE_WINDOW, &main_window::on_close_window, this);
	Bind(wxEVT_TIMER, &main_window::on_position_save_timer, this, position_save_timer->GetId());
}

void main_window::update_ui() {
	const bool has_doc = doc_manager->has_documents();
	const auto enable = [this](const int id, const bool state) noexcept {
		if (auto* item = GetMenuBar()->FindItem(id))
			item->Enable(state);
	};
	constexpr int doc_items[] = {
		wxID_CLOSE,
		wxID_CLOSE_ALL,
		ID_EXPORT,
		wxID_FIND,
		ID_FIND_NEXT,
		ID_FIND_PREVIOUS,
		ID_GO_TO,
		ID_WORD_COUNT,
		ID_DOC_INFO};
	for (const auto id : doc_items)
		enable(id, has_doc);
	if (!has_doc) {
		enable(ID_PREVIOUS_SECTION, false);
		enable(ID_NEXT_SECTION, false);
		enable(ID_TABLE_OF_CONTENTS, false);
		return;
	}
	enable(ID_PREVIOUS_SECTION, doc_manager->active_document_supports_sections());
	enable(ID_NEXT_SECTION, doc_manager->active_document_supports_sections());
	enable(ID_TABLE_OF_CONTENTS, doc_manager->active_document_supports_toc());
}

void main_window::update_title() {
	SetTitle(doc_manager->get_window_title(APP_NAME));
}

void main_window::update_status_bar() {
	status_bar->SetStatusText(doc_manager->get_status_text());
}

void main_window::on_open(wxCommandEvent&) {
	wxFileDialog dlg(this, "Select a document to read", "", "", get_supported_wildcards(), wxFD_OPEN | wxFD_FILE_MUST_EXIST);
	if (dlg.ShowModal() != wxID_OK) return;
	const auto path = dlg.GetPath();
	const auto* par = find_parser_by_extension(wxFileName(path).GetExt());
	if (!par) {
		if (!should_open_as_txt(path)) return;
		par = find_parser_by_extension("txt");
	}
	if (!doc_manager->open_document(path, par)) {
		wxMessageBox("Failed to load document.", "Error", wxICON_ERROR);
		return;
	}
	auto* const text_ctrl = doc_manager->get_active_text_ctrl();
	if (text_ctrl) {
		text_ctrl->Bind(wxEVT_LEFT_UP, &main_window::on_text_cursor_changed, this);
		text_ctrl->Bind(wxEVT_KEY_UP, &main_window::on_text_cursor_changed, this);
	}
	update_title();
	update_status_bar();
	update_ui();
}

void main_window::on_close(wxCommandEvent&) {
	doc_manager->close_document(doc_manager->get_active_tab_index());
	update_title();
	update_status_bar();
	update_ui();
}

void main_window::on_close_all(wxCommandEvent&) {
	doc_manager->close_all_documents();
	update_title();
	update_status_bar();
	update_ui();
}

void main_window::on_export(wxCommandEvent&) {
	auto* const doc = doc_manager->get_active_document();
	if (!doc) return;
	wxFileDialog save_dialog(this, "Export Document", "", doc->title + ".txt", "Text files (*.txt)|*.txt|All files (*.*)|*.*", wxFD_SAVE | wxFD_OVERWRITE_PROMPT);
	if (save_dialog.ShowModal() != wxID_OK) return;
	const auto file_path = save_dialog.GetPath();
	if (!doc_manager->export_document(doc_manager->get_active_tab_index(), file_path))
		wxMessageBox("Failed to export document.", "Error", wxICON_ERROR);
}

void main_window::on_exit(wxCommandEvent&) {
	Close(true);
}

void main_window::on_find(wxCommandEvent&) {
	if (!find_dlg) find_dlg = new find_dialog(this);
	// If there's selected text, use it as the initial search term.
	auto* const text_ctrl = doc_manager->get_active_text_ctrl();
	if (text_ctrl) {
		long start, end;
		text_ctrl->GetSelection(&start, &end);
		if (start != end) {
			const auto selected = text_ctrl->GetStringSelection();
			find_dlg->set_find_text(selected);
		}
	}
	find_dlg->Show();
	find_dlg->Raise();
	find_dlg->focus_find_text();
}

void main_window::on_find_next(wxCommandEvent&) {
	if (find_dlg && find_dlg->IsShown())
		do_find(true);
	else {
		wxCommandEvent evt{};
		on_find(evt);
	}
}

void main_window::on_find_previous(wxCommandEvent&) {
	if (find_dlg && find_dlg->IsShown())
		do_find(false);
	else {
		wxCommandEvent evt{};
		on_find(evt);
	}
}

void main_window::on_go_to(wxCommandEvent&) {
	auto* const text_ctrl = doc_manager->get_active_text_ctrl();
	if (!text_ctrl) return;
	go_to_dialog dlg(this, text_ctrl);
	if (dlg.ShowModal() != wxID_OK) return;
	const auto pos = dlg.get_position();
	doc_manager->go_to_position(pos);
	update_status_bar();
}

void main_window::on_previous_section(wxCommandEvent&) {
	doc_manager->go_to_previous_section();
	update_status_bar();
}

void main_window::on_next_section(wxCommandEvent&) {
	doc_manager->go_to_next_section();
	update_status_bar();
}

void main_window::on_word_count(wxCommandEvent&) {
	const size_t count = doc_manager->get_active_document()->get_word_count();
	wxMessageBox(wxString::Format("The document contains %d %s", count, count == 1 ? "word" : "words"), "Word count", wxICON_INFORMATION);
}

void main_window::on_doc_info(wxCommandEvent&) {
	doc_manager->show_document_info(this);
}

void main_window::on_toc(wxCommandEvent&) {
	doc_manager->show_table_of_contents(this);
	update_status_bar();
}

void main_window::on_about(wxCommandEvent&) {
	wxAboutDialogInfo about_info;
	about_info.SetName(APP_NAME);
	about_info.SetVersion(APP_VERSION);
	about_info.SetCopyright(APP_COPYRIGHT);
	about_info.SetWebSite(APP_WEBSITE);
	wxAboutBox(about_info);
}

void main_window::on_help(wxCommandEvent&) {
	const auto path = wxFileName(wxStandardPaths::Get().GetExecutablePath()).GetPath();
	const auto url = "file://" + wxFileName(path, "readme.html").GetFullPath();
	if (!wxLaunchDefaultBrowser(url))
		wxMessageBox("Failed to launch default browser.", "Error", wxICON_ERROR);
}

void main_window::on_notebook_page_changed(wxBookCtrlEvent& event) {
	const auto old_selection = event.GetOldSelection();
	if (old_selection >= 0) {
		auto* const tab = doc_manager->get_tab(old_selection);
		if (tab && tab->text_ctrl) {
			const auto position = tab->text_ctrl->GetInsertionPoint();
			doc_manager->save_document_position(tab->file_path, position);
		}
	}
	update_title();
	update_status_bar();
	update_ui();
	event.Skip();
}

void main_window::on_text_cursor_changed(wxEvent& event) {
	update_status_bar();
	event.Skip();
}

void main_window::on_close_window(wxCloseEvent& event) {
	if (position_save_timer) {
		position_save_timer->Stop();
		delete position_save_timer;
		position_save_timer = nullptr;
	}
	if (find_dlg) {
		find_dlg->Destroy();
		find_dlg = nullptr;
	}
	doc_manager.reset();
	event.Skip();
}

void main_window::on_position_save_timer(wxTimerEvent&) {
	doc_manager->save_current_tab_position();
}

void main_window::do_find(bool forward) {
	if (!find_dlg) return;
	auto* const text_ctrl = doc_manager->get_active_text_ctrl();
	if (!text_ctrl) return;
	const auto& query = find_dlg->get_find_text();
	if (query.IsEmpty()) return;
	find_options options = find_options::none;
	if (forward) options |= find_options::forward;
	if (find_dlg->get_match_case()) options |= find_options::match_case;
	if (find_dlg->get_match_whole_word()) options |= find_options::match_whole_word;
	if (find_dlg->get_use_regex()) options |= find_options::use_regex;
	long sel_start, sel_end;
	text_ctrl->GetSelection(&sel_start, &sel_end);
	const long start_pos = forward ? sel_end : sel_start;
	long found_pos = doc_manager->find_text(query, start_pos, options);
	if (found_pos == wxNOT_FOUND) {
		speak("No more results. Wrapping search.");
		const auto wrap_pos = forward ? 0 : text_ctrl->GetLastPosition();
		found_pos = doc_manager->find_text(query, wrap_pos, options);
		if (found_pos == wxNOT_FOUND) {
			speak("Not found.");
			return;
		}
	}
	text_ctrl->SetFocus();
	text_ctrl->SetSelection(found_pos, found_pos + query.Length());
	text_ctrl->ShowPosition(found_pos);
	update_status_bar();
}
