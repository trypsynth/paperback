/* document_manager.cpp - manages documents and helps bridge them to the main window.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#include "document_manager.hpp"
#include "config_manager.hpp"
#include "constants.hpp"
#include "dialogs.hpp"
#include "document.hpp"
#include "document_buffer.hpp"
#include "main_window.hpp"
#include "parser.hpp"
#include "utils.hpp"
#include <algorithm>
#include <cstddef>
#include <iterator>
#include <memory>
#include <optional>
#include <string>
#include <utility>
#include <wx/defs.h>
#include <wx/dynarray.h>
#include <wx/event.h>
#include <wx/filename.h>
#include <wx/msgdlg.h>
#include <wx/panel.h>
#include <wx/sizer.h>
#include <wx/string.h>
#include <wx/textctrl.h>
#include <wx/textdlg.h>
#include <wx/translation.h>
#include <wx/utils.h>
#include <wx/wfstream.h>

document_manager::document_manager(wxNotebook* nbk, config_manager& cfg, main_window& win) : notebook{nbk}, config{cfg}, main_win{win} {
}

document_manager::~document_manager() {
	save_all_tab_positions();
	save_all_tab_navigation_histories();
}

void document_manager::show_parser_error(const parser_exception& e) {
	const wxString title = (e.get_severity() == error_severity::warning) ? _("Warning") : _("Error");
	const long icon = (e.get_severity() == error_severity::warning) ? wxICON_WARNING : wxICON_ERROR;
	wxMessageBox(e.get_display_message(), title, icon);
}

bool document_manager::open_file(const wxString& path, bool add_to_recent) {
	if (!wxFileName::FileExists(path)) {
		wxMessageBox(wxString::Format(_("File not found: %s"), path), _("Error"), wxICON_ERROR);
		return false;
	}
	const int existing_tab = find_tab_by_path(path);
	if (existing_tab >= 0) {
		notebook->SetSelection(existing_tab);
		auto* const text_ctrl = get_active_text_ctrl();
		if (text_ctrl != nullptr) {
			text_ctrl->SetFocus();
		}
		return true;
	}
	const auto* par = find_parser_by_extension(wxFileName(path).GetExt());
	if (par == nullptr) {
		par = get_parser_for_unknown_file(path, config);
		if (par == nullptr) {
			return false;
		}
	}
	if (!create_document_tab(path, par, true, add_to_recent)) {
		return false;
	}
	auto* const text_ctrl = get_active_text_ctrl();
	if (text_ctrl != nullptr) {
		text_ctrl->Bind(wxEVT_KEY_UP, &main_window::on_text_cursor_changed, &main_win);
		text_ctrl->Bind(wxEVT_CHAR, &main_window::on_text_char, &main_win);
	}
	update_ui();
	return true;
}

bool document_manager::create_document_tab(const wxString& path, const parser* par, bool set_focus, bool add_to_recent) {
	config.import_document_settings(path);
	std::unique_ptr<document> doc;
	wxString password_in_use;
	const wxString saved_password = config.get_document_password(path);
	auto load_document = [&](const std::optional<std::string>& password) {
		parser_context ctx;
		ctx.file_path = path;
		if (password.has_value()) {
			ctx.password = password;
		}
		return par->load(ctx);
	};
	try {
		std::optional<std::string> initial_password;
		if (!saved_password.IsEmpty()) {
			password_in_use = saved_password;
			initial_password = saved_password.ToStdString();
		}
		doc = load_document(initial_password);
	} catch (const parser_exception& e) {
		if (e.get_error_code() == parser_error_code::password_required) {
			config.set_document_password(path, wxEmptyString);
			password_dialog dlg(&main_win);
			if (dlg.ShowModal() != wxID_OK) {
				throw parser_exception(_("Password is required."), path);
			}
			const wxString entered_password = dlg.get_password();
			password_in_use = entered_password;
			try {
				doc = load_document(std::optional<std::string>{entered_password.ToStdString()});
			} catch (const parser_exception& retry_e) {
				show_parser_error(retry_e);
				return false;
			} catch (const std::exception& retry_e) {
				wxMessageBox(wxString::Format(_("Failed to parse document: %s"), wxString::FromUTF8(retry_e.what())), _("Error"), wxICON_ERROR);
				return false;
			}
		} else {
			show_parser_error(e);
			return false;
		}
	} catch (const std::exception& e) {
		wxMessageBox(wxString::Format(_("Failed to parse document: %s"), wxString::FromUTF8(e.what())), _("Error"), wxICON_ERROR);
		return false;
	}
	if (!doc) {
		return false;
	}
	if (!password_in_use.IsEmpty()) {
		config.set_document_password(path, password_in_use);
	}
	doc->calculate_statistics();
	config.get_navigation_history(path, doc->history, doc->history_index);
	auto* tab_data = new document_tab;
	tab_data->doc = std::move(doc);
	tab_data->file_path = path;
	tab_data->parser = par;
	wxPanel* panel = create_tab_panel(tab_data->doc->buffer.str(), tab_data);
	tab_data->panel = panel;
	notebook->AddPage(panel, tab_data->doc->title, true);
	restore_document_position(tab_data);
	if (set_focus) {
		tab_data->text_ctrl->SetFocus();
	}
	if (add_to_recent) {
		config.add_recent_document(path);
	}
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
	if (index < 0 || index >= get_tab_count()) {
		return;
	}
	const document_tab* tab = get_tab(index);
	if (tab != nullptr && tab->text_ctrl != nullptr) {
		const int position = tab->text_ctrl->GetInsertionPoint();
		save_document_position(tab->file_path, position);
		config.remove_navigation_history(tab->file_path);
		config.set_document_opened(tab->file_path, false);
	}
	notebook->DeletePage(index);
}

void document_manager::close_all_documents() {
	save_all_tab_positions();
	for (int i = 0; i < get_tab_count(); ++i) {
		const document_tab* tab = get_tab(i);
		if (tab != nullptr) {
			config.set_document_opened(tab->file_path, false);
		}
	}
	notebook->DeleteAllPages();
}

bool document_manager::export_document(int index, const wxString& export_path) const {
	const document_tab* tab = get_tab(index);
	if (tab == nullptr || tab->text_ctrl == nullptr) {
		return false;
	}
	const wxString content = tab->text_ctrl->GetValue();
	const wxCharBuffer buf = content.ToUTF8();
	if (!buf.data()) {
		return false;
	}
	wxFileOutputStream out(export_path);
	if (!out.IsOk()) {
		return false;
	}
	out.Write(buf.data(), buf.length());
	return out.IsOk();
}

document_tab* document_manager::get_tab(int index) const {
	if (index < 0 || index >= get_tab_count()) {
		return nullptr;
	}
	const auto* panel = dynamic_cast<wxPanel*>(notebook->GetPage(index));
	return dynamic_cast<document_tab*>(panel->GetClientObject());
}

document_tab* document_manager::get_active_tab() const {
	const int selection = notebook->GetSelection();
	return selection >= 0 ? get_tab(selection) : nullptr;
}

document* document_manager::get_active_document() const {
	const document_tab* tab = get_active_tab();
	return tab != nullptr ? tab->doc.get() : nullptr;
}

wxTextCtrl* document_manager::get_active_text_ctrl() const {
	const document_tab* tab = get_active_tab();
	return tab != nullptr ? tab->text_ctrl : nullptr;
}

const parser* document_manager::get_active_parser() const {
	const document_tab* tab = get_active_tab();
	return tab != nullptr ? tab->parser : nullptr;
}

int document_manager::get_tab_count() const {
	return static_cast<int>(notebook->GetPageCount());
}

int document_manager::get_active_tab_index() const {
	return notebook->GetSelection();
}

void document_manager::go_to_position(int position) const {
	wxTextCtrl* text_ctrl = get_active_text_ctrl();
	if (text_ctrl == nullptr) {
		return;
	}
	const int max_pos = text_ctrl->GetLastPosition();
	position = std::clamp(position, 0, max_pos);
	text_ctrl->SetInsertionPoint(position);
	text_ctrl->ShowPosition(position);
}

void document_manager::navigate_to_section(bool next) const {
	const document* doc = get_active_document();
	wxTextCtrl* text_ctrl = get_active_text_ctrl();
	const parser* par = get_active_parser();
	if (doc == nullptr || text_ctrl == nullptr || par == nullptr) {
		return;
	}
	if (!par->has_flag(parser_flags::supports_sections)) {
		speak(_("No sections."));
		return;
	}
	const int current_pos = text_ctrl->GetInsertionPoint();
	bool wrapping = false;
	// Special case for previous: if we're past the section start, go to section start first.
	if (!next) {
		const int current_index = doc->section_index(current_pos);
		if (current_index != -1) {
			const size_t current_section_offset = doc->offset_for_section(current_index);
			if (static_cast<size_t>(current_pos) > current_section_offset) {
				text_ctrl->SetInsertionPoint(static_cast<long>(current_section_offset));
				long line{0};
				text_ctrl->PositionToXY(static_cast<long>(current_section_offset), nullptr, &line);
				const wxString current_line = text_ctrl->GetLineText(line);
				speak(current_line);
				return;
			}
		}
	}
	int search_pos = current_pos;
	if (!next) {
		const int current_index = doc->section_index(current_pos);
		if (current_index != -1) {
			const size_t current_section_offset = doc->offset_for_section(current_index);
			if (static_cast<size_t>(current_pos) <= current_section_offset) {
				// We're at the start of the current section, so search from just before the section marker.
				search_pos = current_section_offset > 0 ? static_cast<int>(current_section_offset - 1) : 0;
			}
		}
	}
	int target_index = next ? doc->next_section_index(current_pos) : doc->previous_section_index(search_pos);
	if (target_index == -1) {
		if (config.get(config_manager::navigation_wrap)) {
			target_index = next ? doc->next_section_index(-1) : doc->previous_section_index(text_ctrl->GetLastPosition() + 1);
			if (target_index != -1) {
				wrapping = true;
			}
		}
		if (target_index == -1) {
			speak(next ? _("No next section") : _("No previous section"));
			return;
		}
	}
	const size_t offset = doc->offset_for_section(target_index);
	text_ctrl->SetInsertionPoint(static_cast<long>(offset));
	long line{0};
	text_ctrl->PositionToXY(static_cast<long>(offset), nullptr, &line);
	const wxString current_line = text_ctrl->GetLineText(line);
	if (wrapping) {
		speak((next ? _("Wrapping to start. ") : _("Wrapping to end. ")) + current_line);
	} else {
		speak(current_line);
	}
}

void document_manager::go_to_previous_section() const {
	navigate_to_section(false);
}

void document_manager::go_to_next_section() const {
	navigate_to_section(true);
}

void document_manager::go_to_previous_heading() const {
	navigate_to_heading(false);
}

void document_manager::go_to_next_heading() const {
	navigate_to_heading(true);
}

void document_manager::go_to_previous_heading(int level) const {
	navigate_to_heading(false, level);
}

void document_manager::go_to_next_heading(int level) const {
	navigate_to_heading(true, level);
}

void document_manager::navigate_to_page(bool next) const {
	const document* doc = get_active_document();
	wxTextCtrl* text_ctrl = get_active_text_ctrl();
	if (doc == nullptr || text_ctrl == nullptr) {
		return;
	}
	if (doc->buffer.count_markers_by_type(marker_type::page_break) == 0) {
		speak(_("No pages."));
		return;
	}
	const int current_pos = text_ctrl->GetInsertionPoint();
	bool wrapping = false;
	int target_index = next ? doc->next_page_index(current_pos) : doc->previous_page_index(current_pos);
	if (target_index == -1) {
		if (config.get(config_manager::navigation_wrap)) {
			target_index = next ? doc->next_page_index(-1) : doc->previous_page_index(text_ctrl->GetLastPosition() + 1);
			if (target_index != -1) {
				wrapping = true;
			}
		}
		if (target_index == -1) {
			speak(next ? _("No next page.") : _("No previous page."));
			return;
		}
	}
	const size_t offset = doc->offset_for_page(target_index);
	text_ctrl->SetInsertionPoint(static_cast<long>(offset));
	long line{0};
	text_ctrl->PositionToXY(static_cast<long>(offset), nullptr, &line);
	const wxString current_line = text_ctrl->GetLineText(line);
	wxString message = wxString::Format(_("Page %d: %s"), target_index + 1, current_line);
	if (wrapping) {
		message = (next ? _("Wrapping to start. ") : _("Wrapping to end. ")) + message;
	}
	speak(message);
}

void document_manager::go_to_previous_page() const {
	navigate_to_page(false);
}

void document_manager::go_to_next_page() const {
	navigate_to_page(true);
}

static bool is_candidate_for_filter(const bookmark& bm, bool notes) {
	return notes ? bm.has_note() : !bm.has_note();
}

void document_manager::navigate_to_bookmark(bool next) const {
	const document_tab* tab = get_active_tab();
	wxTextCtrl* text_ctrl = get_active_text_ctrl();
	if (tab == nullptr || text_ctrl == nullptr) {
		return;
	}
	const int current_pos = text_ctrl->GetInsertionPoint();
	const bool allow_wrap = config.get(config_manager::navigation_wrap);
	const auto all = config.get_bookmarks(tab->file_path);
	std::vector<bookmark> list;
	list.reserve(all.size());
	for (const auto& bm : all) {
		if (is_candidate_for_filter(bm, /*notes*/ false)) {
			list.push_back(bm);
		}
	}
	if (list.empty()) {
		speak(_("No bookmarks."));
		return;
	}
	auto find_target = [&](long start_from) -> bookmark {
		if (next) {
			for (const auto& bm : list) {
				if (bm.start > start_from) {
					return bm;
				}
			}
		} else {
			for (auto it = list.rbegin(); it != list.rend(); ++it) {
				if (it->start < start_from) {
					return *it;
				}
			}
		}
		return bookmark{-1, -1};
	};
	bool wrapping = false;
	bookmark target = find_target(current_pos);
	if (target.start == -1 && allow_wrap) {
		wrapping = true;
		target = next ? find_target(-1) : find_target(text_ctrl->GetLastPosition() + 1);
	}
	if (target.start == -1) {
		speak(next ? _("No next bookmark") : _("No previous bookmark"));
		return;
	}
	text_ctrl->SetInsertionPoint(target.start);
	wxString text_to_speak;
	if (target.is_whole_line()) {
		long line{0};
		text_ctrl->PositionToXY(target.start, nullptr, &line);
		text_to_speak = text_ctrl->GetLineText(line);
	} else {
		text_to_speak = text_ctrl->GetRange(target.start, target.end);
	}
	int index{0};
	for (size_t i = 0; i < list.size(); ++i) {
		if (list[i] == target) {
			index = static_cast<int>(i);
			break;
		}
	}
	wxString announcement = wxString::Format(_("%s - Bookmark %d"), text_to_speak, index + 1);
	if (wrapping) {
		announcement = (next ? _("Wrapping to start. ") : _("Wrapping to end. ")) + announcement;
	}
	speak(announcement);
}

