/* main_window.cpp - main user interface of Paperback.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#include "main_window.hpp"
#include "app.hpp"
#include "constants.hpp"
#include "dialogs.hpp"
#include "parser.hpp"
#include "structured_nav.hpp"
#include "utils.hpp"
#include <wx/aboutdlg.h>
#include <wx/filename.h>
#include <wx/stdpaths.h>
#include <wx/timer.h>

main_window::main_window() : wxFrame(nullptr, wxID_ANY, APP_NAME) {
	auto* const panel = new wxPanel(this);
	notebook = new wxNotebook(panel, wxID_ANY);
	#ifdef __WXMSW__
		notebook->MSWDisableComposited();
	#endif
	auto* const sizer = new wxBoxSizer(wxVERTICAL);
	sizer->Add(notebook, 1, wxEXPAND | wxALL, 10);
	panel->SetSizer(sizer);
	doc_manager = std::make_unique<document_manager>(notebook);
	create_menus();
	status_bar = CreateStatusBar(1);
	status_bar->SetStatusText("Ready");
	position_save_timer = new wxTimer(this);
	bind_events();
	position_save_timer->Start(POSITION_SAVE_TIMER_INTERVAL);
	update_ui();
}

main_window::~main_window() {
	if (position_save_timer) {
		position_save_timer->Stop();
		position_save_timer = nullptr;
	}
	if (find_dlg) {
		find_dlg->Destroy();
		find_dlg = nullptr;
	}
	doc_manager.reset();
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
	recent_documents_menu = new wxMenu();
	menu->AppendSubMenu(recent_documents_menu, "&Recent Documents");
	update_recent_documents_menu();
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
	menu->Append(ID_GO_TO_PAGE, "Go to &page...\tCtrl+P");
	menu->AppendSeparator();
	menu->Append(ID_PREVIOUS_SECTION, "Previous section\t[");
	menu->Append(ID_NEXT_SECTION, "Next section\t]");
	menu->AppendSeparator();
	structured_nav_manager::create_heading_menu(menu);
	menu->AppendSeparator();
	menu->Append(ID_PREVIOUS_PAGE, "Previous &page\tShift+P");
	menu->Append(ID_NEXT_PAGE, "&Next page\tP");
	return menu;
}

wxMenu* main_window::create_tools_menu() {
	auto* const menu = new wxMenu();
	menu->Append(ID_WORD_COUNT, "&Word count\tCtrl+W");
	menu->Append(ID_DOC_INFO, "Document &info\tCtrl+I");
	menu->AppendSeparator();
	menu->Append(ID_TABLE_OF_CONTENTS, "Table of contents\tCtrl+T");
	menu->AppendSeparator();
	menu->Append(ID_OPTIONS, "&Options\tCtrl+,");
	return menu;
}

wxMenu* main_window::create_help_menu() {
	auto* const menu = new wxMenu();
	menu->Append(wxID_ABOUT, "About " + APP_NAME + "\tCtrl+F1");
	menu->Append(wxID_HELP, "View &help in default browser\tF1");
	menu->Append(ID_HELP_INTERNAL, "View Help in " + APP_NAME + "\tShift+F1");
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
		{ID_GO_TO_PAGE, &main_window::on_go_to_page},
		{ID_PREVIOUS_SECTION, &main_window::on_previous_section},
		{ID_NEXT_SECTION, &main_window::on_next_section},
		{ID_PREVIOUS_HEADING, &main_window::on_previous_heading},
		{ID_NEXT_HEADING, &main_window::on_next_heading},
		{ID_PREVIOUS_HEADING_1, &main_window::on_previous_heading_1},
		{ID_NEXT_HEADING_1, &main_window::on_next_heading_1},
		{ID_PREVIOUS_HEADING_2, &main_window::on_previous_heading_2},
		{ID_NEXT_HEADING_2, &main_window::on_next_heading_2},
		{ID_PREVIOUS_HEADING_3, &main_window::on_previous_heading_3},
		{ID_NEXT_HEADING_3, &main_window::on_next_heading_3},
		{ID_PREVIOUS_HEADING_4, &main_window::on_previous_heading_4},
		{ID_NEXT_HEADING_4, &main_window::on_next_heading_4},
		{ID_PREVIOUS_HEADING_5, &main_window::on_previous_heading_5},
		{ID_NEXT_HEADING_5, &main_window::on_next_heading_5},
		{ID_PREVIOUS_HEADING_6, &main_window::on_previous_heading_6},
		{ID_NEXT_HEADING_6, &main_window::on_next_heading_6},
		{ID_PREVIOUS_PAGE, &main_window::on_previous_page},
		{ID_NEXT_PAGE, &main_window::on_next_page},
		{ID_WORD_COUNT, &main_window::on_word_count},
		{ID_DOC_INFO, &main_window::on_doc_info},
		{ID_TABLE_OF_CONTENTS, &main_window::on_toc},
		{ID_OPTIONS, &main_window::on_options},
		{wxID_ABOUT, &main_window::on_about},
		{wxID_HELP, &main_window::on_help},
		{ID_HELP_INTERNAL, &main_window::on_help_internal},
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
		ID_GO_TO_PAGE,
		ID_WORD_COUNT,
		ID_DOC_INFO};
	for (const auto id : doc_items)
		enable(id, has_doc);
	if (!has_doc) {
		enable(ID_PREVIOUS_SECTION, false);
		enable(ID_NEXT_SECTION, false);
		enable(ID_PREVIOUS_PAGE, false);
		enable(ID_NEXT_PAGE, false);
		enable(ID_TABLE_OF_CONTENTS, false);
		return;
	}
	enable(ID_PREVIOUS_SECTION, true);
	enable(ID_NEXT_SECTION, true);
	enable(ID_PREVIOUS_HEADING, true);
	enable(ID_NEXT_HEADING, true);
	enable(ID_PREVIOUS_HEADING_1, true);
	enable(ID_NEXT_HEADING_1, true);
	enable(ID_PREVIOUS_HEADING_2, true);
	enable(ID_NEXT_HEADING_2, true);
	enable(ID_PREVIOUS_HEADING_3, true);
	enable(ID_NEXT_HEADING_3, true);
	enable(ID_PREVIOUS_HEADING_4, true);
	enable(ID_NEXT_HEADING_4, true);
	enable(ID_PREVIOUS_HEADING_5, true);
	enable(ID_NEXT_HEADING_5, true);
	enable(ID_PREVIOUS_HEADING_6, true);
	enable(ID_NEXT_HEADING_6, true);
	enable(ID_PREVIOUS_PAGE, true);
	enable(ID_NEXT_PAGE, true);
	enable(ID_TABLE_OF_CONTENTS, true);
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
	wxGetApp().open_file(path);
	auto* const text_ctrl = doc_manager->get_active_text_ctrl();
	if (text_ctrl) {
		text_ctrl->Bind(wxEVT_LEFT_UP, &main_window::on_text_cursor_changed, this);
		text_ctrl->Bind(wxEVT_KEY_UP, &main_window::on_text_cursor_changed, this);
	}
	auto& config_mgr = wxGetApp().get_config_manager();
	config_mgr.add_recent_document(path);
	update_recent_documents_menu();
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

void main_window::on_go_to_page(wxCommandEvent&) {
	auto* const doc = doc_manager->get_active_document();
	if (!doc) return;
	if (!doc->has_flag(document_flags::supports_pages)) {
		speak("No pages.");
		return;
	}
	int current_page = 1;
	auto* const text_ctrl = doc_manager->get_active_text_ctrl();
	if (!text_ctrl) return;
	const size_t current_pos = text_ctrl->GetInsertionPoint();
	const int current_page_idx = doc->page_index(current_pos);
	if (current_page_idx >= 0) current_page = current_page_idx + 1; // Convert to 1-based index
	go_to_page_dialog dlg(this, doc, current_page);
	if (dlg.ShowModal() != wxID_OK) return;
	const int page = dlg.get_page_number();
	if (page >= 1 && page <= static_cast<int>(doc->buffer.count_markers_by_type(marker_type::page_break))) {
		const size_t offset = doc->buffer.get_marker_position_by_index(marker_type::page_break, page - 1); // Convert to 0-based index
		doc_manager->go_to_position(offset);
		update_status_bar();
	}
}

void main_window::on_previous_section(wxCommandEvent&) {
	doc_manager->go_to_previous_section();
	update_status_bar();
}

void main_window::on_next_section(wxCommandEvent&) {
	doc_manager->go_to_next_section();
	update_status_bar();
}

void main_window::on_previous_page(wxCommandEvent&) {
	doc_manager->go_to_previous_page();
	update_status_bar();
}

void main_window::on_next_page(wxCommandEvent&) {
	doc_manager->go_to_next_page();
	update_status_bar();
}

void main_window::on_previous_heading(wxCommandEvent&) {
	doc_manager->go_to_previous_heading();
	update_status_bar();
}

void main_window::on_next_heading(wxCommandEvent&) {
	doc_manager->go_to_next_heading();
	update_status_bar();
}

void main_window::on_previous_heading_1(wxCommandEvent&) {
	doc_manager->go_to_previous_heading(1);
	update_status_bar();
}

void main_window::on_next_heading_1(wxCommandEvent&) {
	doc_manager->go_to_next_heading(1);
	update_status_bar();
}

void main_window::on_previous_heading_2(wxCommandEvent&) {
	doc_manager->go_to_previous_heading(2);
	update_status_bar();
}

void main_window::on_next_heading_2(wxCommandEvent&) {
	doc_manager->go_to_next_heading(2);
	update_status_bar();
}

void main_window::on_previous_heading_3(wxCommandEvent&) {
	doc_manager->go_to_previous_heading(3);
	update_status_bar();
}

void main_window::on_next_heading_3(wxCommandEvent&) {
	doc_manager->go_to_next_heading(3);
	update_status_bar();
}

void main_window::on_previous_heading_4(wxCommandEvent&) {
	doc_manager->go_to_previous_heading(4);
	update_status_bar();
}

void main_window::on_next_heading_4(wxCommandEvent&) {
	doc_manager->go_to_next_heading(4);
	update_status_bar();
}

void main_window::on_previous_heading_5(wxCommandEvent&) {
	doc_manager->go_to_previous_heading(5);
	update_status_bar();
}

void main_window::on_next_heading_5(wxCommandEvent&) {
	doc_manager->go_to_next_heading(5);
	update_status_bar();
}

void main_window::on_previous_heading_6(wxCommandEvent&) {
	doc_manager->go_to_previous_heading(6);
	update_status_bar();
}

void main_window::on_next_heading_6(wxCommandEvent&) {
	doc_manager->go_to_next_heading(6);
	update_status_bar();
}

void main_window::on_word_count(wxCommandEvent&) {
	const size_t count = doc_manager->get_active_document()->stats.word_count;
	wxMessageBox(wxString::Format("The document contains %d %s", count, count == 1 ? "word" : "words"), "Word count", wxICON_INFORMATION);
}

void main_window::on_doc_info(wxCommandEvent&) {
	doc_manager->show_document_info(this);
}

void main_window::on_toc(wxCommandEvent&) {
	doc_manager->show_table_of_contents(this);
	update_status_bar();
}

void main_window::on_options(wxCommandEvent&) {
	auto& config_mgr = wxGetApp().get_config_manager();
	wxTextCtrl* active_text_ctrl = doc_manager->get_active_text_ctrl();
	options_dialog dlg(this);
	dlg.set_restore_previous_documents(config_mgr.get_restore_previous_documents());
	dlg.set_word_wrap(config_mgr.get_word_wrap());
	if (dlg.ShowModal() != wxID_OK) return;
	bool old_word_wrap = config_mgr.get_word_wrap();
	bool new_word_wrap = dlg.get_word_wrap();
	config_mgr.set_restore_previous_documents(dlg.get_restore_previous_documents());
	config_mgr.set_word_wrap(new_word_wrap);
	if (old_word_wrap != new_word_wrap) {
		doc_manager->apply_word_wrap(new_word_wrap);
		if (active_text_ctrl && doc_manager->get_active_text_ctrl()) doc_manager->get_active_text_ctrl()->SetFocus();
	}
	config_mgr.flush();
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

void main_window::on_help_internal(wxCommandEvent&) {
	const auto path = wxFileName(wxStandardPaths::Get().GetExecutablePath()).GetPath();
	const auto readme_path = wxFileName(path, "readme.html").GetFullPath();
	if (!wxFileName::FileExists(readme_path)) {
		wxMessageBox("readme.html not found. Please ensure the application was built properly.", "Error", wxICON_ERROR);
		return;
	}
	wxGetApp().open_file(readme_path);
	auto* const text_ctrl = doc_manager->get_active_text_ctrl();
	if (text_ctrl) {
		text_ctrl->Bind(wxEVT_LEFT_UP, &main_window::on_text_cursor_changed, this);
		text_ctrl->Bind(wxEVT_KEY_UP, &main_window::on_text_cursor_changed, this);
	}
	update_title();
	update_status_bar();
	update_ui();
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
	event.Skip();
}

void main_window::on_position_save_timer(wxTimerEvent&) {
	doc_manager->save_current_tab_position();
}

void main_window::on_recent_document(wxCommandEvent& event) {
	const int id = event.GetId();
	const int index = id - ID_RECENT_DOCUMENTS_BASE;
	auto& config_mgr = wxGetApp().get_config_manager();
	const wxArrayString recent_docs = config_mgr.get_recent_documents();
	if (index >= 0 && index < static_cast<int>(recent_docs.GetCount())) {
		const wxString& path = recent_docs[index];
		if (!wxFileName::FileExists(path)) {
			wxMessageBox("File no longer exists: " + path, "Error", wxICON_ERROR);
			update_recent_documents_menu();
			return;
		}
		wxGetApp().open_file(path);
		auto* const text_ctrl = doc_manager->get_active_text_ctrl();
		if (text_ctrl) {
			text_ctrl->Bind(wxEVT_LEFT_UP, &main_window::on_text_cursor_changed, this);
			text_ctrl->Bind(wxEVT_KEY_UP, &main_window::on_text_cursor_changed, this);
		}
		config_mgr.add_recent_document(path);
		update_recent_documents_menu();
		update_title();
		update_status_bar();
		update_ui();
	}
}

void main_window::update_recent_documents_menu() {
	if (!recent_documents_menu) return;
	while (recent_documents_menu->GetMenuItemCount() > 0) {
		wxMenuItem* item = recent_documents_menu->FindItemByPosition(0);
		if (item) {
			Unbind(wxEVT_MENU, &main_window::on_recent_document, this, item->GetId());
			recent_documents_menu->Delete(item);
		}
	}
	auto& config_mgr = wxGetApp().get_config_manager();
	const wxArrayString recent_docs = config_mgr.get_recent_documents();
	if (recent_docs.IsEmpty()) {
		recent_documents_menu->Append(wxID_ANY, "(No recent documents)")->Enable(false);
		return;
	}
	for (size_t i = 0; i < recent_docs.GetCount() && i < 10; ++i) {
		const wxString& path = recent_docs[i];
		const wxString filename = wxFileName(path).GetFullName();
		const wxString menu_text = wxString::Format("&%zu %s", i + 1, filename);
		const int id = ID_RECENT_DOCUMENTS_BASE + static_cast<int>(i);
		recent_documents_menu->Append(id, menu_text, path);
		Bind(wxEVT_MENU, &main_window::on_recent_document, this, id);
	}
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
