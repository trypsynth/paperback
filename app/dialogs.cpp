/* dialogs.cpp - all of Paperback's wxDialog implementations.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#include "dialogs.hpp"
#include "config_manager.hpp"
#include "constants.hpp"
#include "controls.hpp"
#include "document_data.hpp"
#include "parser.hpp"
#include "translation_manager.hpp"
#include <algorithm>
#include <climits>
#include <cmath>
#include <cstddef>
#include <vector>
#include <wx/arrstr.h>
#include <wx/combobox.h>
#include <wx/defs.h>
#include <wx/dialog.h>
#include <wx/dynarray.h>
#include <wx/event.h>
#include <wx/filedlg.h>
#include <wx/filename.h>
#include <wx/listbox.h>
#include <wx/listctrl.h>
#include <wx/msgdlg.h>
#include <wx/sizer.h>
#include <wx/stattext.h>
#include <wx/string.h>
#include <wx/textctrl.h>
#include <wx/textdlg.h>
#include <wx/translation.h>
#include <wx/window.h>

namespace {
bool is_heading_marker(marker_type type) {
	return type >= marker_type::Heading1 && type <= marker_type::Heading6;
}

std::vector<const marker*> markers_by_type(const document* doc, marker_type type) {
	std::vector<const marker*> result;
	if (doc == nullptr) {
		return result;
	}
	for (const auto& m : doc->markers) {
		if (m.type == type) {
			result.push_back(&m);
		}
	}
	return result;
}

std::vector<const marker*> heading_markers(const document* doc) {
	std::vector<const marker*> result;
	if (doc == nullptr) {
		return result;
	}
	result.reserve(doc->markers.size());
	for (const auto& m : doc->markers) {
		if (is_heading_marker(m.type)) {
			result.push_back(&m);
		}
	}
	return result;
}

size_t count_markers(const document* doc, marker_type type) {
	return doc == nullptr ? 0U : static_cast<size_t>(std::count_if(doc->markers.begin(), doc->markers.end(), [type](const marker& m) {
		return m.type == type;
	}));
}
} // namespace

dialog::dialog(wxWindow* parent, const wxString& title, dialog_button_config buttons) : wxDialog(parent, wxID_ANY, title), main_sizer{new wxBoxSizer(wxVERTICAL)}, button_config{buttons} {
	SetSizer(main_sizer);
}

void dialog::set_content(wxSizer* content_sizer) {
	if (layout_finalized) {
		return;
	}
	main_sizer->Add(content_sizer, 1, wxEXPAND | wxALL, DIALOG_PADDING);
}

void dialog::finalize_layout() {
	if (layout_finalized) {
		return;
	}
	create_buttons();
	main_sizer->Add(button_sizer, 0, wxALIGN_RIGHT | wxALL, DIALOG_PADDING);
	SetSizerAndFit(main_sizer);
	CentreOnParent();
	layout_finalized = true;
}

void dialog::create_buttons() {
	button_sizer = new wxStdDialogButtonSizer();
	auto* ok_button = new wxButton(this, wxID_OK);
	button_sizer->AddButton(ok_button);
	if (button_config == dialog_button_config::ok_cancel) {
		button_sizer->AddButton(new wxButton(this, wxID_CANCEL));
	}
	ok_button->SetDefault();
	button_sizer->Realize();
}

all_documents_dialog::all_documents_dialog(wxWindow* parent, config_manager& cfg_mgr, const wxArrayString& open_docs) : dialog(parent, _("All Documents"), dialog_button_config::ok_only), config_mgr(cfg_mgr), open_doc_paths(open_docs) {
	auto* content_sizer = new wxBoxSizer(wxVERTICAL);
	auto* search_sizer = new wxBoxSizer(wxHORIZONTAL);
	auto* search_label = new wxStaticText(this, wxID_ANY, _("&search"));
	search_ctrl = new wxTextCtrl(this, wxID_ANY, wxEmptyString, wxDefaultPosition, wxSize(300, -1));
	search_sizer->Add(search_label, 0, wxALIGN_CENTER_VERTICAL | wxRIGHT, 10);
	search_sizer->Add(search_ctrl, 1, wxALIGN_CENTER_VERTICAL | wxRIGHT, 5);
	content_sizer->Add(search_sizer, 0, wxEXPAND | wxALL, DIALOG_PADDING);
	constexpr int list_width = 800;
	constexpr int list_height = 600;
	constexpr int filename_column_width = 250;
	constexpr int status_column_width = 100;
	constexpr int path_column_width = 450;
	doc_list = new wxListView(this, wxID_ANY, wxDefaultPosition, wxSize(list_width, list_height), wxLC_REPORT | wxLC_SINGLE_SEL);
	doc_list->AppendColumn(_("File Name"), wxLIST_FORMAT_LEFT, filename_column_width);
	doc_list->AppendColumn(_("Status"), wxLIST_FORMAT_LEFT, status_column_width);
	doc_list->AppendColumn(_("Path"), wxLIST_FORMAT_LEFT, path_column_width);
	populate_document_list();
	content_sizer->Add(doc_list, 1, wxEXPAND | wxALL, DIALOG_PADDING);
	doc_list->SetFocus();
	auto* action_sizer = new wxBoxSizer(wxHORIZONTAL);
	open_button = new wxButton(this, wxID_OPEN, _("&Open"));
	remove_button = new wxButton(this, wxID_REMOVE, _("&Remove"));
	action_sizer->Add(open_button, 0, wxRIGHT, DIALOG_PADDING);
	action_sizer->Add(remove_button, 0, wxRIGHT, DIALOG_PADDING);
	content_sizer->Add(action_sizer, 0, wxALIGN_LEFT | wxLEFT | wxRIGHT | wxBOTTOM, DIALOG_PADDING);
	set_content(content_sizer);
	finalize_layout();
	Bind(wxEVT_BUTTON, &all_documents_dialog::on_open, this, wxID_OPEN);
	Bind(wxEVT_BUTTON, &all_documents_dialog::on_remove, this, wxID_REMOVE);
	Bind(wxEVT_TEXT, &all_documents_dialog::on_search, this, wxID_ANY);
	Bind(wxEVT_LIST_ITEM_ACTIVATED, &all_documents_dialog::on_list_item_activated, this, wxID_ANY);
	Bind(wxEVT_LIST_ITEM_SELECTED, &all_documents_dialog::on_list_item_selected, this, wxID_ANY);
	doc_list->Bind(wxEVT_KEY_DOWN, &all_documents_dialog::on_key_down, this);
	if (doc_list->GetItemCount() > 0) {
		const long item = doc_list->GetNextItem(-1, wxLIST_NEXT_ALL, wxLIST_STATE_SELECTED);
		if (item != -1) {
			const wxString status = doc_list->GetItemText(item, 1);
			open_button->Enable(status != _("Missing"));
		}
	} else {
		open_button->Enable(false);
		remove_button->Enable(false);
	}
}

void all_documents_dialog::on_open(wxCommandEvent& /*event*/) {
	const long item = doc_list->GetNextItem(-1, wxLIST_NEXT_ALL, wxLIST_STATE_SELECTED);
	if (item != -1) {
		const wxString path = doc_list->GetItemText(item, 2);
		if (wxFileName::FileExists(path)) {
			selected_path = path;
			EndModal(wxID_OK);
		}
	}
}