void document_manager::navigate_to_note(bool next) const {
	const document_tab* tab = get_active_tab();
	wxTextCtrl* text_ctrl = get_active_text_ctrl();
	if (tab == nullptr || text_ctrl == nullptr) {
		return;
	}
	const int current_pos = text_ctrl->GetInsertionPoint();
	const bool allow_wrap = config.get(config_manager::navigation_wrap);
	const auto all = config.get_bookmarks(tab->file_path);
	std::vector<bookmark> list;
	list.reserve(all.size());
	for (const auto& bm : all) {
		if (is_candidate_for_filter(bm, /*notes*/ true)) {
			list.push_back(bm);
		}
	}
	if (list.empty()) {
		speak(_("No notes."));
		return;
	}
	auto find_target = [&](long start_from) -> bookmark {
		if (next) {
			for (const auto& bm : list) {
				if (bm.start > start_from) {
					return bm;
				}
			}
		} else {
			for (auto it = list.rbegin(); it != list.rend(); ++it) {
				if (it->start < start_from) {
					return *it;
				}
			}
		}
		return bookmark{-1, -1};
	};
	bool wrapping{false};
	bookmark target = find_target(current_pos);
	if (target.start == -1 && allow_wrap) {
		wrapping = true;
		target = next ? find_target(-1) : find_target(text_ctrl->GetLastPosition() + 1);
	}
	if (target.start == -1) {
		speak(next ? _("No next note") : _("No previous note"));
		return;
	}
	text_ctrl->SetInsertionPoint(target.start);
	wxString text_to_speak;
	if (target.is_whole_line()) {
		long line{0};
		text_ctrl->PositionToXY(target.start, nullptr, &line);
		text_to_speak = text_ctrl->GetLineText(line);
	} else {
		text_to_speak = text_ctrl->GetRange(target.start, target.end);
	}
	int index{0};
	for (size_t i = 0; i < list.size(); ++i) {
		if (list[i] == target) {
			index = static_cast<int>(i);
			break;
		}
	}
	wxString announcement;
	announcement = target.has_note() ? wxString::Format(_("%s - %s - Note %d"), target.note, text_to_speak, index + 1) : wxString::Format(_("%s - Note %d"), text_to_speak, index + 1);
	if (wrapping) {
		announcement = (next ? _("Wrapping to start. ") : _("Wrapping to end. ")) + announcement;
	}
	speak(announcement);
}

