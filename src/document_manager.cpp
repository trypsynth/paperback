/* document_manager.cpp - manages documents and helps bridge them to the main window.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#include "document_manager.hpp"
#include "app.hpp"
#include "constants.hpp"
#include "dialogs.hpp"
#include "main_window.hpp"
#include "parser.hpp"
#include "utils.hpp"
#include <wx/config.h>
#include <wx/filename.h>
#include <wx/menu.h>
#include <wx/notebook.h>
#include <wx/panel.h>
#include <wx/textctrl.h>

document_manager::document_manager(wxNotebook* nbk, config_manager& cfg, main_window& win) : notebook{nbk}, config{cfg}, main_win{win} {}

document_manager::~document_manager() {
	save_all_tab_positions();
}

bool document_manager::open_file(const wxString& path, bool add_to_recent) {
	if (!wxFileName::FileExists(path)) {
		wxMessageBox("File not found: " + path, "Error", wxICON_ERROR);
		return false;
	}
	const int existing_tab = find_tab_by_path(path);
	if (existing_tab >= 0) {
		notebook->SetSelection(existing_tab);
		auto* const text_ctrl = get_active_text_ctrl();
		if (text_ctrl) text_ctrl->SetFocus();
		return true;
	}
	auto* par = find_parser_by_extension(wxFileName(path).GetExt());
	if (!par) {
		par = get_parser_for_unknown_file(path, config);
		if (!par) return false;
	}
	if (!create_document_tab(path, par)) {
		wxMessageBox("Failed to load document.", "Error", wxICON_ERROR);
		return false;
	}
	auto* const text_ctrl = get_active_text_ctrl();
	if (text_ctrl) {
		text_ctrl->Bind(wxEVT_KEY_UP, &main_window::on_text_cursor_changed, &main_win);
	}
	if (add_to_recent) {
		config.add_recent_document(path);
	}
	update_ui();
	return true;
}

bool document_manager::create_document_tab(const wxString& path, const parser* par, bool set_focus) {
	std::unique_ptr<document> doc = par->load(path);
	if (!doc) return false;
	doc->calculate_statistics();
	auto* tab_data = new document_tab;
	tab_data->doc = std::move(doc);
	tab_data->file_path = path;
	tab_data->parser = par;
	wxPanel* panel = create_tab_panel(tab_data->doc->buffer.str(), tab_data);
	tab_data->panel = panel;
	notebook->AddPage(panel, tab_data->doc->title, true);
	restore_document_position(tab_data);
	if (set_focus) tab_data->text_ctrl->SetFocus();
	config.add_recent_document(path);
	config.set_document_opened(path, true);
	return true;
}

void document_manager::update_ui() {
	main_win.update_recent_documents_menu();
	main_win.update_title();
	main_win.update_status_bar();
	main_win.update_ui();
}

void document_manager::close_document(int index) {
	if (index < 0 || index >= get_tab_count()) return;
	document_tab* tab = get_tab(index);
	if (tab && tab->text_ctrl) {
		long position = tab->text_ctrl->GetInsertionPoint();
		save_document_position(tab->file_path, position);
		config.set_document_opened(tab->file_path, false);
	}
	notebook->DeletePage(index);
}

void document_manager::close_all_documents() {
	save_all_tab_positions();
	for (int i = 0; i < get_tab_count(); ++i) {
		document_tab* tab = get_tab(i);
		if (tab) config.set_document_opened(tab->file_path, false);
	}
	notebook->DeleteAllPages();
}

bool document_manager::export_document(int index, const wxString& export_path) {
	document_tab* tab = get_tab(index);
	if (!tab || !tab->text_ctrl) return false;
	wxFile file;
	if (!file.Open(export_path, wxFile::write)) return false;
	file.Write(tab->text_ctrl->GetValue());
	file.Close();
	return true;
}

document_tab* document_manager::get_tab(int index) const {
	if (index < 0 || index >= get_tab_count()) return nullptr;
	wxPanel* panel = static_cast<wxPanel*>(notebook->GetPage(index));
	return static_cast<document_tab*>(panel->GetClientObject());
}

document_tab* document_manager::get_active_tab() const {
	int selection = notebook->GetSelection();
	return selection >= 0 ? get_tab(selection) : nullptr;
}

document* document_manager::get_active_document() const {
	document_tab* tab = get_active_tab();
	return tab ? tab->doc.get() : nullptr;
}

wxTextCtrl* document_manager::get_active_text_ctrl() const {
	document_tab* tab = get_active_tab();
	return tab ? tab->text_ctrl : nullptr;
}

const parser* document_manager::get_active_parser() const {
	document_tab* tab = get_active_tab();
	return tab ? tab->parser : nullptr;
}

int document_manager::get_tab_count() const {
	return notebook->GetPageCount();
}

int document_manager::get_active_tab_index() const {
	return notebook->GetSelection();
}

void document_manager::go_to_position(long position) {
	wxTextCtrl* text_ctrl = get_active_text_ctrl();
	if (!text_ctrl) return;
	long max_pos = text_ctrl->GetLastPosition();
	if (position > max_pos) position = max_pos;
	if (position < 0) position = 0;
	text_ctrl->SetInsertionPoint(position);
	text_ctrl->ShowPosition(position);
}

void document_manager::go_to_previous_section() {
	document* doc = get_active_document();
	wxTextCtrl* text_ctrl = get_active_text_ctrl();
	const parser* par = get_active_parser();
	if (!doc || !text_ctrl || !par) return;
	if (!par->has_flag(parser_flags::supports_sections)) {
		speak("No sections.");
		return;
	}
	size_t current_pos = text_ctrl->GetInsertionPoint();
	int current_index = doc->section_index(current_pos);
	if (current_index != -1) {
		size_t current_section_offset = doc->offset_for_section(current_index);
		if (current_pos > current_section_offset) {
			text_ctrl->SetInsertionPoint(current_section_offset);
			long line;
			text_ctrl->PositionToXY(current_section_offset, 0, &line);
			wxString current_line = text_ctrl->GetLineText(line);
			speak(current_line);
			return;
		}
	}
	size_t search_pos = current_pos;
	if (current_index != -1) {
		size_t current_section_offset = doc->offset_for_section(current_index);
		if (current_pos <= current_section_offset) {
			// We're at the start of the current section, so search from just before the section marker.
			search_pos = current_section_offset > 0 ? current_section_offset - 1 : 0;
		}
	}
	int prev_index = doc->previous_section_index(search_pos);
	if (prev_index == -1) {
		speak("No previous section");
		return;
	}
	size_t offset = doc->offset_for_section(prev_index);
	text_ctrl->SetInsertionPoint(offset);
	long line;
	text_ctrl->PositionToXY(offset, 0, &line);
	wxString current_line = text_ctrl->GetLineText(line);
	speak(current_line);
}

void document_manager::go_to_next_section() {
	document* doc = get_active_document();
	wxTextCtrl* text_ctrl = get_active_text_ctrl();
	const parser* par = get_active_parser();
	if (!doc || !text_ctrl || !par) return;
	if (!par->has_flag(parser_flags::supports_sections)) {
		speak("No sections.");
		return;
	}
	size_t current_pos = text_ctrl->GetInsertionPoint();
	int next_index = doc->next_section_index(current_pos);
	if (next_index == -1) {
		speak("No next section");
		return;
	}
	size_t offset = doc->offset_for_section(next_index);
	text_ctrl->SetInsertionPoint(offset);
	long line;
	text_ctrl->PositionToXY(offset, 0, &line);
	wxString current_line = text_ctrl->GetLineText(line);
	speak(current_line);
}

void document_manager::go_to_previous_heading() {
	navigate_to_heading(false);
}

void document_manager::go_to_next_heading() {
	navigate_to_heading(true);
}

void document_manager::go_to_previous_heading(int level) {
	navigate_to_heading(false, level);
}

void document_manager::go_to_next_heading(int level) {
	navigate_to_heading(true, level);
}

void document_manager::go_to_previous_page() {
	document* doc = get_active_document();
	wxTextCtrl* text_ctrl = get_active_text_ctrl();
	if (!doc || !text_ctrl) return;
	if (doc->buffer.count_markers_by_type(marker_type::page_break) == 0) {
		speak("No pages.");
		return;
	}
	size_t current_pos = text_ctrl->GetInsertionPoint();
	int prev_index = doc->previous_page_index(current_pos);
	if (prev_index == -1) {
		speak("No previous page.");
		return;
	}
	size_t offset = doc->offset_for_page(prev_index);
	text_ctrl->SetInsertionPoint(offset);
	long line;
	text_ctrl->PositionToXY(offset, 0, &line);
	wxString current_line = text_ctrl->GetLineText(line);
	speak(wxString::Format("Page %d: %s", prev_index + 1, current_line));
}

void document_manager::go_to_next_page() {
	document* doc = get_active_document();
	wxTextCtrl* text_ctrl = get_active_text_ctrl();
	if (!doc || !text_ctrl) return;
	if (doc->buffer.count_markers_by_type(marker_type::page_break) == 0) {
		speak("No pages.");
		return;
	}
	size_t current_pos = text_ctrl->GetInsertionPoint();
	int next_index = doc->next_page_index(current_pos);
	if (next_index == -1) {
		speak("No next page.");
		return;
	}
	size_t offset = doc->offset_for_page(next_index);
	text_ctrl->SetInsertionPoint(offset);
	long line;
	text_ctrl->PositionToXY(offset, 0, &line);
	wxString current_line = text_ctrl->GetLineText(line);
	speak(wxString::Format("Page %d: %s", next_index + 1, current_line));
}

void document_manager::go_to_previous_bookmark() {
	document_tab* tab = get_active_tab();
	wxTextCtrl* text_ctrl = get_active_text_ctrl();
	if (!tab || !text_ctrl) return;
	long current_pos = text_ctrl->GetInsertionPoint();
	long prev_pos = config.get_previous_bookmark(tab->file_path, current_pos);
	if (prev_pos == -1) {
		speak("No previous bookmark");
		return;
	}
	text_ctrl->SetInsertionPoint(prev_pos);
	long line;
	text_ctrl->PositionToXY(prev_pos, 0, &line);
	wxString current_line = text_ctrl->GetLineText(line);
	wxArrayLong bookmarks = config.get_bookmarks(tab->file_path);
	int bookmark_index = bookmarks.Index(prev_pos);
	speak(wxString::Format("Bookmark %d: %s", bookmark_index + 1, current_line));
}

void document_manager::go_to_next_bookmark() {
	document_tab* tab = get_active_tab();
	wxTextCtrl* text_ctrl = get_active_text_ctrl();
	if (!tab || !text_ctrl) return;
	long current_pos = text_ctrl->GetInsertionPoint();
	long next_pos = config.get_next_bookmark(tab->file_path, current_pos);
	if (next_pos == -1) {
		speak("No next bookmark");
		return;
	}
	text_ctrl->SetInsertionPoint(next_pos);
	long line;
	text_ctrl->PositionToXY(next_pos, 0, &line);
	wxString current_line = text_ctrl->GetLineText(line);
	wxArrayLong bookmarks = config.get_bookmarks(tab->file_path);
	int bookmark_index = bookmarks.Index(next_pos);
	speak(wxString::Format("Bookmark %d: %s", bookmark_index + 1, current_line));
}

void document_manager::toggle_bookmark() {
	document_tab* tab = get_active_tab();
	wxTextCtrl* text_ctrl = get_active_text_ctrl();
	if (!tab || !text_ctrl) return;
	long current_pos = text_ctrl->GetInsertionPoint();
	wxArrayLong bookmarks = config.get_bookmarks(tab->file_path);
	bool was_bookmarked = bookmarks.Index(current_pos) != wxNOT_FOUND;
	config.toggle_bookmark(tab->file_path, current_pos);
	config.flush();
	speak(was_bookmarked ? "Bookmark removed" : "Bookmarked");
}

void document_manager::show_bookmark_dialog(wxWindow* parent) {
	document_tab* tab = get_active_tab();
	wxTextCtrl* text_ctrl = get_active_text_ctrl();
	if (!tab || !text_ctrl) return;
	wxArrayLong bookmarks = config.get_bookmarks(tab->file_path);
	if (bookmarks.IsEmpty()) {
		speak("No bookmarks");
		return;
	}
	long current_pos = text_ctrl->GetInsertionPoint();
	bookmark_dialog dialog(parent, bookmarks, text_ctrl, current_pos);
	if (dialog.ShowModal() != wxID_OK) return;
	long pos = dialog.get_selected_position();
	if (pos < 0) return;
	text_ctrl->SetInsertionPoint(pos);
	text_ctrl->SetFocus();
	long line;
	text_ctrl->PositionToXY(pos, 0, &line);
	wxString current_line = text_ctrl->GetLineText(line);
	speak(wxString::Format("Bookmark: %s", current_line));
	update_ui();
}

void document_manager::show_table_of_contents(wxWindow* parent) {
	document* doc = get_active_document();
	wxTextCtrl* text_ctrl = get_active_text_ctrl();
	const parser* par = get_active_parser();
	if (!doc || !text_ctrl || !par) return;
	if (!par->has_flag(parser_flags::supports_toc)) {
		speak("No table of contents.");
		return;
	}
	if (doc->toc_items.empty()) {
		speak("Table of contents is empty.");
		return;
	}
	size_t current_pos = text_ctrl->GetInsertionPoint();
	int closest_toc_offset = doc->find_closest_toc_offset(current_pos);
	toc_dialog dlg(parent, doc, closest_toc_offset);
	if (dlg.ShowModal() != wxID_OK) return;
	int offset = dlg.get_selected_offset();
	if (offset >= 0) {
		go_to_position(offset);
		text_ctrl->SetFocus();
	}
}

void document_manager::show_document_info(wxWindow* parent) {
	document* doc = get_active_document();
	if (!doc) return;
	document_info_dialog dlg(parent, doc);
	dlg.ShowModal();
}

void document_manager::save_document_position(const wxString& path, long position) const {
	config.set_document_position(path, position);
	config.flush();
}

long document_manager::load_document_position(const wxString& path) const {
	return config.get_document_position(path);
}

void document_manager::save_current_tab_position() {
	document_tab* tab = get_active_tab();
	if (!tab || !tab->text_ctrl) return;
	long position = tab->text_ctrl->GetInsertionPoint();
	save_document_position(tab->file_path, position);
}

void document_manager::save_all_tab_positions() {
	for (int i = 0; i < get_tab_count(); ++i) {
		document_tab* tab = get_tab(i);
		if (tab && tab->text_ctrl) {
			long position = tab->text_ctrl->GetInsertionPoint();
			save_document_position(tab->file_path, position);
		}
	}
}

wxString document_manager::get_status_text() const {
	if (!has_documents()) return "Ready";
	wxTextCtrl* text_ctrl = get_active_text_ctrl();
	if (!text_ctrl) return "Ready";
	long current_pos = text_ctrl->GetInsertionPoint();
	long total_chars = text_ctrl->GetLastPosition();
	int percentage = total_chars > 0 ? (current_pos * 100) / total_chars : 0;
	long line;
	text_ctrl->PositionToXY(current_pos, 0, &line);
	long line_number = line + 1;
	long character_number = current_pos + 1;
	return wxString::Format("line %ld, character %ld, reading %d%%", line_number, character_number, percentage);
}

wxString document_manager::get_window_title(const wxString& app_name) const {
	if (!has_documents()) return app_name;
	document* doc = get_active_document();
	return doc ? app_name + " - " + doc->title : app_name;
}

long document_manager::find_text(const wxString& query, long start_pos, find_options options) const {
	wxTextCtrl* text_ctrl = get_active_text_ctrl();
	if (!text_ctrl) return wxNOT_FOUND;
	const wxString& full_text = text_ctrl->GetValue();
	return ::find_text(full_text, query, start_pos, options);
}

void document_manager::apply_word_wrap(bool word_wrap) {
	int active_tab = get_active_tab_index();
	for (int i = 0; i < get_tab_count(); ++i) {
		document_tab* tab = get_tab(i);
		if (tab && tab->text_ctrl && tab->panel) {
			wxTextCtrl* old_ctrl = tab->text_ctrl;
			long current_pos = old_ctrl->GetInsertionPoint();
			wxString content = old_ctrl->GetValue();
			wxSizer* sizer = tab->panel->GetSizer();
			sizer->Detach(old_ctrl);
			old_ctrl->Destroy();
			long style = wxTE_MULTILINE | wxTE_READONLY | wxTE_RICH2 | (word_wrap ? wxTE_WORDWRAP : wxTE_DONTWRAP);
			wxTextCtrl* new_ctrl = new wxTextCtrl(tab->panel, wxID_ANY, "", wxDefaultPosition, wxDefaultSize, style);
			tab->text_ctrl = new_ctrl;
			new_ctrl->Bind(wxEVT_KEY_UP, &main_window::on_text_cursor_changed, &main_win);
			new_ctrl->Freeze();
			new_ctrl->SetValue(content);
			new_ctrl->SetInsertionPoint(current_pos);
			new_ctrl->ShowPosition(current_pos);
			new_ctrl->Thaw();
			sizer->Add(new_ctrl, 1, wxEXPAND | wxALL, 5);
			tab->panel->Layout();
		}
	}
}

int document_manager::find_tab_by_path(const wxString& path) const {
	wxFileName inputFile(path);
	inputFile.Normalize(wxPATH_NORM_ABSOLUTE | wxPATH_NORM_LONG);
	const wxString inputAbsPath = inputFile.GetFullPath();
	for (int i = 0; i < get_tab_count(); ++i) {
		document_tab* tab = get_tab(i);
		if (tab) {
			wxFileName tabFile(tab->file_path);
			tabFile.Normalize(wxPATH_NORM_ABSOLUTE | wxPATH_NORM_LONG);
			if (tabFile.GetFullPath().IsSameAs(inputAbsPath, false)) return i;
		}
	}
	return -1;
}

void document_manager::create_heading_menu(wxMenu* menu) {
	menu->Append(ID_PREVIOUS_HEADING, "Previous heading\tShift+H");
	menu->Append(ID_NEXT_HEADING, "Next heading\tH");
	menu->AppendSeparator();
	for (int level = 1; level <= 6; ++level) {
		menu->Append(ID_PREVIOUS_HEADING_1 + (level - 1) * 2, wxString::Format("Previous heading level %d\tShift+%d", level, level));
		menu->Append(ID_NEXT_HEADING_1 + (level - 1) * 2, wxString::Format("Next heading level %d\t%d", level, level));
	}
}

void document_manager::setup_text_ctrl(wxTextCtrl* text_ctrl, const wxString& content) {
	text_ctrl->Freeze();
	text_ctrl->SetValue(content);
	text_ctrl->Thaw();
}

void document_manager::restore_document_position(document_tab* tab) {
	if (!tab || !tab->text_ctrl) return;
	long saved_position = load_document_position(tab->file_path);
	if (saved_position > 0) {
		long max_position = tab->text_ctrl->GetLastPosition();
		if (saved_position <= max_position) {
			tab->text_ctrl->SetInsertionPoint(saved_position);
			tab->text_ctrl->ShowPosition(saved_position);
		}
	}
}

wxPanel* document_manager::create_tab_panel(const wxString& content, document_tab* tab_data) {
	wxPanel* panel = new wxPanel(notebook, wxID_ANY);
	auto* sizer = new wxBoxSizer(wxVERTICAL);
	bool word_wrap = config.get_word_wrap();
	long style = wxTE_MULTILINE | wxTE_READONLY | wxTE_RICH2 | (word_wrap ? wxTE_WORDWRAP : wxTE_DONTWRAP);
	auto* text_ctrl = new wxTextCtrl(panel, wxID_ANY, "", wxDefaultPosition, wxDefaultSize, style);
	panel->SetClientObject(tab_data);
	tab_data->text_ctrl = text_ctrl;
	text_ctrl->Bind(wxEVT_KEY_UP, &main_window::on_text_cursor_changed, &main_win);
	sizer->Add(text_ctrl, 1, wxEXPAND | wxALL, 5);
	panel->SetSizer(sizer);
	setup_text_ctrl(text_ctrl, content);
	return panel;
}

void document_manager::navigate_to_heading(bool next, int specific_level) {
	document* doc = get_active_document();
	wxTextCtrl* text_ctrl = get_active_text_ctrl();
	if (!doc || !text_ctrl) return;
	if (doc->buffer.get_heading_markers().size() == 0) {
		speak("No headings.");
		return;
	}
	size_t current_pos = text_ctrl->GetInsertionPoint();
	int target_index = -1;
	target_index = next ? doc->next_heading_index(current_pos, specific_level) : doc->previous_heading_index(current_pos, specific_level);
	if (target_index == -1) {
		wxString msg = (specific_level == -1) ? wxString::Format("No %s heading", next ? "next" : "previous") : wxString::Format("No %s heading at level %d", next ? "next" : "previous", specific_level);
		speak(msg);
		return;
	}
	size_t offset = doc->offset_for_heading(target_index);
	text_ctrl->SetInsertionPoint(offset);
	const marker* heading_marker = doc->get_heading_marker(target_index);
	if (heading_marker) speak(wxString::Format("%s Heading level %d", heading_marker->text, heading_marker->level));
}