void all_documents_dialog::on_remove(wxCommandEvent& /*event*/) {
	const long item = doc_list->GetNextItem(-1, wxLIST_NEXT_ALL, wxLIST_STATE_SELECTED);
	if (item == -1) {
		return;
	}
	if (wxMessageBox(_("Are you sure you want to remove this document from the list? This will also remove its reading position."), _("Confirm"), wxYES_NO | wxICON_INFORMATION) != wxYES) {
		return;
	}
	const wxString path_to_remove = doc_list->GetItemText(item, 2);
	const long removed_index = item;
	config_mgr.remove_document_history(path_to_remove);
	config_mgr.flush();
	populate_document_list();
	if (doc_list->GetItemCount() > 0) {
		long new_selection = removed_index;
		if (new_selection >= doc_list->GetItemCount()) {
			new_selection = doc_list->GetItemCount() - 1;
		}
		doc_list->SetItemState(new_selection, wxLIST_STATE_SELECTED | wxLIST_STATE_FOCUSED, wxLIST_STATE_SELECTED | wxLIST_STATE_FOCUSED);
		doc_list->EnsureVisible(new_selection);
	}
}

void all_documents_dialog::on_search(wxCommandEvent& /*event*/) {
	const wxString filter = search_ctrl->GetValue();
	populate_document_list(filter);
}

void all_documents_dialog::on_list_item_activated(wxListEvent& event) {
	const wxString path = doc_list->GetItemText(event.GetIndex(), 2);
	if (wxFileName::FileExists(path)) {
		selected_path = path;
		EndModal(wxID_OK);
	}
}

void all_documents_dialog::on_list_item_selected(wxListEvent& event) {
	const long item = event.GetIndex();
	if (item != -1 && open_button != nullptr) {
		const wxString status = doc_list->GetItemText(item, 1);
		open_button->Enable(status != _("Missing"));
	}
}

void all_documents_dialog::on_key_down(wxKeyEvent& event) {
	const int key = event.GetKeyCode();
	if (key == WXK_DELETE || key == WXK_NUMPAD_DELETE) {
		const wxCommandEvent remove_event(wxEVT_BUTTON, wxID_REMOVE);
		wxPostEvent(this, remove_event);
	} else {
		event.Skip();
	}
}

void all_documents_dialog::populate_document_list(const wxString& filter) {
	doc_list->DeleteAllItems();
	const wxArrayString recent = config_mgr.get_recent_documents();
	const wxArrayString all = config_mgr.get_all_documents();
	doc_paths.Clear();
	for (const auto& path : recent) {
		if (doc_paths.Index(path) == wxNOT_FOUND) {
			doc_paths.Add(path);
		}
	}
	std::vector<wxString> rest;
	rest.reserve(all.GetCount());
	std::copy_if(all.begin(), all.end(), std::back_inserter(rest), [&](const wxString& path) {
		return doc_paths.Index(path) == wxNOT_FOUND;
	});
	std::sort(rest.begin(), rest.end(), [](const wxString& a, const wxString& b) {
		const wxString an = wxFileName(a).GetFullName();
		const wxString bn = wxFileName(b).GetFullName();
		const int cmp = an.CmpNoCase(bn);
		if (cmp != 0) {
			return cmp < 0;
		}
		return a.CmpNoCase(b) < 0;
	});
	for (const auto& path : rest) {
		doc_paths.Add(path);
	}
	for (const auto& path : doc_paths) {
		const wxFileName fn(path);
		if (!filter.IsEmpty() && fn.GetFullName().Lower().Find(filter.Lower()) == wxNOT_FOUND) {
			continue;
		}
		const long index = doc_list->InsertItem(doc_list->GetItemCount(), fn.GetFullName());
		wxString status;
		if (!wxFileName::FileExists(path)) {
			status = _("Missing");
		} else if (open_doc_paths.Index(path) != wxNOT_FOUND) {
			status = _("Open");
		} else {
			status = _("Closed");
		}
		doc_list->SetItem(index, 1, status);
		doc_list->SetItem(index, 2, path);
	}
	if (doc_list->GetItemCount() > 0) {
		doc_list->SetItemState(0, wxLIST_STATE_SELECTED | wxLIST_STATE_FOCUSED, wxLIST_STATE_SELECTED | wxLIST_STATE_FOCUSED);
		doc_list->EnsureVisible(0);
		if (open_button != nullptr) {
			const wxString status = doc_list->GetItemText(0, 1);
			open_button->Enable(status != _("Missing"));
		}
		if (remove_button != nullptr) {
			remove_button->Enable(true);
		}
	} else {
		if (open_button != nullptr) {
			open_button->Enable(false);
		}
		if (remove_button != nullptr) {
			remove_button->Enable(false);
		}
	}
}

bookmark_dialog::bookmark_dialog(wxWindow* parent, const std::vector<bookmark>& bookmarks, wxTextCtrl* text_ctrl, config_manager& config, const wxString& file_path, long current_pos, bookmark_filter initial_filter) : dialog(parent, _("Jump to Bookmark"), dialog_button_config::ok_cancel), all_bookmarks{bookmarks}, selected_position{-1}, config{config}, file_path{file_path}, text_ctrl{text_ctrl} {
	auto* filter_row = new wxBoxSizer(wxHORIZONTAL);
	auto* filter_label = new wxStaticText(this, wxID_ANY, _("&Filter:"));
	filter_choice = new wxChoice(this, wxID_ANY);
	filter_choice->Append(_("All"));
	filter_choice->Append(_("Bookmarks"));
	filter_choice->Append(_("Notes"));
	int initial_index{0};
	switch (initial_filter) {
		case bookmark_filter::all:
			initial_index = 0;
			break;
		case bookmark_filter::bookmarks_only:
			initial_index = 1;
			break;
		case bookmark_filter::notes_only:
			initial_index = 2;
			break;
	}
	filter_choice->SetSelection(initial_index);
	filter_row->Add(filter_label, 0, wxALIGN_CENTER_VERTICAL | wxRIGHT, 6);
	filter_row->Add(filter_choice, 1, wxEXPAND);
	auto* content_sizer = new wxBoxSizer(wxVERTICAL);
	bookmark_list = new wxListBox(this, wxID_ANY);
	content_sizer->Add(filter_row, 0, wxEXPAND | wxALL, DIALOG_PADDING);
	content_sizer->Add(bookmark_list, 1, wxEXPAND | wxLEFT | wxRIGHT | wxBOTTOM, DIALOG_PADDING);
	auto* action_sizer = new wxStdDialogButtonSizer();
	edit_note_button = new wxButton(this, wxID_EDIT, _("&Edit Note"));
	delete_button = new wxButton(this, wxID_DELETE, _("&Delete"));
	jump_button = new wxButton(this, wxID_OK, _("&Jump"));
	auto* cancel_button = new wxButton(this, wxID_CANCEL, _("&Cancel"));
	action_sizer->AddButton(edit_note_button);
	action_sizer->AddButton(delete_button);
	action_sizer->AddButton(jump_button);
	action_sizer->AddButton(cancel_button);
	action_sizer->Realize();
	content_sizer->Add(action_sizer, 0, wxALIGN_RIGHT | wxALL, DIALOG_PADDING);
	set_content(content_sizer);
	SetSizerAndFit(main_sizer);
	CentreOnParent();
	jump_button->SetDefault();
	jump_button->Enable(false);
	delete_button->Enable(false);
	edit_note_button->Enable(false);
	repopulate_list(current_pos);
	bookmark_list->SetFocus();
	filter_choice->Bind(wxEVT_CHOICE, &bookmark_dialog::on_filter_changed, this);
	bookmark_list->Bind(wxEVT_LISTBOX, &bookmark_dialog::on_list_selection_changed, this);
	bookmark_list->Bind(wxEVT_KEY_DOWN, &bookmark_dialog::on_key_down, this);
	Bind(wxEVT_BUTTON, &bookmark_dialog::on_ok, this, wxID_OK);
	Bind(wxEVT_BUTTON, &bookmark_dialog::on_delete, this, wxID_DELETE);
	Bind(wxEVT_BUTTON, &bookmark_dialog::on_edit_note, this, wxID_EDIT);
}