void document_manager::go_to_previous_bookmark() const {
	navigate_to_bookmark(false);
}

void document_manager::go_to_next_bookmark() const {
	navigate_to_bookmark(true);
}

void document_manager::go_to_previous_note() const {
	navigate_to_note(false);
}

void document_manager::go_to_next_note() const {
	navigate_to_note(true);
}

void document_manager::navigate_to_link(bool next) const {
	const document* doc = get_active_document();
	const wxTextCtrl* text_ctrl = get_active_text_ctrl();
	if (doc == nullptr || text_ctrl == nullptr) {
		return;
	}
	if (doc->buffer.count_markers_by_type(marker_type::link) == 0) {
		speak(_("No links."));
		return;
	}
	const int current_pos = text_ctrl->GetInsertionPoint();
	bool wrapping = false;
	int target_index = next ? doc->buffer.next_marker_index(current_pos, marker_type::link) : doc->buffer.previous_marker_index(current_pos, marker_type::link);
	if (target_index == -1) {
		if (config.get(config_manager::navigation_wrap)) {
			target_index = next ? doc->buffer.next_marker_index(-1, marker_type::link) : doc->buffer.previous_marker_index(text_ctrl->GetLastPosition() + 1, marker_type::link);
			if (target_index != -1) {
				wrapping = true;
			}
		}
		if (target_index == -1) {
			speak(next ? _("No next link.") : _("No previous link."));
			return;
		}
	}
	const marker* link_marker = doc->buffer.get_marker(target_index);
	if (link_marker != nullptr) {
		go_to_position(static_cast<long>(link_marker->pos));
		wxString message = link_marker->text + _(" link");
		if (wrapping) {
			message = (next ? _("Wrapping to start. ") : _("Wrapping to end. ")) + message;
		}
		speak(message);
	}
}

