/* main_window.hpp - primary user interface header file.
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
#include "live_region.hpp"
#include "parser.hpp"
#include "translation_manager.hpp"
#include "utils.hpp"
#include <wx/aboutdlg.h>
#include <wx/filename.h>
#include <wx/stdpaths.h>
#include <wx/timer.h>

main_window::main_window() : wxFrame(nullptr, wxID_ANY, APP_NAME) {
	task_bar_icon_ = new task_bar_icon(this);
	auto* const panel = new wxPanel(this);
	notebook = new wxNotebook(panel, wxID_ANY);
#ifdef __WXMSW__
	notebook->MSWDisableComposited();
#endif
	live_region_label = new wxStaticText(panel, wxID_ANY, "", wxDefaultPosition, wxSize(0, 0));
	live_region_label->Hide();
	[[maybe_unused]] bool live_region_set = set_live_region(live_region_label);
	auto* const sizer = new wxBoxSizer(wxVERTICAL);
	sizer->Add(notebook, 1, wxEXPAND | wxALL, 10);
	panel->SetSizer(sizer);
	doc_manager = std::make_unique<document_manager>(notebook, wxGetApp().get_config_manager(), *this);
	create_menus();
	status_bar = CreateStatusBar(1);
	status_bar->SetStatusText(_("Ready"));
	position_save_timer = new wxTimer(this);
	status_update_timer = new wxTimer(this);
	bind_events();
	update_ui();
	notebook->Bind(wxEVT_KEY_DOWN, &main_window::on_notebook_key_down, this);
}

main_window::~main_window() {
	if (task_bar_icon_) {
		task_bar_icon_->Destroy();
		task_bar_icon_ = nullptr;
	}
	if (position_save_timer) {
		position_save_timer->Stop();
		position_save_timer = nullptr;
	}
	if (status_update_timer) {
		status_update_timer->Stop();
		status_update_timer = nullptr;
	}
	if (find_dlg) {
		find_dlg->Destroy();
		find_dlg = nullptr;
	}
	doc_manager.reset();
}

void main_window::create_menus() {
	auto* const menu_bar = new wxMenuBar();
	menu_bar->Append(create_file_menu(), _("&File"));
	menu_bar->Append(create_go_menu(), _("&Go"));
	menu_bar->Append(create_tools_menu(), _("&Tools"));
	menu_bar->Append(create_help_menu(), _("&Help"));
	SetMenuBar(menu_bar);
}

wxMenu* main_window::create_file_menu() {
	auto* const menu = new wxMenu();
	menu->Append(wxID_OPEN, _("&Open...\tCtrl+O"));
	menu->Append(wxID_CLOSE, _("Close\tCtrl+F4"));
	menu->Append(wxID_CLOSE_ALL, _("Close &All\tCtrl+Shift+F4"));
	menu->AppendSeparator();
	recent_documents_menu = new wxMenu();
	menu->AppendSubMenu(recent_documents_menu, _("&Recent Documents"));
	update_recent_documents_menu();
	menu->AppendSeparator();
	menu->Append(ID_EXPORT, _("&Export...\tCtrl+E"));
	menu->AppendSeparator();
	menu->Append(wxID_EXIT, _("E&xit"));
	return menu;
}

wxMenu* main_window::create_go_menu() {
	auto* const menu = new wxMenu();
	auto& config_mgr = wxGetApp().get_config_manager();
	bool compact = config_mgr.get_compact_go_menu();
	menu->Append(wxID_FIND, _("&Find...\tCtrl+F"));
	menu->Append(ID_FIND_NEXT, _("Find Ne&xt\tF3"));
	menu->Append(ID_FIND_PREVIOUS, _("Find P&revious\tShift+F3"));
	menu->AppendSeparator();
	menu->Append(ID_GO_TO_LINE, _("Go to &line...\tCtrl+G"));
	menu->Append(ID_GO_TO_PERCENT, _("Go to &percent...\tCtrl+Shift+G"));
	menu->AppendSeparator();
	if (compact) {
		auto* sections_menu = new wxMenu();
		sections_menu->Append(ID_PREVIOUS_SECTION, _("Previous section\t["));
		sections_menu->Append(ID_NEXT_SECTION, _("Next section\t]"));
		menu->AppendSubMenu(sections_menu, _("&Sections"));
		auto* headings_menu = new wxMenu();
		document_manager::create_heading_menu(headings_menu);
		menu->AppendSubMenu(headings_menu, _("&Headings"));
		auto* pages_menu = new wxMenu();
		pages_menu->Append(ID_GO_TO_PAGE, _("Go to &page...\tCtrl+P"));
		pages_menu->AppendSeparator();
		pages_menu->Append(ID_PREVIOUS_PAGE, _("Previous &page\tShift+P"));
		pages_menu->Append(ID_NEXT_PAGE, _("&Next page\tP"));
		menu->AppendSubMenu(pages_menu, _("&Pages"));
		auto* bookmarks_menu = new wxMenu();
		bookmarks_menu->Append(ID_PREVIOUS_BOOKMARK, _("Previous &bookmark\tShift+B"));
		bookmarks_menu->Append(ID_NEXT_BOOKMARK, _("Next b&ookmark\tB"));
		bookmarks_menu->Append(ID_TOGGLE_BOOKMARK, _("Toggle bookmark\tCtrl+Shift+B"));
		bookmarks_menu->Append(ID_JUMP_TO_BOOKMARK, _("Jump to bookmark...\tCtrl+B"));
		menu->AppendSubMenu(bookmarks_menu, _("&Bookmarks"));
		auto* links_menu = new wxMenu();
		links_menu->Append(ID_PREVIOUS_LINK, _("Previous lin&k\tShift+K"));
		links_menu->Append(ID_NEXT_LINK, _("Next lin&k\tK"));
		menu->AppendSubMenu(links_menu, _("&Links"));
		auto* tables_menu = new wxMenu();
		tables_menu->Append(ID_PREVIOUS_TABLE, _("Previous &table\tShift+T"));
		tables_menu->Append(ID_NEXT_TABLE, _("Next &table\tT"));
		menu->AppendSubMenu(tables_menu, _("&Tables"));
	} else {
		menu->Append(ID_GO_TO_PAGE, _("Go to &page...\tCtrl+P"));
		menu->AppendSeparator();
		menu->Append(ID_PREVIOUS_SECTION, _("Previous section\t["));
		menu->Append(ID_NEXT_SECTION, _("Next section\t]"));
		menu->AppendSeparator();
		document_manager::create_heading_menu(menu);
		menu->AppendSeparator();
		menu->Append(ID_PREVIOUS_PAGE, _("Previous &page\tShift+P"));
		menu->Append(ID_NEXT_PAGE, _("&Next page\tP"));
		menu->AppendSeparator();
		menu->Append(ID_PREVIOUS_BOOKMARK, _("Previous &bookmark\tShift+B"));
		menu->Append(ID_NEXT_BOOKMARK, _("Next b&ookmark\tB"));
		menu->Append(ID_TOGGLE_BOOKMARK, _("Toggle bookmark\tCtrl+Shift+B"));
		menu->Append(ID_JUMP_TO_BOOKMARK, _("Jump to bookmark...\tCtrl+B"));
		menu->AppendSeparator();
		menu->Append(ID_PREVIOUS_LINK, _("Previous lin&k\tShift+K"));
		menu->Append(ID_NEXT_LINK, _("Next lin&k\tK"));
		menu->AppendSeparator();
		menu->Append(ID_PREVIOUS_TABLE, _("Previous &table\tShift+T"));
		menu->Append(ID_NEXT_TABLE, _("Next &table\tT"));
	}
	return menu;
}

wxMenu* main_window::create_tools_menu() {
	auto* const menu = new wxMenu();
	menu->Append(ID_WORD_COUNT, _("&Word count\tCtrl+W"));
	menu->Append(ID_DOC_INFO, _("Document &info\tCtrl+I"));
	menu->AppendSeparator();
	menu->Append(ID_TABLE_OF_CONTENTS, _("Table of contents\tCtrl+T"));
	menu->AppendSeparator();
	menu->Append(ID_OPTIONS, _("&Options\tCtrl+,"));
	return menu;
}

wxMenu* main_window::create_help_menu() {
	auto* const menu = new wxMenu();
	menu->Append(wxID_ABOUT, wxString::Format(_("About %s\tCtrl+F1"), APP_NAME));
	menu->Append(wxID_HELP, _("View &help in default browser\tF1"));
	menu->Append(ID_HELP_INTERNAL, wxString::Format(_("View Help in %s\tShift+F1"), APP_NAME));
	menu->AppendSeparator();
	menu->Append(ID_DONATE, _("&Donate\tCtrl+D"));
	return menu;
}

void main_window::refresh_ui_language() {
	wxMenuBar* old_menu_bar = GetMenuBar();
	create_menus();
	delete old_menu_bar;
	update_status_bar();
	update_title();
	update_recent_documents_menu();
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
		{ID_GO_TO_LINE, &main_window::on_go_to_line},
		{ID_GO_TO_PERCENT, &main_window::on_go_to_percent},
		{ID_GO_TO_PAGE, &main_window::on_go_to_page},
		{ID_PREVIOUS_SECTION, &main_window::on_previous_section},
		{ID_NEXT_SECTION, &main_window::on_next_section},
		{ID_PREVIOUS_HEADING, &main_window::on_previous_heading},
		{ID_NEXT_HEADING, &main_window::on_next_heading},
		{ID_PREVIOUS_PAGE, &main_window::on_previous_page},
		{ID_NEXT_PAGE, &main_window::on_next_page},
		{ID_PREVIOUS_BOOKMARK, &main_window::on_previous_bookmark},
		{ID_NEXT_BOOKMARK, &main_window::on_next_bookmark},
		{ID_TOGGLE_BOOKMARK, &main_window::on_toggle_bookmark},
		{ID_JUMP_TO_BOOKMARK, &main_window::on_jump_to_bookmark},
		{ID_PREVIOUS_LINK, &main_window::on_previous_link},
		{ID_NEXT_LINK, &main_window::on_next_link},
		{ID_PREVIOUS_TABLE, &main_window::on_previous_table},
		{ID_NEXT_TABLE, &main_window::on_next_table},
		{ID_WORD_COUNT, &main_window::on_word_count},
		{ID_DOC_INFO, &main_window::on_doc_info},
		{ID_TABLE_OF_CONTENTS, &main_window::on_toc},
		{ID_OPTIONS, &main_window::on_options},
		{wxID_ABOUT, &main_window::on_about},
		{wxID_HELP, &main_window::on_help},
		{ID_HELP_INTERNAL, &main_window::on_help_internal},
		{ID_DONATE, &main_window::on_donate},
	};
	for (const auto& [id, handler] : menu_bindings)
		Bind(wxEVT_MENU, handler, this, id);
	for (int level = 1; level <= 6; ++level) {
		const int prev_id = ID_PREVIOUS_HEADING_1 + (level - 1) * 2;
		const int next_id = ID_NEXT_HEADING_1 + (level - 1) * 2;
		Bind(wxEVT_MENU, [this, level](wxCommandEvent&) { navigate_heading_by_level(level, false); }, prev_id);
		Bind(wxEVT_MENU, [this, level](wxCommandEvent&) { navigate_heading_by_level(level, true); }, next_id);
	}
	Bind(wxEVT_NOTEBOOK_PAGE_CHANGED, &main_window::on_notebook_page_changed, this);
	Bind(wxEVT_CLOSE_WINDOW, &main_window::on_close_window, this);
	Bind(wxEVT_ICONIZE, &main_window::on_iconize, this);
	Bind(wxEVT_TIMER, &main_window::on_position_save_timer, this, position_save_timer->GetId());
	Bind(wxEVT_TIMER, &main_window::on_status_update_timer, this, status_update_timer->GetId());
}

void main_window::on_iconize(wxIconizeEvent& event) {
	if (event.IsIconized()) {
		auto& config_mgr = wxGetApp().get_config_manager();
		if (config_mgr.get_minimize_to_tray()) {
			Hide();
			task_bar_icon_->SetIcon(wxICON(wxICON_INFORMATION), APP_NAME);
		}
	}
	event.Skip();
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
		ID_GO_TO_LINE,
		ID_GO_TO_PERCENT,
		ID_GO_TO_PAGE,
		ID_PREVIOUS_SECTION,
		ID_NEXT_SECTION,
		ID_PREVIOUS_HEADING,
		ID_NEXT_HEADING,
		ID_PREVIOUS_HEADING_1,
		ID_NEXT_HEADING_1,
		ID_PREVIOUS_HEADING_2,
		ID_NEXT_HEADING_2,
		ID_PREVIOUS_HEADING_3,
		ID_NEXT_HEADING_3,
		ID_PREVIOUS_HEADING_4,
		ID_NEXT_HEADING_4,
		ID_PREVIOUS_HEADING_5,
		ID_NEXT_HEADING_5,
		ID_PREVIOUS_HEADING_6,
		ID_NEXT_HEADING_6,
		ID_PREVIOUS_PAGE,
		ID_NEXT_PAGE,
		ID_PREVIOUS_BOOKMARK,
		ID_NEXT_BOOKMARK,
		ID_TOGGLE_BOOKMARK,
		ID_JUMP_TO_BOOKMARK,
		ID_PREVIOUS_LINK,
		ID_NEXT_LINK,
		ID_PREVIOUS_TABLE,
		ID_NEXT_TABLE,
		ID_WORD_COUNT,
		ID_DOC_INFO,
		ID_TABLE_OF_CONTENTS,
	};
	for (const auto id : doc_items)
		enable(id, has_doc);
}

void main_window::update_title() {
	SetTitle(doc_manager->get_window_title(APP_NAME));
}

void main_window::update_status_bar() {
	status_bar->SetStatusText(doc_manager->get_status_text());
}

void main_window::on_open(wxCommandEvent&) {
	wxFileDialog dlg(this, _("Select a document to read"), "", "", get_supported_wildcards(), wxFD_OPEN | wxFD_FILE_MUST_EXIST);
	if (dlg.ShowModal() != wxID_OK) {
		return;
	}
	const auto path = dlg.GetPath();
	[[maybe_unused]] bool success = doc_manager->open_file(path);
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
	if (!doc) {
		return;
	}
	wxFileDialog save_dialog(this, _("Export Document"), "", doc->title + ".txt", _("Text files (*.txt)|*.txt|All files (*.*)|*.*"), wxFD_SAVE | wxFD_OVERWRITE_PROMPT);
	if (save_dialog.ShowModal() != wxID_OK) {
		return;
	}
	const auto file_path = save_dialog.GetPath();
	if (!doc_manager->export_document(doc_manager->get_active_tab_index(), file_path)) {
		wxMessageBox(_("Failed to export document."), _("Error"), wxICON_ERROR);
	}
}

void main_window::on_exit(wxCommandEvent&) {
	Close(true);
}

void main_window::on_find(wxCommandEvent&) {
	if (!find_dlg) {
		find_dlg = new find_dialog(this);
	}
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
	if (find_dlg && find_dlg->IsShown()) {
		do_find(true);
	} else {
		wxCommandEvent evt{};
		on_find(evt);
	}
}

void main_window::on_find_previous(wxCommandEvent&) {
	if (find_dlg && find_dlg->IsShown()) {
		do_find(false);
	} else {
		wxCommandEvent evt{};
		on_find(evt);
	}
}

void main_window::on_go_to_line(wxCommandEvent&) {
	auto* const text_ctrl = doc_manager->get_active_text_ctrl();
	if (!text_ctrl) {
		return;
	}
	go_to_line_dialog dlg(this, text_ctrl);
	if (dlg.ShowModal() != wxID_OK) {
		return;
	}
	const auto pos = dlg.get_position();
	doc_manager->go_to_position(pos);
	update_status_bar();
	save_position_immediately();
}

void main_window::on_go_to_percent(wxCommandEvent&) {
	auto* const text_ctrl = doc_manager->get_active_text_ctrl();
	if (!text_ctrl) {
		return;
	}
	go_to_percent_dialog dlg(this, text_ctrl);
	if (dlg.ShowModal() != wxID_OK) {
		return;
	}
	const auto pos = dlg.get_position();
	doc_manager->go_to_position(pos);
	update_status_bar();
	save_position_immediately();
}

void main_window::on_go_to_page(wxCommandEvent&) {
	auto* const doc = doc_manager->get_active_document();
	auto* const par = doc_manager->get_active_parser();
	if (!doc || !par) {
		return;
	}
	if (!par->has_flag(parser_flags::supports_pages)) {
		speak(_("No pages."));
		return;
	}
	int current_page = 1;
	auto* const text_ctrl = doc_manager->get_active_text_ctrl();
	if (!text_ctrl) {
		return;
	}
	const size_t current_pos = text_ctrl->GetInsertionPoint();
	const int current_page_idx = doc->page_index(current_pos);
	if (current_page_idx >= 0) {
		current_page = current_page_idx + 1; // Convert to 1-based index
	}
	go_to_page_dialog dlg(this, doc, par, current_page);
	if (dlg.ShowModal() != wxID_OK) {
		return;
	}
	const int page = dlg.get_page_number();
	if (page >= 1 && page <= static_cast<int>(doc->buffer.count_markers_by_type(marker_type::page_break))) {
		const size_t offset = doc->buffer.get_marker_position_by_index(marker_type::page_break, page - 1); // Convert to 0-based index
		doc_manager->go_to_position(offset);
		update_status_bar();
		save_position_immediately();
	}
}

void main_window::on_previous_section(wxCommandEvent&) {
	doc_manager->go_to_previous_section();
	update_status_bar();
	trigger_throttled_position_save();
}

void main_window::on_next_section(wxCommandEvent&) {
	doc_manager->go_to_next_section();
	update_status_bar();
	trigger_throttled_position_save();
}

void main_window::on_previous_page(wxCommandEvent&) {
	doc_manager->go_to_previous_page();
	update_status_bar();
	trigger_throttled_position_save();
}

void main_window::on_next_page(wxCommandEvent&) {
	doc_manager->go_to_next_page();
	update_status_bar();
	trigger_throttled_position_save();
}

void main_window::on_previous_bookmark(wxCommandEvent&) {
	doc_manager->go_to_previous_bookmark();
	update_status_bar();
	trigger_throttled_position_save();
}

void main_window::on_next_bookmark(wxCommandEvent&) {
	doc_manager->go_to_next_bookmark();
	update_status_bar();
	trigger_throttled_position_save();
}

void main_window::on_toggle_bookmark(wxCommandEvent&) {
	doc_manager->toggle_bookmark();
}

void main_window::on_jump_to_bookmark(wxCommandEvent&) {
	doc_manager->show_bookmark_dialog(this);
	update_status_bar();
	save_position_immediately();
}

void main_window::on_previous_link(wxCommandEvent&) {
	doc_manager->go_to_previous_link();
	update_status_bar();
	trigger_throttled_position_save();
}

void main_window::on_next_link(wxCommandEvent&) {
	doc_manager->go_to_next_link();
	update_status_bar();
	trigger_throttled_position_save();
}

void main_window::on_previous_table(wxCommandEvent&) {
	doc_manager->go_to_previous_table();
	update_status_bar();
	trigger_throttled_position_save();
}

void main_window::on_next_table(wxCommandEvent&) {
	doc_manager->go_to_next_table();
	update_status_bar();
	trigger_throttled_position_save();
}

void main_window::on_previous_heading(wxCommandEvent&) {
	doc_manager->go_to_previous_heading();
	update_status_bar();
	trigger_throttled_position_save();
}

void main_window::on_next_heading(wxCommandEvent&) {
	doc_manager->go_to_next_heading();
	update_status_bar();
	trigger_throttled_position_save();
}

void main_window::on_word_count(wxCommandEvent&) {
	const size_t count = doc_manager->get_active_document()->stats.word_count;
	wxMessageBox(wxString::Format(wxPLURAL("The document contains %d word", "The document contains %d words", count), count), _("Word count"), wxICON_INFORMATION);
}

void main_window::on_doc_info(wxCommandEvent&) {
	doc_manager->show_document_info(this);
}

void main_window::on_toc(wxCommandEvent&) {
	doc_manager->show_table_of_contents(this);
	update_status_bar();
	save_position_immediately();
}

void main_window::on_options(wxCommandEvent&) {
	auto& config_mgr = wxGetApp().get_config_manager();
	wxTextCtrl* active_text_ctrl = doc_manager->get_active_text_ctrl();
	options_dialog dlg(this);
	dlg.set_restore_previous_documents(config_mgr.get_restore_previous_documents());
	dlg.set_word_wrap(config_mgr.get_word_wrap());
	dlg.set_minimize_to_tray(config_mgr.get_minimize_to_tray());
	dlg.set_compact_go_menu(config_mgr.get_compact_go_menu());
	dlg.set_recent_documents_to_show(config_mgr.get_recent_documents_to_show());
	wxString current_language = translation_manager::instance().get_current_language();
	dlg.set_language(current_language);
	if (dlg.ShowModal() != wxID_OK) {
		return;
	}
	bool old_word_wrap = config_mgr.get_word_wrap();
	bool new_word_wrap = dlg.get_word_wrap();
	bool old_compact_menu = config_mgr.get_compact_go_menu();
	bool new_compact_menu = dlg.get_compact_go_menu();
	wxString new_language = dlg.get_language();
	config_mgr.set_restore_previous_documents(dlg.get_restore_previous_documents());
	config_mgr.set_word_wrap(new_word_wrap);
	config_mgr.set_minimize_to_tray(dlg.get_minimize_to_tray());
	config_mgr.set_compact_go_menu(new_compact_menu);
	config_mgr.set_recent_documents_to_show(dlg.get_recent_documents_to_show());
	config_mgr.set_language(new_language);
	if (old_word_wrap != new_word_wrap) {
		doc_manager->apply_word_wrap(new_word_wrap);
		if (active_text_ctrl && doc_manager->get_active_text_ctrl()) {
			doc_manager->get_active_text_ctrl()->SetFocus();
		}
	}
	if (current_language != new_language || old_compact_menu != new_compact_menu) {
		if (current_language != new_language) {
			translation_manager::instance().set_language(new_language);
		}
		refresh_ui_language();
	}
	config_mgr.flush();
	update_recent_documents_menu();
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
	if (!wxLaunchDefaultBrowser(url)) {
		wxMessageBox(_("Failed to launch default browser."), _("Error"), wxICON_ERROR);
	}
}

void main_window::on_help_internal(wxCommandEvent&) {
	const auto path = wxFileName(wxStandardPaths::Get().GetExecutablePath()).GetPath();
	const auto readme_path = wxFileName(path, "readme.html").GetFullPath();
	if (!wxFileName::FileExists(readme_path)) {
		wxMessageBox(_("readme.html not found. Please ensure the application was built properly."), _("Error"), wxICON_ERROR);
		return;
	}
	[[maybe_unused]] bool success = doc_manager->open_file(readme_path, false);
}

void main_window::on_donate(wxCommandEvent&) {
	const wxString url = "https://paypal.me/tygillespie05";
	if (!wxLaunchDefaultBrowser(url)) {
		wxMessageBox(_("Failed to open donation page in browser."), _("Error"), wxICON_ERROR);
	}
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
	trigger_throttled_status_update();
	trigger_throttled_position_save();
	event.Skip();
}

void main_window::on_text_char(wxKeyEvent& event) {
	if (event.GetKeyCode() == WXK_RETURN) {
		doc_manager->activate_current_table();
		doc_manager->activate_current_link();
	} else {
		event.Skip();
	}
}

void main_window::trigger_throttled_position_save() {
	if (position_save_timer->IsRunning()) {
		position_save_timer->Stop();
	}
	position_save_timer->StartOnce(POSITION_SAVE_THROTTLE_MS);
}

void main_window::trigger_throttled_status_update() {
	wxLongLong current_time = wxGetLocalTimeMillis();
	wxLongLong time_since_last_update = current_time - last_status_update_time;
	const int MIN_UPDATE_INTERVAL_MS = 50;
	if (time_since_last_update >= MIN_UPDATE_INTERVAL_MS) {
		update_status_bar();
		last_status_update_time = current_time;
	} else {
		if (status_update_timer->IsRunning()) {
			status_update_timer->Stop();
		}
		int delay = MIN_UPDATE_INTERVAL_MS - time_since_last_update.ToLong();
		status_update_timer->StartOnce(delay);
	}
}

void main_window::save_position_immediately() {
	doc_manager->save_current_tab_position();
}

void main_window::on_close_window(wxCloseEvent& event) {
	if (doc_manager->has_documents()) {
		auto* active_tab = doc_manager->get_active_tab();
		if (active_tab) {
			auto& config_mgr = wxGetApp().get_config_manager();
			config_mgr.set_active_document(active_tab->file_path);
			config_mgr.flush();
		}
	}
	event.Skip();
}

void main_window::on_position_save_timer(wxTimerEvent&) {
	doc_manager->save_current_tab_position();
}

void main_window::on_status_update_timer(wxTimerEvent&) {
	update_status_bar();
	last_status_update_time = wxGetLocalTimeMillis();
}

void main_window::on_recent_document(wxCommandEvent& event) {
	const int id = event.GetId();
	const int index = id - ID_RECENT_DOCUMENTS_BASE;
	auto& config_mgr = wxGetApp().get_config_manager();
	const wxArrayString recent_docs = config_mgr.get_recent_documents();
	if (index >= 0 && index < static_cast<int>(recent_docs.GetCount())) {
		const wxString& path = recent_docs[index];
		[[maybe_unused]] bool success = doc_manager->open_file(path);
	}
}

void main_window::on_show_all_documents(wxCommandEvent& event) {
	auto& config_mgr = wxGetApp().get_config_manager();
	wxArrayString open_docs;
	for (size_t i = 0; i < doc_manager->get_tab_count(); ++i) {
		if (doc_manager->get_tab(i)) {
			open_docs.Add(doc_manager->get_tab(i)->file_path);
		}
	}
	all_documents_dialog dlg(this, config_mgr, open_docs);
	if (dlg.ShowModal() == wxID_OK) {
		wxString path = dlg.get_selected_path();
		if (!path.IsEmpty() && wxFileName::FileExists(path)) {
			[[maybe_unused]]
			bool success = doc_manager->open_file(path);
		}
	}
	update_recent_documents_menu();
}

void main_window::on_notebook_key_down(wxKeyEvent& event) {
	int key = event.GetKeyCode();
	if (key == WXK_DELETE || key == WXK_NUMPAD_DELETE) {
		if (notebook->FindFocus() == notebook) {
			int sel = notebook->GetSelection();
			if (sel != wxNOT_FOUND) {
				doc_manager->close_document(sel);
				update_title();
				update_status_bar();
				update_ui();
				return;
			}
		}
	}
	event.Skip();
}

void main_window::update_recent_documents_menu() {
	if (!recent_documents_menu) {
		return;
	}
	while (recent_documents_menu->GetMenuItemCount() > 0) {
		wxMenuItem* item = recent_documents_menu->FindItemByPosition(0);
		if (item) {
			Unbind(wxEVT_MENU, &main_window::on_recent_document, this, item->GetId());
			recent_documents_menu->Delete(item);
		}
	}
	auto& config_mgr = wxGetApp().get_config_manager();
	const wxArrayString recent_docs = config_mgr.get_recent_documents();
	size_t menu_count = 0;
	for (size_t i = 0; i < recent_docs.GetCount() && menu_count < config_mgr.get_recent_documents_to_show(); ++i) {
		const wxString& path = recent_docs[i];
		const wxString filename = wxFileName(path).GetFullName();
		const wxString menu_text = wxString::Format("&%zu %s", menu_count + 1, filename);
		const int id = ID_RECENT_DOCUMENTS_BASE + static_cast<int>(i);
		recent_documents_menu->Append(id, menu_text, path);
		Bind(wxEVT_MENU, &main_window::on_recent_document, this, id);
		++menu_count;
	}
	if (menu_count == 0) {
		recent_documents_menu->Append(wxID_ANY, _("(No recent documents)"))->Enable(false);
		return;
	}
	recent_documents_menu->AppendSeparator();
	recent_documents_menu->Append(ID_SHOW_ALL_DOCUMENTS, _("Show All..."));
	Bind(wxEVT_MENU, &main_window::on_show_all_documents, this, ID_SHOW_ALL_DOCUMENTS);
}

void main_window::do_find(bool forward) {
	if (!find_dlg) {
		return;
	}
	auto* const text_ctrl = doc_manager->get_active_text_ctrl();
	if (!text_ctrl) {
		return;
	}
	const auto& query = find_dlg->get_find_text();
	if (query.IsEmpty()) {
		return;
	}
	find_options options = find_options::none;
	if (forward) {
		options |= find_options::forward;
	}
	if (find_dlg->get_match_case()) {
		options |= find_options::match_case;
	}
	if (find_dlg->get_match_whole_word()) {
		options |= find_options::match_whole_word;
	}
	if (find_dlg->get_use_regex()) {
		options |= find_options::use_regex;
	}
	long sel_start, sel_end;
	text_ctrl->GetSelection(&sel_start, &sel_end);
	const long start_pos = forward ? sel_end : sel_start;
	long found_pos = doc_manager->find_text(query, start_pos, options);
	if (found_pos == wxNOT_FOUND) {
		speak(_("No more results. Wrapping search."));
		const auto wrap_pos = forward ? 0 : text_ctrl->GetLastPosition();
		found_pos = doc_manager->find_text(query, wrap_pos, options);
		if (found_pos == wxNOT_FOUND) {
			speak(_("Not found."));
			return;
		}
	}
	text_ctrl->SetFocus();
	text_ctrl->SetSelection(found_pos, found_pos + query.Length());
	text_ctrl->ShowPosition(found_pos);
	update_status_bar();
	trigger_throttled_position_save();
}

void main_window::navigate_heading_by_level(int level, bool forward) {
	forward ? doc_manager->go_to_next_heading(level) : doc_manager->go_to_previous_heading(level);
	update_status_bar();
	trigger_throttled_position_save();
}