void bookmark_dialog::on_list_selection_changed(wxCommandEvent& /*event*/) {
	const int selection = bookmark_list->GetSelection();
	if (selection >= 0 && static_cast<size_t>(selection) < bookmark_positions.size()) {
		selected_position = bookmark_positions[static_cast<std::size_t>(selection)].start;
		jump_button->Enable(true);
		delete_button->Enable(true);
		edit_note_button->Enable(true);
	} else {
		selected_position = -1;
		jump_button->Enable(false);
		delete_button->Enable(false);
		edit_note_button->Enable(false);
	}
}

void bookmark_dialog::on_ok(wxCommandEvent& /*event*/) {
	if (selected_position >= 0) {
		EndModal(wxID_OK);
	} else {
		wxMessageBox(_("Please select a bookmark to jump to."), _("Error"), wxICON_ERROR);
	}
}

void bookmark_dialog::on_key_down(wxKeyEvent& event) {
	const int key = event.GetKeyCode();
	if (key == WXK_DELETE || key == WXK_NUMPAD_DELETE) {
		const wxCommandEvent remove_event(wxEVT_BUTTON, wxID_DELETE);
		wxPostEvent(this, remove_event);
	} else {
		event.Skip();
	}
}

void bookmark_dialog::on_delete(wxCommandEvent&) {
	const int selection = bookmark_list->GetSelection();
	if (selection < 0) {
		return;
	}
	const bookmark& deleted_bookmark = bookmark_positions[static_cast<std::size_t>(selection)];
	config.remove_bookmark(file_path, deleted_bookmark.start, deleted_bookmark.end);
	config.flush();
	const auto it = std::find_if(all_bookmarks.begin(), all_bookmarks.end(), [&](const bookmark& bm) {
		return bm.start == deleted_bookmark.start && bm.end == deleted_bookmark.end;
	});
	if (it != all_bookmarks.end()) {
		all_bookmarks.erase(it);
	}
	repopulate_list(text_ctrl ? text_ctrl->GetInsertionPoint() : -1);
}

void bookmark_dialog::on_edit_note(wxCommandEvent&) {
	const int selection = bookmark_list->GetSelection();
	if (selection < 0 || static_cast<size_t>(selection) >= bookmark_positions.size()) {
		return;
	}
	const bookmark& selected_bookmark = bookmark_positions[static_cast<std::size_t>(selection)];
	note_entry_dialog note_dialog(this, _("Bookmark Note"), _("Edit bookmark note:"), selected_bookmark.note);
	if (note_dialog.ShowModal() != wxID_OK) {
		return;
	}
	wxString new_note = note_dialog.get_note();
	config.update_bookmark_note(file_path, selected_bookmark.start, selected_bookmark.end, new_note);
	config.flush();
	const auto it = std::find_if(all_bookmarks.begin(), all_bookmarks.end(), [&](const bookmark& bm) {
		return bm.start == selected_bookmark.start && bm.end == selected_bookmark.end;
	});
	if (it != all_bookmarks.end()) {
		it->note = new_note;
	}
	repopulate_list(text_ctrl ? text_ctrl->GetInsertionPoint() : -1);
}

void bookmark_dialog::on_filter_changed(wxCommandEvent&) {
	repopulate_list(text_ctrl ? text_ctrl->GetInsertionPoint() : -1);
}

void bookmark_dialog::repopulate_list(long current_pos) {
	if (current_pos == -1 && text_ctrl != nullptr) {
		current_pos = text_ctrl->GetInsertionPoint();
	}
	const int sel = filter_choice != nullptr ? filter_choice->GetSelection() : 0;
	const bool show_all = (sel == 0);
	const bool show_bookmarks_only = (sel == 1);
	const bool show_notes_only = (sel == 2);
	bookmark_list->Clear();
	bookmark_positions.clear();
	const long previously_selected = selected_position;
	int closest_index = -1;
	long closest_distance = LONG_MAX;
	auto add_entry = [&](const bookmark& bm) {
		wxString text_snippet;
		if (bm.is_whole_line()) {
			long line{0};
			text_ctrl->PositionToXY(bm.start, nullptr, &line);
			text_snippet = text_ctrl->GetLineText(line);
		} else {
			text_snippet = text_ctrl->GetRange(bm.start, bm.end);
		}
		text_snippet = text_snippet.Strip(wxString::both);
		if (text_snippet.IsEmpty()) {
			text_snippet = _("blank");
		}
		wxString display_text;
		if (bm.has_note()) {
			display_text = wxString::Format("%s - %s", bm.note, text_snippet);
		} else {
			display_text = text_snippet;
		}
		bookmark_positions.push_back(bm);
		bookmark_list->Append(display_text);
	};
	for (const auto& bm : all_bookmarks) {
		if (show_all || (show_bookmarks_only && !bm.has_note()) || (show_notes_only && bm.has_note())) {
			add_entry(bm);
			if (current_pos >= 0) {
				const long distance = std::abs(bm.start - current_pos);
				if (distance < closest_distance) {
					closest_distance = distance;
					closest_index = static_cast<int>(bookmark_positions.size() - 1);
				}
			}
		}
	}
	jump_button->Enable(false);
	delete_button->Enable(false);
	edit_note_button->Enable(false);
	selected_position = -1;
	if (previously_selected >= 0) {
		const auto it_sel = std::find_if(bookmark_positions.begin(), bookmark_positions.end(), [&](const bookmark& bm) {
			return bm.start == previously_selected;
		});
		if (it_sel != bookmark_positions.end()) {
			const int idx = static_cast<int>(std::distance(bookmark_positions.begin(), it_sel));
			bookmark_list->SetSelection(idx);
			selected_position = it_sel->start;
			jump_button->Enable(true);
			delete_button->Enable(true);
			edit_note_button->Enable(true);
			return;
		}
	}
	if (closest_index >= 0) {
		bookmark_list->SetSelection(closest_index);
		selected_position = bookmark_positions[static_cast<std::size_t>(closest_index)].start;
		jump_button->Enable(true);
		delete_button->Enable(true);
		edit_note_button->Enable(true);
	}
}