void document_manager::go_to_previous_link() const {
	navigate_to_link(false);
}

void document_manager::go_to_next_link() const {
	navigate_to_link(true);
}

void document_manager::go_to_previous_position() const {
	document* doc = get_active_document();
	wxTextCtrl* text_ctrl = get_active_text_ctrl();
	if (doc == nullptr || text_ctrl == nullptr) {
		return;
	}
	if (doc->history.empty()) {
		speak(_("No previous position."));
		return;
	}
	const long actual_pos = text_ctrl->GetInsertionPoint();
	if (doc->history[doc->history_index] != actual_pos) {
		if (doc->history_index + 1 < doc->history.size()) {
			if (doc->history[doc->history_index + 1] != actual_pos) {
				doc->history.erase(doc->history.begin() + doc->history_index + 1, doc->history.end());
				doc->history.push_back(actual_pos);
				doc->history_index++;
			} else {
				doc->history_index++;
			}
		} else {
			doc->history.push_back(actual_pos);
			doc->history_index++;
			if (doc->history.size() > 10) {
				doc->history.erase(doc->history.begin());
				doc->history_index--;
			}
		}
	}
	if (doc->history_index > 0) {
		doc->history_index--;
		go_to_position(doc->history[doc->history_index]);
		speak(_("Navigated to previous position."));
	} else {
		speak(_("No previous position."));
	}
}

void document_manager::go_to_next_position() const {
	document* doc = get_active_document();
	wxTextCtrl* text_ctrl = get_active_text_ctrl();
	if (doc == nullptr || text_ctrl == nullptr) {
		return;
	}
	if (doc->history.empty()) {
		speak(_("No next position."));
		return;
	}
	const long actual_pos = text_ctrl->GetInsertionPoint();
	if (doc->history[doc->history_index] != actual_pos) {
		if (doc->history_index + 1 < doc->history.size()) {
			if (doc->history[doc->history_index + 1] != actual_pos) {
				doc->history.erase(doc->history.begin() + doc->history_index + 1, doc->history.end());
				doc->history.push_back(actual_pos);
				doc->history_index++;
			} else {
				doc->history_index++;
			}
		} else {
			doc->history.push_back(actual_pos);
			doc->history_index++;
			if (doc->history.size() > 10) {
				doc->history.erase(doc->history.begin());
				doc->history_index--;
			}
		}
	}
	if (doc->history_index + 1 < doc->history.size()) {
		doc->history_index++;
		go_to_position(doc->history[doc->history_index]);
		speak(_("Navigated to next position."));
	} else {
		speak(_("No next position."));
	}
}

