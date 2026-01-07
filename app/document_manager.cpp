#include "document_manager.hpp"
#include "config_manager.hpp"
#include "constants.hpp"
#include "dialogs.hpp"
#include "document_data.hpp"
#include "main_window.hpp"
#include "parser.hpp"
#include "utils.hpp"
#include <algorithm>
#include <cstdint>
#include <cstddef>
#include <iterator>
#include <memory>
#include <optional>
#include <ranges>
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

namespace {
wxString rust_to_wx(const rust::String& rust_str) {
	return wxString::FromUTF8(std::string(rust_str).c_str());
}

bool supports_feature(uint32_t flags, uint32_t feature) {
	return (flags & feature) != 0;
}

constexpr uint32_t PARSER_SUPPORTS_SECTIONS = 1 << 0;
constexpr uint32_t PARSER_SUPPORTS_TOC = 1 << 1;
constexpr uint32_t PARSER_SUPPORTS_PAGES = 1 << 2;
constexpr uint32_t PARSER_SUPPORTS_LISTS = 1 << 3;

void populate_toc_items(std::vector<std::unique_ptr<toc_item>>& toc_items, const rust::Vec<FfiTocItemWithParent>& ffi_toc_items) {
	if (ffi_toc_items.empty()) return;
	std::vector<toc_item*> item_ptrs;
	item_ptrs.reserve(ffi_toc_items.size());
	for (const auto& rust_toc : ffi_toc_items) {
		auto item = std::make_unique<toc_item>();
		item->name = to_wxstring(rust_toc.name);
		item->ref = to_wxstring(rust_toc.reference);
		item->offset = rust_toc.offset;
		toc_item* item_ptr = nullptr;
		if (rust_toc.parent_index < 0) {
			toc_items.push_back(std::move(item));
			item_ptr = toc_items.back().get();
		} else {
			const auto parent_idx = static_cast<size_t>(rust_toc.parent_index);
			if (parent_idx < item_ptrs.size() && item_ptrs[parent_idx] != nullptr) {
				item_ptrs[parent_idx]->children.push_back(std::move(item));
				item_ptr = item_ptrs[parent_idx]->children.back().get();
			} else {
				// Fallback to root if parent index is invalid.
				toc_items.push_back(std::move(item));
				item_ptr = toc_items.back().get();
			}
		}
		item_ptrs.push_back(item_ptr);
	}
}

} // namespace

void session_document::ensure_toc_loaded() {
	if (toc_loaded) return;
	toc_loaded = true;
	populate_toc_items(toc_items, document_toc_items_with_parents(get_handle()));
}

document_manager::document_manager(wxNotebook* nbk, config_manager& cfg, main_window& win) : notebook{nbk}, config{cfg}, main_win{win} {
}

document_manager::~document_manager() {
	save_all_tab_positions();
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
		if (text_ctrl != nullptr) text_ctrl->SetFocus();
		return true;
	}
	const wxString extension = wxFileName(path).GetExt();
	if (!is_parser_supported(extension)) {
		if (!ensure_parser_for_unknown_file(path, config)) return false;
	}
	if (!create_document_tab(path, true, add_to_recent)) return false;
	auto* const text_ctrl = get_active_text_ctrl();
	if (text_ctrl != nullptr) {
		text_ctrl->Bind(wxEVT_KEY_UP, &main_window::on_text_cursor_changed, &main_win);
		text_ctrl->Bind(wxEVT_CHAR, &main_window::on_text_char, &main_win);
	}
	update_ui();
	return true;
}

