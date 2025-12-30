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

int to_rust_marker(marker_type type) {
	return static_cast<int>(type);
}

std::vector<long> to_long_vector(const rust::Vec<long long>& values) {
	std::vector<long> result(values.size());
	std::transform(values.begin(), values.end(), result.begin(), [](long long value) {
		return static_cast<long>(value);
	});
	return result;
}

rust::Vec<long long> to_rust_history(const std::vector<long>& history) {
	rust::Vec<long long> rust_history;
	rust_history.reserve(history.size());
	std::transform(history.begin(), history.end(), std::back_inserter(rust_history), [](long value) {
		return static_cast<long long>(value);
	});
	return rust_history;
}

wxString to_wxstring(const rust::String& rust_str) {
	const std::string utf8 = std::string(rust_str);
	return wxString::FromUTF8(utf8.c_str());
}

marker to_marker(const FfiMarker& ffi_marker) {
	return marker{
		ffi_marker.position,
		static_cast<marker_type>(ffi_marker.marker_type),
		to_wxstring(ffi_marker.text),
		to_wxstring(ffi_marker.reference),
		ffi_marker.level,
	};
}

bool is_heading_marker(marker_type type) {
	switch (type) {
		case marker_type::Heading1:
		case marker_type::Heading2:
		case marker_type::Heading3:
		case marker_type::Heading4:
		case marker_type::Heading5:
		case marker_type::Heading6:
			return true;
		default:
			return false;
	}
}

std::optional<NavResult> perform_navigation(const document& doc, long position, NavTarget target, NavDirection direction, bool wrap, int level_filter = 0) {
	NavRequest request{};
	request.position = position;
	request.wrap = wrap;
	request.direction = direction;
	request.target = target;
	request.level_filter = level_filter;
	const NavResult result = reader_navigate(**doc.handle, request);
	if (!result.found) {
		return std::nullopt;
	}
	return result;
}

int doc_section_index(const document& doc, size_t position) {
	return document_current_marker(**doc.handle, position, to_rust_marker(marker_type::SectionBreak));
}

int doc_page_index(const document& doc, size_t position) {
	return document_current_marker(**doc.handle, position, to_rust_marker(marker_type::PageBreak));
}

size_t doc_find_closest_toc_offset(const document& doc, size_t position) {
	return document_find_closest_toc_offset(**doc.handle, position);
}

bool doc_has_heading_markers(const document& doc, int level = -1) {
	if (!doc.handle.has_value()) {
		return false;
	}
	const auto& handle = **doc.handle;
	const auto has_marker = [&](marker_type type) {
		return document_count_markers(handle, to_rust_marker(type)) > 0;
	};
	if (level >= 1 && level <= 6) {
		marker_type heading_type = marker_type::Heading1;
		switch (level) {
			case 1: heading_type = marker_type::Heading1; break;
			case 2: heading_type = marker_type::Heading2; break;
			case 3: heading_type = marker_type::Heading3; break;
			case 4: heading_type = marker_type::Heading4; break;
			case 5: heading_type = marker_type::Heading5; break;
			case 6: heading_type = marker_type::Heading6; break;
			default: break;
		}
		return has_marker(heading_type);
	}
	return has_marker(marker_type::Heading1) || has_marker(marker_type::Heading2) || has_marker(marker_type::Heading3)
		|| has_marker(marker_type::Heading4) || has_marker(marker_type::Heading5) || has_marker(marker_type::Heading6);
}

int doc_next_marker_index(const document& doc, long position, marker_type type) {
	return document_next_marker(**doc.handle, position, to_rust_marker(type));
}

int doc_previous_marker_index(const document& doc, long position, marker_type type) {
	return document_previous_marker(**doc.handle, position, to_rust_marker(type));
}

int doc_current_marker_index(const document& doc, size_t position, marker_type type) {
	return document_current_marker(**doc.handle, position, to_rust_marker(type));
}

int doc_find_first_marker_after(const document& doc, long position, marker_type type) {
	return document_find_first_marker_after(**doc.handle, position, to_rust_marker(type));
}