void document_manager::activate_current_link() const {
	document* doc = get_active_document();
	const wxTextCtrl* text_ctrl = get_active_text_ctrl();
	if (doc == nullptr || text_ctrl == nullptr) {
		return;
	}
	const long current_pos = text_ctrl->GetInsertionPoint();
	const int link_index = doc->buffer.current_marker_index(static_cast<size_t>(current_pos), marker_type::link);
	if (link_index == -1) {
		return;
	}
	const marker* link_marker = doc->buffer.get_marker(link_index);
	if (link_marker == nullptr) {
		return;
	}
	const size_t link_end = link_marker->pos + link_marker->text.length();
	if (static_cast<size_t>(current_pos) < link_marker->pos || static_cast<size_t>(current_pos) > link_end) {
		return;
	}
	const wxString href = link_marker->ref;
	if (href.empty()) {
		return;
	}
	if (doc->history.empty() || doc->history[doc->history_index] != current_pos) {
		if (doc->history_index + 1 < doc->history.size()) {
			doc->history.erase(doc->history.begin() + doc->history_index + 1, doc->history.end());
		}
		doc->history.push_back(current_pos);
		doc->history_index = doc->history.size() - 1;
	}
	const wxString href_lower = href.Lower();
	if (href_lower.StartsWith("http:") || href_lower.StartsWith("https:") || href_lower.StartsWith("mailto:")) {
		if (wxLaunchDefaultBrowser(href)) {
			speak(_("Opening link in default browser."));
		} else {
			speak(_("Failed to open link."));
		}
	} else if (href.StartsWith("#")) {
		const wxString id = href.Mid(1);
		auto it = doc->id_positions.find(std::string(id.mb_str()));
		if (it != doc->id_positions.end()) {
			go_to_position(static_cast<long>(it->second));
			speak(_("Navigated to internal link."));
		} else {
			speak(_("Internal link target not found."));
		}
	} else {
		const wxString file_path = href.BeforeFirst('#');
		const wxString fragment = href.AfterFirst('#');
		if (!file_path.empty()) {
			wxFileName link_path;
			link_path.Assign(file_path, wxPATH_UNIX);
			link_path.Normalize(wxPATH_NORM_DOTS, "", wxPATH_UNIX);
			const wxString normalized_file_path = link_path.GetPathWithSep(wxPATH_UNIX) + link_path.GetFullName();
			wxString manifest_id;
			for (auto const& [id, path] : doc->manifest_items) {
				if (path == normalized_file_path) {
					manifest_id = id;
					break;
				}
			}
			if (!manifest_id.empty()) {
				auto it = std::ranges::find(doc->spine_items, std::string(manifest_id.mb_str()));
				if (it != doc->spine_items.end()) {
					const int spine_index = static_cast<int>(std::distance(doc->spine_items.begin(), it));
					size_t section_start = doc->buffer.get_marker_position_by_index(marker_type::section_break, spine_index);
					size_t section_end = (spine_index + 1 < static_cast<int>(doc->spine_items.size()))
						? doc->buffer.get_marker_position_by_index(marker_type::section_break, spine_index + 1)
						: doc->buffer.str().length();
					size_t offset = section_start;
					if (!fragment.empty()) {
						auto frag_it = doc->id_positions.find(std::string(fragment.mb_str()));
						if (frag_it != doc->id_positions.end() && frag_it->second >= section_start && frag_it->second < section_end) {
							offset = frag_it->second;
						}
					}
					go_to_position(static_cast<long>(offset));
					speak(_("Navigated to internal link."));
					return;
				}
			}
		} else if (!fragment.empty()) {
			auto it = doc->id_positions.find(std::string(fragment.mb_str()));
			if (it != doc->id_positions.end()) {
				go_to_position(static_cast<long>(it->second));
				speak(_("Navigated to internal link."));
				return;
			}
		}
		speak(_("Internal link target not found."));
	}
}

void document_manager::navigate_to_list(bool next) const {
	const document* doc = get_active_document();
	wxTextCtrl* text_ctrl = get_active_text_ctrl();
	const parser* par = get_active_parser();
	if (doc == nullptr || text_ctrl == nullptr || par == nullptr) {
		return;
	}
	if (!par->has_flag(parser_flags::supports_lists)) {
		speak(_("No lists."));
		return;
	}
	if (doc->buffer.count_markers_by_type(marker_type::list) == 0) {
		speak(_("No lists."));
		return;
	}
	const int current_pos = text_ctrl->GetInsertionPoint();
	bool wrapping = false;
	int target_index = next ? doc->buffer.next_marker_index(current_pos, marker_type::list) : doc->buffer.previous_marker_index(current_pos, marker_type::list);
	if (target_index == -1) {
		if (config.get(config_manager::navigation_wrap)) {
			target_index = next ? doc->buffer.next_marker_index(-1, marker_type::list) : doc->buffer.previous_marker_index(text_ctrl->GetLastPosition() + 1, marker_type::list);
			if (target_index != -1) {
				wrapping = true;
			}
		}
		if (target_index == -1) {
			speak(next ? _("No next list.") : _("No previous list."));
			return;
		}
	}
	const marker* list_marker = doc->buffer.get_marker(target_index);
	if (list_marker != nullptr) {
		wxString message = wxString::Format(_("List with %d items"), list_marker->level);
		const int first_item_index = doc->buffer.find_first_marker_after(static_cast<long>(list_marker->pos), marker_type::list_item);
		const marker* first_item_marker = doc->buffer.get_marker(first_item_index);
		if (first_item_marker != nullptr) {
			go_to_position(static_cast<long>(first_item_marker->pos));
			long line_num{0};
			text_ctrl->PositionToXY(static_cast<long>(first_item_marker->pos), nullptr, &line_num);
			wxString line_text = text_ctrl->GetLineText(line_num).Trim();
			message += " " + line_text;
		} else {
			go_to_position(static_cast<long>(list_marker->pos));
		}
		if (wrapping) {
			message = (next ? _("Wrapping to start. ") : _("Wrapping to end. ")) + message;
		}
		speak(message);
	}
}

void document_manager::go_to_previous_list() const {
	navigate_to_list(false);
}

void document_manager::go_to_next_list() const {
	navigate_to_list(true);
}