bool document_manager::create_document_tab(const wxString& path, bool set_focus, bool add_to_recent) {
	try {
		config.import_document_settings(path);
		const wxString forced_extension = config.get_document_format(path);
		wxString password_in_use;
		const wxString saved_password = config.get_document_password(path);
		auto load_session = [&](const wxString& password) -> rust::Box<DocumentSession> {
			const std::string path_utf8 = path.ToUTF8().data();
			const std::string password_utf8 = password.ToUTF8().data();
			const std::string extension_utf8 = forced_extension.ToUTF8().data();
			return session_new(path_utf8, password_utf8, extension_utf8);
		};
		rust::Box<DocumentSession> session = [&]() -> rust::Box<DocumentSession> {
			try {
				auto sess = load_session(saved_password);
				if (!saved_password.IsEmpty()) password_in_use = saved_password;
				return sess;
			} catch (const std::exception& e) {
				const std::string error_msg = e.what();
				const auto info = parser_error_info(error_msg);
				if (info.kind == ParserErrorKind::PasswordRequired) {
					config.set_document_password(path, wxEmptyString);
					password_dialog dlg(&main_win);
					if (dlg.ShowModal() != wxID_OK) {
						wxMessageBox(_("Password is required."), _("Error"), wxICON_ERROR);
						throw std::runtime_error("Password dialog cancelled");
					}
					const wxString entered_password = dlg.get_password();
					password_in_use = entered_password;
					return load_session(entered_password);
				}
				wxMessageBox(wxString::Format(_("Failed to parse document: %s"), wxString::FromUTF8(e.what())), _("Error"), wxICON_ERROR);
				throw;
			}
		}();
		if (!password_in_use.IsEmpty()) config.set_document_password(path, password_in_use);
		std::vector<long> history;
		size_t history_index = 0;
		config.get_navigation_history(path, history, history_index);
		if (!history.empty()) {
			rust::Vec<long long> rust_history;
			rust_history.reserve(history.size());
			for (long pos : history) rust_history.push_back(static_cast<long long>(pos));
			rust::Slice<const std::int64_t> history_slice(rust_history.data(), rust_history.size());
			session_set_history(*session, history_slice, history_index);
		}
		auto session_doc = std::make_unique<session_document>(std::move(session));
		auto* tab_data = new document_tab;
		tab_data->session_doc = std::move(session_doc);
		tab_data->file_path = path;
		wxPanel* panel = create_tab_panel(tab_data->session_doc->content, tab_data);
		tab_data->panel = panel;
		notebook->AddPage(panel, tab_data->session_doc->get_title(), true);
		restore_document_position(tab_data);
		if (set_focus) tab_data->text_ctrl->SetFocus();
		if (add_to_recent) config.add_recent_document(path);
		config.set_document_opened(path, true);
		return true;
	} catch (const std::exception&) {
		return false;
	}
}

void document_manager::update_ui() {
	main_win.update_recent_documents_menu();
	main_win.update_title();
	main_win.update_status_bar();
	main_win.update_ui();
}

void document_manager::close_document(int index) {
	if (index < 0 || index >= get_tab_count()) return;
	const document_tab* tab = get_tab(index);
	if (tab != nullptr && tab->text_ctrl != nullptr) {
		const int position = tab->text_ctrl->GetInsertionPoint();
		save_document_position(tab->file_path, position);
		if (tab->session_doc && tab->get_session()) {
			const auto history_data = session_get_history(*tab->get_session());
			if (!history_data.positions.empty()) {
				std::vector<long> history_vec;
				history_vec.reserve(history_data.positions.size());
				for (const auto pos : history_data.positions) history_vec.push_back(static_cast<long>(pos));
				config.set_navigation_history(tab->file_path, history_vec, history_data.index);
			}
		}
		config.set_document_opened(tab->file_path, false);
	}
	notebook->DeletePage(index);
}

void document_manager::close_all_documents() {
	save_all_tab_positions();
	for (int i = 0; i < get_tab_count(); ++i) {
		const document_tab* tab = get_tab(i);
		if (tab != nullptr) 	config.set_document_opened(tab->file_path, false);
	}
	notebook->DeleteAllPages();
}

bool document_manager::export_document(int index, const wxString& export_path) const {
	const document_tab* tab = get_tab(index);
	if (tab == nullptr || tab->session_doc == nullptr) return false;
	try {
		const std::string path_utf8 = export_path.ToUTF8().data();
		session_export_content(*tab->get_session(), path_utf8);
		return true;
	} catch (const std::exception&) {
		return false;
	}
}