std::optional<std::string> current_section_path(const document& doc, size_t position) {
	const int section_index = doc_section_index(doc, position);
	if (section_index < 0) {
		return std::nullopt;
	}
	const auto idx = static_cast<size_t>(section_index);
	if (idx >= doc.spine_items.size()) {
		return std::nullopt;
	}
	const auto& manifest_id = doc.spine_items[idx];
	auto it = doc.manifest_items.find(manifest_id);
	if (it == doc.manifest_items.end()) {
		return std::nullopt;
	}
	return it->second;
}

std::optional<marker> doc_get_marker(const document& doc, int marker_index) {
	if (!doc.handle.has_value()) {
		return std::nullopt;
	}
	const auto result = document_marker_info(**doc.handle, marker_index);
	if (!result.found) {
		return std::nullopt;
	}
	return to_marker(result.marker);
}

void populate_toc_items(std::vector<std::unique_ptr<toc_item>>& toc_items, const rust::Vec<FfiTocItem>& ffi_toc_items) {
	if (ffi_toc_items.empty()) {
		return;
	}
	constexpr int MAX_DEPTH = 32;
	std::vector<std::vector<std::unique_ptr<toc_item>>*> depth_stacks(MAX_DEPTH + 1, nullptr);
	depth_stacks[0] = &toc_items;
	for (const auto& rust_toc : ffi_toc_items) {
		auto item = std::make_unique<toc_item>();
		item->name = to_wxstring(rust_toc.name);
		item->ref = to_wxstring(rust_toc.reference);
		item->offset = rust_toc.offset;
		const int depth = rust_toc.depth;
		if (depth < 0 || depth > MAX_DEPTH) {
			continue;
		}
		std::vector<std::unique_ptr<toc_item>>* parent_list = nullptr;
		const auto parent_it = std::find_if(depth_stacks.rbegin() + (MAX_DEPTH - depth), depth_stacks.rend(), [](const auto* stack) {
			return stack != nullptr;
		});
		if (parent_it != depth_stacks.rend()) {
			parent_list = *parent_it;
		}
		if (parent_list == nullptr) {
			parent_list = &toc_items;
		}
		parent_list->push_back(std::move(item));
		depth_stacks[depth + 1] = &parent_list->back()->children;
		for (int i = depth + 2; i <= MAX_DEPTH; ++i) {
			depth_stacks[i] = nullptr;
		}
	}
}

void ensure_toc_loaded(document& doc) {
	if (doc.toc_loaded) {
		return;
	}
	doc.toc_loaded = true;
	if (!doc.handle.has_value()) {
		return;
	}
	populate_toc_items(doc.toc_items, document_toc_items(**doc.handle));
}

size_t doc_marker_position(const document& doc, int marker_index) {
	return document_marker_position(**doc.handle, marker_index);
}

size_t doc_count_markers_by_type(const document& doc, marker_type type) {
	if (!doc.handle.has_value()) {
		return 0;
	}
	return document_count_markers(**doc.handle, to_rust_marker(type));
}

