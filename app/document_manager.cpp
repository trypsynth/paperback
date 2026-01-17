#include "document_manager.hpp"
#include "config_manager.hpp"
#include "constants.hpp"
#include "dialogs.hpp"
#include "document_data.hpp"
#include "main_window.hpp"
#include "parser.hpp"
#include "utils.hpp"
#include <algorithm>
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
	populate_toc_items(toc_items, session_toc_items_with_parents(*session));
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
		const std::string path_utf8 = path.ToUTF8().data();
		session_load_history_from_config(*session, config.backend_for_ffi(), path_utf8);
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
			const std::string path_utf8 = tab->file_path.ToUTF8().data();
			session_save_history_to_config(*tab->get_session(), config.backend_for_ffi_mut(), path_utf8);
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

namespace {
struct nav_announcements {
	wxString not_supported;
	wxString not_found_next;
	wxString not_found_prev;
	// For formatting found messages - %s is context_text, %d is context_index
	enum class found_format { text_only, text_with_index, text_with_level, page_format, link_format };
	found_format format{found_format::text_only};
};

nav_announcements get_nav_announcements(NavTarget target, int level_filter) {
	switch (target) {
		case NavTarget::Section:
			return {_("No sections."), _("No next section"), _("No previous section"), nav_announcements::found_format::text_only};
		case NavTarget::Heading:
			if (level_filter > 0) {
				return {
					wxString::Format(_("No headings at level %d."), level_filter),
					wxString::Format(_("No next heading at level %d."), level_filter),
					wxString::Format(_("No previous heading at level %d."), level_filter),
					nav_announcements::found_format::text_with_level
				};
			}
			return {_("No headings."), _("No next heading."), _("No previous heading."), nav_announcements::found_format::text_with_level};
		case NavTarget::Page:
			return {_("No pages."), _("No next page."), _("No previous page."), nav_announcements::found_format::page_format};
		case NavTarget::Link:
			return {_("No links."), _("No next link."), _("No previous link."), nav_announcements::found_format::link_format};
		case NavTarget::List:
			return {_("No lists."), _("No next list."), _("No previous list."), nav_announcements::found_format::text_only};
		case NavTarget::ListItem:
			return {_("No list items."), _("No next list item."), _("No previous list item."), nav_announcements::found_format::text_only};
		case NavTarget::Table:
			return {_("No tables."), _("No next table."), _("No previous table."), nav_announcements::found_format::text_only};
		default:
			return {_("Not supported."), _("Not found."), _("Not found."), nav_announcements::found_format::text_only};
	}
}

wxString format_nav_found_message(const nav_announcements& ann, const wxString& context_text, int context_index, bool wrapped, bool next) {
	wxString wrap_prefix;
	if (wrapped) wrap_prefix = next ? _("Wrapping to start. ") : _("Wrapping to end. ");
	switch (ann.format) {
		case nav_announcements::found_format::text_only:
			return wrap_prefix + context_text;
		case nav_announcements::found_format::text_with_level:
			return wrap_prefix + wxString::Format(_("%s Heading level %d"), context_text, context_index);
		case nav_announcements::found_format::page_format:
			return wrap_prefix + wxString::Format(_("Page %d: %s"), context_index + 1, context_text);
		case nav_announcements::found_format::link_format:
			return wrap_prefix + context_text + _(" link");
		case nav_announcements::found_format::text_with_index:
		default:
			return wrap_prefix + context_text;
	}
}
} // namespace

void document_manager::navigate_to_element(NavTarget target, bool next, int level_filter) const {
	const document_tab* tab = get_active_tab();
	wxTextCtrl* text_ctrl = get_active_text_ctrl();
	if (tab == nullptr || text_ctrl == nullptr || tab->session_doc == nullptr) return;
	const auto direction = next ? NavDirection::Next : NavDirection::Previous;
	const auto result = session_navigate_unified(*tab->get_session(), config.backend_for_ffi(), text_ctrl->GetInsertionPoint(), target, direction, level_filter);
	const auto ann = get_nav_announcements(target, level_filter);
	switch (result.outcome) {
		case NavOutcome::NotSupported:
			speak(ann.not_supported);
			return;
		case NavOutcome::NotFound:
			speak(next ? ann.not_found_next : ann.not_found_prev);
			return;
		case NavOutcome::Found:
		case NavOutcome::FoundWrapped: {
			text_ctrl->SetInsertionPoint(static_cast<long>(result.offset));
			const wxString context_text = rust_to_wx(result.context_text);
			const bool wrapped = result.outcome == NavOutcome::FoundWrapped;
			speak(format_nav_found_message(ann, context_text, result.context_index, wrapped, next));
			break;
		}
		default:
			break;
	}
}

void document_manager::navigate_to_section(bool next) const {
	navigate_to_element(NavTarget::Section, next);
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
	navigate_to_element(NavTarget::Page, next);
}

void document_manager::go_to_previous_page() const {
	navigate_to_page(false);
}

void document_manager::go_to_next_page() const {
	navigate_to_page(true);
}