document_info_dialog::document_info_dialog(wxWindow* parent, const document* doc, const wxString& file_path, config_manager& cfg_mgr) : dialog(parent, _("Document Info"), dialog_button_config::ok_only), config_mgr{cfg_mgr}, doc_path{file_path} {
	constexpr int info_width = 600;
	constexpr int info_height = 400;
	info_text_ctrl = new wxTextCtrl(this, wxID_ANY, wxEmptyString, wxDefaultPosition, wxSize(info_width, info_height), wxTE_MULTILINE | wxTE_READONLY);
	wxString info_text;
	info_text << _("Title: ") << doc->title << "\n";
	info_text << _("Author: ") << doc->author << "\n";
	info_text << _("Path: ") << file_path << "\n";
	info_text << _("Total number of words: ") << doc->stats.word_count << ".\n";
	info_text << _("Total number of lines: ") << doc->stats.line_count << ".\n";
	info_text << _("Total number of characters: ") << doc->stats.char_count << ".\n";
	info_text << _("Total number of characters (excluding whitespace): ") << doc->stats.char_count_no_whitespace << ".\n";
	info_text_ctrl->SetValue(info_text);
	auto* content_sizer = new wxBoxSizer(wxVERTICAL);
	content_sizer->Add(info_text_ctrl, 1, wxEXPAND);
	set_content(content_sizer);
	finalize_layout();
}

elements_dialog::elements_dialog(wxWindow* parent, const document* doc, long current_pos) : dialog(parent, _("Elements")), doc(doc), current_pos(current_pos) {
	auto* content_sizer = new wxBoxSizer(wxVERTICAL);
	auto* choice_sizer = new wxBoxSizer(wxHORIZONTAL);
	auto* choice_label = new wxStaticText(this, wxID_ANY, _("&View:"));
	view_choice = new wxComboBox(this, wxID_ANY, "", wxDefaultPosition, wxDefaultSize, 0, nullptr, wxCB_READONLY);
	view_choice->Append(_("Headings"));
	view_choice->Append(_("Links"));
	view_choice->SetSelection(0);
	choice_sizer->Add(choice_label, 0, wxALIGN_CENTER_VERTICAL | wxRIGHT, DIALOG_PADDING);
	choice_sizer->Add(view_choice, 1, wxEXPAND);
	content_sizer->Add(choice_sizer, 0, wxEXPAND | wxALL, DIALOG_PADDING);
	headings_sizer = new wxBoxSizer(wxVERTICAL);
	headings_tree = new wxTreeCtrl(this, wxID_ANY, wxDefaultPosition, wxSize(400, 500), wxTR_DEFAULT_STYLE | wxTR_HIDE_ROOT);
	headings_sizer->Add(headings_tree, 1, wxEXPAND);
	content_sizer->Add(headings_sizer, 1, wxEXPAND | wxLEFT | wxRIGHT | wxBOTTOM, DIALOG_PADDING);
	links_sizer = new wxBoxSizer(wxVERTICAL);
	links_list = new wxListBox(this, wxID_ANY);
	links_sizer->Add(links_list, 1, wxEXPAND);
	content_sizer->Add(links_sizer, 1, wxEXPAND | wxLEFT | wxRIGHT | wxBOTTOM, DIALOG_PADDING);
	populate_headings();
	populate_links();
	links_sizer->Show(false);
	set_content(content_sizer);
	finalize_layout();
	Bind(wxEVT_COMBOBOX, &elements_dialog::on_view_choice_changed, this, view_choice->GetId());
	Bind(wxEVT_TREE_ITEM_ACTIVATED, &elements_dialog::on_heading_activated, this, headings_tree->GetId());
	Bind(wxEVT_BUTTON, &elements_dialog::on_ok, this, wxID_OK);
	if (view_choice->GetSelection() == 0) {
		headings_tree->SetFocus();
	} else {
		links_list->SetFocus();
	}
	view_choice->SetFocus();
}

void elements_dialog::populate_links() {
	const auto link_markers = markers_by_type(doc, marker_type::Link);
	int closest_index = -1;
	for (const auto* link_marker : link_markers) {
		links_list->Append(link_marker->text);
		links_list->SetClientData(links_list->GetCount() - 1, reinterpret_cast<void*>(link_marker->pos));
	}
	if (links_list->IsEmpty()) {
		return;
	}
	for (int i = links_list->GetCount() - 1; i >= 0; i--) {
		const size_t pos = reinterpret_cast<size_t>(links_list->GetClientData(i));
		if (pos <= static_cast<size_t>(current_pos)) {
			closest_index = i;
			break;
		}
	}
	if (closest_index != -1) {
		links_list->SetSelection(closest_index);
	} else {
		links_list->SetSelection(0);
	}
}

void elements_dialog::populate_headings() {
	const wxTreeItemId root = headings_tree->AddRoot(_("Root"));
	std::vector<wxTreeItemId> parent_ids(7, root);
	const auto heading_marker_list = heading_markers(doc);
	wxTreeItemId closest_item;
	for (const auto* heading_marker : heading_marker_list) {
		const int level = heading_marker->level;
		if (level < 1 || level > 6) {
			continue;
		}
		const wxTreeItemId parent_id = parent_ids[level - 1];
		const wxString display_text = heading_marker->text.IsEmpty() ? wxString(_("Untitled")) : heading_marker->text;
		const wxTreeItemId item_id = headings_tree->AppendItem(parent_id, display_text);
		headings_tree->SetItemData(item_id, new toc_tree_item_data(heading_marker->pos));
		if (static_cast<long>(heading_marker->pos) <= current_pos) {
			closest_item = item_id;
		}
		for (int i = level; i < 7; ++i) {
			parent_ids[i] = item_id;
		}
	}
	headings_tree->ExpandAll();
	if (closest_item.IsOk()) {
		headings_tree->SelectItem(closest_item);
		headings_tree->EnsureVisible(closest_item);
	} else {
		wxTreeItemIdValue cookie;
		const wxTreeItemId first_item = headings_tree->GetFirstChild(headings_tree->GetRootItem(), cookie);
		if (first_item.IsOk()) {
			headings_tree->SelectItem(first_item);
			headings_tree->EnsureVisible(first_item);
		}
	}
}

void elements_dialog::on_view_choice_changed(wxCommandEvent& /*event*/) {
	const int selection = view_choice->GetSelection();
	if (selection == 0) {
		headings_sizer->Show(true);
		links_sizer->Show(false);
	} else {
		headings_sizer->Show(false);
		links_sizer->Show(true);
	}
	view_choice->SetFocus();
	main_sizer->Layout();
}

void elements_dialog::on_heading_activated(wxTreeEvent& event) {
	const wxTreeItemId item = event.GetItem();
	if (item.IsOk()) {
		const auto* data = dynamic_cast<toc_tree_item_data*>(headings_tree->GetItemData(item));
		if (data != nullptr) {
			selected_offset = data->offset;
			EndModal(wxID_OK);
		}
	}
}

void elements_dialog::on_ok(wxCommandEvent& /*event*/) {
	if (view_choice->GetSelection() == 0) {
		const wxTreeItemId item = headings_tree->GetSelection();
		if (item.IsOk()) {
			const auto* data = dynamic_cast<toc_tree_item_data*>(headings_tree->GetItemData(item));
			if (data != nullptr) {
				selected_offset = data->offset;
				EndModal(wxID_OK);
			}
		}
	} else {
		const int selection = links_list->GetSelection();
		if (selection != wxNOT_FOUND) {
			selected_offset = static_cast<int>(reinterpret_cast<size_t>(links_list->GetClientData(selection)));
			EndModal(wxID_OK);
		}
	}
}