size_t doc_get_marker_position_by_index(const document& doc, marker_type type, int index) {
	if (!doc.handle.has_value()) {
		return 0;
	}
	return document_marker_position_by_index(**doc.handle, to_rust_marker(type), index);
}
} // namespace

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
	const parser_info* parser = find_parser_by_extension(wxFileName(path).GetExt());
	if (parser == nullptr) {
		parser = get_parser_for_unknown_file(path, config);
		if (parser == nullptr) {
			return false;
		}
	}
	if (!create_document_tab(path, parser, true, add_to_recent)) {
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

bool document_manager::create_document_tab(const wxString& path, const parser_info* parser, bool set_focus, bool add_to_recent) {
	if (parser == nullptr) {
		return false;
	}
	config.import_document_settings(path);
	const wxString forced_extension = config.get_document_format(path);
	std::unique_ptr<document> doc;
	wxString password_in_use;
	const wxString saved_password = config.get_document_password(path);
	auto load_document = [&](const std::optional<std::string>& password) {
		return load_document_from_rust(path, password, forced_extension);
	};
	try {
		std::optional<std::string> initial_password;
		if (!saved_password.IsEmpty()) {
			password_in_use = saved_password;
			initial_password = saved_password.ToUTF8().data();
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
				doc = load_document(std::optional<std::string>{entered_password.ToUTF8().data()});
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
	config.get_navigation_history(path, doc->history, doc->history_index);
	const auto rust_history = to_rust_history(doc->history);
	rust::Slice<const std::int64_t> history_slice(rust_history.data(), rust_history.size());
	const auto normalized = history_normalize(history_slice, doc->history_index);
	doc->history = to_long_vector(normalized.positions);
	doc->history_index = normalized.index;
	auto* tab_data = new document_tab;
	tab_data->doc = std::move(doc);
	tab_data->file_path = path;
	tab_data->parser = parser;
	wxPanel* panel = create_tab_panel(tab_data->doc->content, tab_data);
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

const parser_info* document_manager::get_active_parser() const {
	const document_tab* tab = get_active_tab();
	return tab != nullptr ? tab->parser : nullptr;
}

int document_manager::get_tab_count() const {
	return static_cast<int>(notebook->GetPageCount());
}

int document_manager::get_active_tab_index() const {
	return notebook->GetSelection();
}

int document_manager::page_index(size_t position) const {
	const document* doc = get_active_document();
	return doc != nullptr ? doc_page_index(*doc, position) : -1;
}

size_t document_manager::marker_count(marker_type type) const {
	const document* doc = get_active_document();
	return doc != nullptr ? doc_count_markers_by_type(*doc, type) : 0;
}

size_t document_manager::marker_position_by_index(marker_type type, int index) const {
	const document* doc = get_active_document();
	return doc != nullptr ? doc_get_marker_position_by_index(*doc, type, index) : 0;
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
	const document_tab* tab = get_active_tab();
	wxTextCtrl* text_ctrl = get_active_text_ctrl();
	if (tab == nullptr || text_ctrl == nullptr) return;
	if (tab->has_session()) {
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
		long line{0};
		text_ctrl->PositionToXY(offset, nullptr, &line);
		const wxString current_line = text_ctrl->GetLineText(line);
		if (result.wrapped)
			speak((next ? _("Wrapping to start. ") : _("Wrapping to end. ")) + current_line);
		else
			speak(current_line);
		return;
	}
	const document* doc = get_active_document();
	const parser_info* parser = get_active_parser();
	if (doc == nullptr || parser == nullptr) return;
	if (!parser_supports(parser->flags, parser_flags::supports_sections)) {
		speak(_("No sections."));
		return;
	}
	const bool wrap = config.get(config_manager::navigation_wrap);
	const auto nav = perform_navigation(*doc, text_ctrl->GetInsertionPoint(), NavTarget::Section, next ? NavDirection::Next : NavDirection::Previous, wrap);
	if (!nav.has_value()) {
		speak(next ? _("No next section") : _("No previous section"));
		return;
	}
	const long offset = static_cast<long>(nav->offset);
	text_ctrl->SetInsertionPoint(offset);
	long line{0};
	text_ctrl->PositionToXY(offset, nullptr, &line);
	const wxString current_line = text_ctrl->GetLineText(line);
	if (nav->wrapped) {
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
	if (doc_count_markers_by_type(*doc, marker_type::PageBreak) == 0) {
		speak(_("No pages."));
		return;
	}
	const bool wrap = config.get(config_manager::navigation_wrap);
	const auto nav = perform_navigation(*doc, text_ctrl->GetInsertionPoint(), NavTarget::Page, next ? NavDirection::Next : NavDirection::Previous, wrap);
	if (!nav.has_value()) {
		speak(next ? _("No next page.") : _("No previous page."));
		return;
	}
	const long offset = static_cast<long>(nav->offset);
	text_ctrl->SetInsertionPoint(offset);
	long line{0};
	text_ctrl->PositionToXY(offset, nullptr, &line);
	const wxString current_line = text_ctrl->GetLineText(line);
	const int page_idx = doc_page_index(*doc, nav->offset);
	wxString message = wxString::Format(_("Page %d: %s"), page_idx + 1, current_line);
	if (nav->wrapped) {
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

void document_manager::navigate_to_bookmark(bool next) const {
	const document_tab* tab = get_active_tab();
	wxTextCtrl* text_ctrl = get_active_text_ctrl();
	if (tab == nullptr || text_ctrl == nullptr) {
		return;
	}
	const bool allow_wrap = config.get(config_manager::navigation_wrap);
	const std::string path_utf8 = std::string(tab->file_path.ToUTF8().data());
	const auto result = bookmark_navigate(config.backend_for_ffi(), path_utf8, text_ctrl->GetInsertionPoint(), allow_wrap, next, false);
	if (!result.found) {
		speak(next ? _("No next bookmark") : _("No previous bookmark"));
		return;
	}
	text_ctrl->SetInsertionPoint(static_cast<long>(result.start));
	wxString text_to_speak;
	if (result.start == result.end) {
		long line{0};
		text_ctrl->PositionToXY(static_cast<long>(result.start), nullptr, &line);
		text_to_speak = text_ctrl->GetLineText(line);
	} else {
		text_to_speak = text_ctrl->GetRange(static_cast<long>(result.start), static_cast<long>(result.end));
	}
	const int index = result.index >= 0 ? result.index : 0;
	wxString announcement = wxString::Format(_("%s - Bookmark %d"), text_to_speak, index + 1);
	if (result.wrapped) {
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
	const bool allow_wrap = config.get(config_manager::navigation_wrap);
	const std::string path_utf8 = std::string(tab->file_path.ToUTF8().data());
	const auto result = bookmark_navigate(config.backend_for_ffi(), path_utf8, text_ctrl->GetInsertionPoint(), allow_wrap, next, true);
	if (!result.found) {
		speak(next ? _("No next note") : _("No previous note"));
		return;
	}
	const auto start = static_cast<long>(result.start);
	const auto end = static_cast<long>(result.end);
	text_ctrl->SetInsertionPoint(start);
	wxString text_to_speak;
	if (start == end) {
		long line{0};
		text_ctrl->PositionToXY(start, nullptr, &line);
		text_to_speak = text_ctrl->GetLineText(line);
	} else {
		text_to_speak = text_ctrl->GetRange(start, end);
	}
	auto note = result.note;
	const wxString note_text = wxString::FromUTF8(note.c_str());
	const int index = result.index >= 0 ? result.index : 0;
	wxString announcement = wxString::Format(_("%s - Note %d"), text_to_speak, index + 1);
	if (!note_text.IsEmpty()) {
		announcement = wxString::Format(_("%s - %s - Note %d"), note_text, text_to_speak, index + 1);
	}
	if (result.wrapped) {
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
	if (doc_count_markers_by_type(*doc, marker_type::Link) == 0) {
		speak(_("No links."));
		return;
	}
	const bool wrap = config.get(config_manager::navigation_wrap);
	const auto nav = perform_navigation(*doc, text_ctrl->GetInsertionPoint(), NavTarget::Link, next ? NavDirection::Next : NavDirection::Previous, wrap);
	if (!nav.has_value()) {
		speak(next ? _("No next link.") : _("No previous link."));
		return;
	}
	go_to_position(static_cast<long>(nav->offset));
	const std::string link_text_utf8 = std::string(nav->marker_text);
	wxString message = wxString::FromUTF8(link_text_utf8.c_str()) + _(" link");
	if (nav->wrapped) {
		message = (next ? _("Wrapping to start. ") : _("Wrapping to end. ")) + message;
	}
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
	if (tab == nullptr || text_ctrl == nullptr) return;
	if (tab->has_session()) {
		const long actual_pos = text_ctrl->GetInsertionPoint();
		auto result = session_history_go_back(*tab->get_session(), actual_pos);
		if (result.found) {
			go_to_position(static_cast<long>(result.offset));
			speak(_("Navigated to previous position."));
		} else {
			speak(_("No previous position."));
		}
		return;
	}
	document* doc = get_active_document();
	if (doc == nullptr) {
		return;
	}
	if (doc->history.empty()) {
		speak(_("No previous position."));
		return;
	}
	const long actual_pos = text_ctrl->GetInsertionPoint();
	constexpr size_t max_history = 10;
	const auto rust_history = to_rust_history(doc->history);
	rust::Slice<const std::int64_t> history_slice(rust_history.data(), rust_history.size());
	const auto result = history_go_previous(history_slice, doc->history_index, actual_pos, max_history);
	doc->history = to_long_vector(result.positions);
	doc->history_index = result.index;
	if (result.found) {
		go_to_position(static_cast<long>(result.target));
		speak(_("Navigated to previous position."));
	} else {
		speak(_("No previous position."));
	}
}

void document_manager::go_to_next_position() const {
	document_tab* tab = get_active_tab();
	wxTextCtrl* text_ctrl = get_active_text_ctrl();
	if (tab == nullptr || text_ctrl == nullptr) return;
	if (tab->has_session()) {
		const long actual_pos = text_ctrl->GetInsertionPoint();
		auto result = session_history_go_forward(*tab->get_session(), actual_pos);
		if (result.found) {
			go_to_position(static_cast<long>(result.offset));
			speak(_("Navigated to next position."));
		} else {
			speak(_("No next position."));
		}
		return;
	}
	document* doc = get_active_document();
	if (doc == nullptr) {
		return;
	}
	if (doc->history.empty()) {
		speak(_("No next position."));
		return;
	}
	const long actual_pos = text_ctrl->GetInsertionPoint();
	constexpr size_t max_history = 10;
	const auto rust_history = to_rust_history(doc->history);
	rust::Slice<const std::int64_t> history_slice(rust_history.data(), rust_history.size());
	const auto result = history_go_next(history_slice, doc->history_index, actual_pos, max_history);
	doc->history = to_long_vector(result.positions);
	doc->history_index = result.index;
	if (result.found) {
		go_to_position(static_cast<long>(result.target));
		speak(_("Navigated to next position."));
	} else {
		speak(_("No next position."));
	}
}

void document_manager::activate_current_link() const {
	document_tab* tab = get_active_tab();
	const wxTextCtrl* text_ctrl = get_active_text_ctrl();
	if (tab == nullptr || text_ctrl == nullptr) return;
	if (tab->has_session()) {
		const long current_pos = text_ctrl->GetInsertionPoint();
		auto result = session_activate_link(*tab->get_session(), current_pos);
		if (!result.found) return;  // No link at current position
		switch (result.action) {
			case FfiLinkAction::External:
				if (wxLaunchDefaultBrowser(rust_to_wx(rust::String(result.url))))
					speak(_("Opening link in default browser."));
				else
					speak(_("Failed to open link."));
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
		return;
	}
	document* doc = get_active_document();
	if (doc == nullptr) {
		return;
	}
	const long current_pos = text_ctrl->GetInsertionPoint();
	const int link_index = doc_current_marker_index(*doc, static_cast<size_t>(current_pos), marker_type::Link);
	if (link_index == -1) {
		return;
	}
	const auto link_marker = doc_get_marker(*doc, link_index);
	if (!link_marker.has_value()) {
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
	constexpr size_t max_history = 10;
	const auto rust_history = to_rust_history(doc->history);
	rust::Slice<const std::int64_t> history_slice(rust_history.data(), rust_history.size());
	const auto updated = history_record_position(history_slice, doc->history_index, current_pos, max_history);
	doc->history = to_long_vector(updated.positions);
	doc->history_index = updated.index;
	const wxString href_lower = href.Lower();
	if (href_lower.StartsWith("http:") || href_lower.StartsWith("https:") || href_lower.StartsWith("mailto:")) {
		if (wxLaunchDefaultBrowser(href)) {
			speak(_("Opening link in default browser."));
		} else {
			speak(_("Failed to open link."));
		}
	} else {
		auto resolution = resolve_link(**doc->handle, std::string(href.mb_str()), current_pos);
		if (!resolution.found) {
			speak(_("Internal link target not found."));
			return;
		}
		if (resolution.is_external) {
			if (wxLaunchDefaultBrowser(wxString::FromUTF8(resolution.url.c_str()))) {
				speak(_("Opening link in default browser."));
			} else {
				speak(_("Failed to open link."));
			}
			return;
		}
		go_to_position(static_cast<long>(resolution.offset));
		speak(_("Navigated to internal link."));
	}
}

void document_manager::navigate_to_list(bool next) const {
	const document* doc = get_active_document();
	wxTextCtrl* text_ctrl = get_active_text_ctrl();
	const parser_info* parser = get_active_parser();
	if (doc == nullptr || text_ctrl == nullptr || parser == nullptr) {
		return;
	}
	if (!parser_supports(parser->flags, parser_flags::supports_lists)) {
		speak(_("No lists."));
		return;
	}
	if (doc_count_markers_by_type(*doc, marker_type::List) == 0) {
		speak(_("No lists."));
		return;
	}
	const bool wrap = config.get(config_manager::navigation_wrap);
	const auto nav = perform_navigation(*doc, text_ctrl->GetInsertionPoint(), NavTarget::List, next ? NavDirection::Next : NavDirection::Previous, wrap);
	if (!nav.has_value()) {
		speak(next ? _("No next list.") : _("No previous list."));
		return;
	}
	go_to_position(static_cast<long>(nav->offset));
	const int list_marker_index = doc_current_marker_index(*doc, nav->offset, marker_type::List);
	const auto list_marker = doc_get_marker(*doc, list_marker_index);
	int list_size = nav->marker_level;
	if (list_marker.has_value() && list_marker->level > 0) {
		list_size = list_marker->level;
	}
	wxString message = wxString::Format(_("List with %d items"), list_size);
	if (list_marker.has_value()) {
		const int first_item_index = doc_find_first_marker_after(*doc, static_cast<long>(list_marker->pos), marker_type::ListItem);
		const auto first_item_marker = doc_get_marker(*doc, first_item_index);
		if (first_item_marker.has_value()) {
			long line_num{0};
			text_ctrl->PositionToXY(static_cast<long>(first_item_marker->pos), nullptr, &line_num);
			wxString line_text = text_ctrl->GetLineText(line_num).Trim();
			if (!line_text.IsEmpty()) {
				message += " " + line_text;
			}
		}
	}
	if (nav->wrapped) {
		message = (next ? _("Wrapping to start. ") : _("Wrapping to end. ")) + message;
	}
	speak(message);
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
	const parser_info* parser = get_active_parser();
	if (doc == nullptr || text_ctrl == nullptr || parser == nullptr) {
		return;
	}
	if (!parser_supports(parser->flags, parser_flags::supports_lists)) {
		speak(_("No lists."));
		return;
	}
	if (doc_count_markers_by_type(*doc, marker_type::ListItem) == 0) {
		speak(_("No list items."));
		return;
	}
	const int current_pos = text_ctrl->GetInsertionPoint();
	const int current_list_index = doc_current_marker_index(*doc, static_cast<size_t>(current_pos), marker_type::List);
	const bool wrap = config.get(config_manager::navigation_wrap);
	const auto nav = perform_navigation(*doc, current_pos, NavTarget::ListItem, next ? NavDirection::Next : NavDirection::Previous, wrap);
	if (!nav.has_value()) {
		speak(next ? _("No next list item.") : _("No previous list item."));
		return;
	}
	go_to_position(static_cast<long>(nav->offset));
	const int target_list_index = doc_current_marker_index(*doc, nav->offset, marker_type::List);
	const auto target_list_marker = doc_get_marker(*doc, target_list_index);
	long line_num{0};
	text_ctrl->PositionToXY(static_cast<long>(nav->offset), nullptr, &line_num);
	wxString message = text_ctrl->GetLineText(line_num).Trim();
	if (target_list_index != -1 && target_list_index != current_list_index && target_list_marker.has_value()
		&& target_list_marker->level > 0) {
		message = wxString::Format(_("List with %d items "), target_list_marker->level) + message;
	}
	if (nav->wrapped) {
		message = (next ? _("Wrapping to start. ") : _("Wrapping to end. ")) + message;
	}
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
	const auto bm_it = std::find_if(bookmarks.begin(), bookmarks.end(), [&](const bookmark& bm) {
		return bm.start == bookmark_start && bm.end == bookmark_end;
	});
	if (bm_it != bookmarks.end()) {
		bookmark_exists = true;
		existing_note = bm_it->note;
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
	const auto bm_it = std::find_if(bookmarks.begin(), bookmarks.end(), [&](const bookmark& bm) {
		return bm.start == pos;
	});
	if (bm_it != bookmarks.end()) {
		if (bm_it->is_whole_line()) {
			long line{0};
			text_ctrl->PositionToXY(pos, nullptr, &line);
			text_to_speak = text_ctrl->GetLineText(line);
		} else {
			text_to_speak = text_ctrl->GetRange(bm_it->start, bm_it->end);
		}
		note_to_speak = bm_it->note;
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

void document_manager::show_table_of_contents(wxWindow* parent) {
	document* doc = get_active_document();
	wxTextCtrl* text_ctrl = get_active_text_ctrl();
	const parser_info* parser = get_active_parser();
	if (doc == nullptr || text_ctrl == nullptr || parser == nullptr) {
		return;
	}
	if (!parser_supports(parser->flags, parser_flags::supports_toc)) {
		speak(_("No table of contents."));
		return;
	}
	ensure_toc_loaded(*doc);
	if (doc->toc_items.empty()) {
		speak(_("Table of contents is empty."));
		return;
	}
	const int current_pos = text_ctrl->GetInsertionPoint();
	const int closest_toc_offset = static_cast<int>(doc_find_closest_toc_offset(*doc, static_cast<size_t>(current_pos)));
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
	if (!doc_has_heading_markers(*doc, specific_level)) {
		if (specific_level == -1) {
			speak(_("No headings."));
		} else {
			speak(wxString::Format(_("No headings at level %d."), specific_level));
		}
		return;
	}
	const bool wrap = config.get(config_manager::navigation_wrap);
	const auto nav = perform_navigation(*doc, text_ctrl->GetInsertionPoint(), NavTarget::Heading, next ? NavDirection::Next : NavDirection::Previous, wrap, specific_level);
	if (!nav.has_value()) {
		const wxString msg = (specific_level == -1) ? wxString::Format(_("No %s heading"), next ? _("next") : _("previous")) : wxString::Format(_("No %s heading at level %d"), next ? _("next") : _("previous"), specific_level);
		speak(msg);
		return;
	}
	const long offset = static_cast<long>(nav->offset);
	text_ctrl->SetInsertionPoint(offset);
	wxString message;
	const std::string heading_text_utf8 = std::string(nav->marker_text);
	const wxString heading_wx = wxString::FromUTF8(heading_text_utf8.c_str());
	if (nav->wrapped) {
		message = wxString::Format(_("Wrapping to %s. %s Heading level %d"), next ? _("start") : _("end"), heading_wx, nav->marker_level);
	} else {
		message = wxString::Format(_("%s Heading level %d"), heading_wx, nav->marker_level);
	}
	speak(message);
}

void document_manager::navigate_to_table(bool next) const {
	const document* doc = get_active_document();
	wxTextCtrl* text_ctrl = get_active_text_ctrl();
	if (doc == nullptr || text_ctrl == nullptr) return;
	if (doc_count_markers_by_type(*doc, marker_type::Table) == 0) {
		speak(_("No tables."));
		return;
	}
	const bool wrap = config.get(config_manager::navigation_wrap);
	const auto nav = perform_navigation(*doc, text_ctrl->GetInsertionPoint(), NavTarget::Table, next ? NavDirection::Next : NavDirection::Previous, wrap);
	if (!nav.has_value()) {
		speak(next ? _("No next table.") : _("No previous table."));
		return;
	}
	go_to_position(static_cast<long>(nav->offset));
	const std::string table_text_utf8 = std::string(nav->marker_text);
	wxString message = wxString::FromUTF8(table_text_utf8.c_str());
	if (nav->wrapped) message = (next ? _("Wrapping to start. ") : _("Wrapping to end. ")) + message;
	speak(message);
}

void document_manager::go_to_previous_table() {
	navigate_to_table(false);
}

void document_manager::go_to_next_table() {
	navigate_to_table(true);
}

void document_manager::activate_current_table() {
	const document* doc = get_active_document();
	wxTextCtrl* text_ctrl = get_active_text_ctrl();
	if (doc == nullptr || text_ctrl == nullptr) return;
	const int current_pos = text_ctrl->GetInsertionPoint();
	const int table_index = doc_current_marker_index(*doc, static_cast<size_t>(current_pos), marker_type::Table);
	if (table_index == -1) return;
	const auto table_marker = doc_get_marker(*doc, table_index);
	if (!table_marker.has_value()) return;
	if (static_cast<size_t>(current_pos) < table_marker->pos || static_cast<size_t>(current_pos) > (table_marker->pos + table_marker->text.length())) return;
	table_dialog dlg(&main_win, _("Table"), table_marker->ref);
	dlg.ShowModal();
}