void document_manager::navigate_to_list_item(bool next) const {
	const document* doc = get_active_document();
	wxTextCtrl* text_ctrl = get_active_text_ctrl();
	const parser* par = get_active_parser();
	if (doc == nullptr || text_ctrl == nullptr || par == nullptr) {
		return;
	}
	if (!par->has_flag(parser_flags::supports_lists)) {
		speak(_("No lists."));
		return;
	}
	if (doc->buffer.count_markers_by_type(marker_type::list_item) == 0) {
		speak(_("No list items."));
		return;
	}
	const int current_pos = text_ctrl->GetInsertionPoint();
	// Using current_marker_index with list_item can incorrectly return the previous list item when the caret is outside a list. As such, we instead check for a list_item marker exactly at the current line start.
	long current_line_num{0};
	text_ctrl->PositionToXY(current_pos, nullptr, &current_line_num);
	const long current_line_start = text_ctrl->XYToPosition(0, current_line_num);
	int marker_index_at_line = doc->buffer.find_first_marker_after(current_line_start, marker_type::list_item);
	const marker* line_item_marker = doc->buffer.get_marker(marker_index_at_line);
	int current_list_index{-1};
	if (line_item_marker != nullptr && static_cast<long>(line_item_marker->pos) == current_line_start) {
		current_list_index = doc->buffer.current_marker_index(line_item_marker->pos, marker_type::list);
	}
	bool wrapping = false;
	int target_index = next ? doc->buffer.next_marker_index(current_pos, marker_type::list_item) : doc->buffer.previous_marker_index(current_pos, marker_type::list_item);
	if (target_index == -1) {
		if (config.get(config_manager::navigation_wrap)) {
			target_index = next ? doc->buffer.next_marker_index(-1, marker_type::list_item) : doc->buffer.previous_marker_index(text_ctrl->GetLastPosition() + 1, marker_type::list_item);
			if (target_index != -1) {
				wrapping = true;
			}
		}
		if (target_index == -1) {
			speak(next ? _("No next list item.") : _("No previous list item."));
			return;
		}
	}
	const marker* list_item_marker = doc->buffer.get_marker(target_index);
	if (list_item_marker != nullptr) {
		const int target_list_index = doc->buffer.current_marker_index(list_item_marker->pos, marker_type::list);
		const marker* target_list_marker = doc->buffer.get_marker(target_list_index);
		wxString message;
		if (target_list_index != -1 && target_list_index != current_list_index && target_list_marker != nullptr) {
			message += wxString::Format(_("List with %d items "), target_list_marker->level);
		}
		go_to_position(static_cast<long>(list_item_marker->pos));
		long line_num{0};
		text_ctrl->PositionToXY(static_cast<long>(list_item_marker->pos), nullptr, &line_num);
		message += text_ctrl->GetLineText(line_num).Trim();
		if (wrapping) {
			message = (next ? _("Wrapping to start. ") : _("Wrapping to end. ")) + message;
		}
		speak(message);
	}
}

void document_manager::go_to_previous_list_item() const {
	navigate_to_list_item(false);
}

void document_manager::go_to_next_list_item() const {
	navigate_to_list_item(true);
}

void document_manager::toggle_bookmark() const {
	const document_tab* tab = get_active_tab();
	const wxTextCtrl* text_ctrl = get_active_text_ctrl();
	if (tab == nullptr || text_ctrl == nullptr) {
		return;
	}
	long selection_start = 0;
	long selection_end = 0;
	text_ctrl->GetSelection(&selection_start, &selection_end);
	int bookmark_start, bookmark_end;
	if (selection_end > selection_start) {
		bookmark_start = static_cast<int>(selection_start);
		bookmark_end = static_cast<int>(selection_end);
	} else {
		const int current_pos = text_ctrl->GetInsertionPoint();
		bookmark_start = current_pos;
		bookmark_end = current_pos;
	}
	std::vector<bookmark> bookmarks = config.get_bookmarks(tab->file_path);
	bookmark to_toggle(bookmark_start, bookmark_end);
	bool was_bookmarked = false;
	for (const auto& bm : bookmarks) {
		if (bm == to_toggle) {
			was_bookmarked = true;
			break;
		}
	}
	config.toggle_bookmark(tab->file_path, bookmark_start, bookmark_end);
	config.flush();
	speak(was_bookmarked ? _("Bookmark removed") : _("Bookmarked"));
}

void document_manager::add_bookmark_with_note() const {
	const document_tab* tab = get_active_tab();
	wxTextCtrl* text_ctrl = get_active_text_ctrl();
	if (tab == nullptr || text_ctrl == nullptr) {
		return;
	}
	long selection_start{0};
	long selection_end{0};
	text_ctrl->GetSelection(&selection_start, &selection_end);
	int bookmark_start, bookmark_end;
	if (selection_end > selection_start) {
		bookmark_start = static_cast<int>(selection_start);
		bookmark_end = static_cast<int>(selection_end);
	} else {
		const int current_pos = text_ctrl->GetInsertionPoint();
		bookmark_start = current_pos;
		bookmark_end = current_pos;
	}
	std::vector<bookmark> bookmarks = config.get_bookmarks(tab->file_path);
	bookmark existing_bookmark(bookmark_start, bookmark_end);
	wxString existing_note;
	bool bookmark_exists = false;
	for (const auto& bm : bookmarks) {
		if (bm.start == bookmark_start && bm.end == bookmark_end) {
			bookmark_exists = true;
			existing_note = bm.note;
			break;
		}
	}
	wxString prompt = bookmark_exists ? _("Edit bookmark note:") : _("Enter bookmark note:");
	note_entry_dialog note_dialog(nullptr, _("Bookmark Note"), prompt, existing_note);
	if (note_dialog.ShowModal() != wxID_OK) {
		return;
	}
	wxString note = note_dialog.get_note();
	if (bookmark_exists) {
		config.update_bookmark_note(tab->file_path, bookmark_start, bookmark_end, note);
		speak(_("Bookmark note updated"));
	} else {
		config.add_bookmark(tab->file_path, bookmark_start, bookmark_end, note);
		speak(_("Bookmarked with note"));
	}
	config.flush();
}