find_dialog::find_dialog(wxWindow* parent) : wxDialog(parent, wxID_ANY, _("Find")) {
	constexpr int combo_width = 250;
	constexpr int option_padding = 2;
	constexpr int button_spacing = 5;
	auto* const main_sizer = new wxBoxSizer(wxVERTICAL);
	auto* const find_sizer = new wxBoxSizer(wxHORIZONTAL);
	auto* const find_label = new wxStaticText(this, wxID_ANY, _("Find &what:"));
	find_what_combo = new wxComboBox(this, wxID_ANY, "", wxDefaultPosition, wxSize(combo_width, -1), 0, nullptr, wxTE_PROCESS_ENTER);
	find_sizer->Add(find_label, 0, wxALIGN_CENTER_VERTICAL | wxRIGHT, DIALOG_PADDING);
	find_sizer->Add(find_what_combo, 1, wxEXPAND);
	auto* const options_box = new wxStaticBoxSizer(wxVERTICAL, this, _("Options"));
	match_case_check = new wxCheckBox(this, wxID_ANY, _("&Match case"));
	match_whole_word_check = new wxCheckBox(this, wxID_ANY, _("Match &whole word"));
	use_regex_check = new wxCheckBox(this, wxID_ANY, _("Use &regular expressions"));
	options_box->Add(match_case_check, 0, wxALL, option_padding);
	options_box->Add(match_whole_word_check, 0, wxALL, option_padding);
	options_box->Add(use_regex_check, 0, wxALL, option_padding);
	auto* const button_sizer = new wxBoxSizer(wxHORIZONTAL);
	find_previous_btn = new wxButton(this, wxID_ANY, _("Find &Previous"));
	find_next_btn = new wxButton(this, wxID_ANY, _("Find &Next"));
	cancel_btn = new wxButton(this, wxID_CANCEL, _("Cancel"));
	button_sizer->Add(find_previous_btn, 0, wxRIGHT, button_spacing);
	button_sizer->Add(find_next_btn, 0, wxRIGHT, button_spacing);
	button_sizer->AddStretchSpacer();
	button_sizer->Add(cancel_btn, 0);
	find_next_btn->SetDefault();
	main_sizer->Add(find_sizer, 0, wxEXPAND | wxALL, DIALOG_PADDING);
	main_sizer->Add(options_box, 0, wxEXPAND | wxLEFT | wxRIGHT | wxBOTTOM, DIALOG_PADDING);
	main_sizer->Add(button_sizer, 0, wxEXPAND | wxLEFT | wxRIGHT | wxBOTTOM, DIALOG_PADDING);
	SetSizer(main_sizer);
	find_previous_btn->Bind(wxEVT_BUTTON, &find_dialog::on_find_previous, this);
	find_next_btn->Bind(wxEVT_BUTTON, &find_dialog::on_find_next, this);
	cancel_btn->Bind(wxEVT_BUTTON, &find_dialog::on_cancel, this);
	find_what_combo->Bind(wxEVT_TEXT_ENTER, &find_dialog::on_find_text_enter, this);
	Bind(wxEVT_CLOSE_WINDOW, &find_dialog::on_close, this);
	find_what_combo->SetFocus();
	Fit();
	CenterOnParent();
}

wxString find_dialog::get_find_text() const {
	return find_what_combo->GetValue();
}

bool find_dialog::get_match_case() const noexcept {
	return match_case_check->GetValue();
}

bool find_dialog::get_match_whole_word() const noexcept {
	return match_whole_word_check->GetValue();
}

bool find_dialog::get_use_regex() const noexcept {
	return use_regex_check->GetValue();
}

void find_dialog::set_find_text(const wxString& text) {
	find_what_combo->SetValue(text);
	find_what_combo->SetSelection(-1, -1);
}

void find_dialog::add_to_history(const wxString& text) {
	if (text.IsEmpty()) {
		return;
	}
	const int existing = find_what_combo->FindString(text);
	if (existing != wxNOT_FOUND) {
		find_what_combo->Delete(static_cast<unsigned int>(existing));
	}
	find_what_combo->Insert(text, 0);
	while (find_what_combo->GetCount() > MAX_FIND_HISTORY_SIZE) {
		find_what_combo->Delete(find_what_combo->GetCount() - 1);
	}
	find_what_combo->SetValue(text);
}

void find_dialog::focus_find_text() {
	find_what_combo->SetFocus();
	find_what_combo->SetSelection(-1, -1);
}

void find_dialog::on_find_previous(wxCommandEvent& /*event*/) {
	const wxString text = get_find_text();
	if (!text.IsEmpty()) {
		add_to_history(text);
		const wxCommandEvent find_event(wxEVT_COMMAND_MENU_SELECTED, ID_FIND_PREVIOUS);
		wxPostEvent(GetParent(), find_event);
	}
}

void find_dialog::on_find_next(wxCommandEvent& /*event*/) {
	const wxString text = get_find_text();
	if (!text.IsEmpty()) {
		add_to_history(text);
		const wxCommandEvent find_event(wxEVT_COMMAND_MENU_SELECTED, ID_FIND_NEXT);
		wxPostEvent(GetParent(), find_event);
	}
}

void find_dialog::on_cancel(wxCommandEvent& /*event*/) {
	Hide();
}

void find_dialog::on_find_text_enter(wxCommandEvent& event) {
	const bool has_text = !get_find_text().IsEmpty();
	on_find_next(event);
	if (has_text) {
		Hide();
	}
}

void find_dialog::on_close(wxCloseEvent& /*event*/) {
	Hide();
}

go_to_line_dialog::go_to_line_dialog(wxWindow* parent, wxTextCtrl* text_ctrl) : dialog(parent, _("Go to Line")), textbox{text_ctrl} {
	constexpr int label_spacing = 5;
	auto* line_sizer = new wxBoxSizer(wxHORIZONTAL);
	auto* label = new wxStaticText(this, wxID_ANY, _("&Line number:"));
	long line = 0;
	textbox->PositionToXY(textbox->GetInsertionPoint(), nullptr, &line);
	input_ctrl = new wxSpinCtrl(this, wxID_ANY, wxEmptyString, wxDefaultPosition, wxDefaultSize, wxSP_ARROW_KEYS, 1, textbox->GetNumberOfLines(), line + 1);
	line_sizer->Add(label, 0, wxALIGN_CENTER_VERTICAL | wxRIGHT, label_spacing);
	line_sizer->Add(input_ctrl, 1, wxEXPAND);
	set_content(line_sizer);
	finalize_layout();
}

long go_to_line_dialog::get_position() const {
	const long line = input_ctrl->GetValue();
	if (line >= 1 && line <= textbox->GetNumberOfLines()) {
		return textbox->XYToPosition(0, line - 1);
	}
	return textbox->GetInsertionPoint();
}

go_to_page_dialog::go_to_page_dialog(wxWindow* parent, document* doc, const parser_info* parser, int current_page) : dialog(parent, _("Go to page")), doc_{doc}, parser_{parser} {
	constexpr int label_spacing = 5;
	auto* page_sizer = new wxBoxSizer(wxHORIZONTAL);
	auto* label = new wxStaticText(this, wxID_ANY, wxString::Format(_("Go to page (1/%d):"), get_max_page()));
	input_ctrl = new wxSpinCtrl(this, wxID_ANY, wxEmptyString, wxDefaultPosition, wxDefaultSize, wxSP_ARROW_KEYS, 1, get_max_page(), current_page);
	page_sizer->Add(label, 0, wxALIGN_CENTER_VERTICAL | wxRIGHT, label_spacing);
	page_sizer->Add(input_ctrl, 1, wxEXPAND);
	set_content(page_sizer);
	finalize_layout();
}