document_tab* document_manager::get_tab(int index) const {
	if (index < 0 || index >= get_tab_count()) return nullptr;
	const auto* panel = dynamic_cast<wxPanel*>(notebook->GetPage(index));
	return dynamic_cast<document_tab*>(panel->GetClientObject());
}

document_tab* document_manager::get_active_tab() const {
	const int selection = notebook->GetSelection();
	return selection >= 0 ? get_tab(selection) : nullptr;
}

wxTextCtrl* document_manager::get_active_text_ctrl() const {
	const document_tab* tab = get_active_tab();
	return tab != nullptr ? tab->text_ctrl : nullptr;
}

int document_manager::get_tab_count() const {
	return static_cast<int>(notebook->GetPageCount());
}

int document_manager::get_active_tab_index() const {
	return notebook->GetSelection();
}

void document_manager::go_to_position(int position) const {
	wxTextCtrl* text_ctrl = get_active_text_ctrl();
	if (text_ctrl == nullptr) return;
	const int max_pos = text_ctrl->GetLastPosition();
	position = std::clamp(position, 0, max_pos);
	text_ctrl->SetInsertionPoint(position);
	text_ctrl->ShowPosition(position);
}

void document_manager::navigate_to_section(bool next) const {
	const document_tab* tab = get_active_tab();
	wxTextCtrl* text_ctrl = get_active_text_ctrl();
	if (tab == nullptr || text_ctrl == nullptr || tab->session_doc == nullptr) return;
	const bool wrap = config.get(config_manager::navigation_wrap);
	const auto result = session_navigate_section(*tab->get_session(), text_ctrl->GetInsertionPoint(), wrap, next);
	if (result.not_supported) {
		speak(_("No sections."));
		return;
	}
	if (!result.found) {
		speak(next ? _("No next section") : _("No previous section"));
		return;
	}
	const long offset = static_cast<long>(result.offset);
	text_ctrl->SetInsertionPoint(offset);
	const wxString current_line = rust_to_wx(rust::String(result.marker_text));
	if (result.wrapped)
		speak((next ? _("Wrapping to start. ") : _("Wrapping to end. ")) + current_line);
	else
		speak(current_line);
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
	const document_tab* tab = get_active_tab();
	wxTextCtrl* text_ctrl = get_active_text_ctrl();
	if (tab == nullptr || text_ctrl == nullptr || tab->session_doc == nullptr) return;
	const bool wrap = config.get(config_manager::navigation_wrap);
	const auto result = session_navigate_page(*tab->get_session(), text_ctrl->GetInsertionPoint(), wrap, next);
	if (result.not_supported) {
		speak(_("No pages."));
		return;
	}
	if (!result.found) {
		speak(next ? _("No next page.") : _("No previous page."));
		return;
	}
	const long offset = static_cast<long>(result.offset);
	text_ctrl->SetInsertionPoint(offset);
	const wxString current_line = rust_to_wx(rust::String(result.marker_text));
	wxString message = wxString::Format(_("Page %d: %s"), result.marker_index + 1, current_line);
	if (result.wrapped) message = (next ? _("Wrapping to start. ") : _("Wrapping to end. ")) + message;
	speak(message);
}

void document_manager::go_to_previous_page() const {
	navigate_to_page(false);
}

void document_manager::go_to_next_page() const {
	navigate_to_page(true);
}

void document_manager::navigate_to_bookmark(bool next) const {
	const document_tab* tab = get_active_tab();
	wxTextCtrl* text_ctrl = get_active_text_ctrl();
	if (tab == nullptr || text_ctrl == nullptr || tab->session_doc == nullptr) return;
	const bool allow_wrap = config.get(config_manager::navigation_wrap);
	const auto result = session_navigate_bookmark_display(*tab->get_session(), config.backend_for_ffi(), text_ctrl->GetInsertionPoint(), allow_wrap, next, false);
	if (!result.found) {
		speak(next ? _("No next bookmark") : _("No previous bookmark"));
		return;
	}
	const long start = static_cast<long>(result.start);
	text_ctrl->SetInsertionPoint(start);
	const wxString text_to_speak = rust_to_wx(result.snippet);
	const int index = result.index >= 0 ? result.index : 0;
	wxString announcement = wxString::Format(_("%s - Bookmark %d"), text_to_speak, index + 1);
	if (result.wrapped) announcement = (next ? _("Wrapping to start. ") : _("Wrapping to end. ")) + announcement;
	speak(announcement);
}