void document_manager::show_bookmark_dialog(wxWindow* parent, bookmark_filter initial_filter) {
	const document_tab* tab = get_active_tab();
	wxTextCtrl* text_ctrl = get_active_text_ctrl();
	if (tab == nullptr || text_ctrl == nullptr) {
		return;
	}
	const std::vector<bookmark> bookmarks = config.get_bookmarks(tab->file_path);
	if (bookmarks.empty()) {
		speak(_("No bookmarks"));
		return;
	}
	const int current_pos = text_ctrl->GetInsertionPoint();
	bookmark_dialog dialog(parent, bookmarks, text_ctrl, config, tab->file_path, current_pos, initial_filter);
	const int result = dialog.ShowModal();
	if (result != wxID_OK) {
		return;
	}
	const int pos = dialog.get_selected_position();
	if (pos < 0) {
		return;
	}
	text_ctrl->SetInsertionPoint(pos);
	text_ctrl->SetFocus();
	wxString text_to_speak;
	wxString note_to_speak;
	for (const auto& bm : bookmarks) {
		if (bm.start == pos) {
			if (bm.is_whole_line()) {
				long line{0};
				text_ctrl->PositionToXY(pos, nullptr, &line);
				text_to_speak = text_ctrl->GetLineText(line);
			} else {
				text_to_speak = text_ctrl->GetRange(bm.start, bm.end);
			}
			note_to_speak = bm.note;
			break;
		}
	}
	wxString announcement;
	if (!note_to_speak.IsEmpty()) {
		announcement = wxString::Format(_("Bookmark: %s - %s"), note_to_speak, text_to_speak);
	} else {
		announcement = wxString::Format(_("Bookmark: %s"), text_to_speak);
	}
	speak(announcement);
	update_ui();
}

void document_manager::show_table_of_contents(wxWindow* parent) const {
	const document* doc = get_active_document();
	wxTextCtrl* text_ctrl = get_active_text_ctrl();
	const parser* par = get_active_parser();
	if (doc == nullptr || text_ctrl == nullptr || par == nullptr) {
		return;
	}
	if (!par->has_flag(parser_flags::supports_toc)) {
		speak(_("No table of contents."));
		return;
	}
	if (doc->toc_items.empty()) {
		speak(_("Table of contents is empty."));
		return;
	}
	const int current_pos = text_ctrl->GetInsertionPoint();
	const int closest_toc_offset = doc->find_closest_toc_offset(current_pos);
	toc_dialog dlg(parent, doc, closest_toc_offset);
	if (dlg.ShowModal() != wxID_OK) {
		return;
	}
	const int offset = dlg.get_selected_offset();
	if (offset >= 0) {
		go_to_position(offset);
		text_ctrl->SetFocus();
	}
}

void document_manager::show_document_info(wxWindow* parent) {
	const document_tab* tab = get_active_tab();
	if (tab == nullptr) {
		return;
	}
	const document* doc = tab->doc.get();
	if (doc == nullptr) {
		return;
	}
	document_info_dialog dlg(parent, doc, tab->file_path, config);
	dlg.ShowModal();
	if (dlg.imported_position > -1) {
		go_to_position(dlg.imported_position);
	}
}

void document_manager::save_document_position(const wxString& path, long position) const {
	config.set_document_position(path, position);
	config.flush();
}

long document_manager::load_document_position(const wxString& path) const {
	return config.get_document_position(path);
}

void document_manager::save_current_tab_position() const {
	const document_tab* tab = get_active_tab();
	if (tab == nullptr || tab->text_ctrl == nullptr) {
		return;
	}
	const int position = tab->text_ctrl->GetInsertionPoint();
	save_document_position(tab->file_path, position);
}

void document_manager::save_all_tab_positions() const {
	for (int i = 0; i < get_tab_count(); ++i) {
		const document_tab* tab = get_tab(i);
		if (tab != nullptr && tab->text_ctrl != nullptr) {
			const int position = tab->text_ctrl->GetInsertionPoint();
			save_document_position(tab->file_path, position);
		}
	}
}

void document_manager::save_all_tab_navigation_histories() const {
	for (int i = 0; i < get_tab_count(); ++i) {
		const document_tab* tab = get_tab(i);
		if (tab != nullptr && tab->doc != nullptr) {
			config.set_navigation_history(tab->file_path, tab->doc->history, tab->doc->history_index);
		}
	}
}

wxString document_manager::get_status_text() const {
	if (!has_documents()) {
		return _("Ready");
	}
	const wxTextCtrl* text_ctrl = get_active_text_ctrl();
	if (text_ctrl == nullptr) {
		return _("Ready");
	}
	const int current_pos = text_ctrl->GetInsertionPoint();
	const int total_chars = text_ctrl->GetLastPosition();
	const int percentage = total_chars > 0 ? (current_pos * 100) / total_chars : 0;
	long line{0};
	text_ctrl->PositionToXY(current_pos, nullptr, &line);
	const long line_number = line + 1;
	const long character_number = current_pos + 1;
	return wxString::Format(_("line %ld, character %ld, reading %d%%"), line_number, character_number, percentage);
}

wxString document_manager::get_window_title(const wxString& app_name) const {
	if (!has_documents()) {
		return app_name;
	}
	const document* doc = get_active_document();
	return doc != nullptr ? app_name + " - " + doc->title : app_name;
}

int document_manager::find_text(const wxString& query, int start_pos, find_options options) const {
	const wxTextCtrl* text_ctrl = get_active_text_ctrl();
	if (text_ctrl == nullptr) {
		return wxNOT_FOUND;
	}
	const wxString& full_text = text_ctrl->GetValue();
	return ::find_text(full_text, query, start_pos, options);
}

void document_manager::apply_word_wrap(bool word_wrap) {
	for (int i = 0; i < get_tab_count(); ++i) {
		document_tab* tab = get_tab(i);
		if (tab != nullptr && tab->text_ctrl != nullptr && tab->panel != nullptr) {
			wxTextCtrl* old_ctrl = tab->text_ctrl;
			const int current_pos = old_ctrl->GetInsertionPoint();
			const wxString content = old_ctrl->GetValue();
			wxSizer* sizer = tab->panel->GetSizer();
			sizer->Detach(old_ctrl);
			old_ctrl->Destroy();
			const long style = wxTE_MULTILINE | wxTE_READONLY | wxTE_RICH2 | (word_wrap ? wxTE_WORDWRAP : wxTE_DONTWRAP);
			auto* new_ctrl = new wxTextCtrl(tab->panel, wxID_ANY, "", wxDefaultPosition, wxDefaultSize, style);
			tab->text_ctrl = new_ctrl;
			new_ctrl->Bind(wxEVT_KEY_UP, &main_window::on_text_cursor_changed, &main_win);
			new_ctrl->Freeze();
			new_ctrl->SetValue(content);
			new_ctrl->SetInsertionPoint(current_pos);
			new_ctrl->ShowPosition(current_pos);
			new_ctrl->Thaw();
			sizer->Add(new_ctrl, 1, wxEXPAND | wxALL, static_cast<int>(5));
			tab->panel->Layout();
		}
	}
}