int go_to_page_dialog::get_page_number() const {
	const long page = input_ctrl->GetValue();
	if (page >= 1 && page <= get_max_page()) {
		return static_cast<int>(page);
	}
	return 1;
}

int go_to_page_dialog::get_max_page() const {
	if (doc_ == nullptr || parser_ == nullptr) {
		return 1;
	}
	if (!parser_supports(parser_->flags, parser_flags::supports_pages)) {
		return 1;
	}
	return static_cast<int>(count_markers(doc_, marker_type::PageBreak));
}

go_to_percent_dialog::go_to_percent_dialog(wxWindow* parent, wxTextCtrl* text_ctrl) : dialog(parent, _("Go to Percent")), textbox{text_ctrl} {
	constexpr int percent_max = 100;
	constexpr int label_spacing = 5;
	const long current_pos = textbox->GetInsertionPoint();
	const long total_pos = textbox->GetLastPosition();
	const int current_percent = total_pos > 0 ? static_cast<int>((current_pos * percent_max) / total_pos) : 0;
	auto* input_label = new wxStaticText(this, wxID_ANY, _("P&ercent:"));
	input_ctrl = new wxSpinCtrl(this, wxID_ANY, wxEmptyString, wxDefaultPosition, wxDefaultSize, wxSP_ARROW_KEYS, 0, percent_max, current_percent);
	auto* slider_label = new wxStaticText(this, wxID_ANY, _("&Percent"));
	percent_slider = new accessible_slider(this, wxID_ANY, current_percent, 0, percent_max);
	auto* content_sizer = new wxBoxSizer(wxVERTICAL);
	content_sizer->Add(slider_label, 0, wxALIGN_CENTER_VERTICAL | wxRIGHT, label_spacing);
	content_sizer->Add(percent_slider, 0, wxEXPAND | wxBOTTOM, label_spacing);
	content_sizer->Add(input_label, 0, wxALIGN_CENTER_VERTICAL | wxLEFT, label_spacing);
	content_sizer->Add(input_ctrl, 0, wxEXPAND);
	percent_slider->Bind(wxEVT_SLIDER, &go_to_percent_dialog::on_slider_changed, this);
	input_ctrl->Bind(wxEVT_SPINCTRL, &go_to_percent_dialog::on_spin_changed, this);
	set_content(content_sizer);
	finalize_layout();
	percent_slider->SetFocus();
}

long go_to_percent_dialog::get_position() const {
	constexpr int percent_max = 100;
	const long percent = input_ctrl->GetValue();
	const long total_chars = textbox->GetLastPosition();
	return (percent * total_chars + percent_max - 1) / percent_max;
}

void go_to_percent_dialog::on_slider_changed(wxCommandEvent& /*event*/) {
	const int slider_value = percent_slider->GetValue();
	input_ctrl->SetValue(slider_value);
}

void go_to_percent_dialog::on_spin_changed(wxSpinEvent& /*event*/) {
	const int spin_value = input_ctrl->GetValue();
	percent_slider->SetValue(spin_value);
}

open_as_dialog::open_as_dialog(wxWindow* parent, const wxString& path) : dialog(parent, _("Open As")) {
	constexpr int label_padding = 5;
	auto* content_sizer = new wxBoxSizer(wxVERTICAL);
	auto* label = new wxStaticText(this, wxID_ANY, wxString::Format(_("No suitable parser was found for %s.\nHow would you like to open this file?"), path));
	content_sizer->Add(label, 0, wxALL, label_padding);
	auto* format_sizer = new wxBoxSizer(wxHORIZONTAL);
	auto* format_label = new wxStaticText(this, wxID_ANY, _("Open &as:"));
	format_combo = new wxComboBox(this, wxID_ANY, "", wxDefaultPosition, wxDefaultSize, 0, nullptr, wxCB_READONLY);
	format_combo->Append(_("Plain Text"));
	format_combo->Append(_("HTML"));
	format_combo->Append(_("Markdown"));
	format_combo->SetSelection(0);
	format_sizer->Add(format_label, 0, wxALIGN_CENTER_VERTICAL | wxRIGHT, DIALOG_PADDING);
	format_sizer->Add(format_combo, 1, wxEXPAND);
	content_sizer->Add(format_sizer, 0, wxEXPAND | wxALL, label_padding);
	set_content(content_sizer);
	finalize_layout();
	format_combo->SetFocus();
}

wxString open_as_dialog::get_selected_format() const {
	const int selection = format_combo->GetSelection();
	switch (selection) {
		case 1:
			return "html";
		case 2:
			return "md";
		default:
			return "txt";
	}
}

note_entry_dialog::note_entry_dialog(wxWindow* parent, const wxString& title, const wxString& message, const wxString& existing_note) : dialog(parent, title) {
	auto* content_sizer = new wxBoxSizer(wxVERTICAL);
	auto* message_label = new wxStaticText(this, wxID_ANY, message);
	content_sizer->Add(message_label, 0, wxALL, DIALOG_PADDING);
	note_ctrl = new wxTextCtrl(this, wxID_ANY, existing_note, wxDefaultPosition, wxSize(400, 200), wxTE_MULTILINE);
	content_sizer->Add(note_ctrl, 1, wxEXPAND | wxLEFT | wxRIGHT | wxBOTTOM, DIALOG_PADDING);
	set_content(content_sizer);
	finalize_layout();
	note_ctrl->SetFocus();
	note_ctrl->Bind(wxEVT_KEY_DOWN, &note_entry_dialog::on_key_down, this);
}

wxString note_entry_dialog::get_note() const {
	return note_ctrl->GetValue();
}

void note_entry_dialog::on_key_down(wxKeyEvent& event) {
	if (event.GetKeyCode() == WXK_RETURN && event.ShiftDown()) {
		note_ctrl->WriteText("\n");
	} else if (event.GetKeyCode() == WXK_RETURN) {
		EndModal(wxID_OK);
	} else {
		event.Skip();
	}
}