void document_manager::navigate_to_note(bool next) const {
	const document_tab* tab = get_active_tab();
	wxTextCtrl* text_ctrl = get_active_text_ctrl();
	if (tab == nullptr || text_ctrl == nullptr || tab->session_doc == nullptr) return;
	const bool allow_wrap = config.get(config_manager::navigation_wrap);
	auto result = session_navigate_bookmark_display(*tab->get_session(), config.backend_for_ffi(), text_ctrl->GetInsertionPoint(), allow_wrap, next, true);
	if (!result.found) {
		speak(next ? _("No next note") : _("No previous note"));
		return;
	}
	const long start = static_cast<long>(result.start);
	text_ctrl->SetInsertionPoint(start);
	const wxString text_to_speak = rust_to_wx(result.snippet);
	const wxString note_text = wxString::FromUTF8(result.note.c_str());
	const int index = result.index >= 0 ? result.index : 0;
	wxString announcement = wxString::Format(_("%s - Note %d"), text_to_speak, index + 1);
	if (!note_text.IsEmpty()) announcement = wxString::Format(_("%s - %s - Note %d"), note_text, text_to_speak, index + 1);
	if (result.wrapped) announcement = (next ? _("Wrapping to start. ") : _("Wrapping to end. ")) + announcement;
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
	const document_tab* tab = get_active_tab();
	wxTextCtrl* text_ctrl = get_active_text_ctrl();
	if (tab == nullptr || text_ctrl == nullptr || tab->session_doc == nullptr) return;
	const bool wrap = config.get(config_manager::navigation_wrap);
	const auto result = session_navigate_link(*tab->get_session(), text_ctrl->GetInsertionPoint(), wrap, next);
	if (result.not_supported) {
		speak(_("No links."));
		return;
	}
	if (!result.found) {
		speak(next ? _("No next link.") : _("No previous link."));
		return;
	}
	const long offset = static_cast<long>(result.offset);
	text_ctrl->SetInsertionPoint(offset);
	const wxString link_text = rust_to_wx(rust::String(result.marker_text));
	wxString message = link_text + _(" link");
	if (result.wrapped) message = (next ? _("Wrapping to start. ") : _("Wrapping to end. ")) + message;
	speak(message);
}

void document_manager::go_to_previous_link() const {
	navigate_to_link(false);
}

void document_manager::go_to_next_link() const {
	navigate_to_link(true);
}

void document_manager::go_to_previous_position() const {
	document_tab* tab = get_active_tab();
	wxTextCtrl* text_ctrl = get_active_text_ctrl();
	if (tab == nullptr || text_ctrl == nullptr || tab->session_doc == nullptr) return;
	const long actual_pos = text_ctrl->GetInsertionPoint();
	auto result = session_history_go_back(*tab->get_session(), actual_pos);
	if (result.found) {
		go_to_position(static_cast<long>(result.offset));
		speak(_("Navigated to previous position."));
	} else {
		speak(_("No previous position."));
	}
}

void document_manager::go_to_next_position() const {
	document_tab* tab = get_active_tab();
	wxTextCtrl* text_ctrl = get_active_text_ctrl();
	if (tab == nullptr || text_ctrl == nullptr || tab->session_doc == nullptr) return;
	const long actual_pos = text_ctrl->GetInsertionPoint();
	auto result = session_history_go_forward(*tab->get_session(), actual_pos);
	if (result.found) {
		go_to_position(static_cast<long>(result.offset));
		speak(_("Navigated to next position."));
	} else {
		speak(_("No next position."));
	}
}