int document_manager::find_tab_by_path(const wxString& path) const {
	wxFileName input_file(path);
	input_file.Normalize(static_cast<unsigned>(wxPATH_NORM_ABSOLUTE) | static_cast<unsigned>(wxPATH_NORM_LONG));
	const wxString input_abs_path = input_file.GetFullPath();
	for (int i = 0; i < get_tab_count(); ++i) {
		const document_tab* tab = get_tab(i);
		if (tab == nullptr) {
			continue;
		}
		wxFileName tab_file(tab->file_path);
		tab_file.Normalize(static_cast<unsigned>(wxPATH_NORM_ABSOLUTE) | static_cast<unsigned>(wxPATH_NORM_LONG));
		if (tab_file.GetFullPath().IsSameAs(input_abs_path, false)) {
			return i;
		}
	}
	return -1;
}

void document_manager::create_heading_menu(wxMenu* menu) {
	menu->Append(ID_PREVIOUS_HEADING, _("Previous heading\tShift+H"));
	menu->Append(ID_NEXT_HEADING, _("Next heading\tH"));
	menu->AppendSeparator();
	for (int level = 1; level <= MAX_HEADING_LEVELS; ++level) {
		menu->Append(ID_PREVIOUS_HEADING_1 + ((level - 1) * 2), wxString::Format(_("Previous heading level %d\tShift+%d"), level, level));
		menu->Append(ID_NEXT_HEADING_1 + ((level - 1) * 2), wxString::Format(_("Next heading level %d\t%d"), level, level));
	}
}

void document_manager::setup_text_ctrl(wxTextCtrl* text_ctrl, const wxString& content) {
	text_ctrl->Freeze();
	text_ctrl->SetValue(content);
	text_ctrl->Thaw();
}

void document_manager::restore_document_position(document_tab* tab) const {
	if (tab == nullptr || tab->text_ctrl == nullptr) {
		return;
	}
	const int saved_position = load_document_position(tab->file_path);
	if (saved_position > 0) {
		const int max_position = tab->text_ctrl->GetLastPosition();
		if (saved_position <= max_position) {
			tab->text_ctrl->SetInsertionPoint(saved_position);
			tab->text_ctrl->ShowPosition(saved_position);
		}
	}
}

wxPanel* document_manager::create_tab_panel(const wxString& content, document_tab* tab_data) {
	auto* panel = new wxPanel(notebook, wxID_ANY);
	auto* sizer = new wxBoxSizer(wxVERTICAL);
	const bool word_wrap = config.get(config_manager::word_wrap);
	const long style = wxTE_MULTILINE | wxTE_READONLY | wxTE_RICH2 | (word_wrap ? wxTE_WORDWRAP : wxTE_DONTWRAP);
	auto* text_ctrl = new wxTextCtrl(panel, wxID_ANY, "", wxDefaultPosition, wxDefaultSize, style);
	panel->SetClientObject(tab_data);
	tab_data->text_ctrl = text_ctrl;
	text_ctrl->Bind(wxEVT_KEY_UP, &main_window::on_text_cursor_changed, &main_win);
	text_ctrl->Bind(wxEVT_CHAR, &main_window::on_text_char, &main_win);
	sizer->Add(text_ctrl, 1, wxEXPAND | wxALL, static_cast<int>(5));
	panel->SetSizer(sizer);
	setup_text_ctrl(text_ctrl, content);
	return panel;
}

void document_manager::navigate_to_heading(bool next, int specific_level) const {
	const document* doc = get_active_document();
	wxTextCtrl* text_ctrl = get_active_text_ctrl();
	if (doc == nullptr || text_ctrl == nullptr) {
		return;
	}
	if (doc->buffer.get_heading_markers().empty()) {
		speak(_("No headings."));
		return;
	}
	const int current_pos = text_ctrl->GetInsertionPoint();
	int target_index = -1;
	bool wrapping = false;
	target_index = next ? doc->next_heading_index(current_pos, specific_level) : doc->previous_heading_index(current_pos, specific_level);
	if (target_index == -1) {
		if (config.get(config_manager::navigation_wrap)) {
			target_index = next ? doc->next_heading_index(-1, specific_level) : doc->previous_heading_index(text_ctrl->GetLastPosition() + 1, specific_level);
			if (target_index != -1) {
				wrapping = true;
			}
		}
		if (target_index == -1) {
			const wxString msg = (specific_level == -1) ? wxString::Format(_("No %s heading"), next ? _("next") : _("previous")) : wxString::Format(_("No %s heading at level %d"), next ? _("next") : _("previous"), specific_level);
			speak(msg);
			return;
		}
	}
	const size_t offset = doc->offset_for_heading(target_index);
	text_ctrl->SetInsertionPoint(static_cast<long>(offset));
	const marker* heading_marker = doc->get_heading_marker(target_index);
	if (heading_marker != nullptr) {
		wxString message;
		if (wrapping) {
			message = wxString::Format(_("Wrapping to %s. %s Heading level %d"), next ? _("start") : _("end"), heading_marker->text, heading_marker->level);
		} else {
			message = wxString::Format(_("%s Heading level %d"), heading_marker->text, heading_marker->level);
		}
		speak(message);
	}
}