options_dialog::options_dialog(wxWindow* parent) : dialog(parent, _("Options")) {
	constexpr int option_padding = 5;
	constexpr int max_recent_docs = 100;
	constexpr int default_recent_docs = 10;
	auto* general_box = new wxStaticBoxSizer(wxVERTICAL, this, _("General"));
	restore_docs_check = new wxCheckBox(this, wxID_ANY, _("&Restore previously opened documents on startup"));
	general_box->Add(restore_docs_check, 0, wxALL, option_padding);
	word_wrap_check = new wxCheckBox(this, wxID_ANY, _("&Word wrap"));
	general_box->Add(word_wrap_check, 0, wxALL, option_padding);
	minimize_to_tray_check = new wxCheckBox(this, wxID_ANY, _("&Minimize to system tray"));
	general_box->Add(minimize_to_tray_check, 0, wxALL, option_padding);
	start_maximized_check = new wxCheckBox(this, wxID_ANY, _("&Start maximized"));
	general_box->Add(start_maximized_check, 0, wxALL, option_padding);
	compact_go_menu_check = new wxCheckBox(this, wxID_ANY, _("Show compact &go menu"));
	general_box->Add(compact_go_menu_check, 0, wxALL, option_padding);
	navigation_wrap_check = new wxCheckBox(this, wxID_ANY, _("&Wrap navigation"));
	general_box->Add(navigation_wrap_check, 0, wxALL, option_padding);
	check_for_updates_on_startup_check = new wxCheckBox(this, wxID_ANY, _("Check for &updates on startup"));
	general_box->Add(check_for_updates_on_startup_check, 0, wxALL, option_padding);
	auto* recent_docs_sizer = new wxBoxSizer(wxHORIZONTAL);
	auto* recent_docs_label = new wxStaticText(this, wxID_ANY, _("Number of &recent documents to show:"));
	recent_docs_count_spin = new wxSpinCtrl(this, wxID_ANY, wxEmptyString, wxDefaultPosition, wxDefaultSize, wxSP_ARROW_KEYS, 0, max_recent_docs, default_recent_docs);
	recent_docs_sizer->Add(recent_docs_label, 0, wxALIGN_CENTER_VERTICAL | wxRIGHT, DIALOG_PADDING);
	recent_docs_sizer->Add(recent_docs_count_spin, 0, wxALIGN_CENTER_VERTICAL);
	general_box->Add(recent_docs_sizer, 0, wxALL, option_padding);
	auto* language_sizer = new wxBoxSizer(wxHORIZONTAL);
	auto* language_label = new wxStaticText(this, wxID_ANY, _("&Language:"));
	language_combo = new wxComboBox(this, wxID_ANY, "", wxDefaultPosition, wxDefaultSize, 0, nullptr, wxCB_READONLY);
	const auto& available_languages = translation_manager::instance().get_available_languages();
	for (const auto& lang : available_languages) {
		language_combo->Append(lang.native_name, new wxStringClientData(lang.code));
	}
	language_sizer->Add(language_label, 0, wxALIGN_CENTER_VERTICAL | wxRIGHT, DIALOG_PADDING);
	language_sizer->Add(language_combo, 0, wxALIGN_CENTER_VERTICAL);
	general_box->Add(language_sizer, 0, wxALL, option_padding);
	set_content(general_box);
	Bind(wxEVT_BUTTON, &options_dialog::on_ok, this, wxID_OK);
	Bind(wxEVT_BUTTON, &options_dialog::on_cancel, this, wxID_CANCEL);
	finalize_layout();
}

bool options_dialog::get_restore_previous_documents() const {
	return restore_docs_check != nullptr ? restore_docs_check->GetValue() : false;
}

void options_dialog::set_restore_previous_documents(bool restore) {
	if (restore_docs_check != nullptr) {
		restore_docs_check->SetValue(restore);
	}
}

bool options_dialog::get_word_wrap() const {
	return word_wrap_check != nullptr ? word_wrap_check->GetValue() : false;
}

void options_dialog::set_word_wrap(bool word_wrap) {
	if (word_wrap_check != nullptr) {
		word_wrap_check->SetValue(word_wrap);
	}
}

bool options_dialog::get_minimize_to_tray() const {
	return minimize_to_tray_check != nullptr ? minimize_to_tray_check->GetValue() : false;
}

void options_dialog::set_minimize_to_tray(bool minimize) {
	if (minimize_to_tray_check != nullptr) {
		minimize_to_tray_check->SetValue(minimize);
	}
}

bool options_dialog::get_start_maximized() const {
	return start_maximized_check != nullptr ? start_maximized_check->GetValue() : false;
}

void options_dialog::set_start_maximized(bool maximized) {
	if (start_maximized_check != nullptr) {
		start_maximized_check->SetValue(maximized);
	}
}

bool options_dialog::get_compact_go_menu() const {
	return compact_go_menu_check != nullptr ? compact_go_menu_check->GetValue() : true;
}

void options_dialog::set_compact_go_menu(bool compact) {
	if (compact_go_menu_check != nullptr) {
		compact_go_menu_check->SetValue(compact);
	}
}

bool options_dialog::get_navigation_wrap() const {
	return navigation_wrap_check != nullptr ? navigation_wrap_check->GetValue() : false;
}

void options_dialog::set_navigation_wrap(bool value) {
	if (navigation_wrap_check) {
		navigation_wrap_check->SetValue(value);
	}
}

bool options_dialog::get_check_for_updates_on_startup() const {
	return check_for_updates_on_startup_check != nullptr ? check_for_updates_on_startup_check->GetValue() : true;
}

void options_dialog::set_check_for_updates_on_startup(bool check) {
	if (check_for_updates_on_startup_check != nullptr) {
		check_for_updates_on_startup_check->SetValue(check);
	}
}

int options_dialog::get_recent_documents_to_show() const {
	constexpr int default_value = 10;
	return recent_docs_count_spin != nullptr ? recent_docs_count_spin->GetValue() : default_value;
}

void options_dialog::set_recent_documents_to_show(int count) {
	if (recent_docs_count_spin != nullptr) {
		recent_docs_count_spin->SetValue(count);
	}
}

wxString options_dialog::get_language() const {
	if (language_combo == nullptr) {
		return {};
	}
	const int selection = language_combo->GetSelection();
	if (selection == wxNOT_FOUND) {
		return {};
	}
	const auto* data = dynamic_cast<wxStringClientData*>(language_combo->GetClientObject(selection));
	return data != nullptr ? data->GetData() : wxString{};
}

void options_dialog::set_language(const wxString& language) {
	if (language_combo == nullptr) {
		return;
	}
	for (unsigned int i = 0; i < language_combo->GetCount(); ++i) {
		const auto* data = dynamic_cast<wxStringClientData*>(language_combo->GetClientObject(i));
		if (data != nullptr && data->GetData() == language) {
			language_combo->SetSelection(static_cast<int>(i));
			return;
		}
	}
}

void options_dialog::on_ok(wxCommandEvent& /*event*/) {
	EndModal(wxID_OK);
}

void options_dialog::on_cancel(wxCommandEvent& /*event*/) {
	EndModal(wxID_CANCEL);
}

password_dialog::password_dialog(wxWindow* parent) : dialog(parent, _("Document Password")) {
	auto* content_sizer = new wxBoxSizer(wxVERTICAL);
	auto* message_label = new wxStaticText(this, wxID_ANY, _("&Password"));
	content_sizer->Add(message_label, 0, wxALL, DIALOG_PADDING);
	password_ctrl = new wxTextCtrl(this, wxID_ANY, wxEmptyString, wxDefaultPosition, wxSize(300, -1), wxTE_PASSWORD);
	content_sizer->Add(password_ctrl, 0, wxEXPAND | wxLEFT | wxRIGHT | wxBOTTOM, DIALOG_PADDING);
	set_content(content_sizer);
	finalize_layout();
	password_ctrl->SetFocus();
}

wxString password_dialog::get_password() const {
	return password_ctrl->GetValue();
}

sleep_timer_dialog::sleep_timer_dialog(wxWindow* parent, int initial_duration) : dialog(parent, _("Sleep Timer")) {
	constexpr int label_spacing = 5;
	auto* sizer = new wxBoxSizer(wxHORIZONTAL);
	auto* label = new wxStaticText(this, wxID_ANY, _("&Minutes:"));
	input_ctrl = new wxSpinCtrl(this, wxID_ANY, wxEmptyString, wxDefaultPosition, wxDefaultSize, wxSP_ARROW_KEYS, 1, 999, initial_duration);
	sizer->Add(label, 0, wxALIGN_CENTER_VERTICAL | wxRIGHT, label_spacing);
	sizer->Add(input_ctrl, 1, wxEXPAND);
	set_content(sizer);
	finalize_layout();
}