void document_manager::activate_current_link() const {
	document_tab* tab = get_active_tab();
	const wxTextCtrl* text_ctrl = get_active_text_ctrl();
	if (tab == nullptr || text_ctrl == nullptr || tab->session_doc == nullptr) return;
	const long current_pos = text_ctrl->GetInsertionPoint();
	auto result = session_activate_link(*tab->get_session(), current_pos);
	if (!result.found) return;  // No link at current position
	switch (result.action) {
		case FfiLinkAction::External:
			if (wxLaunchDefaultBrowser(rust_to_wx(rust::String(result.url)))) speak(_("Opening link in default browser."));
			else speak(_("Failed to open link."));
			break;
		case FfiLinkAction::Internal:
			go_to_position(static_cast<long>(result.offset));
			speak(_("Navigated to internal link."));
			break;
		case FfiLinkAction::NotFound:
			speak(_("Internal link target not found."));
			break;
		default:
			break;
	}
}

void document_manager::navigate_to_list(bool next) const {
	const document_tab* tab = get_active_tab();
	wxTextCtrl* text_ctrl = get_active_text_ctrl();
	if (tab == nullptr || text_ctrl == nullptr || tab->session_doc == nullptr) return;
	const bool wrap = config.get(config_manager::navigation_wrap);
	const auto result = session_navigate_list(*tab->get_session(), text_ctrl->GetInsertionPoint(), wrap, next);
	if (result.not_supported) {
		speak(_("No lists."));
		return;
	}
	if (!result.found) {
		speak(next ? _("No next list.") : _("No previous list."));
		return;
	}
	const long offset = static_cast<long>(result.offset);
	text_ctrl->SetInsertionPoint(offset);
	const wxString current_line = rust_to_wx(rust::String(result.marker_text));
	wxString message = current_line;
	if (result.wrapped) message = (next ? _("Wrapping to start. ") : _("Wrapping to end. ")) + message;
	speak(message);
}

void document_manager::go_to_previous_list() const {
	navigate_to_list(false);
}

void document_manager::go_to_next_list() const {
	navigate_to_list(true);
}

void document_manager::navigate_to_list_item(bool next) const {
	const document_tab* tab = get_active_tab();
	wxTextCtrl* text_ctrl = get_active_text_ctrl();
	if (tab == nullptr || text_ctrl == nullptr || tab->session_doc == nullptr) return;
	const bool wrap = config.get(config_manager::navigation_wrap);
	const auto result = session_navigate_list_item(*tab->get_session(), text_ctrl->GetInsertionPoint(), wrap, next);
	if (result.not_supported) {
		speak(_("No list items."));
		return;
	}
	if (!result.found) {
		speak(next ? _("No next list item.") : _("No previous list item."));
		return;
	}
	const long offset = static_cast<long>(result.offset);
	text_ctrl->SetInsertionPoint(offset);
	const wxString current_line = rust_to_wx(rust::String(result.marker_text));
	wxString message = current_line;
	if (result.wrapped) message = (next ? _("Wrapping to start. ") : _("Wrapping to end. ")) + message;
	speak(message);
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
	if (tab == nullptr || text_ctrl == nullptr) return;
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
	const bool was_bookmarked = std::any_of(bookmarks.begin(), bookmarks.end(), [&](const bookmark& bm) {
		return bm == to_toggle;
	});
	config.toggle_bookmark(tab->file_path, bookmark_start, bookmark_end);
	config.flush();
	speak(was_bookmarked ? _("Bookmark removed") : _("Bookmarked"));
}