void document_manager::navigate_to_bookmark_or_note(bool next, bool notes_only) const {
	const document_tab* tab = get_active_tab();
	wxTextCtrl* text_ctrl = get_active_text_ctrl();
	if (tab == nullptr || text_ctrl == nullptr || tab->session_doc == nullptr) return;
	const auto direction = next ? NavDirection::Next : NavDirection::Previous;
	const auto result = session_navigate_bookmark_unified(*tab->get_session(), config.backend_for_ffi(), text_ctrl->GetInsertionPoint(), direction, notes_only);
	if (result.outcome == NavOutcome::NotFound) {
		if (notes_only)
			speak(next ? _("No next note") : _("No previous note"));
		else
			speak(next ? _("No next bookmark") : _("No previous bookmark"));
		return;
	}
	text_ctrl->SetInsertionPoint(static_cast<long>(result.offset));
	const wxString snippet = rust_to_wx(result.context_text);
	const int index = result.context_index >= 0 ? result.context_index : 0;
	wxString announcement;
	if (notes_only) {
		const wxString note_text = rust_to_wx(result.secondary_text);
		if (!note_text.IsEmpty())
			announcement = wxString::Format(_("%s - %s - Note %d"), note_text, snippet, index + 1);
		else
			announcement = wxString::Format(_("%s - Note %d"), snippet, index + 1);
	} else {
		announcement = wxString::Format(_("%s - Bookmark %d"), snippet, index + 1);
	}
	if (result.outcome == NavOutcome::FoundWrapped)
		announcement = (next ? _("Wrapping to start. ") : _("Wrapping to end. ")) + announcement;
	speak(announcement);
}

void document_manager::navigate_to_bookmark(bool next) const {
	navigate_to_bookmark_or_note(next, false);
}

void document_manager::navigate_to_note(bool next) const {
	navigate_to_bookmark_or_note(next, true);
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
	navigate_to_element(NavTarget::Link, next);
}

void document_manager::go_to_previous_link() const {
	navigate_to_link(false);
}

void document_manager::go_to_next_link() const {
	navigate_to_link(true);
}

void document_manager::navigate_history(bool next) const {
	document_tab* tab = get_active_tab();
	wxTextCtrl* text_ctrl = get_active_text_ctrl();
	if (tab == nullptr || text_ctrl == nullptr || tab->session_doc == nullptr) return;
	const auto direction = next ? NavDirection::Next : NavDirection::Previous;
	const auto result = session_history_navigate(*tab->get_session(), config.backend_for_ffi(), text_ctrl->GetInsertionPoint(), direction);
	if (result.outcome == NavOutcome::Found) {
		go_to_position(static_cast<long>(result.offset));
		speak(next ? _("Navigated to next position.") : _("Navigated to previous position."));
	} else {
		speak(next ? _("No next position.") : _("No previous position."));
	}
}

void document_manager::go_to_previous_position() const {
	navigate_history(false);
}

void document_manager::go_to_next_position() const {
	navigate_history(true);
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
	navigate_to_element(NavTarget::List, next);
}

void document_manager::go_to_previous_list() const {
	navigate_to_list(false);
}

void document_manager::go_to_next_list() const {
	navigate_to_list(true);
}

void document_manager::navigate_to_list_item(bool next) const {
	navigate_to_element(NavTarget::ListItem, next);
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
	const bool was_bookmarked = config_manager_toggle_bookmark_with_result(config.backend_for_ffi_mut(), tab->file_path.ToUTF8().data(), bookmark_start, bookmark_end, "");
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
	auto info = bookmark_info(config.backend_for_ffi(), tab->file_path.ToUTF8().data(), bookmark_start, bookmark_end);
	const bool bookmark_exists = info.found;
	const wxString existing_note = wxString::FromUTF8(info.note.c_str());
	wxString prompt = bookmark_exists ? _("Edit bookmark note:") : _("Enter bookmark note:");
	note_entry_dialog note_dialog(nullptr, _("Bookmark Note"), prompt, existing_note);
	if (note_dialog.ShowModal() != wxID_OK) return;
	wxString note = note_dialog.get_note();
	const bool updated = config_manager_upsert_bookmark_note(config.backend_for_ffi_mut(), tab->file_path.ToUTF8().data(), bookmark_start, bookmark_end, note.ToUTF8().data());
	speak(updated ? _("Bookmark note updated") : _("Bookmarked with note"));
	config.flush();
}

void document_manager::show_bookmark_dialog(wxWindow* parent, bookmark_filter initial_filter) {
	const document_tab* tab = get_active_tab();
	wxTextCtrl* text_ctrl = get_active_text_ctrl();
	if (tab == nullptr || text_ctrl == nullptr || tab->session_doc == nullptr) return;
	if (bookmark_count(config.backend_for_ffi(), tab->file_path.ToUTF8().data()) == 0) {
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
	auto display = session_bookmark_display_at_position(*tab->get_session(), config.backend_for_ffi(), pos);
	const wxString text_to_speak = rust_to_wx(display.snippet);
	const wxString note_to_speak = wxString::FromUTF8(display.note.c_str());
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
	if (!session_supports_toc(*tab->session_doc->session)) {
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
	const long max_position = tab->text_ctrl->GetLastPosition();
	const long position = config.get_validated_document_position(tab->file_path, max_position);
	if (position >= 0) {
		tab->text_ctrl->SetInsertionPoint(position);
		tab->text_ctrl->ShowPosition(position);
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
	navigate_to_element(NavTarget::Heading, next, specific_level);
}

void document_manager::navigate_to_table(bool next) const {
	navigate_to_element(NavTarget::Table, next);
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