int sleep_timer_dialog::get_duration() const {
	return input_ctrl->GetValue();
}

toc_dialog::toc_dialog(wxWindow* parent, const document* doc, int current_offset) : dialog(parent, _("Table of Contents")), selected_offset{-1} {
	search_timer_ = new wxTimer(this);
	tree = new wxTreeCtrl(this, wxID_ANY, wxDefaultPosition, wxDefaultSize, wxTR_HIDE_ROOT);
	const wxTreeItemId root = tree->AddRoot(_("Root"));
	populate_tree(doc->toc_items, root);
	if (current_offset != -1) {
		find_and_select_item(root, current_offset);
	}
	auto* content_sizer = new wxBoxSizer(wxVERTICAL);
	content_sizer->Add(tree, 1, wxEXPAND);
	set_content(content_sizer);
	Bind(wxEVT_TREE_SEL_CHANGED, &toc_dialog::on_tree_selection_changed, this);
	Bind(wxEVT_TREE_ITEM_ACTIVATED, &toc_dialog::on_tree_item_activated, this, wxID_ANY);
	Bind(wxEVT_BUTTON, &toc_dialog::on_ok, this, wxID_OK);
	Bind(wxEVT_CHAR_HOOK, &toc_dialog::on_char_hook, this);
	Bind(wxEVT_TIMER, &toc_dialog::on_search_timer, this, search_timer_->GetId());
	finalize_layout();
}

void toc_dialog::populate_tree(const std::vector<std::unique_ptr<toc_item>>& items, const wxTreeItemId& parent) {
	for (const auto& item : items) {
		const wxString display_text = item->name.IsEmpty() ? wxString(_("Untitled")) : item->name;
		const wxTreeItemId item_id = tree->AppendItem(parent, display_text);
		tree->SetItemData(item_id, new toc_tree_item_data(item->offset));
		if (!item->children.empty()) {
			populate_tree(item->children, item_id);
		}
	}
}

void toc_dialog::find_and_select_item(const wxTreeItemId& parent, int offset) {
	wxTreeItemIdValue cookie{};
	for (wxTreeItemId item_id = tree->GetFirstChild(parent, cookie); item_id.IsOk(); item_id = tree->GetNextChild(parent, cookie)) {
		const auto* data = dynamic_cast<toc_tree_item_data*>(tree->GetItemData(item_id));
		if (data != nullptr && data->offset == offset) {
			tree->SelectItem(item_id);
			tree->SetFocusedItem(item_id);
			tree->EnsureVisible(item_id);
			selected_offset = data->offset;
			return;
		}
		if (tree->ItemHasChildren(item_id)) {
			find_and_select_item(item_id, offset);
		}
	}
}

void toc_dialog::on_tree_selection_changed(wxTreeEvent& event) {
	const wxTreeItemId item = event.GetItem();
	if (!item.IsOk()) {
		return;
	}
	const auto* data = dynamic_cast<toc_tree_item_data*>(tree->GetItemData(item));
	if (data == nullptr) {
		return;
	}
	selected_offset = data->offset;
}

void toc_dialog::on_tree_item_activated(wxTreeEvent& /*event*/) {
	if (selected_offset >= 0) {
		EndModal(wxID_OK);
	}
}

void toc_dialog::on_ok(wxCommandEvent& /*event*/) {
	if (selected_offset >= 0) {
		EndModal(wxID_OK);
	} else {
		wxMessageBox(_("Please select a section from the table of contents."), _("No Selection"), wxOK | wxICON_INFORMATION, this);
	}
}

void toc_dialog::on_char_hook(wxKeyEvent& event) {
	const int key_code = event.GetKeyCode();
	wxWindow* focused = wxWindow::FindFocus();
	if (focused != tree || key_code < WXK_SPACE || key_code >= WXK_DELETE) {
		event.Skip();
		return;
	}
	const wxChar current_char = static_cast<wxChar>(event.GetUnicodeKey());
	if (search_string_.IsEmpty()) {
		if (current_char == ' ') {
			return;
		}
		search_string_ = current_char;
		search_timer_->StartOnce(500);
		event.Skip();
		return;
	}
	if (search_string_.Last() != current_char) {
		search_string_ += current_char;
		search_timer_->StartOnce(500);
		if (!find_and_select_item_by_name(search_string_, tree->GetRootItem())) {
			search_string_.RemoveLast();
			wxBell();
		}
	} else {
		search_timer_->StartOnce(500);
		event.Skip();
	}
}

void toc_dialog::on_search_timer(wxTimerEvent&) {
	search_string_.Clear();
}

bool toc_dialog::find_and_select_item_by_name(const wxString& name, const wxTreeItemId& parent) {
	wxTreeItemIdValue cookie{};
	for (wxTreeItemId item_id = tree->GetFirstChild(parent, cookie); item_id.IsOk(); item_id = tree->GetNextChild(parent, cookie)) {
		if (tree->GetItemText(item_id).Lower().StartsWith(name.Lower())) {
			tree->SelectItem(item_id);
			tree->SetFocusedItem(item_id);
			tree->EnsureVisible(item_id);
			return true;
		}
		if (tree->ItemHasChildren(item_id)) {
			if (find_and_select_item_by_name(name, item_id)) {
				return true;
			}
		}
	}
	return false;
}

update_dialog::update_dialog(wxWindow* parent, const wxString& new_version, const wxString& changelog) : dialog(parent, wxString::Format(_("Update to %s"), new_version), dialog_button_config::ok_cancel) {
	auto* content_sizer = new wxBoxSizer(wxVERTICAL);
	auto* message = new wxStaticText(this, wxID_ANY, _("A new version of Paperback is available. Here's what's new:"));
	content_sizer->Add(message, 0, wxALL, DIALOG_PADDING);
	changelog_ctrl = new wxTextCtrl(this, wxID_ANY, changelog, wxDefaultPosition, wxSize(500, 300), wxTE_MULTILINE | wxTE_READONLY | wxTE_RICH2);
	content_sizer->Add(changelog_ctrl, 1, wxEXPAND | wxLEFT | wxRIGHT | wxBOTTOM, DIALOG_PADDING);
	set_content(content_sizer);
	finalize_layout();
	auto* ok_button = FindWindow(wxID_OK);
	if (ok_button) ok_button->SetLabel(_("&Yes"));
	auto* cancel_button = FindWindow(wxID_CANCEL);
	if (cancel_button) cancel_button->SetLabel(_("&No"));
	changelog_ctrl->SetFocus();
}

view_note_dialog::view_note_dialog(wxWindow* parent, const wxString& note_text) : dialog(parent, _("View Note"), dialog_button_config::ok_only) {
	auto* content_sizer = new wxBoxSizer(wxVERTICAL);
	note_ctrl = new wxTextCtrl(this, wxID_ANY, note_text, wxDefaultPosition, wxSize(400, 200), wxTE_MULTILINE | wxTE_READONLY | wxTE_RICH2);
	content_sizer->Add(note_ctrl, 1, wxEXPAND | wxALL, DIALOG_PADDING);
	set_content(content_sizer);
	finalize_layout();
	FindWindow(wxID_OK)->SetLabel(_("Close"));
	note_ctrl->SetFocus();
}