void document_manager::add_bookmark_with_note() const {
	const document_tab* tab = get_active_tab();
	wxTextCtrl* text_ctrl = get_active_text_ctrl();
	if (tab == nullptr || text_ctrl == nullptr) return;
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
	const auto bm_it = std::find_if(bookmarks.begin(), bookmarks.end(), [&](const bookmark& bm) {
		return bm.start == bookmark_start && bm.end == bookmark_end;
	});
	if (bm_it != bookmarks.end()) {
		bookmark_exists = true;
		existing_note = bm_it->note;
	}
	wxString prompt = bookmark_exists ? _("Edit bookmark note:") : _("Enter bookmark note:");
	note_entry_dialog note_dialog(nullptr, _("Bookmark Note"), prompt, existing_note);
	if (note_dialog.ShowModal() != wxID_OK) return;
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
	if (tab == nullptr || text_ctrl == nullptr || tab->session_doc == nullptr) return;
	const std::vector<bookmark> bookmarks = config.get_bookmarks(tab->file_path);
	if (bookmarks.empty()) {
		speak(_("No bookmarks"));
		return;
	}
	const int current_pos = text_ctrl->GetInsertionPoint();
	bookmark_dialog dialog(parent, tab->session_doc.get(), text_ctrl, config, tab->file_path, current_pos, initial_filter);
	const int result = dialog.ShowModal();
	if (result != wxID_OK) return;
	const int pos = dialog.get_selected_position();
	if (pos < 0) return;
	text_ctrl->SetInsertionPoint(pos);
	text_ctrl->SetFocus();
	wxString text_to_speak;
	wxString note_to_speak;
	const auto bm_it = std::find_if(bookmarks.begin(), bookmarks.end(), [&](const bookmark& bm) {
		return bm.start == pos;
	});
	if (bm_it != bookmarks.end()) {
		if (bm_it->is_whole_line()) {
			text_to_speak = rust_to_wx(session_get_line_text(*tab->get_session(), pos));
		} else {
			text_to_speak = rust_to_wx(session_get_text_range(*tab->get_session(), bm_it->start, bm_it->end));
		}
		note_to_speak = bm_it->note;
	}
	wxString announcement;
	if (!note_to_speak.IsEmpty()) announcement = wxString::Format(_("Bookmark: %s - %s"), note_to_speak, text_to_speak);
	else announcement = wxString::Format(_("Bookmark: %s"), text_to_speak);
	speak(announcement);
	update_ui();
}

void document_manager::show_table_of_contents(wxWindow* parent) {
	document_tab* tab = get_active_tab();
	wxTextCtrl* text_ctrl = get_active_text_ctrl();
	if (tab == nullptr || text_ctrl == nullptr || tab->session_doc == nullptr) return;
	if (!supports_feature(tab->session_doc->get_parser_flags(), PARSER_SUPPORTS_TOC)) {
		speak(_("No table of contents."));
		return;
	}
	tab->session_doc->ensure_toc_loaded();
	if (tab->session_doc->toc_items.empty()) {
		speak(_("Table of contents is empty."));
		return;
	}
	const int current_pos = text_ctrl->GetInsertionPoint();
	const int closest_toc_offset = static_cast<int>(tab->session_doc->find_closest_toc_offset(static_cast<size_t>(current_pos)));
	toc_dialog dlg(parent, tab->session_doc.get(), closest_toc_offset);
	if (dlg.ShowModal() != wxID_OK) return;
	const int offset = dlg.get_selected_offset();
	if (offset >= 0) {
		go_to_position(offset);
		text_ctrl->SetFocus();
	}
}

void document_manager::show_document_info(wxWindow* parent) {
	const document_tab* tab = get_active_tab();
	if (tab == nullptr) return;
	document_info_dialog dlg(parent, tab->session_doc.get(), tab->file_path, config);
	dlg.ShowModal();
	if (dlg.imported_position > -1) go_to_position(dlg.imported_position);
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
	if (tab == nullptr || tab->text_ctrl == nullptr) return;
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

wxString document_manager::get_status_text() const {
	if (!has_documents()) return _("Ready");
	const document_tab* tab = get_active_tab();
	const wxTextCtrl* text_ctrl = get_active_text_ctrl();
	if (tab == nullptr || text_ctrl == nullptr || tab->session_doc == nullptr) return _("Ready");
	const long current_pos = text_ctrl->GetInsertionPoint();
	const auto status = session_get_status_info(*tab->get_session(), current_pos);
	return wxString::Format(_("line %lld, character %lld, reading %d%%"), status.line_number, status.character_number, status.percentage);
}

wxString document_manager::get_window_title(const wxString& app_name) const {
	if (!has_documents()) return app_name;
	const document_tab* tab = get_active_tab();
	if (tab != nullptr && tab->session_doc != nullptr) return app_name + " - " + tab->session_doc->get_title();
	return app_name;
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
		if (tab == nullptr) continue;
		wxFileName tab_file(tab->file_path);
		tab_file.Normalize(static_cast<unsigned>(wxPATH_NORM_ABSOLUTE) | static_cast<unsigned>(wxPATH_NORM_LONG));
		if (tab_file.GetFullPath().IsSameAs(input_abs_path, false)) return i;
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
	if (tab == nullptr || tab->text_ctrl == nullptr) return;
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
	const document_tab* tab = get_active_tab();
	wxTextCtrl* text_ctrl = get_active_text_ctrl();
	if (tab == nullptr || text_ctrl == nullptr || tab->session_doc == nullptr) return;
	const bool wrap = config.get(config_manager::navigation_wrap);
	const auto result = session_navigate_heading(*tab->get_session(), text_ctrl->GetInsertionPoint(), wrap, next, specific_level);
	if (result.not_supported) {
		if (specific_level > 0) speak(wxString::Format(_("No level %d headings."), specific_level));
		else speak(_("No headings."));
		return;
	}
	if (!result.found) {
		if (specific_level > 0) speak(next ? wxString::Format(_("No next level %d heading."), specific_level) : wxString::Format(_("No previous level %d heading."), specific_level));
		else speak(next ? _("No next heading.") : _("No previous heading."));
		return;
	}
	const long offset = static_cast<long>(result.offset);
	text_ctrl->SetInsertionPoint(offset);
	const wxString heading_text = rust_to_wx(rust::String(result.marker_text));
	wxString message;
	if (result.wrapped) message = wxString::Format(_("Wrapping to %s. %s Heading level %d"), next ? _("start") : _("end"), heading_text, result.marker_level);
	else message = wxString::Format(_("%s Heading level %d"), heading_text, result.marker_level);
	speak(message);
}

void document_manager::navigate_to_table(bool next) const {
	const document_tab* tab = get_active_tab();
	wxTextCtrl* text_ctrl = get_active_text_ctrl();
	if (tab == nullptr || text_ctrl == nullptr || tab->session_doc == nullptr) return;
	const bool wrap = config.get(config_manager::navigation_wrap);
	const auto result = session_navigate_table(*tab->get_session(), text_ctrl->GetInsertionPoint(), wrap, next);
	if (result.not_supported) {
		speak(_("No tables."));
		return;
	}
	if (!result.found) {
		speak(next ? _("No next table.") : _("No previous table."));
		return;
	}
	const long offset = static_cast<long>(result.offset);
	text_ctrl->SetInsertionPoint(offset);
	// Use marker_text (caption or first row) if available, otherwise use line text.
	wxString message = rust_to_wx(rust::String(result.marker_text));
	if (result.wrapped) message = (next ? _("Wrapping to start. ") : _("Wrapping to end. ")) + message;
	speak(message);
}

void document_manager::go_to_previous_table() {
	navigate_to_table(false);
}

void document_manager::go_to_next_table() {
	navigate_to_table(true);
}

void document_manager::activate_current_table() {
	const document_tab* tab = get_active_tab();
	wxTextCtrl* text_ctrl = get_active_text_ctrl();
	if (tab == nullptr || text_ctrl == nullptr || tab->session_doc == nullptr) return;
	const int current_pos = text_ctrl->GetInsertionPoint();
	const rust::String table_html = session_get_table_at_position(*tab->session_doc->session, current_pos);
	const wxString table_content = rust_to_wx(table_html);
	if (table_content.IsEmpty()) return;
	web_view_dialog dlg(&main_win, _("Table"), table_content);
	dlg.ShowModal();
}
